#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type Bigint;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
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
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_number_get_literal(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_append_buf(a: jv, buf: *const libc::c_char, len: libc::c_int)
     -> jv;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_length(object: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_next(_: jv, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_valid(_: jv, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_key(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_object_iter_value(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_keys(_: jv) -> jv;
    #[no_mangle]
    fn jvp_dtoa_fmt(C: *mut dtoa_context, b: *mut libc::c_char,
                    x: libc::c_double) -> *mut libc::c_char;
    #[no_mangle]
    fn tsd_dtoa_context_get() -> *mut dtoa_context;
    #[no_mangle]
    fn jvp_utf8_next(in_0: *const libc::c_char, end: *const libc::c_char,
                     codepoint: *mut libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn jvp_number_is_nan(_: jv) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtoa_context {
    pub freelist: [*mut Bigint; 8],
    pub p5s: *mut Bigint,
}
// Color table. See https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
// for how to choose these.
static mut color_kinds: [jv_kind; 7] =
    [JV_KIND_NULL, JV_KIND_FALSE, JV_KIND_TRUE, JV_KIND_NUMBER,
     JV_KIND_STRING, JV_KIND_ARRAY, JV_KIND_OBJECT];
static mut color_bufs: [[libc::c_char; 16]; 7] = [[0; 16]; 7];
static mut color_bufps: [*const libc::c_char; 8] =
    [0 as *const libc::c_char; 8];
static mut def_colors: [*const libc::c_char; 7] =
    [b"\x1b[1;30m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[0;37m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[0;37m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[0;37m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[0;32m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[1;37m\x00" as *const u8 as *const libc::c_char,
     b"\x1b[1;37m\x00" as *const u8 as *const libc::c_char];
static mut colors: *mut *const libc::c_char =
    unsafe { def_colors.as_ptr() as *mut _ };
unsafe extern "C" fn put_buf(mut s: *const libc::c_char, mut len: libc::c_int,
                             mut fout: *mut FILE, mut strout: *mut jv,
                             mut is_tty: libc::c_int) {
    if !strout.is_null() {
        *strout = jv_string_append_buf(*strout, s, len)
    } else {
        fwrite(s as *const libc::c_void, 1 as libc::c_int as libc::c_ulong,
               len as libc::c_ulong, fout);
    };
}
unsafe extern "C" fn put_char(mut c: libc::c_char, mut fout: *mut FILE,
                              mut strout: *mut jv, mut T: libc::c_int) {
    put_buf(&mut c, 1 as libc::c_int, fout, strout, T);
}
unsafe extern "C" fn put_str(mut s: *const libc::c_char, mut fout: *mut FILE,
                             mut strout: *mut jv, mut T: libc::c_int) {
    put_buf(s, strlen(s) as libc::c_int, fout, strout, T);
}
unsafe extern "C" fn put_indent(mut n: libc::c_int, mut flags: libc::c_int,
                                mut fout: *mut FILE, mut strout: *mut jv,
                                mut T: libc::c_int) {
    if flags & JV_PRINT_TAB as libc::c_int != 0 {
        loop  {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 != 0) { break ; }
            put_char('\t' as i32 as libc::c_char, fout, strout, T);
        }
    } else {
        n *=
            (flags &
                 (JV_PRINT_SPACE0 as libc::c_int |
                      JV_PRINT_SPACE1 as libc::c_int |
                      JV_PRINT_SPACE2 as libc::c_int)) >> 8 as libc::c_int;
        loop  {
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 != 0) { break ; }
            put_char(' ' as i32 as libc::c_char, fout, strout, T);
        }
    };
}
unsafe extern "C" fn jvp_dump_string(mut str: jv, mut ascii_only: libc::c_int,
                                     mut F: *mut FILE, mut S: *mut jv,
                                     mut T: libc::c_int) {
    if jv_get_kind(str) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(str) == JV_KIND_STRING\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv_print.c\x00" as *const u8 as
                          *const libc::c_char,
                      119 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void jvp_dump_string(jv, int, FILE *, jv *, int)\x00")).as_ptr());
    };
    let mut i: *const libc::c_char = jv_string_value(str);
    let mut end: *const libc::c_char =
        i.offset(jv_string_length_bytes(jv_copy(str)) as isize);
    let mut cstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 32] = [0; 32];
    put_char('\"' as i32 as libc::c_char, F, S, T);
    loop  {
        cstart = i;
        i = jvp_utf8_next(cstart, end, &mut c);
        if i.is_null() { break ; }
        if c != -(1 as libc::c_int) {
        } else {
            __assert_fail(b"c != -1\x00" as *const u8 as *const libc::c_char,
                          b"src/jv_print.c\x00" as *const u8 as
                              *const libc::c_char,
                          127 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 49],
                                                    &[libc::c_char; 49]>(b"void jvp_dump_string(jv, int, FILE *, jv *, int)\x00")).as_ptr());
        };
        let mut unicode_escape: libc::c_int = 0 as libc::c_int;
        if 0x20 as libc::c_int <= c && c <= 0x7e as libc::c_int {
            // printable ASCII
            if c == '\"' as i32 || c == '\\' as i32 {
                put_char('\\' as i32 as libc::c_char, F, S, T);
            }
            put_char(c as libc::c_char, F, S, T);
        } else if c < 0x20 as libc::c_int || c == 0x7f as libc::c_int {
            // ASCII control character
            match c {
                8 => {
                    put_char('\\' as i32 as libc::c_char, F, S, T);
                    put_char('b' as i32 as libc::c_char, F, S, T);
                }
                9 => {
                    put_char('\\' as i32 as libc::c_char, F, S, T);
                    put_char('t' as i32 as libc::c_char, F, S, T);
                }
                13 => {
                    put_char('\\' as i32 as libc::c_char, F, S, T);
                    put_char('r' as i32 as libc::c_char, F, S, T);
                }
                10 => {
                    put_char('\\' as i32 as libc::c_char, F, S, T);
                    put_char('n' as i32 as libc::c_char, F, S, T);
                }
                12 => {
                    put_char('\\' as i32 as libc::c_char, F, S, T);
                    put_char('f' as i32 as libc::c_char, F, S, T);
                }
                _ => { unicode_escape = 1 as libc::c_int }
            }
        } else if ascii_only != 0 {
            unicode_escape = 1 as libc::c_int
        } else {
            put_buf(cstart,
                    i.wrapping_offset_from(cstart) as libc::c_long as
                        libc::c_int, F, S, T);
        }
        if unicode_escape != 0 {
            if c <= 0xffff as libc::c_int {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 32]>() as
                             libc::c_ulong,
                         b"\\u%04x\x00" as *const u8 as *const libc::c_char,
                         c);
            } else {
                c -= 0x10000 as libc::c_int;
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 32]>() as
                             libc::c_ulong,
                         b"\\u%04x\\u%04x\x00" as *const u8 as
                             *const libc::c_char,
                         0xd800 as libc::c_int |
                             (c & 0xffc00 as libc::c_int) >>
                                 10 as libc::c_int,
                         0xdc00 as libc::c_int | c & 0x3ff as libc::c_int);
            }
            put_str(buf.as_mut_ptr(), F, S, T);
        }
    }
    if c != -(1 as libc::c_int) {
    } else {
        __assert_fail(b"c != -1\x00" as *const u8 as *const libc::c_char,
                      b"src/jv_print.c\x00" as *const u8 as
                          *const libc::c_char,
                      181 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void jvp_dump_string(jv, int, FILE *, jv *, int)\x00")).as_ptr());
    };
    put_char('\"' as i32 as libc::c_char, F, S, T);
}
unsafe extern "C" fn put_refcnt(mut C: *mut dtoa_context,
                                mut refcnt: libc::c_int, mut F: *mut FILE,
                                mut S: *mut jv, mut T: libc::c_int) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    put_char(' ' as i32 as libc::c_char, F, S, T);
    put_char('(' as i32 as libc::c_char, F, S, T);
    put_str(jvp_dtoa_fmt(C, buf.as_mut_ptr(), refcnt as libc::c_double), F, S,
            T);
    put_char(')' as i32 as libc::c_char, F, S, T);
}
unsafe extern "C" fn jv_dump_term(mut C: *mut dtoa_context, mut x: jv,
                                  mut flags: libc::c_int,
                                  mut indent: libc::c_int, mut F: *mut FILE,
                                  mut S: *mut jv) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut color: *const libc::c_char = 0 as *const libc::c_char;
    let mut refcnt: libc::c_double =
        if flags & JV_PRINT_REFCOUNT as libc::c_int != 0 {
            (jv_get_refcnt(x)) - 1 as libc::c_int
        } else { -(1 as libc::c_int) } as libc::c_double;
    if flags & JV_PRINT_COLOR as libc::c_int != 0 {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[jv_kind; 7]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<jv_kind>()
                                                       as libc::c_ulong) {
            if jv_get_kind(x) as libc::c_uint ==
                   color_kinds[i as usize] as libc::c_uint {
                color = *colors.offset(i as isize);
                put_str(color, F, S, flags & JV_PRINT_ISATTY as libc::c_int);
                break ;
            } else { i = i.wrapping_add(1) }
        }
    }
    if indent > 256 as libc::c_int {
        put_str(b"<skipped: too deep>\x00" as *const u8 as
                    *const libc::c_char, F, S,
                flags & JV_PRINT_ISATTY as libc::c_int);
    } else {
        match jv_get_kind(x) as libc::c_uint {
            1 => {
                put_str(b"null\x00" as *const u8 as *const libc::c_char, F, S,
                        flags & JV_PRINT_ISATTY as libc::c_int);
            }
            2 => {
                put_str(b"false\x00" as *const u8 as *const libc::c_char, F,
                        S, flags & JV_PRINT_ISATTY as libc::c_int);
            }
            3 => {
                put_str(b"true\x00" as *const u8 as *const libc::c_char, F, S,
                        flags & JV_PRINT_ISATTY as libc::c_int);
            }
            4 => {
                if jvp_number_is_nan(x) != 0 {
                    jv_dump_term(C, jv_null(), flags, indent, F, S);
                } else {
                    let mut literal_data: *const libc::c_char =
                        jv_number_get_literal(x);
                    if !literal_data.is_null() {
                        put_str(literal_data, F, S,
                                flags & JV_PRINT_ISATTY as libc::c_int);
                    } else {
                        let mut d: libc::c_double = jv_number_value(x);
                        if d != d {
                            // JSON doesn't have NaN, so we'll render it as "null"
                            put_str(b"null\x00" as *const u8 as
                                        *const libc::c_char, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        } else {
                            // Normalise infinities to something we can print in valid JSON
                            if d > 1.7976931348623157e+308f64 {
                                d = 1.7976931348623157e+308f64
                            }
                            if d < -1.7976931348623157e+308f64 {
                                d = -1.7976931348623157e+308f64
                            }
                            put_str(jvp_dtoa_fmt(C, buf.as_mut_ptr(), d), F,
                                    S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                    }
                }
            }
            5 => {
                jvp_dump_string(x, flags & JV_PRINT_ASCII as libc::c_int, F,
                                S, flags & JV_PRINT_ISATTY as libc::c_int);
                if flags & JV_PRINT_REFCOUNT as libc::c_int != 0 {
                    put_refcnt(C, refcnt as libc::c_int, F, S,
                               flags & JV_PRINT_ISATTY as libc::c_int);
                }
            }
            6 => {
                if jv_array_length(jv_copy(x)) == 0 as libc::c_int {
                    put_str(b"[]\x00" as *const u8 as *const libc::c_char, F,
                            S, flags & JV_PRINT_ISATTY as libc::c_int);
                } else {
                    put_str(b"[\x00" as *const u8 as *const libc::c_char, F,
                            S, flags & JV_PRINT_ISATTY as libc::c_int);
                    if flags & JV_PRINT_PRETTY as libc::c_int != 0 {
                        put_char('\n' as i32 as libc::c_char, F, S,
                                 flags & JV_PRINT_ISATTY as libc::c_int);
                        put_indent(indent + 1 as libc::c_int, flags, F, S,
                                   flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                    let mut jv_len__: libc::c_int =
                        jv_array_length(jv_copy(x));
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    let mut jv_j__: libc::c_int = 1 as libc::c_int;
                    while jv_j__ != 0 {
                        let mut elem: jv =
                            jv{kind_flags: 0,
                               pad_: 0,
                               offset: 0,
                               size: 0,
                               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                        while if i_0 < jv_len__ {
                                  elem = jv_array_get(jv_copy(x), i_0);
                                  1 as libc::c_int
                              } else { 0 as libc::c_int } != 0 {
                            if i_0 != 0 as libc::c_int {
                                if flags & JV_PRINT_PRETTY as libc::c_int != 0
                                   {
                                    put_str(b",\n\x00" as *const u8 as
                                                *const libc::c_char, F, S,
                                            flags &
                                                JV_PRINT_ISATTY as
                                                    libc::c_int);
                                    put_indent(indent + 1 as libc::c_int,
                                               flags, F, S,
                                               flags &
                                                   JV_PRINT_ISATTY as
                                                       libc::c_int);
                                } else {
                                    put_str(b",\x00" as *const u8 as
                                                *const libc::c_char, F, S,
                                            flags &
                                                JV_PRINT_ISATTY as
                                                    libc::c_int);
                                }
                            }
                            jv_dump_term(C, elem, flags,
                                         indent + 1 as libc::c_int, F, S);
                            if !color.is_null() {
                                put_str(color, F, S,
                                        flags &
                                            JV_PRINT_ISATTY as libc::c_int);
                            }
                            i_0 += 1
                        }
                        jv_j__ = 0 as libc::c_int
                    }
                    if flags & JV_PRINT_PRETTY as libc::c_int != 0 {
                        put_char('\n' as i32 as libc::c_char, F, S,
                                 flags & JV_PRINT_ISATTY as libc::c_int);
                        put_indent(indent, flags, F, S,
                                   flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                    if !color.is_null() {
                        put_str(color, F, S,
                                flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                    put_char(']' as i32 as libc::c_char, F, S,
                             flags & JV_PRINT_ISATTY as libc::c_int);
                    if flags & JV_PRINT_REFCOUNT as libc::c_int != 0 {
                        put_refcnt(C, refcnt as libc::c_int, F, S,
                                   flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                }
            }
            7 => {
                if jv_object_length(jv_copy(x)) == 0 as libc::c_int {
                    put_str(b"{}\x00" as *const u8 as *const libc::c_char, F,
                            S, flags & JV_PRINT_ISATTY as libc::c_int);
                } else {
                    put_char('{' as i32 as libc::c_char, F, S,
                             flags & JV_PRINT_ISATTY as libc::c_int);
                    if flags & JV_PRINT_PRETTY as libc::c_int != 0 {
                        put_char('\n' as i32 as libc::c_char, F, S,
                                 flags & JV_PRINT_ISATTY as libc::c_int);
                        put_indent(indent + 1 as libc::c_int, flags, F, S,
                                   flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                    let mut first: libc::c_int = 1 as libc::c_int;
                    let mut i_1: libc::c_int = 0 as libc::c_int;
                    let mut keyset: jv = jv_null();
                    loop  {
                        let mut key: jv =
                            jv{kind_flags: 0,
                               pad_: 0,
                               offset: 0,
                               size: 0,
                               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                        let mut value: jv =
                            jv{kind_flags: 0,
                               pad_: 0,
                               offset: 0,
                               size: 0,
                               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                        if flags & JV_PRINT_SORTED as libc::c_int != 0 {
                            if first != 0 {
                                keyset = jv_keys(jv_copy(x));
                                i_1 = 0 as libc::c_int
                            } else { i_1 += 1 }
                            if i_1 >= jv_array_length(jv_copy(keyset)) {
                                jv_free(keyset);
                                break ;
                            } else {
                                key = jv_array_get(jv_copy(keyset), i_1);
                                value =
                                    jv_object_get(jv_copy(x), jv_copy(key))
                            }
                        } else {
                            if first != 0 {
                                i_1 = jv_object_iter(x)
                            } else { i_1 = jv_object_iter_next(x, i_1) }
                            if jv_object_iter_valid(x, i_1) == 0 { break ; }
                            key = jv_object_iter_key(x, i_1);
                            value = jv_object_iter_value(x, i_1)
                        }
                        if first == 0 {
                            if flags & JV_PRINT_PRETTY as libc::c_int != 0 {
                                put_str(b",\n\x00" as *const u8 as
                                            *const libc::c_char, F, S,
                                        flags &
                                            JV_PRINT_ISATTY as libc::c_int);
                                put_indent(indent + 1 as libc::c_int, flags,
                                           F, S,
                                           flags &
                                               JV_PRINT_ISATTY as
                                                   libc::c_int);
                            } else {
                                put_str(b",\x00" as *const u8 as
                                            *const libc::c_char, F, S,
                                        flags &
                                            JV_PRINT_ISATTY as libc::c_int);
                            }
                        }
                        if !color.is_null() {
                            put_str(b"\x1b[0m\x00" as *const u8 as
                                        *const libc::c_char, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                        first = 0 as libc::c_int;
                        if !color.is_null() {
                            put_str(b"\x1b[34;1m\x00" as *const u8 as
                                        *const libc::c_char, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                        jvp_dump_string(key,
                                        flags & JV_PRINT_ASCII as libc::c_int,
                                        F, S,
                                        flags &
                                            JV_PRINT_ISATTY as libc::c_int);
                        jv_free(key);
                        if !color.is_null() {
                            put_str(b"\x1b[0m\x00" as *const u8 as
                                        *const libc::c_char, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                        if !color.is_null() {
                            put_str(color, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                        put_str(if flags & JV_PRINT_PRETTY as libc::c_int != 0
                                   {
                                    b": \x00" as *const u8 as
                                        *const libc::c_char
                                } else {
                                    b":\x00" as *const u8 as
                                        *const libc::c_char
                                }, F, S,
                                flags & JV_PRINT_ISATTY as libc::c_int);
                        if !color.is_null() {
                            put_str(b"\x1b[0m\x00" as *const u8 as
                                        *const libc::c_char, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                        jv_dump_term(C, value, flags,
                                     indent + 1 as libc::c_int, F, S);
                        if !color.is_null() {
                            put_str(color, F, S,
                                    flags & JV_PRINT_ISATTY as libc::c_int);
                        }
                    }
                    if flags & JV_PRINT_PRETTY as libc::c_int != 0 {
                        put_char('\n' as i32 as libc::c_char, F, S,
                                 flags & JV_PRINT_ISATTY as libc::c_int);
                        put_indent(indent, flags, F, S,
                                   flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                    if !color.is_null() {
                        put_str(color, F, S,
                                flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                    put_char('}' as i32 as libc::c_char, F, S,
                             flags & JV_PRINT_ISATTY as libc::c_int);
                    if flags & JV_PRINT_REFCOUNT as libc::c_int != 0 {
                        put_refcnt(C, refcnt as libc::c_int, F, S,
                                   flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                }
            }
            0 | _ => {
                if flags & JV_PRINT_INVALID as libc::c_int != 0 {
                    let mut msg: jv = jv_invalid_get_msg(jv_copy(x));
                    if jv_get_kind(msg) as libc::c_uint ==
                           JV_KIND_STRING as libc::c_int as libc::c_uint {
                        put_str(b"<invalid:\x00" as *const u8 as
                                    *const libc::c_char, F, S,
                                flags & JV_PRINT_ISATTY as libc::c_int);
                        jvp_dump_string(msg,
                                        flags | JV_PRINT_ASCII as libc::c_int,
                                        F, S,
                                        flags &
                                            JV_PRINT_ISATTY as libc::c_int);
                        put_str(b">\x00" as *const u8 as *const libc::c_char,
                                F, S, flags & JV_PRINT_ISATTY as libc::c_int);
                    } else {
                        put_str(b"<invalid>\x00" as *const u8 as
                                    *const libc::c_char, F, S,
                                flags & JV_PRINT_ISATTY as libc::c_int);
                    }
                } else {
                    if 0 as libc::c_int != 0 &&
                           !(b"Invalid value\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
                    } else {
                        __assert_fail(b"0 && \"Invalid value\"\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"src/jv_print.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      221 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 69],
                                                                &[libc::c_char; 69]>(b"void jv_dump_term(struct dtoa_context *, jv, int, int, FILE *, jv *)\x00")).as_ptr());
                    };
                }
            }
        }
    }
    jv_free(x);
    if !color.is_null() {
        put_str(b"\x1b[0m\x00" as *const u8 as *const libc::c_char, F, S,
                flags & JV_PRINT_ISATTY as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_dumpf(mut x: jv, mut f: *mut FILE,
                                  mut flags: libc::c_int) {
    jv_dump_term(tsd_dtoa_context_get(), x, flags, 0 as libc::c_int, f,
                 0 as *mut jv);
}
#[no_mangle]
pub unsafe extern "C" fn jv_dump(mut x: jv, mut flags: libc::c_int) {
    jv_dumpf(x, stdout, flags);
}
/* This one is nice for use in debuggers */
#[no_mangle]
pub unsafe extern "C" fn jv_show(mut x: jv, mut flags: libc::c_int) {
    if flags == -(1 as libc::c_int) {
        flags =
            JV_PRINT_PRETTY as libc::c_int | JV_PRINT_COLOR as libc::c_int |
                (if (2 as libc::c_int) < 0 as libc::c_int ||
                        2 as libc::c_int > 7 as libc::c_int {
                     (JV_PRINT_TAB as libc::c_int) |
                         JV_PRINT_PRETTY as libc::c_int
                 } else {
                     (if 2 as libc::c_int == 0 as libc::c_int {
                          0 as libc::c_int
                      } else {
                          ((2 as libc::c_int) << 8 as libc::c_int) |
                              JV_PRINT_PRETTY as libc::c_int
                      })
                 })
    }
    jv_dumpf(jv_copy(x), stderr, flags | JV_PRINT_INVALID as libc::c_int);
    fflush(stderr);
}
#[no_mangle]
pub unsafe extern "C" fn jv_dump_string(mut x: jv, mut flags: libc::c_int)
 -> jv {
    let mut s: jv = jv_string(b"\x00" as *const u8 as *const libc::c_char);
    jv_dump_term(tsd_dtoa_context_get(), x, flags, 0 as libc::c_int,
                 0 as *mut FILE, &mut s);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn jv_dump_string_trunc(mut x: jv,
                                              mut outbuf: *mut libc::c_char,
                                              mut bufsize: size_t)
 -> *mut libc::c_char {
    x = jv_dump_string(x, 0 as libc::c_int);
    let mut p: *const libc::c_char = jv_string_value(x);
    let len: size_t = strlen(p);
    libc::strncpy(outbuf, p, bufsize as libc::size_t);
    *outbuf.offset(bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                       isize) = 0 as libc::c_int as libc::c_char;
    if len > bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) &&
           bufsize >= 4 as libc::c_int as libc::c_ulong {
        // Indicate truncation with '...'
        *outbuf.offset(bufsize.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                           as isize) = '.' as i32 as libc::c_char;
        *outbuf.offset(bufsize.wrapping_sub(3 as libc::c_int as libc::c_ulong)
                           as isize) = '.' as i32 as libc::c_char;
        *outbuf.offset(bufsize.wrapping_sub(4 as libc::c_int as libc::c_ulong)
                           as isize) = '.' as i32 as libc::c_char
    }
    jv_free(x);
    return outbuf;
}
