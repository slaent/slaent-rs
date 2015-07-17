use Map;
use common::{self, PageOff, PostId, TagSlug, ThreadId};
use http_muncher::{AsciiStr, Parser, ParserHandler};
use jetscii::AsciiChars;
use std::str;
use std::sync::Arc;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Route<'a> {
    Home,
    Favicon,
    ForumPage(TagSlug<'a>, PageOff),
    ForumThreadNew(TagSlug<'a>),
    Forum(TagSlug<'a>),
    ThreadPagePost(ThreadId, PageOff, PostId),
    ThreadPostEdit(ThreadId, PostId),
    ThreadPostHistoryPage(ThreadId, PostId, PageOff),
    ThreadPostHistory(ThreadId, PostId),
    ThreadPage(ThreadId, PageOff),
    ThreadReply(ThreadId),
    ThreadLast(ThreadId),
    ThreadEdit(ThreadId),
    ThreadHistoryPage(ThreadId, PageOff),
    ThreadHistory(ThreadId),
    Thread(ThreadId),
    Robots,
}

/*enum Header {
    Connection,
}

bitflags! {
    flags Connection: u8 {
        const KEEP_ALIVE = 0b00000001,
        const CLOSE = 0b00000010,
    }
}*/

#[derive(Clone,Debug,PartialEq)]
pub enum Response {
    Empty,
    Static(&'static [u8]),
    Body(Arc<String>),
}

pub struct EchoHandler<'a> {
    pub route: Option<Route<'a>>,
    pub response: Option<Response>,
    thread_page_map: &'static Map,
    /*header: Option<Header>,
    pub connection: Connection,*/
}

impl<'a> EchoHandler<'a> {
    pub fn new(thread_page_map: &'static Map) -> Self {
        EchoHandler {
            route: None,
            response: None,
            thread_page_map: thread_page_map,
            /*header: None,
            connection: Connection::empty(),*/
        }
    }
}

impl<'a> ParserHandler<'a> for EchoHandler<'a> {
    fn on_url(&mut self, parser: &Parser, url_ascii: &'a AsciiStr) -> bool {
        use self::Route::*;

        let utf8 = |range| AsciiStr::as_str(&url_ascii[range]);
        let utf8_f = |range| AsciiStr::as_str(&url_ascii[range]);

        macro_rules! p ( ($x:expr) => {{ match $x.parse() {
            Ok(thread) => thread,
            Err(_) => return true
        }}} );

        macro_rules! s ( ($x:expr) => {{ match TagSlug::new($x) {
            Ok(slug) => slug,
            Err(_) => return true
        }}} );

        const SLASH: AsciiChars = AsciiChars::from_words(0x000000000000002f, 0, 1);
        let url = url_ascii.as_bytes();
        self.route = Some(match url.len() {
            // /
            0...1 => Home,
            len => match url[1] {
                // /forum
                b'f' => if len > 7 {
                    match SLASH.find(utf8(7..)) {
                        Some(i) => {
                            let i = i + 7;
                            let slug = url_ascii[7..i].as_str();
                            if i + 1 < len {
                                match url[i + 1] {
                                    b'p' => if i + 6 < len {
                                        if url[len - 1] == b'/' {
                                            ForumPage(s!(slug), p!(utf8_f(i + 6..len - 1)))
                                        } else {
                                            ForumPage(s!(slug), p!(utf8(i + 6..)))
                                        }
                                    } else {
                                        return true
                                    },
                                    b't' => ForumThreadNew(s!(slug)),
                                    _ => return true
                                }
                            } else {
                                Forum(s!(slug))
                            }
                        },
                        None => if url[7] == b'n' {
                            Favicon
                        } else {
                            Forum(s!(utf8(7..)))
                        }
                    }
                } else {
                    return true;
                },
                b't' => if len > 8 {
                    let len = url.len();
                    match SLASH.find(utf8(8..)) {
                        Some(i) => {
                            let i = i + 8;
                            let thread = p!(utf8_f(8..i));
                            if i + 1 < len {
                                match url[i + 1] {
                                    b'p' => if i + 6 < len {
                                        match SLASH.find(utf8(i + 6..)) {
                                            Some(i_) => {
                                                let i_ = i_ + i + 6;
                                                if i_ + 1 < len {
                                                    match url[i_ + 1] {
                                                        b'p' => if i_ + 6 < len {
                                                            let page = p!(utf8_f(i + 6..i_));
                                                            if url[len - 1] == b'/' {
                                                                ThreadPagePost(thread, page, p!(utf8_f(i_ + 6..len - 1)))
                                                            } else {
                                                                ThreadPagePost(thread, page, p!(utf8(i_ + 6..)))
                                                            }
                                                        } else {
                                                            return true
                                                        },
                                                        b'e' => ThreadPostEdit(thread, p!(utf8_f(i + 6..i_))),
                                                        b'h' => if i_ + 14 < len {
                                                            let post = p!(utf8_f(i + 6..i_));
                                                            if url[len - 1] == b'/' {
                                                                ThreadPostHistoryPage(thread, post, p!(utf8_f(i_ + 14..len - 1)))
                                                            } else {
                                                                ThreadPostHistoryPage(thread, post, p!(utf8(i_ + 14..)))
                                                            }
                                                        } else {
                                                            ThreadPostHistory(thread, p!(utf8_f(i + 6..i_)))
                                                        },
                                                        _ => return true
                                                    }
                                                } else {
                                                    ThreadPage(thread, p!(utf8_f(i + 6..i_)))
                                                }
                                            },
                                            None => ThreadPage(thread, p!(utf8(i + 6..)))
                                        }
                                    } else {
                                        return true
                                    },
                                    b'r' => ThreadReply(thread),
                                    b'l' => ThreadLast(thread),
                                    b'e' => ThreadEdit(thread),
                                    b'h' => if i + 14 < len {
                                        if url[len - 1] == b'/' {
                                            ThreadHistoryPage(thread, p!(utf8_f(i + 14..len - 1)))
                                        } else {
                                            ThreadHistoryPage(thread, p!(utf8(i + 14..)))
                                        }
                                    } else {
                                        ThreadHistory(thread)
                                    },
                                    _ => return true
                                }
                            } else {
                                Thread(thread)
                            }
                        },
                        None => Thread(p!(utf8(8..)))
                    }
                } else {
                    return true
                },
  /*url(r'^post/(?P<thread_id>\d+)/reply/$', 'create_reply', name = 'create_post_reply'),
  url(r'^post/preview$', 'preview_post', name = 'preview_post'),
  url(r'^search/(?P<query>[^/]*)/$', 'search_posts', name = 'search_posts'),
  url(r'^search/(?P<query>[^/]*)/page/(?P<page_num>\d+)/$', 'search_posts', name = 'search_posts'),
  url(r'^search/$', 'search_posts', name = 'search_posts'),*/
                /*'p' => match len {
                },
                's' => match len {
                },*/
                b'r' => match len {
                    11 => Robots,
                    _ => return true,
                },
                _ => return true,
            }
        });
        // println!("{:?}", self.route);
        true
    }

    /*fn on_header_field(&mut self, parser: &Parser<Self>, field: &'a AsciiStr) -> bool {
        if !self.connection.is_all() {
            if field == b"Connection" {
                self.header = Some(Header::Connection);
            }
        }
        true
    }

    fn on_header_value(&mut self, parser: &Parser<Self>, val: &'a [u8]) -> bool {
        if let Some(header) = self.header.take() {
            match header {
                Header::Connection => {
                    if val == b"close" {
                        self.connection.insert(CLOSE);
                    } else if val == b"Keep-Alive" {
                        self.connection.insert(KEEP_ALIVE);
                    }
                }
            }
        }
        true
    }*/

    fn on_headers_complete(&mut self, parser: &Parser) -> bool {
        use self::Route::*;

        let route = match self.route {
            Some(ref route) => route,
            None => return false
        };

        self.response = match *route {
            Home => Some(Response::Empty),
            Favicon => Some(Response::Empty),
            ForumPage(_, _) => Some(Response::Empty),
            ForumThreadNew(_) => Some(Response::Empty),
            Forum(_) => Some(Response::Empty),
            ThreadPagePost(_, _, _) => Some(Response::Empty),
            ThreadPostEdit(_, _) => Some(Response::Empty),
            ThreadPostHistoryPage(_, _, _) => Some(Response::Empty),
            ThreadPostHistory(_, _) => Some(Response::Empty),
            ThreadPage(thread_id, page_id) => {
                let thread_page = common::ThreadPage::new(thread_id, page_id);
                self.thread_page_map.find(&thread_page).map(Response::Body)
            }
            ThreadReply(_) => Some(Response::Empty),
            ThreadLast(_) => Some(Response::Empty),
            ThreadEdit(_) => Some(Response::Empty),
            ThreadHistoryPage(_, _) => Some(Response::Empty),
            ThreadHistory(_) => Some(Response::Empty),
            Thread(_) => Some(Response::Empty),
            Robots => Some(Response::Empty),
        };
        true
    }
}
