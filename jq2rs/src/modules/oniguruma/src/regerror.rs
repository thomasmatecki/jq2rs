#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    static mut OnigEncodingASCII: OnigEncodingType;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn onigenc_str_bytelen_null(enc: OnigEncoding, p: *const OnigUChar)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
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
pub struct OnigErrorInfo {
    pub enc: OnigEncoding,
    pub par: *mut OnigUChar,
    pub par_end: *mut OnigUChar,
}
pub type va_list = __builtin_va_list;
/* *********************************************************************
  regerror.c -  Oniguruma (regular expression library)
**********************************************************************/
/*-
 * Copyright (c) 2002-2007  K.Kosako  <sndgk393 AT ybb DOT ne DOT jp>
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
/* for vsnprintf() */
#[no_mangle]
pub unsafe extern "C" fn onig_error_code_to_format(mut code: libc::c_int)
 -> *mut OnigUChar {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if code >= 0 as libc::c_int { return 0 as *mut OnigUChar }
    match code {
        -1 => {
            p =
                b"mismatch\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -2 => {
            p =
                b"no support in this configuration\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -5 => {
            p =
                b"fail to memory allocation\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -15 => {
            p =
                b"match-stack limit over\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -16 => {
            p =
                b"parse depth limit over\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -6 => {
            p =
                b"undefined type (bug)\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -11 => {
            p =
                b"internal parser error (bug)\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -12 => {
            p =
                b"stack error (bug)\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -13 => {
            p =
                b"undefined bytecode (bug)\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -14 => {
            p =
                b"unexpected bytecode (bug)\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -21 => {
            p =
                b"default multibyte-encoding is not setted\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char
        }
        -22 => {
            p =
                b"can\'t convert to wide-char on specified multibyte-encoding\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        -23 => {
            p =
                b"fail to initialize\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -30 => {
            p =
                b"invalid argument\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -100 => {
            p =
                b"end pattern at left brace\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -101 => {
            p =
                b"end pattern at left bracket\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -102 => {
            p =
                b"empty char-class\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -103 => {
            p =
                b"premature end of char-class\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -104 => {
            p =
                b"end pattern at escape\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -105 => {
            p =
                b"end pattern at meta\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -106 => {
            p =
                b"end pattern at control\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -108 => {
            p =
                b"invalid meta-code syntax\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -109 => {
            p =
                b"invalid control-code syntax\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -110 => {
            p =
                b"char-class value at end of range\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -111 => {
            p =
                b"char-class value at start of range\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -112 => {
            p =
                b"unmatched range specifier in char-class\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -113 => {
            p =
                b"target of repeat operator is not specified\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char
        }
        -114 => {
            p =
                b"target of repeat operator is invalid\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -115 => {
            p =
                b"nested repeat operator\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -116 => {
            p =
                b"unmatched close parenthesis\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -117 => {
            p =
                b"end pattern with unmatched parenthesis\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -118 => {
            p =
                b"end pattern in group\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -119 => {
            p =
                b"undefined group option\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -121 => {
            p =
                b"invalid POSIX bracket type\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -122 => {
            p =
                b"invalid pattern in look-behind\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -123 => {
            p =
                b"invalid repeat range {lower,upper}\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -200 => {
            p =
                b"too big number\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -201 => {
            p =
                b"too big number for repeat range\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -202 => {
            p =
                b"upper is smaller than lower in repeat range\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char
        }
        -203 => {
            p =
                b"empty range in char class\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -204 => {
            p =
                b"mismatch multibyte code length in char-class range\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char
        }
        -205 => {
            p =
                b"too many multibyte code ranges are specified\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char
        }
        -206 => {
            p =
                b"too short multibyte code string\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -207 => {
            p =
                b"too big backref number\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -208 => {
            p =
                b"invalid backref number/name\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -209 => {
            p =
                b"numbered backref/call is not allowed. (use name)\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char
        }
        -210 => {
            p =
                b"too many captures\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -401 => {
            p =
                b"too big wide-char value\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -212 => {
            p =
                b"too long wide-char value\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -400 => {
            p =
                b"invalid code point value\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -214 => {
            p =
                b"group name is empty\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -215 => {
            p =
                b"invalid group name <%n>\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -216 => {
            p =
                b"invalid char in group name <%n>\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -217 => {
            p =
                b"undefined name <%n> reference\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -218 => {
            p =
                b"undefined group <%n> reference\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -219 => {
            p =
                b"multiplex defined name <%n>\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -220 => {
            p =
                b"multiplex definition name <%n> call\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -221 => {
            p =
                b"never ending recursion\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -222 => {
            p =
                b"group number is too big for capture history\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char
        }
        -223 => {
            p =
                b"invalid character property name {%n}\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -402 => {
            p =
                b"not supported encoding combination\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -403 => {
            p =
                b"invalid combination of options\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -500 => {
            p =
                b"library is not initialized\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        _ => {
            p =
                b"undefined error code\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
    }
    return p as *mut OnigUChar;
}
unsafe extern "C" fn sprint_byte(mut s: *mut libc::c_char,
                                 mut v: libc::c_uint) {
    snprintf(s, 3 as libc::c_int as libc::c_ulong,
             b"%02x\x00" as *const u8 as *const libc::c_char,
             v & 0o377 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn sprint_byte_with_x(mut s: *mut libc::c_char,
                                        mut v: libc::c_uint) {
    snprintf(s, 5 as libc::c_int as libc::c_ulong,
             b"\\x%02x\x00" as *const u8 as *const libc::c_char,
             v & 0o377 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn to_ascii(mut enc: OnigEncoding, mut s: *mut OnigUChar,
                              mut end: *mut OnigUChar,
                              mut buf: *mut OnigUChar,
                              mut buf_size: libc::c_int,
                              mut is_over: *mut libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut code: OnigCodePoint = 0;
    if (*enc).min_enc_len > 1 as libc::c_int {
        p = s;
        len = 0 as libc::c_int;
        while p < end {
            code =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            if code >= 0x80 as libc::c_int as libc::c_uint {
                if code > 0xffff as libc::c_int as libc::c_uint &&
                       len + 10 as libc::c_int <= buf_size {
                    sprint_byte_with_x(&mut *buf.offset(len as isize) as
                                           *mut OnigUChar as
                                           *mut libc::c_char,
                                       code >> 24 as libc::c_int);
                    sprint_byte(&mut *buf.offset((len + 4 as libc::c_int) as
                                                     isize) as *mut OnigUChar
                                    as *mut libc::c_char,
                                code >> 16 as libc::c_int);
                    sprint_byte(&mut *buf.offset((len + 6 as libc::c_int) as
                                                     isize) as *mut OnigUChar
                                    as *mut libc::c_char,
                                code >> 8 as libc::c_int);
                    sprint_byte(&mut *buf.offset((len + 8 as libc::c_int) as
                                                     isize) as *mut OnigUChar
                                    as *mut libc::c_char, code);
                    len += 10 as libc::c_int
                } else {
                    if !(len + 6 as libc::c_int <= buf_size) { break ; }
                    sprint_byte_with_x(&mut *buf.offset(len as isize) as
                                           *mut OnigUChar as
                                           *mut libc::c_char,
                                       code >> 8 as libc::c_int);
                    sprint_byte(&mut *buf.offset((len + 4 as libc::c_int) as
                                                     isize) as *mut OnigUChar
                                    as *mut libc::c_char, code);
                    len += 6 as libc::c_int
                }
            } else {
                let fresh0 = len;
                len = len + 1;
                *buf.offset(fresh0 as isize) = code as OnigUChar
            }
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if len >= buf_size { break ; }
        }
        *is_over = if p < end { 1 as libc::c_int } else { 0 as libc::c_int }
    } else {
        len =
            if end.wrapping_offset_from(s) as libc::c_long >
                   buf_size as libc::c_long {
                buf_size as libc::c_long
            } else { end.wrapping_offset_from(s) as libc::c_long } as
                libc::c_int;
        memcpy(buf as *mut libc::c_void, s as *const libc::c_void,
               len as size_t);
        *is_over =
            if (buf_size as libc::c_long) <
                   end.wrapping_offset_from(s) as libc::c_long {
                1 as libc::c_int
            } else { 0 as libc::c_int }
    }
    return len;
}
/* predefined syntaxes (see regsyntax.c) */
/* default syntax */
/* syntax (operators) */
/* . */
/* * */
/* + */
/* ? */
/* {lower,upper} */
/* \{lower,upper\} */
/* | */
/* \| */
/* (...)   */
/* \(...\) */
/* \A, \Z, \z */
/* \G     */
/* \num   */
/* [...]  */
/* \w, \W */
/* \<. \> */
/* \b, \B */
/* \s, \S */
/* \d, \D */
/* ^, $   */
/* [:xxxx:] */
/* ??,*?,+?,{n,m}? */
/* \n,\r,\t,\a ... */
/* \cx  */
/* \OOO */
/* \xHH */
/* \x{7HHHHHHH} */
/* \Q...\E */
/* (?...) */
/* (?imsx),(?-imsx) */
/* (?imx), (?-imx)  */
/* ?+,*+,++ */
/* {n,m}+   */
/* [...&&..[..]..] */
/* (?<name>...) */
/* \k<name> */
/* \g<name>, \g<n> */
/* (?@..),(?@<x>..) */
/* \C-x */
/* \M-x */
/* \v as VTAB */
/* \uHHHH */
/* \`, \' */
/* \p{...}, \P{...} */
/* \p{^..}, \P{^..} */
/* #define ONIG_SYN_OP2_CHAR_PROPERTY_PREFIX_IS (1U<<18) */
/* \h, \H */
/* \ */
/* syntax (behavior) */
/* not implemented */
/* ?, *, +, {n,m} */
/* error or ignore */
/* ...)... */
/* {??? */
/* {,n} => {0,n} */
/* /(\1)/,/\1()/ ..*/
/* (?<=a|bc) */
/* see doc/RE */
/* (?<x>)(?<x>) */
/* a{n}?=(?:a{n})? */
/* syntax (behavior) in char class [...] */
/* [^...] */
/* [..\w..] etc.. */
/* [0-9-a]=[0-9\-a] */
/* syntax (behavior) warning */
/* [,-,] */
/* (?:a*)+ */
/* meta character specifiers (onig_set_meta_char()) */
/* error codes */
/* normal return */
/* internal error */
/* general error */
/* syntax error */
/* values error (syntax error) */
/* errors related to thread */
/* #define ONIGERR_OVER_THREAD_PASS_LIMIT_COUNT                -1001 */
/* must be smaller than BIT_STATUS_BITS_NUM (unsigned int * 8) */
/* group number */
/* match result region type */
/* extended */
/* capture history tree root */
/* capture tree traverse */
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
/* Oniguruma Native API */
/* onig_init(): deprecated function. Use onig_initialize(). */
#[no_mangle]
pub unsafe extern "C" fn onig_error_code_to_str(mut s: *mut OnigUChar,
                                                mut code: libc::c_int,
                                                mut args: ...)
 -> libc::c_int {
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut einfo: *mut OnigErrorInfo = 0 as *mut OnigErrorInfo;
    let mut len: libc::c_int = 0;
    let mut is_over: libc::c_int = 0;
    let mut parbuf: [OnigUChar; 30] = [0; 30];
    let mut vargs: ::std::ffi::VaListImpl;
    vargs = args.clone();
    match code {
        -217 | -218 | -219 | -220 | -215 | -216 | -223 => {
            einfo = vargs.arg::<*mut OnigErrorInfo>();
            len =
                to_ascii((*einfo).enc, (*einfo).par, (*einfo).par_end,
                         parbuf.as_mut_ptr(),
                         30 as libc::c_int - 3 as libc::c_int, &mut is_over);
            q = onig_error_code_to_format(code);
            p = s;
            while *q as libc::c_int != '\u{0}' as i32 {
                let mut current_block_14: u64;
                if *q as libc::c_int == '%' as i32 {
                    q = q.offset(1);
                    if *q as libc::c_int == 'n' as i32 {
                        /* '%n': name */
                        memcpy(p as *mut libc::c_void,
                               parbuf.as_mut_ptr() as *const libc::c_void,
                               len as libc::c_ulong);
                        p = p.offset(len as isize);
                        if is_over != 0 as libc::c_int {
                            memcpy(p as *mut libc::c_void,
                                   b"...\x00" as *const u8 as
                                       *const libc::c_char as
                                       *const libc::c_void,
                                   3 as libc::c_int as libc::c_ulong);
                            p = p.offset(3 as libc::c_int as isize)
                        }
                        q = q.offset(1);
                        current_block_14 = 10652014663920648156;
                    } else { current_block_14 = 1983168500538454261; }
                } else { current_block_14 = 1983168500538454261; }
                match current_block_14 {
                    1983168500538454261 => {
                        let fresh1 = q;
                        q = q.offset(1);
                        let fresh2 = p;
                        p = p.offset(1);
                        *fresh2 = *fresh1
                    }
                    _ => { }
                }
            }
            *p = '\u{0}' as i32 as OnigUChar;
            len = p.wrapping_offset_from(s) as libc::c_long as libc::c_int
        }
        _ => {
            q = onig_error_code_to_format(code);
            len = onigenc_str_bytelen_null(&mut OnigEncodingASCII, q);
            memcpy(s as *mut libc::c_void, q as *const libc::c_void,
                   len as libc::c_ulong);
            *s.offset(len as isize) = '\u{0}' as i32 as OnigUChar
        }
    }
    return len;
}
/* *********************************************************************
  regint.h -  Oniguruma (regular expression library)
**********************************************************************/
/*-
 * Copyright (c) 2002-2013  K.Kosako  <sndgk393 AT ybb DOT ne DOT jp>
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
/* for debug */
/* #define ONIG_DEBUG_PARSE_TREE */
/* #define ONIG_DEBUG_COMPILE */
/* #define ONIG_DEBUG_SEARCH */
/* #define ONIG_DEBUG_MATCH */
/* #define ONIG_DONT_OPTIMIZE */
/* for byte-code statistical data. */
/* #define ONIG_DEBUG_STATISTICS */
/* config */
/* spec. config */
/* \k<name+n>, \k<name-n> */
/* /(?:()|())*\2/ */
/* /\n$/ =~ "\n" */
/* !!! moved to regenc.h. */
/* #define USE_CRNL_AS_LINE_TERMINATOR */
/* internal config */
/* unlimited */
/* */
/* escape other system UChar definition */
/* "\<", "\>" */
/* #define USE_COMBINATION_EXPLOSION_CHECK */
/* (X*)* */
/* */
/* PLATFORM_UNALIGNED_WORD_ACCESS */
/* stack pop level */
/* optimize flags */
/* Slow Search */
/* Boyer Moore Search */
/* BM   (but not simple match) */
/* Slow Search (ignore case) */
/* char map */
/* bit status */
/* OP_SET_OPTION is required for these options.
#define IS_DYNAMIC_OPTION(option) \
  (((option) & (ONIG_OPTION_MULTILINE | ONIG_OPTION_IGNORECASE)) != 0)
*/
/* ignore-case and multibyte status are included in compiled code. */
/* bitset */
/* bytes buffer */
/* from < to */
/* from > to */
/* from > to */
/* ".*" optimize info */
/* ".*" optimize info (multi-line) */
/* operation code */
/* matching process terminator (no more alternative) */
/* pattern code terminator (success end) */
/* single byte, N = 1 */
/* single byte, N = 2 */
/* single byte, N = 3 */
/* single byte, N = 4 */
/* single byte, N = 5 */
/* single byte */
/* mb-length = 2 N = 1 */
/* mb-length = 2 N = 2 */
/* mb-length = 2 N = 3 */
/* mb-length = 2 */
/* mb-length = 3 */
/* other length */
/* single byte, N = 1, ignore case */
/* single byte,        ignore case */
/* pointer to CClassNode node */
/* "."  */
/* "."  multi-line */
/* ".*" */
/* ".*" multi-line */
/* \k<xxx+n>, \k<xxx-n> */
/* push back-tracker to stack */
/* push back-tracker to stack */
/* push back-tracker to stack */
/* push marker to stack */
/* pop stack and move */
/* if match exact then push, else jump. */
/* if match exact then push, else none. */
/* {n,m} */
/* {n,m}? (non greedy) */
/* non greedy */
/* search and get in stack */
/* search and get in stack (non greedy) */
/* null loop checker start */
/* null loop checker end   */
/* null loop checker end (with capture status) */
/* with capture status and push check-end */
/* (?=...)  start */
/* (?=...)  end   */
/* (?!...)  start */
/* (?!...)  end   */
/* (?>...)  start */
/* (?>...)  end   */
/* (?<=...) start (no needs end opcode) */
/* (?<!...) start */
/* (?<!...) end   */
/* \g<name> */
/* combination explosion check and push */
/* check ok -> push, else jump  */
/* check only */
/* no need: IS_DYNAMIC_OPTION() == 0 */
/* set option and push recover option */
/* set option */
/* code point's address must be aligned address. */
/* op-code + arg size */
/* cclass node */
/* struct _Node* next; */
  /* unsigned int flags; */
/* multi-byte info or NULL */
/* byte code position */
/* string position */
/* previous char position of pstr */
/* for OP_REPEAT_INC, OP_REPEAT_INC_NG */
/* byte code position (head of repeated target) */
/* repeat id */
/* index of stack */
/* memory num */
/* start/end position */
/* Following information is set, if this stack type is MEM-START */
/* prev. info (for backtrack  "(...)*" ) */
/* prev. info (for backtrack  "(...)*" ) */
/* null check id */
/* start position */
/* byte code position */
/* null check id */
/* string position */
/* search start position (for \G: BEGIN_POSITION) */
/* for ONIG_OPTION_FIND_LONGEST */
#[no_mangle]
pub unsafe extern "C" fn onig_snprintf_with_pattern(mut buf: *mut OnigUChar,
                                                    mut bufsize: libc::c_int,
                                                    mut enc: OnigEncoding,
                                                    mut pat: *mut OnigUChar,
                                                    mut pat_end:
                                                        *mut OnigUChar,
                                                    mut fmt: *const OnigUChar,
                                                    mut args: ...) {
    let mut n: libc::c_int = 0;
    let mut need: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut bp: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut bs: [OnigUChar; 6] = [0; 6];
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    n =
        vsnprintf(buf as *mut libc::c_char, bufsize as libc::c_ulong,
                  fmt as *const libc::c_char, args_0.as_va_list());
    need =
        (pat_end.wrapping_offset_from(pat) as libc::c_long *
             4 as libc::c_int as libc::c_long +
             4 as libc::c_int as libc::c_long) as libc::c_int;
    if n + need < bufsize {
        strcat(buf as *mut libc::c_char,
               b": /\x00" as *const u8 as *const libc::c_char);
        s =
            buf.offset(onigenc_str_bytelen_null(&mut OnigEncodingASCII,
                                                buf as *const OnigUChar) as
                           isize);
        p = pat;
        while p < pat_end {
            if (*enc).mbc_enc_len.expect("non-null function pointer")(p) !=
                   1 as libc::c_int {
                len =
                    (*enc).mbc_enc_len.expect("non-null function pointer")(p);
                if (*enc).min_enc_len == 1 as libc::c_int {
                    loop  {
                        let fresh3 = len;
                        len = len - 1;
                        if !(fresh3 > 0 as libc::c_int) { break ; }
                        let fresh4 = p;
                        p = p.offset(1);
                        let fresh5 = s;
                        s = s.offset(1);
                        *fresh5 = *fresh4
                    }
                } else {
                    /* for UTF16/32 */
                    let mut blen: libc::c_int = 0;
                    loop  {
                        let fresh6 = len;
                        len = len - 1;
                        if !(fresh6 > 0 as libc::c_int) { break ; }
                        let fresh7 = p;
                        p = p.offset(1);
                        sprint_byte_with_x(bs.as_mut_ptr() as
                                               *mut libc::c_char,
                                           *fresh7 as libc::c_uint);
                        blen =
                            onigenc_str_bytelen_null(&mut OnigEncodingASCII,
                                                     bs.as_mut_ptr());
                        bp = bs.as_mut_ptr();
                        loop  {
                            let fresh8 = blen;
                            blen = blen - 1;
                            if !(fresh8 > 0 as libc::c_int) { break ; }
                            let fresh9 = bp;
                            bp = bp.offset(1);
                            let fresh10 = s;
                            s = s.offset(1);
                            *fresh10 = *fresh9
                        }
                    }
                }
            } else if *p as libc::c_int == '\\' as i32 {
                let fresh11 = p;
                p = p.offset(1);
                let fresh12 = s;
                s = s.offset(1);
                *fresh12 = *fresh11;
                len =
                    (*enc).mbc_enc_len.expect("non-null function pointer")(p);
                loop  {
                    let fresh13 = len;
                    len = len - 1;
                    if !(fresh13 > 0 as libc::c_int) { break ; }
                    let fresh14 = p;
                    p = p.offset(1);
                    let fresh15 = s;
                    s = s.offset(1);
                    *fresh15 = *fresh14
                }
            } else if *p as libc::c_int == '/' as i32 {
                let fresh16 = s;
                s = s.offset(1);
                *fresh16 = '\\' as i32 as libc::c_uchar;
                let fresh17 = p;
                p = p.offset(1);
                let fresh18 = s;
                s = s.offset(1);
                *fresh18 = *fresh17
            } else if (*enc).is_code_ctype.expect("non-null function pointer")(*p
                                                                                   as
                                                                                   OnigCodePoint,
                                                                               7
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   OnigCtype)
                          == 0 &&
                          (*enc).is_code_ctype.expect("non-null function pointer")(*p
                                                                                       as
                                                                                       OnigCodePoint,
                                                                                   9
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       OnigCtype)
                              == 0 {
                let fresh19 = p;
                p = p.offset(1);
                sprint_byte_with_x(bs.as_mut_ptr() as *mut libc::c_char,
                                   *fresh19 as libc::c_uint);
                len =
                    onigenc_str_bytelen_null(&mut OnigEncodingASCII,
                                             bs.as_mut_ptr());
                bp = bs.as_mut_ptr();
                loop  {
                    let fresh20 = len;
                    len = len - 1;
                    if !(fresh20 > 0 as libc::c_int) { break ; }
                    let fresh21 = bp;
                    bp = bp.offset(1);
                    let fresh22 = s;
                    s = s.offset(1);
                    *fresh22 = *fresh21
                }
            } else {
                let fresh23 = p;
                p = p.offset(1);
                let fresh24 = s;
                s = s.offset(1);
                *fresh24 = *fresh23
            }
        }
        let fresh25 = s;
        s = s.offset(1);
        *fresh25 = '/' as i32 as OnigUChar;
        *s = '\u{0}' as i32 as OnigUChar
    };
}
