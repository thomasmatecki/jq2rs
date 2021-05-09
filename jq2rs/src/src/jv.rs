#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type Bigint;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jvp_utf8_next(in_0: *const libc::c_char, end: *const libc::c_char,
                     codepoint: *mut libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn jvp_utf8_is_valid(in_0: *const libc::c_char, end: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn jvp_utf8_encode(codepoint: libc::c_int, out: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn _jq_memmem(haystack: *const libc::c_void, haystacklen: size_t,
                  needle: *const libc::c_void, needlelen: size_t)
     -> *const libc::c_void;
    #[no_mangle]
    fn jvp_strtod(C: *mut dtoa_context, s: *const libc::c_char,
                  se: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn tsd_dtoa_context_get() -> *mut dtoa_context;
    #[no_mangle]
    fn decNumberToString(_: *const decNumber, _: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn decNumberReduce(_: *mut decNumber, _: *const decNumber,
                       _: *mut decContext) -> *mut decNumber;
    #[no_mangle]
    fn decNumberCompare(_: *mut decNumber, _: *const decNumber,
                        _: *const decNumber, _: *mut decContext)
     -> *mut decNumber;
    #[no_mangle]
    fn decNumberFromString(_: *mut decNumber, _: *const libc::c_char,
                           _: *mut decContext) -> *mut decNumber;
    #[no_mangle]
    fn pthread_once(__once_control: *mut pthread_once_t,
                    __init_routine: Option<unsafe extern "C" fn() -> ()>)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_setspecific(__key: pthread_key_t,
                           __pointer: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_key_create(__key: *mut pthread_key_t,
                          __destr_function:
                              Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void)
                                         -> ()>) -> libc::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
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
pub type int8_t = libc::c_schar;
pub type int32_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type pthread_key_t = libc::c_uint;
pub type pthread_once_t = libc::c_int;
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
pub type va_list = __builtin_va_list;
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
pub struct jv_refcnt {
    pub count: libc::c_int,
}
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
pub const JVP_PAYLOAD_ALLOCATED: C2RustUnnamed_0 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jvp_literal_number {
    pub refcnt: jv_refcnt,
    pub num_double: libc::c_double,
    pub literal_data: *mut libc::c_char,
    pub num_decimal: decNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decNumber {
    pub digits: int32_t,
    pub exponent: int32_t,
    pub bits: uint8_t,
    pub lsu: [uint16_t; 1],
}
pub const JVP_NUMBER_DECIMAL: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jvp_invalid {
    pub refcnt: jv_refcnt,
    pub errmsg: jv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jvp_object {
    pub refcnt: jv_refcnt,
    pub next_free: libc::c_int,
    pub elements: [object_slot; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_slot {
    pub next: libc::c_int,
    pub hash: uint32_t,
    pub string: jv,
    pub value: jv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jvp_string {
    pub refcnt: jv_refcnt,
    pub hash: uint32_t,
    pub length_hashed: uint32_t,
    pub alloc_length: uint32_t,
    pub data: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jvp_array {
    pub refcnt: jv_refcnt,
    pub length: libc::c_int,
    pub alloc_length: libc::c_int,
    pub elements: [jv; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtoa_context {
    pub freelist: [*mut Bigint; 8],
    pub p5s: *mut Bigint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decNumberDoublePrecision {
    pub number: decNumber,
    pub units: [uint16_t; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decContext {
    pub digits: int32_t,
    pub emax: int32_t,
    pub emin: int32_t,
    pub round: rounding,
    pub traps: uint32_t,
    pub status: uint32_t,
    pub clamp: uint8_t,
}
pub type rounding = libc::c_uint;
pub const DEC_ROUND_MAX: rounding = 8;
pub const DEC_ROUND_05UP: rounding = 7;
pub const DEC_ROUND_FLOOR: rounding = 6;
pub const DEC_ROUND_DOWN: rounding = 5;
pub const DEC_ROUND_HALF_DOWN: rounding = 4;
pub const DEC_ROUND_HALF_EVEN: rounding = 3;
pub const DEC_ROUND_HALF_UP: rounding = 2;
pub const DEC_ROUND_UP: rounding = 1;
pub const DEC_ROUND_CEILING: rounding = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decNumberSingle {
    pub number: decNumber,
    pub units: [uint16_t; 1],
}
pub const JVP_PAYLOAD_NONE: C2RustUnnamed_0 = 0;
pub const ITER_FINISHED: C2RustUnnamed_2 = -2;
pub const JVP_NUMBER_NATIVE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
/*
 * Numbers
 */
pub type C2RustUnnamed_1 = libc::c_uint;
/*
 * Object iteration (internal helpers)
 */
pub type C2RustUnnamed_2 = libc::c_int;
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
static mut JV_REFCNT_INIT: jv_refcnt =
    { let mut init = jv_refcnt{count: 1 as libc::c_int,}; init };
unsafe extern "C" fn jvp_refcnt_inc(mut c: *mut jv_refcnt) {
    (*c).count += 1;
}
unsafe extern "C" fn jvp_refcnt_dec(mut c: *mut jv_refcnt) -> libc::c_int {
    (*c).count -= 1;
    return ((*c).count == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn jvp_refcnt_unshared(mut c: *mut jv_refcnt)
 -> libc::c_int {
    if (*c).count > 0 as libc::c_int {
    } else {
        __assert_fail(b"c->count > 0\x00" as *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      46 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"int jvp_refcnt_unshared(jv_refcnt *)\x00")).as_ptr());
    };
    return ((*c).count == 1 as libc::c_int) as libc::c_int;
}
/*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
#[no_mangle]
pub unsafe extern "C" fn jv_get_kind(mut x: jv) -> jv_kind {
    return (x.kind_flags as libc::c_int & 0xf as libc::c_int) as jv_kind;
}
#[no_mangle]
pub unsafe extern "C" fn jv_kind_name(mut k: jv_kind) -> *const libc::c_char {
    match k as libc::c_uint {
        0 => { return b"<invalid>\x00" as *const u8 as *const libc::c_char }
        1 => { return b"null\x00" as *const u8 as *const libc::c_char }
        2 => { return b"boolean\x00" as *const u8 as *const libc::c_char }
        3 => { return b"boolean\x00" as *const u8 as *const libc::c_char }
        4 => { return b"number\x00" as *const u8 as *const libc::c_char }
        5 => { return b"string\x00" as *const u8 as *const libc::c_char }
        6 => { return b"array\x00" as *const u8 as *const libc::c_char }
        7 => { return b"object\x00" as *const u8 as *const libc::c_char }
        _ => { }
    }
    if 0 as libc::c_int != 0 &&
           !(b"invalid kind\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"0 && \"invalid kind\"\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      90 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"const char *jv_kind_name(jv_kind)\x00")).as_ptr());
    };
    return b"<unknown>\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub static mut JV_NULL: jv =
    {
        let mut init =
            jv{kind_flags:
                   (JV_KIND_NULL as libc::c_int & 0xf as libc::c_int |
                        JVP_PAYLOAD_NONE as libc::c_int & 0xf0 as libc::c_int)
                       as libc::c_uchar,
               pad_: 0 as libc::c_int as libc::c_uchar,
               offset: 0 as libc::c_int as libc::c_ushort,
               size: 0 as libc::c_int,
               u:
                   C2RustUnnamed{ptr:
                                     0 as *const jv_refcnt as
                                         *mut jv_refcnt,},};
        init
    };
#[no_mangle]
pub static mut JV_INVALID: jv =
    {
        let mut init =
            jv{kind_flags:
                   (JV_KIND_INVALID as libc::c_int & 0xf as libc::c_int |
                        JVP_PAYLOAD_NONE as libc::c_int & 0xf0 as libc::c_int)
                       as libc::c_uchar,
               pad_: 0 as libc::c_int as libc::c_uchar,
               offset: 0 as libc::c_int as libc::c_ushort,
               size: 0 as libc::c_int,
               u:
                   C2RustUnnamed{ptr:
                                     0 as *const jv_refcnt as
                                         *mut jv_refcnt,},};
        init
    };
#[no_mangle]
pub static mut JV_FALSE: jv =
    {
        let mut init =
            jv{kind_flags:
                   (JV_KIND_FALSE as libc::c_int & 0xf as libc::c_int |
                        JVP_PAYLOAD_NONE as libc::c_int & 0xf0 as libc::c_int)
                       as libc::c_uchar,
               pad_: 0 as libc::c_int as libc::c_uchar,
               offset: 0 as libc::c_int as libc::c_ushort,
               size: 0 as libc::c_int,
               u:
                   C2RustUnnamed{ptr:
                                     0 as *const jv_refcnt as
                                         *mut jv_refcnt,},};
        init
    };
#[no_mangle]
pub static mut JV_TRUE: jv =
    {
        let mut init =
            jv{kind_flags:
                   (JV_KIND_TRUE as libc::c_int & 0xf as libc::c_int |
                        JVP_PAYLOAD_NONE as libc::c_int & 0xf0 as libc::c_int)
                       as libc::c_uchar,
               pad_: 0 as libc::c_int as libc::c_uchar,
               offset: 0 as libc::c_int as libc::c_ushort,
               size: 0 as libc::c_int,
               u:
                   C2RustUnnamed{ptr:
                                     0 as *const jv_refcnt as
                                         *mut jv_refcnt,},};
        init
    };
#[no_mangle]
pub unsafe extern "C" fn jv_true() -> jv { return JV_TRUE; }
#[no_mangle]
pub unsafe extern "C" fn jv_false() -> jv { return JV_FALSE; }
#[no_mangle]
pub unsafe extern "C" fn jv_null() -> jv { return JV_NULL; }
#[no_mangle]
pub unsafe extern "C" fn jv_bool(mut x: libc::c_int) -> jv {
    return if x != 0 { JV_TRUE } else { JV_FALSE };
}
#[no_mangle]
pub unsafe extern "C" fn jv_invalid_with_msg(mut err: jv) -> jv {
    if err.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NULL as libc::c_int {
        return JV_INVALID
    }
    let mut i: *mut jvp_invalid =
        jv_mem_alloc(::std::mem::size_of::<jvp_invalid>() as libc::c_ulong) as
            *mut jvp_invalid;
    (*i).refcnt = JV_REFCNT_INIT;
    (*i).errmsg = err;
    let mut x: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_INVALID as libc::c_int & 0xf as libc::c_int |
                            JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u: C2RustUnnamed{ptr: &mut (*i).refcnt,},};
            init
        };
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn jv_invalid() -> jv { return JV_INVALID; }
#[no_mangle]
pub unsafe extern "C" fn jv_invalid_get_msg(mut inv: jv) -> jv {
    if inv.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_INVALID as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(inv, JV_KIND_INVALID)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      142 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"jv jv_invalid_get_msg(jv)\x00")).as_ptr());
    };
    let mut x: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if inv.kind_flags as libc::c_int ==
           JV_KIND_INVALID as libc::c_int & 0xf as libc::c_int |
               JVP_PAYLOAD_ALLOCATED as libc::c_int & 0xf0 as libc::c_int {
        x = jv_copy((*(inv.u.ptr as *mut jvp_invalid)).errmsg)
    } else { x = jv_null() }
    jv_free(inv);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn jv_invalid_has_msg(mut inv: jv) -> libc::c_int {
    if inv.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_INVALID as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(inv, JV_KIND_INVALID)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      157 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"int jv_invalid_has_msg(jv)\x00")).as_ptr());
    };
    let mut r: libc::c_int =
        (inv.kind_flags as libc::c_int ==
             JV_KIND_INVALID as libc::c_int & 0xf as libc::c_int |
                 JVP_PAYLOAD_ALLOCATED as libc::c_int & 0xf0 as libc::c_int)
            as libc::c_int;
    jv_free(inv);
    return r;
}
unsafe extern "C" fn jvp_invalid_free(mut x: jv) {
    if x.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_INVALID as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(x, JV_KIND_INVALID)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      164 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void jvp_invalid_free(jv)\x00")).as_ptr());
    };
    if x.kind_flags as libc::c_int ==
           JV_KIND_INVALID as libc::c_int & 0xf as libc::c_int |
               JVP_PAYLOAD_ALLOCATED as libc::c_int & 0xf0 as libc::c_int &&
           jvp_refcnt_dec(x.u.ptr) != 0 {
        jv_free((*(x.u.ptr as *mut jvp_invalid)).errmsg);
        jv_mem_free(x.u.ptr as *mut libc::c_void);
    };
}
static mut dec_ctx_key: pthread_key_t = 0;
static mut dec_ctx_dbl_key: pthread_key_t = 0;
static mut dec_ctx_once: pthread_once_t = 0 as libc::c_int;
// atexit finalizer to clean up the tsd dec contexts if main() exits
// without having called pthread_exit()
unsafe extern "C" fn tsd_dec_ctx_fini() {
    jv_mem_free(pthread_getspecific(dec_ctx_key)); // cannot fail
    jv_mem_free(pthread_getspecific(dec_ctx_dbl_key));
    pthread_setspecific(dec_ctx_key, 0 as *const libc::c_void);
    pthread_setspecific(dec_ctx_dbl_key, 0 as *const libc::c_void);
}
unsafe extern "C" fn tsd_dec_ctx_init() {
    if pthread_key_create(&mut dec_ctx_key,
                          Some(jv_mem_free as
                                   unsafe extern "C" fn(_: *mut libc::c_void)
                                       -> ())) != 0 as libc::c_int {
        fprintf(stderr,
                b"error: cannot create thread specific key\x00" as *const u8
                    as *const libc::c_char);
        abort();
    }
    if pthread_key_create(&mut dec_ctx_dbl_key,
                          Some(jv_mem_free as
                                   unsafe extern "C" fn(_: *mut libc::c_void)
                                       -> ())) != 0 as libc::c_int {
        fprintf(stderr,
                b"error: cannot create thread specific key\x00" as *const u8
                    as *const libc::c_char);
        abort();
    }
    atexit(::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                   Option<unsafe extern "C" fn()
                                              ->
                                                  ()>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                        ->
                                                                                            (),
                                                                                    unsafe extern "C" fn()
                                                                                        ->
                                                                                            ()>(tsd_dec_ctx_fini))));
}
unsafe extern "C" fn tsd_dec_ctx_get(mut key: *mut pthread_key_t)
 -> *mut decContext {
    pthread_once(&mut dec_ctx_once,
                 ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                         Option<unsafe extern "C" fn()
                                                    ->
                                                        ()>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                              ->
                                                                                                  (),
                                                                                          unsafe extern "C" fn()
                                                                                              ->
                                                                                                  ()>(tsd_dec_ctx_init))));
    let mut ctx: *mut decContext =
        pthread_getspecific(*key) as *mut decContext;
    if !ctx.is_null() { return ctx }
    let mut _ctx: decContext =
        {
            let mut init =
                decContext{digits: 0 as libc::c_int,
                           emax: 999999999 as libc::c_int,
                           emin: 0 as libc::c_int,
                           round: DEC_ROUND_HALF_UP,
                           traps: 0 as libc::c_int as uint32_t,
                           status: 0 as libc::c_int as uint32_t,
                           clamp: 0 as libc::c_int as uint8_t,};
            init
        };
    if key == &mut dec_ctx_key as *mut pthread_key_t {
        _ctx.digits = 999999999 as libc::c_int
    } else if key == &mut dec_ctx_dbl_key as *mut pthread_key_t {
        _ctx.digits = 17 as libc::c_int
    }
    ctx =
        malloc(::std::mem::size_of::<decContext>() as libc::c_ulong) as
            *mut decContext;
    if !ctx.is_null() {
        *ctx = _ctx;
        if pthread_setspecific(*key, ctx as *const libc::c_void) !=
               0 as libc::c_int {
            fprintf(stderr,
                    b"error: cannot store thread specific data\x00" as
                        *const u8 as *const libc::c_char);
            abort();
        }
    }
    return ctx;
}
unsafe extern "C" fn jvp_literal_number_ptr(mut j: jv)
 -> *mut jvp_literal_number {
    if j.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_FLAGS(j, JVP_FLAGS_NUMBER_LITERAL)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      283 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"jvp_literal_number *jvp_literal_number_ptr(jv)\x00")).as_ptr());
    };
    return j.u.ptr as *mut jvp_literal_number;
}
unsafe extern "C" fn jvp_dec_number_ptr(mut j: jv) -> *mut decNumber {
    if j.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_FLAGS(j, JVP_FLAGS_NUMBER_LITERAL)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      288 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"decNumber *jvp_dec_number_ptr(jv)\x00")).as_ptr());
    };
    return &mut (*(j.u.ptr as *mut jvp_literal_number)).num_decimal;
}
unsafe extern "C" fn jvp_literal_number_alloc(mut literal_length:
                                                  libc::c_uint)
 -> *mut jvp_literal_number {
    /* The number of units needed is ceil(DECNUMDIGITS/DECDPUN)         */
    let mut units: libc::c_int =
        literal_length.wrapping_add(3 as libc::c_int as
                                        libc::c_uint).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_div(3
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint)
            as libc::c_int;
    let mut n: *mut jvp_literal_number =
        jv_mem_alloc((::std::mem::size_of::<jvp_literal_number>() as
                          libc::c_ulong).wrapping_add((::std::mem::size_of::<uint16_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(units
                                                                                           as
                                                                                           libc::c_ulong)))
            as *mut jvp_literal_number;
    return n;
}
unsafe extern "C" fn jvp_literal_number_new(mut literal: *const libc::c_char)
 -> jv {
    let mut n: *mut jvp_literal_number =
        jvp_literal_number_alloc(strlen(literal) as libc::c_uint);
    (*n).refcnt = JV_REFCNT_INIT;
    (*n).literal_data = 0 as *mut libc::c_char;
    let mut ctx: *mut decContext = tsd_dec_ctx_get(&mut dec_ctx_key);
    decNumberFromString(&mut (*n).num_decimal, literal, ctx);
    (*n).num_double = ::std::f32::NAN as libc::c_double;
    if (*ctx).status & 0x1 as libc::c_int as libc::c_uint != 0 {
        jv_mem_free(n as *mut libc::c_void);
        return JV_INVALID
    }
    let mut r: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
                            ((JVP_NUMBER_DECIMAL as libc::c_int) <<
                                 4 as libc::c_int & 0x70 as libc::c_int |
                                 (if 1 as libc::c_int != 0 {
                                      JVP_PAYLOAD_ALLOCATED as libc::c_int
                                  } else { 0 as libc::c_int })) &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u: C2RustUnnamed{ptr: &mut (*n).refcnt,},};
            init
        };
    return r;
}
unsafe extern "C" fn jvp_literal_number_to_double(mut j: jv)
 -> libc::c_double {
    if j.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_FLAGS(j, JVP_FLAGS_NUMBER_LITERAL)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      325 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"double jvp_literal_number_to_double(jv)\x00")).as_ptr());
    };
    let mut p_dec_number: *mut decNumber = jvp_dec_number_ptr(j);
    let mut dec_double: decNumberDoublePrecision =
        decNumberDoublePrecision{number:
                                     decNumber{digits: 0,
                                               exponent: 0,
                                               bits: 0,
                                               lsu: [0; 1],},
                                 units: [0; 17],};
    let mut literal: [libc::c_char; 32] = [0; 32];
    // reduce the number to the shortest possible form
  // while also making sure than no more than BIN64_DEC_PRECISION 
  // digits are used (dec_context_to_double)
    decNumberReduce(&mut dec_double.number, p_dec_number,
                    tsd_dec_ctx_get(&mut dec_ctx_dbl_key));
    decNumberToString(&mut dec_double.number, literal.as_mut_ptr());
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    return jvp_strtod(tsd_dtoa_context_get(), literal.as_mut_ptr(), &mut end);
}
unsafe extern "C" fn jvp_number_equal(mut a: jv, mut b: jv) -> libc::c_int {
    return (jvp_number_cmp(a, b) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn jvp_literal_number_literal(mut n: jv)
 -> *const libc::c_char {
    if n.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_FLAGS(n, JVP_FLAGS_NUMBER_LITERAL)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      348 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"const char *jvp_literal_number_literal(jv)\x00")).as_ptr());
    };
    let mut pdec: *mut decNumber = jvp_dec_number_ptr(n);
    let mut plit: *mut jvp_literal_number = jvp_literal_number_ptr(n);
    if (*pdec).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int {
        return b"null\x00" as *const u8 as *const libc::c_char
    }
    if (*pdec).bits as libc::c_int & 0x40 as libc::c_int != 0 as libc::c_int {
        // For backward compatibility.
        if (*pdec).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
            return b"-1.7976931348623157e+308\x00" as *const u8 as
                       *const libc::c_char
        } else {
            return b"1.7976931348623157e+308\x00" as *const u8 as
                       *const libc::c_char
        }
    }
    if (*plit).literal_data.is_null() {
        let mut len: libc::c_int =
            (*jvp_dec_number_ptr(n)).digits + 14 as libc::c_int;
        (*plit).literal_data =
            jv_mem_alloc(len as size_t) as *mut libc::c_char;
        // Preserve the actual precision as we have parsed it
    // don't do decNumberTrim(pdec);
        decNumberToString(pdec, (*plit).literal_data);
    }
    return (*plit).literal_data;
}
#[no_mangle]
pub unsafe extern "C" fn jv_number_has_literal(mut n: jv) -> libc::c_int {
    if n.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(n, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      379 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"int jv_number_has_literal(jv)\x00")).as_ptr());
    };
    return (n.kind_flags as libc::c_int ==
                JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
                    ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                         0x70 as libc::c_int |
                         (if 1 as libc::c_int != 0 {
                              JVP_PAYLOAD_ALLOCATED as libc::c_int
                          } else { 0 as libc::c_int })) & 0xf0 as libc::c_int)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jv_number_get_literal(mut n: jv)
 -> *const libc::c_char {
    if n.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(n, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      384 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"const char *jv_number_get_literal(jv)\x00")).as_ptr());
    };
    if n.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
        return jvp_literal_number_literal(n)
    } else { return 0 as *const libc::c_char };
}
unsafe extern "C" fn jvp_number_free(mut j: jv) {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      394 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void jvp_number_free(jv)\x00")).as_ptr());
    };
    if j.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int &&
           jvp_refcnt_dec(j.u.ptr) != 0 {
        let mut n: *mut jvp_literal_number = jvp_literal_number_ptr(j);
        if !(*n).literal_data.is_null() {
            jv_mem_free((*n).literal_data as *mut libc::c_void);
        }
        jv_mem_free(n as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_number_with_literal(mut literal:
                                                    *const libc::c_char)
 -> jv {
    return jvp_literal_number_new(literal);
}
#[no_mangle]
pub unsafe extern "C" fn jv_number(mut x: libc::c_double) -> jv {
    let mut j: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
                            ((JVP_NUMBER_NATIVE as libc::c_int) <<
                                 4 as libc::c_int & 0x70 as libc::c_int |
                                 (if 0 as libc::c_int != 0 {
                                      JVP_PAYLOAD_ALLOCATED as libc::c_int
                                  } else { 0 as libc::c_int })) &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u: C2RustUnnamed{number: x,},};
            init
        };
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn jv_number_value(mut j: jv) -> libc::c_double {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      414 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"double jv_number_value(jv)\x00")).as_ptr());
    };
    if j.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
        let mut n: *mut jvp_literal_number = jvp_literal_number_ptr(j);
        if j.size != 1 as libc::c_int {
            (*n).num_double = jvp_literal_number_to_double(j);
            j.size = 1 as libc::c_int
        }
        return (*n).num_double
    } else { return j.u.number };
}
#[no_mangle]
pub unsafe extern "C" fn jv_is_integer(mut j: jv) -> libc::c_int {
    if !(j.kind_flags as libc::c_int & 0xf as libc::c_int ==
             JV_KIND_NUMBER as libc::c_int) {
        return 0 as libc::c_int
    }
    let mut x: libc::c_double = jv_number_value(j);
    let mut ipart: libc::c_double = 0.;
    let mut fpart: libc::c_double = modf(x, &mut ipart);
    return (fabs(fpart) < 2.2204460492503131e-16f64) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jvp_number_is_nan(mut n: jv) -> libc::c_int {
    if n.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(n, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      447 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"int jvp_number_is_nan(jv)\x00")).as_ptr());
    };
    if n.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
        let mut pdec: *mut decNumber = jvp_dec_number_ptr(n);
        return ((*pdec).bits as libc::c_int &
                    (0x20 as libc::c_int | 0x10 as libc::c_int) !=
                    0 as libc::c_int) as libc::c_int
    } else { return (n.u.number != n.u.number) as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn jvp_number_cmp(mut a: jv, mut b: jv) -> libc::c_int {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      458 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"int jvp_number_cmp(jv, jv)\x00")).as_ptr());
    };
    if b.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(b, JV_KIND_NUMBER)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      459 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"int jvp_number_cmp(jv, jv)\x00")).as_ptr());
    };
    if a.kind_flags as libc::c_int ==
           JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
               ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                    0x70 as libc::c_int |
                    (if 1 as libc::c_int != 0 {
                         JVP_PAYLOAD_ALLOCATED as libc::c_int
                     } else { 0 as libc::c_int })) & 0xf0 as libc::c_int &&
           b.kind_flags as libc::c_int ==
               JV_KIND_NUMBER as libc::c_int & 0xf as libc::c_int |
                   ((JVP_NUMBER_DECIMAL as libc::c_int) << 4 as libc::c_int &
                        0x70 as libc::c_int |
                        (if 1 as libc::c_int != 0 {
                             JVP_PAYLOAD_ALLOCATED as libc::c_int
                         } else { 0 as libc::c_int })) & 0xf0 as libc::c_int {
        let mut res: decNumberSingle =
            decNumberSingle{number:
                                decNumber{digits: 0,
                                          exponent: 0,
                                          bits: 0,
                                          lsu: [0; 1],},
                            units: [0; 1],};
        decNumberCompare(&mut res.number, jvp_dec_number_ptr(a),
                         jvp_dec_number_ptr(b),
                         tsd_dec_ctx_get(&mut dec_ctx_key));
        if *res.number.lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
               res.number.digits == 1 as libc::c_int &&
               res.number.bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) == 0 as libc::c_int {
            return 0 as libc::c_int
        } else if res.number.bits as libc::c_int & 0x80 as libc::c_int !=
                      0 as libc::c_int {
            return -(1 as libc::c_int)
        } else { return 1 as libc::c_int }
    } else {
        let mut da: libc::c_double = jv_number_value(a);
        let mut db: libc::c_double = jv_number_value(b);
        if da < db {
            return -(1 as libc::c_int)
        } else if da == db {
            return 0 as libc::c_int
        } else { return 1 as libc::c_int }
    };
}
unsafe extern "C" fn imax(mut a: libc::c_int, mut b: libc::c_int)
 -> libc::c_int {
    if a > b { return a } else { return b };
}
unsafe extern "C" fn jvp_array_ptr(mut a: jv) -> *mut jvp_array {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      507 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"jvp_array *jvp_array_ptr(jv)\x00")).as_ptr());
    };
    return a.u.ptr as *mut jvp_array;
}
unsafe extern "C" fn jvp_array_alloc(mut size: libc::c_uint)
 -> *mut jvp_array {
    let mut a: *mut jvp_array =
        jv_mem_alloc((::std::mem::size_of::<jvp_array>() as
                          libc::c_ulong).wrapping_add((::std::mem::size_of::<jv>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(size
                                                                                           as
                                                                                           libc::c_ulong)))
            as *mut jvp_array;
    (*a).refcnt.count = 1 as libc::c_int;
    (*a).length = 0 as libc::c_int;
    (*a).alloc_length = size as libc::c_int;
    return a;
}
unsafe extern "C" fn jvp_array_new(mut size: libc::c_uint) -> jv {
    let mut r: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_ARRAY as libc::c_int & 0xf as libc::c_int |
                            JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u:
                       C2RustUnnamed{ptr:
                                         &mut (*(jvp_array_alloc as
                                                     unsafe extern "C" fn(_:
                                                                              libc::c_uint)
                                                         ->
                                                             *mut jvp_array)(size)).refcnt,},};
            init
        };
    return r;
}
unsafe extern "C" fn jvp_array_free(mut a: jv) {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      525 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"void jvp_array_free(jv)\x00")).as_ptr());
    };
    if jvp_refcnt_dec(a.u.ptr) != 0 {
        let mut array: *mut jvp_array = jvp_array_ptr(a);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*array).length {
            jv_free(*(*array).elements.as_mut_ptr().offset(i as isize));
            i += 1
        }
        jv_mem_free(array as *mut libc::c_void);
    };
}
unsafe extern "C" fn jvp_array_length(mut a: jv) -> libc::c_int {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      536 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"int jvp_array_length(jv)\x00")).as_ptr());
    };
    return a.size;
}
unsafe extern "C" fn jvp_array_offset(mut a: jv) -> libc::c_int {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      541 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"int jvp_array_offset(jv)\x00")).as_ptr());
    };
    return a.offset as libc::c_int;
}
unsafe extern "C" fn jvp_array_read(mut a: jv, mut i: libc::c_int)
 -> *mut jv {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      546 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"jv *jvp_array_read(jv, int)\x00")).as_ptr());
    };
    if i >= 0 as libc::c_int && i < jvp_array_length(a) {
        let mut array: *mut jvp_array = jvp_array_ptr(a);
        if i + jvp_array_offset(a) < (*array).length {
        } else {
            __assert_fail(b"i + jvp_array_offset(a) < array->length\x00" as
                              *const u8 as *const libc::c_char,
                          b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                          549 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_char; 28]>(b"jv *jvp_array_read(jv, int)\x00")).as_ptr());
        };
        return &mut *(*array).elements.as_mut_ptr().offset((i +
                                                                (jvp_array_offset
                                                                     as
                                                                     unsafe extern "C" fn(_:
                                                                                              jv)
                                                                         ->
                                                                             libc::c_int)(a))
                                                               as isize) as
                   *mut jv
    } else { return 0 as *mut jv };
}
unsafe extern "C" fn jvp_array_write(mut a: *mut jv, mut i: libc::c_int)
 -> *mut jv {
    if i >= 0 as libc::c_int {
    } else {
        __assert_fail(b"i >= 0\x00" as *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      557 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"jv *jvp_array_write(jv *, int)\x00")).as_ptr());
    };
    let mut array: *mut jvp_array = jvp_array_ptr(*a);
    let mut pos: libc::c_int = i + jvp_array_offset(*a);
    if pos < (*array).alloc_length && jvp_refcnt_unshared((*a).u.ptr) != 0 {
        // use existing array space
        let mut j: libc::c_int = (*array).length;
        while j <= pos {
            *(*array).elements.as_mut_ptr().offset(j as isize) = JV_NULL;
            j += 1
        }
        (*array).length = imax(pos + 1 as libc::c_int, (*array).length);
        (*a).size = imax(i + 1 as libc::c_int, (*a).size);
        return &mut *(*array).elements.as_mut_ptr().offset(pos as isize) as
                   *mut jv
    } else {
        // allocate a new array
        let mut new_length: libc::c_int =
            imax(i + 1 as libc::c_int, jvp_array_length(*a));
        let mut new_array: *mut jvp_array =
            jvp_array_alloc((new_length * 3 as libc::c_int / 2 as libc::c_int)
                                as libc::c_uint);
        let mut j_0: libc::c_int = 0;
        j_0 = 0 as libc::c_int;
        while j_0 < jvp_array_length(*a) {
            *(*new_array).elements.as_mut_ptr().offset(j_0 as isize) =
                jv_copy(*(*array).elements.as_mut_ptr().offset((j_0 +
                                                                    jvp_array_offset(*a))
                                                                   as isize));
            j_0 += 1
        }
        while j_0 < new_length {
            *(*new_array).elements.as_mut_ptr().offset(j_0 as isize) =
                JV_NULL;
            j_0 += 1
        }
        (*new_array).length = new_length;
        jvp_array_free(*a);
        let mut new_jv: jv =
            {
                let mut init =
                    jv{kind_flags:
                           (JV_KIND_ARRAY as libc::c_int & 0xf as libc::c_int
                                |
                                JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                    0xf0 as libc::c_int) as libc::c_uchar,
                       pad_: 0 as libc::c_int as libc::c_uchar,
                       offset: 0 as libc::c_int as libc::c_ushort,
                       size: new_length,
                       u: C2RustUnnamed{ptr: &mut (*new_array).refcnt,},};
                init
            };
        *a = new_jv;
        return &mut *(*new_array).elements.as_mut_ptr().offset(i as isize) as
                   *mut jv
    };
}
unsafe extern "C" fn jvp_array_equal(mut a: jv, mut b: jv) -> libc::c_int {
    if jvp_array_length(a) != jvp_array_length(b) { return 0 as libc::c_int }
    if jvp_array_ptr(a) == jvp_array_ptr(b) &&
           jvp_array_offset(a) == jvp_array_offset(b) {
        return 1 as libc::c_int
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < jvp_array_length(a) {
        if jv_equal(jv_copy(*jvp_array_read(a, i)),
                    jv_copy(*jvp_array_read(b, i))) == 0 {
            return 0 as libc::c_int
        }
        i += 1
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn jvp_clamp_slice_params(mut len: libc::c_int,
                                            mut pstart: *mut libc::c_int,
                                            mut pend: *mut libc::c_int) {
    if *pstart < 0 as libc::c_int { *pstart = len + *pstart }
    if *pend < 0 as libc::c_int { *pend = len + *pend }
    if *pstart < 0 as libc::c_int { *pstart = 0 as libc::c_int }
    if *pstart > len { *pstart = len }
    if *pend > len { *pend = len }
    if *pend < *pstart { *pend = *pstart };
}
unsafe extern "C" fn jvp_array_contains(mut a: jv, mut b: jv) -> libc::c_int {
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut jv_len__: libc::c_int = jv_array_length(jv_copy(b));
    let mut bi: libc::c_int = 0 as libc::c_int;
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut belem: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if bi < jv_len__ {
                  belem = jv_array_get(jv_copy(b), bi);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            let mut ri: libc::c_int = 0 as libc::c_int;
            let mut jv_len___0: libc::c_int = jv_array_length(jv_copy(a));
            let mut ai: libc::c_int = 0 as libc::c_int;
            let mut jv_j___0: libc::c_int = 1 as libc::c_int;
            while jv_j___0 != 0 {
                let mut aelem: jv =
                    jv{kind_flags: 0,
                       pad_: 0,
                       offset: 0,
                       size: 0,
                       u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                while if ai < jv_len___0 {
                          aelem = jv_array_get(jv_copy(a), ai);
                          1 as libc::c_int
                      } else { 0 as libc::c_int } != 0 {
                    if jv_contains(aelem, jv_copy(belem)) != 0 {
                        ri = 1 as libc::c_int;
                        break ;
                    } else { ai += 1 }
                }
                jv_j___0 = 0 as libc::c_int
            }
            jv_free(belem);
            if ri == 0 { r = 0 as libc::c_int; break ; } else { bi += 1 }
        }
        jv_j__ = 0 as libc::c_int
    }
    return r;
}
/*
 * Public
 */
unsafe extern "C" fn jvp_array_slice(mut a: jv, mut start: libc::c_int,
                                     mut end: libc::c_int) -> jv {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      640 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"jv jvp_array_slice(jv, int, int)\x00")).as_ptr());
    };
    let mut len: libc::c_int = jvp_array_length(a);
    jvp_clamp_slice_params(len, &mut start, &mut end);
    if 0 as libc::c_int <= start && start <= end && end <= len {
    } else {
        __assert_fail(b"0 <= start && start <= end && end <= len\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      643 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"jv jvp_array_slice(jv, int, int)\x00")).as_ptr());
    };
    // FIXME: maybe slice should reallocate if the slice is small enough
    if start == end { jv_free(a); return jv_array() }
    if a.offset as libc::c_int + start >=
           (1 as libc::c_int) <<
               (::std::mem::size_of::<libc::c_ushort>() as
                    libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                    libc::c_ulong) {
        let mut r: jv = jv_array_sized(end - start);
        let mut i: libc::c_int = start;
        while i < end {
            r = jv_array_append(r, jv_array_get(jv_copy(a), i));
            i += 1
        }
        jv_free(a);
        return r
    } else {
        a.offset = (a.offset as libc::c_int + start) as libc::c_ushort;
        a.size = end - start;
        return a
    };
}
/*
 * Arrays (public interface)
 */
