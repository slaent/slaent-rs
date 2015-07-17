use libc;
use miniz_sys as sys;
use std::io;
use std::mem;
use std::ptr;

pub struct Compressor {
    sys: sys::tdefl_compressor,
}

impl Compressor {
    pub fn new() -> Self {
        const MZ_DEFAULT_WINDOW_BITS: sys::mz_uint = 15;

        unsafe {
            let mut compressor = Compressor {
                sys: mem::uninitialized(),
            };
            compressor.reset();
            compressor
        }
    }

    pub fn reset(&mut self) {
        let flags = 1 /* # hash probes */ | sys::TDEFL_GREEDY_PARSING_FLAG /*| sys::TDEFL_WRITE_ZLIB_HEADER | sys::TDEFL_NONDETERMINISTIC_PARSING_FLAG*/;
        unsafe {
            sys::tdefl_init(&mut self.sys, None, ptr::null_mut(), flags as libc::c_int);
        }
    }

    pub fn compress(&mut self, inp: &[u8], out: &mut [u8]) -> io::Result<usize> {
        let mut input_len = inp.len() as libc::size_t;
        let mut output_len = out.len() as libc::size_t;
        unsafe {
            //FIXME: Don't ignore errors
            sys::tdefl_compress(&mut self.sys, inp.as_ptr() as *const _, &mut input_len,
                                out.as_ptr() as *mut _, &mut output_len, sys::TDEFL_FINISH)
        };
        Ok(output_len as usize)
    }
}
