#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn onig_is_in_code_range(p: *const OnigUChar, code: OnigCodePoint)
     -> libc::c_int;
    #[no_mangle]
    static OnigEncAsciiCtypeTable: [libc::c_ushort; 0];
    #[no_mangle]
    fn euc_jp_lookup_property_name(str: *const libc::c_char,
                                   len: libc::c_uint)
     -> *mut PropertyNameCtype;
    #[no_mangle]
    fn onigenc_ascii_get_case_fold_codes_by_str(flag: OnigCaseFoldType,
                                                p: *const OnigUChar,
                                                end: *const OnigUChar,
                                                items:
                                                    *mut OnigCaseFoldCodeItem)
     -> libc::c_int;
    #[no_mangle]
    fn onigenc_ascii_apply_all_case_fold(flag: OnigCaseFoldType,
                                         f: OnigApplyAllCaseFoldFunc,
                                         arg: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    static OnigEncAsciiToLowerCaseTable: [OnigUChar; 0];
    #[no_mangle]
    fn onigenc_is_mbc_newline_0x0a(p: *const OnigUChar, end: *const OnigUChar)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type OnigCaseFoldType = libc::c_uint;
/* #define ONIGENC_CASE_FOLD_HIRAGANA_KATAKANA  (1<<1) */
/* #define ONIGENC_CASE_FOLD_KATAKANA_WIDTH     (1<<2) */
/* 13 => Unicode:0x1ffc */
/* code range */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigCaseFoldCodeItem {
    pub byte_len: libc::c_int,
    pub code_len: libc::c_int,
    pub code: [OnigCodePoint; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropertyNameCtype {
    pub name: *mut libc::c_char,
    pub ctype: libc::c_int,
}
static mut EncLen_EUCJP: [libc::c_int; 256] =
    [1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int];
unsafe extern "C" fn mbc_enc_len(mut p: *const OnigUChar) -> libc::c_int {
    return EncLen_EUCJP[*p as usize];
}
unsafe extern "C" fn is_valid_mbc_string(mut p: *const OnigUChar,
                                         mut end: *const OnigUChar)
 -> libc::c_int {
    while p < end {
        if (*p as libc::c_int) < 0x80 as libc::c_int {
            p = p.offset(1)
        } else if *p as libc::c_int > 0xa0 as libc::c_int {
            if *p as libc::c_int == 0xff as libc::c_int {
                return 0 as libc::c_int
            }
            p = p.offset(1);
            if p >= end { return 0 as libc::c_int }
            if (*p as libc::c_int) < 0xa1 as libc::c_int ||
                   *p as libc::c_int == 0xff as libc::c_int {
                return 0 as libc::c_int
            }
            p = p.offset(1)
        } else if *p as libc::c_int == 0x8e as libc::c_int {
            p = p.offset(1);
            if p >= end { return 0 as libc::c_int }
            if (*p as libc::c_int) < 0xa1 as libc::c_int ||
                   *p as libc::c_int > 0xdf as libc::c_int {
                return 0 as libc::c_int
            }
            p = p.offset(1)
        } else if *p as libc::c_int == 0x8f as libc::c_int {
            p = p.offset(1);
            if p >= end { return 0 as libc::c_int }
            if (*p as libc::c_int) < 0xa1 as libc::c_int ||
                   *p as libc::c_int == 0xff as libc::c_int {
                return 0 as libc::c_int
            }
            p = p.offset(1);
            if p >= end { return 0 as libc::c_int }
            if (*p as libc::c_int) < 0xa1 as libc::c_int ||
                   *p as libc::c_int == 0xff as libc::c_int {
                return 0 as libc::c_int
            }
            p = p.offset(1)
        } else { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mbc_to_code(mut p: *const OnigUChar,
                                 mut end: *const OnigUChar) -> OnigCodePoint {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut n: OnigCodePoint = 0;
    len =
        OnigEncodingEUC_JP.mbc_enc_len.expect("non-null function pointer")(p);
    let fresh0 = p;
    p = p.offset(1);
    n = *fresh0 as OnigCodePoint;
    if len == 1 as libc::c_int { return n }
    i = 1 as libc::c_int;
    while i < len {
        if p >= end { break ; }
        let fresh1 = p;
        p = p.offset(1);
        c = *fresh1 as libc::c_int;
        n <<= 8 as libc::c_int;
        n =
            (n as libc::c_uint).wrapping_add(c as libc::c_uint) as
                OnigCodePoint as OnigCodePoint;
        i += 1
    }
    return n;
}
unsafe extern "C" fn code_to_mbclen(mut code: OnigCodePoint) -> libc::c_int {
    if code < 128 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else if code & 0xff0000 as libc::c_int as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
        return 3 as libc::c_int
    } else if code & 0xff00 as libc::c_int as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
        return 2 as libc::c_int
    } else { return -(400 as libc::c_int) };
}
unsafe extern "C" fn code_to_mbc(mut code: OnigCodePoint,
                                 mut buf: *mut OnigUChar) -> libc::c_int {
    let mut p: *mut OnigUChar = buf;
    if code & 0xff0000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 =
            (code >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as OnigUChar
    }
    if code & 0xff00 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 =
            (code >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as OnigUChar
    }
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = (code & 0xff as libc::c_int as libc::c_uint) as OnigUChar;
    if OnigEncodingEUC_JP.mbc_enc_len.expect("non-null function pointer")(buf)
           as libc::c_long != p.wrapping_offset_from(buf) as libc::c_long {
        return -(400 as libc::c_int)
    }
    return p.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn mbc_case_fold(mut flag: OnigCaseFoldType,
                                   mut pp: *mut *const OnigUChar,
                                   mut end: *const OnigUChar,
                                   mut lower: *mut OnigUChar) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *const OnigUChar = *pp;
    if (*p as libc::c_int) < 128 as libc::c_int {
        *lower = *OnigEncAsciiToLowerCaseTable.as_ptr().offset(*p as isize);
        *pp = (*pp).offset(1);
        return 1 as libc::c_int
    } else {
        let mut i: libc::c_int = 0;
        len =
            OnigEncodingEUC_JP.mbc_enc_len.expect("non-null function pointer")(p);
        i = 0 as libc::c_int;
        while i < len {
            let fresh5 = p;
            p = p.offset(1);
            let fresh6 = lower;
            lower = lower.offset(1);
            *fresh6 = *fresh5;
            i += 1
        }
        *pp = (*pp).offset(len as isize);
        return len
        /* return byte length of converted char to lower */
    };
}
unsafe extern "C" fn left_adjust_char_head(mut start: *const OnigUChar,
                                           mut s: *const OnigUChar)
 -> *mut OnigUChar {
    /* In this encoding
     mb-trail bytes doesn't mix with single bytes.
  */
    let mut p: *const OnigUChar = 0 as *const OnigUChar;
    let mut len: libc::c_int = 0;
    if s <= start { return s as *mut OnigUChar }
    p = s;
    while !((*p as libc::c_int - 0xa1 as libc::c_int) as OnigUChar as
                libc::c_int > 0xfe as libc::c_int - 0xa1 as libc::c_int) &&
              p > start {
        p = p.offset(-1)
    }
    len =
        OnigEncodingEUC_JP.mbc_enc_len.expect("non-null function pointer")(p);
    if p.offset(len as isize) > s { return p as *mut OnigUChar }
    p = p.offset(len as isize);
    return p.offset((s.wrapping_offset_from(p) as libc::c_long &
                         !(1 as libc::c_int) as libc::c_long) as isize) as
               *mut OnigUChar;
}
unsafe extern "C" fn is_allowed_reverse_match(mut s: *const OnigUChar,
                                              mut end: *const OnigUChar)
 -> libc::c_int {
    let c: OnigUChar = *s;
    if c as libc::c_int <= 0x7e as libc::c_int ||
           c as libc::c_int == 0x8e as libc::c_int ||
           c as libc::c_int == 0x8f as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
static mut CR_Hiragana: [OnigCodePoint; 3] =
    [1 as libc::c_int as OnigCodePoint,
     0xa4a1 as libc::c_int as OnigCodePoint,
     0xa4f3 as libc::c_int as OnigCodePoint];
/* CR_Hiragana */
static mut CR_Katakana: [OnigCodePoint; 7] =
    [3 as libc::c_int as OnigCodePoint,
     0xa5a1 as libc::c_int as OnigCodePoint,
     0xa5f6 as libc::c_int as OnigCodePoint,
     0xaaa6 as libc::c_int as OnigCodePoint,
     0xaaaf as libc::c_int as OnigCodePoint,
     0xaab1 as libc::c_int as OnigCodePoint,
     0xaadd as libc::c_int as OnigCodePoint];
/* CR_Katakana */
static mut PropertyList: [*const OnigCodePoint; 2] =
    unsafe { [CR_Hiragana.as_ptr(), CR_Katakana.as_ptr()] };
unsafe extern "C" fn property_name_to_ctype(mut enc: OnigEncoding,
                                            mut p: *mut OnigUChar,
                                            mut end: *mut OnigUChar)
 -> libc::c_int {
    let mut pc: *mut PropertyNameCtype = 0 as *mut PropertyNameCtype;
    let mut len: libc::c_int =
        end.wrapping_offset_from(p) as libc::c_long as libc::c_int;
    let mut q: [libc::c_char; 32] = [0; 32];
    if (len as libc::c_ulong) <
           (::std::mem::size_of::<[libc::c_char; 32]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        memcpy(q.as_mut_ptr() as *mut libc::c_void, p as *const libc::c_void,
               len as size_t);
        q[len as usize] = '\u{0}' as i32 as libc::c_char;
        pc = euc_jp_lookup_property_name(q.as_mut_ptr(), len as libc::c_uint);
        if !pc.is_null() { return (*pc).ctype }
    }
    return -(223 as libc::c_int);
}
unsafe extern "C" fn is_code_ctype(mut code: OnigCodePoint,
                                   mut ctype: libc::c_uint) -> libc::c_int {
    if ctype <= 14 as libc::c_int as libc::c_uint {
        if code < 128 as libc::c_int as libc::c_uint {
            return (*OnigEncAsciiCtypeTable.as_ptr().offset(code as isize) as
                        libc::c_int & (1 as libc::c_int) << ctype !=
                        0 as libc::c_int) as libc::c_int
        } else {
            if ctype == 12 as libc::c_int as libc::c_uint ||
                   ctype == 5 as libc::c_int as libc::c_uint ||
                   ctype == 7 as libc::c_int as libc::c_uint {
                return if code_to_mbclen(code) > 1 as libc::c_int {
                           1 as libc::c_int
                       } else { 0 as libc::c_int }
            }
        }
    } else {
        ctype =
            ctype.wrapping_sub((14 as libc::c_int + 1 as libc::c_int) as
                                   libc::c_uint);
        if ctype >=
               (::std::mem::size_of::<[*const OnigCodePoint; 2]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*const OnigCodePoint>()
                                                    as libc::c_ulong) as
                   libc::c_uint {
            return -(6 as libc::c_int)
        }
        return onig_is_in_code_range(PropertyList[ctype as usize] as
                                         *mut OnigUChar, code)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_ctype_code_range(mut ctype: OnigCtype,
                                          mut sb_out: *mut OnigCodePoint,
                                          mut ranges:
                                              *mut *const OnigCodePoint)
 -> libc::c_int {
    if ctype <= 14 as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int)
    } else {
        *sb_out = 0x80 as libc::c_int as OnigCodePoint;
        ctype =
            (ctype as
                 libc::c_uint).wrapping_sub((14 as libc::c_int +
                                                 1 as libc::c_int) as
                                                libc::c_uint) as OnigCtype as
                OnigCtype;
        if ctype as libc::c_ulong >=
               (::std::mem::size_of::<[*const OnigCodePoint; 2]>() as
                    libc::c_ulong as OnigCtype as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*const OnigCodePoint>()
                                                    as libc::c_ulong) {
            return -(6 as libc::c_int)
        }
        *ranges = PropertyList[ctype as usize];
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub static mut OnigEncodingEUC_JP: OnigEncodingType =
    unsafe {
        {
            let mut init =
                OnigEncodingTypeST{mbc_enc_len:
                                       Some(mbc_enc_len as
                                                unsafe extern "C" fn(_:
                                                                         *const OnigUChar)
                                                    -> libc::c_int),
                                   name:
                                       b"EUC-JP\x00" as *const u8 as
                                           *const libc::c_char,
                                   max_enc_len: 3 as libc::c_int,
                                   min_enc_len: 1 as libc::c_int,
                                   is_mbc_newline:
                                       Some(onigenc_is_mbc_newline_0x0a as
                                                unsafe extern "C" fn(_:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar)
                                                    -> libc::c_int),
                                   mbc_to_code:
                                       Some(mbc_to_code as
                                                unsafe extern "C" fn(_:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar)
                                                    -> OnigCodePoint),
                                   code_to_mbclen:
                                       Some(code_to_mbclen as
                                                unsafe extern "C" fn(_:
                                                                         OnigCodePoint)
                                                    -> libc::c_int),
                                   code_to_mbc:
                                       Some(code_to_mbc as
                                                unsafe extern "C" fn(_:
                                                                         OnigCodePoint,
                                                                     _:
                                                                         *mut OnigUChar)
                                                    -> libc::c_int),
                                   mbc_case_fold:
                                       Some(mbc_case_fold as
                                                unsafe extern "C" fn(_:
                                                                         OnigCaseFoldType,
                                                                     _:
                                                                         *mut *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *mut OnigUChar)
                                                    -> libc::c_int),
                                   apply_all_case_fold:
                                       Some(onigenc_ascii_apply_all_case_fold
                                                as
                                                unsafe extern "C" fn(_:
                                                                         OnigCaseFoldType,
                                                                     _:
                                                                         OnigApplyAllCaseFoldFunc,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> libc::c_int),
                                   get_case_fold_codes_by_str:
                                       Some(onigenc_ascii_get_case_fold_codes_by_str
                                                as
                                                unsafe extern "C" fn(_:
                                                                         OnigCaseFoldType,
                                                                     _:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *mut OnigCaseFoldCodeItem)
                                                    -> libc::c_int),
                                   property_name_to_ctype:
                                       Some(property_name_to_ctype as
                                                unsafe extern "C" fn(_:
                                                                         OnigEncoding,
                                                                     _:
                                                                         *mut OnigUChar,
                                                                     _:
                                                                         *mut OnigUChar)
                                                    -> libc::c_int),
                                   is_code_ctype:
                                       Some(is_code_ctype as
                                                unsafe extern "C" fn(_:
                                                                         OnigCodePoint,
                                                                     _:
                                                                         libc::c_uint)
                                                    -> libc::c_int),
                                   get_ctype_code_range:
                                       Some(get_ctype_code_range as
                                                unsafe extern "C" fn(_:
                                                                         OnigCtype,
                                                                     _:
                                                                         *mut OnigCodePoint,
                                                                     _:
                                                                         *mut *const OnigCodePoint)
                                                    -> libc::c_int),
                                   left_adjust_char_head:
                                       Some(left_adjust_char_head as
                                                unsafe extern "C" fn(_:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar)
                                                    -> *mut OnigUChar),
                                   is_allowed_reverse_match:
                                       Some(is_allowed_reverse_match as
                                                unsafe extern "C" fn(_:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar)
                                                    -> libc::c_int),
                                   init: None,
                                   is_initialized: None,
                                   is_valid_mbc_string:
                                       Some(is_valid_mbc_string as
                                                unsafe extern "C" fn(_:
                                                                         *const OnigUChar,
                                                                     _:
                                                                         *const OnigUChar)
                                                    -> libc::c_int),};
            init
        }
    };