#[no_mangle]
pub unsafe extern "C" fn jv_array_sized(mut n: libc::c_int) -> jv {
    return jvp_array_new(n as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn jv_array() -> jv {
    return jv_array_sized(16 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_length(mut j: jv) -> libc::c_int {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      677 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"int jv_array_length(jv)\x00")).as_ptr());
    };
    let mut len: libc::c_int = jvp_array_length(j);
    jv_free(j);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_get(mut j: jv, mut idx: libc::c_int) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      684 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jv_array_get(jv, int)\x00")).as_ptr());
    };
    let mut slot: *mut jv = jvp_array_read(j, idx);
    let mut val: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if !slot.is_null() { val = jv_copy(*slot) } else { val = jv_invalid() }
    jv_free(j);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_set(mut j: jv, mut idx: libc::c_int,
                                      mut val: jv) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      697 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"jv jv_array_set(jv, int, jv)\x00")).as_ptr());
    };
    if idx < 0 as libc::c_int { idx = jvp_array_length(j) + idx }
    if idx < 0 as libc::c_int {
        jv_free(j);
        jv_free(val);
        return jv_invalid_with_msg(jv_string(b"Out of bounds negative array index\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    // copy/free of val,j coalesced
    let mut slot: *mut jv = jvp_array_write(&mut j, idx);
    jv_free(*slot);
    *slot = val;
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_append(mut j: jv, mut val: jv) -> jv {
    // copy/free of val,j coalesced
    return jv_array_set(j, jv_array_length(jv_copy(j)), val);
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_concat(mut a: jv, mut b: jv) -> jv {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      719 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"jv jv_array_concat(jv, jv)\x00")).as_ptr());
    };
    if b.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(b, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      720 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"jv jv_array_concat(jv, jv)\x00")).as_ptr());
    };
    // FIXME: could be faster
    let mut jv_len__: libc::c_int = jv_array_length(jv_copy(b));
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut elem: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if i < jv_len__ {
                  elem = jv_array_get(jv_copy(b), i);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            a = jv_array_append(a, elem);
            i += 1
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(b);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_slice(mut a: jv, mut start: libc::c_int,
                                        mut end: libc::c_int) -> jv {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      731 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"jv jv_array_slice(jv, int, int)\x00")).as_ptr());
    };
    // copy/free of a coalesced
    return jvp_array_slice(a, start, end);
}
#[no_mangle]
pub unsafe extern "C" fn jv_array_indexes(mut a: jv, mut b: jv) -> jv {
    let mut res: jv = jv_array();
    let mut idx: libc::c_int = -(1 as libc::c_int);
    let mut jv_len__: libc::c_int = jv_array_length(jv_copy(a));
    let mut ai: libc::c_int = 0 as libc::c_int;
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut aelem: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if ai < jv_len__ {
                  aelem = jv_array_get(jv_copy(a), ai);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            jv_free(aelem);
            let mut jv_len___0: libc::c_int = jv_array_length(jv_copy(b));
            let mut bi: libc::c_int = 0 as libc::c_int;
            let mut jv_j___0: libc::c_int = 1 as libc::c_int;
            while jv_j___0 != 0 {
                let mut belem: jv =
                    jv{kind_flags: 0,
                       pad_: 0,
                       offset: 0,
                       size: 0,
                       u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                while if bi < jv_len___0 {
                          belem = jv_array_get(jv_copy(b), bi);
                          1 as libc::c_int
                      } else { 0 as libc::c_int } != 0 {
                    if jv_equal(jv_array_get(jv_copy(a), ai + bi),
                                jv_copy(belem)) == 0 {
                        idx = -(1 as libc::c_int)
                    } else if bi == 0 as libc::c_int &&
                                  idx == -(1 as libc::c_int) {
                        idx = ai
                    }
                    jv_free(belem);
                    bi += 1
                }
                jv_j___0 = 0 as libc::c_int
            }
            if idx > -(1 as libc::c_int) {
                res = jv_array_append(res, jv_number(idx as libc::c_double))
            }
            idx = -(1 as libc::c_int);
            ai += 1
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(a);
    jv_free(b);
    return res;
}
unsafe extern "C" fn jvp_string_ptr(mut a: jv) -> *mut jvp_string {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      774 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"jvp_string *jvp_string_ptr(jv)\x00")).as_ptr());
    };
    return a.u.ptr as *mut jvp_string;
}
unsafe extern "C" fn jvp_string_alloc(mut size: uint32_t) -> *mut jvp_string {
    let mut s: *mut jvp_string =
        jv_mem_alloc((::std::mem::size_of::<jvp_string>() as
                          libc::c_ulong).wrapping_add(size as
                                                          libc::c_ulong).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
            as *mut jvp_string;
    (*s).refcnt.count = 1 as libc::c_int;
    (*s).alloc_length = size;
    return s;
}
/* Copy a UTF8 string, replacing all badly encoded points with U+FFFD */
unsafe extern "C" fn jvp_string_copy_replace_bad(mut data:
                                                     *const libc::c_char,
                                                 mut length: uint32_t) -> jv {
    let mut end: *const libc::c_char =
        data.offset(length as
                        isize); // worst case: all bad bytes, each becomes a 3-byte U+FFFD
    let mut i: *const libc::c_char = data;
    let mut cstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut maxlength: uint32_t =
        length.wrapping_mul(3 as libc::c_int as
                                libc::c_uint).wrapping_add(1 as libc::c_int as
                                                               libc::c_uint);
    let mut s: *mut jvp_string = jvp_string_alloc(maxlength);
    let mut out: *mut libc::c_char = (*s).data.as_mut_ptr();
    let mut c: libc::c_int = 0 as libc::c_int;
    loop  {
        cstart = i;
        i = jvp_utf8_next(cstart, end, &mut c);
        if i.is_null() { break ; }
        if c == -(1 as libc::c_int) {
            c = 0xfffd as libc::c_int
            // U+FFFD REPLACEMENT CHARACTER
        }
        out = out.offset(jvp_utf8_encode(c, out) as isize);
        if out < (*s).data.as_mut_ptr().offset(maxlength as isize) {
        } else {
            __assert_fail(b"out < s->data + maxlength\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                          801 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 55],
                                                    &[libc::c_char; 55]>(b"jv jvp_string_copy_replace_bad(const char *, uint32_t)\x00")).as_ptr());
        };
    }
    length =
        out.wrapping_offset_from((*s).data.as_mut_ptr()) as libc::c_long as
            uint32_t;
    *(*s).data.as_mut_ptr().offset(length as isize) =
        0 as libc::c_int as libc::c_char;
    (*s).length_hashed = length << 1 as libc::c_int;
    let mut r: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_STRING as libc::c_int & 0xf as libc::c_int |
                            JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u: C2RustUnnamed{ptr: &mut (*s).refcnt,},};
            init
        };
    return r;
}
/* Assumes valid UTF8 */
unsafe extern "C" fn jvp_string_new(mut data: *const libc::c_char,
                                    mut length: uint32_t) -> jv {
    let mut s: *mut jvp_string = jvp_string_alloc(length);
    (*s).length_hashed = length << 1 as libc::c_int;
    if !data.is_null() {
        memcpy((*s).data.as_mut_ptr() as *mut libc::c_void,
               data as *const libc::c_void, length as libc::c_ulong);
    }
    *(*s).data.as_mut_ptr().offset(length as isize) =
        0 as libc::c_int as libc::c_char;
    let mut r: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_STRING as libc::c_int & 0xf as libc::c_int |
                            JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u: C2RustUnnamed{ptr: &mut (*s).refcnt,},};
            init
        };
    return r;
}
unsafe extern "C" fn jvp_string_empty_new(mut length: uint32_t) -> jv {
    let mut s: *mut jvp_string = jvp_string_alloc(length);
    (*s).length_hashed = 0 as libc::c_int as uint32_t;
    memset((*s).data.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           length as libc::c_ulong);
    let mut r: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_STRING as libc::c_int & 0xf as libc::c_int |
                            JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: 0 as libc::c_int,
                   u: C2RustUnnamed{ptr: &mut (*s).refcnt,},};
            init
        };
    return r;
}
unsafe extern "C" fn jvp_string_free(mut js: jv) {
    let mut s: *mut jvp_string = jvp_string_ptr(js);
    if jvp_refcnt_dec(&mut (*s).refcnt) != 0 {
        jv_mem_free(s as *mut libc::c_void);
    };
}
unsafe extern "C" fn jvp_string_length(mut s: *mut jvp_string) -> uint32_t {
    return (*s).length_hashed >> 1 as libc::c_int;
}
unsafe extern "C" fn jvp_string_remaining_space(mut s: *mut jvp_string)
 -> uint32_t {
    if (*s).alloc_length >= jvp_string_length(s) {
    } else {
        __assert_fail(b"s->alloc_length >= jvp_string_length(s)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      842 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"uint32_t jvp_string_remaining_space(jvp_string *)\x00")).as_ptr());
    };
    let mut r: uint32_t =
        (*s).alloc_length.wrapping_sub(jvp_string_length(s));
    return r;
}
unsafe extern "C" fn jvp_string_append(mut string: jv,
                                       mut data: *const libc::c_char,
                                       mut len: uint32_t) -> jv {
    let mut s: *mut jvp_string = jvp_string_ptr(string);
    let mut currlen: uint32_t = jvp_string_length(s);
    if jvp_refcnt_unshared(string.u.ptr) != 0 &&
           jvp_string_remaining_space(s) >= len {
        // the next string fits at the end of a
        memcpy((*s).data.as_mut_ptr().offset(currlen as isize) as
                   *mut libc::c_void, data as *const libc::c_void,
               len as libc::c_ulong);
        *(*s).data.as_mut_ptr().offset(currlen.wrapping_add(len) as isize) =
            0 as libc::c_int as libc::c_char;
        (*s).length_hashed = currlen.wrapping_add(len) << 1 as libc::c_int;
        return string
    } else {
        // allocate a bigger buffer and copy
        let mut allocsz: uint32_t =
            currlen.wrapping_add(len).wrapping_mul(2 as libc::c_int as
                                                       libc::c_uint);
        if allocsz < 32 as libc::c_int as libc::c_uint {
            allocsz = 32 as libc::c_int as uint32_t
        }
        let mut news: *mut jvp_string = jvp_string_alloc(allocsz);
        (*news).length_hashed = currlen.wrapping_add(len) << 1 as libc::c_int;
        memcpy((*news).data.as_mut_ptr() as *mut libc::c_void,
               (*s).data.as_mut_ptr() as *const libc::c_void,
               currlen as libc::c_ulong);
        memcpy((*news).data.as_mut_ptr().offset(currlen as isize) as
                   *mut libc::c_void, data as *const libc::c_void,
               len as libc::c_ulong);
        *(*news).data.as_mut_ptr().offset(currlen.wrapping_add(len) as isize)
            = 0 as libc::c_int as libc::c_char;
        jvp_string_free(string);
        let mut r: jv =
            {
                let mut init =
                    jv{kind_flags:
                           (JV_KIND_STRING as libc::c_int & 0xf as libc::c_int
                                |
                                JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                    0xf0 as libc::c_int) as libc::c_uchar,
                       pad_: 0 as libc::c_int as libc::c_uchar,
                       offset: 0 as libc::c_int as libc::c_ushort,
                       size: 0 as libc::c_int,
                       u: C2RustUnnamed{ptr: &mut (*news).refcnt,},};
                init
            };
        return r
    };
}
static mut HASH_SEED: uint32_t = 0x432a9843 as libc::c_int as uint32_t;
unsafe extern "C" fn rotl32(mut x: uint32_t, mut r: int8_t) -> uint32_t {
    return x << r as libc::c_int | x >> 32 as libc::c_int - r as libc::c_int;
}
unsafe extern "C" fn jvp_string_hash(mut jstr: jv) -> uint32_t {
    let mut str: *mut jvp_string = jvp_string_ptr(jstr);
    if (*str).length_hashed & 1 as libc::c_int as libc::c_uint != 0 {
        return (*str).hash
    }
    /* The following is based on MurmurHash3.
     MurmurHash3 was written by Austin Appleby, and is placed
     in the public domain. */
    let mut data: *const uint8_t =
        (*str).data.as_mut_ptr() as
            *const uint8_t; //FIXME: endianness/alignment
    let mut len: libc::c_int = jvp_string_length(str) as libc::c_int;
    let nblocks: libc::c_int = len / 4 as libc::c_int;
    let mut h1: uint32_t = HASH_SEED;
    let c1: uint32_t = 0xcc9e2d51 as libc::c_uint;
    let c2: uint32_t = 0x1b873593 as libc::c_int as uint32_t;
    let mut blocks: *const uint32_t =
        data.offset((nblocks * 4 as libc::c_int) as isize) as *const uint32_t;
    let mut i: libc::c_int = -nblocks;
    while i != 0 {
        let mut k1: uint32_t = *blocks.offset(i as isize);
        k1 = (k1 as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
        k1 = rotl32(k1, 15 as libc::c_int as int8_t);
        k1 = (k1 as libc::c_uint).wrapping_mul(c2) as uint32_t as uint32_t;
        h1 ^= k1;
        h1 = rotl32(h1, 13 as libc::c_int as int8_t);
        h1 =
            h1.wrapping_mul(5 as libc::c_int as
                                libc::c_uint).wrapping_add(0xe6546b64 as
                                                               libc::c_uint);
        i += 1
    }
    let mut tail: *const uint8_t =
        data.offset((nblocks * 4 as libc::c_int) as isize);
    let mut k1_0: uint32_t = 0 as libc::c_int as uint32_t;
    let mut current_block_16: u64;
    match len & 3 as libc::c_int {
        3 => {
            k1_0 ^=
                ((*tail.offset(2 as libc::c_int as isize) as libc::c_int) <<
                     16 as libc::c_int) as libc::c_uint;
            current_block_16 = 5176583331874022658;
        }
        2 => { current_block_16 = 5176583331874022658; }
        1 => { current_block_16 = 18109563915060528586; }
        _ => { current_block_16 = 11194104282611034094; }
    }
    match current_block_16 {
        5176583331874022658 => {
            k1_0 ^=
                ((*tail.offset(1 as libc::c_int as isize) as libc::c_int) <<
                     8 as libc::c_int) as libc::c_uint;
            current_block_16 = 18109563915060528586;
        }
        _ => { }
    }
    match current_block_16 {
        18109563915060528586 => {
            k1_0 ^= *tail.offset(0 as libc::c_int as isize) as libc::c_uint;
            k1_0 =
                (k1_0 as libc::c_uint).wrapping_mul(c1) as uint32_t as
                    uint32_t;
            k1_0 = rotl32(k1_0, 15 as libc::c_int as int8_t);
            k1_0 =
                (k1_0 as libc::c_uint).wrapping_mul(c2) as uint32_t as
                    uint32_t;
            h1 ^= k1_0
        }
        _ => { }
    }
    h1 ^= len as libc::c_uint;
    h1 ^= h1 >> 16 as libc::c_int;
    h1 =
        (h1 as libc::c_uint).wrapping_mul(0x85ebca6b as libc::c_uint) as
            uint32_t as uint32_t;
    h1 ^= h1 >> 13 as libc::c_int;
    h1 =
        (h1 as libc::c_uint).wrapping_mul(0xc2b2ae35 as libc::c_uint) as
            uint32_t as uint32_t;
    h1 ^= h1 >> 16 as libc::c_int;
    (*str).length_hashed |= 1 as libc::c_int as libc::c_uint;
    (*str).hash = h1;
    return h1;
}
unsafe extern "C" fn jvp_string_equal(mut a: jv, mut b: jv) -> libc::c_int {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      937 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"int jvp_string_equal(jv, jv)\x00")).as_ptr());
    };
    if b.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(b, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      938 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"int jvp_string_equal(jv, jv)\x00")).as_ptr());
    };
    let mut stra: *mut jvp_string = jvp_string_ptr(a);
    let mut strb: *mut jvp_string = jvp_string_ptr(b);
    if jvp_string_length(stra) != jvp_string_length(strb) {
        return 0 as libc::c_int
    }
    return (memcmp((*stra).data.as_mut_ptr() as *const libc::c_void,
                   (*strb).data.as_mut_ptr() as *const libc::c_void,
                   jvp_string_length(stra) as libc::c_ulong) ==
                0 as libc::c_int) as libc::c_int;
}
/*
 * Strings (public API)
 */
