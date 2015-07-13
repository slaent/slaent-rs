use http_muncher::Parser;
use mio::*;
use mio::tcp::*;
use mio::util::Slab;
use route::EchoHandler;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};

const SERVER: Token = Token(0);

#[derive(Clone,Copy,Debug,PartialEq)]
enum State {
    Read,
    Write(&'static [u8]),
}

struct EchoConn {
    sock: TcpStream,
    state: State,
    events: EventSet,
}

impl EchoConn {
    fn new(sock: TcpStream) -> EchoConn {
        EchoConn {
            sock: sock,
            state: State::Read,
            events: EventSet::none()
        }
    }

    fn writable(&mut self) -> io::Result<()> {
        let message = match self.state {
            State::Write(message) => message,
            _ => return Ok(())
        };

        loop {
            match self.sock.try_write(message) {
                Ok(None) => {
                    self.events.remove(EventSet::writable());
                    if let State::Write(_) = self.state {
                        // Error!
                        self.state = State::Read;
                    }
                    return Ok(());
                },
                Ok(Some(r)) => {
                    self.state = State::Read;
                    if r == message.len() {
                        self.events.remove(EventSet::writable());
                        return Ok(());
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    fn readable(&mut self, mut buf: &mut [u8], parser: &mut Parser<EchoHandler>) -> io::Result<()> {
        const NO_CONTENT: &'static [u8] = b"HTTP/1.1 204 No Content\r\n\r\n";
        const NOT_FOUND: &'static [u8] = b"HTTP/1.1 404 Not Found\r\nContent-Length: 1\r\n\r\n ";

        if self.state != State::Read { return Ok(()); }
        let len = buf.len();
        loop {
            match self.sock.try_read(&mut buf) {
                Ok(None) => {
                    self.events.remove(EventSet::readable());
                    if self.state == State::Read {
                        // Error!
                        *parser = Parser::request(EchoHandler::new());
                    }
                    return Ok(())
                },
                Ok(Some(r)) => {
                    //println!("{}", String::from_utf8_lossy(&buf[..r]));
                    parser.parse(&buf[..r]);
                    //println!("{:?}", parser);
                    if r < len {
                        self.events.remove(EventSet::readable());
                    }
                    if parser.has_error() {
                        self.state = State::Write(NOT_FOUND);
                        *parser = Parser::request(EchoHandler::new());
                    } else /*|| parser.is_final_chunk()*/ {
                        self.state = State::Write(NO_CONTENT);
                        if r < len  {
                            *parser = Parser::request(EchoHandler::new());
                            return Ok(());
                        }
                    }
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
    }
}

struct EchoServer {
    buf: [u8; 2048],
    request_parser: Parser<EchoHandler>,
    conns: Slab<EchoConn>,
    sock: TcpListener,
}

impl EchoServer {
    fn new(sock: TcpListener) -> Self {
        EchoServer {
            buf: [0; 2048],
            request_parser: Parser::request(EchoHandler::new()),
            conns: Slab::new_starting_at(Token(1), 8192),
            sock: sock,
        }
    }

    fn accept(&mut self, event_loop: &mut EventLoop<EchoServer>) -> io::Result<()> {
        loop {
            // We check has_remaining() instead of just matching on the result of the insert to
            // avoid expensive initialization of TcpSocket unless we have room for it.
            if !self.conns.has_remaining() {
                return Ok(());
            }
            let sock = match self.sock.accept() {
                Ok(Some(sock)) => sock,
                _ => return Ok(())
            };
            let conn = EchoConn::new(sock);
            let tok = self.conns.insert(conn).ok().unwrap();

            // Register the connection
            if let Err(_) = event_loop.register_opt(&self.conns[tok].sock, tok, EventSet::all(), PollOpt::edge()) {
                self.disconn(tok);
                return Ok(());
            }
        }
    }

    fn conn_readable(&mut self, tok: Token, events: EventSet) -> io::Result<()> {
        let is_readable = {
            let conn = &mut self.conn(tok);
            if events.is_readable() {
                conn.events.insert(EventSet::readable());
                true
            } else {
                conn.events.is_readable()
            }
        };
        if is_readable {
            self.conns[tok].readable(&mut self.buf, &mut self.request_parser)
        } else {
            Ok(())
        }
    }

    fn conn_writable(&mut self, tok: Token, events: EventSet) -> io::Result<()> {
        let is_writable = {
            let conn = &mut self.conn(tok);
            if events.is_writable() {
                conn.events.insert(EventSet::writable());
                true
            } else {
                conn.events.is_writable()
            }
        };
        if is_writable {
            self.conn(tok).writable()
        } else {
            Ok(())
        }
    }

    fn conn(&mut self, tok: Token) -> &mut EchoConn {
        &mut self.conns[tok]
    }

    fn disconn(&mut self, tok: Token) {
        self.conns.remove(tok);
    }
}

impl Handler for EchoServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<EchoServer>, token: Token, events: EventSet) {
        match token {
            SERVER => self.accept(event_loop).unwrap(),
            _ => {
                if events.is_hup() || events.is_error() ||
                   self.conn_readable(token, events).is_err() ||
                   self.conn_writable(token, events).is_err() {
                    // Nothing we can really do in case of errror.
                    let _ = event_loop.deregister(&self.conn(token).sock);
                    self.disconn(token);
                }
            }
        }
    }
}

pub fn test_echo_server() {
    let ref addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 18080));

    let srv = TcpSocket::v4().unwrap();
    srv.set_reuseaddr(true).unwrap();
    srv.bind(addr).unwrap();
    let srv = srv.listen(1024).unwrap();

    let mut event_loop = EventLoop::new().unwrap();

    event_loop.register_opt(&srv, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();

    let mut server = EchoServer::new(srv);

    // Start the event loop
    event_loop.run(&mut server).unwrap();
}
