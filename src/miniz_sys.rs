#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/* automatically generated by rust-bindgen */
use libc::{size_t, time_t, FILE};

pub type mz_ulong = ::libc::c_ulong;
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const MZ_DEFAULT_STRATEGY: ::libc::c_uint = 0;
pub const MZ_FILTERED: ::libc::c_uint = 1;
pub const MZ_HUFFMAN_ONLY: ::libc::c_uint = 2;
pub const MZ_RLE: ::libc::c_uint = 3;
pub const MZ_FIXED: ::libc::c_uint = 4;
pub type mz_alloc_func =
    ::std::option::Option<extern "C" fn(opaque: *mut ::libc::c_void,
                                        items: size_t, size: size_t)
                              -> *mut ::libc::c_void>;
pub type mz_free_func =
    ::std::option::Option<extern "C" fn(opaque: *mut ::libc::c_void,
                                        address: *mut ::libc::c_void) -> ()>;
pub type mz_realloc_func =
    ::std::option::Option<extern "C" fn(opaque: *mut ::libc::c_void,
                                        address: *mut ::libc::c_void,
                                        items: size_t, size: size_t)
                              -> *mut ::libc::c_void>;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const MZ_NO_FLUSH: ::libc::c_uint = 0;
pub const MZ_PARTIAL_FLUSH: ::libc::c_uint = 1;
pub const MZ_SYNC_FLUSH: ::libc::c_uint = 2;
pub const MZ_FULL_FLUSH: ::libc::c_uint = 3;
pub const MZ_FINISH: ::libc::c_uint = 4;
pub const MZ_BLOCK: ::libc::c_uint = 5;
pub type Enum_Unnamed3 = ::libc::c_int;
pub const MZ_OK: ::libc::c_int = 0;
pub const MZ_STREAM_END: ::libc::c_int = 1;
pub const MZ_NEED_DICT: ::libc::c_int = 2;
pub const MZ_ERRNO: ::libc::c_int = -1;
pub const MZ_STREAM_ERROR: ::libc::c_int = -2;
pub const MZ_DATA_ERROR: ::libc::c_int = -3;
pub const MZ_MEM_ERROR: ::libc::c_int = -4;
pub const MZ_BUF_ERROR: ::libc::c_int = -5;
pub const MZ_VERSION_ERROR: ::libc::c_int = -6;
pub const MZ_PARAM_ERROR: ::libc::c_int = -10000;
pub type Enum_Unnamed4 = ::libc::c_int;
pub const MZ_NO_COMPRESSION: ::libc::c_int = 0;
pub const MZ_BEST_SPEED: ::libc::c_int = 1;
pub const MZ_BEST_COMPRESSION: ::libc::c_int = 9;
pub const MZ_UBER_COMPRESSION: ::libc::c_int = 10;
pub const MZ_DEFAULT_LEVEL: ::libc::c_int = 6;
pub const MZ_DEFAULT_COMPRESSION: ::libc::c_int = -1;
pub enum Struct_mz_internal_state { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mz_stream_s {
    pub next_in: *const ::libc::c_uchar,
    pub avail_in: ::libc::c_uint,
    pub total_in: mz_ulong,
    pub next_out: *mut ::libc::c_uchar,
    pub avail_out: ::libc::c_uint,
    pub total_out: mz_ulong,
    pub msg: *mut ::libc::c_char,
    pub state: *mut Struct_mz_internal_state,
    pub zalloc: mz_alloc_func,
    pub zfree: mz_free_func,
    pub opaque: *mut ::libc::c_void,
    pub data_type: ::libc::c_int,
    pub adler: mz_ulong,
    pub reserved: mz_ulong,
}
impl ::std::clone::Clone for Struct_mz_stream_s {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_mz_stream_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mz_stream = Struct_mz_stream_s;
pub type mz_streamp = *mut mz_stream;
pub type Byte = ::libc::c_uchar;
pub type uInt = ::libc::c_uint;
pub type uLong = mz_ulong;
pub type Bytef = Byte;
pub type uIntf = uInt;
pub type charf = ::libc::c_char;
pub type intf = ::libc::c_int;
pub type voidpf = *mut ::libc::c_void;
pub type uLongf = uLong;
pub type voidp = *mut ::libc::c_void;
pub type voidpc = *mut ::libc::c_void;
pub type mz_uint8 = ::libc::c_uchar;
pub type mz_int16 = ::libc::c_short;
pub type mz_uint16 = ::libc::c_ushort;
pub type mz_uint32 = ::libc::c_uint;
pub type mz_uint = ::libc::c_uint;
pub type mz_int64 = ::libc::c_longlong;
pub type mz_uint64 = ::libc::c_ulonglong;
pub type mz_bool = ::libc::c_int;
pub type Enum_Unnamed5 = ::libc::c_uint;
pub const MZ_ZIP_MAX_IO_BUF_SIZE: ::libc::c_uint = 65536;
pub const MZ_ZIP_MAX_ARCHIVE_FILENAME_SIZE: ::libc::c_uint = 260;
pub const MZ_ZIP_MAX_ARCHIVE_FILE_COMMENT_SIZE: ::libc::c_uint = 256;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub m_file_index: mz_uint32,
    pub m_central_dir_ofs: mz_uint32,
    pub m_version_made_by: mz_uint16,
    pub m_version_needed: mz_uint16,
    pub m_bit_flag: mz_uint16,
    pub m_method: mz_uint16,
    pub m_time: time_t,
    pub m_crc32: mz_uint32,
    pub m_comp_size: mz_uint64,
    pub m_uncomp_size: mz_uint64,
    pub m_internal_attr: mz_uint16,
    pub m_external_attr: mz_uint32,
    pub m_local_header_ofs: mz_uint64,
    pub m_comment_size: mz_uint32,
    pub m_filename: [::libc::c_char; 260usize],
    pub m_comment: [::libc::c_char; 256usize],
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mz_zip_archive_file_stat = Struct_Unnamed6;
pub type mz_file_read_func =
    ::std::option::Option<extern "C" fn(pOpaque: *mut ::libc::c_void,
                                        file_ofs: mz_uint64,
                                        pBuf: *mut ::libc::c_void, n: size_t)
                              -> size_t>;
pub type mz_file_write_func =
    ::std::option::Option<extern "C" fn(pOpaque: *mut ::libc::c_void,
                                        file_ofs: mz_uint64,
                                        pBuf: *const ::libc::c_void,
                                        n: size_t) -> size_t>;
pub type mz_zip_internal_state = Struct_mz_zip_internal_state_tag;
pub type Enum_Unnamed7 = ::libc::c_uint;
pub const MZ_ZIP_MODE_INVALID: ::libc::c_uint = 0;
pub const MZ_ZIP_MODE_READING: ::libc::c_uint = 1;
pub const MZ_ZIP_MODE_WRITING: ::libc::c_uint = 2;
pub const MZ_ZIP_MODE_WRITING_HAS_BEEN_FINALIZED: ::libc::c_uint = 3;
pub type mz_zip_mode = Enum_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mz_zip_archive_tag {
    pub m_archive_size: mz_uint64,
    pub m_central_directory_file_ofs: mz_uint64,
    pub m_total_files: mz_uint,
    pub m_zip_mode: mz_zip_mode,
    pub m_file_offset_alignment: mz_uint,
    pub m_pAlloc: mz_alloc_func,
    pub m_pFree: mz_free_func,
    pub m_pRealloc: mz_realloc_func,
    pub m_pAlloc_opaque: *mut ::libc::c_void,
    pub m_pRead: mz_file_read_func,
    pub m_pWrite: mz_file_write_func,
    pub m_pIO_opaque: *mut ::libc::c_void,
    pub m_pState: *mut mz_zip_internal_state,
}
impl ::std::clone::Clone for Struct_mz_zip_archive_tag {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_mz_zip_archive_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mz_zip_archive = Struct_mz_zip_archive_tag;
pub type Enum_Unnamed8 = ::libc::c_uint;
pub const MZ_ZIP_FLAG_CASE_SENSITIVE: ::libc::c_uint = 256;
pub const MZ_ZIP_FLAG_IGNORE_PATH: ::libc::c_uint = 512;
pub const MZ_ZIP_FLAG_COMPRESSED_DATA: ::libc::c_uint = 1024;
pub const MZ_ZIP_FLAG_DO_NOT_SORT_CENTRAL_DIRECTORY: ::libc::c_uint = 2048;
pub type mz_zip_flags = Enum_Unnamed8;
pub type Enum_Unnamed9 = ::libc::c_uint;
pub const TINFL_FLAG_PARSE_ZLIB_HEADER: ::libc::c_uint = 1;
pub const TINFL_FLAG_HAS_MORE_INPUT: ::libc::c_uint = 2;
pub const TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF: ::libc::c_uint = 4;
pub const TINFL_FLAG_COMPUTE_ADLER32: ::libc::c_uint = 8;
pub type tinfl_put_buf_func_ptr =
    ::std::option::Option<extern "C" fn(pBuf: *const ::libc::c_void,
                                        len: ::libc::c_int,
                                        pUser: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type tinfl_decompressor = Struct_tinfl_decompressor_tag;
pub type Enum_Unnamed10 = ::libc::c_int;
pub const TINFL_STATUS_BAD_PARAM: ::libc::c_int = -3;
pub const TINFL_STATUS_ADLER32_MISMATCH: ::libc::c_int = -2;
pub const TINFL_STATUS_FAILED: ::libc::c_int = -1;
pub const TINFL_STATUS_DONE: ::libc::c_int = 0;
pub const TINFL_STATUS_NEEDS_MORE_INPUT: ::libc::c_int = 1;
pub const TINFL_STATUS_HAS_MORE_OUTPUT: ::libc::c_int = 2;
pub type tinfl_status = Enum_Unnamed10;
pub type Enum_Unnamed11 = ::libc::c_uint;
pub const TINFL_MAX_HUFF_TABLES: ::libc::c_uint = 3;
pub const TINFL_MAX_HUFF_SYMBOLS_0: ::libc::c_uint = 288;
pub const TINFL_MAX_HUFF_SYMBOLS_1: ::libc::c_uint = 32;
pub const TINFL_MAX_HUFF_SYMBOLS_2: ::libc::c_uint = 19;
pub const TINFL_FAST_LOOKUP_BITS: ::libc::c_uint = 10;
pub const TINFL_FAST_LOOKUP_SIZE: ::libc::c_uint = 1024;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub m_code_size: [mz_uint8; 288usize],
    pub m_look_up: [mz_int16; 1024usize],
    pub m_tree: [mz_int16; 576usize],
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type tinfl_huff_table = Struct_Unnamed12;
pub type tinfl_bit_buf_t = mz_uint64;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tinfl_decompressor_tag {
    pub m_state: mz_uint32,
    pub m_num_bits: mz_uint32,
    pub m_zhdr0: mz_uint32,
    pub m_zhdr1: mz_uint32,
    pub m_z_adler32: mz_uint32,
    pub m_final: mz_uint32,
    pub m_type: mz_uint32,
    pub m_check_adler32: mz_uint32,
    pub m_dist: mz_uint32,
    pub m_counter: mz_uint32,
    pub m_num_extra: mz_uint32,
    pub m_table_sizes: [mz_uint32; 3usize],
    pub m_bit_buf: tinfl_bit_buf_t,
    pub m_dist_from_out_buf_start: size_t,
    pub m_tables: [tinfl_huff_table; 3usize],
    pub m_raw_header: [mz_uint8; 4usize],
    pub m_len_codes: [mz_uint8; 457usize],
}
impl ::std::clone::Clone for Struct_tinfl_decompressor_tag {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_tinfl_decompressor_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_Unnamed13 = ::libc::c_uint;
pub const TDEFL_HUFFMAN_ONLY: ::libc::c_uint = 0;
pub const TDEFL_DEFAULT_MAX_PROBES: ::libc::c_uint = 128;
pub const TDEFL_MAX_PROBES_MASK: ::libc::c_uint = 4095;
pub type Enum_Unnamed14 = ::libc::c_uint;
pub const TDEFL_WRITE_ZLIB_HEADER: ::libc::c_uint = 4096;
pub const TDEFL_COMPUTE_ADLER32: ::libc::c_uint = 8192;
pub const TDEFL_GREEDY_PARSING_FLAG: ::libc::c_uint = 16384;
pub const TDEFL_NONDETERMINISTIC_PARSING_FLAG: ::libc::c_uint = 32768;
pub const TDEFL_RLE_MATCHES: ::libc::c_uint = 65536;
pub const TDEFL_FILTER_MATCHES: ::libc::c_uint = 131072;
pub const TDEFL_FORCE_ALL_STATIC_BLOCKS: ::libc::c_uint = 262144;
pub const TDEFL_FORCE_ALL_RAW_BLOCKS: ::libc::c_uint = 524288;
pub type tdefl_put_buf_func_ptr =
    ::std::option::Option<extern "C" fn(pBuf: *const ::libc::c_void,
                                        len: ::libc::c_int,
                                        pUser: *mut ::libc::c_void)
                              -> mz_bool>;
pub type Enum_Unnamed15 = ::libc::c_uint;
pub const TDEFL_MAX_HUFF_TABLES: ::libc::c_uint = 3;
pub const TDEFL_MAX_HUFF_SYMBOLS_0: ::libc::c_uint = 288;
pub const TDEFL_MAX_HUFF_SYMBOLS_1: ::libc::c_uint = 32;
pub const TDEFL_MAX_HUFF_SYMBOLS_2: ::libc::c_uint = 19;
pub const TDEFL_LZ_DICT_SIZE: ::libc::c_uint = 32768;
pub const TDEFL_LZ_DICT_SIZE_MASK: ::libc::c_uint = 32767;
pub const TDEFL_MIN_MATCH_LEN: ::libc::c_uint = 3;
pub const TDEFL_MAX_MATCH_LEN: ::libc::c_uint = 258;
pub type Enum_Unnamed16 = ::libc::c_uint;
pub const TDEFL_LZ_CODE_BUF_SIZE: ::libc::c_uint = 65536;
pub const TDEFL_OUT_BUF_SIZE: ::libc::c_uint = 85196;
pub const TDEFL_MAX_HUFF_SYMBOLS: ::libc::c_uint = 288;
pub const TDEFL_LZ_HASH_BITS: ::libc::c_uint = 15;
pub const TDEFL_LEVEL1_HASH_SIZE_MASK: ::libc::c_uint = 4095;
pub const TDEFL_LZ_HASH_SHIFT: ::libc::c_uint = 5;
pub const TDEFL_LZ_HASH_SIZE: ::libc::c_uint = 32768;
pub type Enum_Unnamed17 = ::libc::c_int;
pub const TDEFL_STATUS_BAD_PARAM: ::libc::c_int = -2;
pub const TDEFL_STATUS_PUT_BUF_FAILED: ::libc::c_int = -1;
pub const TDEFL_STATUS_OKAY: ::libc::c_int = 0;
pub const TDEFL_STATUS_DONE: ::libc::c_int = 1;
pub type tdefl_status = Enum_Unnamed17;
pub type Enum_Unnamed18 = ::libc::c_uint;
pub const TDEFL_NO_FLUSH: ::libc::c_uint = 0;
pub const TDEFL_SYNC_FLUSH: ::libc::c_uint = 2;
pub const TDEFL_FULL_FLUSH: ::libc::c_uint = 3;
pub const TDEFL_FINISH: ::libc::c_uint = 4;
pub type tdefl_flush = Enum_Unnamed18;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed19 {
    pub m_pPut_buf_func: tdefl_put_buf_func_ptr,
    pub m_pPut_buf_user: *mut ::libc::c_void,
    pub m_flags: mz_uint,
    pub m_max_probes: [mz_uint; 2usize],
    pub m_greedy_parsing: ::libc::c_int,
    pub m_adler32: mz_uint,
    pub m_lookahead_pos: mz_uint,
    pub m_lookahead_size: mz_uint,
    pub m_dict_size: mz_uint,
    pub m_pLZ_code_buf: *mut mz_uint8,
    pub m_pLZ_flags: *mut mz_uint8,
    pub m_pOutput_buf: *mut mz_uint8,
    pub m_pOutput_buf_end: *mut mz_uint8,
    pub m_num_flags_left: mz_uint,
    pub m_total_lz_bytes: mz_uint,
    pub m_lz_code_buf_dict_pos: mz_uint,
    pub m_bits_in: mz_uint,
    pub m_bit_buffer: mz_uint,
    pub m_saved_match_dist: mz_uint,
    pub m_saved_match_len: mz_uint,
    pub m_saved_lit: mz_uint,
    pub m_output_flush_ofs: mz_uint,
    pub m_output_flush_remaining: mz_uint,
    pub m_finished: mz_uint,
    pub m_block_index: mz_uint,
    pub m_wants_to_finish: mz_uint,
    pub m_prev_return_status: tdefl_status,
    pub m_pIn_buf: *const ::libc::c_void,
    pub m_pOut_buf: *mut ::libc::c_void,
    pub m_pIn_buf_size: *mut size_t,
    pub m_pOut_buf_size: *mut size_t,
    pub m_flush: tdefl_flush,
    pub m_pSrc: *const mz_uint8,
    pub m_src_buf_left: size_t,
    pub m_out_buf_ofs: size_t,
    pub m_dict: [mz_uint8; 33025usize],
    pub m_huff_count: [[mz_uint16; 288usize]; 3usize],
    pub m_huff_codes: [[mz_uint16; 288usize]; 3usize],
    pub m_huff_code_sizes: [[mz_uint8; 288usize]; 3usize],
    pub m_lz_code_buf: [mz_uint8; 65536usize],
    pub m_next: [mz_uint16; 32768usize],
    pub m_hash: [mz_uint16; 32768usize],
    pub m_output_buf: [mz_uint8; 85196usize],
}
impl ::std::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed19 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type tdefl_compressor = Struct_Unnamed19;
pub type mz_validate_uint16 = [::libc::c_uchar; 1usize];
pub type mz_validate_uint32 = [::libc::c_uchar; 1usize];
pub type mz_validate_uint64 = [::libc::c_uchar; 1usize];
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed20 {
    pub m_decomp: tinfl_decompressor,
    pub m_dict_ofs: mz_uint,
    pub m_dict_avail: mz_uint,
    pub m_first_call: mz_uint,
    pub m_has_flushed: mz_uint,
    pub m_window_bits: ::libc::c_int,
    pub m_dict: [mz_uint8; 32768usize],
    pub m_last_status: tinfl_status,
}
impl ::std::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed20 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type inflate_state = Struct_Unnamed20;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed21 {
    pub m_key: mz_uint16,
    pub m_sym_index: mz_uint16,
}
impl ::std::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed21 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type tdefl_sym_freq = Struct_Unnamed21;
pub type Enum_Unnamed22 = ::libc::c_uint;
pub const TDEFL_MAX_SUPPORTED_HUFF_CODESIZE: ::libc::c_uint = 32;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed23 {
    pub m_size: size_t,
    pub m_capacity: size_t,
    pub m_pBuf: *mut mz_uint8,
    pub m_expandable: mz_bool,
}
impl ::std::clone::Clone for Struct_Unnamed23 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed23 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type tdefl_output_buffer = Struct_Unnamed23;
pub type Enum_Unnamed24 = ::libc::c_uint;
pub const MZ_ZIP_END_OF_CENTRAL_DIR_HEADER_SIG: ::libc::c_uint = 101010256;
pub const MZ_ZIP_CENTRAL_DIR_HEADER_SIG: ::libc::c_uint = 33639248;
pub const MZ_ZIP_LOCAL_DIR_HEADER_SIG: ::libc::c_uint = 67324752;
pub const MZ_ZIP_LOCAL_DIR_HEADER_SIZE: ::libc::c_uint = 30;
pub const MZ_ZIP_CENTRAL_DIR_HEADER_SIZE: ::libc::c_uint = 46;
pub const MZ_ZIP_END_OF_CENTRAL_DIR_HEADER_SIZE: ::libc::c_uint = 22;
pub const MZ_ZIP_CDH_SIG_OFS: ::libc::c_uint = 0;
pub const MZ_ZIP_CDH_VERSION_MADE_BY_OFS: ::libc::c_uint = 4;
pub const MZ_ZIP_CDH_VERSION_NEEDED_OFS: ::libc::c_uint = 6;
pub const MZ_ZIP_CDH_BIT_FLAG_OFS: ::libc::c_uint = 8;
pub const MZ_ZIP_CDH_METHOD_OFS: ::libc::c_uint = 10;
pub const MZ_ZIP_CDH_FILE_TIME_OFS: ::libc::c_uint = 12;
pub const MZ_ZIP_CDH_FILE_DATE_OFS: ::libc::c_uint = 14;
pub const MZ_ZIP_CDH_CRC32_OFS: ::libc::c_uint = 16;
pub const MZ_ZIP_CDH_COMPRESSED_SIZE_OFS: ::libc::c_uint = 20;
pub const MZ_ZIP_CDH_DECOMPRESSED_SIZE_OFS: ::libc::c_uint = 24;
pub const MZ_ZIP_CDH_FILENAME_LEN_OFS: ::libc::c_uint = 28;
pub const MZ_ZIP_CDH_EXTRA_LEN_OFS: ::libc::c_uint = 30;
pub const MZ_ZIP_CDH_COMMENT_LEN_OFS: ::libc::c_uint = 32;
pub const MZ_ZIP_CDH_DISK_START_OFS: ::libc::c_uint = 34;
pub const MZ_ZIP_CDH_INTERNAL_ATTR_OFS: ::libc::c_uint = 36;
pub const MZ_ZIP_CDH_EXTERNAL_ATTR_OFS: ::libc::c_uint = 38;
pub const MZ_ZIP_CDH_LOCAL_HEADER_OFS: ::libc::c_uint = 42;
pub const MZ_ZIP_LDH_SIG_OFS: ::libc::c_uint = 0;
pub const MZ_ZIP_LDH_VERSION_NEEDED_OFS: ::libc::c_uint = 4;
pub const MZ_ZIP_LDH_BIT_FLAG_OFS: ::libc::c_uint = 6;
pub const MZ_ZIP_LDH_METHOD_OFS: ::libc::c_uint = 8;
pub const MZ_ZIP_LDH_FILE_TIME_OFS: ::libc::c_uint = 10;
pub const MZ_ZIP_LDH_FILE_DATE_OFS: ::libc::c_uint = 12;
pub const MZ_ZIP_LDH_CRC32_OFS: ::libc::c_uint = 14;
pub const MZ_ZIP_LDH_COMPRESSED_SIZE_OFS: ::libc::c_uint = 18;
pub const MZ_ZIP_LDH_DECOMPRESSED_SIZE_OFS: ::libc::c_uint = 22;
pub const MZ_ZIP_LDH_FILENAME_LEN_OFS: ::libc::c_uint = 26;
pub const MZ_ZIP_LDH_EXTRA_LEN_OFS: ::libc::c_uint = 28;
pub const MZ_ZIP_ECDH_SIG_OFS: ::libc::c_uint = 0;
pub const MZ_ZIP_ECDH_NUM_THIS_DISK_OFS: ::libc::c_uint = 4;
pub const MZ_ZIP_ECDH_NUM_DISK_CDIR_OFS: ::libc::c_uint = 6;
pub const MZ_ZIP_ECDH_CDIR_NUM_ENTRIES_ON_DISK_OFS: ::libc::c_uint = 8;
pub const MZ_ZIP_ECDH_CDIR_TOTAL_ENTRIES_OFS: ::libc::c_uint = 10;
pub const MZ_ZIP_ECDH_CDIR_SIZE_OFS: ::libc::c_uint = 12;
pub const MZ_ZIP_ECDH_CDIR_OFS_OFS: ::libc::c_uint = 16;
pub const MZ_ZIP_ECDH_COMMENT_SIZE_OFS: ::libc::c_uint = 20;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed25 {
    pub m_p: *mut ::libc::c_void,
    pub m_size: size_t,
    pub m_capacity: size_t,
    pub m_element_size: mz_uint,
}
impl ::std::clone::Clone for Struct_Unnamed25 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed25 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mz_zip_array = Struct_Unnamed25;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mz_zip_internal_state_tag {
    pub m_central_dir: mz_zip_array,
    pub m_central_dir_offsets: mz_zip_array,
    pub m_sorted_central_dir_offsets: mz_zip_array,
    pub m_pFile: *mut FILE,
    pub m_pMem: *mut ::libc::c_void,
    pub m_mem_size: size_t,
    pub m_mem_capacity: size_t,
}
impl ::std::clone::Clone for Struct_mz_zip_internal_state_tag {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_mz_zip_internal_state_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed26 {
    pub m_pZip: *mut mz_zip_archive,
    pub m_cur_archive_file_ofs: mz_uint64,
    pub m_comp_size: mz_uint64,
}
impl ::std::clone::Clone for Struct_Unnamed26 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed26 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mz_zip_writer_add_state = Struct_Unnamed26;
#[link(name = "miniz")]
extern "C" {
    pub fn mz_free(p: *mut ::libc::c_void) -> ();
    pub fn mz_adler32(adler: mz_ulong, ptr: *const ::libc::c_uchar,
                      buf_len: size_t) -> mz_ulong;
    pub fn mz_crc32(crc: mz_ulong, ptr: *const mz_uint8, buf_len: size_t)
     -> mz_ulong;
    pub fn mz_version() -> *const ::libc::c_char;
    pub fn mz_deflateInit(pStream: mz_streamp, level: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mz_deflateInit2(pStream: mz_streamp, level: ::libc::c_int,
                           method: ::libc::c_int, window_bits: ::libc::c_int,
                           mem_level: ::libc::c_int, strategy: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mz_deflateReset(pStream: mz_streamp) -> ::libc::c_int;
    pub fn mz_deflate(pStream: mz_streamp, flush: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mz_deflateEnd(pStream: mz_streamp) -> ::libc::c_int;
    pub fn mz_deflateBound(pStream: mz_streamp, source_len: mz_ulong)
     -> mz_ulong;
    pub fn mz_compress(pDest: *mut ::libc::c_uchar, pDest_len: *mut mz_ulong,
                       pSource: *const ::libc::c_uchar, source_len: mz_ulong)
     -> ::libc::c_int;
    pub fn mz_compress2(pDest: *mut ::libc::c_uchar, pDest_len: *mut mz_ulong,
                        pSource: *const ::libc::c_uchar, source_len: mz_ulong,
                        level: ::libc::c_int) -> ::libc::c_int;
    pub fn mz_compressBound(source_len: mz_ulong) -> mz_ulong;
    pub fn mz_inflateInit(pStream: mz_streamp) -> ::libc::c_int;
    pub fn mz_inflateInit2(pStream: mz_streamp, window_bits: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mz_inflate(pStream: mz_streamp, flush: ::libc::c_int)
     -> ::libc::c_int;
    pub fn mz_inflateEnd(pStream: mz_streamp) -> ::libc::c_int;
    pub fn mz_uncompress(pDest: *mut ::libc::c_uchar,
                         pDest_len: *mut mz_ulong,
                         pSource: *const ::libc::c_uchar,
                         source_len: mz_ulong) -> ::libc::c_int;
    pub fn mz_error(err: ::libc::c_int) -> *const ::libc::c_char;
    pub fn mz_zip_reader_init(pZip: *mut mz_zip_archive, size: mz_uint64,
                              flags: mz_uint32) -> mz_bool;
    pub fn mz_zip_reader_init_mem(pZip: *mut mz_zip_archive,
                                  pMem: *const ::libc::c_void, size: size_t,
                                  flags: mz_uint32) -> mz_bool;
    pub fn mz_zip_reader_init_file(pZip: *mut mz_zip_archive,
                                   pFilename: *const ::libc::c_char,
                                   flags: mz_uint32) -> mz_bool;
    pub fn mz_zip_reader_get_num_files(pZip: *mut mz_zip_archive) -> mz_uint;
    pub fn mz_zip_reader_file_stat(pZip: *mut mz_zip_archive,
                                   file_index: mz_uint,
                                   pStat: *mut mz_zip_archive_file_stat)
     -> mz_bool;
    pub fn mz_zip_reader_is_file_a_directory(pZip: *mut mz_zip_archive,
                                             file_index: mz_uint) -> mz_bool;
    pub fn mz_zip_reader_is_file_encrypted(pZip: *mut mz_zip_archive,
                                           file_index: mz_uint) -> mz_bool;
    pub fn mz_zip_reader_get_filename(pZip: *mut mz_zip_archive,
                                      file_index: mz_uint,
                                      pFilename: *mut ::libc::c_char,
                                      filename_buf_size: mz_uint) -> mz_uint;
    pub fn mz_zip_reader_locate_file(pZip: *mut mz_zip_archive,
                                     pName: *const ::libc::c_char,
                                     pComment: *const ::libc::c_char,
                                     flags: mz_uint) -> ::libc::c_int;
    pub fn mz_zip_reader_extract_to_mem_no_alloc(pZip: *mut mz_zip_archive,
                                                 file_index: mz_uint,
                                                 pBuf: *mut ::libc::c_void,
                                                 buf_size: size_t,
                                                 flags: mz_uint,
                                                 pUser_read_buf:
                                                     *mut ::libc::c_void,
                                                 user_read_buf_size: size_t)
     -> mz_bool;
    pub fn mz_zip_reader_extract_file_to_mem_no_alloc(pZip:
                                                          *mut mz_zip_archive,
                                                      pFilename:
                                                          *const ::libc::c_char,
                                                      pBuf:
                                                          *mut ::libc::c_void,
                                                      buf_size: size_t,
                                                      flags: mz_uint,
                                                      pUser_read_buf:
                                                          *mut ::libc::c_void,
                                                      user_read_buf_size:
                                                          size_t) -> mz_bool;
    pub fn mz_zip_reader_extract_to_mem(pZip: *mut mz_zip_archive,
                                        file_index: mz_uint,
                                        pBuf: *mut ::libc::c_void,
                                        buf_size: size_t, flags: mz_uint)
     -> mz_bool;
    pub fn mz_zip_reader_extract_file_to_mem(pZip: *mut mz_zip_archive,
                                             pFilename: *const ::libc::c_char,
                                             pBuf: *mut ::libc::c_void,
                                             buf_size: size_t, flags: mz_uint)
     -> mz_bool;
    pub fn mz_zip_reader_extract_to_heap(pZip: *mut mz_zip_archive,
                                         file_index: mz_uint,
                                         pSize: *mut size_t, flags: mz_uint)
     -> *mut ::libc::c_void;
    pub fn mz_zip_reader_extract_file_to_heap(pZip: *mut mz_zip_archive,
                                              pFilename:
                                                  *const ::libc::c_char,
                                              pSize: *mut size_t,
                                              flags: mz_uint)
     -> *mut ::libc::c_void;
    pub fn mz_zip_reader_extract_to_callback(pZip: *mut mz_zip_archive,
                                             file_index: mz_uint,
                                             pCallback: mz_file_write_func,
                                             pOpaque: *mut ::libc::c_void,
                                             flags: mz_uint) -> mz_bool;
    pub fn mz_zip_reader_extract_file_to_callback(pZip: *mut mz_zip_archive,
                                                  pFilename:
                                                      *const ::libc::c_char,
                                                  pCallback:
                                                      mz_file_write_func,
                                                  pOpaque:
                                                      *mut ::libc::c_void,
                                                  flags: mz_uint) -> mz_bool;
    pub fn mz_zip_reader_extract_to_file(pZip: *mut mz_zip_archive,
                                         file_index: mz_uint,
                                         pDst_filename: *const ::libc::c_char,
                                         flags: mz_uint) -> mz_bool;
    pub fn mz_zip_reader_extract_file_to_file(pZip: *mut mz_zip_archive,
                                              pArchive_filename:
                                                  *const ::libc::c_char,
                                              pDst_filename:
                                                  *const ::libc::c_char,
                                              flags: mz_uint) -> mz_bool;
    pub fn mz_zip_reader_end(pZip: *mut mz_zip_archive) -> mz_bool;
    pub fn mz_zip_writer_init(pZip: *mut mz_zip_archive,
                              existing_size: mz_uint64) -> mz_bool;
    pub fn mz_zip_writer_init_heap(pZip: *mut mz_zip_archive,
                                   size_to_reserve_at_beginning: size_t,
                                   initial_allocation_size: size_t)
     -> mz_bool;
    pub fn mz_zip_writer_init_file(pZip: *mut mz_zip_archive,
                                   pFilename: *const ::libc::c_char,
                                   size_to_reserve_at_beginning: mz_uint64)
     -> mz_bool;
    pub fn mz_zip_writer_init_from_reader(pZip: *mut mz_zip_archive,
                                          pFilename: *const ::libc::c_char)
     -> mz_bool;
    pub fn mz_zip_writer_add_mem(pZip: *mut mz_zip_archive,
                                 pArchive_name: *const ::libc::c_char,
                                 pBuf: *const ::libc::c_void,
                                 buf_size: size_t, level_and_flags: mz_uint)
     -> mz_bool;
    pub fn mz_zip_writer_add_mem_ex(pZip: *mut mz_zip_archive,
                                    pArchive_name: *const ::libc::c_char,
                                    pBuf: *const ::libc::c_void,
                                    buf_size: size_t,
                                    pComment: *const ::libc::c_void,
                                    comment_size: mz_uint16,
                                    level_and_flags: mz_uint,
                                    uncomp_size: mz_uint64,
                                    uncomp_crc32: mz_uint32) -> mz_bool;
    pub fn mz_zip_writer_add_file(pZip: *mut mz_zip_archive,
                                  pArchive_name: *const ::libc::c_char,
                                  pSrc_filename: *const ::libc::c_char,
                                  pComment: *const ::libc::c_void,
                                  comment_size: mz_uint16,
                                  level_and_flags: mz_uint) -> mz_bool;
    pub fn mz_zip_writer_add_from_zip_reader(pZip: *mut mz_zip_archive,
                                             pSource_zip: *mut mz_zip_archive,
                                             file_index: mz_uint) -> mz_bool;
    pub fn mz_zip_writer_finalize_archive(pZip: *mut mz_zip_archive)
     -> mz_bool;
    pub fn mz_zip_writer_finalize_heap_archive(pZip: *mut mz_zip_archive,
                                               pBuf: *mut *mut ::libc::c_void,
                                               pSize: *mut size_t) -> mz_bool;
    pub fn mz_zip_writer_end(pZip: *mut mz_zip_archive) -> mz_bool;
    pub fn mz_zip_add_mem_to_archive_file_in_place(pZip_filename:
                                                       *const ::libc::c_char,
                                                   pArchive_name:
                                                       *const ::libc::c_char,
                                                   pBuf:
                                                       *const ::libc::c_void,
                                                   buf_size: size_t,
                                                   pComment:
                                                       *const ::libc::c_void,
                                                   comment_size: mz_uint16,
                                                   level_and_flags: mz_uint)
     -> mz_bool;
    pub fn mz_zip_extract_archive_file_to_heap(pZip_filename:
                                                   *const ::libc::c_char,
                                               pArchive_name:
                                                   *const ::libc::c_char,
                                               pSize: *mut size_t,
                                               flags: mz_uint)
     -> *mut ::libc::c_void;
    pub fn tinfl_decompress_mem_to_heap(pSrc_buf: *const ::libc::c_void,
                                        src_buf_len: size_t,
                                        pOut_len: *mut size_t,
                                        flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn tinfl_decompress_mem_to_mem(pOut_buf: *mut ::libc::c_void,
                                       out_buf_len: size_t,
                                       pSrc_buf: *const ::libc::c_void,
                                       src_buf_len: size_t,
                                       flags: ::libc::c_int) -> size_t;
    pub fn tinfl_decompress_mem_to_callback(pIn_buf: *const ::libc::c_void,
                                            pIn_buf_size: *mut size_t,
                                            pPut_buf_func:
                                                tinfl_put_buf_func_ptr,
                                            pPut_buf_user:
                                                *mut ::libc::c_void,
                                            flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn tinfl_decompress(r: *mut tinfl_decompressor,
                            pIn_buf_next: *const mz_uint8,
                            pIn_buf_size: *mut size_t,
                            pOut_buf_start: *mut mz_uint8,
                            pOut_buf_next: *mut mz_uint8,
                            pOut_buf_size: *mut size_t,
                            decomp_flags: mz_uint32) -> tinfl_status;
    pub fn tdefl_compress_mem_to_heap(pSrc_buf: *const ::libc::c_void,
                                      src_buf_len: size_t,
                                      pOut_len: *mut size_t,
                                      flags: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn tdefl_compress_mem_to_mem(pOut_buf: *mut ::libc::c_void,
                                     out_buf_len: size_t,
                                     pSrc_buf: *const ::libc::c_void,
                                     src_buf_len: size_t,
                                     flags: ::libc::c_int) -> size_t;
    pub fn tdefl_write_image_to_png_file_in_memory_ex(pImage:
                                                          *const ::libc::c_void,
                                                      w: ::libc::c_int,
                                                      h: ::libc::c_int,
                                                      num_chans:
                                                          ::libc::c_int,
                                                      pLen_out: *mut size_t,
                                                      level: mz_uint,
                                                      flip: mz_bool)
     -> *mut ::libc::c_void;
    pub fn tdefl_write_image_to_png_file_in_memory(pImage:
                                                       *const ::libc::c_void,
                                                   w: ::libc::c_int,
                                                   h: ::libc::c_int,
                                                   num_chans: ::libc::c_int,
                                                   pLen_out: *mut size_t)
     -> *mut ::libc::c_void;
    pub fn tdefl_compress_mem_to_output(pBuf: *const ::libc::c_void,
                                        buf_len: size_t,
                                        pPut_buf_func: tdefl_put_buf_func_ptr,
                                        pPut_buf_user: *mut ::libc::c_void,
                                        flags: ::libc::c_int) -> mz_bool;
    pub fn tdefl_init(d: *mut tdefl_compressor,
                      pPut_buf_func: tdefl_put_buf_func_ptr,
                      pPut_buf_user: *mut ::libc::c_void,
                      flags: ::libc::c_int) -> tdefl_status;
    pub fn tdefl_compress(d: *mut tdefl_compressor,
                          pIn_buf: *const ::libc::c_void,
                          pIn_buf_size: *mut size_t,
                          pOut_buf: *mut ::libc::c_void,
                          pOut_buf_size: *mut size_t, flush: tdefl_flush)
     -> tdefl_status;
    pub fn tdefl_compress_buffer(d: *mut tdefl_compressor,
                                 pIn_buf: *const ::libc::c_void,
                                 in_buf_size: size_t, flush: tdefl_flush)
     -> tdefl_status;
    pub fn tdefl_get_prev_return_status(d: *mut tdefl_compressor)
     -> tdefl_status;
    pub fn tdefl_get_adler32(d: *mut tdefl_compressor) -> mz_uint32;
    pub fn tdefl_create_comp_flags_from_zip_params(level: ::libc::c_int,
                                                   window_bits: ::libc::c_int,
                                                   strategy: ::libc::c_int)
     -> mz_uint;
}