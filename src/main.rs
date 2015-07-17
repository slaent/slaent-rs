#![feature(const_fn)]
#![feature(hashmap_hasher)]
#![feature(num_bits_bytes)]
#![feature(plugin)]
#![plugin(postgres_macros)]
#![feature(range_inclusive)]
#![feature(result_expect)]
#![feature(scoped)]
#![plugin(maud_macros)]

#[macro_use] extern crate bitflags;
extern crate cuckoo;
#[macro_use] extern crate error_type;
#[macro_use] extern crate fallthrough;
extern crate flate2;
extern crate http_muncher;
//extern crate iron;
extern crate jetscii;
extern crate libc;
extern crate mio;
extern crate nix;
#[cfg(test)] extern crate num;
//#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate rand;
//extern crate router;

use common::ThreadPage;
use cuckoo::CuckooHashMap;
use cuckoo::nodemap::FnvHasher;
use flate2::write::GzEncoder;
//use iron::prelude::*;
//use iron::status::{self, Status};
//use router::Router;
use std::cell::Cell;
use std::collections::hash_state::DefaultState;
use std::error::Error;
use std::io::{Read, Write};
use std::sync::Arc;

mod common;
mod fast_echo;
#[macro_use] mod lazy_static;
mod miniz;
mod miniz_sys;
mod route;
//mod sys;

const THREAD_ID_PARAM: &'static str = "thread_id";
const PAGE_ID_PARAM: &'static str = "page_id";

thread_local!(static PAGE_ID: Cell<common::page::Raw> = Cell::new(0));

pub type Key = ThreadPage;

pub type Val = Arc<String>;

//pub type Map = CuckooHashMap<Key, Val>;
pub type Map = CuckooHashMap<Key, Val, DefaultState<FnvHasher>>;

lazy_static! {
    //pub static ref THREAD_PAGE_MAP: Map = CuckooHashMap::new();
    pub static ref THREAD_PAGE_MAP: Map = CuckooHashMap::default();
}

/*fn err<E: Error + Send + 'static>(e: E, s: Status) -> IronError {
    /*let description = format!("{}", e.description());
    IronError::new(e, (s, description))*/
    IronError::new(e, s)
}

fn thread_page<'a, 'b>(req: &'a mut Request, thread_page_map: &'b Map) -> IronResult<Response> {
    /*let router = req.extensions.get::<Router>().unwrap();
    let thread_id = try!(router.find(THREAD_ID_PARAM).unwrap().parse().map_err( |e| err(e, status::BadRequest)));*/
    //let page_id = try!(router.find(PAGE_ID_PARAM).unwrap().parse().map_err( |e| err(e, status::BadRequest)));
    let thread_id = try!(common::ThreadId::new(1).map_err( |e| err(common::thread::ParseError::from(e), status::BadRequest)));
    let page_id = common::PageOff::new(PAGE_ID.with(|v| {
        let page_id = v.get();
        v.set(page_id.wrapping_add(1));
        page_id
    }));
    let thread_page = ThreadPage::new(thread_id, page_id);
    /*let query = sql!(concat!("SELECT p.id, b.title, b.body, p.date, d.post_id AS thread_id, b.id AS blob_id, u.id AS uid, u.username, u.tag AS user_tag, i.image AS user_avatar
FROM (
  SELECT post_id, descendent_id
  FROM posts_visible_post_descendents d
  WHERE d.post_id = $1
  ORDER BY descendent_id
  LIMIT $2", "OFFSET $3
) AS d
JOIN posts_post p ON p.id = d.descendent_id
JOIN posts_revision_blob b ON p.blob_id = b.id
JOIN profiles_userprofile u ON u.id = p.owner_id
JOIN images_image i ON u.avatar_id = i.id
ORDER BY descendent_id;"));*/
    match thread_page_map.find(&thread_page) {
        Some(body) => {
            Ok(Response::with((status::Ok, &**body)))
        },
        None => Ok(Response::with(status::NotFound))
    }
}

fn thread_page_insert<'a, 'b>(req: &'a mut Request, thread_page_map: &'b Map) -> IronResult<Response> {
    /*let router = req.extensions.get::<Router>().unwrap();
    let thread_id = try!(router.find(THREAD_ID_PARAM).unwrap().parse().map_err( |e| err(e, status::BadRequest)));*/
    let thread_id = try!(common::ThreadId::new(1).map_err( |e| err(common::thread::ParseError::from(e), status::BadRequest)));
    //let page_id = try!(router.find(PAGE_ID_PARAM).unwrap().parse().map_err( |e| err(e, status::BadRequest)));
    let page_id = common::PageOff::new(PAGE_ID.with(|v| {
        let page_id = v.get();
        v.set(page_id.wrapping_add(1));
        page_id
    }));
    let mut body = String::new();
    req.body.read_to_string(&mut body);
    let thread_page = ThreadPage::new(thread_id, page_id);
    match thread_page_map.insert(thread_page, Arc::new(body.into())) {
        Ok(()) => {
            Ok(Response::with(status::Created))
        },
        Err(_) => Ok(Response::with(status::Conflict))
    }
}*/

fn main() {
    const NUM_THREADS: usize = 1;

    // let mut router = Router::new();
    let thread_page_map: &'static Map = THREAD_PAGE_MAP.get();
    /*router.get(/*format!("/thread/:{}/page/:{}", THREAD_ID_PARAM, PAGE_ID_PARAM)*/"/",
               move |req: &mut Request| thread_page(req, thread_page_map));
    router.post(/*format!("/thread/:{}/page/:{}", THREAD_ID_PARAM, PAGE_ID_PARAM)*/"/",
               move |req: &mut Request| thread_page_insert(req, thread_page_map));*/
    /*Iron::new(/*router*/
    |_: &mut Request| {
        Ok(Response::with((status::Ok)))
    }).listen_with("localhost:3000", NUM_THREADS, iron::Protocol::Http).unwrap();*/
    let home = Arc::<String>::new(include_str!("../static/slaent.html").into());
    for thread_id in (0..1) {
        for page_off in ::std::iter::range_inclusive(0u16, 65535) {
            let body = if page_off == 0 {
                home.clone()
            } else {
                let body = ::std::iter::repeat(&*format!("test thread {} page {}", thread_id, page_off)).take(1500).collect::<String>();
                //let mut gzip = GzEncoder::new(Vec::new(), flate2::Compression::Best);
                //write!(gzip, "{}", body).unwrap();
                //let body = gzip.finish().unwrap();
                Arc::new(body)
            };
            let thread_id = common::ThreadId::new(thread_id).unwrap();
            let page_off = common::PageOff::new(page_off);
            let thread_page = ThreadPage::new(thread_id, page_off);
            thread_page_map.insert(thread_page, body).ok().unwrap();
        }
    }
    fast_echo::test_echo_server(thread_page_map);
}