#[no_mangle]
pub unsafe extern "C" fn jv_string_sized(mut str: *const libc::c_char,
                                         mut len: libc::c_int) -> jv {
    return if jvp_utf8_is_valid(str, str.offset(len as isize)) != 0 {
               jvp_string_new(str, len as uint32_t)
           } else { jvp_string_copy_replace_bad(str, len as uint32_t) };
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_empty(mut len: libc::c_int) -> jv {
    return jvp_string_empty_new(len as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn jv_string(mut str: *const libc::c_char) -> jv {
    return jv_string_sized(str, strlen(str) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_length_bytes(mut j: jv) -> libc::c_int {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      965 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"int jv_string_length_bytes(jv)\x00")).as_ptr());
    };
    let mut r: libc::c_int =
        jvp_string_length(jvp_string_ptr(j)) as libc::c_int;
    jv_free(j);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_length_codepoints(mut j: jv)
 -> libc::c_int {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      972 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"int jv_string_length_codepoints(jv)\x00")).as_ptr());
    };
    let mut i: *const libc::c_char = jv_string_value(j);
    let mut end: *const libc::c_char =
        i.offset(jv_string_length_bytes(jv_copy(j)) as isize);
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    loop  {
        i = jvp_utf8_next(i, end, &mut c);
        if i.is_null() { break ; }
        len += 1
    }
    jv_free(j);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_indexes(mut j: jv, mut k: jv) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      983 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"jv jv_string_indexes(jv, jv)\x00")).as_ptr());
    };
    if k.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(k, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      984 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"jv jv_string_indexes(jv, jv)\x00")).as_ptr());
    };
    let mut jstr: *const libc::c_char = jv_string_value(j);
    let mut idxstr: *const libc::c_char = jv_string_value(k);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut jlen: libc::c_int = jv_string_length_bytes(jv_copy(j));
    let mut idxlen: libc::c_int = jv_string_length_bytes(jv_copy(k));
    let mut a: jv = jv_array();
    if idxlen != 0 as libc::c_int {
        p = jstr;
        loop  {
            p =
                _jq_memmem(p as *const libc::c_void,
                           jstr.offset(jlen as isize).wrapping_offset_from(p)
                               as libc::c_long as size_t,
                           idxstr as *const libc::c_void, idxlen as size_t) as
                    *const libc::c_char;
            if p.is_null() { break ; }
            a =
                jv_array_append(a,
                                jv_number(p.wrapping_offset_from(jstr) as
                                              libc::c_long as
                                              libc::c_double));
            p = p.offset(idxlen as isize)
        }
    }
    jv_free(j);
    jv_free(k);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_split(mut j: jv, mut sep: jv) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1005 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"jv jv_string_split(jv, jv)\x00")).as_ptr());
    };
    if sep.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(sep, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1006 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"jv jv_string_split(jv, jv)\x00")).as_ptr());
    };
    let mut jstr: *const libc::c_char = jv_string_value(j);
    let mut jend: *const libc::c_char =
        jstr.offset(jv_string_length_bytes(jv_copy(j)) as isize);
    let mut sepstr: *const libc::c_char = jv_string_value(sep);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut seplen: libc::c_int = jv_string_length_bytes(jv_copy(sep));
    let mut a: jv = jv_array();
    if jv_get_refcnt(a) == 1 as libc::c_int {
    } else {
        __assert_fail(b"jv_get_refcnt(a) == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1014 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"jv jv_string_split(jv, jv)\x00")).as_ptr());
    };
    if seplen == 0 as libc::c_int {
        let mut c: libc::c_int = 0;
        loop  {
            jstr = jvp_utf8_next(jstr, jend, &mut c);
            if jstr.is_null() { break ; }
            a =
                jv_array_append(a,
                                jv_string_append_codepoint(jv_string(b"\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                           c as uint32_t))
        }
    } else {
        p = jstr;
        while p < jend {
            s =
                _jq_memmem(p as *const libc::c_void,
                           jend.wrapping_offset_from(p) as libc::c_long as
                               size_t, sepstr as *const libc::c_void,
                           seplen as size_t) as *const libc::c_char;
            if s.is_null() { s = jend }
            a =
                jv_array_append(a,
                                jv_string_sized(p,
                                                s.wrapping_offset_from(p) as
                                                    libc::c_long as
                                                    libc::c_int));
            // Add an empty string to denote that j ends on a sep
            if s.offset(seplen as isize) == jend && seplen != 0 as libc::c_int
               {
                a =
                    jv_array_append(a,
                                    jv_string(b"\x00" as *const u8 as
                                                  *const libc::c_char))
            } // U+FFFD REPLACEMENT CHARACTER
            p = s.offset(seplen as isize)
        }
    }
    jv_free(j);
    jv_free(sep);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_explode(mut j: jv) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1037 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jv_string_explode(jv)\x00")).as_ptr());
    };
    let mut i: *const libc::c_char = jv_string_value(j);
    let mut len: libc::c_int = jv_string_length_bytes(jv_copy(j));
    let mut end: *const libc::c_char = i.offset(len as isize);
    let mut a: jv = jv_array_sized(len);
    let mut c: libc::c_int = 0;
    loop  {
        i = jvp_utf8_next(i, end, &mut c);
        if i.is_null() { break ; }
        a = jv_array_append(a, jv_number(c as libc::c_double))
    }
    jv_free(j);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_implode(mut j: jv) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_ARRAY as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_ARRAY)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1050 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jv_string_implode(jv)\x00")).as_ptr());
    };
    let mut len: libc::c_int = jv_array_length(jv_copy(j));
    let mut s: jv = jv_string_empty(len);
    let mut i: libc::c_int = 0;
    if len >= 0 as libc::c_int {
    } else {
        __assert_fail(b"len >= 0\x00" as *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1055 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jv_string_implode(jv)\x00")).as_ptr());
    };
    i = 0 as libc::c_int;
    while i < len {
        let mut n: jv = jv_array_get(jv_copy(j), i);
        if n.kind_flags as libc::c_int & 0xf as libc::c_int ==
               JV_KIND_NUMBER as libc::c_int {
        } else {
            __assert_fail(b"JVP_HAS_KIND(n, JV_KIND_NUMBER)\x00" as *const u8
                              as *const libc::c_char,
                          b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                          1059 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 25],
                                                    &[libc::c_char; 25]>(b"jv jv_string_implode(jv)\x00")).as_ptr());
        };
        let mut nv: libc::c_int = jv_number_value(n) as libc::c_int;
        jv_free(n);
        if nv > 0x10ffff as libc::c_int { nv = 0xfffd as libc::c_int }
        s = jv_string_append_codepoint(s, nv as uint32_t);
        i += 1
    }
    jv_free(j);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_hash(mut j: jv) -> libc::c_ulong {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1072 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"unsigned long jv_string_hash(jv)\x00")).as_ptr());
    };
    let mut hash: uint32_t = jvp_string_hash(j);
    jv_free(j);
    return hash as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_value(mut j: jv) -> *const libc::c_char {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1079 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"const char *jv_string_value(jv)\x00")).as_ptr());
    };
    return (*jvp_string_ptr(j)).data.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_slice(mut j: jv, mut start: libc::c_int,
                                         mut end: libc::c_int) -> jv {
    if j.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(j, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1084 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"jv jv_string_slice(jv, int, int)\x00")).as_ptr());
    };
    let mut s: *const libc::c_char = jv_string_value(j);
    let mut len: libc::c_int = jv_string_length_bytes(jv_copy(j));
    let mut i: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    let mut res: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    jvp_clamp_slice_params(len, &mut start, &mut end);
    if 0 as libc::c_int <= start && start <= end && end <= len {
    } else {
        __assert_fail(b"0 <= start && start <= end && end <= len\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1093 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"jv jv_string_slice(jv, int, int)\x00")).as_ptr());
    };
    /* Look for byte offset corresponding to start codepoints */
    p = s;
    i = 0 as libc::c_int;
    while i < start {
        p = jvp_utf8_next(p, s.offset(len as isize), &mut c);
        if p.is_null() {
            jv_free(j);
            return jv_string_empty(16 as libc::c_int)
        }
        if c == -(1 as libc::c_int) {
            jv_free(j);
            return jv_invalid_with_msg(jv_string(b"Invalid UTF-8 string\x00"
                                                     as *const u8 as
                                                     *const libc::c_char))
        }
        i += 1
    }
    /* Look for byte offset corresponding to end codepoints */
    e = p;
    while !e.is_null() && i < end {
        e = jvp_utf8_next(e, s.offset(len as isize), &mut c);
        if e.is_null() {
            e = s.offset(len as isize);
            break ;
        } else {
            if c == -(1 as libc::c_int) {
                jv_free(j);
                return jv_invalid_with_msg(jv_string(b"Invalid UTF-8 string\x00"
                                                         as *const u8 as
                                                         *const libc::c_char))
            }
            i += 1
        }
    }
    /*
   * NOTE: Ideally we should do here what jvp_array_slice() does instead
   * of allocating a new string as we do!  However, we assume NUL-
   * terminated strings all over, and in the jv API, so for now we waste
   * memory like a drunken navy programmer.  There's probably nothing we
   * can do about it.
   */
    res =
        jv_string_sized(p,
                        e.wrapping_offset_from(p) as libc::c_long as
                            libc::c_int);
    jv_free(j);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_concat(mut a: jv, mut b: jv) -> jv {
    a =
        jvp_string_append(a, jv_string_value(b),
                          jvp_string_length(jvp_string_ptr(b)));
    jv_free(b);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_append_buf(mut a: jv,
                                              mut buf: *const libc::c_char,
                                              mut len: libc::c_int) -> jv {
    if jvp_utf8_is_valid(buf, buf.offset(len as isize)) != 0 {
        a = jvp_string_append(a, buf, len as uint32_t)
    } else {
        let mut b: jv = jvp_string_copy_replace_bad(buf, len as uint32_t);
        a = jv_string_concat(a, b)
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_append_codepoint(mut a: jv,
                                                    mut c: uint32_t) -> jv {
    let mut buf: [libc::c_char; 5] = [0; 5];
    let mut len: libc::c_int =
        jvp_utf8_encode(c as libc::c_int, buf.as_mut_ptr());
    a = jvp_string_append(a, buf.as_mut_ptr(), len as uint32_t);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_append_str(mut a: jv,
                                              mut str: *const libc::c_char)
 -> jv {
    return jv_string_append_buf(a, str, strlen(str) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_vfmt(mut fmt: *const libc::c_char,
                                        mut ap: ::std::ffi::VaList) -> jv {
    let mut size: libc::c_int = 1024 as libc::c_int;
    loop  {
        let mut buf: *mut libc::c_char =
            jv_mem_alloc(size as size_t) as *mut libc::c_char;
        let mut ap2: ::std::ffi::VaListImpl;
        ap2 = ap.clone();
        let mut n: libc::c_int =
            vsnprintf(buf, size as libc::c_ulong, fmt, ap2.as_va_list());
        /*
     * NOTE: here we support old vsnprintf()s that return -1 because the
     * buffer is too small.
     */
        if n >= 0 as libc::c_int && n < size {
            let mut ret: jv = jv_string_sized(buf, n);
            jv_mem_free(buf as *mut libc::c_void);
            return ret
        } else {
            jv_mem_free(buf as *mut libc::c_void);
            size =
                if n > 0 as libc::c_int {
                    (n) * 2 as libc::c_int
                } else { (size) * 2 as libc::c_int }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_string_fmt(mut fmt: *const libc::c_char,
                                       mut args: ...) -> jv {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    let mut res: jv = jv_string_vfmt(fmt, args_0.as_va_list());
    return res;
}
/* warning: nontrivial justification of alignment */
unsafe extern "C" fn jvp_object_new(mut size: libc::c_int) -> jv {
    // Allocates an object of (size) slots and (size*2) hash buckets.
    // size must be a power of two
    if size > 0 as libc::c_int &&
           size & size - 1 as libc::c_int == 0 as libc::c_int {
    } else {
        __assert_fail(b"size > 0 && (size & (size - 1)) == 0\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1216 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"jv jvp_object_new(int)\x00")).as_ptr());
    };
    let mut obj: *mut jvp_object =
        jv_mem_alloc((::std::mem::size_of::<jvp_object>() as
                          libc::c_ulong).wrapping_add((::std::mem::size_of::<object_slot>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(size
                                                                                           as
                                                                                           libc::c_ulong)).wrapping_add((::std::mem::size_of::<libc::c_int>()
                                                                                                                             as
                                                                                                                             libc::c_ulong).wrapping_mul((size
                                                                                                                                                              *
                                                                                                                                                              2
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_int)
                                                                                                                                                             as
                                                                                                                                                             libc::c_ulong)))
            as *mut jvp_object;
    (*obj).refcnt.count = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        (*(*obj).elements.as_mut_ptr().offset(i as isize)).next =
            i - 1 as libc::c_int;
        (*(*obj).elements.as_mut_ptr().offset(i as isize)).string = JV_NULL;
        (*(*obj).elements.as_mut_ptr().offset(i as isize)).hash =
            0 as libc::c_int as uint32_t;
        (*(*obj).elements.as_mut_ptr().offset(i as isize)).value = JV_NULL;
        i += 1
    }
    (*obj).next_free = 0 as libc::c_int;
    let mut hashbuckets: *mut libc::c_int =
        &mut *(*obj).elements.as_mut_ptr().offset(size as isize) as
            *mut object_slot as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size * 2 as libc::c_int {
        *hashbuckets.offset(i_0 as isize) = -(1 as libc::c_int);
        i_0 += 1
    }
    let mut r: jv =
        {
            let mut init =
                jv{kind_flags:
                       (JV_KIND_OBJECT as libc::c_int & 0xf as libc::c_int |
                            JVP_PAYLOAD_ALLOCATED as libc::c_int &
                                0xf0 as libc::c_int) as libc::c_uchar,
                   pad_: 0 as libc::c_int as libc::c_uchar,
                   offset: 0 as libc::c_int as libc::c_ushort,
                   size: size,
                   u: C2RustUnnamed{ptr: &mut (*obj).refcnt,},};
            init
        };
    return r;
}
unsafe extern "C" fn jvp_object_ptr(mut o: jv) -> *mut jvp_object {
    if o.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(o, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1238 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"jvp_object *jvp_object_ptr(jv)\x00")).as_ptr());
    };
    return o.u.ptr as *mut jvp_object;
}
unsafe extern "C" fn jvp_object_mask(mut o: jv) -> uint32_t {
    if o.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(o, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1243 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"uint32_t jvp_object_mask(jv)\x00")).as_ptr());
    };
    return (o.size * 2 as libc::c_int - 1 as libc::c_int) as uint32_t;
}
unsafe extern "C" fn jvp_object_size(mut o: jv) -> libc::c_int {
    if o.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(o, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1248 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"int jvp_object_size(jv)\x00")).as_ptr());
    };
    return o.size;
}
unsafe extern "C" fn jvp_object_buckets(mut o: jv) -> *mut libc::c_int {
    return &mut *(*(jvp_object_ptr as
                        unsafe extern "C" fn(_: jv)
                            ->
                                *mut jvp_object)(o)).elements.as_mut_ptr().offset(o.size
                                                                                      as
                                                                                      isize)
               as *mut object_slot as *mut libc::c_int;
}
unsafe extern "C" fn jvp_object_find_bucket(mut object: jv, mut key: jv)
 -> *mut libc::c_int {
    return jvp_object_buckets(object).offset((jvp_object_mask(object) &
                                                  jvp_string_hash(key)) as
                                                 isize);
}
unsafe extern "C" fn jvp_object_get_slot(mut object: jv,
                                         mut slot: libc::c_int)
 -> *mut object_slot {
    if slot == -(1 as libc::c_int) ||
           slot >= 0 as libc::c_int && slot < jvp_object_size(object) {
    } else {
        __assert_fail(b"slot == -1 || (slot >= 0 && slot < jvp_object_size(object))\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1261 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"struct object_slot *jvp_object_get_slot(jv, int)\x00")).as_ptr());
    };
    if slot == -(1 as libc::c_int) {
        return 0 as *mut object_slot
    } else {
        return &mut *(*(jvp_object_ptr as
                            unsafe extern "C" fn(_: jv)
                                ->
                                    *mut jvp_object)(object)).elements.as_mut_ptr().offset(slot
                                                                                               as
                                                                                               isize)
                   as *mut object_slot
    };
}
unsafe extern "C" fn jvp_object_next_slot(mut object: jv,
                                          mut slot: *mut object_slot)
 -> *mut object_slot {
    return jvp_object_get_slot(object, (*slot).next);
}
unsafe extern "C" fn jvp_object_find_slot(mut object: jv, mut keystr: jv,
                                          mut bucket: *mut libc::c_int)
 -> *mut object_slot {
    let mut hash: uint32_t = jvp_string_hash(keystr);
    let mut curr: *mut object_slot = jvp_object_get_slot(object, *bucket);
    while !curr.is_null() {
        if (*curr).hash == hash &&
               jvp_string_equal(keystr, (*curr).string) != 0 {
            return curr
        }
        curr = jvp_object_next_slot(object, curr)
    }
    return 0 as *mut object_slot;
}
unsafe extern "C" fn jvp_object_add_slot(mut object: jv, mut key: jv,
                                         mut bucket: *mut libc::c_int)
 -> *mut object_slot {
    let mut o: *mut jvp_object = jvp_object_ptr(object);
    let mut newslot_idx: libc::c_int = (*o).next_free;
    if newslot_idx == jvp_object_size(object) { return 0 as *mut object_slot }
    let mut newslot: *mut object_slot =
        jvp_object_get_slot(object, newslot_idx);
    (*o).next_free += 1;
    (*newslot).next = *bucket;
    *bucket = newslot_idx;
    (*newslot).hash = jvp_string_hash(key);
    (*newslot).string = key;
    return newslot;
}
unsafe extern "C" fn jvp_object_read(mut object: jv, mut key: jv) -> *mut jv {
    if key.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(key, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1296 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"jv *jvp_object_read(jv, jv)\x00")).as_ptr());
    };
    let mut bucket: *mut libc::c_int = jvp_object_find_bucket(object, key);
    let mut slot: *mut object_slot =
        jvp_object_find_slot(object, key, bucket);
    if slot.is_null() {
        return 0 as *mut jv
    } else { return &mut (*slot).value };
}
unsafe extern "C" fn jvp_object_free(mut o: jv) {
    if o.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(o, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1304 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void jvp_object_free(jv)\x00")).as_ptr());
    };
    if jvp_refcnt_dec(o.u.ptr) != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < jvp_object_size(o) {
            let mut slot: *mut object_slot = jvp_object_get_slot(o, i);
            if jv_get_kind((*slot).string) as libc::c_uint !=
                   JV_KIND_NULL as libc::c_int as libc::c_uint {
                jvp_string_free((*slot).string);
                jv_free((*slot).value);
            }
            i += 1
        }
        jv_mem_free(jvp_object_ptr(o) as *mut libc::c_void);
    };
}
unsafe extern "C" fn jvp_object_rehash(mut object: jv) -> jv {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1318 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jvp_object_rehash(jv)\x00")).as_ptr());
    };
    if jvp_refcnt_unshared(object.u.ptr) != 0 {
    } else {
        __assert_fail(b"jvp_refcnt_unshared(object.u.ptr)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1319 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jvp_object_rehash(jv)\x00")).as_ptr());
    };
    let mut size: libc::c_int = jvp_object_size(object);
    let mut new_object: jv = jvp_object_new(size * 2 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let mut slot: *mut object_slot = jvp_object_get_slot(object, i);
        if !(jv_get_kind((*slot).string) as libc::c_uint ==
                 JV_KIND_NULL as libc::c_int as libc::c_uint) {
            let mut new_bucket: *mut libc::c_int =
                jvp_object_find_bucket(new_object, (*slot).string);
            if jvp_object_find_slot(new_object, (*slot).string,
                                    new_bucket).is_null() {
            } else {
                __assert_fail(b"!jvp_object_find_slot(new_object, slot->string, new_bucket)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"src/jv.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1326 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 25],
                                                        &[libc::c_char; 25]>(b"jv jvp_object_rehash(jv)\x00")).as_ptr());
            };
            let mut new_slot: *mut object_slot =
                jvp_object_add_slot(new_object, (*slot).string, new_bucket);
            if !new_slot.is_null() {
            } else {
                __assert_fail(b"new_slot\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/jv.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1328 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 25],
                                                        &[libc::c_char; 25]>(b"jv jvp_object_rehash(jv)\x00")).as_ptr());
            };
            (*new_slot).value = (*slot).value
        }
        i += 1
    }
    // references are transported, just drop the old table
    jv_mem_free(jvp_object_ptr(object) as *mut libc::c_void);
    return new_object;
}
unsafe extern "C" fn jvp_object_unshare(mut object: jv) -> jv {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1337 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"jv jvp_object_unshare(jv)\x00")).as_ptr());
    };
    if jvp_refcnt_unshared(object.u.ptr) != 0 { return object }
    let mut new_object: jv = jvp_object_new(jvp_object_size(object));
    (*jvp_object_ptr(new_object)).next_free =
        (*jvp_object_ptr(object)).next_free;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < jvp_object_size(new_object) {
        let mut old_slot: *mut object_slot = jvp_object_get_slot(object, i);
        let mut new_slot: *mut object_slot =
            jvp_object_get_slot(new_object, i);
        *new_slot = *old_slot;
        if jv_get_kind((*old_slot).string) as libc::c_uint !=
               JV_KIND_NULL as libc::c_int as libc::c_uint {
            (*new_slot).string = jv_copy((*old_slot).string);
            (*new_slot).value = jv_copy((*old_slot).value)
        }
        i += 1
    }
    let mut old_buckets: *mut libc::c_int = jvp_object_buckets(object);
    let mut new_buckets: *mut libc::c_int = jvp_object_buckets(new_object);
    memcpy(new_buckets as *mut libc::c_void,
           old_buckets as *const libc::c_void,
           (::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong).wrapping_mul(jvp_object_size(new_object) as
                                                libc::c_ulong).wrapping_mul(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong));
    jvp_object_free(object);
    if jvp_refcnt_unshared(new_object.u.ptr) != 0 {
    } else {
        __assert_fail(b"jvp_refcnt_unshared(new_object.u.ptr)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1358 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"jv jvp_object_unshare(jv)\x00")).as_ptr());
    };
    return new_object;
}
unsafe extern "C" fn jvp_object_write(mut object: *mut jv, mut key: jv)
 -> *mut jv {
    *object = jvp_object_unshare(*object);
    let mut bucket: *mut libc::c_int = jvp_object_find_bucket(*object, key);
    let mut slot: *mut object_slot =
        jvp_object_find_slot(*object, key, bucket);
    if !slot.is_null() {
        // already has the key
        jvp_string_free(key);
        return &mut (*slot).value
    }
    slot = jvp_object_add_slot(*object, key, bucket);
    if !slot.is_null() {
        (*slot).value = jv_invalid()
    } else {
        *object = jvp_object_rehash(*object);
        bucket = jvp_object_find_bucket(*object, key);
        if jvp_object_find_slot(*object, key, bucket).is_null() {
        } else {
            __assert_fail(b"!jvp_object_find_slot(*object, key, bucket)\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                          1377 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 31],
                                                    &[libc::c_char; 31]>(b"jv *jvp_object_write(jv *, jv)\x00")).as_ptr());
        };
        slot = jvp_object_add_slot(*object, key, bucket);
        if !slot.is_null() {
        } else {
            __assert_fail(b"slot\x00" as *const u8 as *const libc::c_char,
                          b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                          1379 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 31],
                                                    &[libc::c_char; 31]>(b"jv *jvp_object_write(jv *, jv)\x00")).as_ptr());
        };
        (*slot).value = jv_invalid()
    }
    return &mut (*slot).value;
}
unsafe extern "C" fn jvp_object_delete(mut object: *mut jv, mut key: jv)
 -> libc::c_int {
    if key.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(key, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1386 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"int jvp_object_delete(jv *, jv)\x00")).as_ptr());
    };
    *object = jvp_object_unshare(*object);
    let mut bucket: *mut libc::c_int = jvp_object_find_bucket(*object, key);
    let mut prev_ptr: *mut libc::c_int = bucket;
    let mut hash: uint32_t = jvp_string_hash(key);
    let mut curr: *mut object_slot = jvp_object_get_slot(*object, *bucket);
    while !curr.is_null() {
        if hash == (*curr).hash && jvp_string_equal(key, (*curr).string) != 0
           {
            *prev_ptr = (*curr).next;
            jvp_string_free((*curr).string);
            (*curr).string = JV_NULL;
            jv_free((*curr).value);
            return 1 as libc::c_int
        }
        prev_ptr = &mut (*curr).next;
        curr = jvp_object_next_slot(*object, curr)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn jvp_object_length(mut object: jv) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < jvp_object_size(object) {
        let mut slot: *mut object_slot = jvp_object_get_slot(object, i);
        if jv_get_kind((*slot).string) as libc::c_uint !=
               JV_KIND_NULL as libc::c_int as libc::c_uint {
            n += 1
        }
        i += 1
    }
    return n;
}
unsafe extern "C" fn jvp_object_equal(mut o1: jv, mut o2: jv) -> libc::c_int {
    let mut len2: libc::c_int = jvp_object_length(o2);
    let mut len1: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < jvp_object_size(o1) {
        let mut slot: *mut object_slot = jvp_object_get_slot(o1, i);
        if !(jv_get_kind((*slot).string) as libc::c_uint ==
                 JV_KIND_NULL as libc::c_int as libc::c_uint) {
            let mut slot2: *mut jv = jvp_object_read(o2, (*slot).string);
            if slot2.is_null() { return 0 as libc::c_int }
            // FIXME: do less refcounting here
            if jv_equal(jv_copy((*slot).value), jv_copy(*slot2)) == 0 {
                return 0 as libc::c_int
            }
            len1 += 1
        }
        i += 1
    }
    return (len1 == len2) as libc::c_int;
}
unsafe extern "C" fn jvp_object_contains(mut a: jv, mut b: jv)
 -> libc::c_int {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1431 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"int jvp_object_contains(jv, jv)\x00")).as_ptr());
    };
    if b.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(b, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1432 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"int jvp_object_contains(jv, jv)\x00")).as_ptr());
    };
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut jv_i__: libc::c_int = jv_object_iter(b);
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut key: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        let mut b_val: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if jv_object_iter_valid(b, jv_i__) != 0 {
                  key = jv_object_iter_key(b, jv_i__);
                  b_val = jv_object_iter_value(b, jv_i__);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            let mut a_val: jv = jv_object_get(jv_copy(a), jv_copy(key));
            r = jv_contains(a_val, b_val);
            jv_free(key);
            if r == 0 { break ; }
            jv_i__ = jv_object_iter_next(b, jv_i__)
        }
        jv_j__ = 0 as libc::c_int
    }
    return r;
}
/*
 * Objects (public interface)
 */
