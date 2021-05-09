#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    static mut OnigEncodingASCII: OnigEncodingType;
}
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
pub struct OnigPairCaseFoldCodes {
    pub from: OnigCodePoint,
    pub to: OnigCodePoint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosixBracketEntryType {
    pub name: *mut OnigUChar,
    pub ctype: libc::c_int,
    pub len: libc::c_short,
}
/* *********************************************************************
  regenc.c -  Oniguruma (regular expression library)
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
#[no_mangle]
pub static mut OnigEncDefaultCharEncoding: OnigEncoding =
    unsafe {
        &OnigEncodingASCII as *const OnigEncodingType as *mut OnigEncodingType
    };
#[no_mangle]
pub unsafe extern "C" fn onigenc_init() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_initialize_encoding(mut enc: OnigEncoding)
 -> libc::c_int {
    if (*enc).init.is_some() &&
           (*enc).is_initialized.expect("non-null function pointer")() ==
               0 as libc::c_int {
        let mut r: libc::c_int =
            (*enc).init.expect("non-null function pointer")();
        return r
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_get_default_encoding() -> OnigEncoding {
    return OnigEncDefaultCharEncoding;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_set_default_encoding(mut enc: OnigEncoding)
 -> libc::c_int {
    OnigEncDefaultCharEncoding = enc;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_get_right_adjust_char_head(mut enc:
                                                                OnigEncoding,
                                                            mut start:
                                                                *const OnigUChar,
                                                            mut s:
                                                                *const OnigUChar)
 -> *mut OnigUChar {
    let mut p: *mut OnigUChar =
        (*enc).left_adjust_char_head.expect("non-null function pointer")(start,
                                                                         s);
    if p < s as *mut OnigUChar {
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize)
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_get_right_adjust_char_head_with_prev(mut enc:
                                                                          OnigEncoding,
                                                                      mut start:
                                                                          *const OnigUChar,
                                                                      mut s:
                                                                          *const OnigUChar,
                                                                      mut prev:
                                                                          *mut *const OnigUChar)
 -> *mut OnigUChar {
    let mut p: *mut OnigUChar =
        (*enc).left_adjust_char_head.expect("non-null function pointer")(start,
                                                                         s);
    if p < s as *mut OnigUChar {
        if !prev.is_null() { *prev = p as *const OnigUChar }
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize)
    } else if !prev.is_null() {
        *prev = 0 as *mut libc::c_void as *const OnigUChar
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_get_prev_char_head(mut enc: OnigEncoding,
                                                    mut start:
                                                        *const OnigUChar,
                                                    mut s: *const OnigUChar)
 -> *mut OnigUChar {
    if s <= start { return 0 as *mut libc::c_void as *mut OnigUChar }
    return (*enc).left_adjust_char_head.expect("non-null function pointer")(start,
                                                                            s.offset(-(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize)));
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_step_back(mut enc: OnigEncoding,
                                           mut start: *const OnigUChar,
                                           mut s: *const OnigUChar,
                                           mut n: libc::c_int)
 -> *mut OnigUChar {
    while !(s as *mut libc::c_void).is_null() &&
              { let fresh0 = n; n = n - 1; (fresh0) > 0 as libc::c_int } {
        if s <= start { return 0 as *mut libc::c_void as *mut OnigUChar }
        s =
            (*enc).left_adjust_char_head.expect("non-null function pointer")(start,
                                                                             s.offset(-(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)))
    }
    return s as *mut OnigUChar;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_step(mut enc: OnigEncoding,
                                      mut p: *const OnigUChar,
                                      mut end: *const OnigUChar,
                                      mut n: libc::c_int) -> *mut OnigUChar {
    let mut q: *mut OnigUChar = p as *mut OnigUChar;
    loop  {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 > 0 as libc::c_int) { break ; }
        q =
            q.offset((*enc).mbc_enc_len.expect("non-null function pointer")(q)
                         as isize)
    }
    return if q <= end as *mut OnigUChar { q } else { 0 as *mut OnigUChar };
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_strlen(mut enc: OnigEncoding,
                                        mut p: *const OnigUChar,
                                        mut end: *const OnigUChar)
 -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut q: *mut OnigUChar = p as *mut OnigUChar;
    while q < end as *mut OnigUChar {
        q =
            q.offset((*enc).mbc_enc_len.expect("non-null function pointer")(q)
                         as isize);
        n += 1
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_strlen_null(mut enc: OnigEncoding,
                                             mut s: *const OnigUChar)
 -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut p: *mut OnigUChar = s as *mut OnigUChar;
    loop  {
        if *p as libc::c_int == '\u{0}' as i32 {
            let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
            let mut len: libc::c_int = (*enc).min_enc_len;
            if len == 1 as libc::c_int { return n }
            q = p.offset(1 as libc::c_int as isize);
            while len > 1 as libc::c_int {
                if *q as libc::c_int != '\u{0}' as i32 { break ; }
                q = q.offset(1);
                len -= 1
            }
            if len == 1 as libc::c_int { return n }
        }
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        n += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_str_bytelen_null(mut enc: OnigEncoding,
                                                  mut s: *const OnigUChar)
 -> libc::c_int {
    let mut start: *mut OnigUChar = s as *mut OnigUChar;
    let mut p: *mut OnigUChar = s as *mut OnigUChar;
    loop  {
        if *p as libc::c_int == '\u{0}' as i32 {
            let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
            let mut len: libc::c_int = (*enc).min_enc_len;
            if len == 1 as libc::c_int {
                return p.wrapping_offset_from(start) as libc::c_long as
                           libc::c_int
            }
            q = p.offset(1 as libc::c_int as isize);
            while len > 1 as libc::c_int {
                if *q as libc::c_int != '\u{0}' as i32 { break ; }
                q = q.offset(1);
                len -= 1
            }
            if len == 1 as libc::c_int {
                return p.wrapping_offset_from(start) as libc::c_long as
                           libc::c_int
            }
        }
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize)
    };
}
#[no_mangle]
pub static mut OnigEncAsciiToLowerCaseTable: [OnigUChar; 256] =
    ['\u{0}' as i32 as OnigUChar, '\u{1}' as i32 as OnigUChar,
     '\u{2}' as i32 as OnigUChar, '\u{3}' as i32 as OnigUChar,
     '\u{4}' as i32 as OnigUChar, '\u{5}' as i32 as OnigUChar,
     '\u{6}' as i32 as OnigUChar, '\u{7}' as i32 as OnigUChar,
     '\u{8}' as i32 as OnigUChar, '\t' as i32 as OnigUChar,
     '\n' as i32 as OnigUChar, '\u{b}' as i32 as OnigUChar,
     '\u{c}' as i32 as OnigUChar, '\r' as i32 as OnigUChar,
     '\u{e}' as i32 as OnigUChar, '\u{f}' as i32 as OnigUChar,
     '\u{10}' as i32 as OnigUChar, '\u{11}' as i32 as OnigUChar,
     '\u{12}' as i32 as OnigUChar, '\u{13}' as i32 as OnigUChar,
     '\u{14}' as i32 as OnigUChar, '\u{15}' as i32 as OnigUChar,
     '\u{16}' as i32 as OnigUChar, '\u{17}' as i32 as OnigUChar,
     '\u{18}' as i32 as OnigUChar, '\u{19}' as i32 as OnigUChar,
     '\u{1a}' as i32 as OnigUChar, '\u{1b}' as i32 as OnigUChar,
     '\u{1c}' as i32 as OnigUChar, '\u{1d}' as i32 as OnigUChar,
     '\u{1e}' as i32 as OnigUChar, '\u{1f}' as i32 as OnigUChar,
     ' ' as i32 as OnigUChar, '!' as i32 as OnigUChar,
     '\"' as i32 as OnigUChar, '#' as i32 as OnigUChar,
     '$' as i32 as OnigUChar, '%' as i32 as OnigUChar,
     '&' as i32 as OnigUChar, '\'' as i32 as OnigUChar,
     '(' as i32 as OnigUChar, ')' as i32 as OnigUChar,
     '*' as i32 as OnigUChar, '+' as i32 as OnigUChar,
     ',' as i32 as OnigUChar, '-' as i32 as OnigUChar,
     '.' as i32 as OnigUChar, '/' as i32 as OnigUChar,
     '0' as i32 as OnigUChar, '1' as i32 as OnigUChar,
     '2' as i32 as OnigUChar, '3' as i32 as OnigUChar,
     '4' as i32 as OnigUChar, '5' as i32 as OnigUChar,
     '6' as i32 as OnigUChar, '7' as i32 as OnigUChar,
     '8' as i32 as OnigUChar, '9' as i32 as OnigUChar,
     ':' as i32 as OnigUChar, ';' as i32 as OnigUChar,
     '<' as i32 as OnigUChar, '=' as i32 as OnigUChar,
     '>' as i32 as OnigUChar, '?' as i32 as OnigUChar,
     '@' as i32 as OnigUChar, 'a' as i32 as OnigUChar,
     'b' as i32 as OnigUChar, 'c' as i32 as OnigUChar,
     'd' as i32 as OnigUChar, 'e' as i32 as OnigUChar,
     'f' as i32 as OnigUChar, 'g' as i32 as OnigUChar,
     'h' as i32 as OnigUChar, 'i' as i32 as OnigUChar,
     'j' as i32 as OnigUChar, 'k' as i32 as OnigUChar,
     'l' as i32 as OnigUChar, 'm' as i32 as OnigUChar,
     'n' as i32 as OnigUChar, 'o' as i32 as OnigUChar,
     'p' as i32 as OnigUChar, 'q' as i32 as OnigUChar,
     'r' as i32 as OnigUChar, 's' as i32 as OnigUChar,
     't' as i32 as OnigUChar, 'u' as i32 as OnigUChar,
     'v' as i32 as OnigUChar, 'w' as i32 as OnigUChar,
     'x' as i32 as OnigUChar, 'y' as i32 as OnigUChar,
     'z' as i32 as OnigUChar, '[' as i32 as OnigUChar,
     '\\' as i32 as OnigUChar, ']' as i32 as OnigUChar,
     '^' as i32 as OnigUChar, '_' as i32 as OnigUChar,
     '`' as i32 as OnigUChar, 'a' as i32 as OnigUChar,
     'b' as i32 as OnigUChar, 'c' as i32 as OnigUChar,
     'd' as i32 as OnigUChar, 'e' as i32 as OnigUChar,
     'f' as i32 as OnigUChar, 'g' as i32 as OnigUChar,
     'h' as i32 as OnigUChar, 'i' as i32 as OnigUChar,
     'j' as i32 as OnigUChar, 'k' as i32 as OnigUChar,
     'l' as i32 as OnigUChar, 'm' as i32 as OnigUChar,
     'n' as i32 as OnigUChar, 'o' as i32 as OnigUChar,
     'p' as i32 as OnigUChar, 'q' as i32 as OnigUChar,
     'r' as i32 as OnigUChar, 's' as i32 as OnigUChar,
     't' as i32 as OnigUChar, 'u' as i32 as OnigUChar,
     'v' as i32 as OnigUChar, 'w' as i32 as OnigUChar,
     'x' as i32 as OnigUChar, 'y' as i32 as OnigUChar,
     'z' as i32 as OnigUChar, '{' as i32 as OnigUChar,
     '|' as i32 as OnigUChar, '}' as i32 as OnigUChar,
     '~' as i32 as OnigUChar, '\u{7f}' as i32 as OnigUChar,
     -128i32 as OnigUChar, -127i32 as OnigUChar, -126i32 as OnigUChar,
     -125i32 as OnigUChar, -124i32 as OnigUChar, -123i32 as OnigUChar,
     -122i32 as OnigUChar, -121i32 as OnigUChar, -120i32 as OnigUChar,
     -119i32 as OnigUChar, -118i32 as OnigUChar, -117i32 as OnigUChar,
     -116i32 as OnigUChar, -115i32 as OnigUChar, -114i32 as OnigUChar,
     -113i32 as OnigUChar, -112i32 as OnigUChar, -111i32 as OnigUChar,
     -110i32 as OnigUChar, -109i32 as OnigUChar, -108i32 as OnigUChar,
     -107i32 as OnigUChar, -106i32 as OnigUChar, -105i32 as OnigUChar,
     -104i32 as OnigUChar, -103i32 as OnigUChar, -102i32 as OnigUChar,
     -101i32 as OnigUChar, -100i32 as OnigUChar, -99i32 as OnigUChar,
     -98i32 as OnigUChar, -97i32 as OnigUChar, -96i32 as OnigUChar,
     -95i32 as OnigUChar, -94i32 as OnigUChar, -93i32 as OnigUChar,
     -92i32 as OnigUChar, -91i32 as OnigUChar, -90i32 as OnigUChar,
     -89i32 as OnigUChar, -88i32 as OnigUChar, -87i32 as OnigUChar,
     -86i32 as OnigUChar, -85i32 as OnigUChar, -84i32 as OnigUChar,
     -83i32 as OnigUChar, -82i32 as OnigUChar, -81i32 as OnigUChar,
     -80i32 as OnigUChar, -79i32 as OnigUChar, -78i32 as OnigUChar,
     -77i32 as OnigUChar, -76i32 as OnigUChar, -75i32 as OnigUChar,
     -74i32 as OnigUChar, -73i32 as OnigUChar, -72i32 as OnigUChar,
     -71i32 as OnigUChar, -70i32 as OnigUChar, -69i32 as OnigUChar,
     -68i32 as OnigUChar, -67i32 as OnigUChar, -66i32 as OnigUChar,
     -65i32 as OnigUChar, -64i32 as OnigUChar, -63i32 as OnigUChar,
     -62i32 as OnigUChar, -61i32 as OnigUChar, -60i32 as OnigUChar,
     -59i32 as OnigUChar, -58i32 as OnigUChar, -57i32 as OnigUChar,
     -56i32 as OnigUChar, -55i32 as OnigUChar, -54i32 as OnigUChar,
     -53i32 as OnigUChar, -52i32 as OnigUChar, -51i32 as OnigUChar,
     -50i32 as OnigUChar, -49i32 as OnigUChar, -48i32 as OnigUChar,
     -47i32 as OnigUChar, -46i32 as OnigUChar, -45i32 as OnigUChar,
     -44i32 as OnigUChar, -43i32 as OnigUChar, -42i32 as OnigUChar,
     -41i32 as OnigUChar, -40i32 as OnigUChar, -39i32 as OnigUChar,
     -38i32 as OnigUChar, -37i32 as OnigUChar, -36i32 as OnigUChar,
     -35i32 as OnigUChar, -34i32 as OnigUChar, -33i32 as OnigUChar,
     -32i32 as OnigUChar, -31i32 as OnigUChar, -30i32 as OnigUChar,
     -29i32 as OnigUChar, -28i32 as OnigUChar, -27i32 as OnigUChar,
     -26i32 as OnigUChar, -25i32 as OnigUChar, -24i32 as OnigUChar,
     -23i32 as OnigUChar, -22i32 as OnigUChar, -21i32 as OnigUChar,
     -20i32 as OnigUChar, -19i32 as OnigUChar, -18i32 as OnigUChar,
     -17i32 as OnigUChar, -16i32 as OnigUChar, -15i32 as OnigUChar,
     -14i32 as OnigUChar, -13i32 as OnigUChar, -12i32 as OnigUChar,
     -11i32 as OnigUChar, -10i32 as OnigUChar, -9i32 as OnigUChar,
     -8i32 as OnigUChar, -7i32 as OnigUChar, -6i32 as OnigUChar,
     -5i32 as OnigUChar, -4i32 as OnigUChar, -3i32 as OnigUChar,
     -2i32 as OnigUChar, -1i32 as OnigUChar];
#[no_mangle]
pub static mut OnigEncAsciiCtypeTable: [libc::c_ushort; 256] =
    [0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x420c as libc::c_int as libc::c_ushort,
     0x4209 as libc::c_int as libc::c_ushort,
     0x4208 as libc::c_int as libc::c_ushort,
     0x4208 as libc::c_int as libc::c_ushort,
     0x4208 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0x4284 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x78b0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x7ca2 as libc::c_int as libc::c_ushort,
     0x7ca2 as libc::c_int as libc::c_ushort,
     0x7ca2 as libc::c_int as libc::c_ushort,
     0x7ca2 as libc::c_int as libc::c_ushort,
     0x7ca2 as libc::c_int as libc::c_ushort,
     0x7ca2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x74a2 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x51a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x78e2 as libc::c_int as libc::c_ushort,
     0x78e2 as libc::c_int as libc::c_ushort,
     0x78e2 as libc::c_int as libc::c_ushort,
     0x78e2 as libc::c_int as libc::c_ushort,
     0x78e2 as libc::c_int as libc::c_ushort,
     0x78e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x70e2 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x41a0 as libc::c_int as libc::c_ushort,
     0x4008 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort,
     0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort];
#[no_mangle]
pub static mut OnigEncISO_8859_1_ToLowerCaseTable: [OnigUChar; 256] =
    ['\u{0}' as i32 as OnigUChar, '\u{1}' as i32 as OnigUChar,
     '\u{2}' as i32 as OnigUChar, '\u{3}' as i32 as OnigUChar,
     '\u{4}' as i32 as OnigUChar, '\u{5}' as i32 as OnigUChar,
     '\u{6}' as i32 as OnigUChar, '\u{7}' as i32 as OnigUChar,
     '\u{8}' as i32 as OnigUChar, '\t' as i32 as OnigUChar,
     '\n' as i32 as OnigUChar, '\u{b}' as i32 as OnigUChar,
     '\u{c}' as i32 as OnigUChar, '\r' as i32 as OnigUChar,
     '\u{e}' as i32 as OnigUChar, '\u{f}' as i32 as OnigUChar,
     '\u{10}' as i32 as OnigUChar, '\u{11}' as i32 as OnigUChar,
     '\u{12}' as i32 as OnigUChar, '\u{13}' as i32 as OnigUChar,
     '\u{14}' as i32 as OnigUChar, '\u{15}' as i32 as OnigUChar,
     '\u{16}' as i32 as OnigUChar, '\u{17}' as i32 as OnigUChar,
     '\u{18}' as i32 as OnigUChar, '\u{19}' as i32 as OnigUChar,
     '\u{1a}' as i32 as OnigUChar, '\u{1b}' as i32 as OnigUChar,
     '\u{1c}' as i32 as OnigUChar, '\u{1d}' as i32 as OnigUChar,
     '\u{1e}' as i32 as OnigUChar, '\u{1f}' as i32 as OnigUChar,
     ' ' as i32 as OnigUChar, '!' as i32 as OnigUChar,
     '\"' as i32 as OnigUChar, '#' as i32 as OnigUChar,
     '$' as i32 as OnigUChar, '%' as i32 as OnigUChar,
     '&' as i32 as OnigUChar, '\'' as i32 as OnigUChar,
     '(' as i32 as OnigUChar, ')' as i32 as OnigUChar,
     '*' as i32 as OnigUChar, '+' as i32 as OnigUChar,
     ',' as i32 as OnigUChar, '-' as i32 as OnigUChar,
     '.' as i32 as OnigUChar, '/' as i32 as OnigUChar,
     '0' as i32 as OnigUChar, '1' as i32 as OnigUChar,
     '2' as i32 as OnigUChar, '3' as i32 as OnigUChar,
     '4' as i32 as OnigUChar, '5' as i32 as OnigUChar,
     '6' as i32 as OnigUChar, '7' as i32 as OnigUChar,
     '8' as i32 as OnigUChar, '9' as i32 as OnigUChar,
     ':' as i32 as OnigUChar, ';' as i32 as OnigUChar,
     '<' as i32 as OnigUChar, '=' as i32 as OnigUChar,
     '>' as i32 as OnigUChar, '?' as i32 as OnigUChar,
     '@' as i32 as OnigUChar, 'a' as i32 as OnigUChar,
     'b' as i32 as OnigUChar, 'c' as i32 as OnigUChar,
     'd' as i32 as OnigUChar, 'e' as i32 as OnigUChar,
     'f' as i32 as OnigUChar, 'g' as i32 as OnigUChar,
     'h' as i32 as OnigUChar, 'i' as i32 as OnigUChar,
     'j' as i32 as OnigUChar, 'k' as i32 as OnigUChar,
     'l' as i32 as OnigUChar, 'm' as i32 as OnigUChar,
     'n' as i32 as OnigUChar, 'o' as i32 as OnigUChar,
     'p' as i32 as OnigUChar, 'q' as i32 as OnigUChar,
     'r' as i32 as OnigUChar, 's' as i32 as OnigUChar,
     't' as i32 as OnigUChar, 'u' as i32 as OnigUChar,
     'v' as i32 as OnigUChar, 'w' as i32 as OnigUChar,
     'x' as i32 as OnigUChar, 'y' as i32 as OnigUChar,
     'z' as i32 as OnigUChar, '[' as i32 as OnigUChar,
     '\\' as i32 as OnigUChar, ']' as i32 as OnigUChar,
     '^' as i32 as OnigUChar, '_' as i32 as OnigUChar,
     '`' as i32 as OnigUChar, 'a' as i32 as OnigUChar,
     'b' as i32 as OnigUChar, 'c' as i32 as OnigUChar,
     'd' as i32 as OnigUChar, 'e' as i32 as OnigUChar,
     'f' as i32 as OnigUChar, 'g' as i32 as OnigUChar,
     'h' as i32 as OnigUChar, 'i' as i32 as OnigUChar,
     'j' as i32 as OnigUChar, 'k' as i32 as OnigUChar,
     'l' as i32 as OnigUChar, 'm' as i32 as OnigUChar,
     'n' as i32 as OnigUChar, 'o' as i32 as OnigUChar,
     'p' as i32 as OnigUChar, 'q' as i32 as OnigUChar,
     'r' as i32 as OnigUChar, 's' as i32 as OnigUChar,
     't' as i32 as OnigUChar, 'u' as i32 as OnigUChar,
     'v' as i32 as OnigUChar, 'w' as i32 as OnigUChar,
     'x' as i32 as OnigUChar, 'y' as i32 as OnigUChar,
     'z' as i32 as OnigUChar, '{' as i32 as OnigUChar,
     '|' as i32 as OnigUChar, '}' as i32 as OnigUChar,
     '~' as i32 as OnigUChar, '\u{7f}' as i32 as OnigUChar,
     -128i32 as OnigUChar, -127i32 as OnigUChar, -126i32 as OnigUChar,
     -125i32 as OnigUChar, -124i32 as OnigUChar, -123i32 as OnigUChar,
     -122i32 as OnigUChar, -121i32 as OnigUChar, -120i32 as OnigUChar,
     -119i32 as OnigUChar, -118i32 as OnigUChar, -117i32 as OnigUChar,
     -116i32 as OnigUChar, -115i32 as OnigUChar, -114i32 as OnigUChar,
     -113i32 as OnigUChar, -112i32 as OnigUChar, -111i32 as OnigUChar,
     -110i32 as OnigUChar, -109i32 as OnigUChar, -108i32 as OnigUChar,
     -107i32 as OnigUChar, -106i32 as OnigUChar, -105i32 as OnigUChar,
     -104i32 as OnigUChar, -103i32 as OnigUChar, -102i32 as OnigUChar,
     -101i32 as OnigUChar, -100i32 as OnigUChar, -99i32 as OnigUChar,
     -98i32 as OnigUChar, -97i32 as OnigUChar, -96i32 as OnigUChar,
     -95i32 as OnigUChar, -94i32 as OnigUChar, -93i32 as OnigUChar,
     -92i32 as OnigUChar, -91i32 as OnigUChar, -90i32 as OnigUChar,
     -89i32 as OnigUChar, -88i32 as OnigUChar, -87i32 as OnigUChar,
     -86i32 as OnigUChar, -85i32 as OnigUChar, -84i32 as OnigUChar,
     -83i32 as OnigUChar, -82i32 as OnigUChar, -81i32 as OnigUChar,
     -80i32 as OnigUChar, -79i32 as OnigUChar, -78i32 as OnigUChar,
     -77i32 as OnigUChar, -76i32 as OnigUChar, -75i32 as OnigUChar,
     -74i32 as OnigUChar, -73i32 as OnigUChar, -72i32 as OnigUChar,
     -71i32 as OnigUChar, -70i32 as OnigUChar, -69i32 as OnigUChar,
     -68i32 as OnigUChar, -67i32 as OnigUChar, -66i32 as OnigUChar,
     -65i32 as OnigUChar, -32i32 as OnigUChar, -31i32 as OnigUChar,
     -30i32 as OnigUChar, -29i32 as OnigUChar, -28i32 as OnigUChar,
     -27i32 as OnigUChar, -26i32 as OnigUChar, -25i32 as OnigUChar,
     -24i32 as OnigUChar, -23i32 as OnigUChar, -22i32 as OnigUChar,
     -21i32 as OnigUChar, -20i32 as OnigUChar, -19i32 as OnigUChar,
     -18i32 as OnigUChar, -17i32 as OnigUChar, -16i32 as OnigUChar,
     -15i32 as OnigUChar, -14i32 as OnigUChar, -13i32 as OnigUChar,
     -12i32 as OnigUChar, -11i32 as OnigUChar, -10i32 as OnigUChar,
     -41i32 as OnigUChar, -8i32 as OnigUChar, -7i32 as OnigUChar,
     -6i32 as OnigUChar, -5i32 as OnigUChar, -4i32 as OnigUChar,
     -3i32 as OnigUChar, -2i32 as OnigUChar, -33i32 as OnigUChar,
     -32i32 as OnigUChar, -31i32 as OnigUChar, -30i32 as OnigUChar,
     -29i32 as OnigUChar, -28i32 as OnigUChar, -27i32 as OnigUChar,
     -26i32 as OnigUChar, -25i32 as OnigUChar, -24i32 as OnigUChar,
     -23i32 as OnigUChar, -22i32 as OnigUChar, -21i32 as OnigUChar,
     -20i32 as OnigUChar, -19i32 as OnigUChar, -18i32 as OnigUChar,
     -17i32 as OnigUChar, -16i32 as OnigUChar, -15i32 as OnigUChar,
     -14i32 as OnigUChar, -13i32 as OnigUChar, -12i32 as OnigUChar,
     -11i32 as OnigUChar, -10i32 as OnigUChar, -9i32 as OnigUChar,
     -8i32 as OnigUChar, -7i32 as OnigUChar, -6i32 as OnigUChar,
     -5i32 as OnigUChar, -4i32 as OnigUChar, -3i32 as OnigUChar,
     -2i32 as OnigUChar, -1i32 as OnigUChar];
#[no_mangle]
pub unsafe extern "C" fn onigenc_set_default_caseconv_table(mut table:
                                                                *const OnigUChar) {
    /* Sorry */
    /* nothing */
  /* obsoleted. */
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_get_left_adjust_char_head(mut enc:
                                                               OnigEncoding,
                                                           mut start:
                                                               *const OnigUChar,
                                                           mut s:
                                                               *const OnigUChar)
 -> *mut OnigUChar {
    return (*enc).left_adjust_char_head.expect("non-null function pointer")(start,
                                                                            s);
}
#[no_mangle]
pub static mut OnigAsciiLowerMap: [OnigPairCaseFoldCodes; 26] =
    [{
         let mut init =
             OnigPairCaseFoldCodes{from: 0x41 as libc::c_int as OnigCodePoint,
                                   to: 0x61 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x42 as libc::c_int as OnigCodePoint,
                                   to: 0x62 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x43 as libc::c_int as OnigCodePoint,
                                   to: 0x63 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x44 as libc::c_int as OnigCodePoint,
                                   to: 0x64 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x45 as libc::c_int as OnigCodePoint,
                                   to: 0x65 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x46 as libc::c_int as OnigCodePoint,
                                   to: 0x66 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x47 as libc::c_int as OnigCodePoint,
                                   to: 0x67 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x48 as libc::c_int as OnigCodePoint,
                                   to: 0x68 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x49 as libc::c_int as OnigCodePoint,
                                   to: 0x69 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x4a as libc::c_int as OnigCodePoint,
                                   to: 0x6a as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x4b as libc::c_int as OnigCodePoint,
                                   to: 0x6b as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x4c as libc::c_int as OnigCodePoint,
                                   to: 0x6c as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x4d as libc::c_int as OnigCodePoint,
                                   to: 0x6d as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x4e as libc::c_int as OnigCodePoint,
                                   to: 0x6e as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x4f as libc::c_int as OnigCodePoint,
                                   to: 0x6f as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x50 as libc::c_int as OnigCodePoint,
                                   to: 0x70 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x51 as libc::c_int as OnigCodePoint,
                                   to: 0x71 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x52 as libc::c_int as OnigCodePoint,
                                   to: 0x72 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x53 as libc::c_int as OnigCodePoint,
                                   to: 0x73 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x54 as libc::c_int as OnigCodePoint,
                                   to: 0x74 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x55 as libc::c_int as OnigCodePoint,
                                   to: 0x75 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x56 as libc::c_int as OnigCodePoint,
                                   to: 0x76 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x57 as libc::c_int as OnigCodePoint,
                                   to: 0x77 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x58 as libc::c_int as OnigCodePoint,
                                   to: 0x78 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x59 as libc::c_int as OnigCodePoint,
                                   to: 0x79 as libc::c_int as OnigCodePoint,};
         init
     },
     {
         let mut init =
             OnigPairCaseFoldCodes{from: 0x5a as libc::c_int as OnigCodePoint,
                                   to: 0x7a as libc::c_int as OnigCodePoint,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn onigenc_ascii_apply_all_case_fold(mut flag:
                                                               OnigCaseFoldType,
                                                           mut f:
                                                               OnigApplyAllCaseFoldFunc,
                                                           mut arg:
                                                               *mut libc::c_void)
 -> libc::c_int {
    let mut code: OnigCodePoint = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[OnigPairCaseFoldCodes; 26]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<OnigPairCaseFoldCodes>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        code = OnigAsciiLowerMap[i as usize].to;
        r =
            Some(f.expect("non-null function pointer")).expect("non-null function pointer")(OnigAsciiLowerMap[i
                                                                                                                  as
                                                                                                                  usize].from,
                                                                                            &mut code,
                                                                                            1
                                                                                                as
                                                                                                libc::c_int,
                                                                                            arg);
        if r != 0 as libc::c_int { return r }
        code = OnigAsciiLowerMap[i as usize].from;
        r =
            Some(f.expect("non-null function pointer")).expect("non-null function pointer")(OnigAsciiLowerMap[i
                                                                                                                  as
                                                                                                                  usize].to,
                                                                                            &mut code,
                                                                                            1
                                                                                                as
                                                                                                libc::c_int,
                                                                                            arg);
        if r != 0 as libc::c_int { return r }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_ascii_get_case_fold_codes_by_str(mut flag:
                                                                      OnigCaseFoldType,
                                                                  mut p:
                                                                      *const OnigUChar,
                                                                  mut end:
                                                                      *const OnigUChar,
                                                                  mut items:
                                                                      *mut OnigCaseFoldCodeItem)
 -> libc::c_int {
    if 0x41 as libc::c_int <= *p as libc::c_int &&
           *p as libc::c_int <= 0x5a as libc::c_int {
        (*items.offset(0 as libc::c_int as isize)).byte_len =
            1 as libc::c_int;
        (*items.offset(0 as libc::c_int as isize)).code_len =
            1 as libc::c_int;
        (*items.offset(0 as libc::c_int as
                           isize)).code[0 as libc::c_int as usize] =
            (*p as libc::c_int + 0x20 as libc::c_int) as OnigCodePoint;
        return 1 as libc::c_int
    } else if 0x61 as libc::c_int <= *p as libc::c_int &&
                  *p as libc::c_int <= 0x7a as libc::c_int {
        (*items.offset(0 as libc::c_int as isize)).byte_len =
            1 as libc::c_int;
        (*items.offset(0 as libc::c_int as isize)).code_len =
            1 as libc::c_int;
        (*items.offset(0 as libc::c_int as
                           isize)).code[0 as libc::c_int as usize] =
            (*p as libc::c_int - 0x20 as libc::c_int) as OnigCodePoint;
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn ss_apply_all_case_fold(mut flag: OnigCaseFoldType,
                                            mut f: OnigApplyAllCaseFoldFunc,
                                            mut arg: *mut libc::c_void)
 -> libc::c_int {
    static mut ss: [OnigCodePoint; 2] =
        [0x73 as libc::c_int as OnigCodePoint,
         0x73 as libc::c_int as OnigCodePoint];
    return Some(f.expect("non-null function pointer")).expect("non-null function pointer")(0xdf
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               OnigCodePoint,
                                                                                           ss.as_mut_ptr(),
                                                                                           2
                                                                                               as
                                                                                               libc::c_int,
                                                                                           arg);
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_apply_all_case_fold_with_map(mut map_size:
                                                                  libc::c_int,
                                                              mut map:
                                                                  *const OnigPairCaseFoldCodes,
                                                              mut ess_tsett_flag:
                                                                  libc::c_int,
                                                              mut flag:
                                                                  OnigCaseFoldType,
                                                              mut f:
                                                                  OnigApplyAllCaseFoldFunc,
                                                              mut arg:
                                                                  *mut libc::c_void)
 -> libc::c_int {
    let mut code: OnigCodePoint = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = onigenc_ascii_apply_all_case_fold(flag, f, arg);
    if r != 0 as libc::c_int { return r }
    i = 0 as libc::c_int;
    while i < map_size {
        code = (*map.offset(i as isize)).to;
        r =
            Some(f.expect("non-null function pointer")).expect("non-null function pointer")((*map.offset(i
                                                                                                             as
                                                                                                             isize)).from,
                                                                                            &mut code,
                                                                                            1
                                                                                                as
                                                                                                libc::c_int,
                                                                                            arg);
        if r != 0 as libc::c_int { return r }
        code = (*map.offset(i as isize)).from;
        r =
            Some(f.expect("non-null function pointer")).expect("non-null function pointer")((*map.offset(i
                                                                                                             as
                                                                                                             isize)).to,
                                                                                            &mut code,
                                                                                            1
                                                                                                as
                                                                                                libc::c_int,
                                                                                            arg);
        if r != 0 as libc::c_int { return r }
        i += 1
    }
    if ess_tsett_flag != 0 as libc::c_int {
        return ss_apply_all_case_fold(flag, f, arg)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_get_case_fold_codes_by_str_with_map(mut map_size:
                                                                         libc::c_int,
                                                                     mut map:
                                                                         *const OnigPairCaseFoldCodes,
                                                                     mut ess_tsett_flag:
                                                                         libc::c_int,
                                                                     mut flag:
                                                                         OnigCaseFoldType,
                                                                     mut p:
                                                                         *const OnigUChar,
                                                                     mut end:
                                                                         *const OnigUChar,
                                                                     mut items:
                                                                         *mut OnigCaseFoldCodeItem)
 -> libc::c_int {
    if 0x41 as libc::c_int <= *p as libc::c_int &&
           *p as libc::c_int <= 0x5a as libc::c_int {
        (*items.offset(0 as libc::c_int as isize)).byte_len =
            1 as libc::c_int;
        (*items.offset(0 as libc::c_int as isize)).code_len =
            1 as libc::c_int;
        (*items.offset(0 as libc::c_int as
                           isize)).code[0 as libc::c_int as usize] =
            (*p as libc::c_int + 0x20 as libc::c_int) as OnigCodePoint;
        if *p as libc::c_int == 0x53 as libc::c_int &&
               ess_tsett_flag != 0 as libc::c_int &&
               end > p.offset(1 as libc::c_int as isize) &&
               (*p.offset(1 as libc::c_int as isize) as libc::c_int ==
                    0x53 as libc::c_int ||
                    *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                        0x73 as libc::c_int) {
            /* SS */
            (*items.offset(1 as libc::c_int as isize)).byte_len =
                2 as libc::c_int;
            (*items.offset(1 as libc::c_int as isize)).code_len =
                1 as libc::c_int;
            (*items.offset(1 as libc::c_int as
                               isize)).code[0 as libc::c_int as usize] =
                0xdf as libc::c_int as OnigCodePoint;
            return 2 as libc::c_int
        } else { return 1 as libc::c_int }
    } else {
        if 0x61 as libc::c_int <= *p as libc::c_int &&
               *p as libc::c_int <= 0x7a as libc::c_int {
            (*items.offset(0 as libc::c_int as isize)).byte_len =
                1 as libc::c_int;
            (*items.offset(0 as libc::c_int as isize)).code_len =
                1 as libc::c_int;
            (*items.offset(0 as libc::c_int as
                               isize)).code[0 as libc::c_int as usize] =
                (*p as libc::c_int - 0x20 as libc::c_int) as OnigCodePoint;
            if *p as libc::c_int == 0x73 as libc::c_int &&
                   ess_tsett_flag != 0 as libc::c_int &&
                   end > p.offset(1 as libc::c_int as isize) &&
                   (*p.offset(1 as libc::c_int as isize) as libc::c_int ==
                        0x73 as libc::c_int ||
                        *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                            0x53 as libc::c_int) {
                /* ss */
                (*items.offset(1 as libc::c_int as isize)).byte_len =
                    2 as libc::c_int;
                (*items.offset(1 as libc::c_int as isize)).code_len =
                    1 as libc::c_int;
                (*items.offset(1 as libc::c_int as
                                   isize)).code[0 as libc::c_int as usize] =
                    0xdf as libc::c_int as OnigCodePoint;
                return 2 as libc::c_int
            } else { return 1 as libc::c_int }
        } else {
            if *p as libc::c_int == 0xdf as libc::c_int &&
                   ess_tsett_flag != 0 as libc::c_int {
                (*items.offset(0 as libc::c_int as isize)).byte_len =
                    1 as libc::c_int;
                (*items.offset(0 as libc::c_int as isize)).code_len =
                    2 as libc::c_int;
                (*items.offset(0 as libc::c_int as
                                   isize)).code[0 as libc::c_int as usize] =
                    's' as i32 as OnigCodePoint;
                (*items.offset(0 as libc::c_int as
                                   isize)).code[1 as libc::c_int as usize] =
                    's' as i32 as OnigCodePoint;
                (*items.offset(1 as libc::c_int as isize)).byte_len =
                    1 as libc::c_int;
                (*items.offset(1 as libc::c_int as isize)).code_len =
                    2 as libc::c_int;
                (*items.offset(1 as libc::c_int as
                                   isize)).code[0 as libc::c_int as usize] =
                    'S' as i32 as OnigCodePoint;
                (*items.offset(1 as libc::c_int as
                                   isize)).code[1 as libc::c_int as usize] =
                    'S' as i32 as OnigCodePoint;
                (*items.offset(2 as libc::c_int as isize)).byte_len =
                    1 as libc::c_int;
                (*items.offset(2 as libc::c_int as isize)).code_len =
                    2 as libc::c_int;
                (*items.offset(2 as libc::c_int as
                                   isize)).code[0 as libc::c_int as usize] =
                    's' as i32 as OnigCodePoint;
                (*items.offset(2 as libc::c_int as
                                   isize)).code[1 as libc::c_int as usize] =
                    'S' as i32 as OnigCodePoint;
                (*items.offset(3 as libc::c_int as isize)).byte_len =
                    1 as libc::c_int;
                (*items.offset(3 as libc::c_int as isize)).code_len =
                    2 as libc::c_int;
                (*items.offset(3 as libc::c_int as
                                   isize)).code[0 as libc::c_int as usize] =
                    'S' as i32 as OnigCodePoint;
                (*items.offset(3 as libc::c_int as
                                   isize)).code[1 as libc::c_int as usize] =
                    's' as i32 as OnigCodePoint;
                return 4 as libc::c_int
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < map_size {
                    if *p as libc::c_uint == (*map.offset(i as isize)).from {
                        (*items.offset(0 as libc::c_int as isize)).byte_len =
                            1 as libc::c_int;
                        (*items.offset(0 as libc::c_int as isize)).code_len =
                            1 as libc::c_int;
                        (*items.offset(0 as libc::c_int as
                                           isize)).code[0 as libc::c_int as
                                                            usize] =
                            (*map.offset(i as isize)).to;
                        return 1 as libc::c_int
                    } else {
                        if *p as libc::c_uint == (*map.offset(i as isize)).to
                           {
                            (*items.offset(0 as libc::c_int as
                                               isize)).byte_len =
                                1 as libc::c_int;
                            (*items.offset(0 as libc::c_int as
                                               isize)).code_len =
                                1 as libc::c_int;
                            (*items.offset(0 as libc::c_int as
                                               isize)).code[0 as libc::c_int
                                                                as usize] =
                                (*map.offset(i as isize)).from;
                            return 1 as libc::c_int
                        }
                    }
                    i += 1
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_not_support_get_ctype_code_range(mut ctype:
                                                                      OnigCtype,
                                                                  mut sb_out:
                                                                      *mut OnigCodePoint,
                                                                  mut ranges:
                                                                      *mut *const OnigCodePoint)
 -> libc::c_int {
    return -(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_is_mbc_newline_0x0a(mut p: *const OnigUChar,
                                                     mut end:
                                                         *const OnigUChar)
 -> libc::c_int {
    if p < end {
        if *p as libc::c_int == 0xa as libc::c_int { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/* for single byte encodings */
#[no_mangle]
pub unsafe extern "C" fn onigenc_ascii_mbc_case_fold(mut flag:
                                                         OnigCaseFoldType,
                                                     mut p:
                                                         *mut *const OnigUChar,
                                                     mut end:
                                                         *const OnigUChar,
                                                     mut lower:
                                                         *mut OnigUChar)
 -> libc::c_int {
    *lower = OnigEncAsciiToLowerCaseTable[**p as usize];
    *p = (*p).offset(1);
    return 1 as libc::c_int;
    /* return byte length of converted char to lower */
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_single_byte_mbc_enc_len(mut p:
                                                             *const OnigUChar)
 -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_single_byte_mbc_to_code(mut p:
                                                             *const OnigUChar,
                                                         mut end:
                                                             *const OnigUChar)
 -> OnigCodePoint {
    return *p as OnigCodePoint;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_single_byte_code_to_mbclen(mut code:
                                                                OnigCodePoint)
 -> libc::c_int {
    return if code < 0x100 as libc::c_int as libc::c_uint {
               1 as libc::c_int
           } else { -(400 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_single_byte_code_to_mbc(mut code:
                                                             OnigCodePoint,
                                                         mut buf:
                                                             *mut OnigUChar)
 -> libc::c_int {
    *buf = (code & 0xff as libc::c_int as libc::c_uint) as OnigUChar;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_single_byte_left_adjust_char_head(mut start:
                                                                       *const OnigUChar,
                                                                   mut s:
                                                                       *const OnigUChar)
 -> *mut OnigUChar {
    return s as *mut OnigUChar;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_always_true_is_allowed_reverse_match(mut s:
                                                                          *const OnigUChar,
                                                                      mut end:
                                                                          *const OnigUChar)
 -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_always_false_is_allowed_reverse_match(mut s:
                                                                           *const OnigUChar,
                                                                       mut end:
                                                                           *const OnigUChar)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_always_true_is_valid_mbc_string(mut s:
                                                                     *const OnigUChar,
                                                                 mut end:
                                                                     *const OnigUChar)
 -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_length_check_is_valid_mbc_string(mut enc:
                                                                      OnigEncoding,
                                                                  mut p:
                                                                      *const OnigUChar,
                                                                  mut end:
                                                                      *const OnigUChar)
 -> libc::c_int {
    while p < end {
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize)
    }
    if p != end { return 0 as libc::c_int } else { return 1 as libc::c_int };
}
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
/* case fold flag */
/* #define ONIGENC_CASE_FOLD_HIRAGANA_KATAKANA  (1<<1) */
/* #define ONIGENC_CASE_FOLD_KATAKANA_WIDTH     (1<<2) */
/* 13 => Unicode:0x1ffc */
/* code range */
/* argument(original) character(s) byte length */
/* number of code */
/* work size */
/* 18: 6(max-byte) * 3(case-fold chars) */
/* character types */
/* alpha || digit */
/* encoding API */
#[no_mangle]
pub unsafe extern "C" fn onigenc_is_valid_mbc_string(mut enc: OnigEncoding,
                                                     mut s: *const OnigUChar,
                                                     mut end:
                                                         *const OnigUChar)
 -> libc::c_int {
    return (*enc).is_valid_mbc_string.expect("non-null function pointer")(s,
                                                                          end);
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mbn_mbc_to_code(mut enc: OnigEncoding,
                                                 mut p: *const OnigUChar,
                                                 mut end: *const OnigUChar)
 -> OnigCodePoint {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut n: OnigCodePoint = 0;
    len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
    let fresh2 = p;
    p = p.offset(1);
    n = *fresh2 as OnigCodePoint;
    if len == 1 as libc::c_int { return n }
    i = 1 as libc::c_int;
    while i < len {
        if p >= end { break ; }
        let fresh3 = p;
        p = p.offset(1);
        c = *fresh3 as libc::c_int;
        n <<= 8 as libc::c_int;
        n =
            (n as libc::c_uint).wrapping_add(c as libc::c_uint) as
                OnigCodePoint as OnigCodePoint;
        i += 1
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mbn_mbc_case_fold(mut enc: OnigEncoding,
                                                   mut flag: OnigCaseFoldType,
                                                   mut pp:
                                                       *mut *const OnigUChar,
                                                   mut end: *const OnigUChar,
                                                   mut lower: *mut OnigUChar)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *const OnigUChar = *pp;
    if (*p as libc::c_int) < 128 as libc::c_int {
        *lower = OnigEncAsciiToLowerCaseTable[*p as usize];
        *pp = (*pp).offset(1);
        return 1 as libc::c_int
    } else {
        let mut i: libc::c_int = 0;
        len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
        i = 0 as libc::c_int;
        while i < len {
            let fresh4 = p;
            p = p.offset(1);
            let fresh5 = lower;
            lower = lower.offset(1);
            *fresh5 = *fresh4;
            i += 1
        }
        *pp = (*pp).offset(len as isize);
        return len
        /* return byte length of converted to lower char */
    };
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mb2_code_to_mbclen(mut code: OnigCodePoint)
 -> libc::c_int {
    if code & 0xff00 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        return 2 as libc::c_int
    } else { return 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mb4_code_to_mbclen(mut code: OnigCodePoint)
 -> libc::c_int {
    if code & 0xff000000 as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        return 4 as libc::c_int
    } else if code & 0xff0000 as libc::c_int as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
        return 3 as libc::c_int
    } else if code & 0xff00 as libc::c_int as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
        return 2 as libc::c_int
    } else { return 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mb2_code_to_mbc(mut enc: OnigEncoding,
                                                 mut code: OnigCodePoint,
                                                 mut buf: *mut OnigUChar)
 -> libc::c_int {
    let mut p: *mut OnigUChar = buf;
    if code & 0xff00 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 =
            (code >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as OnigUChar
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = (code & 0xff as libc::c_int as libc::c_uint) as OnigUChar;
    if (*enc).mbc_enc_len.expect("non-null function pointer")(buf) as
           libc::c_long != p.wrapping_offset_from(buf) as libc::c_long {
        return -(400 as libc::c_int)
    }
    return p.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mb4_code_to_mbc(mut enc: OnigEncoding,
                                                 mut code: OnigCodePoint,
                                                 mut buf: *mut OnigUChar)
 -> libc::c_int {
    let mut p: *mut OnigUChar = buf;
    if code & 0xff000000 as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 =
            (code >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as OnigUChar
    }
    if code & 0xff0000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint || p != buf {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 =
            (code >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as OnigUChar
    }
    if code & 0xff00 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint || p != buf {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 =
            (code >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as OnigUChar
    }
    let fresh11 = p;
    p = p.offset(1);
    *fresh11 = (code & 0xff as libc::c_int as libc::c_uint) as OnigUChar;
    if (*enc).mbc_enc_len.expect("non-null function pointer")(buf) as
           libc::c_long != p.wrapping_offset_from(buf) as libc::c_long {
        return -(400 as libc::c_int)
    }
    return p.wrapping_offset_from(buf) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_minimum_property_name_to_ctype(mut enc:
                                                                    OnigEncoding,
                                                                mut p:
                                                                    *mut OnigUChar,
                                                                mut end:
                                                                    *mut OnigUChar)
 -> libc::c_int {
    static mut PBS: [PosixBracketEntryType; 15] =
        [{
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Alnum\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 13 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Alpha\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 1 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Blank\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 2 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Cntrl\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 3 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Digit\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 4 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Graph\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 5 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Lower\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 6 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Print\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 7 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Punct\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 8 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Space\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 9 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Upper\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 10 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"XDigit\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 11 as libc::c_int,
                                       len:
                                           6 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"ASCII\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 14 as libc::c_int,
                                       len:
                                           5 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           b"Word\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut OnigUChar,
                                       ctype: 12 as libc::c_int,
                                       len:
                                           4 as libc::c_int as
                                               libc::c_short,};
             init
         },
         {
             let mut init =
                 PosixBracketEntryType{name:
                                           0 as *const libc::c_void as
                                               *mut libc::c_void as
                                               *mut OnigUChar,
                                       ctype: -(1 as libc::c_int),
                                       len:
                                           0 as libc::c_int as
                                               libc::c_short,};
             init
         }];
    let mut pb: *mut PosixBracketEntryType = 0 as *mut PosixBracketEntryType;
    let mut len: libc::c_int = 0;
    len = onigenc_strlen(enc, p, end);
    pb = PBS.as_mut_ptr();
    while !((*pb).name as *mut libc::c_void).is_null() {
        if len == (*pb).len as libc::c_int &&
               onigenc_with_ascii_strncmp(enc, p, end, (*pb).name,
                                          (*pb).len as libc::c_int) ==
                   0 as libc::c_int {
            return (*pb).ctype
        }
        pb = pb.offset(1)
    }
    return -(223 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mb2_is_code_ctype(mut enc: OnigEncoding,
                                                   mut code: OnigCodePoint,
                                                   mut ctype: libc::c_uint)
 -> libc::c_int {
    if code < 128 as libc::c_int as libc::c_uint {
        return (OnigEncAsciiCtypeTable[code as usize] as libc::c_int &
                    (1 as libc::c_int) << ctype != 0 as libc::c_int) as
                   libc::c_int
    } else {
        if ctype == 12 as libc::c_int as libc::c_uint ||
               ctype == 5 as libc::c_int as libc::c_uint ||
               ctype == 7 as libc::c_int as libc::c_uint {
            return if (*enc).code_to_mbclen.expect("non-null function pointer")(code)
                          > 1 as libc::c_int {
                       1 as libc::c_int
                   } else { 0 as libc::c_int }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_mb4_is_code_ctype(mut enc: OnigEncoding,
                                                   mut code: OnigCodePoint,
                                                   mut ctype: libc::c_uint)
 -> libc::c_int {
    if code < 128 as libc::c_int as libc::c_uint {
        return (OnigEncAsciiCtypeTable[code as usize] as libc::c_int &
                    (1 as libc::c_int) << ctype != 0 as libc::c_int) as
                   libc::c_int
    } else {
        if ctype == 12 as libc::c_int as libc::c_uint ||
               ctype == 5 as libc::c_int as libc::c_uint ||
               ctype == 7 as libc::c_int as libc::c_uint {
            return if (*enc).code_to_mbclen.expect("non-null function pointer")(code)
                          > 1 as libc::c_int {
                       1 as libc::c_int
                   } else { 0 as libc::c_int }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onigenc_with_ascii_strncmp(mut enc: OnigEncoding,
                                                    mut p: *const OnigUChar,
                                                    mut end: *const OnigUChar,
                                                    mut sascii:
                                                        *const OnigUChar,
                                                    mut n: libc::c_int)
 -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    loop  {
        let fresh12 = n;
        n = n - 1;
        if !(fresh12 > 0 as libc::c_int) { break ; }
        if p >= end { return *sascii as libc::c_int }
        c =
            (*enc).mbc_to_code.expect("non-null function pointer")(p, end) as
                libc::c_int;
        x = *sascii as libc::c_int - c;
        if x != 0 { return x }
        sascii = sascii.offset(1);
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_codes_cmp(mut a: *mut OnigCodePoint,
                                        mut b: *mut OnigCodePoint,
                                        mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *a.offset(i as isize) != *b.offset(i as isize) {
            return -(1 as libc::c_int)
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *********************************************************************
  regenc.h -  Oniguruma (regular expression library)
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
/* character types bit flag */
/* #define USE_CRNL_AS_LINE_TERMINATOR */
/* #define USE_UNICODE_CASE_FOLD_TURKISH_AZERI */
/* #define USE_UNICODE_ALL_LINE_TERMINATORS */
/* see Unicode.org UTS #18 */
/* for encoding system implementation (internal) */
/* methods for single byte encoding */
/* methods for multi byte encoding */
//ONIG_EXTERN const struct PropertyNameCtype* unicode_lookup_property_name P_((register const char *str, register unsigned int len));
/* in enc/unicode.c */
/* from unicode generated codes */
#[no_mangle]
pub unsafe extern "C" fn onig_codes_byte_at(mut codes: *mut OnigCodePoint,
                                            mut at: libc::c_int)
 -> libc::c_int {
    let mut index: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut code: OnigCodePoint = 0;
    index = at / 3 as libc::c_int;
    b = at % 3 as libc::c_int;
    code = *codes.offset(index as isize);
    return (code >> (2 as libc::c_int - b) * 8 as libc::c_int &
                0xff as libc::c_int as libc::c_uint) as libc::c_int;
}
