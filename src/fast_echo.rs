use Map;
use http_muncher::{Parser, ParserSettings};
use mio::*;
use mio::tcp::*;
use mio::util::Slab;
use route::{self, Response, EchoHandler};
use std::io::{self, Cursor, Write};
use std::mem;
use std::net::{SocketAddr, SocketAddrV4};
use std::os::unix::io::AsRawFd;
use std::thread;

const SERVER: Token = Token(0);

#[derive(Clone,Debug)]
enum State {
    Read,
    Write(Response),
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
        use nix::sys::uio::{self, IoVec};
        use route::Response::*;

        const NO_HEADERS: &'static [u8] = b"HTTP/1.1 200 \r\nContent-Type: text/html\r\nContent-Length: ";

        /*let message = match self.state {
            State::Write(ref message) => message,
            _ => return Ok(())
        };*/

        loop {
            let mut len;
            let res = match /* *message */self.state {
                State::Read => return Ok(()),
                State::Write(Static(message)) => {
                    len = message.len();
                    self.sock.try_write(message)
                },
                State::Write(Body(ref body)) => {
                    const U10_BITS: usize = 4;
                    /// Max bits in a base 10 representation of usize:
                    /// log_2(usize::MAX) / log_2(10) (+ 1 ?).
                    const DIGIT_BYTES: usize = ::std::usize::BITS / U10_BITS + 1;
                    /// Four bytes for \r\n\r\n.
                    const NEWLINE_BYTES: usize = 4;
                    const MAX_BYTES: usize = DIGIT_BYTES + NEWLINE_BYTES;
                    let mut digits = [0u8; MAX_BYTES];
                    let mut cursor = Cursor::new(&mut digits[..]);
                    let body = body.as_bytes();
                    len = body.len() + NO_HEADERS.len();
                    write!(&mut cursor, "{}", body.len()).and_then( |_| {
                        let pos = cursor.position() as usize;
                        let digits = cursor.into_inner();
                        if pos < MAX_BYTES - NEWLINE_BYTES {
                            digits[pos] = b'\r';
                            digits[pos + 1] = b'\n';
                            digits[pos + 2] = b'\r';
                            digits[pos + 3] = b'\n';
 
                            let headers = IoVec::from_slice(NO_HEADERS);
                            let digits_len = pos + NEWLINE_BYTES;
                            len += digits_len;
                            let digits = &digits[0..digits_len];
                            let digits = IoVec::from_slice(digits);
                            let body = IoVec::from_slice(body);
                            uio::writev(self.sock.as_raw_fd(), &[headers, digits, body])
                                .map(Some)
                                .map_err( |err| io::Error::from_raw_os_error(err.errno() as i32))
                        } else {
                            // Couldn't fit into the content length (which should never actually
                            // happen, but whatever... if it does better to return than crash, and
                            // I doubt you actually want to write a four billion byte response).
                            Ok(None)
                        }
                    })
                },
                State::Write(Empty) => panic!("Get rid of this ASAP"),
            };
            match res {
                Ok(None) => {
                    self.events.remove(EventSet::writable());
                    if let State::Write(_) = self.state {
                        // Error!
                        self.state = State::Read;
                    }
                    if self.events.is_hup() {
                        self.events.remove(EventSet::hup());
                        return Err(io::Error::last_os_error());
                    }
                    return Ok(());
                },
                Ok(Some(r)) => {
                    self.state = State::Read;
                    if r == len {
                        self.events.remove(EventSet::writable());
                        if self.events.is_hup() {
                            self.events.remove(EventSet::hup());
                            return Err(io::Error::last_os_error());
                        }
                        return Ok(());
                    } else {
                        println!("hm");
                        return Ok(());
                    }
                },
                Err(e) => {
                    println!("error");
                    return Err(e);
                }
            }
        }
    }

    fn readable<'a>(&mut self, shared: &mut EchoShared<'a>/*, settings: &ParserSettings<EchoHandler<'a>>*/) -> io::Result<()> {
        const NO_CONTENT: &'static [u8] = b"HTTP/1.1 204 \r\nCache-Control:max-age=604800\r\n\r\n";
        const NO_CONTENT_KEEPALIVE: &'static [u8] = b"HTTP/1.1 204 \r\nConnection: Keep-Alive\r\n\r\n";
        const NO_CONTENT_CLOSE: &'static [u8] = b"HTTP/1.1 204 \r\nConnection: close\r\n\r\n";

        const NOT_FOUND: &'static [u8] = b"HTTP/1.1 404 Not Found\r\nContent-Length: 127\r\n\r\n<!doctype html><meta charset=utf-8><title>Not Found</title><h1>Not Found</h1><p>The requested URL was not found on this server.";
        const NOT_FOUND_CLOSE: &'static [u8] = b"HTTP/1.1 404 Not Found\r\nContent-Length: 127\r\nConnection: Close\r\n<!doctype html><meta charset=utf-8><title>Not Found</title><h1>Not Found</h1><p>The requested URL was not found on this server.";
        const NOT_FOUND_KEEPALIVE: &'static [u8] = b"HTTP/1.1 404 Not Found\r\nContent-Length: 127\r\nConnection: Keep-Alive\r\n\r\n<!doctype html><meta charset=utf-8><title>Not Found</title><h1>Not Found</h1><p>The requested URL was not found on this server.";

        if let State::Write(_) = self.state { return Ok(()); }
        let EchoShared { ref mut buf, request_parser: ref mut parser, thread_page_map } = *shared;
        let mut buf = &mut **buf;
        let settings = ParserSettings::new();
        let len = buf.len();
        let mut handler = EchoHandler::new(thread_page_map);
        loop {
            match self.sock.try_read(&mut buf) {
                Ok(None) => {
                    self.events.remove(EventSet::readable());
                    if let State::Read = self.state {
                        // Error!
                        *parser = Parser::request();
                    }
                    return Ok(())
                },
                Ok(Some(r)) => {
                    let mut buf_ = buf;
                    let (data, rest) = buf_.split_at_mut(r);
                    buf = rest;
                    //println!("{}", String::from_utf8_lossy(data));
                    parser.parse(&mut handler, &settings, &*data);
                    //println!("{:?}", parser);
                    let keep_alive = parser.should_keep_alive();
                    if !keep_alive {
                        self.events.insert(EventSet::hup());
                    }
                    let keep_alive_by_default = parser.http_version() >= (1, 1);
                    let good = if keep_alive == keep_alive_by_default {
                        NO_CONTENT
                    } else if keep_alive {
                        NO_CONTENT_KEEPALIVE
                    } else {
                        NO_CONTENT_CLOSE
                    };
                    /*let good = if parser.should_keep_alive() {
                        if parser.http_version() < (1, 1) {
                            NO_CONTENT_KEEPALIVE
                        } else {
                            NO_CONTENT
                        }
                    } else {
                        self.events.insert(EventSet::hup());
                        if parser.http_version() < (1, 1) {
                            NO_CONTENT
                        } else {
                            NO_CONTENT_CLOSE
                        }
                    };*/
                    let status = parser.status();
                    let bad = {
                        if keep_alive == keep_alive_by_default {
                            NOT_FOUND
                        } else if keep_alive {
                            NOT_FOUND_KEEPALIVE
                        } else {
                            NOT_FOUND_CLOSE
                        }
                    };
                    self.state = State::Write(match handler.response.take() {
                        Some(response) => {
                            if status.errno().is_ok() {
                                *parser = Parser::request();
                                match response {
                                    Response::Body(_) => response,
                                    Response::Empty => Response::Static(good),
                                    s @ Response::Static(_) => s,
                                }
                            } else {
                                *parser = Parser::request();
                                Response::Static(bad)
                            }
                        },
                        _ => {
                            *parser = Parser::request();
                            Response::Static(bad)
                        }
                    });
                    if r < len /*|| parser.is_final_chunk()*/ {
                        self.events.remove(EventSet::readable());
                        return Ok(());
                    }
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
    }
}

struct EchoShared<'a> {
    buf: &'a mut [u8],
    request_parser: Parser,
    //settings: &'a ParserSettings<'b, EchoHandler<'b>>,
    thread_page_map: &'static Map,
}

impl<'a> EchoShared<'a> {
    fn new(buf: &'a mut [u8], thread_page_map: &'static Map/*, settings: &'a ParserSettings<'b, EchoHandler<'b>>*/) -> Self {
        EchoShared {
            buf: buf,
            request_parser: Parser::request(),
            //settings: settings,
            thread_page_map: thread_page_map,
        }
    }
}

struct EchoServer</*'a, */'b> {// where 'b: 'a {
    shared: EchoShared<'b>,
    conns: Slab<EchoConn>,
    sock: TcpListener,
}