#[no_mangle]
pub unsafe extern "C" fn jv_object() -> jv {
    return jvp_object_new(8 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_get(mut object: jv, mut key: jv) -> jv {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1455 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jv_object_get(jv, jv)\x00")).as_ptr());
    };
    if key.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(key, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1456 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv jv_object_get(jv, jv)\x00")).as_ptr());
    };
    let mut slot: *mut jv = jvp_object_read(object, key);
    let mut val: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if !slot.is_null() { val = jv_copy(*slot) } else { val = jv_invalid() }
    jv_free(object);
    jv_free(key);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_has(mut object: jv, mut key: jv)
 -> libc::c_int {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1470 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"int jv_object_has(jv, jv)\x00")).as_ptr());
    };
    if key.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(key, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1471 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"int jv_object_has(jv, jv)\x00")).as_ptr());
    };
    let mut slot: *mut jv = jvp_object_read(object, key);
    let mut res: libc::c_int =
        if !slot.is_null() { 1 as libc::c_int } else { 0 as libc::c_int };
    jv_free(object);
    jv_free(key);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_set(mut object: jv, mut key: jv,
                                       mut value: jv) -> jv {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1480 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"jv jv_object_set(jv, jv, jv)\x00")).as_ptr());
    };
    if key.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(key, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1481 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"jv jv_object_set(jv, jv, jv)\x00")).as_ptr());
    };
    // copy/free of object, key, value coalesced
    let mut slot: *mut jv = jvp_object_write(&mut object, key);
    jv_free(*slot);
    *slot = value;
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_delete(mut object: jv, mut key: jv) -> jv {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1490 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"jv jv_object_delete(jv, jv)\x00")).as_ptr());
    };
    if key.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(key, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1491 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"jv jv_object_delete(jv, jv)\x00")).as_ptr());
    };
    jvp_object_delete(&mut object, key);
    jv_free(key);
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_length(mut object: jv) -> libc::c_int {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1498 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"int jv_object_length(jv)\x00")).as_ptr());
    };
    let mut n: libc::c_int = jvp_object_length(object);
    jv_free(object);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_merge(mut a: jv, mut b: jv) -> jv {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1505 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"jv jv_object_merge(jv, jv)\x00")).as_ptr());
    };
    let mut jv_i__: libc::c_int = jv_object_iter(b);
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut k: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        let mut v: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if jv_object_iter_valid(b, jv_i__) != 0 {
                  k = jv_object_iter_key(b, jv_i__);
                  v = jv_object_iter_value(b, jv_i__);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            a = jv_object_set(a, k, v);
            jv_i__ = jv_object_iter_next(b, jv_i__)
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(b);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_merge_recursive(mut a: jv, mut b: jv)
 -> jv {
    if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(a, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1514 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"jv jv_object_merge_recursive(jv, jv)\x00")).as_ptr());
    };
    if b.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(b, JV_KIND_OBJECT)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1515 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"jv jv_object_merge_recursive(jv, jv)\x00")).as_ptr());
    };
    let mut jv_i__: libc::c_int = jv_object_iter(b);
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut k: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        let mut v: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if jv_object_iter_valid(b, jv_i__) != 0 {
                  k = jv_object_iter_key(b, jv_i__);
                  v = jv_object_iter_value(b, jv_i__);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            let mut elem: jv = jv_object_get(jv_copy(a), jv_copy(k));
            if jv_is_valid(elem) != 0 &&
                   elem.kind_flags as libc::c_int & 0xf as libc::c_int ==
                       JV_KIND_OBJECT as libc::c_int &&
                   v.kind_flags as libc::c_int & 0xf as libc::c_int ==
                       JV_KIND_OBJECT as libc::c_int {
                a = jv_object_set(a, k, jv_object_merge_recursive(elem, v))
            } else { jv_free(elem); a = jv_object_set(a, k, v) }
            jv_i__ = jv_object_iter_next(b, jv_i__)
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(b);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_iter_valid(mut object: jv,
                                              mut i: libc::c_int)
 -> libc::c_int {
    return (i != ITER_FINISHED as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_iter(mut object: jv) -> libc::c_int {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1543 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"int jv_object_iter(jv)\x00")).as_ptr());
    };
    return jv_object_iter_next(object, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_iter_next(mut object: jv,
                                             mut iter: libc::c_int)
 -> libc::c_int {
    if object.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_OBJECT as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(object, JV_KIND_OBJECT)\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1548 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int jv_object_iter_next(jv, int)\x00")).as_ptr());
    };
    if iter != ITER_FINISHED as libc::c_int {
    } else {
        __assert_fail(b"iter != ITER_FINISHED\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1549 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int jv_object_iter_next(jv, int)\x00")).as_ptr());
    };
    let mut slot: *mut object_slot = 0 as *mut object_slot;
    loop  {
        iter += 1;
        if iter >= jvp_object_size(object) {
            return ITER_FINISHED as libc::c_int
        }
        slot = jvp_object_get_slot(object, iter);
        if !(jv_get_kind((*slot).string) as libc::c_uint ==
                 JV_KIND_NULL as libc::c_int as libc::c_uint) {
            break ;
        }
    }
    if jv_get_kind((*jvp_object_get_slot(object, iter)).string) as
           libc::c_uint == JV_KIND_STRING as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(jvp_object_get_slot(object,iter)->string) == JV_KIND_STRING\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1558 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int jv_object_iter_next(jv, int)\x00")).as_ptr());
    };
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_iter_key(mut object: jv,
                                            mut iter: libc::c_int) -> jv {
    let mut s: jv = (*jvp_object_get_slot(object, iter)).string;
    if s.kind_flags as libc::c_int & 0xf as libc::c_int ==
           JV_KIND_STRING as libc::c_int {
    } else {
        __assert_fail(b"JVP_HAS_KIND(s, JV_KIND_STRING)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv.c\x00" as *const u8 as *const libc::c_char,
                      1564 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"jv jv_object_iter_key(jv, int)\x00")).as_ptr());
    };
    return jv_copy(s);
}
#[no_mangle]
pub unsafe extern "C" fn jv_object_iter_value(mut object: jv,
                                              mut iter: libc::c_int) -> jv {
    return jv_copy((*jvp_object_get_slot(object, iter)).value);
}
/*
 * Memory management
 */
