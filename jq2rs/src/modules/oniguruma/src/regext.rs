#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut OnigEncodingASCII: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingISO_8859_1: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF16_BE: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF16_LE: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF32_BE: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF32_LE: OnigEncodingType;
    #[no_mangle]
    fn onig_compile(reg: *mut regex_t, pattern: *const OnigUChar,
                    pattern_end: *const OnigUChar, einfo: *mut OnigErrorInfo)
     -> libc::c_int;
    #[no_mangle]
    fn onig_reg_init(reg: *mut regex_t, option: OnigOptionType,
                     case_fold_flag: OnigCaseFoldType, enc: OnigEncoding,
                     syntax: *mut OnigSyntaxType) -> libc::c_int;
    #[no_mangle]
    fn onig_free(_: OnigRegex);
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
pub struct OnigCompileInfo {
    pub num_of_elements: libc::c_int,
    pub pattern_enc: OnigEncoding,
    pub target_enc: OnigEncoding,
    pub syntax: *mut OnigSyntaxType,
    pub option: OnigOptionType,
    pub case_fold_flag: OnigCaseFoldType,
}
/* *********************************************************************
  regext.c -  Oniguruma (regular expression library)
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
unsafe extern "C" fn conv_ext0be32(mut s: *const OnigUChar,
                                   mut end: *const OnigUChar,
                                   mut conv: *mut OnigUChar) {
    while s < end {
        let fresh0 = conv;
        conv = conv.offset(1);
        *fresh0 = '\u{0}' as i32 as OnigUChar;
        let fresh1 = conv;
        conv = conv.offset(1);
        *fresh1 = '\u{0}' as i32 as OnigUChar;
        let fresh2 = conv;
        conv = conv.offset(1);
        *fresh2 = '\u{0}' as i32 as OnigUChar;
        let fresh3 = s;
        s = s.offset(1);
        let fresh4 = conv;
        conv = conv.offset(1);
        *fresh4 = *fresh3
    };
}
unsafe extern "C" fn conv_ext0le32(mut s: *const OnigUChar,
                                   mut end: *const OnigUChar,
                                   mut conv: *mut OnigUChar) {
    while s < end {
        let fresh5 = s;
        s = s.offset(1);
        let fresh6 = conv;
        conv = conv.offset(1);
        *fresh6 = *fresh5;
        let fresh7 = conv;
        conv = conv.offset(1);
        *fresh7 = '\u{0}' as i32 as OnigUChar;
        let fresh8 = conv;
        conv = conv.offset(1);
        *fresh8 = '\u{0}' as i32 as OnigUChar;
        let fresh9 = conv;
        conv = conv.offset(1);
        *fresh9 = '\u{0}' as i32 as OnigUChar
    };
}
unsafe extern "C" fn conv_ext0be(mut s: *const OnigUChar,
                                 mut end: *const OnigUChar,
                                 mut conv: *mut OnigUChar) {
    while s < end {
        let fresh10 = conv;
        conv = conv.offset(1);
        *fresh10 = '\u{0}' as i32 as OnigUChar;
        let fresh11 = s;
        s = s.offset(1);
        let fresh12 = conv;
        conv = conv.offset(1);
        *fresh12 = *fresh11
    };
}
unsafe extern "C" fn conv_ext0le(mut s: *const OnigUChar,
                                 mut end: *const OnigUChar,
                                 mut conv: *mut OnigUChar) {
    while s < end {
        let fresh13 = s;
        s = s.offset(1);
        let fresh14 = conv;
        conv = conv.offset(1);
        *fresh14 = *fresh13;
        let fresh15 = conv;
        conv = conv.offset(1);
        *fresh15 = '\u{0}' as i32 as OnigUChar
    };
}
unsafe extern "C" fn conv_swap4bytes(mut s: *const OnigUChar,
                                     mut end: *const OnigUChar,
                                     mut conv: *mut OnigUChar) {
    while s < end {
        let fresh16 = conv;
        conv = conv.offset(1);
        *fresh16 = *s.offset(3 as libc::c_int as isize);
        let fresh17 = conv;
        conv = conv.offset(1);
        *fresh17 = *s.offset(2 as libc::c_int as isize);
        let fresh18 = conv;
        conv = conv.offset(1);
        *fresh18 = *s.offset(1 as libc::c_int as isize);
        let fresh19 = conv;
        conv = conv.offset(1);
        *fresh19 = *s.offset(0 as libc::c_int as isize);
        s = s.offset(4 as libc::c_int as isize)
    };
}
unsafe extern "C" fn conv_swap2bytes(mut s: *const OnigUChar,
                                     mut end: *const OnigUChar,
                                     mut conv: *mut OnigUChar) {
    while s < end {
        let fresh20 = conv;
        conv = conv.offset(1);
        *fresh20 = *s.offset(1 as libc::c_int as isize);
        let fresh21 = conv;
        conv = conv.offset(1);
        *fresh21 = *s.offset(0 as libc::c_int as isize);
        s = s.offset(2 as libc::c_int as isize)
    };
}
unsafe extern "C" fn conv_encoding(mut from: OnigEncoding,
                                   mut to: OnigEncoding,
                                   mut s: *const OnigUChar,
                                   mut end: *const OnigUChar,
                                   mut conv: *mut *mut OnigUChar,
                                   mut conv_end: *mut *mut OnigUChar)
 -> libc::c_int {
    let mut len: libc::c_int =
        end.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    let mut current_block_19: u64;
    if to == &mut OnigEncodingUTF16_BE as *mut OnigEncodingType {
        if from == &mut OnigEncodingASCII as *mut OnigEncodingType ||
               from == &mut OnigEncodingISO_8859_1 as *mut OnigEncodingType {
            *conv =
                malloc((len * 2 as libc::c_int) as libc::c_ulong) as
                    *mut OnigUChar;
            if (*conv as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            *conv_end = (*conv).offset((len * 2 as libc::c_int) as isize);
            conv_ext0be(s, end, *conv);
            return 0 as libc::c_int
        } else if from == &mut OnigEncodingUTF16_LE as *mut OnigEncodingType {
            current_block_19 = 9754527956383385389;
        } else { current_block_19 = 11194104282611034094; }
    } else if to == &mut OnigEncodingUTF16_LE as *mut OnigEncodingType {
        if from == &mut OnigEncodingASCII as *mut OnigEncodingType ||
               from == &mut OnigEncodingISO_8859_1 as *mut OnigEncodingType {
            *conv =
                malloc((len * 2 as libc::c_int) as libc::c_ulong) as
                    *mut OnigUChar;
            if (*conv as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            *conv_end = (*conv).offset((len * 2 as libc::c_int) as isize);
            conv_ext0le(s, end, *conv);
            return 0 as libc::c_int
        } else if from == &mut OnigEncodingUTF16_BE as *mut OnigEncodingType {
            current_block_19 = 9754527956383385389;
        } else { current_block_19 = 11194104282611034094; }
    } else { current_block_19 = 11194104282611034094; }
    match current_block_19 {
        11194104282611034094 => { }
        _ => {
            *conv = malloc(len as libc::c_ulong) as *mut OnigUChar;
            if (*conv as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            *conv_end = (*conv).offset(len as isize);
            conv_swap2bytes(s, end, *conv);
            return 0 as libc::c_int
        }
    }
    let mut current_block_40: u64;
    if to == &mut OnigEncodingUTF32_BE as *mut OnigEncodingType {
        if from == &mut OnigEncodingASCII as *mut OnigEncodingType ||
               from == &mut OnigEncodingISO_8859_1 as *mut OnigEncodingType {
            *conv =
                malloc((len * 4 as libc::c_int) as libc::c_ulong) as
                    *mut OnigUChar;
            if (*conv as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            *conv_end = (*conv).offset((len * 4 as libc::c_int) as isize);
            conv_ext0be32(s, end, *conv);
            return 0 as libc::c_int
        } else if from == &mut OnigEncodingUTF32_LE as *mut OnigEncodingType {
            current_block_40 = 6397155717754647463;
        } else { current_block_40 = 12497913735442871383; }
    } else if to == &mut OnigEncodingUTF32_LE as *mut OnigEncodingType {
        if from == &mut OnigEncodingASCII as *mut OnigEncodingType ||
               from == &mut OnigEncodingISO_8859_1 as *mut OnigEncodingType {
            *conv =
                malloc((len * 4 as libc::c_int) as libc::c_ulong) as
                    *mut OnigUChar;
            if (*conv as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            *conv_end = (*conv).offset((len * 4 as libc::c_int) as isize);
            conv_ext0le32(s, end, *conv);
            return 0 as libc::c_int
        } else if from == &mut OnigEncodingUTF32_BE as *mut OnigEncodingType {
            current_block_40 = 6397155717754647463;
        } else { current_block_40 = 12497913735442871383; }
    } else { current_block_40 = 12497913735442871383; }
    match current_block_40 {
        12497913735442871383 => { }
        _ => {
            *conv = malloc(len as libc::c_ulong) as *mut OnigUChar;
            if (*conv as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            *conv_end = (*conv).offset(len as isize);
            conv_swap4bytes(s, end, *conv);
            return 0 as libc::c_int
        }
    }
    return -(402 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn onig_new_deluxe(mut reg: *mut *mut regex_t,
                                         mut pattern: *const OnigUChar,
                                         mut pattern_end: *const OnigUChar,
                                         mut ci: *mut OnigCompileInfo,
                                         mut einfo: *mut OnigErrorInfo)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut cpat: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut cpat_end: *mut OnigUChar = 0 as *mut OnigUChar;
    if !(einfo as *mut libc::c_void).is_null() {
        (*einfo).par = 0 as *mut libc::c_void as *mut OnigUChar
    }
    if (*ci).pattern_enc != (*ci).target_enc {
        r =
            conv_encoding((*ci).pattern_enc, (*ci).target_enc, pattern,
                          pattern_end, &mut cpat, &mut cpat_end);
        if r != 0 { return r }
    } else {
        cpat = pattern as *mut OnigUChar;
        cpat_end = pattern_end as *mut OnigUChar
    }
    *reg =
        malloc(::std::mem::size_of::<regex_t>() as libc::c_ulong) as
            *mut regex_t;
    if (*reg as *mut libc::c_void).is_null() {
        r = -(5 as libc::c_int)
    } else {
        r =
            onig_reg_init(*reg, (*ci).option, (*ci).case_fold_flag,
                          (*ci).target_enc, (*ci).syntax);
        if r != 0 {
            current_block = 12147809277273686145;
        } else {
            r = onig_compile(*reg, cpat, cpat_end, einfo);
            if r != 0 {
                current_block = 12147809277273686145;
            } else { current_block = 12549387264633237035; }
        }
        match current_block {
            12549387264633237035 => { }
            _ => { onig_free(*reg); *reg = 0 as *mut regex_t }
        }
    }
    if cpat != pattern as *mut OnigUChar { free(cpat as *mut libc::c_void); }
    return r;
}