impl</*'a, */'b> EchoServer</*'a, */'b> {
    fn new(sock: TcpListener, shared: EchoShared<'b>) -> Self {
        EchoServer {
            shared: shared,
            conns: Slab::new_starting_at(Token(1), 65536),
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

            // Don't buffer output in TCP - kills latency sensitive benchmarks
            if let Err(_) = sock.set_nodelay(true) {
                let _ = sock.shutdown(Shutdown::Both);
                return Ok(());
            }

            let conn = EchoConn::new(sock);
            let tok = self.conns.insert(conn).ok().unwrap();

            // Register the connection
            if let Err(_) = event_loop.register_opt(&self.conns[tok].sock, tok, EventSet::all(), PollOpt::edge()) {
                self.disconn(tok).map( |conn| conn.sock.shutdown(Shutdown::Both) );
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
            //let settings = ParserSettings::new();
            //let settings = self.settings;
            self.conns[tok].readable(&mut self.shared/*&mut self.request_parser, &settings*/)
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

    fn disconn(&mut self, tok: Token) -> Option<EchoConn> {
        self.conns.remove(tok)
    }
}

impl<'a/*, 'b*/> Handler for EchoServer<'a/*, 'b*/> {
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

pub fn test_echo_server(thread_page_map: &'static Map) {
    let ref addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 18080));

    (0..4).map( |_| thread::scoped(move || {
        let srv = TcpSocket::v4().unwrap();
        srv.set_reuseaddr(true).unwrap();
        srv.set_reuseport(true).unwrap();
        srv.set_nodelay(true).unwrap();
        srv.bind(addr).unwrap();
        let mut buf = [0; 2048];
        let mut buf = &mut buf[..];
        // let settings = ParserSettings::new();
        let shared = EchoShared::new(buf, thread_page_map/*, &settings*/);
        let srv = srv.listen(65536).unwrap();

        let mut event_loop = EventLoop::new().unwrap();

        event_loop.register_opt(&srv, SERVER, EventSet::readable(), PollOpt::edge()).unwrap();

        let mut server = EchoServer::new(srv, shared);

        // Start the event loop
        event_loop.run(&mut server).unwrap();
    })).collect::<Vec<_>>();
}
