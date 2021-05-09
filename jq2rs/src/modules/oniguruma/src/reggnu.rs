#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn onig_compile(reg: *mut regex_t, pattern: *const OnigUChar,
                    pattern_end: *const OnigUChar, einfo: *mut OnigErrorInfo)
     -> libc::c_int;
    #[no_mangle]
    static mut OnigDefaultCaseFoldFlag: OnigCaseFoldType;
    #[no_mangle]
    static mut OnigEncodingASCII: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF8: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingEUC_JP: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingSJIS: OnigEncodingType;
    #[no_mangle]
    fn onig_initialize_encoding(enc: OnigEncoding) -> libc::c_int;
    #[no_mangle]
    fn onigenc_set_default_encoding(enc: OnigEncoding) -> libc::c_int;
    #[no_mangle]
    fn onigenc_set_default_caseconv_table(table: *const OnigUChar);
    #[no_mangle]
    fn onigenc_get_right_adjust_char_head(enc: OnigEncoding,
                                          start: *const OnigUChar,
                                          s: *const OnigUChar)
     -> *mut OnigUChar;
    #[no_mangle]
    static mut OnigDefaultSyntax: *mut OnigSyntaxType;
    /* Oniguruma Native API */
    #[no_mangle]
    fn onig_initialize(encodings: *mut OnigEncoding, n: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn onig_error_code_to_str(s: *mut OnigUChar, err_code: libc::c_int,
                              _: ...) -> libc::c_int;
    #[no_mangle]
    fn onig_reg_init(reg: *mut regex_t, option: OnigOptionType,
                     case_fold_flag: OnigCaseFoldType, enc: OnigEncoding,
                     syntax: *mut OnigSyntaxType) -> libc::c_int;
    #[no_mangle]
    fn onig_free(_: OnigRegex);
    #[no_mangle]
    fn onig_search(_: OnigRegex, str: *const OnigUChar, end: *const OnigUChar,
                   start: *const OnigUChar, range: *const OnigUChar,
                   region: *mut OnigRegion, option: OnigOptionType)
     -> libc::c_int;
    #[no_mangle]
    fn onig_match(_: OnigRegex, str: *const OnigUChar, end: *const OnigUChar,
                  at: *const OnigUChar, region: *mut OnigRegion,
                  option: OnigOptionType) -> libc::c_int;
    #[no_mangle]
    fn onig_region_free(region: *mut OnigRegion, free_self: libc::c_int);
    #[no_mangle]
    static mut OnigEncDefaultCharEncoding: OnigEncoding;
}
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
/* general error */
/* syntax error */
/* values error (syntax error) */
/* errors related to thread */
/* #define ONIGERR_OVER_THREAD_PASS_LIMIT_COUNT                -1001 */
/* must be smaller than BIT_STATUS_BITS_NUM (unsigned int * 8) */
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
pub type OnigCaptureTreeNode = OnigCaptureTreeNodeStruct;
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
/* escape compile-conflict */
/* *********************************************************************
  reggnu.c -  Oniguruma (regular expression library)
**********************************************************************/
/*-
 * Copyright (c) 2002-2008  K.Kosako  <sndgk393 AT ybb DOT ne DOT jp>
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
#[no_mangle]
pub unsafe extern "C" fn re_free_registers(mut r: *mut OnigRegion) {
    /* 0: don't free self */
    onig_region_free(r, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn re_adjust_startpos(mut reg: *mut regex_t,
                                            mut string: *const libc::c_char,
                                            mut size: libc::c_int,
                                            mut startpos: libc::c_int,
                                            mut range: libc::c_int)
 -> libc::c_int {
    if startpos > 0 as libc::c_int &&
           (*(*reg).enc).max_enc_len != 1 as libc::c_int && startpos < size {
        let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
        let mut s: *mut OnigUChar =
            (string as *mut OnigUChar).offset(startpos as isize);
        if range > 0 as libc::c_int {
            p =
                onigenc_get_right_adjust_char_head((*reg).enc,
                                                   string as *mut OnigUChar,
                                                   s)
        } else {
            p =
                (*(*reg).enc).left_adjust_char_head.expect("non-null function pointer")(string
                                                                                            as
                                                                                            *mut OnigUChar,
                                                                                        s)
        }
        return p.wrapping_offset_from(string as *mut OnigUChar) as
                   libc::c_long as libc::c_int
    }
    return startpos;
}
#[no_mangle]
pub unsafe extern "C" fn re_match(mut reg: *mut regex_t,
                                  mut str: *const libc::c_char,
                                  mut size: libc::c_int, mut pos: libc::c_int,
                                  mut regs: *mut re_registers)
 -> libc::c_int {
    return onig_match(reg, str as *mut OnigUChar,
                      str.offset(size as isize) as *mut OnigUChar,
                      str.offset(pos as isize) as *mut OnigUChar, regs,
                      0 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn re_search(mut bufp: *mut regex_t,
                                   mut string: *const libc::c_char,
                                   mut size: libc::c_int,
                                   mut startpos: libc::c_int,
                                   mut range: libc::c_int,
                                   mut regs: *mut re_registers)
 -> libc::c_int {
    return onig_search(bufp, string as *mut OnigUChar,
                       string.offset(size as isize) as *mut OnigUChar,
                       string.offset(startpos as isize) as *mut OnigUChar,
                       string.offset(startpos as isize).offset(range as isize)
                           as *mut OnigUChar, regs, 0 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn re_compile_pattern(mut pattern: *const libc::c_char,
                                            mut size: libc::c_int,
                                            mut reg: *mut regex_t,
                                            mut ebuf: *mut libc::c_char)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut einfo: OnigErrorInfo =
        OnigErrorInfo{enc: 0 as *mut OnigEncodingType,
                      par: 0 as *mut OnigUChar,
                      par_end: 0 as *mut OnigUChar,};
    r =
        onig_compile(reg, pattern as *mut OnigUChar,
                     pattern.offset(size as isize) as *mut OnigUChar,
                     &mut einfo);
    if r != 0 as libc::c_int {
        if !(ebuf as *mut libc::c_void).is_null() {
            onig_error_code_to_str(ebuf as *mut OnigUChar, r,
                                   &mut einfo as *mut OnigErrorInfo);
        }
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn re_free_pattern(mut reg: *mut regex_t) {
    onig_free(reg);
}
#[no_mangle]
pub unsafe extern "C" fn re_alloc_pattern(mut reg: *mut *mut regex_t)
 -> libc::c_int {
    *reg =
        malloc(::std::mem::size_of::<regex_t>() as libc::c_ulong) as
            *mut regex_t;
    if (*reg as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    return onig_reg_init(*reg, 0 as libc::c_uint, OnigDefaultCaseFoldFlag,
                         OnigEncDefaultCharEncoding, OnigDefaultSyntax);
}
#[no_mangle]
pub unsafe extern "C" fn re_set_casetable(mut table: *const libc::c_char) {
    onigenc_set_default_caseconv_table(table as *mut OnigUChar);
}
/* GNU regex options */
#[no_mangle]
pub unsafe extern "C" fn re_mbcinit(mut mb_code: libc::c_int) {
    let mut enc: OnigEncoding = 0 as *mut OnigEncodingType;
    match mb_code {
        0 => { enc = &mut OnigEncodingASCII }
        1 => { enc = &mut OnigEncodingEUC_JP }
        2 => { enc = &mut OnigEncodingSJIS }
        3 => { enc = &mut OnigEncodingUTF8 }
        _ => { return }
    }
    onig_initialize(0 as *mut OnigEncoding, 0 as libc::c_int);
    onig_initialize_encoding(enc);
    onigenc_set_default_encoding(enc);
}
