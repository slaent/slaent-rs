pub use self::post as thread;
pub use self::post::Id as PostId;
pub use self::tag::Slug as TagSlug;
pub use self::thread::Id as ThreadId;
pub use self::page::Off as PageOff;

pub mod tag {
    use std::fmt;
    use std::ops::Deref;

    pub type Raw<'a> = &'a str;

    pub const MAX: usize = 50;

    #[derive(Clone,Copy,Debug,PartialEq)]
    pub struct Slug<'a>(Raw<'a>);

    impl<'a> Slug<'a> {
        pub fn new(slug: Raw<'a>) -> Result<Slug<'a>, SlugOverflow> {
            if slug.len() <= MAX {
                Ok(Slug(slug))
            } else {
                Err(SlugOverflow)
            }
        }
    }

    impl<'a> Deref for Slug<'a> {
        type Target = Raw<'a>;

        fn deref(&self) -> &Raw<'a> {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct SlugOverflow;

    impl fmt::Display for SlugOverflow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SlugOverflow")
        }
    }
}

#[allow(unused_variables)]
pub mod post {
    use std::fmt;
    use std::num;
    use std::ops::Deref;
    use std::str::FromStr;
    pub use std::u64 as raw;

    pub type Raw = u64;

    pub const BITS: usize = 48;

    // 15 decimal digits
    // pub const DIGITS: usize = 15;

    pub const MAX: Raw = raw::MAX >> (raw::BITS - BITS);

    #[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
    pub struct Id(Raw);

    impl Id {
        pub fn new(id: Raw) -> Result<Self, IdOverflow> {
            if id <= MAX {
                Ok(Id(id))
            } else {
                Err(IdOverflow)
            }
        }
    }

    impl Deref for Id {
        type Target = Raw;

        fn deref(&self) -> &Raw {
            &self.0
        }
    }

    #[derive(Debug)]
    pub struct IdOverflow;

    impl fmt::Display for IdOverflow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "IdOverflow")
        }
    }

    error_type! {
        #[derive(Debug)]
        pub enum ParseError {
            Parse(/*<Raw as FromStr>::Err*/num::ParseIntError) {
                cause;
            },
            Overflow(IdOverflow) {
                desc(e) concat!("number too large to be a valid post ID");
            },
        }
    }

    impl FromStr for Id {
        type Err = ParseError;

        fn from_str(src: &str) -> Result<Self, ParseError> {
            let id = try!(src.parse());
            Ok(try!(Id::new(id)))
        }
    }
}

pub mod page {
    //use std::fmt;
    use std::ops::Deref;
    use std::str::FromStr;
    pub use std::u16 as raw;

    pub type Raw = u16;

    pub const BITS: usize = 16;

    // 5 decimal digits
    // pub const DIGITS: usize = 5;

    // pub const MAX: Raw = raw::MAX >> (raw::BITS - BITS);

    #[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
    pub struct Off(Raw);

    impl Off {
        pub fn new(off: Raw) -> /*Result<*/Self/*, OffOverflow>*/ {
            /*if off <= MAX {
                Ok(*/Off(off)/*)
            } else {
                Err(OffOverflow)
            }*/
        }
    }

    impl Deref for Off {
        type Target = Raw;

        fn deref(&self) -> &Raw {
            &self.0
        }
    }

    /*#[derive(Debug)]
    pub struct OffOverflow;

    impl fmt::Display for OffOverflow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "OffOverflow")
        }
    }*/

    /*error_type! {
        #[derive(Debug)]
        pub enum ParseError {
            Parse(<Raw as FromStr>::Err) {
                cause;
            },
            Overflow(OffOverflow) {
                desc(e) concat!("number too large to be a valid page offset");
            },
        }
    }*/
    pub type ParseError = <Raw as FromStr>::Err;

    impl FromStr for Off {
        type Err = ParseError;

        fn from_str(src: &str) -> Result<Self, ParseError> {
            let off = try!(src.parse());
            Ok(/*try!(*/Off::new(off)/*)*/)
        }
    }
}

/// Maximum number of posts per page
//pub const POSTS_PER_PAGE: post::Raw = 50;

/// Maximum number of posts per thread
//pub const POSTS_PER_THREAD: post::Raw = POSTS_PER_PAGE * page::MAX as post::Raw;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct ThreadPage(u64);

impl ThreadPage {
    pub fn new(thread: ThreadId, page: PageOff) -> Self {
        ThreadPage(*thread << page::BITS | *page as post::Raw)
    }
}


/*#[cfg(test)]
mod tests {
    use num::NumCast;
    use std::usize;
    use super::*;

    #[test]
    fn check_bounds() {
        // Make sure page::MAX fits in a post::Raw.  This should always be true since there can't
        // be more posts on a page than posts overall.
        let page_max = post::Raw::cast(page::MAX).unwrap();

        // Make sure POSTS_PER_THREAD fits in post::Raw.  This should always be true since there
        // can't be more posts in a thread than posts overall.
        assert_eq!(POSTS_PER_PAGE.checked_mul(page_max).unwrap(), POSTS_PER_THREAD);

        // POSTS_PER_THREAD should be <= post::MAX, since a thread with n pages has n *
        // POSTS_PER_PAGE posts, and there cannot be more pages than posts.
        assert!(POSTS_PER_THREAD <= post::MAX);

        // POSTS_PER_PAGE should be <= usize::MAX, since we may need to store a vector of posts on
        // a page.
        assert!(POSTS_PER_PAGE <= usize::MAX);

        // POSTS_PER_THREAD should be <= thread_page::MAX, since a thread with n pages has n *
        // POSTS_PER_PAGE posts, and there cannot be more pages than posts.
    }
}*/
