#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jv_parser;
    pub type jq_state;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn realpath(__name: *const libc::c_char, __resolved: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmem(__haystack: *const libc::c_void, __haystacklen: size_t,
              __needle: *const libc::c_void, __needlelen: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int)
     -> libc::c_long;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn clearerr(__stream: *mut FILE);
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_invalid() -> jv;
    #[no_mangle]
    fn jv_invalid_with_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_has_msg(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_sized(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_concat(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_parser_set_buf(_: *mut jv_parser, _: *const libc::c_char,
                         _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn jv_parser_remaining(_: *mut jv_parser) -> libc::c_int;
    #[no_mangle]
    fn jv_parser_next(_: *mut jv_parser) -> jv;
    #[no_mangle]
    fn jv_parser_free(_: *mut jv_parser);
    #[no_mangle]
    fn jq_get_input_cb(_: *mut jq_state, _: *mut jq_input_cb,
                       _: *mut *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed = 12;
pub const _PC_PRIO_IO: C2RustUnnamed = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed = 10;
pub const _PC_SYNC_IO: C2RustUnnamed = 9;
pub const _PC_VDISABLE: C2RustUnnamed = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed = 5;
pub const _PC_PATH_MAX: C2RustUnnamed = 4;
pub const _PC_NAME_MAX: C2RustUnnamed = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed = 2;
pub const _PC_MAX_CANON: C2RustUnnamed = 1;
pub const _PC_LINK_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
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
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ptr: *mut jv_refcnt,
    pub number: libc::c_double,
}
pub type jq_input_cb
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: *mut libc::c_void)
               -> jv>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jq_util_input_state {
    pub err_cb: jq_util_msg_cb,
    pub err_cb_data: *mut libc::c_void,
    pub parser: *mut jv_parser,
    pub current_input: *mut FILE,
    pub files: *mut *mut libc::c_char,
    pub nfiles: libc::c_int,
    pub curr_file: libc::c_int,
    pub failures: libc::c_int,
    pub slurped: jv,
    pub buf: [libc::c_char; 4096],
    pub buf_valid_len: size_t,
    pub current_filename: jv,
    pub current_line: size_t,
}
pub type jq_util_msg_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char)
               -> ()>;
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn expand_path(mut path: jv) -> jv {
    if jv_get_kind(path) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(path) == JV_KIND_STRING\x00" as *const u8
                          as *const libc::c_char,
                      b"src/util.c\x00" as *const u8 as *const libc::c_char,
                      83 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"jv expand_path(jv)\x00")).as_ptr());
    };
    let mut pstr: *const libc::c_char = jv_string_value(path);
    let mut ret: jv = path;
    if jv_string_length_bytes(jv_copy(path)) > 1 as libc::c_int &&
           *pstr.offset(0 as libc::c_int as isize) as libc::c_int ==
               '~' as i32 &&
           *pstr.offset(1 as libc::c_int as isize) as libc::c_int ==
               '/' as i32 {
        let mut home: jv = get_home();
        if jv_is_valid(home) != 0 {
            ret =
                jv_string_fmt(b"%s/%s\x00" as *const u8 as
                                  *const libc::c_char, jv_string_value(home),
                              pstr.offset(2 as libc::c_int as isize));
            jv_free(home);
        } else {
            let mut emsg: jv = jv_invalid_get_msg(home);
            ret =
                jv_invalid_with_msg(jv_string_fmt(b"Could not expand %s. (%s)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  pstr,
                                                  jv_string_value(emsg)));
            jv_free(emsg);
        }
        jv_free(path);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_home() -> jv {
    let mut ret: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
    let mut home: *mut libc::c_char =
        getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
    if home.is_null() {
        let mut pwd: *mut passwd = getpwuid(getuid());
        if !pwd.is_null() {
            ret = jv_string((*pwd).pw_dir)
        } else {
            ret =
                jv_invalid_with_msg(jv_string(b"Could not find home directory.\x00"
                                                  as *const u8 as
                                                  *const libc::c_char))
        }
    } else { ret = jv_string(home) }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn jq_realpath(mut path: jv) -> jv {
    let mut path_max: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    path_max =
        pathconf(jv_string_value(path), _PC_PATH_MAX as libc::c_int) as
            libc::c_int;
    if path_max > 0 as libc::c_int {
        buf = jv_mem_alloc(path_max as size_t) as *mut libc::c_char
    }
    let mut tmp: *mut libc::c_char = realpath(jv_string_value(path), buf);
    if tmp.is_null() { free(buf as *mut libc::c_void); return path }
    jv_free(path);
    path = jv_string(tmp);
    free(tmp as *mut libc::c_void);
    return path;
}
/*
 * The Windows CRT and console are something else.  In order for the
 * console to get UTF-8 written to it correctly we have to bypass stdio
 * completely.  No amount of fflush()ing helps.  If the first byte of a
 * buffer being written with fwrite() is non-ASCII UTF-8 then the
 * console misinterprets the byte sequence.  But one must not
 * WriteFile() if stdout is a file!1!!
 *
 * We carry knowledge of whether the FILE * is a tty everywhere we
 * output to it just so we can write with WriteFile() if stdout is a
 * console on WIN32.
 */
#[no_mangle]
pub unsafe extern "C" fn _jq_memmem(mut haystack: *const libc::c_void,
                                    mut haystacklen: size_t,
                                    mut needle: *const libc::c_void,
                                    mut needlelen: size_t)
 -> *const libc::c_void {
    return memmem(haystack, haystacklen, needle, needlelen) as
               *const libc::c_void;
    /* !HAVE_MEMMEM */
}
unsafe extern "C" fn fprinter(mut data: *mut libc::c_void,
                              mut fname: *const libc::c_char) {
    fprintf(data as *mut FILE,
            b"jq: error: Could not open file %s: %s\n\x00" as *const u8 as
                *const libc::c_char, fname, strerror(*__errno_location()));
}
// If parser == NULL -> RAW
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_init(mut err_cb: jq_util_msg_cb,
                                            mut err_cb_data:
                                                *mut libc::c_void)
 -> *mut jq_util_input_state {
    if err_cb.is_none() {
        err_cb =
            Some(fprinter as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *const libc::c_char) -> ());
        err_cb_data = stderr as *mut libc::c_void
    }
    let mut new_state: *mut jq_util_input_state =
        jv_mem_calloc(1 as libc::c_int as size_t,
                      ::std::mem::size_of::<jq_util_input_state>() as
                          libc::c_ulong) as *mut jq_util_input_state;
    (*new_state).err_cb = err_cb;
    (*new_state).err_cb_data = err_cb_data;
    (*new_state).slurped = jv_invalid();
    (*new_state).current_filename = jv_invalid();
    return new_state;
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_set_parser(mut state:
                                                      *mut jq_util_input_state,
                                                  mut parser: *mut jv_parser,
                                                  mut slurp: libc::c_int) {
    if jv_is_valid((*state).slurped) == 0 {
    } else {
        __assert_fail(b"!jv_is_valid(state->slurped)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/util.c\x00" as *const u8 as *const libc::c_char,
                      221 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 71],
                                                &[libc::c_char; 71]>(b"void jq_util_input_set_parser(jq_util_input_state *, jv_parser *, int)\x00")).as_ptr());
    };
    (*state).parser = parser;
    if parser.is_null() && slurp != 0 {
        (*state).slurped =
            jv_string(b"\x00" as *const u8 as *const libc::c_char)
    } else if slurp != 0 {
        (*state).slurped = jv_array()
    } else { (*state).slurped = jv_invalid() };
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_free(mut state:
                                                *mut *mut jq_util_input_state) {
    let mut old_state: *mut jq_util_input_state = *state;
    *state = 0 as *mut jq_util_input_state;
    if old_state.is_null() { return }
    if !(*old_state).parser.is_null() { jv_parser_free((*old_state).parser); }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*old_state).nfiles {
        free(*(*old_state).files.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free((*old_state).files as *mut libc::c_void);
    jv_free((*old_state).slurped);
    jv_free((*old_state).current_filename);
    jv_mem_free(old_state as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_add_input(mut state:
                                                     *mut jq_util_input_state,
                                                 mut fname:
                                                     *const libc::c_char) {
    (*state).files =
        jv_mem_realloc((*state).files as *mut libc::c_void,
                       (((*state).nfiles + 1 as libc::c_int) as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                            as libc::c_ulong))
            as *mut *mut libc::c_char;
    let fresh0 = (*state).nfiles;
    (*state).nfiles = (*state).nfiles + 1;
    let ref mut fresh1 = *(*state).files.offset(fresh0 as isize);
    *fresh1 = jv_mem_strdup(fname);
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_errors(mut state:
                                                  *mut jq_util_input_state)
 -> libc::c_int {
    return (*state).failures;
}
unsafe extern "C" fn next_file(mut state: *mut jq_util_input_state)
 -> *const libc::c_char {
    if (*state).curr_file < (*state).nfiles {
        let fresh2 = (*state).curr_file;
        (*state).curr_file = (*state).curr_file + 1;
        return *(*state).files.offset(fresh2 as isize)
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_next_input_cb(mut jq: *mut jq_state,
                                                     mut data:
                                                         *mut libc::c_void)
 -> jv {
    return jq_util_input_next_input(data as *mut jq_util_input_state);
}
// Return the current_filename:current_line
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_get_position(mut jq: *mut jq_state)
 -> jv {
    let mut cb: jq_input_cb = None;
    let mut cb_data: *mut libc::c_void = 0 as *mut libc::c_void;
    jq_get_input_cb(jq, &mut cb, &mut cb_data);
    if cb ==
           Some(jq_util_input_next_input_cb as
                    unsafe extern "C" fn(_: *mut jq_state,
                                         _: *mut libc::c_void) -> jv) {
    } else {
        __assert_fail(b"cb == jq_util_input_next_input_cb\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/util.c\x00" as *const u8 as *const libc::c_char,
                      364 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"jv jq_util_input_get_position(jq_state *)\x00")).as_ptr());
    };
    if cb !=
           Some(jq_util_input_next_input_cb as
                    unsafe extern "C" fn(_: *mut jq_state,
                                         _: *mut libc::c_void) -> jv) {
        return jv_invalid_with_msg(jv_string(b"Invalid jq_util_input API usage\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    let mut s: *mut jq_util_input_state = cb_data as *mut jq_util_input_state;
    // We can't assert that current_filename is a string because if
  // the error was a JSON parser error then we may not have set
  // current_filename yet.
    if jv_get_kind((*s).current_filename) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return jv_string(b"<unknown>\x00" as *const u8 as *const libc::c_char)
    }
    let mut v: jv =
        jv_string_fmt(b"%s:%lu\x00" as *const u8 as *const libc::c_char,
                      jv_string_value((*s).current_filename),
                      (*s).current_line);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_get_current_filename(mut jq:
                                                                *mut jq_state)
 -> jv {
    let mut cb: jq_input_cb = None;
    let mut cb_data: *mut libc::c_void = 0 as *mut libc::c_void;
    jq_get_input_cb(jq, &mut cb, &mut cb_data);
    if cb !=
           Some(jq_util_input_next_input_cb as
                    unsafe extern "C" fn(_: *mut jq_state,
                                         _: *mut libc::c_void) -> jv) {
        return jv_invalid_with_msg(jv_string(b"Unknown input filename\x00" as
                                                 *const u8 as
                                                 *const libc::c_char))
    }
    let mut s: *mut jq_util_input_state = cb_data as *mut jq_util_input_state;
    let mut v: jv = jv_copy((*s).current_filename);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_get_current_line(mut jq: *mut jq_state)
 -> jv {
    let mut cb: jq_input_cb = None;
    let mut cb_data: *mut libc::c_void = 0 as *mut libc::c_void;
    jq_get_input_cb(jq, &mut cb, &mut cb_data);
    if cb !=
           Some(jq_util_input_next_input_cb as
                    unsafe extern "C" fn(_: *mut jq_state,
                                         _: *mut libc::c_void) -> jv) {
        return jv_invalid_with_msg(jv_string(b"Unknown input line number\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    let mut s: *mut jq_util_input_state = cb_data as *mut jq_util_input_state;
    let mut v: jv = jv_number((*s).current_line as libc::c_double);
    return v;
}
// Blocks to read one more input from stdin and/or given files
// When slurping, it returns just one value
#[no_mangle]
pub unsafe extern "C" fn jq_util_input_next_input(mut state:
                                                      *mut jq_util_input_state)
 -> jv {
    let mut is_last: libc::c_int = 0 as libc::c_int; // need more input
    let mut has_more: libc::c_int = 0 as libc::c_int;
    let mut value: jv = jv_invalid();
    loop  {
        if (*state).parser.is_null() {
            // Raw input
            is_last = jq_util_input_read_more(state);
            if !((*state).buf_valid_len == 0 as libc::c_int as libc::c_ulong)
               {
                if jv_is_valid((*state).slurped) != 0 {
                    // Slurped raw input
                    (*state).slurped =
                        jv_string_concat((*state).slurped,
                                         jv_string_sized((*state).buf.as_mut_ptr(),
                                                         (*state).buf_valid_len
                                                             as libc::c_int))
                } else {
                    if jv_is_valid(value) == 0 {
                        value =
                            jv_string(b"\x00" as *const u8 as
                                          *const libc::c_char)
                    }
                    if (*state).buf[(*state).buf_valid_len.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
                                        as usize] as libc::c_int ==
                           '\n' as i32 {
                        // whole line
                        (*state).buf[(*state).buf_valid_len.wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                         as usize] =
                            0 as libc::c_int as libc::c_char;
                        return jv_string_concat(value,
                                                jv_string_sized((*state).buf.as_mut_ptr(),
                                                                (*state).buf_valid_len.wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)
                                                                    as
                                                                    libc::c_int))
                    }
                    value =
                        jv_string_concat(value,
                                         jv_string_sized((*state).buf.as_mut_ptr(),
                                                         (*state).buf_valid_len
                                                             as libc::c_int));
                    (*state).buf[0 as libc::c_int as usize] =
                        '\u{0}' as i32 as libc::c_char;
                    (*state).buf_valid_len = 0 as libc::c_int as size_t
                }
            }
        } else {
            if jv_parser_remaining((*state).parser) == 0 as libc::c_int {
                is_last = jq_util_input_read_more(state);
                jv_parser_set_buf((*state).parser, (*state).buf.as_mut_ptr(),
                                  (*state).buf_valid_len as libc::c_int,
                                  (is_last == 0) as libc::c_int);
            }
            value = jv_parser_next((*state).parser);
            if jv_is_valid((*state).slurped) != 0 {
                // When slurping an input that doesn't have a trailing newline,
        // we might have more than one value on the same line, so let's check
        // to see if we have more data to parse.
                has_more = jv_parser_remaining((*state).parser);
                if jv_is_valid(value) != 0 {
                    (*state).slurped =
                        jv_array_append((*state).slurped, value);
                    value = jv_invalid()
                } else if jv_invalid_has_msg(jv_copy(value)) != 0 {
                    return value
                }
                // Not slurped parsed input
            } else if jv_is_valid(value) != 0 ||
                          jv_invalid_has_msg(jv_copy(value)) != 0 {
                return value
            }
        }
        if !(is_last == 0 || has_more != 0) { break ; }
    }
    if jv_is_valid((*state).slurped) != 0 {
        value = (*state).slurped;
        (*state).slurped = jv_invalid()
    }
    return value;
}
