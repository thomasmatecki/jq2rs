#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    static mut OnigUnicodeFolds3: [OnigCodePoint; 0];
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
/* maximum key range = 14, duplicates = 0 */
#[inline]
unsafe extern "C" fn hash(mut codes: *mut OnigCodePoint) -> libc::c_uint {
    static mut asso_values: [libc::c_uchar; 256] =
        [6 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         1 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         4 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         5 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         4 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         10 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         9 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         1 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         0 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         8 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar];
    return (asso_values[onig_codes_byte_at(codes, 8 as libc::c_int) as
                            libc::c_uchar as usize] as libc::c_int +
                asso_values[onig_codes_byte_at(codes, 5 as libc::c_int) as
                                libc::c_uchar as usize] as libc::c_int +
                asso_values[onig_codes_byte_at(codes, 2 as libc::c_int) as
                                libc::c_uchar as usize] as libc::c_int) as
               libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_fold3_key(mut codes: *mut OnigCodePoint)
 -> libc::c_int {
    static mut wordlist: [libc::c_int; 14] =
        [62 as libc::c_int, 47 as libc::c_int, 31 as libc::c_int,
         57 as libc::c_int, 41 as libc::c_int, 25 as libc::c_int,
         52 as libc::c_int, 36 as libc::c_int, 20 as libc::c_int,
         67 as libc::c_int, 15 as libc::c_int, 10 as libc::c_int,
         5 as libc::c_int, 0 as libc::c_int];
    if 0 as libc::c_int == 0 as libc::c_int {
        let mut key: libc::c_int = hash(codes) as libc::c_int;
        if key <= 13 as libc::c_int {
            let mut index: libc::c_int = wordlist[key as usize];
            if index >= 0 as libc::c_int &&
                   onig_codes_cmp(codes,
                                  OnigUnicodeFolds3.as_mut_ptr().offset(index
                                                                            as
                                                                            isize),
                                  3 as libc::c_int) == 0 as libc::c_int {
                return index
            }
        }
    }
    return -(1 as libc::c_int);
}
