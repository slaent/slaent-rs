use std::mem;
use std::sync::{Once, ONCE_INIT};
use slz_sys::*;

/// Used to determine whether the dist table has already been initialized.
/// Must *only* be called from Slz::init().
static DIST_TABLE: Once = ONCE_INIT;


/// Represents the initialized Slz library.
#[derive(Copy,Clone)]
pub struct Slz(());

/// Represents a deflate stream 
pub struct DeflateStream(Struct_slz_stream);

impl Slz {
    /// Initialize SLZ library.  This is a requirement before any compression may be performed.
    pub fn init() -> Self {
        DIST_TABLE.call_once(|| unsafe {
            slz_prepare_dist_table();
        });
        Slz(())
    }

    /// Initializes a new deflate stream.
    ///
    /// `should_compress`: if false, no compression will actually be performed.
    pub fn deflate(&self, should_compress: bool) -> DeflateStream {
        unsafe {
            let mut stream = mem::uninitialized();
            slz_rfc1951_init(&mut stream, if should_compress { 1 } else { 0 });
            DeflateStream(stream)
        }
    }
}

impl DeflateStream {
    /// Compresses bytes from `inp` to `out` according to the deflate algorithm.
    /// `more` means there may be additional input later.
    ///
    /// The output may be up to 5 bytes larger than the input (7, if `more` is false).
    /// Failure to enforce this will cause a panic when the function is called.
    ///
    /// Returns the number of output bytes.
    pub fn encode(&mut self, out: &mut [u8], inp: &[u8], more: bool) -> usize {
        unsafe {
            let diff_len = if more { 0 } else { 2 } + 5;
            // Can safely assume neither inp nor out have lengths exceeding 7, since there would
            // be no room for the Struct_slz_stream otherwise.
            assert!(out.len() >= inp.len() + diff_len,
                    "Not enough space in output buffer: {} bytes required, but only {} bytes allocated.",
                    inp.len() + diff_len, out.len());
            // Implicitly assumes inp.len() <= i64::MAX, which is probably wrong...
            slz_rfc1951_encode(&mut self.0, out.as_mut_ptr(), inp.as_ptr(), inp.len() as i64,
                               if more {1 } else { 0 }) as usize
        }
    }

    /// Flushes any pending bytes into `buf`.
    ///
    /// Returns the number of bytes written.
    pub fn finish(&mut self, buf: &mut [u8; 4]) -> usize {
        unsafe {
            slz_rfc1951_finish(&mut self.0, buf.as_mut_ptr()) as usize
        }
    }
}