#[no_mangle]
pub unsafe extern "C" fn jv_copy(mut j: jv) -> jv {
    if j.kind_flags as libc::c_int & JVP_PAYLOAD_ALLOCATED as libc::c_int != 0
       {
        jvp_refcnt_inc(j.u.ptr);
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn jv_free(mut j: jv) {
    match j.kind_flags as libc::c_int & 0xf as libc::c_int {
        6 => { jvp_array_free(j); }
        5 => { jvp_string_free(j); }
        7 => { jvp_object_free(j); }
        0 => { jvp_invalid_free(j); }
        4 => { jvp_number_free(j); }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_get_refcnt(mut j: jv) -> libc::c_int {
    if j.kind_flags as libc::c_int & JVP_PAYLOAD_ALLOCATED as libc::c_int != 0
       {
        return (*j.u.ptr).count
    } else { return 1 as libc::c_int };
}
/*
 * Higher-level operations
 */
#[no_mangle]
pub unsafe extern "C" fn jv_equal(mut a: jv, mut b: jv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if jv_get_kind(a) as libc::c_uint != jv_get_kind(b) as libc::c_uint {
        r = 0 as libc::c_int
    } else if a.kind_flags as libc::c_int &
                  JVP_PAYLOAD_ALLOCATED as libc::c_int != 0 &&
                  b.kind_flags as libc::c_int &
                      JVP_PAYLOAD_ALLOCATED as libc::c_int != 0 &&
                  a.kind_flags as libc::c_int == b.kind_flags as libc::c_int
                  && a.size == b.size && a.u.ptr == b.u.ptr {
        r = 1 as libc::c_int
    } else {
        match jv_get_kind(a) as libc::c_uint {
            4 => { r = jvp_number_equal(a, b) }
            6 => { r = jvp_array_equal(a, b) }
            5 => { r = jvp_string_equal(a, b) }
            7 => { r = jvp_object_equal(a, b) }
            _ => { r = 1 as libc::c_int }
        }
    }
    jv_free(a);
    jv_free(b);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn jv_identical(mut a: jv, mut b: jv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if a.kind_flags as libc::c_int != b.kind_flags as libc::c_int ||
           a.offset as libc::c_int != b.offset as libc::c_int ||
           a.size != b.size {
        r = 0 as libc::c_int
    } else if a.kind_flags as libc::c_int &
                  JVP_PAYLOAD_ALLOCATED as libc::c_int != 0 {
        /* b has the same flags */
        r = (a.u.ptr == b.u.ptr) as libc::c_int
    } else {
        r =
            (memcmp(&mut a.u.ptr as *mut *mut jv_refcnt as
                        *const libc::c_void,
                    &mut b.u.ptr as *mut *mut jv_refcnt as
                        *const libc::c_void,
                    ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
                 == 0 as libc::c_int) as libc::c_int
    }
    jv_free(a);
    jv_free(b);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn jv_contains(mut a: jv, mut b: jv) -> libc::c_int {
    let mut r: libc::c_int = 1 as libc::c_int;
    if jv_get_kind(a) as libc::c_uint != jv_get_kind(b) as libc::c_uint {
        r = 0 as libc::c_int
    } else if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
                  JV_KIND_OBJECT as libc::c_int {
        r = jvp_object_contains(a, b)
    } else if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
                  JV_KIND_ARRAY as libc::c_int {
        r = jvp_array_contains(a, b)
    } else if a.kind_flags as libc::c_int & 0xf as libc::c_int ==
                  JV_KIND_STRING as libc::c_int {
        let mut b_len: libc::c_int = jv_string_length_bytes(jv_copy(b));
        if b_len != 0 as libc::c_int {
            r =
                (_jq_memmem(jv_string_value(a) as *const libc::c_void,
                            jv_string_length_bytes(jv_copy(a)) as size_t,
                            jv_string_value(b) as *const libc::c_void,
                            b_len as size_t) != 0 as *const libc::c_void) as
                    libc::c_int
        } else { r = 1 as libc::c_int }
    } else { r = jv_equal(jv_copy(a), jv_copy(b)) }
    jv_free(a);
    jv_free(b);
    return r;
}
