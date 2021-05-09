#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    //ONIG_EXTERN const struct PropertyNameCtype* unicode_lookup_property_name P_((register const char *str, register unsigned int len));
    /* in enc/unicode.c */
    /* from unicode generated codes */
    #[no_mangle]
    static mut OnigUnicodeFolds2: [OnigCodePoint; 0];
    #[no_mangle]
    fn onig_codes_cmp(a: *mut OnigCodePoint, b: *mut OnigCodePoint,
                      n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn onig_codes_byte_at(code: *mut OnigCodePoint, at: libc::c_int)
     -> libc::c_int;
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
pub type OnigCodePoint = libc::c_uint;
/* maximum key range = 59, duplicates = 0 */
#[inline]
unsafe extern "C" fn hash(mut codes: *mut OnigCodePoint) -> libc::c_uint {
    static mut asso_values: [libc::c_uchar; 256] =
        [58 as libc::c_int as libc::c_uchar,
         57 as libc::c_int as libc::c_uchar,
         56 as libc::c_int as libc::c_uchar,
         55 as libc::c_int as libc::c_uchar,
         54 as libc::c_int as libc::c_uchar,
         53 as libc::c_int as libc::c_uchar,
         52 as libc::c_int as libc::c_uchar,
         16 as libc::c_int as libc::c_uchar,
         50 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         15 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         25 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         3 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         49 as libc::c_int as libc::c_uchar,
         48 as libc::c_int as libc::c_uchar,
         47 as libc::c_int as libc::c_uchar,
         46 as libc::c_int as libc::c_uchar,
         45 as libc::c_int as libc::c_uchar,
         44 as libc::c_int as libc::c_uchar,
         43 as libc::c_int as libc::c_uchar,
         42 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         21 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         2 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         40 as libc::c_int as libc::c_uchar,
         20 as libc::c_int as libc::c_uchar,
         39 as libc::c_int as libc::c_uchar,
         38 as libc::c_int as libc::c_uchar,
         37 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         5 as libc::c_int as libc::c_uchar,
         36 as libc::c_int as libc::c_uchar,
         20 as libc::c_int as libc::c_uchar,
         7 as libc::c_int as libc::c_uchar,
         25 as libc::c_int as libc::c_uchar,
         34 as libc::c_int as libc::c_uchar,
         29 as libc::c_int as libc::c_uchar,
         32 as libc::c_int as libc::c_uchar,
         16 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         31 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         2 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         25 as libc::c_int as libc::c_uchar,
         15 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         28 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         2 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         11 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         24 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         22 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         11 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         7 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         16 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         1 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         16 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         15 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         6 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar,
         59 as libc::c_int as libc::c_uchar];
    return (asso_values[onig_codes_byte_at(codes, 5 as libc::c_int) as
                            libc::c_uchar as usize] as libc::c_int +
                asso_values[onig_codes_byte_at(codes, 2 as libc::c_int) as
                                libc::c_uchar as usize] as libc::c_int) as
               libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_fold2_key(mut codes: *mut OnigCodePoint)
 -> libc::c_int {
    static mut wordlist: [libc::c_int; 59] =
        [101 as libc::c_int, 253 as libc::c_int, 76 as libc::c_int,
         29 as libc::c_int, 24 as libc::c_int, 239 as libc::c_int,
         96 as libc::c_int, 71 as libc::c_int, 92 as libc::c_int,
         67 as libc::c_int, 4 as libc::c_int, 62 as libc::c_int,
         8 as libc::c_int, 58 as libc::c_int, 234 as libc::c_int,
         109 as libc::c_int, 164 as libc::c_int, 88 as libc::c_int,
         84 as libc::c_int, 80 as libc::c_int, 214 as libc::c_int,
         0 as libc::c_int, 54 as libc::c_int, 261 as libc::c_int,
         50 as libc::c_int, 105 as libc::c_int, 121 as libc::c_int,
         125 as libc::c_int, 257 as libc::c_int, 42 as libc::c_int,
         38 as libc::c_int, 249 as libc::c_int, 46 as libc::c_int,
         117 as libc::c_int, 12 as libc::c_int, 113 as libc::c_int,
         244 as libc::c_int, 229 as libc::c_int, 224 as libc::c_int,
         219 as libc::c_int, 209 as libc::c_int, 16 as libc::c_int,
         204 as libc::c_int, 199 as libc::c_int, 194 as libc::c_int,
         189 as libc::c_int, 184 as libc::c_int, 179 as libc::c_int,
         174 as libc::c_int, 169 as libc::c_int, 20 as libc::c_int,
         34 as libc::c_int, 159 as libc::c_int, 154 as libc::c_int,
         149 as libc::c_int, 144 as libc::c_int, 139 as libc::c_int,
         134 as libc::c_int, 129 as libc::c_int];
    if 0 as libc::c_int == 0 as libc::c_int {
        let mut key: libc::c_int = hash(codes) as libc::c_int;
        if key <= 58 as libc::c_int {
            let mut index: libc::c_int = wordlist[key as usize];
            if index >= 0 as libc::c_int &&
                   onig_codes_cmp(codes,
                                  OnigUnicodeFolds2.as_mut_ptr().offset(index
                                                                            as
                                                                            isize),
                                  2 as libc::c_int) == 0 as libc::c_int {
                return index
            }
        }
    }
    return -(1 as libc::c_int);
}
