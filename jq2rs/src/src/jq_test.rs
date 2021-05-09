#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jq_state;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_get_refcnt(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_equal(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_slice(_: jv, _: libc::c_int, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_sized(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_hash(_: jv) -> libc::c_ulong;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_dump(_: jv, flags: libc::c_int);
    #[no_mangle]
    fn jv_dump_string(_: jv, flags: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_parse(string: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_parse_sized(string: *const libc::c_char, length: libc::c_int) -> jv;
    #[no_mangle]
    fn jq_init() -> *mut jq_state;
    #[no_mangle]
    fn jq_set_error_cb(_: *mut jq_state, _: jq_msg_cb, _: *mut libc::c_void);
    #[no_mangle]
    fn jq_compile(_: *mut jq_state, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn jq_dump_disassembly(_: *mut jq_state, _: libc::c_int);
    #[no_mangle]
    fn jq_start(_: *mut jq_state, value: jv, _: libc::c_int);
    #[no_mangle]
    fn jq_next(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_teardown(_: *mut *mut jq_state);
    #[no_mangle]
    fn jq_set_attr(_: *mut jq_state, _: jv, _: jv);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type jv_kind = libc::c_uint;
pub const JV_KIND_OBJECT: jv_kind = 7;
pub const JV_KIND_ARRAY: jv_kind = 6;
pub const JV_KIND_STRING: jv_kind = 5;
pub const JV_KIND_NUMBER: jv_kind = 4;
pub const JV_KIND_TRUE: jv_kind = 3;
pub const JV_KIND_FALSE: jv_kind = 2;
pub const JV_KIND_NULL: jv_kind = 1;
pub const JV_KIND_INVALID: jv_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jv {
    pub kind_flags: libc::c_uchar,
    pub pad_: libc::c_uchar,
    pub offset: libc::c_ushort,
    pub size: libc::c_int,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *mut jv_refcnt,
    pub number: libc::c_double,
}
pub type jv_print_flags = libc::c_uint;
pub const JV_PRINT_SPACE2: jv_print_flags = 1024;
pub const JV_PRINT_SPACE1: jv_print_flags = 512;
pub const JV_PRINT_SPACE0: jv_print_flags = 256;
pub const JV_PRINT_ISATTY: jv_print_flags = 128;
pub const JV_PRINT_TAB: jv_print_flags = 64;
pub const JV_PRINT_REFCOUNT: jv_print_flags = 32;
pub const JV_PRINT_INVALID: jv_print_flags = 16;
pub const JV_PRINT_SORTED: jv_print_flags = 8;
pub const JV_PRINT_COLOUR: jv_print_flags = 4;
pub const JV_PRINT_COLOR: jv_print_flags = 4;
pub const JV_PRINT_ASCII: jv_print_flags = 2;
pub const JV_PRINT_PRETTY: jv_print_flags = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const JQ_DEBUG_TRACE_ALL: C2RustUnnamed_0 = 3;
pub const JQ_DEBUG_TRACE_DETAIL: C2RustUnnamed_0 = 2;
pub const JQ_DEBUG_TRACE: C2RustUnnamed_0 = 1;
pub type jq_msg_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: jv) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct err_data {
    pub buf: [libc::c_char; 4096],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn skipline(mut buf: *const libc::c_char) -> libc::c_int {
    let mut p: libc::c_int = 0 as libc::c_int;
    while *buf.offset(p as isize) as libc::c_int == ' ' as i32 ||
              *buf.offset(p as isize) as libc::c_int == '\t' as i32 {
        p += 1
    }
    if *buf.offset(p as isize) as libc::c_int == '#' as i32 ||
           *buf.offset(p as isize) as libc::c_int == '\n' as i32 ||
           *buf.offset(p as isize) as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
