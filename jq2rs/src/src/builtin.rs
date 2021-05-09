#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jq_state;
    pub type inst;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn strptime(__s: *const libc::c_char, __fmt: *const libc::c_char,
                __tp: *mut tm) -> *mut libc::c_char;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn timegm(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn acos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn asin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cosh(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sinh(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tanh(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn acosh(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn asinh(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atanh(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn exp10(__x: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow10(__x: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn expm1(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log1p(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn logb(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn exp2(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log2(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cbrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __isinfl(__value: f128::f128) -> libc::c_int;
    #[no_mangle]
    fn __isinf(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __isinff(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn drem(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn significand(__x: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn copysign(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __isnanf(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __isnanl(__value: f128::f128) -> libc::c_int;
    #[no_mangle]
    fn __isnan(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn j0(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn j1(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn jn(_: libc::c_int, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn y0(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn y1(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn yn(_: libc::c_int, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn erf(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn erfc(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn lgamma(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tgamma(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn gamma(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn lgamma_r(_: libc::c_double, __signgamp: *mut libc::c_int)
     -> libc::c_double;
    #[no_mangle]
    fn rint(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn nextafter(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn nexttoward(_: libc::c_double, _: f128::f128) -> libc::c_double;
    #[no_mangle]
    fn remainder(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn scalbln(_: libc::c_double, _: libc::c_long) -> libc::c_double;
    #[no_mangle]
    fn nearbyint(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn round(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn trunc(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fdim(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __fpclassifyf(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __fpclassify(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __fpclassifyl(__value: f128::f128) -> libc::c_int;
    #[no_mangle]
    fn fma(_: libc::c_double, _: libc::c_double, _: libc::c_double)
     -> libc::c_double;
    #[no_mangle]
    fn scalb(__x: libc::c_double, __n: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut OnigEncodingUTF8: OnigEncodingType;
    #[no_mangle]
    static mut OnigSyntaxPerl_NG: OnigSyntaxType;
    #[no_mangle]
    fn onig_error_code_to_str() -> libc::c_int;
    #[no_mangle]
    fn onig_new(_: *mut OnigRegex, pattern: *const OnigUChar,
                pattern_end: *const OnigUChar, option: OnigOptionType,
                enc: OnigEncoding, syntax: *mut OnigSyntaxType,
                einfo: *mut OnigErrorInfo) -> libc::c_int;
    #[no_mangle]
    fn onig_free(_: OnigRegex);
    #[no_mangle]
    fn onig_search(_: OnigRegex, str: *const OnigUChar, end: *const OnigUChar,
                   start: *const OnigUChar, range: *const OnigUChar,
                   region: *mut OnigRegion, option: OnigOptionType)
     -> libc::c_int;
    #[no_mangle]
    fn onig_region_new() -> *mut OnigRegion;
    #[no_mangle]
    fn onig_region_free(region: *mut OnigRegion, free_self: libc::c_int);
    #[no_mangle]
    fn onig_foreach_name(reg: OnigRegex,
                         func:
                             Option<unsafe extern "C" fn(_: *const OnigUChar,
                                                         _: *const OnigUChar,
                                                         _: libc::c_int,
                                                         _: *mut libc::c_int,
                                                         _: OnigRegex,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int>,
                         arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_kind_name(_: jv_kind) -> *const libc::c_char;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_equal(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_contains(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_invalid() -> jv;
    #[no_mangle]
    fn jv_invalid_with_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_has_msg(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_true() -> jv;
    #[no_mangle]
    fn jv_false() -> jv;
    #[no_mangle]
    fn jv_bool(_: libc::c_int) -> jv;
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
    fn jv_array_set(_: jv, _: libc::c_int, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_concat(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_sized(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_empty(len: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_length_codepoints(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_indexes(j: jv, k: jv) -> jv;
    #[no_mangle]
    fn jv_string_concat(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_string_append_buf(a: jv, buf: *const libc::c_char, len: libc::c_int)
     -> jv;
    #[no_mangle]
    fn jv_string_append_str(a: jv, str: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_split(j: jv, sep: jv) -> jv;
    #[no_mangle]
    fn jv_string_explode(j: jv) -> jv;
    #[no_mangle]
    fn jv_string_implode(j: jv) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_object_length(object: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_merge(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_object_merge_recursive(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_dumpf(_: jv, f: *mut FILE, flags: libc::c_int);
    #[no_mangle]
    fn jv_dump_string(_: jv, flags: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_dump_string_trunc(x: jv, outbuf: *mut libc::c_char, bufsize: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn jv_parse(string: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_parse_sized(string: *const libc::c_char, length: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_has(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_setpath(_: jv, _: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_getpath(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_delpaths(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_keys(_: jv) -> jv;
    #[no_mangle]
    fn jv_keys_unsorted(_: jv) -> jv;
    #[no_mangle]
    fn jv_cmp(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_group(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_sort(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jq_halt(_: *mut jq_state, _: jv, _: jv);
    #[no_mangle]
    fn jq_get_input_cb(_: *mut jq_state, _: *mut jq_input_cb,
                       _: *mut *mut libc::c_void);
    #[no_mangle]
    fn jq_get_debug_cb(_: *mut jq_state, _: *mut jq_msg_cb,
                       _: *mut *mut libc::c_void);
    #[no_mangle]
    fn jq_get_jq_origin(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_get_prog_origin(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_get_lib_dirs(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_util_input_get_current_filename(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_util_input_get_current_line(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn locfile_init(_: *mut jq_state, _: *const libc::c_char,
                    _: *const libc::c_char, _: libc::c_int) -> *mut locfile;
    #[no_mangle]
    fn locfile_free(_: *mut locfile);
    #[no_mangle]
    fn gen_noop() -> block;
    #[no_mangle]
    fn gen_op_simple(op: opcode) -> block;
    #[no_mangle]
    fn gen_const(constant: jv) -> block;
    #[no_mangle]
    fn gen_op_bound(op: opcode, binder: block) -> block;
    #[no_mangle]
    fn gen_op_var_fresh(op: opcode, name: *const libc::c_char) -> block;
    #[no_mangle]
    fn gen_function(name: *const libc::c_char, formals: block, body: block)
     -> block;
    #[no_mangle]
    fn gen_param(name: *const libc::c_char) -> block;
    #[no_mangle]
    fn gen_call(name: *const libc::c_char, body: block) -> block;
    #[no_mangle]
    fn gen_condbranch(iftrue: block, iffalse: block) -> block;
    #[no_mangle]
    fn block_list_funcs(body: block, omit_underscores: libc::c_int) -> jv;
    #[no_mangle]
    fn gen_cbinding(functions: *const cfunction, nfunctions: libc::c_int,
                    b: block) -> block;
    #[no_mangle]
    fn block_join(a: block, b: block) -> block;
    #[no_mangle]
    fn block_bind_referenced(binder: block, body: block,
                             bindflags: libc::c_int) -> block;
    #[no_mangle]
    fn jq_parse_library(locations: *mut locfile, answer: *mut block)
     -> libc::c_int;
    #[no_mangle]
    fn load_module_meta(jq: *mut jq_state, modname: jv) -> jv;
    #[no_mangle]
    fn jvp_utf8_next(in_0: *const libc::c_char, end: *const libc::c_char,
                     codepoint: *mut libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn jvp_utf8_decode_length(startchar: libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn jv_mem_calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    // FIXME Should autoconf check for this!
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn _jq_path_append(_: *mut jq_state, _: jv, _: jv, _: jv) -> jv;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/* *********************************************************************
  oniguruma.h - Oniguruma (regular expression library)
**********************************************************************/
/*-
 * Copyright (c) 2002-2016  K.Kosako  <sndgk393 AT ybb DOT ne DOT jp>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/* escape Mac OS X/Xcode 2.4/gcc 4.0.1 problem */
/* PART: character encoding */
pub type OnigCodePoint = libc::c_uint;
pub type OnigUChar = libc::c_uchar;
pub type OnigCtype = libc::c_uint;
pub type OnigLen = libc::c_uint;
pub type OnigCaseFoldType = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigCaseFoldCodeItem {
    pub byte_len: libc::c_int,
    pub code_len: libc::c_int,
    pub code: [OnigCodePoint; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigMetaCharTableType {
    pub esc: OnigCodePoint,
    pub anychar: OnigCodePoint,
    pub anytime: OnigCodePoint,
    pub zero_or_one_time: OnigCodePoint,
    pub one_or_more_time: OnigCodePoint,
    pub anychar_anytime: OnigCodePoint,
}
pub type OnigApplyAllCaseFoldFunc
    =
    Option<unsafe extern "C" fn(_: OnigCodePoint, _: *mut OnigCodePoint,
                                _: libc::c_int, _: *mut libc::c_void)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigEncodingTypeST {
    pub mbc_enc_len: Option<unsafe extern "C" fn(_: *const OnigUChar)
                                -> libc::c_int>,
    pub name: *const libc::c_char,
    pub max_enc_len: libc::c_int,
    pub min_enc_len: libc::c_int,
    pub is_mbc_newline: Option<unsafe extern "C" fn(_: *const OnigUChar,
                                                    _: *const OnigUChar)
                                   -> libc::c_int>,
    pub mbc_to_code: Option<unsafe extern "C" fn(_: *const OnigUChar,
                                                 _: *const OnigUChar)
                                -> OnigCodePoint>,
    pub code_to_mbclen: Option<unsafe extern "C" fn(_: OnigCodePoint)
                                   -> libc::c_int>,
    pub code_to_mbc: Option<unsafe extern "C" fn(_: OnigCodePoint,
                                                 _: *mut OnigUChar)
                                -> libc::c_int>,
    pub mbc_case_fold: Option<unsafe extern "C" fn(_: OnigCaseFoldType,
                                                   _: *mut *const OnigUChar,
                                                   _: *const OnigUChar,
                                                   _: *mut OnigUChar)
                                  -> libc::c_int>,
    pub apply_all_case_fold: Option<unsafe extern "C" fn(_: OnigCaseFoldType,
                                                         _:
                                                             OnigApplyAllCaseFoldFunc,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int>,
    pub get_case_fold_codes_by_str: Option<unsafe extern "C" fn(_:
                                                                    OnigCaseFoldType,
                                                                _:
                                                                    *const OnigUChar,
                                                                _:
                                                                    *const OnigUChar,
                                                                _:
                                                                    *mut OnigCaseFoldCodeItem)
                                               -> libc::c_int>,
    pub property_name_to_ctype: Option<unsafe extern "C" fn(_:
                                                                *mut OnigEncodingTypeST,
                                                            _: *mut OnigUChar,
                                                            _: *mut OnigUChar)
                                           -> libc::c_int>,
    pub is_code_ctype: Option<unsafe extern "C" fn(_: OnigCodePoint,
                                                   _: OnigCtype)
                                  -> libc::c_int>,
    pub get_ctype_code_range: Option<unsafe extern "C" fn(_: OnigCtype,
                                                          _:
                                                              *mut OnigCodePoint,
                                                          _:
                                                              *mut *const OnigCodePoint)
                                         -> libc::c_int>,
    pub left_adjust_char_head: Option<unsafe extern "C" fn(_:
                                                               *const OnigUChar,
                                                           _:
                                                               *const OnigUChar)
                                          -> *mut OnigUChar>,
    pub is_allowed_reverse_match: Option<unsafe extern "C" fn(_:
                                                                  *const OnigUChar,
                                                              _:
                                                                  *const OnigUChar)
                                             -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub is_initialized: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub is_valid_mbc_string: Option<unsafe extern "C" fn(_: *const OnigUChar,
                                                         _: *const OnigUChar)
                                        -> libc::c_int>,
}
pub type OnigEncodingType = OnigEncodingTypeST;
pub type OnigEncoding = *mut OnigEncodingType;
/* PART: regular expression */
/* config parameters */
/* constants */
pub type OnigOptionType = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigSyntaxType {
    pub op: libc::c_uint,
    pub op2: libc::c_uint,
    pub behavior: libc::c_uint,
    pub options: OnigOptionType,
    pub meta_char_table: OnigMetaCharTableType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigCaptureTreeNodeStruct {
    pub group: libc::c_int,
    pub beg: libc::c_int,
    pub end: libc::c_int,
    pub allocated: libc::c_int,
    pub num_childs: libc::c_int,
    pub childs: *mut *mut OnigCaptureTreeNodeStruct,
}
/* internal error */
/* general error */
/* syntax error */
/* values error (syntax error) */
/* errors related to thread */
/* #define ONIGERR_OVER_THREAD_PASS_LIMIT_COUNT                -1001 */
/* must be smaller than BIT_STATUS_BITS_NUM (unsigned int * 8) */
pub type OnigCaptureTreeNode = OnigCaptureTreeNodeStruct;
/* group number */
/* match result region type */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub allocated: libc::c_int,
    pub num_regs: libc::c_int,
    pub beg: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub history_root: *mut OnigCaptureTreeNode,
}
/* capture history tree root */
/* capture tree traverse */
pub type OnigRegion = re_registers;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigErrorInfo {
    pub enc: OnigEncoding,
    pub par: *mut OnigUChar,
    pub par_end: *mut OnigUChar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigRepeatRange {
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub p: *mut libc::c_uchar,
    pub used: libc::c_uint,
    pub alloc: libc::c_uint,
    pub num_mem: libc::c_int,
    pub num_repeat: libc::c_int,
    pub num_null_check: libc::c_int,
    pub num_comb_exp_check: libc::c_int,
    pub num_call: libc::c_int,
    pub capture_history: libc::c_uint,
    pub bt_mem_start: libc::c_uint,
    pub bt_mem_end: libc::c_uint,
    pub stack_pop_level: libc::c_int,
    pub repeat_range_alloc: libc::c_int,
    pub repeat_range: *mut OnigRepeatRange,
    pub enc: OnigEncoding,
    pub options: OnigOptionType,
    pub syntax: *mut OnigSyntaxType,
    pub case_fold_flag: OnigCaseFoldType,
    pub name_table: *mut libc::c_void,
    pub optimize: libc::c_int,
    pub threshold_len: libc::c_int,
    pub anchor: libc::c_int,
    pub anchor_dmin: OnigLen,
    pub anchor_dmax: OnigLen,
    pub sub_anchor: libc::c_int,
    pub exact: *mut libc::c_uchar,
    pub exact_end: *mut libc::c_uchar,
    pub map: [libc::c_uchar; 256],
    pub int_map: *mut libc::c_int,
    pub int_map_backward: *mut libc::c_int,
    pub dmin: OnigLen,
    pub dmax: OnigLen,
    pub chain: *mut re_pattern_buffer,
}
pub type OnigRegexType = re_pattern_buffer;
pub type OnigRegex = *mut OnigRegexType;
pub type regex_t = OnigRegexType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
pub type uint32_t = libc::c_uint;
pub type intmax_t = libc::c_long;
pub type jv_kind = libc::c_uint;
pub const JV_KIND_OBJECT: jv_kind = 7;
pub const JV_KIND_ARRAY: jv_kind = 6;
pub const JV_KIND_STRING: jv_kind = 5;
pub const JV_KIND_NUMBER: jv_kind = 4;
pub const JV_KIND_TRUE: jv_kind = 3;
pub const JV_KIND_FALSE: jv_kind = 2;
pub const JV_KIND_NULL: jv_kind = 1;
pub const JV_KIND_INVALID: jv_kind = 0;
/* common members of BBuf(bytes-buffer) */
/* compiled pattern */
/* used space for p */
/* allocated space for p */
/* used memory(...) num counted from 1 */
/* OP_REPEAT/OP_REPEAT_NG id-counter */
/* OP_NULL_CHECK_START/END id counter */
/* combination explosion check */
/* number of subexp call */
/* (?@...) flag (1-31) */
/* need backtrack flag */
/* need backtrack flag */
/* optimization info (string search, char-map and anchors) */
/* optimize flag */
/* search str-length for apply optimize */
/* BEGIN_BUF, BEGIN_POS, (SEMI_)END_BUF */
/* (SEMI_)END_BUF anchor distance */
/* (SEMI_)END_BUF anchor distance */
/* start-anchor for exact or map */
/* used as BM skip or char-map */
/* BM skip for exact_len > 255 */
/* BM skip for backward search */
/* min-distance of exact or map */
/* max-distance of exact or map */
/* regex_t link chain */
/* escape compile-conflict */
/* All of the fields of this struct are private.
   Really. Do not play with them. */
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
pub type jq_msg_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: jv) -> ()>;
pub type jq_input_cb
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: *mut libc::c_void)
               -> jv>;
pub type opcode = libc::c_uint;
pub const STOREVN: opcode = 40;
pub const DESTRUCTURE_ALT: opcode = 39;
pub const GENLABEL: opcode = 38;
pub const MODULEMETA: opcode = 37;
pub const DEPS: opcode = 36;
pub const CLOSURE_PARAM_REGULAR: opcode = 35;
pub const TOP: opcode = 34;
pub const CLOSURE_CREATE_C: opcode = 33;
pub const CLOSURE_CREATE: opcode = 32;
pub const CLOSURE_REF: opcode = 31;
pub const CLOSURE_PARAM: opcode = 30;
pub const TAIL_CALL_JQ: opcode = 29;
pub const RET: opcode = 28;
pub const CALL_JQ: opcode = 27;
pub const CALL_BUILTIN: opcode = 26;
pub const PATH_END: opcode = 25;
pub const PATH_BEGIN: opcode = 24;
pub const SUBEXP_END: opcode = 23;
pub const SUBEXP_BEGIN: opcode = 22;
pub const RANGE: opcode = 21;
pub const INSERT: opcode = 20;
pub const APPEND: opcode = 19;
pub const BACKTRACK: opcode = 18;
pub const JUMP_F: opcode = 17;
pub const JUMP: opcode = 16;
pub const FORK_OPT: opcode = 15;
pub const FORK: opcode = 14;
pub const EACH_OPT: opcode = 13;
pub const EACH: opcode = 12;
pub const INDEX_OPT: opcode = 11;
pub const INDEX: opcode = 10;
pub const STORE_GLOBAL: opcode = 9;
pub const STOREV: opcode = 8;
pub const LOADVN: opcode = 7;
pub const LOADV: opcode = 6;
pub const POP: opcode = 5;
pub const PUSHK_UNDER: opcode = 4;
pub const DUP2: opcode = 3;
pub const DUPN: opcode = 2;
pub const DUP: opcode = 1;
pub const LOADK: opcode = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OP_BIND_WILDCARD: C2RustUnnamed_1 = 2048;
pub const OP_HAS_BINDING: C2RustUnnamed_1 = 1024;
pub const OP_IS_CALL_PSEUDO: C2RustUnnamed_1 = 128;
pub const OP_HAS_UFUNC: C2RustUnnamed_1 = 64;
pub const OP_HAS_CFUNC: C2RustUnnamed_1 = 32;
pub const OP_HAS_BRANCH: C2RustUnnamed_1 = 8;
pub const OP_HAS_VARIABLE: C2RustUnnamed_1 = 4;
pub const OP_HAS_CONSTANT: C2RustUnnamed_1 = 2;
pub type cfunction_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfunction {
    pub fptr: cfunction_ptr,
    pub name: *const libc::c_char,
    pub nargs: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub first: *mut inst,
    pub last: *mut inst,
}
pub type cmp_op = libc::c_uint;
pub const CMP_OP_GREATEREQ: cmp_op = 3;
pub const CMP_OP_LESSEQ: cmp_op = 2;
pub const CMP_OP_GREATER: cmp_op = 1;
pub const CMP_OP_LESS: cmp_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytecoded_builtin {
    pub name: *const libc::c_char,
    pub code: block,
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn type_error(mut bad: jv, mut msg: *const libc::c_char)
 -> jv {
    let mut errbuf: [libc::c_char; 15] = [0; 15];
    let mut err: jv =
        jv_invalid_with_msg(jv_string_fmt(b"%s (%s) %s\x00" as *const u8 as
                                              *const libc::c_char,
                                          jv_kind_name(jv_get_kind(bad)),
                                          jv_dump_string_trunc(jv_copy(bad),
                                                               errbuf.as_mut_ptr(),
                                                               ::std::mem::size_of::<[libc::c_char; 15]>()
                                                                   as
                                                                   libc::c_ulong),
                                          msg));
    jv_free(bad);
    return err;
}
unsafe extern "C" fn type_error2(mut bad1: jv, mut bad2: jv,
                                 mut msg: *const libc::c_char) -> jv {
    let mut errbuf1: [libc::c_char; 15] = [0; 15];
    let mut errbuf2: [libc::c_char; 15] = [0; 15];
    let mut err: jv =
        jv_invalid_with_msg(jv_string_fmt(b"%s (%s) and %s (%s) %s\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          jv_kind_name(jv_get_kind(bad1)),
                                          jv_dump_string_trunc(jv_copy(bad1),
                                                               errbuf1.as_mut_ptr(),
                                                               ::std::mem::size_of::<[libc::c_char; 15]>()
                                                                   as
                                                                   libc::c_ulong),
                                          jv_kind_name(jv_get_kind(bad2)),
                                          jv_dump_string_trunc(jv_copy(bad2),
                                                               errbuf2.as_mut_ptr(),
                                                               ::std::mem::size_of::<[libc::c_char; 15]>()
                                                                   as
                                                                   libc::c_ulong),
                                          msg));
    jv_free(bad1);
    jv_free(bad2);
    return err;
}
#[inline]
unsafe extern "C" fn ret_error(mut bad: jv, mut msg: jv) -> jv {
    jv_free(bad);
    return jv_invalid_with_msg(msg);
}
#[inline]
unsafe extern "C" fn ret_error2(mut bad1: jv, mut bad2: jv, mut msg: jv)
 -> jv {
    jv_free(bad1);
    jv_free(bad2);
    return jv_invalid_with_msg(msg);
}
unsafe extern "C" fn f_plus(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint {
        jv_free(a);
        return b
    } else if jv_get_kind(b) as libc::c_uint ==
                  JV_KIND_NULL as libc::c_int as libc::c_uint {
        jv_free(b);
        return a
    } else if jv_get_kind(a) as libc::c_uint ==
                  JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
                  jv_get_kind(b) as libc::c_uint ==
                      JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        let mut r: jv = jv_number(jv_number_value(a) + jv_number_value(b));
        jv_free(a);
        jv_free(b);
        return r
    } else if jv_get_kind(a) as libc::c_uint ==
                  JV_KIND_STRING as libc::c_int as libc::c_uint &&
                  jv_get_kind(b) as libc::c_uint ==
                      JV_KIND_STRING as libc::c_int as libc::c_uint {
        return jv_string_concat(a, b)
    } else if jv_get_kind(a) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
                  jv_get_kind(b) as libc::c_uint ==
                      JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_array_concat(a, b)
    } else if jv_get_kind(a) as libc::c_uint ==
                  JV_KIND_OBJECT as libc::c_int as libc::c_uint &&
                  jv_get_kind(b) as libc::c_uint ==
                      JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        return jv_object_merge(a, b)
    } else {
        return type_error2(a, b,
                           b"cannot be added\x00" as *const u8 as
                               *const libc::c_char)
    };
}
unsafe extern "C" fn f_acos(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(acos(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_acosh(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(acosh(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_asin(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(asin(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_asinh(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(asinh(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_atan(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(atan(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_atan2(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                             mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(atan2(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_atanh(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(atanh(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_cbrt(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(cbrt(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_cos(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(cos(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_cosh(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(cosh(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_exp(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(exp(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_exp2(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(exp2(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_floor(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(floor(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_hypot(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                             mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(hypot(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_j0(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(j0(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_j1(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(j1(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_log(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(log(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_log10(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(log10(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_log2(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(log2(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_pow(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                           mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(pow(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_remainder(mut jq: *mut jq_state, mut input: jv,
                                 mut a: jv, mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(remainder(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_sin(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(sin(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_sinh(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(sinh(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_sqrt(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(sqrt(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_tan(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(tan(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_tanh(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(tanh(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_tgamma(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(tgamma(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_y0(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(y0(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_y1(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(y1(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_jn(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                          mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(jn(jv_number_value(a) as libc::c_int, jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_yn(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                          mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(yn(jv_number_value(a) as libc::c_int, jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_ceil(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(ceil(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_copysign(mut jq: *mut jq_state, mut input: jv,
                                mut a: jv, mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(copysign(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_drem(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(drem(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_erf(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(erf(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_erfc(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(erfc(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_exp10(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(exp10(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_expm1(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(expm1(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_fabs(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(fabs(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_fdim(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(fdim(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_fma(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                           mut b: jv, mut c: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        jv_free(c);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        jv_free(c);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(c) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        jv_free(b);
        return type_error(c,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(fma(jv_number_value(a), jv_number_value(b),
                      jv_number_value(c)));
    jv_free(a);
    jv_free(b);
    jv_free(c);
    return ret;
}
unsafe extern "C" fn f_fmax(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(fmax(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_fmin(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(fmin(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_fmod(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(fmod(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_gamma(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(gamma(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_lgamma(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(lgamma(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_log1p(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(log1p(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_logb(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(logb(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_nearbyint(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(nearbyint(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_nextafter(mut jq: *mut jq_state, mut input: jv,
                                 mut a: jv, mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(nextafter(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_nexttoward(mut jq: *mut jq_state, mut input: jv,
                                  mut a: jv, mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(nexttoward(jv_number_value(a),
                             f128::f128::new(jv_number_value(b))));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_pow10(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(pow10(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_rint(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(rint(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_round(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(round(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_scalb(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                             mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(scalb(jv_number_value(a), jv_number_value(b)));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_scalbln(mut jq: *mut jq_state, mut input: jv,
                               mut a: jv, mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(scalbln(jv_number_value(a),
                          jv_number_value(b) as libc::c_long));
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_significand(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(significand(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_trunc(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(trunc(jv_number_value(input)));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_ldexp(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                             mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(b);
        return type_error(a,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    if jv_get_kind(b) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(b,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv =
        jv_number(ldexp(jv_number_value(a),
                        jv_number_value(b) as libc::c_int));
    jv_free(a);
    jv_free(b);
    return ret;
}
// ifdef __APPLE__
// Clean up after ourselves
unsafe extern "C" fn f_frexp(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut exp_0: libc::c_int = 0;
    let mut d: libc::c_double = frexp(jv_number_value(input), &mut exp_0);
    let mut ret: jv =
        jv_array_append(jv_array_append(jv_array(), jv_number(d)),
                        jv_number(exp_0 as libc::c_double));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_modf(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut i: libc::c_double = 0.;
    let mut ret: jv =
        jv_array_append(jv_array(),
                        jv_number(modf(jv_number_value(input), &mut i)));
    jv_free(input);
    return jv_array_append(ret, jv_number(i));
}
unsafe extern "C" fn f_lgamma_r(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut sign: libc::c_int = 0;
    let mut ret: jv =
        jv_array_append(jv_array(),
                        jv_number(lgamma_r(jv_number_value(input),
                                           &mut sign)));
    jv_free(input);
    return jv_array_append(ret, jv_number(sign as libc::c_double));
}
unsafe extern "C" fn f_negate(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"cannot be negated\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut ret: jv = jv_number(-jv_number_value(input));
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_startswith(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint ||
           jv_get_kind(b) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
        return ret_error2(a, b,
                          jv_string(b"startswith() requires string inputs\x00"
                                        as *const u8 as *const libc::c_char))
    }
    let mut alen: libc::c_int = jv_string_length_bytes(jv_copy(a));
    let mut blen: libc::c_int = jv_string_length_bytes(jv_copy(b));
    let mut ret: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
    if blen <= alen &&
           memcmp(jv_string_value(a) as *const libc::c_void,
                  jv_string_value(b) as *const libc::c_void,
                  blen as libc::c_ulong) == 0 as libc::c_int {
        ret = jv_true()
    } else { ret = jv_false() }
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_endswith(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint ||
           jv_get_kind(b) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
        return ret_error2(a, b,
                          jv_string(b"endswith() requires string inputs\x00"
                                        as *const u8 as *const libc::c_char))
    }
    let mut astr: *const libc::c_char = jv_string_value(a);
    let mut bstr: *const libc::c_char = jv_string_value(b);
    let mut alen: size_t = jv_string_length_bytes(jv_copy(a)) as size_t;
    let mut blen: size_t = jv_string_length_bytes(jv_copy(b)) as size_t;
    let mut ret: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
    if alen < blen ||
           memcmp(astr.offset(alen.wrapping_sub(blen) as isize) as
                      *const libc::c_void, bstr as *const libc::c_void, blen)
               != 0 as libc::c_int {
        ret = jv_false()
    } else { ret = jv_true() }
    jv_free(a);
    jv_free(b);
    return ret;
}
unsafe extern "C" fn f_ltrimstr(mut jq: *mut jq_state, mut input: jv,
                                mut left: jv) -> jv {
    if jv_get_kind(f_startswith(jq, jv_copy(input), jv_copy(left))) as
           libc::c_uint != JV_KIND_TRUE as libc::c_int as libc::c_uint {
        jv_free(left);
        return input
    }
    /*
   * FIXME It'd be better to share the suffix with the original input --
   * that we could do, we just can't share prefixes.
   */
    let mut prefixlen: libc::c_int = jv_string_length_bytes(left);
    let mut res: jv =
        jv_string_sized(jv_string_value(input).offset(prefixlen as isize),
                        jv_string_length_bytes(jv_copy(input)) - prefixlen);
    jv_free(input);
    return res;
}
unsafe extern "C" fn f_rtrimstr(mut jq: *mut jq_state, mut input: jv,
                                mut right: jv) -> jv {
    if jv_get_kind(f_endswith(jq, jv_copy(input), jv_copy(right))) as
           libc::c_uint == JV_KIND_TRUE as libc::c_int as libc::c_uint {
        let mut res: jv =
            jv_string_sized(jv_string_value(input),
                            jv_string_length_bytes(jv_copy(input)) -
                                jv_string_length_bytes(right));
        jv_free(input);
        return res
    }
    jv_free(right);
    return input;
}
unsafe extern "C" fn f_minus(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                             mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
           jv_get_kind(b) as libc::c_uint ==
               JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        let mut r: jv = jv_number(jv_number_value(a) - jv_number_value(b));
        jv_free(a);
        jv_free(b);
        return r
    } else if jv_get_kind(a) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
                  jv_get_kind(b) as libc::c_uint ==
                      JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        let mut out: jv = jv_array();
        let mut jv_len__: libc::c_int = jv_array_length(jv_copy(a));
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut jv_j__: libc::c_int = 1 as libc::c_int;
        while jv_j__ != 0 {
            let mut x: jv =
                jv{kind_flags: 0,
                   pad_: 0,
                   offset: 0,
                   size: 0,
                   u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
            while if i < jv_len__ {
                      x = jv_array_get(jv_copy(a), i);
                      1 as libc::c_int
                  } else { 0 as libc::c_int } != 0 {
                let mut include: libc::c_int = 1 as libc::c_int;
                let mut jv_len___0: libc::c_int = jv_array_length(jv_copy(b));
                let mut j: libc::c_int = 0 as libc::c_int;
                let mut jv_j___0: libc::c_int = 1 as libc::c_int;
                while jv_j___0 != 0 {
                    let mut y: jv =
                        jv{kind_flags: 0,
                           pad_: 0,
                           offset: 0,
                           size: 0,
                           u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
                    while if j < jv_len___0 {
                              y = jv_array_get(jv_copy(b), j);
                              1 as libc::c_int
                          } else { 0 as libc::c_int } != 0 {
                        if jv_equal(jv_copy(x), y) != 0 {
                            include = 0 as libc::c_int;
                            break ;
                        } else { j += 1 }
                    }
                    jv_j___0 = 0 as libc::c_int
                }
                if include != 0 { out = jv_array_append(out, jv_copy(x)) }
                jv_free(x);
                i += 1
            }
            jv_j__ = 0 as libc::c_int
        }
        jv_free(a);
        jv_free(b);
        return out
    } else {
        return type_error2(a, b,
                           b"cannot be subtracted\x00" as *const u8 as
                               *const libc::c_char)
    };
}
unsafe extern "C" fn f_multiply(mut jq: *mut jq_state, mut input: jv,
                                mut a: jv, mut b: jv) -> jv {
    let mut ak: jv_kind = jv_get_kind(a);
    let mut bk: jv_kind = jv_get_kind(b);
    jv_free(input);
    if ak as libc::c_uint == JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
           bk as libc::c_uint == JV_KIND_NUMBER as libc::c_int as libc::c_uint
       {
        let mut r: jv = jv_number(jv_number_value(a) * jv_number_value(b));
        jv_free(a);
        jv_free(b);
        return r
    } else if ak as libc::c_uint ==
                  JV_KIND_STRING as libc::c_int as libc::c_uint &&
                  bk as libc::c_uint ==
                      JV_KIND_NUMBER as libc::c_int as libc::c_uint ||
                  ak as libc::c_uint ==
                      JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
                      bk as libc::c_uint ==
                          JV_KIND_STRING as libc::c_int as libc::c_uint {
        let mut str: jv = a;
        let mut num: jv = b;
        if ak as libc::c_uint == JV_KIND_NUMBER as libc::c_int as libc::c_uint
           {
            str = b;
            num = a
        }
        let mut res: jv = jv_null();
        let mut n: libc::c_int = jv_number_value(num) as libc::c_int;
        if n > 0 as libc::c_int {
            let mut alen: size_t =
                jv_string_length_bytes(jv_copy(str)) as size_t;
            res =
                jv_string_empty(alen.wrapping_mul(n as libc::c_ulong) as
                                    libc::c_int);
            while n > 0 as libc::c_int {
                res =
                    jv_string_append_buf(res, jv_string_value(str),
                                         alen as libc::c_int);
                n -= 1
            }
        }
        jv_free(str);
        jv_free(num);
        return res
    } else if ak as libc::c_uint ==
                  JV_KIND_OBJECT as libc::c_int as libc::c_uint &&
                  bk as libc::c_uint ==
                      JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        return jv_object_merge_recursive(a, b)
    } else {
        return type_error2(a, b,
                           b"cannot be multiplied\x00" as *const u8 as
                               *const libc::c_char)
    };
}
unsafe extern "C" fn f_divide(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                              mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
           jv_get_kind(b) as libc::c_uint ==
               JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        if jv_number_value(b) == 0.0f64 {
            return type_error2(a, b,
                               b"cannot be divided because the divisor is zero\x00"
                                   as *const u8 as *const libc::c_char)
        }
        let mut r: jv = jv_number(jv_number_value(a) / jv_number_value(b));
        jv_free(a);
        jv_free(b);
        return r
    } else if jv_get_kind(a) as libc::c_uint ==
                  JV_KIND_STRING as libc::c_int as libc::c_uint &&
                  jv_get_kind(b) as libc::c_uint ==
                      JV_KIND_STRING as libc::c_int as libc::c_uint {
        return jv_string_split(a, b)
    } else {
        return type_error2(a, b,
                           b"cannot be divided\x00" as *const u8 as
                               *const libc::c_char)
    };
}
unsafe extern "C" fn f_mod(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                           mut b: jv) -> jv {
    jv_free(input);
    if jv_get_kind(a) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
           jv_get_kind(b) as libc::c_uint ==
               JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        if jv_number_value(b) as intmax_t == 0 as libc::c_int as libc::c_long
           {
            return type_error2(a, b,
                               b"cannot be divided (remainder) because the divisor is zero\x00"
                                   as *const u8 as *const libc::c_char)
        }
        let mut r: jv =
            jv_number((jv_number_value(a) as intmax_t %
                           jv_number_value(b) as intmax_t) as libc::c_double);
        jv_free(a);
        jv_free(b);
        return r
    } else {
        return type_error2(a, b,
                           b"cannot be divided (remainder)\x00" as *const u8
                               as *const libc::c_char)
    };
}
unsafe extern "C" fn f_equal(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                             mut b: jv) -> jv {
    jv_free(input);
    return jv_bool(jv_equal(a, b));
}
unsafe extern "C" fn f_notequal(mut jq: *mut jq_state, mut input: jv,
                                mut a: jv, mut b: jv) -> jv {
    jv_free(input);
    return jv_bool((jv_equal(a, b) == 0) as libc::c_int);
}
unsafe extern "C" fn order_cmp(mut input: jv, mut a: jv, mut b: jv,
                               mut op: cmp_op) -> jv {
    jv_free(input);
    let mut r: libc::c_int = jv_cmp(a, b);
    return jv_bool((op as libc::c_uint ==
                        CMP_OP_LESS as libc::c_int as libc::c_uint &&
                        r < 0 as libc::c_int ||
                        op as libc::c_uint ==
                            CMP_OP_LESSEQ as libc::c_int as libc::c_uint &&
                            r <= 0 as libc::c_int ||
                        op as libc::c_uint ==
                            CMP_OP_GREATEREQ as libc::c_int as libc::c_uint &&
                            r >= 0 as libc::c_int ||
                        op as libc::c_uint ==
                            CMP_OP_GREATER as libc::c_int as libc::c_uint &&
                            r > 0 as libc::c_int) as libc::c_int);
}
unsafe extern "C" fn f_less(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                            mut b: jv) -> jv {
    return order_cmp(input, a, b, CMP_OP_LESS);
}
unsafe extern "C" fn f_greater(mut jq: *mut jq_state, mut input: jv,
                               mut a: jv, mut b: jv) -> jv {
    return order_cmp(input, a, b, CMP_OP_GREATER);
}
unsafe extern "C" fn f_lesseq(mut jq: *mut jq_state, mut input: jv, mut a: jv,
                              mut b: jv) -> jv {
    return order_cmp(input, a, b, CMP_OP_LESSEQ);
}
unsafe extern "C" fn f_greatereq(mut jq: *mut jq_state, mut input: jv,
                                 mut a: jv, mut b: jv) -> jv {
    return order_cmp(input, a, b, CMP_OP_GREATEREQ);
}
unsafe extern "C" fn f_contains(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint == jv_get_kind(b) as libc::c_uint {
        return jv_bool(jv_contains(a, b))
    } else {
        return type_error2(a, b,
                           b"cannot have their containment checked\x00" as
                               *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn f_dump(mut jq: *mut jq_state, mut input: jv) -> jv {
    return jv_dump_string(input, 0 as libc::c_int);
}
unsafe extern "C" fn f_json_parse(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"only strings can be parsed\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut res: jv =
        jv_parse_sized(jv_string_value(input),
                       jv_string_length_bytes(jv_copy(input)));
    jv_free(input);
    return res;
}
unsafe extern "C" fn f_tonumber(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return input
    }
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        let mut parsed: jv = jv_parse(jv_string_value(input));
        if jv_is_valid(parsed) == 0 ||
               jv_get_kind(parsed) as libc::c_uint ==
                   JV_KIND_NUMBER as libc::c_int as libc::c_uint {
            jv_free(input);
            return parsed
        }
    }
    return type_error(input,
                      b"cannot be parsed as a number\x00" as *const u8 as
                          *const libc::c_char);
}
unsafe extern "C" fn f_length(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_number(jv_array_length(input) as libc::c_double)
    } else if jv_get_kind(input) as libc::c_uint ==
                  JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        return jv_number(jv_object_length(input) as libc::c_double)
    } else if jv_get_kind(input) as libc::c_uint ==
                  JV_KIND_STRING as libc::c_int as libc::c_uint {
        return jv_number(jv_string_length_codepoints(input) as libc::c_double)
    } else if jv_get_kind(input) as libc::c_uint ==
                  JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        let mut r: jv = jv_number(fabs(jv_number_value(input)));
        jv_free(input);
        return r
    } else if jv_get_kind(input) as libc::c_uint ==
                  JV_KIND_NULL as libc::c_int as libc::c_uint {
        jv_free(input);
        return jv_number(0 as libc::c_int as libc::c_double)
    } else {
        return type_error(input,
                          b"has no length\x00" as *const u8 as
                              *const libc::c_char)
    };
}
unsafe extern "C" fn f_tostring(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return input
    } else { return jv_dump_string(input, 0 as libc::c_int) };
}
unsafe extern "C" fn f_utf8bytelength(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return type_error(input,
                          b"only strings have UTF-8 byte length\x00" as
                              *const u8 as *const libc::c_char)
    }
    return jv_number(jv_string_length_bytes(input) as libc::c_double);
}
static mut BASE64_ENCODE_TABLE: [libc::c_uchar; 65] =
    unsafe {
        *::std::mem::transmute::<&[u8; 65],
                                 &[libc::c_uchar; 65]>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00")
    };
static mut BASE64_INVALID_ENTRY: libc::c_uchar =
    0xff as libc::c_int as libc::c_uchar;
static mut BASE64_DECODE_TABLE: [libc::c_uchar; 255] =
    [0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar, 62 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar, 63 as libc::c_int as libc::c_uchar,
     52 as libc::c_int as libc::c_uchar, 53 as libc::c_int as libc::c_uchar,
     54 as libc::c_int as libc::c_uchar, 55 as libc::c_int as libc::c_uchar,
     56 as libc::c_int as libc::c_uchar, 57 as libc::c_int as libc::c_uchar,
     58 as libc::c_int as libc::c_uchar, 59 as libc::c_int as libc::c_uchar,
     60 as libc::c_int as libc::c_uchar, 61 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar, 99 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     5 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar,
     7 as libc::c_int as libc::c_uchar, 8 as libc::c_int as libc::c_uchar,
     9 as libc::c_int as libc::c_uchar, 10 as libc::c_int as libc::c_uchar,
     11 as libc::c_int as libc::c_uchar, 12 as libc::c_int as libc::c_uchar,
     13 as libc::c_int as libc::c_uchar, 14 as libc::c_int as libc::c_uchar,
     15 as libc::c_int as libc::c_uchar, 16 as libc::c_int as libc::c_uchar,
     17 as libc::c_int as libc::c_uchar, 18 as libc::c_int as libc::c_uchar,
     19 as libc::c_int as libc::c_uchar, 20 as libc::c_int as libc::c_uchar,
     21 as libc::c_int as libc::c_uchar, 22 as libc::c_int as libc::c_uchar,
     23 as libc::c_int as libc::c_uchar, 24 as libc::c_int as libc::c_uchar,
     25 as libc::c_int as libc::c_uchar, 0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar, 26 as libc::c_int as libc::c_uchar,
     27 as libc::c_int as libc::c_uchar, 28 as libc::c_int as libc::c_uchar,
     29 as libc::c_int as libc::c_uchar, 30 as libc::c_int as libc::c_uchar,
     31 as libc::c_int as libc::c_uchar, 32 as libc::c_int as libc::c_uchar,
     33 as libc::c_int as libc::c_uchar, 34 as libc::c_int as libc::c_uchar,
     35 as libc::c_int as libc::c_uchar, 36 as libc::c_int as libc::c_uchar,
     37 as libc::c_int as libc::c_uchar, 38 as libc::c_int as libc::c_uchar,
     39 as libc::c_int as libc::c_uchar, 40 as libc::c_int as libc::c_uchar,
     41 as libc::c_int as libc::c_uchar, 42 as libc::c_int as libc::c_uchar,
     43 as libc::c_int as libc::c_uchar, 44 as libc::c_int as libc::c_uchar,
     45 as libc::c_int as libc::c_uchar, 46 as libc::c_int as libc::c_uchar,
     47 as libc::c_int as libc::c_uchar, 48 as libc::c_int as libc::c_uchar,
     49 as libc::c_int as libc::c_uchar, 50 as libc::c_int as libc::c_uchar,
     51 as libc::c_int as libc::c_uchar, 0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar];
unsafe extern "C" fn escape_string(mut input: jv,
                                   mut escapings: *const libc::c_char) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(input) == JV_KIND_STRING\x00" as *const u8
                          as *const libc::c_char,
                      b"src/builtin.c\x00" as *const u8 as
                          *const libc::c_char,
                      542 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"jv escape_string(jv, const char *)\x00")).as_ptr());
    };
    let mut lookup: [*const libc::c_char; 128] =
        [0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char];
    let mut p: *const libc::c_char = escapings;
    lookup[0 as libc::c_int as usize] =
        b"\\0\x00" as *const u8 as *const libc::c_char;
    while *p != 0 {
        lookup[*p as libc::c_int as usize] =
            p.offset(1 as libc::c_int as isize);
        p = p.offset(1);
        p = p.offset(strlen(p) as isize);
        p = p.offset(1)
    }
    let mut ret: jv = jv_string(b"\x00" as *const u8 as *const libc::c_char);
    let mut i: *const libc::c_char = jv_string_value(input);
    let mut end: *const libc::c_char =
        i.offset(jv_string_length_bytes(jv_copy(input)) as isize);
    let mut cstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0 as libc::c_int;
    loop  {
        cstart = i;
        i = jvp_utf8_next(cstart, end, &mut c);
        if i.is_null() { break ; }
        if c < 128 as libc::c_int && !lookup[c as usize].is_null() {
            ret = jv_string_append_str(ret, lookup[c as usize])
        } else {
            ret =
                jv_string_append_buf(ret, cstart,
                                     i.wrapping_offset_from(cstart) as
                                         libc::c_long as libc::c_int)
        }
    }
    jv_free(input);
    return ret;
}
unsafe extern "C" fn f_keys(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint ||
           jv_get_kind(input) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_keys(input)
    } else {
        return type_error(input,
                          b"has no keys\x00" as *const u8 as
                              *const libc::c_char)
    };
}
unsafe extern "C" fn f_keys_unsorted(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint ||
           jv_get_kind(input) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_keys_unsorted(input)
    } else {
        return type_error(input,
                          b"has no keys\x00" as *const u8 as
                              *const libc::c_char)
    };
}
unsafe extern "C" fn f_sort(mut jq: *mut jq_state, mut input: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_sort(input, jv_copy(input))
    } else {
        return type_error(input,
                          b"cannot be sorted, as it is not an array\x00" as
                              *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn f_sort_by_impl(mut jq: *mut jq_state, mut input: jv,
                                    mut keys: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
           jv_get_kind(keys) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
           jv_array_length(jv_copy(input)) == jv_array_length(jv_copy(keys)) {
        return jv_sort(input, keys)
    } else {
        return type_error2(input, keys,
                           b"cannot be sorted, as they are not both arrays\x00"
                               as *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn f_group_by_impl(mut jq: *mut jq_state, mut input: jv,
                                     mut keys: jv) -> jv {
    if jv_get_kind(input) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
           jv_get_kind(keys) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
           jv_array_length(jv_copy(input)) == jv_array_length(jv_copy(keys)) {
        return jv_group(input, keys)
    } else {
        return type_error2(input, keys,
                           b"cannot be sorted, as they are not both arrays\x00"
                               as *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn f_match_name_iter(mut name: *const OnigUChar,
                                       mut name_end: *const OnigUChar,
                                       mut ngroups: libc::c_int,
                                       mut groups: *mut libc::c_int,
                                       mut reg: *mut regex_t,
                                       mut arg: *mut libc::c_void)
 -> libc::c_int {
    let mut captures: jv = *(arg as *mut jv);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < ngroups {
        let mut cap: jv =
            jv_array_get(jv_copy(captures),
                         *groups.offset(i as isize) - 1 as libc::c_int);
        if jv_get_kind(cap) as libc::c_uint ==
               JV_KIND_OBJECT as libc::c_int as libc::c_uint {
            cap =
                jv_object_set(cap,
                              jv_string(b"name\x00" as *const u8 as
                                            *const libc::c_char),
                              jv_string_sized(name as *const libc::c_char,
                                              name_end.wrapping_offset_from(name)
                                                  as libc::c_long as
                                                  libc::c_int));
            captures =
                jv_array_set(captures,
                             *groups.offset(i as isize) - 1 as libc::c_int,
                             cap)
        } else { jv_free(cap); }
        i += 1
    }
    *(arg as *mut jv) = captures;
    return 0 as libc::c_int;
}
unsafe extern "C" fn f_match(mut jq: *mut jq_state, mut input: jv,
                             mut regex: jv, mut modifiers: jv,
                             mut testmode: jv) -> jv {
    let mut test: libc::c_int = jv_equal(testmode, jv_true());
    let mut result: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
    let mut onigret: libc::c_int = 0;
    let mut global: libc::c_int = 0 as libc::c_int;
    let mut reg: *mut regex_t = 0 as *mut regex_t;
    let mut einfo: OnigErrorInfo =
        OnigErrorInfo{enc: 0 as *mut OnigEncodingType,
                      par: 0 as *mut OnigUChar,
                      par_end: 0 as *mut OnigUChar,};
    let mut region: *mut OnigRegion = 0 as *mut OnigRegion;
    if jv_get_kind(input) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        jv_free(regex);
        jv_free(modifiers);
        return type_error(input,
                          b"cannot be matched, as it is not a string\x00" as
                              *const u8 as *const libc::c_char)
    }
    if jv_get_kind(regex) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        jv_free(input);
        jv_free(modifiers);
        return type_error(regex,
                          b"is not a string\x00" as *const u8 as
                              *const libc::c_char)
    }
    let mut options: OnigOptionType =
        ((((((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                 << 1 as libc::c_int) << 1 as libc::c_int) <<
               1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int) <<
            1 as libc::c_int;
    if jv_get_kind(modifiers) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        let mut modarray: jv = jv_string_explode(jv_copy(modifiers));
        let mut jv_len__: libc::c_int = jv_array_length(jv_copy(modarray));
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut jv_j__: libc::c_int = 1 as libc::c_int;
        while jv_j__ != 0 {
            let mut mod_0: jv =
                jv{kind_flags: 0,
                   pad_: 0,
                   offset: 0,
                   size: 0,
                   u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
            while if i < jv_len__ {
                      mod_0 = jv_array_get(jv_copy(modarray), i);
                      1 as libc::c_int
                  } else { 0 as libc::c_int } != 0 {
                match jv_number_value(mod_0) as libc::c_int {
                    103 => { global = 1 as libc::c_int }
                    105 => { options |= 1 as libc::c_uint }
                    120 => {
                        options |= (1 as libc::c_uint) << 1 as libc::c_int
                    }
                    109 => {
                        options |=
                            ((1 as libc::c_uint) << 1 as libc::c_int) <<
                                1 as libc::c_int
                    }
                    115 => {
                        options |=
                            (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                 1 as libc::c_int) << 1 as libc::c_int
                    }
                    112 => {
                        options |=
                            ((1 as libc::c_uint) << 1 as libc::c_int) <<
                                1 as libc::c_int |
                                (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                     1 as libc::c_int) << 1 as libc::c_int
                    }
                    108 => {
                        options |=
                            ((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                  1 as libc::c_int) << 1 as libc::c_int) <<
                                1 as libc::c_int
                    }
                    110 => {
                        options |=
                            (((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                   1 as libc::c_int) << 1 as libc::c_int) <<
                                 1 as libc::c_int) << 1 as libc::c_int
                    }
                    _ => {
                        jv_free(input);
                        jv_free(regex);
                        jv_free(modarray);
                        return jv_invalid_with_msg(jv_string_concat(modifiers,
                                                                    jv_string(b" is not a valid modifier string\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char)))
                    }
                }
                i += 1
            }
            jv_j__ = 0 as libc::c_int
        }
        jv_free(modarray);
    } else if jv_get_kind(modifiers) as libc::c_uint !=
                  JV_KIND_NULL as libc::c_int as libc::c_uint {
        // If it isn't a string or null, then it is the wrong type...
        jv_free(input);
        jv_free(regex);
        return type_error(modifiers,
                          b"is not a string\x00" as *const u8 as
                              *const libc::c_char)
    }
    jv_free(modifiers);
    onigret =
        onig_new(&mut reg, jv_string_value(regex) as *const OnigUChar,
                 jv_string_value(regex).offset(jv_string_length_bytes(jv_copy(regex))
                                                   as isize) as
                     *const OnigUChar, options, &mut OnigEncodingUTF8,
                 &mut OnigSyntaxPerl_NG, &mut einfo);
    if onigret != 0 as libc::c_int {
        let mut ebuf: [OnigUChar; 90] = [0; 90];
        onig_error_code_to_str(ebuf.as_mut_ptr(), onigret, &mut einfo);
        jv_free(input);
        jv_free(regex);
        return jv_invalid_with_msg(jv_string_concat(jv_string(b"Regex failure: \x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char),
                                                    jv_string(ebuf.as_mut_ptr()
                                                                  as
                                                                  *mut libc::c_char)))
    }
    result = if test != 0 { jv_false() } else { jv_array() };
    let mut input_string: *const libc::c_char = jv_string_value(input);
    let mut start: *const OnigUChar =
        jv_string_value(input) as *const OnigUChar;
    let length: libc::c_ulong =
        jv_string_length_bytes(jv_copy(input)) as libc::c_ulong;
    let mut end: *const OnigUChar = start.offset(length as isize);
    region = onig_region_new();
    loop  {
        onigret =
            onig_search(reg, jv_string_value(input) as *const OnigUChar, end,
                        start, end, region, 0 as libc::c_uint);
        if onigret >= 0 as libc::c_int {
            if test != 0 {
                result = jv_true();
                break ;
            } else {
                // Zero-width match
                if *(*region).end.offset(0 as libc::c_int as isize) ==
                       *(*region).beg.offset(0 as libc::c_int as isize) {
                    let mut idx: libc::c_ulong = 0;
                    let mut fr: *const libc::c_char = input_string;
                    idx = 0 as libc::c_int as libc::c_ulong;
                    while fr <
                              input_string.offset(*(*region).beg.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                      as isize) {
                        fr = fr.offset(jvp_utf8_decode_length(*fr) as isize);
                        idx = idx.wrapping_add(1)
                    }
                    let mut match_0: jv =
                        jv_object_set(jv_object(),
                                      jv_string(b"offset\x00" as *const u8 as
                                                    *const libc::c_char),
                                      jv_number(idx as libc::c_double));
                    match_0 =
                        jv_object_set(match_0,
                                      jv_string(b"length\x00" as *const u8 as
                                                    *const libc::c_char),
                                      jv_number(0 as libc::c_int as
                                                    libc::c_double));
                    match_0 =
                        jv_object_set(match_0,
                                      jv_string(b"string\x00" as *const u8 as
                                                    *const libc::c_char),
                                      jv_string(b"\x00" as *const u8 as
                                                    *const libc::c_char));
                    match_0 =
                        jv_object_set(match_0,
                                      jv_string(b"captures\x00" as *const u8
                                                    as *const libc::c_char),
                                      jv_array());
                    result = jv_array_append(result, match_0);
                    start = start.offset(1 as libc::c_int as isize)
                } else {
                    let mut idx_0: libc::c_ulong = 0;
                    let mut len: libc::c_ulong = 0;
                    let mut fr_0: *const libc::c_char = input_string;
                    len = 0 as libc::c_int as libc::c_ulong;
                    idx_0 = len;
                    while fr_0 <
                              input_string.offset(*(*region).end.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                      as isize) {
                        if fr_0 ==
                               input_string.offset(*(*region).beg.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                       as isize) {
                            idx_0 = len;
                            len = 0 as libc::c_int as libc::c_ulong
                        }
                        fr_0 =
                            fr_0.offset(jvp_utf8_decode_length(*fr_0) as
                                            isize);
                        len = len.wrapping_add(1)
                    }
                    let mut match_1: jv =
                        jv_object_set(jv_object(),
                                      jv_string(b"offset\x00" as *const u8 as
                                                    *const libc::c_char),
                                      jv_number(idx_0 as libc::c_double));
                    let mut blen: libc::c_ulong =
                        (*(*region).end.offset(0 as libc::c_int as isize) -
                             *(*region).beg.offset(0 as libc::c_int as isize))
                            as libc::c_ulong;
                    match_1 =
                        jv_object_set(match_1,
                                      jv_string(b"length\x00" as *const u8 as
                                                    *const libc::c_char),
                                      jv_number(len as libc::c_double));
                    match_1 =
                        jv_object_set(match_1,
                                      jv_string(b"string\x00" as *const u8 as
                                                    *const libc::c_char),
                                      jv_string_sized(input_string.offset(*(*region).beg.offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                                                              as
                                                                              isize),
                                                      blen as libc::c_int));
                    let mut captures: jv = jv_array();
                    let mut i_0: libc::c_int = 1 as libc::c_int;
                    while i_0 < (*region).num_regs {
                        // Empty capture.
                        if *(*region).beg.offset(i_0 as isize) ==
                               *(*region).end.offset(i_0 as isize) {
                            // Didn't match.
                            let mut cap: jv =
                                jv{kind_flags: 0,
                                   pad_: 0,
                                   offset: 0,
                                   size: 0,
                                   u:
                                       C2RustUnnamed_0{ptr:
                                                           0 as
                                                               *mut jv_refcnt,},};
                            if *(*region).beg.offset(i_0 as isize) ==
                                   -(1 as libc::c_int) {
                                cap =
                                    jv_object_set(jv_object(),
                                                  jv_string(b"offset\x00" as
                                                                *const u8 as
                                                                *const libc::c_char),
                                                  jv_number(-(1 as
                                                                  libc::c_int)
                                                                as
                                                                libc::c_double));
                                cap =
                                    jv_object_set(cap,
                                                  jv_string(b"string\x00" as
                                                                *const u8 as
                                                                *const libc::c_char),
                                                  jv_null())
                            } else {
                                fr_0 = input_string;
                                idx_0 = 0 as libc::c_int as libc::c_ulong;
                                while fr_0 <
                                          input_string.offset(*(*region).beg.offset(i_0
                                                                                        as
                                                                                        isize)
                                                                  as isize) {
                                    fr_0 =
                                        fr_0.offset(jvp_utf8_decode_length(*fr_0)
                                                        as isize);
                                    idx_0 = idx_0.wrapping_add(1)
                                }
                                cap =
                                    jv_object_set(jv_object(),
                                                  jv_string(b"offset\x00" as
                                                                *const u8 as
                                                                *const libc::c_char),
                                                  jv_number(idx_0 as
                                                                libc::c_double));
                                cap =
                                    jv_object_set(cap,
                                                  jv_string(b"string\x00" as
                                                                *const u8 as
                                                                *const libc::c_char),
                                                  jv_string(b"\x00" as
                                                                *const u8 as
                                                                *const libc::c_char))
                            }
                            cap =
                                jv_object_set(cap,
                                              jv_string(b"length\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                              jv_number(0 as libc::c_int as
                                                            libc::c_double));
                            cap =
                                jv_object_set(cap,
                                              jv_string(b"name\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                              jv_null());
                            captures = jv_array_append(captures, cap)
                        } else {
                            fr_0 = input_string;
                            len = 0 as libc::c_int as libc::c_ulong;
                            idx_0 = len;
                            while fr_0 <
                                      input_string.offset(*(*region).end.offset(i_0
                                                                                    as
                                                                                    isize)
                                                              as isize) {
                                if fr_0 ==
                                       input_string.offset(*(*region).beg.offset(i_0
                                                                                     as
                                                                                     isize)
                                                               as isize) {
                                    idx_0 = len;
                                    len = 0 as libc::c_int as libc::c_ulong
                                }
                                fr_0 =
                                    fr_0.offset(jvp_utf8_decode_length(*fr_0)
                                                    as isize);
                                len = len.wrapping_add(1)
                            }
                            blen =
                                (*(*region).end.offset(i_0 as isize) -
                                     *(*region).beg.offset(i_0 as isize)) as
                                    libc::c_ulong;
                            let mut cap_0: jv =
                                jv_object_set(jv_object(),
                                              jv_string(b"offset\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                              jv_number(idx_0 as
                                                            libc::c_double));
                            cap_0 =
                                jv_object_set(cap_0,
                                              jv_string(b"length\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                              jv_number(len as
                                                            libc::c_double));
                            cap_0 =
                                jv_object_set(cap_0,
                                              jv_string(b"string\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                              jv_string_sized(input_string.offset(*(*region).beg.offset(i_0
                                                                                                            as
                                                                                                            isize)
                                                                                      as
                                                                                      isize),
                                                              blen as
                                                                  libc::c_int));
                            cap_0 =
                                jv_object_set(cap_0,
                                              jv_string(b"name\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                              jv_null());
                            captures = jv_array_append(captures, cap_0)
                        }
                        i_0 += 1
                    }
                    onig_foreach_name(reg,
                                      Some(f_match_name_iter as
                                               unsafe extern "C" fn(_:
                                                                        *const OnigUChar,
                                                                    _:
                                                                        *const OnigUChar,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut regex_t,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> libc::c_int),
                                      &mut captures as *mut jv as
                                          *mut libc::c_void);
                    match_1 =
                        jv_object_set(match_1,
                                      jv_string(b"captures\x00" as *const u8
                                                    as *const libc::c_char),
                                      captures);
                    result = jv_array_append(result, match_1);
                    start =
                        input_string.offset(*(*region).end.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                                as isize) as *const OnigUChar;
                    onig_region_free(region, 0 as libc::c_int);
                }
                if !(global != 0 && start != end) { break ; }
            }
        } else {
            if onigret == -(1 as libc::c_int) { break ; }
            let mut ebuf_0: [OnigUChar; 90] = [0; 90];
            onig_error_code_to_str(ebuf_0.as_mut_ptr(), onigret, &mut einfo);
            jv_free(result);
            result =
                jv_invalid_with_msg(jv_string_concat(jv_string(b"Regex failure: \x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char),
                                                     jv_string(ebuf_0.as_mut_ptr()
                                                                   as
                                                                   *mut libc::c_char)));
            break ;
        }
    }
    onig_region_free(region, 1 as libc::c_int);
    region = 0 as *mut OnigRegion;
    if !region.is_null() { onig_region_free(region, 1 as libc::c_int); }
    onig_free(reg);
    jv_free(input);
    jv_free(regex);
    return result;
}
/* !HAVE_LIBONIG */
/* HAVE_LIBONIG */
unsafe extern "C" fn minmax_by(mut values: jv, mut keys: jv,
                               mut is_min: libc::c_int) -> jv {
    if jv_get_kind(values) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return type_error2(values, keys,
                           b"cannot be iterated over\x00" as *const u8 as
                               *const libc::c_char)
    }
    if jv_get_kind(keys) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return type_error2(values, keys,
                           b"cannot be iterated over\x00" as *const u8 as
                               *const libc::c_char)
    }
    if jv_array_length(jv_copy(values)) != jv_array_length(jv_copy(keys)) {
        return type_error2(values, keys,
                           b"have wrong length\x00" as *const u8 as
                               *const libc::c_char)
    }
    if jv_array_length(jv_copy(values)) == 0 as libc::c_int {
        jv_free(values);
        jv_free(keys);
        return jv_null()
    }
    let mut ret: jv = jv_array_get(jv_copy(values), 0 as libc::c_int);
    let mut retkey: jv = jv_array_get(jv_copy(keys), 0 as libc::c_int);
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < jv_array_length(jv_copy(values)) {
        let mut item: jv = jv_array_get(jv_copy(keys), i);
        let mut cmp: libc::c_int = jv_cmp(jv_copy(item), jv_copy(retkey));
        if (cmp < 0 as libc::c_int) as libc::c_int ==
               (is_min == 1 as libc::c_int) as libc::c_int {
            jv_free(retkey);
            retkey = item;
            jv_free(ret);
            ret = jv_array_get(jv_copy(values), i)
        } else { jv_free(item); }
        i += 1
    }
    jv_free(values);
    jv_free(keys);
    jv_free(retkey);
    return ret;
}
unsafe extern "C" fn f_min(mut jq: *mut jq_state, mut x: jv) -> jv {
    return minmax_by(x, jv_copy(x), 1 as libc::c_int);
}
unsafe extern "C" fn f_max(mut jq: *mut jq_state, mut x: jv) -> jv {
    return minmax_by(x, jv_copy(x), 0 as libc::c_int);
}
unsafe extern "C" fn f_min_by_impl(mut jq: *mut jq_state, mut x: jv,
                                   mut y: jv) -> jv {
    return minmax_by(x, y, 1 as libc::c_int);
}
unsafe extern "C" fn f_max_by_impl(mut jq: *mut jq_state, mut x: jv,
                                   mut y: jv) -> jv {
    return minmax_by(x, y, 0 as libc::c_int);
}
unsafe extern "C" fn f_type(mut jq: *mut jq_state, mut input: jv) -> jv {
    let mut out: jv = jv_string(jv_kind_name(jv_get_kind(input)));
    jv_free(input);
    return out;
}
unsafe extern "C" fn f_isinfinite(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    let mut k: jv_kind = jv_get_kind(input);
    if k as libc::c_uint != JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(input);
        return jv_false()
    }
    let mut n: libc::c_double = jv_number_value(input);
    jv_free(input);
    return if if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                     ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
                  __isinff(n as libc::c_float)
              } else if ::std::mem::size_of::<libc::c_double>() as
                            libc::c_ulong ==
                            ::std::mem::size_of::<libc::c_double>() as
                                libc::c_ulong {
                  __isinf(n)
              } else { __isinfl(f128::f128::new(n)) } != 0 {
               jv_true()
           } else { jv_false() };
}
unsafe extern "C" fn f_isnan(mut jq: *mut jq_state, mut input: jv) -> jv {
    let mut k: jv_kind = jv_get_kind(input);
    if k as libc::c_uint != JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(input);
        return jv_false()
    }
    let mut n: libc::c_double = jv_number_value(input);
    jv_free(input);
    return if if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                     ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
                  __isnanf(n as libc::c_float)
              } else if ::std::mem::size_of::<libc::c_double>() as
                            libc::c_ulong ==
                            ::std::mem::size_of::<libc::c_double>() as
                                libc::c_ulong {
                  __isnan(n)
              } else { __isnanl(f128::f128::new(n)) } != 0 {
               jv_true()
           } else { jv_false() };
}
unsafe extern "C" fn f_isnormal(mut jq: *mut jq_state, mut input: jv) -> jv {
    let mut k: jv_kind = jv_get_kind(input);
    if k as libc::c_uint != JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(input);
        return jv_false()
    }
    let mut n: libc::c_double = jv_number_value(input);
    jv_free(input);
    return if (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                      ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                  {
                   __fpclassifyf(n as libc::c_float)
               } else {
                   (if ::std::mem::size_of::<libc::c_double>() as
                           libc::c_ulong ==
                           ::std::mem::size_of::<libc::c_double>() as
                               libc::c_ulong {
                        __fpclassify(n)
                    } else { __fpclassifyl(f128::f128::new(n)) })
               }) == 4 as libc::c_int {
               jv_true()
           } else { jv_false() };
}
unsafe extern "C" fn f_infinite(mut jq: *mut jq_state, mut input: jv) -> jv {
    jv_free(input);
    return jv_number(::std::f32::INFINITY as libc::c_double);
}
unsafe extern "C" fn f_nan(mut jq: *mut jq_state, mut input: jv) -> jv {
    jv_free(input);
    return jv_number(::std::f32::NAN as libc::c_double);
}
unsafe extern "C" fn f_error(mut jq: *mut jq_state, mut input: jv) -> jv {
    return jv_invalid_with_msg(input);
}
unsafe extern "C" fn f_halt(mut jq: *mut jq_state, mut input: jv) -> jv {
    jv_free(input);
    jq_halt(jq, jv_invalid(), jv_invalid());
    return jv_true();
}
unsafe extern "C" fn f_halt_error(mut jq: *mut jq_state, mut input: jv,
                                  mut a: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return type_error(input,
                          b"halt_error/1: number required\x00" as *const u8 as
                              *const libc::c_char)
    }
    jq_halt(jq, a, input);
    return jv_true();
}
unsafe extern "C" fn f_get_search_list(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    jv_free(input);
    return jq_get_lib_dirs(jq);
}
unsafe extern "C" fn f_get_prog_origin(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    jv_free(input);
    return jq_get_prog_origin(jq);
}
unsafe extern "C" fn f_get_jq_origin(mut jq: *mut jq_state, mut input: jv)
 -> jv {
    jv_free(input);
    return jq_get_jq_origin(jq);
}
unsafe extern "C" fn f_string_split(mut jq: *mut jq_state, mut a: jv,
                                    mut b: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint ||
           jv_get_kind(b) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
        return ret_error2(a, b,
                          jv_string(b"split input and separator must be strings\x00"
                                        as *const u8 as *const libc::c_char))
    }
    return jv_string_split(a, b);
}
unsafe extern "C" fn f_string_explode(mut jq: *mut jq_state, mut a: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return ret_error(a,
                         jv_string(b"explode input must be a string\x00" as
                                       *const u8 as *const libc::c_char))
    }
    return jv_string_explode(a);
}
unsafe extern "C" fn f_string_indexes(mut jq: *mut jq_state, mut a: jv,
                                      mut b: jv) -> jv {
    return jv_string_indexes(a, b);
}
unsafe extern "C" fn f_string_implode(mut jq: *mut jq_state, mut a: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return ret_error(a,
                         jv_string(b"implode input must be an array\x00" as
                                       *const u8 as *const libc::c_char))
    }
    return jv_string_implode(a);
}
unsafe extern "C" fn f_setpath(mut jq: *mut jq_state, mut a: jv, mut b: jv,
                               mut c: jv) -> jv {
    return jv_setpath(a, b, c);
}
unsafe extern "C" fn f_getpath(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    return _jq_path_append(jq, a, b, jv_getpath(jv_copy(a), jv_copy(b)));
}
unsafe extern "C" fn f_delpaths(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    return jv_delpaths(a, b);
}
unsafe extern "C" fn f_has(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    return jv_has(a, b);
}
unsafe extern "C" fn f_modulemeta(mut jq: *mut jq_state, mut a: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return ret_error(a,
                         jv_string(b"modulemeta input module name must be a string\x00"
                                       as *const u8 as *const libc::c_char))
    }
    return load_module_meta(jq, a);
}
unsafe extern "C" fn f_input(mut jq: *mut jq_state, mut input: jv) -> jv {
    jv_free(input);
    let mut cb: jq_input_cb = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    jq_get_input_cb(jq, &mut cb, &mut data);
    if cb.is_none() {
        return jv_invalid_with_msg(jv_string(b"break\x00" as *const u8 as
                                                 *const libc::c_char))
    }
    let mut v: jv = cb.expect("non-null function pointer")(jq, data);
    if jv_is_valid(v) != 0 || jv_invalid_has_msg(jv_copy(v)) != 0 { return v }
    return jv_invalid_with_msg(jv_string(b"break\x00" as *const u8 as
                                             *const libc::c_char));
}
unsafe extern "C" fn f_debug(mut jq: *mut jq_state, mut input: jv) -> jv {
    let mut cb: jq_msg_cb = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    jq_get_debug_cb(jq, &mut cb, &mut data);
    if cb.is_some() {
        cb.expect("non-null function pointer")(data, jv_copy(input));
    }
    return input;
}
unsafe extern "C" fn f_stderr(mut jq: *mut jq_state, mut input: jv) -> jv {
    jv_dumpf(jv_copy(input), stderr, 0 as libc::c_int);
    return input;
}
unsafe extern "C" fn tm2jv(mut tm: *mut tm) -> jv {
    return jv_array_append(jv_array_append(jv_array_append(jv_array_append(jv_array_append(jv_array_append(jv_array_append(jv_array_append(jv_array(),
                                                                                                                                           jv_number(((*tm).tm_year
                                                                                                                                                          +
                                                                                                                                                          1900
                                                                                                                                                              as
                                                                                                                                                              libc::c_int)
                                                                                                                                                         as
                                                                                                                                                         libc::c_double)),
                                                                                                                           jv_number((*tm).tm_mon
                                                                                                                                         as
                                                                                                                                         libc::c_double)),
                                                                                                           jv_number((*tm).tm_mday
                                                                                                                         as
                                                                                                                         libc::c_double)),
                                                                                           jv_number((*tm).tm_hour
                                                                                                         as
                                                                                                         libc::c_double)),
                                                                           jv_number((*tm).tm_min
                                                                                         as
                                                                                         libc::c_double)),
                                                           jv_number((*tm).tm_sec
                                                                         as
                                                                         libc::c_double)),
                                           jv_number((*tm).tm_wday as
                                                         libc::c_double)),
                           jv_number((*tm).tm_yday as libc::c_double));
}
/*
 * mktime() has side-effects and anyways, returns time in the local
 * timezone, not UTC.  We want timegm(), which isn't standard.
 *
 * To make things worse, mktime() tells you what the timezone
 * adjustment is, but you have to #define _BSD_SOURCE to get this
 * field of struct tm on some systems.
 *
 * This is all to blame on POSIX, of course.
 *
 * Our wrapper tries to use timegm() if available, or mktime() and
 * correct for its side-effects if possible.
 *
 * Returns (time_t)-2 if mktime()'s side-effects cannot be corrected.
 */
unsafe extern "C" fn my_mktime(mut tm: *mut tm) -> time_t {
    return timegm(tm);
}
/* Compute and set tm_wday */
unsafe extern "C" fn set_tm_wday(mut tm: *mut tm) {
    /*
   * https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Gauss.27s_algorithm
   * https://cs.uwaterloo.ca/~alopez-o/math-faq/node73.html
   *
   * Tested with dates from 1900-01-01 through 2100-01-01.  This
   * algorithm produces the wrong day-of-the-week number for dates in
   * the range 1900-01-01..1900-02-28, and for 2100-01-01..2100-02-28.
   * Since this is only needed on OS X and *BSD, we might just document
   * this.
   */
    let mut century: libc::c_int =
        (1900 as libc::c_int + (*tm).tm_year) / 100 as libc::c_int;
    let mut year: libc::c_int =
        (1900 as libc::c_int + (*tm).tm_year) % 100 as libc::c_int;
    if (*tm).tm_mon < 2 as libc::c_int { year -= 1 }
    /*
   * The month value in the wday computation below is shifted so that
   * March is 1, April is 2, .., January is 11, and February is 12.
   */
    let mut mon: libc::c_int = (*tm).tm_mon - 1 as libc::c_int;
    if mon < 1 as libc::c_int { mon += 12 as libc::c_int }
    let mut wday: libc::c_int =
        ((*tm).tm_mday +
             floor(2.6f64 * mon as libc::c_double - 0.2f64) as libc::c_int +
             year + floor(year as libc::c_double / 4.0f64) as libc::c_int +
             floor(century as libc::c_double / 4.0f64) as libc::c_int -
             2 as libc::c_int * century) % 7 as libc::c_int;
    if wday < 0 as libc::c_int { wday += 7 as libc::c_int }
    (*tm).tm_wday = wday;
}
/*
 * Compute and set tm_yday.
 *
 */
unsafe extern "C" fn set_tm_yday(mut tm: *mut tm) {
    static mut d: [libc::c_int; 12] =
        [0 as libc::c_int, 31 as libc::c_int, 59 as libc::c_int,
         90 as libc::c_int, 120 as libc::c_int, 151 as libc::c_int,
         181 as libc::c_int, 212 as libc::c_int, 243 as libc::c_int,
         273 as libc::c_int, 304 as libc::c_int, 334 as libc::c_int];
    let mut mon: libc::c_int = (*tm).tm_mon;
    let mut year: libc::c_int = 1900 as libc::c_int + (*tm).tm_year;
    let mut leap_day: libc::c_int = 0 as libc::c_int;
    if (*tm).tm_mon > 1 as libc::c_int &&
           (year % 4 as libc::c_int == 0 as libc::c_int &&
                year % 100 as libc::c_int != 0 as libc::c_int ||
                year % 400 as libc::c_int == 0 as libc::c_int) {
        leap_day = 1 as libc::c_int
    }
    /* Bound check index into d[] */
    if mon < 0 as libc::c_int { mon = -mon } // sentinel
    if mon > 11 as libc::c_int { mon %= 12 as libc::c_int } // sentinel
    let mut yday: libc::c_int =
        d[mon as usize] + leap_day + (*tm).tm_mday - 1 as libc::c_int;
    if yday == (*tm).tm_yday || (*tm).tm_yday == 367 as libc::c_int {
    } else {
        __assert_fail(b"yday == tm->tm_yday || tm->tm_yday == 367\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/builtin.c\x00" as *const u8 as
                          *const libc::c_char,
                      1363 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void set_tm_yday(struct tm *)\x00")).as_ptr());
    };
    (*tm).tm_yday = yday;
}
unsafe extern "C" fn f_strptime(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_STRING as libc::c_int as libc::c_uint ||
           jv_get_kind(b) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
        return ret_error2(a, b,
                          jv_string(b"strptime/1 requires string inputs and arguments\x00"
                                        as *const u8 as *const libc::c_char))
    }
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    memset(&mut tm as *mut tm as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<tm>() as libc::c_ulong);
    tm.tm_wday = 8 as libc::c_int;
    tm.tm_yday = 367 as libc::c_int;
    let mut input: *const libc::c_char = jv_string_value(a);
    let mut fmt: *const libc::c_char = jv_string_value(b);
    let mut end: *const libc::c_char = strptime(input, fmt, &mut tm);
    if end.is_null() ||
           *end as libc::c_int != '\u{0}' as i32 &&
               *(*__ctype_b_loc()).offset(*end as libc::c_int as isize) as
                   libc::c_int &
                   _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                   0 {
        return ret_error2(a, b,
                          jv_string_fmt(b"date \"%s\" does not match format \"%s\"\x00"
                                            as *const u8 as
                                            *const libc::c_char, input, fmt))
    }
    jv_free(b);
    /*
   * This is OS X or some *BSD whose strptime() is just not that
   * helpful!
   *
   * We don't know that the format string did involve parsing a
   * year, or a month (if tm->tm_mon == 0).  But with our invalid
   * day-of-week and day-of-year sentinel checks above, the worst
   * this can do is produce garbage.
   */
    if tm.tm_wday == 8 as libc::c_int && tm.tm_mday != 0 as libc::c_int &&
           tm.tm_mon >= 0 as libc::c_int && tm.tm_mon <= 11 as libc::c_int {
        set_tm_wday(&mut tm); // must come after `*end` because `end` is a pointer into `a`'s string
    }
    if tm.tm_yday == 367 as libc::c_int && tm.tm_mday != 0 as libc::c_int &&
           tm.tm_mon >= 0 as libc::c_int && tm.tm_mon <= 11 as libc::c_int {
        set_tm_yday(&mut tm);
    }
    let mut r: jv = tm2jv(&mut tm);
    if *end as libc::c_int != '\u{0}' as i32 {
        r = jv_array_append(r, jv_string(end))
    }
    jv_free(a);
    return r;
}
unsafe extern "C" fn jv2tm(mut a: jv, mut tm: *mut tm) -> libc::c_int {
    memset(tm as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<tm>() as libc::c_ulong);
    let mut n: jv = jv_array_get(jv_copy(a), 0 as libc::c_int);
    if jv_get_kind(n) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_year = jv_number_value(n) as libc::c_int;
    jv_free(n);
    (*tm).tm_year -= 1900 as libc::c_int;
    let mut n_0: jv = jv_array_get(jv_copy(a), 1 as libc::c_int);
    if jv_get_kind(n_0) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_mon = jv_number_value(n_0) as libc::c_int;
    jv_free(n_0);
    let mut n_1: jv = jv_array_get(jv_copy(a), 2 as libc::c_int);
    if jv_get_kind(n_1) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_mday = jv_number_value(n_1) as libc::c_int;
    jv_free(n_1);
    let mut n_2: jv = jv_array_get(jv_copy(a), 3 as libc::c_int);
    if jv_get_kind(n_2) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_hour = jv_number_value(n_2) as libc::c_int;
    jv_free(n_2);
    let mut n_3: jv = jv_array_get(jv_copy(a), 4 as libc::c_int);
    if jv_get_kind(n_3) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_min = jv_number_value(n_3) as libc::c_int;
    jv_free(n_3);
    let mut n_4: jv = jv_array_get(jv_copy(a), 5 as libc::c_int);
    if jv_get_kind(n_4) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_sec = jv_number_value(n_4) as libc::c_int;
    jv_free(n_4);
    let mut n_5: jv = jv_array_get(jv_copy(a), 6 as libc::c_int);
    if jv_get_kind(n_5) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_wday = jv_number_value(n_5) as libc::c_int;
    jv_free(n_5);
    let mut n_6: jv = jv_array_get(jv_copy(a), 7 as libc::c_int);
    if jv_get_kind(n_6) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(a);
        return 0 as libc::c_int
    }
    (*tm).tm_yday = jv_number_value(n_6) as libc::c_int;
    jv_free(n_6);
    jv_free(a);
    // We use UTC everywhere (gettimeofday, gmtime) and UTC does not do DST.
  // Setting tm_isdst to 0 is done by the memset.
  // tm->tm_isdst = 0;
    // The standard permits the tm structure to contain additional members. We
  // hope it is okay to initialize them to zero, because the standard does not
  // provide an alternative.
    return 1 as libc::c_int;
}
unsafe extern "C" fn f_mktime(mut jq: *mut jq_state, mut a: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return ret_error(a,
                         jv_string(b"mktime requires array inputs\x00" as
                                       *const u8 as *const libc::c_char))
    }
    if jv_array_length(jv_copy(a)) < 6 as libc::c_int {
        return ret_error(a,
                         jv_string(b"mktime requires parsed datetime inputs\x00"
                                       as *const u8 as *const libc::c_char))
    }
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    if jv2tm(a, &mut tm) == 0 {
        return jv_invalid_with_msg(jv_string(b"mktime requires parsed datetime inputs\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    let mut t: time_t = my_mktime(&mut tm);
    if t == -(1 as libc::c_int) as time_t {
        return jv_invalid_with_msg(jv_string(b"invalid gmtime representation\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    if t == -(2 as libc::c_int) as time_t {
        return jv_invalid_with_msg(jv_string(b"mktime not supported on this platform\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    return jv_number(t as libc::c_double);
}
unsafe extern "C" fn f_gmtime(mut jq: *mut jq_state, mut a: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return ret_error(a,
                         jv_string(b"gmtime() requires numeric inputs\x00" as
                                       *const u8 as *const libc::c_char))
    }
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut tmp: *mut tm = 0 as *mut tm;
    memset(&mut tm as *mut tm as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<tm>() as libc::c_ulong);
    let mut fsecs: libc::c_double = jv_number_value(a);
    let mut secs: time_t = fsecs as time_t;
    jv_free(a);
    tmp = gmtime_r(&mut secs, &mut tm);
    if tmp.is_null() {
        return jv_invalid_with_msg(jv_string(b"error converting number of seconds since epoch to datetime\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    a = tm2jv(tmp);
    return jv_array_set(a, 5 as libc::c_int,
                        jv_number(jv_number_value(jv_array_get(jv_copy(a),
                                                               5 as
                                                                   libc::c_int))
                                      + (fsecs - floor(fsecs))));
}
unsafe extern "C" fn f_localtime(mut jq: *mut jq_state, mut a: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        return ret_error(a,
                         jv_string(b"localtime() requires numeric inputs\x00"
                                       as *const u8 as *const libc::c_char))
    }
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut tmp: *mut tm = 0 as *mut tm;
    memset(&mut tm as *mut tm as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<tm>() as libc::c_ulong);
    let mut fsecs: libc::c_double = jv_number_value(a);
    let mut secs: time_t = fsecs as time_t;
    jv_free(a);
    tmp = localtime_r(&mut secs, &mut tm);
    if tmp.is_null() {
        return jv_invalid_with_msg(jv_string(b"error converting number of seconds since epoch to datetime\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    a = tm2jv(tmp);
    return jv_array_set(a, 5 as libc::c_int,
                        jv_number(jv_number_value(jv_array_get(jv_copy(a),
                                                               5 as
                                                                   libc::c_int))
                                      + (fsecs - floor(fsecs))));
}
unsafe extern "C" fn f_strftime(mut jq: *mut jq_state, mut a: jv, mut b: jv)
 -> jv {
    if jv_get_kind(a) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        a = f_gmtime(jq, a);
        if jv_is_valid(a) == 0 { jv_free(b); return a }
    } else if jv_get_kind(a) as libc::c_uint !=
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return ret_error2(a, b,
                          jv_string(b"strftime/1 requires parsed datetime inputs\x00"
                                        as *const u8 as *const libc::c_char))
    } else {
        if jv_get_kind(b) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            return ret_error2(a, b,
                              jv_string(b"strftime/1 requires a string format\x00"
                                            as *const u8 as
                                            *const libc::c_char))
        }
    }
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    if jv2tm(a, &mut tm) == 0 {
        return ret_error(b,
                         jv_string(b"strftime/1 requires parsed datetime inputs\x00"
                                       as *const u8 as *const libc::c_char))
    }
    let mut fmt: *const libc::c_char = jv_string_value(b);
    let mut alloced: size_t =
        strlen(fmt).wrapping_add(100 as libc::c_int as libc::c_ulong);
    let mut fresh0 = ::std::vec::from_elem(0, alloced as usize);
    let mut buf: *mut libc::c_char = fresh0.as_mut_ptr() as *mut libc::c_char;
    let mut n: size_t = strftime(buf, alloced, fmt, &mut tm);
    jv_free(b);
    /* POSIX doesn't provide errno values for strftime() failures; weird */
    if n == 0 as libc::c_int as libc::c_ulong || n > alloced {
        return jv_invalid_with_msg(jv_string(b"strftime/1: unknown system failure\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    return jv_string(buf);
}
unsafe extern "C" fn f_strflocaltime(mut jq: *mut jq_state, mut a: jv,
                                     mut b: jv) -> jv {
    if jv_get_kind(a) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        a = f_localtime(jq, a)
    } else if jv_get_kind(a) as libc::c_uint !=
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return ret_error2(a, b,
                          jv_string(b"strflocaltime/1 requires parsed datetime inputs\x00"
                                        as *const u8 as *const libc::c_char))
    } else {
        if jv_get_kind(b) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            return ret_error2(a, b,
                              jv_string(b"strflocaltime/1 requires a string format\x00"
                                            as *const u8 as
                                            *const libc::c_char))
        }
    }
    let mut tm: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    if jv2tm(a, &mut tm) == 0 {
        return jv_invalid_with_msg(jv_string(b"strflocaltime/1 requires parsed datetime inputs\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    let mut fmt: *const libc::c_char = jv_string_value(b);
    let mut alloced: size_t =
        strlen(fmt).wrapping_add(100 as libc::c_int as libc::c_ulong);
    let mut fresh1 = ::std::vec::from_elem(0, alloced as usize);
    let mut buf: *mut libc::c_char = fresh1.as_mut_ptr() as *mut libc::c_char;
    let mut n: size_t = strftime(buf, alloced, fmt, &mut tm);
    jv_free(b);
    /* POSIX doesn't provide errno values for strftime() failures; weird */
    if n == 0 as libc::c_int as libc::c_ulong || n > alloced {
        return jv_invalid_with_msg(jv_string(b"strflocaltime/1: unknown system failure\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    return jv_string(buf);
}
unsafe extern "C" fn f_now(mut jq: *mut jq_state, mut a: jv) -> jv {
    jv_free(a);
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    if gettimeofday(&mut tv, 0 as *mut timezone) == -(1 as libc::c_int) {
        return jv_number(time(0 as *mut time_t) as libc::c_double)
    }
    return jv_number(tv.tv_sec as libc::c_double +
                         tv.tv_usec as libc::c_double / 1000000.0f64);
}
unsafe extern "C" fn f_current_filename(mut jq: *mut jq_state, mut a: jv)
 -> jv {
    jv_free(a);
    let mut r: jv = jq_util_input_get_current_filename(jq);
    if jv_is_valid(r) != 0 { return r }
    jv_free(r);
    return jv_null();
}
unsafe extern "C" fn f_current_line(mut jq: *mut jq_state, mut a: jv) -> jv {
    jv_free(a);
    return jq_util_input_get_current_line(jq);
}
static mut function_list: [cfunction; 130] =
    unsafe {
        [{
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_acos
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"acos\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_acosh
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"acosh\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_asin
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"asin\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_asinh
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"asinh\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_atan
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"atan\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_atan2
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"atan2\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_atanh
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"atanh\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_cbrt
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"cbrt\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_cos
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"cos\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_cosh
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"cosh\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_exp
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"exp\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_exp2
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"exp2\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_floor
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"floor\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_hypot
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"hypot\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_j0
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"j0\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_j1
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"j1\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_log
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"log\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_log10
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"log10\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_log2
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"log2\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_pow
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"pow\x00" as *const u8 as *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_remainder
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"remainder\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_sin
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"sin\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_sinh
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"sinh\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_sqrt
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"sqrt\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_tan
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"tan\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_tanh
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"tanh\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_tgamma
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"tgamma\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_y0
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"y0\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_y1
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"y1\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_jn
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"jn\x00" as *const u8 as *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_yn
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"yn\x00" as *const u8 as *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_ceil
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"ceil\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_copysign
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"copysign\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_drem
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"drem\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_erf
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"erf\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_erfc
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"erfc\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_exp10
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"exp10\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_expm1
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"expm1\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_fabs
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fabs\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_fdim
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fdim\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_fma
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fma\x00" as *const u8 as *const libc::c_char,
                           nargs: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_fmax
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fmax\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_fmin
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fmin\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_fmod
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fmod\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_gamma
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"gamma\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_lgamma
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"lgamma\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_log1p
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"log1p\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_logb
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"logb\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_nearbyint
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"nearbyint\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_nextafter
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"nextafter\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_nexttoward
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"nexttoward\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_pow10
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"pow10\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_rint
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"rint\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_round
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"round\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_scalb
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"scalb\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_scalbln
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"scalbln\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_significand
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"significand\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_trunc
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"trunc\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_ldexp
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"ldexp\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_frexp
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"frexp\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_modf
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"modf\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_lgamma_r
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"lgamma_r\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_plus
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_plus\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_negate
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_negate\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_minus
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_minus\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_multiply
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_multiply\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_divide
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_divide\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_mod
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_mod\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_dump
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"tojson\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_json_parse
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"fromjson\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_tonumber
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"tonumber\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_tostring
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"tostring\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_keys
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"keys\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_keys_unsorted
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"keys_unsorted\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_startswith
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"startswith\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_endswith
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"endswith\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_ltrimstr
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"ltrimstr\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_rtrimstr
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"rtrimstr\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_string_split
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"split\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_string_explode
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"explode\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_string_implode
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"implode\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_string_indexes
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_strindices\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_setpath
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"setpath\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_getpath
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"getpath\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_delpaths
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"delpaths\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_has
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"has\x00" as *const u8 as *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_equal
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_equal\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_notequal
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_notequal\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_less
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_less\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_greater
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_greater\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_lesseq
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_lesseq\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_greatereq
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_greatereq\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_contains
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"contains\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_length
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"length\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_utf8bytelength
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"utf8bytelength\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_type
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"type\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_isinfinite
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"isinfinite\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_isnan
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"isnan\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_isnormal
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"isnormal\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_infinite
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"infinite\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_nan
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"nan\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_sort
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"sort\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_sort_by_impl
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_sort_by_impl\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_group_by_impl
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_group_by_impl\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_min
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"min\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_max
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"max\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_min_by_impl
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_min_by_impl\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_max_by_impl
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_max_by_impl\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_error
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"error\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_format
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"format\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_env
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"env\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_halt
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"halt\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_halt_error
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"halt_error\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_get_search_list
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"get_search_list\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_get_prog_origin
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"get_prog_origin\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_get_jq_origin
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"get_jq_origin\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_match
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"_match_impl\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_modulemeta
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"modulemeta\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_input
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"input\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_debug
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"debug\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_stderr
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"stderr\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_strptime
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"strptime\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_strftime
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"strftime\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_strflocaltime
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"strflocaltime\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_mktime
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"mktime\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_gmtime
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"gmtime\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_localtime
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"localtime\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_now
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"now\x00" as *const u8 as *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_current_filename
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"input_filename\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 cfunction{fptr:
                               ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                       *mut jq_state,
                                                                                   _:
                                                                                       jv)
                                                                  -> jv>,
                                                       cfunction_ptr>(Some(f_current_line
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut jq_state,
                                                                                                    _:
                                                                                                        jv)
                                                                                   ->
                                                                                       jv)),
                           name:
                               b"input_line_number\x00" as *const u8 as
                                   *const libc::c_char,
                           nargs: 1 as libc::c_int,};
             init
         }]
    };
unsafe extern "C" fn bind_bytecoded_builtins(mut b: block) -> block {
    let mut builtins: block = gen_noop();
    let mut builtin_defs: [bytecoded_builtin; 2] =
        [{
             let mut init =
                 bytecoded_builtin{name:
                                       b"empty\x00" as *const u8 as
                                           *const libc::c_char,
                                   code: gen_op_simple(BACKTRACK),};
             init
         },
         {
             let mut init =
                 bytecoded_builtin{name:
                                       b"not\x00" as *const u8 as
                                           *const libc::c_char,
                                   code:
                                       gen_condbranch(gen_const(jv_false()),
                                                      gen_const(jv_true())),};
             init
         }];
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[bytecoded_builtin; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<bytecoded_builtin>()
                                                   as libc::c_ulong) {
        builtins =
            block_join(builtins,
                       gen_function(builtin_defs[i as usize].name, gen_noop(),
                                    builtin_defs[i as usize].code));
        i = i.wrapping_add(1)
    }
    let mut builtin_def_1arg: [bytecoded_builtin; 1] =
        [{
             let mut init =
                 bytecoded_builtin{name:
                                       b"path\x00" as *const u8 as
                                           *const libc::c_char,
                                   code:
                                       block_join(block_join(gen_op_simple(PATH_BEGIN),
                                                             gen_call(b"arg\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      gen_noop())),
                                                  gen_op_simple(PATH_END)),};
             init
         }];
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i_0 as libc::c_ulong) <
              (::std::mem::size_of::<[bytecoded_builtin; 1]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<bytecoded_builtin>()
                                                   as libc::c_ulong) {
        builtins =
            block_join(builtins,
                       gen_function(builtin_def_1arg[i_0 as usize].name,
                                    gen_param(b"arg\x00" as *const u8 as
                                                  *const libc::c_char),
                                    builtin_def_1arg[i_0 as usize].code));
        i_0 = i_0.wrapping_add(1)
    }
    // Note that we can now define `range` as a jq-coded function
    let mut rangevar: block =
        gen_op_var_fresh(STOREV,
                         b"rangevar\x00" as *const u8 as *const libc::c_char);
    let mut rangestart: block =
        gen_op_var_fresh(STOREV,
                         b"rangestart\x00" as *const u8 as
                             *const libc::c_char);
    let mut range: block =
        block_join(block_join(block_join(block_join(block_join(block_join(block_join(gen_op_simple(DUP),
                                                                                     gen_call(b"start\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char,
                                                                                              gen_noop())),
                                                                          rangestart),
                                                               gen_call(b"end\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        gen_noop())),
                                                    gen_op_simple(DUP)),
                                         gen_op_bound(LOADV, rangestart)),
                              rangevar), gen_op_bound(RANGE, rangevar));
    builtins =
        block_join(builtins,
                   gen_function(b"range\x00" as *const u8 as
                                    *const libc::c_char,
                                block_join(gen_param(b"start\x00" as *const u8
                                                         as
                                                         *const libc::c_char),
                                           gen_param(b"end\x00" as *const u8
                                                         as
                                                         *const libc::c_char)),
                                range));
    return block_join(builtins, b);
}
static mut jq_builtins: [libc::c_char; 11115] =
    unsafe {
        *::std::mem::transmute::<&[u8; 11115],
                                 &[libc::c_char; 11115]>(b"def halt_error: halt_error(5);\ndef error(msg): msg|error;\ndef map(f): [.[] | f];\ndef select(f): if f then . else empty end;\ndef sort_by(f): _sort_by_impl(map([f]));\ndef group_by(f): _group_by_impl(map([f]));\ndef unique: group_by(.) | map(.[0]);\ndef unique_by(f): group_by(f) | map(.[0]);\ndef max_by(f): _max_by_impl(map([f]));\ndef min_by(f): _min_by_impl(map([f]));\ndef add: reduce .[] as $x (null; . + $x);\ndef del(f): delpaths([path(f)]);\ndef _assign(paths; $value): reduce path(paths) as $p (.; setpath($p; $value));\ndef _modify(paths; update): reduce path(paths) as $p (.; label $out | (setpath($p; getpath($p) | update) | ., break $out), delpaths([$p]));\ndef map_values(f): .[] |= f;\n\n# recurse\ndef recurse(f): def r: ., (f | r); r;\ndef recurse(f; cond): def r: ., (f | select(cond) | r); r;\ndef recurse: recurse(.[]?);\ndef recurse_down: recurse;\n\ndef to_entries: [keys_unsorted[] as $k | {key: $k, value: .[$k]}];\ndef from_entries: map({(.key // .Key // .name // .Name): (if has(\"value\") then .value else .Value end)}) | add | .//={};\ndef with_entries(f): to_entries | map(f) | from_entries;\ndef reverse: [.[length - 1 - range(0;length)]];\ndef indices($i): if type == \"array\" and ($i|type) == \"array\" then .[$i]\n  elif type == \"array\" then .[[$i]]\n  elif type == \"string\" and ($i|type) == \"string\" then _strindices($i)\n  else .[$i] end;\ndef index($i):   indices($i) | .[0];       # TODO: optimize\ndef rindex($i):  indices($i) | .[-1:][0];  # TODO: optimize\ndef paths: path(recurse(if (type|. == \"array\" or . == \"object\") then .[] else empty end))|select(length > 0);\ndef paths(node_filter): . as $dot|paths|select(. as $p|$dot|getpath($p)|node_filter);\ndef isfinite: type == \"number\" and (isinfinite | not);\ndef arrays: select(type == \"array\");\ndef objects: select(type == \"object\");\ndef iterables: select(type|. == \"array\" or . == \"object\");\ndef booleans: select(type == \"boolean\");\ndef numbers: select(type == \"number\");\ndef normals: select(isnormal);\ndef finites: select(isfinite);\ndef strings: select(type == \"string\");\ndef nulls: select(. == null);\ndef values: select(. != null);\ndef scalars: select(type|. != \"array\" and . != \"object\");\ndef leaf_paths: paths(scalars);\ndef join($x): reduce .[] as $i (null;\n            (if .==null then \"\" else .+$x end) +\n            ($i | if type==\"boolean\" or type==\"number\" then tostring else .//\"\" end)\n        ) // \"\";\ndef _flatten($x): reduce .[] as $i ([]; if $i | type == \"array\" and $x != 0 then . + ($i | _flatten($x-1)) else . + [$i] end);\ndef flatten($x): if $x < 0 then error(\"flatten depth must not be negative\") else _flatten($x) end;\ndef flatten: _flatten(-1);\ndef range($x): range(0;$x);\ndef fromdateiso8601: strptime(\"%Y-%m-%dT%H:%M:%SZ\")|mktime;\ndef todateiso8601: strftime(\"%Y-%m-%dT%H:%M:%SZ\");\ndef fromdate: fromdateiso8601;\ndef todate: todateiso8601;\ndef match(re; mode): _match_impl(re; mode; false)|.[];\ndef match($val): ($val|type) as $vt | if $vt == \"string\" then match($val; null)\n   elif $vt == \"array\" and ($val | length) > 1 then match($val[0]; $val[1])\n   elif $vt == \"array\" and ($val | length) > 0 then match($val[0]; null)\n   else error( $vt + \" not a string or array\") end;\ndef test(re; mode): _match_impl(re; mode; true);\ndef test($val): ($val|type) as $vt | if $vt == \"string\" then test($val; null)\n   elif $vt == \"array\" and ($val | length) > 1 then test($val[0]; $val[1])\n   elif $vt == \"array\" and ($val | length) > 0 then test($val[0]; null)\n   else error( $vt + \" not a string or array\") end;\ndef capture(re; mods): match(re; mods) | reduce ( .captures | .[] | select(.name != null) | { (.name) : .string } ) as $pair ({}; . + $pair);\ndef capture($val): ($val|type) as $vt | if $vt == \"string\" then capture($val; null)\n   elif $vt == \"array\" and ($val | length) > 1 then capture($val[0]; $val[1])\n   elif $vt == \"array\" and ($val | length) > 0 then capture($val[0]; null)\n   else error( $vt + \" not a string or array\") end;\ndef scan(re):\n  match(re; \"g\")\n  |  if (.captures|length > 0)\n      then [ .captures | .[] | .string ]\n      else .string\n      end ;\n#\n# If input is an array, then emit a stream of successive subarrays of length n (or less),\n# and similarly for strings.\ndef _nwise(a; $n): if a|length <= $n then a else a[0:$n] , _nwise(a[$n:]; $n) end;\ndef _nwise($n): _nwise(.; $n);\n#\n# splits/1 produces a stream; split/1 is retained for backward compatibility.\ndef splits($re; flags): . as $s\n#  # multiple occurrences of \"g\" are acceptable\n  | [ match($re; \"g\" + flags) | (.offset, .offset + .length) ]\n  | [0] + . +[$s|length]\n  | _nwise(2)\n  | $s[.[0]:.[1] ] ;\ndef splits($re): splits($re; null);\n#\n# split emits an array for backward compatibility\ndef split($re; flags): [ splits($re; flags) ];\n#\n# If s contains capture variables, then create a capture object and pipe it to s\ndef sub($re; s):\n  . as $in\n  | [match($re)]\n  | if length == 0 then $in\n    else .[0]\n    | . as $r\n#  # create the \"capture\" object:\n    | reduce ( $r | .captures | .[] | select(.name != null) | { (.name) : .string } ) as $pair\n        ({}; . + $pair)\n    | $in[0:$r.offset] + s + $in[$r.offset+$r.length:]\n    end ;\n#\n# If s contains capture variables, then create a capture object and pipe it to s\ndef sub($re; s; flags):\n  def subg: [explode[] | select(. != 103)] | implode;\n  # \"fla\" should be flags with all occurrences of g removed; gs should be non-nil if flags has a g\n  def sub1(fla; gs):\n    def mysub:\n      . as $in\n      | [match($re; fla)]\n      | if length == 0 then $in\n        else .[0] as $edit\n        | ($edit | .offset + .length) as $len\n        # create the \"capture\" object:\n        | reduce ( $edit | .captures | .[] | select(.name != null) | { (.name) : .string } ) as $pair\n            ({}; . + $pair)\n        | $in[0:$edit.offset]\n          + s\n          + ($in[$len:] | if length > 0 and gs then mysub else . end)\n        end ;\n    mysub ;\n    (flags | index(\"g\")) as $gs\n    | (flags | if $gs then subg else . end) as $fla\n    | sub1($fla; $gs);\n#\ndef sub($re; s): sub($re; s; \"\");\n# repeated substitution of re (which may contain named captures)\ndef gsub($re; s; flags): sub($re; s; flags + \"g\");\ndef gsub($re; s): sub($re; s; \"g\");\n\n########################################################################\n# generic iterator/generator\ndef while(cond; update):\n     def _while:\n         if cond then ., (update | _while) else empty end;\n     _while;\ndef until(cond; next):\n     def _until:\n         if cond then . else (next|_until) end;\n     _until;\ndef limit($n; exp):\n    if $n > 0 then label $out | foreach exp as $item ($n; .-1; $item, if . <= 0 then break $out else empty end)\n    elif $n == 0 then empty\n    else exp end;\n# range/3, with a `by` expression argument\ndef range($init; $upto; $by):\n    if $by > 0 then $init|while(. < $upto; . + $by)\n  elif $by < 0 then $init|while(. > $upto; . + $by)\n  else empty end;\ndef first(g): label $out | g | ., break $out;\ndef isempty(g): first((g|false), true);\ndef all(generator; condition): isempty(generator|condition and empty);\ndef any(generator; condition): isempty(generator|condition or empty)|not;\ndef all(condition): all(.[]; condition);\ndef any(condition): any(.[]; condition);\ndef all: all(.[]; .);\ndef any: any(.[]; .);\ndef last(g): reduce g as $item (null; $item);\ndef nth($n; g): if $n < 0 then error(\"nth doesn\'t support negative indices\") else last(limit($n + 1; g)) end;\ndef first: .[0];\ndef last: .[-1];\ndef nth($n): .[$n];\ndef combinations:\n    if length == 0 then [] else\n        .[0][] as $x\n          | (.[1:] | combinations) as $y\n          | [$x] + $y\n    end;\ndef combinations(n):\n    . as $dot\n      | [range(n) | $dot]\n      | combinations;\n# transpose a possibly jagged matrix, quickly;\n# rows are padded with nulls so the result is always rectangular.\ndef transpose:\n  if . == [] then []\n  else . as $in\n  | (map(length) | max) as $max\n  | length as $length\n  | reduce range(0; $max) as $j\n      ([]; . + [reduce range(0;$length) as $i ([]; . + [ $in[$i][$j] ] )] )\n\t        end;\ndef in(xs): . as $x | xs | has($x);\ndef inside(xs): . as $x | xs | contains($x);\ndef repeat(exp):\n     def _repeat:\n         exp, _repeat;\n     _repeat;\ndef inputs: try repeat(input) catch if .==\"break\" then empty else error end;\n# like ruby\'s downcase - only characters A to Z are affected\ndef ascii_downcase:\n  explode | map( if 65 <= . and . <= 90 then . + 32  else . end) | implode;\n# like ruby\'s upcase - only characters a to z are affected\ndef ascii_upcase:\n  explode | map( if 97 <= . and . <= 122 then . - 32  else . end) | implode;\n\n# Streaming utilities\ndef truncate_stream(stream):\n  . as $n | null | stream | . as $input | if (.[0]|length) > $n then setpath([0];$input[0][$n:]) else empty end;\ndef fromstream(i): {x: null, e: false} as $init |\n  # .x = object being built; .e = emit and reset state\n  foreach i as $i ($init\n  ; if .e then $init else . end\n  | if $i|length == 2\n    then setpath([\"e\"]; $i[0]|length==0) | setpath([\"x\"]+$i[0]; $i[1])\n    else setpath([\"e\"]; $i[0]|length==1) end\n  ; if .e then .x else empty end);\ndef tostream:\n  path(def r: (.[]?|r), .; r) as $p |\n  getpath($p) |\n  reduce path(.[]?) as $q ([$p, .]; [$p+$q]);\n\n\n# Assuming the input array is sorted, bsearch/1 returns\n# the index of the target if the target is in the input array; and otherwise\n#  (-1 - ix), where ix is the insertion point that would leave the array sorted.\n# If the input is not sorted, bsearch will terminate but with irrelevant results.\ndef bsearch($target):\n  if length == 0 then -1\n  elif length == 1 then\n     if $target == .[0] then 0 elif $target < .[0] then -1 else -2 end\n  else . as $in\n    # state variable: [start, end, answer]\n    # where start and end are the upper and lower offsets to use.\n    | [0, length-1, null]\n    | until( .[0] > .[1] ;\n             if .[2] != null then (.[1] = -1)               # i.e. break\n             else\n               ( ( (.[1] + .[0]) / 2 ) | floor ) as $mid\n               | $in[$mid] as $monkey\n               | if $monkey == $target  then (.[2] = $mid)   # success\n                 elif .[0] == .[1]     then (.[1] = -1)     # failure\n                 elif $monkey < $target then (.[0] = ($mid + 1))\n                 else (.[1] = ($mid - 1))\n                 end\n             end )\n    | if .[2] == null then          # compute the insertion point\n         if $in[ .[0] ] < $target then (-2 -.[0])\n         else (-1 -.[0])\n         end\n      else .[2]\n      end\n  end;\n\n# Apply f to composite entities recursively, and to atoms\ndef walk(f):\n  . as $in\n  | if type == \"object\" then\n      reduce keys_unsorted[] as $key\n        ( {}; . + { ($key):  ($in[$key] | walk(f)) } ) | f\n  elif type == \"array\" then map( walk(f) ) | f\n  else f\n  end;\n\n# SQL-ish operators here:\ndef INDEX(stream; idx_expr):\n  reduce stream as $row ({}; .[$row|idx_expr|tostring] = $row);\ndef INDEX(idx_expr): INDEX(.[]; idx_expr);\ndef JOIN($idx; idx_expr):\n  [.[] | [., $idx[idx_expr]]];\ndef JOIN($idx; stream; idx_expr):\n  stream | [., $idx[idx_expr]];\ndef JOIN($idx; stream; idx_expr; join_expr):\n  stream | [., $idx[idx_expr]] | join_expr;\ndef IN(s): any(s == .; .);\ndef IN(src; s): any(src == s; .);\n\x00")
    };
/* Include jq-coded builtins */
/* Include unsupported math functions next */
unsafe extern "C" fn gen_builtin_list(mut builtins: block) -> block {
    let mut list: jv =
        jv_array_append(block_list_funcs(builtins, 1 as libc::c_int),
                        jv_string(b"builtins/0\x00" as *const u8 as
                                      *const libc::c_char));
    return block_join(builtins,
                      gen_function(b"builtins\x00" as *const u8 as
                                       *const libc::c_char, gen_noop(),
                                   gen_const(list)));
}
#[no_mangle]
pub unsafe extern "C" fn builtins_bind(mut jq: *mut jq_state,
                                       mut bb: *mut block) -> libc::c_int {
    let mut builtins: block =
        block{first: 0 as *mut inst, last: 0 as *mut inst,};
    let mut src: *mut locfile =
        locfile_init(jq, b"<builtin>\x00" as *const u8 as *const libc::c_char,
                     jq_builtins.as_ptr(),
                     (::std::mem::size_of::<[libc::c_char; 11115]>() as
                          libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong) as
                         libc::c_int);
    let mut nerrors: libc::c_int = jq_parse_library(src, &mut builtins);
    if nerrors == 0 {
    } else {
        __assert_fail(b"!nerrors\x00" as *const u8 as *const libc::c_char,
                      b"src/builtin.c\x00" as *const u8 as
                          *const libc::c_char,
                      1847 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int builtins_bind(jq_state *, block *)\x00")).as_ptr());
    };
    locfile_free(src);
    builtins = bind_bytecoded_builtins(builtins);
    builtins =
        gen_cbinding(function_list.as_ptr(),
                     (::std::mem::size_of::<[cfunction; 130]>() as
                          libc::c_ulong).wrapping_div(::std::mem::size_of::<cfunction>()
                                                          as libc::c_ulong) as
                         libc::c_int, builtins);
    builtins = gen_builtin_list(builtins);
    *bb =
        block_bind_referenced(builtins, *bb,
                              OP_IS_CALL_PSEUDO as libc::c_int);
    return nerrors;
}
