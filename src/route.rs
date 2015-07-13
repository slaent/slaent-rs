use common::{PageOff, PostId, TagSlug, ThreadId};
use http_muncher::ParserHandler;
use jetscii::AsciiChars;
use std::str;

#[derive(Debug)]
enum Route {
    Home,
    Favicon,
    ForumPage(TagSlug, PageOff),
    ForumThreadNew(TagSlug),
    Forum(TagSlug),
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

pub struct EchoHandler {
    route: Option<Route>,
}

impl EchoHandler {
    pub fn new() -> Self {
        EchoHandler {
            route: None,
        }
    }
}

impl ParserHandler for EchoHandler {
    fn on_url(&mut self, url: &[u8]) -> bool {
        use self::Route::*;

        let utf8 = |range| unsafe {
            // Must be valid ASCII, or else the parser wouldn't have let it through.
            str::from_utf8_unchecked(&url[range])
        };
        let utf8_f = |range| unsafe {
            // Must be valid ASCII, or else the parser wouldn't have let it through.
            str::from_utf8_unchecked(&url[range])
        };

        macro_rules! p ( ($x:expr) => {{ match $x.parse() {
            Ok(thread) => thread,
            Err(_) => return false
        }}} );

        const SLASH: AsciiChars = AsciiChars::from_words(0x000000000000002f, 0, 1);
        self.route = Some(match url.len() {
            // /
            0...1 => Home,
            len => match url[1] {
                // /forum
                b'f' => if len > 7 {
                    match SLASH.find(utf8(7..)) {
                        Some(i) => {
                            let i = i + 7;
                            let slug = ();//utf8(..i);
                            if i + 1 < len {
                                match url[i + 1] {
                                    b'p' => if i + 6 < len {
                                        if url[len - 1] == b'/' {
                                            ForumPage(slug, p!(utf8_f(i + 6..len - 1)))
                                        } else {
                                            ForumPage(slug, p!(utf8(i + 6..)))
                                        }
                                    } else {
                                        return false
                                    },
                                    b't' => ForumThreadNew(slug),
                                    _ => return false
                                }
                            } else {
                                Forum(slug)
                            }
                        },
                        None => if url[7] == b'n' {
                            Favicon
                        } else {
                            Forum(/*utf8(7..)*/())
                        }
                    }
                } else {
                    return false;
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
                                                            return false
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
                                                        _ => return false
                                                    }
                                                } else {
                                                    ThreadPage(thread, p!(utf8_f(i + 6..i_)))
                                                }
                                            },
                                            None => ThreadPage(thread, p!(utf8(i + 6..)))
                                        }
                                    } else {
                                        return false
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
                                    _ => return false
                                }
                            } else {
                                Thread(thread)
                            }
                        },
                        None => Thread(p!(utf8(8..)))
                    }
                } else {
                    return false
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
                    _ => return false,
                },
                _ => return false,
            }
        });
        //println!("{:?}", route);
        true
    }
}
