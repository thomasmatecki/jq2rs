#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type Bigint;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
    fn jv_invalid() -> jv;
    #[no_mangle]
    fn jv_invalid_with_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_has_msg(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_true() -> jv;
    #[no_mangle]
    fn jv_false() -> jv;
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn jv_number_with_literal(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_set(_: jv, _: libc::c_int, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_slice(_: jv, _: libc::c_int, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_sized(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_vfmt(_: *const libc::c_char, _: ::std::ffi::VaList) -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value_0: jv) -> jv;
    #[no_mangle]
    fn jv_object_length(object: jv) -> libc::c_int;
    #[no_mangle]
    fn jvp_dtoa_context_init(ctx: *mut dtoa_context);
    #[no_mangle]
    fn jvp_dtoa_context_free(ctx: *mut dtoa_context);
    #[no_mangle]
    fn jvp_utf8_encode(codepoint: libc::c_int, out: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
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
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const JV_PARSE_STREAM_ERRORS: C2RustUnnamed_0 = 4;
pub const JV_PARSE_STREAMING: C2RustUnnamed_0 = 2;
pub const JV_PARSE_SEQ: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct jv_parser {
    pub curr_buf: *const libc::c_char,
    pub curr_buf_length: libc::c_int,
    pub curr_buf_pos: libc::c_int,
    pub curr_buf_is_partial: libc::c_int,
    pub eof: libc::c_int,
    pub bom_strip_position: libc::c_uint,
    pub flags: libc::c_int,
    pub stack: *mut jv,
    pub stackpos: libc::c_int,
    pub stacklen: libc::c_int,
    pub path: jv,
    pub last_seen: last_seen,
    pub output: jv,
    pub next: jv,
    pub tokenbuf: *mut libc::c_char,
    pub tokenpos: libc::c_int,
    pub tokenlen: libc::c_int,
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub dtoa: dtoa_context,
    pub st: C2RustUnnamed_1,
    #[bitfield(name = "last_ch_was_ws", ty = "libc::c_uint", bits = "0..=0")]
    pub last_ch_was_ws: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const JV_PARSER_WAITING_FOR_RS: C2RustUnnamed_1 = 3;
pub const JV_PARSER_STRING_ESCAPE: C2RustUnnamed_1 = 2;
pub const JV_PARSER_STRING: C2RustUnnamed_1 = 1;
pub const JV_PARSER_NORMAL: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtoa_context {
    pub freelist: [*mut Bigint; 8],
    pub p5s: *mut Bigint,
}
pub type last_seen = libc::c_uint;
pub const JV_LAST_VALUE: last_seen = 86;
pub const JV_LAST_COMMA: last_seen = 44;
pub const JV_LAST_COLON: last_seen = 58;
pub const JV_LAST_OPEN_OBJECT: last_seen = 123;
pub const JV_LAST_OPEN_ARRAY: last_seen = 91;
pub const JV_LAST_NONE: last_seen = 0;
pub type presult = *const libc::c_char;
pub const INVALID: chclass = 4;
pub const STRUCTURE: chclass = 2;
pub const QUOTE: chclass = 3;
pub const WHITESPACE: chclass = 1;
pub const LITERAL: chclass = 0;
pub type chclass = libc::c_uint;
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn parser_init(mut p: *mut jv_parser,
                                 mut flags: libc::c_int) {
    (*p).flags = flags;
    if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
        (*p).path = jv_array()
    } else {
        (*p).path = jv_invalid();
        (*p).flags &= !(JV_PARSE_STREAM_ERRORS as libc::c_int)
    }
    (*p).stack = 0 as *mut jv;
    (*p).stackpos = 0 as libc::c_int;
    (*p).stacklen = (*p).stackpos;
    (*p).last_seen = JV_LAST_NONE;
    (*p).output = jv_invalid();
    (*p).next = jv_invalid();
    (*p).tokenbuf = 0 as *mut libc::c_char;
    (*p).tokenpos = 0 as libc::c_int;
    (*p).tokenlen = (*p).tokenpos;
    if (*p).flags & JV_PARSE_SEQ as libc::c_int != 0 {
        (*p).st = JV_PARSER_WAITING_FOR_RS
    } else { (*p).st = JV_PARSER_NORMAL }
    (*p).eof = 0 as libc::c_int;
    (*p).curr_buf = 0 as *const libc::c_char;
    (*p).curr_buf_is_partial = 0 as libc::c_int;
    (*p).curr_buf_pos = (*p).curr_buf_is_partial;
    (*p).curr_buf_length = (*p).curr_buf_pos;
    (*p).bom_strip_position = 0 as libc::c_int as libc::c_uint;
    (*p).set_last_ch_was_ws(0 as libc::c_int as libc::c_uint);
    (*p).line = 1 as libc::c_int;
    (*p).column = 0 as libc::c_int;
    jvp_dtoa_context_init(&mut (*p).dtoa);
}
unsafe extern "C" fn parser_reset(mut p: *mut jv_parser) {
    if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
        jv_free((*p).path);
        (*p).path = jv_array();
        (*p).stacklen = 0 as libc::c_int
    }
    (*p).last_seen = JV_LAST_NONE;
    jv_free((*p).output);
    (*p).output = jv_invalid();
    jv_free((*p).next);
    (*p).next = jv_invalid();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*p).stackpos {
        jv_free(*(*p).stack.offset(i as isize));
        i += 1
    }
    (*p).stackpos = 0 as libc::c_int;
    (*p).tokenpos = 0 as libc::c_int;
    (*p).st = JV_PARSER_NORMAL;
}
unsafe extern "C" fn parser_free(mut p: *mut jv_parser) {
    parser_reset(p);
    jv_free((*p).path);
    jv_free((*p).output);
    jv_mem_free((*p).stack as *mut libc::c_void);
    jv_mem_free((*p).tokenbuf as *mut libc::c_void);
    jvp_dtoa_context_free(&mut (*p).dtoa);
}
unsafe extern "C" fn value(mut p: *mut jv_parser, mut val: jv) -> presult {
    if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
        if jv_is_valid((*p).next) != 0 ||
               (*p).last_seen as libc::c_uint ==
                   JV_LAST_VALUE as libc::c_int as libc::c_uint {
            jv_free(val);
            return b"Expected separator between values\x00" as *const u8 as
                       *const libc::c_char
        }
        if (*p).stacklen > 0 as libc::c_int {
            (*p).last_seen = JV_LAST_VALUE
        } else { (*p).last_seen = JV_LAST_NONE }
    } else if jv_is_valid((*p).next) != 0 {
        jv_free(val);
        return b"Expected separator between values\x00" as *const u8 as
                   *const libc::c_char
    }
    jv_free((*p).next);
    (*p).next = val;
    return 0 as presult;
}
unsafe extern "C" fn push(mut p: *mut jv_parser, mut v: jv) {
    if (*p).stackpos <= (*p).stacklen {
    } else {
        __assert_fail(b"p->stackpos <= p->stacklen\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_parse.c\x00" as *const u8 as
                          *const libc::c_char,
                      147 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void push(struct jv_parser *, jv)\x00")).as_ptr());
    };
    if (*p).stackpos == (*p).stacklen {
        (*p).stacklen = (*p).stacklen * 2 as libc::c_int + 10 as libc::c_int;
        (*p).stack =
            jv_mem_realloc((*p).stack as *mut libc::c_void,
                           ((*p).stacklen as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<jv>()
                                                                as
                                                                libc::c_ulong))
                as *mut jv
    }
    if (*p).stackpos < (*p).stacklen {
    } else {
        __assert_fail(b"p->stackpos < p->stacklen\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_parse.c\x00" as *const u8 as
                          *const libc::c_char,
                      152 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void push(struct jv_parser *, jv)\x00")).as_ptr());
    };
    let fresh0 = (*p).stackpos;
    (*p).stackpos = (*p).stackpos + 1;
    *(*p).stack.offset(fresh0 as isize) = v;
}
unsafe extern "C" fn parse_token(mut p: *mut jv_parser, mut ch: libc::c_char)
 -> presult {
    match ch as libc::c_int {
        91 => {
            if (*p).stackpos >= 256 as libc::c_int {
                return b"Exceeds depth limit for parsing\x00" as *const u8 as
                           *const libc::c_char
            }
            if jv_is_valid((*p).next) != 0 {
                return b"Expected separator between values\x00" as *const u8
                           as *const libc::c_char
            }
            push(p, jv_array());
        }
        123 => {
            if (*p).stackpos >= 256 as libc::c_int {
                return b"Exceeds depth limit for parsing\x00" as *const u8 as
                           *const libc::c_char
            }
            if jv_is_valid((*p).next) != 0 {
                return b"Expected separator between values\x00" as *const u8
                           as *const libc::c_char
            }
            push(p, jv_object());
        }
        58 => {
            if jv_is_valid((*p).next) == 0 {
                return b"Expected string key before \':\'\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).stackpos == 0 as libc::c_int ||
                   jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                       1 as libc::c_int) as
                                                      isize)) as libc::c_uint
                       != JV_KIND_OBJECT as libc::c_int as libc::c_uint {
                return b"\':\' not as part of an object\x00" as *const u8 as
                           *const libc::c_char
            }
            if jv_get_kind((*p).next) as libc::c_uint !=
                   JV_KIND_STRING as libc::c_int as libc::c_uint {
                return b"Object keys must be strings\x00" as *const u8 as
                           *const libc::c_char
            }
            push(p, (*p).next);
            (*p).next = jv_invalid()
        }
        44 => {
            if jv_is_valid((*p).next) == 0 {
                return b"Expected value before \',\'\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).stackpos == 0 as libc::c_int {
                return b"\',\' not as part of an object or array\x00" as
                           *const u8 as *const libc::c_char
            }
            if jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                   1 as libc::c_int) as
                                                  isize)) as libc::c_uint ==
                   JV_KIND_ARRAY as libc::c_int as libc::c_uint {
                *(*p).stack.offset(((*p).stackpos - 1 as libc::c_int) as
                                       isize) =
                    jv_array_append(*(*p).stack.offset(((*p).stackpos -
                                                            1 as libc::c_int)
                                                           as isize),
                                    (*p).next);
                (*p).next = jv_invalid()
            } else if jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                          1 as libc::c_int) as
                                                         isize)) as
                          libc::c_uint ==
                          JV_KIND_STRING as libc::c_int as libc::c_uint {
                if (*p).stackpos > 1 as libc::c_int &&
                       jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                           2 as libc::c_int)
                                                          as isize)) as
                           libc::c_uint ==
                           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"p->stackpos > 1 && jv_get_kind(p->stack[p->stackpos-2]) == JV_KIND_OBJECT\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"src/jv_parse.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  190 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 46],
                                                            &[libc::c_char; 46]>(b"presult parse_token(struct jv_parser *, char)\x00")).as_ptr());
                };
                *(*p).stack.offset(((*p).stackpos - 2 as libc::c_int) as
                                       isize) =
                    jv_object_set(*(*p).stack.offset(((*p).stackpos -
                                                          2 as libc::c_int) as
                                                         isize),
                                  *(*p).stack.offset(((*p).stackpos -
                                                          1 as libc::c_int) as
                                                         isize), (*p).next);
                (*p).stackpos -= 1;
                (*p).next = jv_invalid()
            } else {
                // this case hits on input like {"a", "b"}
                return b"Objects must consist of key:value pairs\x00" as
                           *const u8 as *const libc::c_char
            }
        }
        93 => {
            if (*p).stackpos == 0 as libc::c_int ||
                   jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                       1 as libc::c_int) as
                                                      isize)) as libc::c_uint
                       != JV_KIND_ARRAY as libc::c_int as libc::c_uint {
                return b"Unmatched \']\'\x00" as *const u8 as
                           *const libc::c_char
            }
            if jv_is_valid((*p).next) != 0 {
                *(*p).stack.offset(((*p).stackpos - 1 as libc::c_int) as
                                       isize) =
                    jv_array_append(*(*p).stack.offset(((*p).stackpos -
                                                            1 as libc::c_int)
                                                           as isize),
                                    (*p).next);
                (*p).next = jv_invalid()
            } else if jv_array_length(jv_copy(*(*p).stack.offset(((*p).stackpos
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     isize)))
                          != 0 as libc::c_int {
                // this case hits on input like [1,2,3,]
                return b"Expected another array element\x00" as *const u8 as
                           *const libc::c_char
            } // push
            jv_free((*p).next);
            (*p).stackpos -= 1;
            (*p).next = *(*p).stack.offset((*p).stackpos as isize)
        }
        125 => {
            if (*p).stackpos == 0 as libc::c_int {
                return b"Unmatched \'}\'\x00" as *const u8 as
                           *const libc::c_char
            }
            if jv_is_valid((*p).next) != 0 {
                if jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                       1 as libc::c_int) as
                                                      isize)) as libc::c_uint
                       != JV_KIND_STRING as libc::c_int as libc::c_uint {
                    return b"Objects must consist of key:value pairs\x00" as
                               *const u8 as *const libc::c_char
                }
                if (*p).stackpos > 1 as libc::c_int &&
                       jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                           2 as libc::c_int)
                                                          as isize)) as
                           libc::c_uint ==
                           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"p->stackpos > 1 && jv_get_kind(p->stack[p->stackpos-2]) == JV_KIND_OBJECT\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"src/jv_parse.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  223 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 46],
                                                            &[libc::c_char; 46]>(b"presult parse_token(struct jv_parser *, char)\x00")).as_ptr());
                };
                *(*p).stack.offset(((*p).stackpos - 2 as libc::c_int) as
                                       isize) =
                    jv_object_set(*(*p).stack.offset(((*p).stackpos -
                                                          2 as libc::c_int) as
                                                         isize),
                                  *(*p).stack.offset(((*p).stackpos -
                                                          1 as libc::c_int) as
                                                         isize), (*p).next);
                (*p).stackpos -= 1;
                (*p).next = jv_invalid()
            } else {
                if jv_get_kind(*(*p).stack.offset(((*p).stackpos -
                                                       1 as libc::c_int) as
                                                      isize)) as libc::c_uint
                       != JV_KIND_OBJECT as libc::c_int as libc::c_uint {
                    return b"Unmatched \'}\'\x00" as *const u8 as
                               *const libc::c_char
                }
                if jv_object_length(jv_copy(*(*p).stack.offset(((*p).stackpos
                                                                    -
                                                                    1 as
                                                                        libc::c_int)
                                                                   as isize)))
                       != 0 as libc::c_int {
                    return b"Expected another key-value pair\x00" as *const u8
                               as *const libc::c_char
                }
            }
            jv_free((*p).next);
            (*p).stackpos -= 1;
            (*p).next = *(*p).stack.offset((*p).stackpos as isize)
        }
        _ => { }
    }
    return 0 as presult;
}
unsafe extern "C" fn stream_token(mut p: *mut jv_parser, mut ch: libc::c_char)
 -> presult {
    let mut k: jv_kind = JV_KIND_INVALID;
    let mut last: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    match ch as libc::c_int {
        91 => {
            if jv_is_valid((*p).next) != 0 {
                return b"Expected a separator between values\x00" as *const u8
                           as *const libc::c_char
            }
            (*p).path =
                jv_array_append((*p).path,
                                jv_number(0 as libc::c_int as
                                              libc::c_double));
            (*p).last_seen = JV_LAST_OPEN_ARRAY;
            (*p).stacklen += 1
        }
        123 => {
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_VALUE as libc::c_int as libc::c_uint {
                return b"Expected a separator between values\x00" as *const u8
                           as *const libc::c_char
            }
            // Push object key: null, since we don't know it yet
            (*p).path = jv_array_append((*p).path, jv_null()); // push
            (*p).last_seen =
                JV_LAST_OPEN_OBJECT; // ready for another name:value pair
            (*p).stacklen += 1
        }
        58 => {
            last = jv_invalid();
            if (*p).stacklen == 0 as libc::c_int ||
                   {
                       last =
                           jv_array_get(jv_copy((*p).path),
                                        (*p).stacklen - 1 as libc::c_int);
                       (jv_get_kind(last) as libc::c_uint) ==
                           JV_KIND_NUMBER as libc::c_int as libc::c_uint
                   } {
                jv_free(last);
                return b"\':\' not as part of an object\x00" as *const u8 as
                           *const libc::c_char
            }
            jv_free(last);
            if jv_is_valid((*p).next) == 0 ||
                   (*p).last_seen as libc::c_uint ==
                       JV_LAST_NONE as libc::c_int as libc::c_uint {
                return b"Expected string key before \':\'\x00" as *const u8 as
                           *const libc::c_char
            }
            if jv_get_kind((*p).next) as libc::c_uint !=
                   JV_KIND_STRING as libc::c_int as libc::c_uint {
                return b"Object keys must be strings\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).last_seen as libc::c_uint !=
                   JV_LAST_VALUE as libc::c_int as libc::c_uint {
                return b"\':\' should follow a key\x00" as *const u8 as
                           *const libc::c_char
            }
            (*p).last_seen = JV_LAST_COLON;
            (*p).path =
                jv_array_set((*p).path, (*p).stacklen - 1 as libc::c_int,
                             (*p).next);
            (*p).next = jv_invalid()
        }
        44 => {
            if (*p).last_seen as libc::c_uint !=
                   JV_LAST_VALUE as libc::c_int as libc::c_uint {
                return b"Expected value before \',\'\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).stacklen == 0 as libc::c_int {
                return b"\',\' not as part of an object or array\x00" as
                           *const u8 as *const libc::c_char
            }
            last =
                jv_array_get(jv_copy((*p).path),
                             (*p).stacklen - 1 as libc::c_int);
            k = jv_get_kind(last);
            if k as libc::c_uint ==
                   JV_KIND_NUMBER as libc::c_int as libc::c_uint {
                let mut idx: libc::c_int =
                    jv_number_value(last) as libc::c_int;
                if jv_is_valid((*p).next) != 0 {
                    (*p).output =
                        jv_array_append(jv_array_append(jv_array(),
                                                        jv_copy((*p).path)),
                                        (*p).next);
                    (*p).next = jv_invalid()
                }
                (*p).path =
                    jv_array_set((*p).path, (*p).stacklen - 1 as libc::c_int,
                                 jv_number((idx + 1 as libc::c_int) as
                                               libc::c_double));
                (*p).last_seen = JV_LAST_COMMA
            } else if k as libc::c_uint ==
                          JV_KIND_STRING as libc::c_int as libc::c_uint {
                if jv_is_valid((*p).next) != 0 {
                    (*p).output =
                        jv_array_append(jv_array_append(jv_array(),
                                                        jv_copy((*p).path)),
                                        (*p).next);
                    (*p).next = jv_invalid()
                }
                (*p).path =
                    jv_array_set((*p).path, (*p).stacklen - 1 as libc::c_int,
                                 jv_true());
                (*p).last_seen = JV_LAST_COMMA
            } else {
                if k as libc::c_uint ==
                       JV_KIND_NULL as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"k == JV_KIND_NULL\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/jv_parse.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  305 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"presult stream_token(struct jv_parser *, char)\x00")).as_ptr());
                };
                // this case hits on input like {,}
      // make sure to handle input like {"a", "b"} and {"a":, ...}
                jv_free(last); // pop
                return b"Objects must consist of key:value pairs\x00" as
                           *const u8 as *const libc::c_char
            }
            jv_free(last);
        }
        93 => {
            if (*p).stacklen == 0 as libc::c_int {
                return b"Unmatched \']\' at the top-level\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_COMMA as libc::c_int as libc::c_uint {
                return b"Expected another array element\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_OPEN_ARRAY as libc::c_int as libc::c_uint {
                if jv_is_valid((*p).next) == 0 {
                } else {
                    __assert_fail(b"!jv_is_valid(p->next)\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/jv_parse.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  320 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"presult stream_token(struct jv_parser *, char)\x00")).as_ptr());
                };
            }
            last =
                jv_array_get(jv_copy((*p).path),
                             (*p).stacklen - 1 as libc::c_int);
            k = jv_get_kind(last);
            jv_free(last);
            if k as libc::c_uint !=
                   JV_KIND_NUMBER as libc::c_int as libc::c_uint {
                return b"Unmatched \']\' in the middle of an object\x00" as
                           *const u8 as *const libc::c_char
            }
            if jv_is_valid((*p).next) != 0 {
                (*p).output =
                    jv_array_append(jv_array_append(jv_array_append(jv_array(),
                                                                    jv_copy((*p).path)),
                                                    (*p).next), jv_true());
                (*p).next = jv_invalid()
            } else if (*p).last_seen as libc::c_uint !=
                          JV_LAST_OPEN_ARRAY as libc::c_int as libc::c_uint {
                (*p).output = jv_array_append(jv_array(), jv_copy((*p).path))
            }
            (*p).stacklen -= 1;
            (*p).path =
                jv_array_slice((*p).path, 0 as libc::c_int, (*p).stacklen);
            //assert(!jv_is_valid(p->next));
            jv_free((*p).next); // Empty arrays are leaves
            (*p).next = jv_invalid();
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_OPEN_ARRAY as libc::c_int as libc::c_uint {
                (*p).output =
                    jv_array_append(jv_array_append(jv_array(),
                                                    jv_copy((*p).path)),
                                    jv_array())
            }
            if (*p).stacklen == 0 as libc::c_int {
                (*p).last_seen = JV_LAST_NONE
            } else { (*p).last_seen = JV_LAST_VALUE }
        }
        125 => {
            if (*p).stacklen == 0 as libc::c_int {
                return b"Unmatched \'}\' at the top-level\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_COMMA as libc::c_int as libc::c_uint {
                return b"Expected another key:value pair\x00" as *const u8 as
                           *const libc::c_char
            }
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_OPEN_OBJECT as libc::c_int as libc::c_uint {
                if jv_is_valid((*p).next) == 0 {
                } else {
                    __assert_fail(b"!jv_is_valid(p->next)\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/jv_parse.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  355 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"presult stream_token(struct jv_parser *, char)\x00")).as_ptr());
                };
            }
            last =
                jv_array_get(jv_copy((*p).path),
                             (*p).stacklen - 1 as libc::c_int);
            k = jv_get_kind(last);
            jv_free(last);
            if k as libc::c_uint ==
                   JV_KIND_NUMBER as libc::c_int as libc::c_uint {
                return b"Unmatched \'}\' in the middle of an array\x00" as
                           *const u8 as *const libc::c_char
            }
            if jv_is_valid((*p).next) != 0 {
                if k as libc::c_uint !=
                       JV_KIND_STRING as libc::c_int as libc::c_uint {
                    return b"Objects must consist of key:value pairs\x00" as
                               *const u8 as *const libc::c_char
                }
                (*p).output =
                    jv_array_append(jv_array_append(jv_array_append(jv_array(),
                                                                    jv_copy((*p).path)),
                                                    (*p).next), jv_true());
                (*p).next = jv_invalid()
            } else {
                // Perhaps {"a":[]}
                if (*p).last_seen as libc::c_uint ==
                       JV_LAST_COLON as libc::c_int as libc::c_uint {
                    // Looks like {"a":}
                    return b"Missing value in key:value pair\x00" as *const u8
                               as *const libc::c_char
                }
                if (*p).last_seen as libc::c_uint ==
                       JV_LAST_COMMA as libc::c_int as libc::c_uint {
                    // Looks like {"a":0,}
                    return b"Expected another key-value pair\x00" as *const u8
                               as *const libc::c_char
                } // pop
                if (*p).last_seen as libc::c_uint ==
                       JV_LAST_OPEN_ARRAY as libc::c_int as libc::c_uint {
                    return b"Unmatched \'}\' in the middle of an array\x00" as
                               *const u8 as *const libc::c_char
                } // Empty arrays are leaves
                if (*p).last_seen as libc::c_uint !=
                       JV_LAST_VALUE as libc::c_int as libc::c_uint &&
                       (*p).last_seen as libc::c_uint !=
                           JV_LAST_OPEN_OBJECT as libc::c_int as libc::c_uint
                   {
                    return b"Unmatched \'}\'\x00" as *const u8 as
                               *const libc::c_char
                }
                if (*p).last_seen as libc::c_uint !=
                       JV_LAST_OPEN_OBJECT as libc::c_int as libc::c_uint {
                    (*p).output =
                        jv_array_append(jv_array(), jv_copy((*p).path))
                }
            }
            (*p).stacklen -= 1;
            (*p).path =
                jv_array_slice((*p).path, 0 as libc::c_int, (*p).stacklen);
            jv_free((*p).next);
            (*p).next = jv_invalid();
            if (*p).last_seen as libc::c_uint ==
                   JV_LAST_OPEN_OBJECT as libc::c_int as libc::c_uint {
                (*p).output =
                    jv_array_append(jv_array_append(jv_array(),
                                                    jv_copy((*p).path)),
                                    jv_object())
            }
            if (*p).stacklen == 0 as libc::c_int {
                (*p).last_seen = JV_LAST_NONE
            } else { (*p).last_seen = JV_LAST_VALUE }
        }
        _ => { }
    }
    return 0 as presult;
}
unsafe extern "C" fn tokenadd(mut p: *mut jv_parser, mut c: libc::c_char) {
    if (*p).tokenpos <= (*p).tokenlen {
    } else {
        __assert_fail(b"p->tokenpos <= p->tokenlen\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_parse.c\x00" as *const u8 as
                          *const libc::c_char,
                      400 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void tokenadd(struct jv_parser *, char)\x00")).as_ptr());
    };
    if (*p).tokenpos >= (*p).tokenlen - 1 as libc::c_int {
        (*p).tokenlen = (*p).tokenlen * 2 as libc::c_int + 256 as libc::c_int;
        (*p).tokenbuf =
            jv_mem_realloc((*p).tokenbuf as *mut libc::c_void,
                           (*p).tokenlen as size_t) as *mut libc::c_char
    }
    if (*p).tokenpos < (*p).tokenlen {
    } else {
        __assert_fail(b"p->tokenpos < p->tokenlen\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_parse.c\x00" as *const u8 as
                          *const libc::c_char,
                      405 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void tokenadd(struct jv_parser *, char)\x00")).as_ptr());
    };
    let fresh1 = (*p).tokenpos;
    (*p).tokenpos = (*p).tokenpos + 1;
    *(*p).tokenbuf.offset(fresh1 as isize) = c;
}
unsafe extern "C" fn unhex4(mut hex: *mut libc::c_char) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let fresh2 = hex;
        hex = hex.offset(1);
        let mut c: libc::c_char = *fresh2;
        let mut n: libc::c_int = 0;
        if '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32 {
            n = c as libc::c_int - '0' as i32
        } else if 'a' as i32 <= c as libc::c_int &&
                      c as libc::c_int <= 'f' as i32 {
            n = c as libc::c_int - 'a' as i32 + 10 as libc::c_int
        } else if 'A' as i32 <= c as libc::c_int &&
                      c as libc::c_int <= 'F' as i32 {
            n = c as libc::c_int - 'A' as i32 + 10 as libc::c_int
        } else { return -(1 as libc::c_int) }
        r <<= 4 as libc::c_int;
        r |= n;
        i += 1
    }
    return r;
}
unsafe extern "C" fn found_string(mut p: *mut jv_parser) -> presult {
    let mut in_0: *mut libc::c_char = (*p).tokenbuf;
    let mut out: *mut libc::c_char = (*p).tokenbuf;
    let mut end: *mut libc::c_char =
        (*p).tokenbuf.offset((*p).tokenpos as isize);
    while in_0 < end {
        let fresh3 = in_0;
        in_0 = in_0.offset(1);
        let mut c: libc::c_char = *fresh3;
        if c as libc::c_int == '\\' as i32 {
            if in_0 >= end {
                return b"Expected escape character at end of string\x00" as
                           *const u8 as *const libc::c_char
            }
            let fresh4 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh4;
            let mut hexvalue: libc::c_int = 0;
            let mut codepoint: libc::c_ulong = 0;
            match c as libc::c_int {
                92 | 34 | 47 => {
                    let fresh5 = out;
                    out = out.offset(1);
                    *fresh5 = c
                }
                98 => {
                    let fresh6 = out;
                    out = out.offset(1);
                    *fresh6 = '\u{8}' as i32 as libc::c_char
                }
                102 => {
                    let fresh7 = out;
                    out = out.offset(1);
                    *fresh7 = '\u{c}' as i32 as libc::c_char
                }
                116 => {
                    let fresh8 = out;
                    out = out.offset(1);
                    *fresh8 = '\t' as i32 as libc::c_char
                }
                110 => {
                    let fresh9 = out;
                    out = out.offset(1);
                    *fresh9 = '\n' as i32 as libc::c_char
                }
                114 => {
                    let fresh10 = out;
                    out = out.offset(1);
                    *fresh10 = '\r' as i32 as libc::c_char
                }
                117 => {
                    /* ahh, the complicated case */
                    if in_0.offset(4 as libc::c_int as isize) > end {
                        return b"Invalid \\uXXXX escape\x00" as *const u8 as
                                   *const libc::c_char
                    }
                    hexvalue = unhex4(in_0);
                    if hexvalue < 0 as libc::c_int {
                        return b"Invalid characters in \\uXXXX escape\x00" as
                                   *const u8 as *const libc::c_char
                    }
                    codepoint = hexvalue as libc::c_ulong;
                    in_0 = in_0.offset(4 as libc::c_int as isize);
                    if 0xd800 as libc::c_int as libc::c_ulong <= codepoint &&
                           codepoint <= 0xdbff as libc::c_int as libc::c_ulong
                       {
                        /* who thought UTF-16 surrogate pairs were a good idea? */
                        if in_0.offset(6 as libc::c_int as isize) > end ||
                               *in_0.offset(0 as libc::c_int as isize) as
                                   libc::c_int != '\\' as i32 ||
                               *in_0.offset(1 as libc::c_int as isize) as
                                   libc::c_int != 'u' as i32 {
                            return b"Invalid \\uXXXX\\uXXXX surrogate pair escape\x00"
                                       as *const u8 as *const libc::c_char
                        } // U+FFFD REPLACEMENT CHARACTER
                        let mut surrogate: libc::c_ulong =
                            unhex4(in_0.offset(2 as libc::c_int as isize)) as
                                libc::c_ulong;
                        if !(0xdc00 as libc::c_int as libc::c_ulong <=
                                 surrogate &&
                                 surrogate <=
                                     0xdfff as libc::c_int as libc::c_ulong) {
                            return b"Invalid \\uXXXX\\uXXXX surrogate pair escape\x00"
                                       as *const u8 as *const libc::c_char
                        }
                        in_0 = in_0.offset(6 as libc::c_int as isize);
                        codepoint =
                            (0x10000 as libc::c_int as
                                 libc::c_ulong).wrapping_add(codepoint.wrapping_sub(0xd800
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                                 <<
                                                                 10 as
                                                                     libc::c_int
                                                                 |
                                                                 surrogate.wrapping_sub(0xdc00
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong))
                    }
                    if codepoint > 0x10ffff as libc::c_int as libc::c_ulong {
                        codepoint = 0xfffd as libc::c_int as libc::c_ulong
                    }
                    out =
                        out.offset(jvp_utf8_encode(codepoint as libc::c_int,
                                                   out) as isize)
                }
                _ => {
                    return b"Invalid escape\x00" as *const u8 as
                               *const libc::c_char
                }
            }
        } else {
            if c as libc::c_int > 0 as libc::c_int &&
                   (c as libc::c_int) < 0x1f as libc::c_int {
                return b"Invalid string: control characters from U+0000 through U+001F must be escaped\x00"
                           as *const u8 as *const libc::c_char
            }
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = c
        }
    }
    let mut msg__: presult =
        value(p,
              jv_string_sized((*p).tokenbuf,
                              out.wrapping_offset_from((*p).tokenbuf) as
                                  libc::c_long as libc::c_int));
    if !msg__.is_null() { return msg__ }
    (*p).tokenpos = 0 as libc::c_int;
    return 0 as presult;
}
unsafe extern "C" fn check_literal(mut p: *mut jv_parser) -> presult {
    if (*p).tokenpos == 0 as libc::c_int { return 0 as presult }
    let mut pattern: *const libc::c_char = 0 as *const libc::c_char;
    let mut plen: libc::c_int = 0;
    let mut v: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    match *(*p).tokenbuf.offset(0 as libc::c_int as isize) as libc::c_int {
        116 => {
            pattern = b"true\x00" as *const u8 as *const libc::c_char;
            plen = 4 as libc::c_int;
            v = jv_true()
        }
        102 => {
            pattern = b"false\x00" as *const u8 as *const libc::c_char;
            plen = 5 as libc::c_int;
            v = jv_false()
        }
        110 => {
            pattern = b"null\x00" as *const u8 as *const libc::c_char;
            plen = 4 as libc::c_int;
            v = jv_null()
        }
        _ => { }
    }
    if !pattern.is_null() {
        if (*p).tokenpos != plen {
            return b"Invalid literal\x00" as *const u8 as *const libc::c_char
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < plen {
            if *(*p).tokenbuf.offset(i as isize) as libc::c_int !=
                   *pattern.offset(i as isize) as libc::c_int {
                return b"Invalid literal\x00" as *const u8 as
                           *const libc::c_char
            }
            i += 1
        }
        let mut msg__: presult = value(p, v);
        if !msg__.is_null() { return msg__ }
    } else {
        // FIXME: better parser
        *(*p).tokenbuf.offset((*p).tokenpos as isize) =
            0 as libc::c_int as libc::c_char;
        let mut number: jv = jv_number_with_literal((*p).tokenbuf);
        if jv_get_kind(number) as libc::c_uint ==
               JV_KIND_INVALID as libc::c_int as libc::c_uint {
            return b"Invalid numeric literal\x00" as *const u8 as
                       *const libc::c_char
        }
        let mut msg___0: presult = value(p, number);
        if !msg___0.is_null() { return msg___0 }
    }
    (*p).tokenpos = 0 as libc::c_int;
    return 0 as presult;
}
unsafe extern "C" fn classify(mut c: libc::c_char) -> chclass {
    match c as libc::c_int {
        32 | 9 | 13 | 10 => { return WHITESPACE }
        34 => { return QUOTE }
        91 | 44 | 93 | 123 | 58 | 125 => { return STRUCTURE }
        _ => { return LITERAL }
    };
}
static mut OK: presult =
    b"output produced\x00" as *const u8 as *const libc::c_char;
unsafe extern "C" fn parse_check_done(mut p: *mut jv_parser, mut out: *mut jv)
 -> libc::c_int {
    if (*p).stackpos == 0 as libc::c_int && jv_is_valid((*p).next) != 0 {
        *out = (*p).next;
        (*p).next = jv_invalid();
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn stream_check_done(mut p: *mut jv_parser,
                                       mut out: *mut jv) -> libc::c_int {
    if (*p).stacklen == 0 as libc::c_int && jv_is_valid((*p).next) != 0 {
        *out =
            jv_array_append(jv_array_append(jv_array(), jv_copy((*p).path)),
                            (*p).next);
        (*p).next = jv_invalid();
        return 1 as libc::c_int
    } else if jv_is_valid((*p).output) != 0 {
        if jv_array_length(jv_copy((*p).output)) > 2 as libc::c_int {
            // At end of an array or object, necessitating one more output by
      // which to indicate this
            *out =
                jv_array_slice(jv_copy((*p).output), 0 as libc::c_int,
                               2 as libc::c_int);
            (*p).output =
                jv_array_slice((*p).output, 0 as libc::c_int,
                               1 as libc::c_int)
            // arrange one more output
        } else {
            // No further processing needed
            *out = (*p).output;
            (*p).output = jv_invalid()
        }
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn parse_check_truncation(mut p: *mut jv_parser)
 -> libc::c_int {
    return ((*p).flags & JV_PARSE_SEQ as libc::c_int != 0 &&
                (*p).last_ch_was_ws() == 0 &&
                ((*p).stackpos > 0 as libc::c_int ||
                     (*p).tokenpos > 0 as libc::c_int ||
                     jv_get_kind((*p).next) as libc::c_uint ==
                         JV_KIND_NUMBER as libc::c_int as libc::c_uint)) as
               libc::c_int;
}
unsafe extern "C" fn stream_check_truncation(mut p: *mut jv_parser)
 -> libc::c_int {
    let mut k: jv_kind = jv_get_kind((*p).next);
    return ((*p).stacklen > 0 as libc::c_int ||
                k as libc::c_uint ==
                    JV_KIND_NUMBER as libc::c_int as libc::c_uint ||
                k as libc::c_uint ==
                    JV_KIND_TRUE as libc::c_int as libc::c_uint ||
                k as libc::c_uint ==
                    JV_KIND_FALSE as libc::c_int as libc::c_uint ||
                k as libc::c_uint ==
                    JV_KIND_NULL as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn parse_is_top_num(mut p: *mut jv_parser) -> libc::c_int {
    return ((*p).stackpos == 0 as libc::c_int &&
                jv_get_kind((*p).next) as libc::c_uint ==
                    JV_KIND_NUMBER as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn stream_is_top_num(mut p: *mut jv_parser) -> libc::c_int {
    return ((*p).stacklen == 0 as libc::c_int &&
                jv_get_kind((*p).next) as libc::c_uint ==
                    JV_KIND_NUMBER as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn scan(mut p: *mut jv_parser, mut ch: libc::c_char,
                          mut out: *mut jv) -> presult {
    (*p).column += 1;
    if ch as libc::c_int == '\n' as i32 {
        (*p).line += 1;
        (*p).column = 0 as libc::c_int
    }
    if ch as libc::c_int == '\u{1e}' as i32 {
        /* ASCII RS; see draft-ietf-json-sequence-07 */
        if if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
               stream_check_truncation(p)
           } else { parse_check_truncation(p) } != 0 {
            if check_literal(p).is_null() &&
                   (if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
                        stream_is_top_num(p)
                    } else { parse_is_top_num(p) }) != 0 {
                return b"Potentially truncated top-level numeric value\x00" as
                           *const u8 as *const libc::c_char
            }
            return b"Truncated value\x00" as *const u8 as *const libc::c_char
        }
        let mut msg__: presult = check_literal(p);
        if !msg__.is_null() { return msg__ }
        if (*p).st as libc::c_uint ==
               JV_PARSER_NORMAL as libc::c_int as libc::c_uint &&
               (if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
                    stream_check_done(p, out)
                } else { parse_check_done(p, out) }) != 0 {
            return OK
        }
        // shouldn't happen?
        if jv_is_valid(*out) == 0 {
        } else {
            __assert_fail(b"!jv_is_valid(*out)\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/jv_parse.c\x00" as *const u8 as
                              *const libc::c_char,
                          632 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 45],
                                                    &[libc::c_char; 45]>(b"presult scan(struct jv_parser *, char, jv *)\x00")).as_ptr());
        };
        parser_reset(p);
        jv_free(*out);
        *out = jv_invalid();
        return OK
    }
    let mut answer: presult = 0 as presult;
    (*p).set_last_ch_was_ws(0 as libc::c_int as libc::c_uint);
    if (*p).st as libc::c_uint ==
           JV_PARSER_NORMAL as libc::c_int as libc::c_uint {
        let mut cls: chclass = classify(ch);
        if cls as libc::c_uint == WHITESPACE as libc::c_int as libc::c_uint {
            (*p).set_last_ch_was_ws(1 as libc::c_int as libc::c_uint)
        }
        if cls as libc::c_uint != LITERAL as libc::c_int as libc::c_uint {
            let mut msg___0: presult = check_literal(p);
            if !msg___0.is_null() { return msg___0 }
            if if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
                   stream_check_done(p, out)
               } else { parse_check_done(p, out) } != 0 {
                answer = OK
            }
        }
        match cls as libc::c_uint {
            0 => { tokenadd(p, ch); }
            3 => { (*p).st = JV_PARSER_STRING }
            2 => {
                let mut msg___1: presult =
                    if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
                        stream_token(p, ch)
                    } else { parse_token(p, ch) };
                if !msg___1.is_null() { return msg___1 }
            }
            4 => {
                return b"Invalid character\x00" as *const u8 as
                           *const libc::c_char
            }
            1 | _ => { }
        }
        if if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
               stream_check_done(p, out)
           } else { parse_check_done(p, out) } != 0 {
            answer = OK
        }
    } else if ch as libc::c_int == '\"' as i32 &&
                  (*p).st as libc::c_uint ==
                      JV_PARSER_STRING as libc::c_int as libc::c_uint {
        let mut msg___2: presult = found_string(p);
        if !msg___2.is_null() { return msg___2 }
        (*p).st = JV_PARSER_NORMAL;
        if if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 {
               stream_check_done(p, out)
           } else { parse_check_done(p, out) } != 0 {
            answer = OK
        }
    } else {
        tokenadd(p, ch);
        if ch as libc::c_int == '\\' as i32 &&
               (*p).st as libc::c_uint ==
                   JV_PARSER_STRING as libc::c_int as libc::c_uint {
            (*p).st = JV_PARSER_STRING_ESCAPE
        } else { (*p).st = JV_PARSER_STRING }
    }
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn jv_parser_new(mut flags: libc::c_int)
 -> *mut jv_parser {
    let mut p: *mut jv_parser =
        jv_mem_alloc(::std::mem::size_of::<jv_parser>() as libc::c_ulong) as
            *mut jv_parser;
    parser_init(p, flags);
    (*p).flags = flags;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn jv_parser_free(mut p: *mut jv_parser) {
    parser_free(p);
    jv_mem_free(p as *mut libc::c_void);
}
static mut UTF8_BOM: [libc::c_uchar; 3] =
    [0xef as libc::c_int as libc::c_uchar,
     0xbb as libc::c_int as libc::c_uchar,
     0xbf as libc::c_int as libc::c_uchar];
#[no_mangle]
pub unsafe extern "C" fn jv_parser_remaining(mut p: *mut jv_parser)
 -> libc::c_int {
    if (*p).curr_buf.is_null() { return 0 as libc::c_int }
    return (*p).curr_buf_length - (*p).curr_buf_pos;
}
#[no_mangle]
pub unsafe extern "C" fn jv_parser_set_buf(mut p: *mut jv_parser,
                                           mut buf: *const libc::c_char,
                                           mut length: libc::c_int,
                                           mut is_partial: libc::c_int) {
    if ((*p).curr_buf.is_null() || (*p).curr_buf_pos == (*p).curr_buf_length)
           &&
           !(b"previous buffer not exhausted\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"(p->curr_buf == 0 || p->curr_buf_pos == p->curr_buf_length) && \"previous buffer not exhausted\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv_parse.c\x00" as *const u8 as
                          *const libc::c_char,
                      703 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void jv_parser_set_buf(struct jv_parser *, const char *, int, int)\x00")).as_ptr());
    };
    while length > 0 as libc::c_int &&
              ((*p).bom_strip_position as libc::c_ulong) <
                  ::std::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong
          {
        if *buf as libc::c_uchar as libc::c_int ==
               UTF8_BOM[(*p).bom_strip_position as usize] as libc::c_int {
            // matched a BOM character
            buf = buf.offset(1);
            length -= 1;
            (*p).bom_strip_position = (*p).bom_strip_position.wrapping_add(1)
        } else if (*p).bom_strip_position == 0 as libc::c_int as libc::c_uint
         {
            // no BOM in this document
            (*p).bom_strip_position =
                ::std::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong
                    as libc::c_uint
        } else {
            // malformed BOM (prefix present, rest missing)
            (*p).bom_strip_position = 0xff as libc::c_int as libc::c_uint
        }
    } // Need a buffer
    (*p).curr_buf = buf;
    (*p).curr_buf_length = length;
    (*p).curr_buf_pos = 0 as libc::c_int;
    (*p).curr_buf_is_partial = is_partial;
}
unsafe extern "C" fn make_error(mut p: *mut jv_parser,
                                mut fmt: *const libc::c_char, mut args: ...)
 -> jv {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut e: jv = jv_string_vfmt(fmt, ap.as_va_list());
    if (*p).flags & JV_PARSE_STREAM_ERRORS as libc::c_int != 0 {
        return jv_array_append(jv_array_append(jv_array(), e),
                               jv_copy((*p).path))
    }
    return jv_invalid_with_msg(e);
}
#[no_mangle]
pub unsafe extern "C" fn jv_parser_next(mut p: *mut jv_parser) -> jv {
    if (*p).eof != 0 { return jv_invalid() }
    if (*p).curr_buf.is_null() { return jv_invalid() }
    if (*p).bom_strip_position == 0xff as libc::c_int as libc::c_uint {
        if (*p).flags & JV_PARSE_SEQ as libc::c_int == 0 {
            return jv_invalid_with_msg(jv_string(b"Malformed BOM\x00" as
                                                     *const u8 as
                                                     *const libc::c_char))
        }
        (*p).st = JV_PARSER_WAITING_FOR_RS;
        parser_reset(p);
    }
    let mut value_0: jv = jv_invalid();
    if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 &&
           stream_check_done(p, &mut value_0) != 0 {
        return value_0
    }
    let mut ch: libc::c_char = 0;
    let mut msg: presult = 0 as presult;
    while msg.is_null() && (*p).curr_buf_pos < (*p).curr_buf_length {
        let fresh12 = (*p).curr_buf_pos;
        (*p).curr_buf_pos = (*p).curr_buf_pos + 1;
        ch = *(*p).curr_buf.offset(fresh12 as isize);
        if (*p).st as libc::c_uint ==
               JV_PARSER_WAITING_FOR_RS as libc::c_int as libc::c_uint {
            if ch as libc::c_int == '\n' as i32 {
                (*p).line += 1;
                (*p).column = 0 as libc::c_int
            } else { (*p).column += 1 }
            if ch as libc::c_int == '\u{1e}' as i32 {
                (*p).st = JV_PARSER_NORMAL
            }
            // need to resync, wait for RS
        } else { msg = scan(p, ch, &mut value_0) }
    }
    if msg == OK {
        return value_0
    } else if !msg.is_null() {
        jv_free(value_0);
        if ch as libc::c_int != '\u{1e}' as i32 &&
               (*p).flags & JV_PARSE_SEQ as libc::c_int != 0 {
            // Skip to the next RS
            (*p).st =
                JV_PARSER_WAITING_FOR_RS; // Else ch must be RS; don't clear buf so we can start parsing again after this ch
            value_0 =
                make_error(p,
                           b"%s at line %d, column %d (need RS to resync)\x00"
                               as *const u8 as *const libc::c_char, msg,
                           (*p).line, (*p).column);
            parser_reset(p);
            return value_0
        }
        value_0 =
            make_error(p,
                       b"%s at line %d, column %d\x00" as *const u8 as
                           *const libc::c_char, msg, (*p).line, (*p).column);
        parser_reset(p);
        if (*p).flags & JV_PARSE_SEQ as libc::c_int == 0 {
            // We're not parsing a JSON text sequence; throw this buffer away.
      // XXX We should fail permanently here.
            (*p).curr_buf = 0 as *const libc::c_char;
            (*p).curr_buf_pos = 0 as libc::c_int
        }
        return value_0
    } else if (*p).curr_buf_is_partial != 0 {
        if (*p).curr_buf_pos == (*p).curr_buf_length {
        } else {
            __assert_fail(b"p->curr_buf_pos == p->curr_buf_length\x00" as
                              *const u8 as *const libc::c_char,
                          b"src/jv_parse.c\x00" as *const u8 as
                              *const libc::c_char,
                          790 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"jv jv_parser_next(struct jv_parser *)\x00")).as_ptr());
        };
        // need another buffer
        return jv_invalid()
    } else {
        // at EOF
        (*p).eof = 1 as libc::c_int;
        if (*p).curr_buf_pos == (*p).curr_buf_length {
        } else {
            __assert_fail(b"p->curr_buf_pos == p->curr_buf_length\x00" as
                              *const u8 as *const libc::c_char,
                          b"src/jv_parse.c\x00" as *const u8 as
                              *const libc::c_char,
                          796 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"jv jv_parser_next(struct jv_parser *)\x00")).as_ptr());
        };
        jv_free(value_0);
        if (*p).st as libc::c_uint ==
               JV_PARSER_WAITING_FOR_RS as libc::c_int as libc::c_uint {
            return make_error(p,
                              b"Unfinished abandoned text at EOF at line %d, column %d\x00"
                                  as *const u8 as *const libc::c_char,
                              (*p).line, (*p).column)
        }
        if (*p).st as libc::c_uint !=
               JV_PARSER_NORMAL as libc::c_int as libc::c_uint {
            value_0 =
                make_error(p,
                           b"Unfinished string at EOF at line %d, column %d\x00"
                               as *const u8 as *const libc::c_char, (*p).line,
                           (*p).column);
            parser_reset(p);
            (*p).st = JV_PARSER_WAITING_FOR_RS;
            return value_0
        }
        msg = check_literal(p);
        if !msg.is_null() {
            value_0 =
                make_error(p,
                           b"%s at EOF at line %d, column %d\x00" as *const u8
                               as *const libc::c_char, msg, (*p).line,
                           (*p).column);
            parser_reset(p);
            (*p).st = JV_PARSER_WAITING_FOR_RS;
            return value_0
        }
        if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 &&
               (*p).stacklen != 0 as libc::c_int ||
               (*p).flags & JV_PARSE_STREAMING as libc::c_int == 0 &&
                   (*p).stackpos != 0 as libc::c_int {
            value_0 =
                make_error(p,
                           b"Unfinished JSON term at EOF at line %d, column %d\x00"
                               as *const u8 as *const libc::c_char, (*p).line,
                           (*p).column);
            parser_reset(p);
            (*p).st = JV_PARSER_WAITING_FOR_RS;
            return value_0
        }
        // p->next is either invalid (nothing here, but no syntax error)
    // or valid (this is the value). either way it's the thing to return
        if (*p).flags & JV_PARSE_STREAMING as libc::c_int != 0 &&
               jv_is_valid((*p).next) != 0 {
            value_0 =
                jv_array_append(jv_array_append(jv_array(),
                                                jv_copy((*p).path)),
                                (*p).next)
            // except in streaming mode we've got to make it [path,value]
        } else { value_0 = (*p).next }
        (*p).next = jv_invalid();
        if (*p).flags & JV_PARSE_SEQ as libc::c_int != 0 &&
               (*p).last_ch_was_ws() == 0 &&
               jv_get_kind(value_0) as libc::c_uint ==
                   JV_KIND_NUMBER as libc::c_int as libc::c_uint {
            jv_free(value_0);
            return make_error(p,
                              b"Potentially truncated top-level numeric value at EOF at line %d, column %d\x00"
                                  as *const u8 as *const libc::c_char,
                              (*p).line, (*p).column)
        }
        return value_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_parse_sized(mut string: *const libc::c_char,
                                        mut length: libc::c_int) -> jv {
    let mut parser: jv_parser =
        jv_parser{curr_buf: 0 as *const libc::c_char,
                  curr_buf_length: 0,
                  curr_buf_pos: 0,
                  curr_buf_is_partial: 0,
                  eof: 0,
                  bom_strip_position: 0,
                  flags: 0,
                  stack: 0 as *mut jv,
                  stackpos: 0,
                  stacklen: 0,
                  path:
                      jv{kind_flags: 0,
                         pad_: 0,
                         offset: 0,
                         size: 0,
                         u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},},
                  last_seen: JV_LAST_NONE,
                  output:
                      jv{kind_flags: 0,
                         pad_: 0,
                         offset: 0,
                         size: 0,
                         u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},},
                  next:
                      jv{kind_flags: 0,
                         pad_: 0,
                         offset: 0,
                         size: 0,
                         u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},},
                  tokenbuf: 0 as *mut libc::c_char,
                  tokenpos: 0,
                  tokenlen: 0,
                  line: 0,
                  column: 0,
                  dtoa:
                      dtoa_context{freelist: [0 as *mut Bigint; 8],
                                   p5s: 0 as *mut Bigint,},
                  st: JV_PARSER_NORMAL,
                  last_ch_was_ws: [0; 1],
                  c2rust_padding: [0; 3],};
    parser_init(&mut parser, 0 as libc::c_int);
    jv_parser_set_buf(&mut parser, string, length, 0 as libc::c_int);
    let mut value_0: jv = jv_parser_next(&mut parser);
    if jv_is_valid(value_0) != 0 {
        let mut next: jv = jv_parser_next(&mut parser);
        if jv_is_valid(next) != 0 {
            // multiple JSON values, we only wanted one
            jv_free(value_0);
            jv_free(next);
            value_0 =
                jv_invalid_with_msg(jv_string(b"Unexpected extra JSON values\x00"
                                                  as *const u8 as
                                                  *const libc::c_char))
        } else if jv_invalid_has_msg(jv_copy(next)) != 0 {
            // parser error after the first JSON value
            jv_free(value_0);
            value_0 = next
        } else {
            // a single valid JSON value
            jv_free(next);
        }
    } else if !(jv_invalid_has_msg(jv_copy(value_0)) != 0) {
        // no value at all
        jv_free(value_0);
        value_0 =
            jv_invalid_with_msg(jv_string(b"Expected JSON value\x00" as
                                              *const u8 as
                                              *const libc::c_char))
    }
    parser_free(&mut parser);
    if jv_is_valid(value_0) == 0 && jv_invalid_has_msg(jv_copy(value_0)) != 0
       {
        let mut msg: jv = jv_invalid_get_msg(value_0);
        value_0 =
            jv_invalid_with_msg(jv_string_fmt(b"%s (while parsing \'%s\')\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              jv_string_value(msg), string));
        jv_free(msg);
    }
    return value_0;
}
#[no_mangle]
pub unsafe extern "C" fn jv_parse(mut string: *const libc::c_char) -> jv {
    return jv_parse_sized(string, strlen(string) as libc::c_int);
}
