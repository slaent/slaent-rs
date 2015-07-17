use std::os::unix::io::RawFd;
use std::ptr;
use libc::{c_int, off_t};
use nix::sys::uio::IoVec;
use nix::errno::Errno;
use nix;

#[cfg(target_os = "macos")]
mod ffi {
    #![allow(improper_ctypes)]
    use libc::{c_int, off_t};
    use nix::sys::uio::IoVec;

    #[repr(C)]
    pub struct sf_hdtr<'a> {
        pub headers: *const IoVec<&'a [u8]>,
        pub hdr_cnt: c_int,
        pub trailers: *const IoVec<&'a [u8]>,
        pub trl_cnt: c_int,
    }

    extern {
        pub fn sendfile(fd: c_int, s: c_int, offset: off_t, len: *mut off_t, hdtr: *const sf_hdtr,
                        flags: c_int) -> c_int;
    }
}

pub fn sendfile(fd: RawFd, s: RawFd, offset: u64, len: u64, headers: &[IoVec<&[u8]>], trailers: &[IoVec<&[u8]>]) -> nix::Result<usize> {
    let hdtr = ffi::sf_hdtr {
        headers: if headers.len() == 0 { ptr::null_mut() } else { headers.as_ptr() },
        hdr_cnt: headers.len() as c_int,
        trailers: if trailers.len() == 0 { ptr::null_mut() } else { trailers.as_ptr() },
        trl_cnt: trailers.len() as c_int,
    };
    let mut len = len as i64;
    let res = unsafe {
        ffi::sendfile(fd, s, offset as off_t, &mut len, &hdtr, 0)
    };
    if res < 0 {
        Err(nix::Error::Sys(Errno::last()))
    } else {
        Ok(len as usize)
    }
}

#[cfg(test)]
mod tests {
    use libc::consts::os::bsd44::AF_UNIX;
    use nix::fcntl;
    use nix::sys::socket::{self, AddressFamily, SockType, SockFlag};
    use nix::sys::stat::Mode;
    use nix::sys::uio::*;
    use nix::unistd::*;
    use rand::{thread_rng, Rng};
    use std::{cmp, iter};
    use std::fs::{OpenOptions, remove_file};
    use std::os::unix::io::AsRawFd;
    use std::path::Path;
    
    #[test]
    fn test_sendfile() {
        let s:String = thread_rng().gen_ascii_chars().take(128).collect();
        let to_write = s.as_bytes().to_vec();
        let mut storage = Vec::new();
        let mut allocated = 0;
        while allocated < to_write.len() {
            let left = to_write.len() - allocated;
            let vec_len = if left <= 64 { left } else { thread_rng().gen_range(64, cmp::min(256, left)) };
            let v: Vec<u8> = iter::repeat(0u8).take(vec_len).collect();
            storage.push(v);
            allocated += vec_len;
        }
        let mut iovecs = Vec::with_capacity(storage.len());
        for v in storage.iter_mut() {
            iovecs.push(IoVec::from_slice(&mut v[..]));
        }
        let (reader, writer) = socket::socketpair(AddressFamily::Unix, SockType::Stream, 0, SockFlag::empty()).unwrap();
        // FileDesc will close its filedesc (reader).
        let mut read_buf: Vec<u8> = iter::repeat(0u8).take(128 * 16).collect();
        // Blocking io, should write all data.
        let fd = fcntl::open(Path::new("/etc/hosts"), fcntl::O_RDONLY, Mode::empty()).unwrap();
        let write_res = super::sendfile(fd, writer, 0, 0, &iovecs, &[]);

        // Successful write
        assert!(write_res.is_ok());
        let written = write_res.ok().unwrap();
        // Check whether we written all data
        //assert_eq!(to_write.len(), written);
        let read_res = read(reader, &mut read_buf[..]);
        // Successful read
        assert!(read_res.is_ok());
        let read = read_res.ok().unwrap() as usize;
        // Check we have read as much as we written
        assert_eq!(read, written);
        println!("{:?}", String::from_utf8_lossy(&read_buf[..read]));
        // Check equality of written and read data
        //assert_eq!(&to_write, &read_buf);
        let close_res = close(writer);
        assert!(close_res.is_ok());
        let close_res = close(reader);
        assert!(close_res.is_ok());
        let close_res = close(fd);
        assert!(close_res.is_ok());
    }
}
