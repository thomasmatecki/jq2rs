#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jq_state;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_vfmt(_: *const libc::c_char, _: ::std::ffi::VaList) -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jq_report_error(_: *mut jq_state, _: jv);
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct location {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locfile {
    pub fname: jv,
    pub data: *const libc::c_char,
    pub length: libc::c_int,
    pub linemap: *mut libc::c_int,
    pub nlines: libc::c_int,
    pub error: *mut libc::c_char,
    pub jq: *mut jq_state,
    pub refct: libc::c_int,
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int; // at start of line, not of \n
}
#[no_mangle]
pub unsafe extern "C" fn locfile_init(mut jq: *mut jq_state,
                                      mut fname: *const libc::c_char,
                                      mut data: *const libc::c_char,
                                      mut length: libc::c_int)
 -> *mut locfile {
    let mut l: *mut locfile =
        jv_mem_alloc(::std::mem::size_of::<locfile>() as libc::c_ulong) as
            *mut locfile; // virtual last \n
    (*l).jq =
        jq; // == if pos at start (before, never ==, because pos never on \n)
    (*l).fname = jv_string(fname);
    (*l).data = jv_mem_alloc(length as size_t) as *const libc::c_char;
    memcpy((*l).data as *mut libc::c_char as *mut libc::c_void,
           data as *const libc::c_void, length as libc::c_ulong);
    (*l).length = length;
    (*l).nlines = 1 as libc::c_int;
    (*l).refct = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        if *data.offset(i as isize) as libc::c_int == '\n' as i32 {
            (*l).nlines += 1
        }
        i += 1
    }
    (*l).linemap =
        jv_mem_calloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                      ((*l).nlines + 1 as libc::c_int) as size_t) as
            *mut libc::c_int;
    *(*l).linemap.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let mut line: libc::c_int = 1 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < length {
        if *data.offset(i_0 as isize) as libc::c_int == '\n' as i32 {
            *(*l).linemap.offset(line as isize) = i_0 + 1 as libc::c_int;
            line += 1
        }
        i_0 += 1
    }
    *(*l).linemap.offset((*l).nlines as isize) = length + 1 as libc::c_int;
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn locfile_retain(mut l: *mut locfile) -> *mut locfile {
    (*l).refct += 1;
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn locfile_free(mut l: *mut locfile) {
    (*l).refct -= 1;
    if (*l).refct == 0 as libc::c_int {
        jv_free((*l).fname);
        jv_mem_free((*l).linemap as *mut libc::c_void);
        jv_mem_free((*l).data as *mut libc::c_char as *mut libc::c_void);
        jv_mem_free(l as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn locfile_get_line(mut l: *mut locfile,
                                          mut pos: libc::c_int)
 -> libc::c_int {
    if pos < (*l).length {
    } else {
        __assert_fail(b"pos < l->length\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/locfile.c\x00" as *const u8 as
                          *const libc::c_char,
                      51 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"int locfile_get_line(struct locfile *, int)\x00")).as_ptr());
    };
    let mut line: libc::c_int = 1 as libc::c_int;
    while *(*l).linemap.offset(line as isize) <= pos { line += 1 }
    if (line - 1 as libc::c_int) < (*l).nlines {
    } else {
        __assert_fail(b"line-1 < l->nlines\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/locfile.c\x00" as *const u8 as
                          *const libc::c_char,
                      54 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"int locfile_get_line(struct locfile *, int)\x00")).as_ptr());
    };
    return line - 1 as libc::c_int;
}
unsafe extern "C" fn locfile_line_length(mut l: *mut locfile,
                                         mut line: libc::c_int)
 -> libc::c_int {
    if line < (*l).nlines {
    } else {
        __assert_fail(b"line < l->nlines\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/locfile.c\x00" as *const u8 as
                          *const libc::c_char,
                      59 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int locfile_line_length(struct locfile *, int)\x00")).as_ptr());
    };
    return *(*l).linemap.offset((line + 1 as libc::c_int) as isize) -
               *(*l).linemap.offset(line as isize) - 1 as libc::c_int;
    // -1 to omit \n
}
#[no_mangle]
pub unsafe extern "C" fn locfile_locate(mut l: *mut locfile,
                                        mut loc: location,
                                        mut fmt: *const libc::c_char,
                                        mut args: ...) {
    let mut fmtargs: ::std::ffi::VaListImpl;
    fmtargs = args.clone();
    let mut startline: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    if loc.start != -(1 as libc::c_int) {
        startline = locfile_get_line(l, loc.start);
        offset = *(*l).linemap.offset(startline as isize)
    }
    let mut m1: jv = jv_string_vfmt(fmt, fmtargs.as_va_list());
    if jv_is_valid(m1) == 0 { jq_report_error((*l).jq, m1); return }
    if loc.start == -(1 as libc::c_int) {
        jq_report_error((*l).jq,
                        jv_string_fmt(b"jq: error: %s\n<unknown location>\x00"
                                          as *const u8 as *const libc::c_char,
                                      jv_string_value(m1)));
        jv_free(m1);
        return
    }
    let mut m2: jv =
        jv_string_fmt(b"%s at %s, line %d:\n%.*s%*s\x00" as *const u8 as
                          *const libc::c_char, jv_string_value(m1),
                      jv_string_value((*l).fname),
                      startline + 1 as libc::c_int,
                      locfile_line_length(l, startline),
                      (*l).data.offset(offset as isize), loc.start - offset,
                      b"\x00" as *const u8 as *const libc::c_char);
    jv_free(m1);
    jq_report_error((*l).jq, m2);
}
