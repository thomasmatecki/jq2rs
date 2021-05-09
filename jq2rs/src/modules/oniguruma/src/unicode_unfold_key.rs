#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ByUnfoldKey {
    pub code: OnigCodePoint,
    pub index: libc::c_int,
    pub fold_len: libc::c_int,
}
/* maximum key range = 1534, duplicates = 0 */
#[inline]
unsafe extern "C" fn hash(mut codes: *mut OnigCodePoint) -> libc::c_uint {
    static mut asso_values: [libc::c_ushort; 291] =
        [8 as libc::c_int as libc::c_ushort,
         6 as libc::c_int as libc::c_ushort,
         2 as libc::c_int as libc::c_ushort,
         124 as libc::c_int as libc::c_ushort,
         5 as libc::c_int as libc::c_ushort,
         1 as libc::c_int as libc::c_ushort,
         36 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         11 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         16 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         562 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         77 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         0 as libc::c_int as libc::c_ushort,
         3 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         61 as libc::c_int as libc::c_ushort,
         628 as libc::c_int as libc::c_ushort,
         1379 as libc::c_int as libc::c_ushort,
         206 as libc::c_int as libc::c_ushort,
         1378 as libc::c_int as libc::c_ushort,
         607 as libc::c_int as libc::c_ushort,
         1372 as libc::c_int as libc::c_ushort,
         597 as libc::c_int as libc::c_ushort,
         1399 as libc::c_int as libc::c_ushort,
         569 as libc::c_int as libc::c_ushort,
         1371 as libc::c_int as libc::c_ushort,
         4 as libc::c_int as libc::c_ushort,
         1365 as libc::c_int as libc::c_ushort,
         559 as libc::c_int as libc::c_ushort,
         1359 as libc::c_int as libc::c_ushort,
         548 as libc::c_int as libc::c_ushort,
         1353 as libc::c_int as libc::c_ushort,
         836 as libc::c_int as libc::c_ushort,
         1393 as libc::c_int as libc::c_ushort,
         830 as libc::c_int as libc::c_ushort,
         1345 as libc::c_int as libc::c_ushort,
         587 as libc::c_int as libc::c_ushort,
         1344 as libc::c_int as libc::c_ushort,
         581 as libc::c_int as libc::c_ushort,
         1336 as libc::c_int as libc::c_ushort,
         539 as libc::c_int as libc::c_ushort,
         1335 as libc::c_int as libc::c_ushort,
         530 as libc::c_int as libc::c_ushort,
         982 as libc::c_int as libc::c_ushort,
         521 as libc::c_int as libc::c_ushort,
         970 as libc::c_int as libc::c_ushort,
         818 as libc::c_int as libc::c_ushort,
         1389 as libc::c_int as libc::c_ushort,
         723 as libc::c_int as libc::c_ushort,
         1329 as libc::c_int as libc::c_ushort,
         351 as libc::c_int as libc::c_ushort,
         1320 as libc::c_int as libc::c_ushort,
         333 as libc::c_int as libc::c_ushort,
         1312 as libc::c_int as libc::c_ushort,
         293 as libc::c_int as libc::c_ushort,
         1311 as libc::c_int as libc::c_ushort,
         320 as libc::c_int as libc::c_ushort,
         1304 as libc::c_int as libc::c_ushort,
         176 as libc::c_int as libc::c_ushort,
         589 as libc::c_int as libc::c_ushort,
         311 as libc::c_int as libc::c_ushort,
         1165 as libc::c_int as libc::c_ushort,
         302 as libc::c_int as libc::c_ushort,
         1384 as libc::c_int as libc::c_ushort,
         1243 as libc::c_int as libc::c_ushort,
         579 as libc::c_int as libc::c_ushort,
         780 as libc::c_int as libc::c_ushort,
         173 as libc::c_int as libc::c_ushort,
         1230 as libc::c_int as libc::c_ushort,
         147 as libc::c_int as libc::c_ushort,
         1213 as libc::c_int as libc::c_ushort,
         75 as libc::c_int as libc::c_ushort,
         1219 as libc::c_int as libc::c_ushort,
         1296 as libc::c_int as libc::c_ushort,
         1009 as libc::c_int as libc::c_ushort,
         1293 as libc::c_int as libc::c_ushort,
         1282 as libc::c_int as libc::c_ushort,
         1267 as libc::c_int as libc::c_ushort,
         1217 as libc::c_int as libc::c_ushort,
         1030 as libc::c_int as libc::c_ushort,
         331 as libc::c_int as libc::c_ushort,
         1291 as libc::c_int as libc::c_ushort,
         1210 as libc::c_int as libc::c_ushort,
         1286 as libc::c_int as libc::c_ushort,
         998 as libc::c_int as libc::c_ushort,
         500 as libc::c_int as libc::c_ushort,
         993 as libc::c_int as libc::c_ushort,
         1359 as libc::c_int as libc::c_ushort,
         806 as libc::c_int as libc::c_ushort,
         1281 as libc::c_int as libc::c_ushort,
         510 as libc::c_int as libc::c_ushort,
         1048 as libc::c_int as libc::c_ushort,
         501 as libc::c_int as libc::c_ushort,
         662 as libc::c_int as libc::c_ushort,
         797 as libc::c_int as libc::c_ushort,
         754 as libc::c_int as libc::c_ushort,
         792 as libc::c_int as libc::c_ushort,
         372 as libc::c_int as libc::c_ushort,
         775 as libc::c_int as libc::c_ushort,
         290 as libc::c_int as libc::c_ushort,
         768 as libc::c_int as libc::c_ushort,
         228 as libc::c_int as libc::c_ushort,
         755 as libc::c_int as libc::c_ushort,
         292 as libc::c_int as libc::c_ushort,
         1159 as libc::c_int as libc::c_ushort,
         489 as libc::c_int as libc::c_ushort,
         1135 as libc::c_int as libc::c_ushort,
         267 as libc::c_int as libc::c_ushort,
         1229 as libc::c_int as libc::c_ushort,
         233 as libc::c_int as libc::c_ushort,
         1053 as libc::c_int as libc::c_ushort,
         222 as libc::c_int as libc::c_ushort,
         728 as libc::c_int as libc::c_ushort,
         159 as libc::c_int as libc::c_ushort,
         708 as libc::c_int as libc::c_ushort,
         484 as libc::c_int as libc::c_ushort,
         695 as libc::c_int as libc::c_ushort,
         155 as libc::c_int as libc::c_ushort,
         995 as libc::c_int as libc::c_ushort,
         247 as libc::c_int as libc::c_ushort,
         686 as libc::c_int as libc::c_ushort,
         859 as libc::c_int as libc::c_ushort,
         674 as libc::c_int as libc::c_ushort,
         747 as libc::c_int as libc::c_ushort,
         618 as libc::c_int as libc::c_ushort,
         561 as libc::c_int as libc::c_ushort,
         381 as libc::c_int as libc::c_ushort,
         313 as libc::c_int as libc::c_ushort,
         987 as libc::c_int as libc::c_ushort,
         167 as libc::c_int as libc::c_ushort,
         975 as libc::c_int as libc::c_ushort,
         165 as libc::c_int as libc::c_ushort,
         1279 as libc::c_int as libc::c_ushort,
         388 as libc::c_int as libc::c_ushort,
         1207 as libc::c_int as libc::c_ushort,
         157 as libc::c_int as libc::c_ushort,
         765 as libc::c_int as libc::c_ushort,
         900 as libc::c_int as libc::c_ushort,
         1007 as libc::c_int as libc::c_ushort,
         794 as libc::c_int as libc::c_ushort,
         476 as libc::c_int as libc::c_ushort,
         21 as libc::c_int as libc::c_ushort,
         1198 as libc::c_int as libc::c_ushort,
         1271 as libc::c_int as libc::c_ushort,
         490 as libc::c_int as libc::c_ushort,
         1265 as libc::c_int as libc::c_ushort,
         478 as libc::c_int as libc::c_ushort,
         1245 as libc::c_int as libc::c_ushort,
         18 as libc::c_int as libc::c_ushort,
         8 as libc::c_int as libc::c_ushort,
         253 as libc::c_int as libc::c_ushort,
         1188 as libc::c_int as libc::c_ushort,
         652 as libc::c_int as libc::c_ushort,
         7 as libc::c_int as libc::c_ushort,
         245 as libc::c_int as libc::c_ushort,
         1185 as libc::c_int as libc::c_ushort,
         415 as libc::c_int as libc::c_ushort,
         1256 as libc::c_int as libc::c_ushort,
         226 as libc::c_int as libc::c_ushort,
         1177 as libc::c_int as libc::c_ushort,
         54 as libc::c_int as libc::c_ushort,
         1169 as libc::c_int as libc::c_ushort,
         214 as libc::c_int as libc::c_ushort,
         1155 as libc::c_int as libc::c_ushort,
         195 as libc::c_int as libc::c_ushort,
         607 as libc::c_int as libc::c_ushort,
         42 as libc::c_int as libc::c_ushort,
         963 as libc::c_int as libc::c_ushort,
         30 as libc::c_int as libc::c_ushort,
         1147 as libc::c_int as libc::c_ushort,
         185 as libc::c_int as libc::c_ushort,
         1139 as libc::c_int as libc::c_ushort,
         465 as libc::c_int as libc::c_ushort,
         1129 as libc::c_int as libc::c_ushort,
         451 as libc::c_int as libc::c_ushort,
         1121 as libc::c_int as libc::c_ushort,
         86 as libc::c_int as libc::c_ushort,
         948 as libc::c_int as libc::c_ushort,
         136 as libc::c_int as libc::c_ushort,
         940 as libc::c_int as libc::c_ushort,
         76 as libc::c_int as libc::c_ushort,
         909 as libc::c_int as libc::c_ushort,
         66 as libc::c_int as libc::c_ushort,
         664 as libc::c_int as libc::c_ushort,
         126 as libc::c_int as libc::c_ushort,
         644 as libc::c_int as libc::c_ushort,
         116 as libc::c_int as libc::c_ushort,
         632 as libc::c_int as libc::c_ushort,
         106 as libc::c_int as libc::c_ushort,
         930 as libc::c_int as libc::c_ushort,
         166 as libc::c_int as libc::c_ushort,
         925 as libc::c_int as libc::c_ushort,
         149 as libc::c_int as libc::c_ushort,
         915 as libc::c_int as libc::c_ushort,
         96 as libc::c_int as libc::c_ushort,
         903 as libc::c_int as libc::c_ushort,
         390 as libc::c_int as libc::c_ushort,
         364 as libc::c_int as libc::c_ushort,
         283 as libc::c_int as libc::c_ushort,
         746 as libc::c_int as libc::c_ushort,
         273 as libc::c_int as libc::c_ushort,
         1098 as libc::c_int as libc::c_ushort,
         372 as libc::c_int as libc::c_ushort,
         1095 as libc::c_int as libc::c_ushort,
         265 as libc::c_int as libc::c_ushort,
         528 as libc::c_int as libc::c_ushort,
         361 as libc::c_int as libc::c_ushort,
         311 as libc::c_int as libc::c_ushort,
         897 as libc::c_int as libc::c_ushort,
         1195 as libc::c_int as libc::c_ushort,
         396 as libc::c_int as libc::c_ushort,
         1103 as libc::c_int as libc::c_ushort,
         425 as libc::c_int as libc::c_ushort,
         1094 as libc::c_int as libc::c_ushort,
         1088 as libc::c_int as libc::c_ushort,
         893 as libc::c_int as libc::c_ushort,
         887 as libc::c_int as libc::c_ushort,
         573 as libc::c_int as libc::c_ushort,
         407 as libc::c_int as libc::c_ushort,
         237 as libc::c_int as libc::c_ushort,
         1083 as libc::c_int as libc::c_ushort,
         934 as libc::c_int as libc::c_ushort,
         1145 as libc::c_int as libc::c_ushort,
         432 as libc::c_int as libc::c_ushort,
         1076 as libc::c_int as libc::c_ushort,
         679 as libc::c_int as libc::c_ushort,
         714 as libc::c_int as libc::c_ushort,
         956 as libc::c_int as libc::c_ushort,
         1112 as libc::c_int as libc::c_ushort,
         509 as libc::c_int as libc::c_ushort,
         880 as libc::c_int as libc::c_ushort,
         62 as libc::c_int as libc::c_ushort,
         873 as libc::c_int as libc::c_ushort,
         157 as libc::c_int as libc::c_ushort,
         864 as libc::c_int as libc::c_ushort,
         276 as libc::c_int as libc::c_ushort,
         1069 as libc::c_int as libc::c_ushort,
         112 as libc::c_int as libc::c_ushort,
         855 as libc::c_int as libc::c_ushort,
         156 as libc::c_int as libc::c_ushort,
         1063 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         848 as libc::c_int as libc::c_ushort,
         152 as libc::c_int as libc::c_ushort,
         1057 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1047 as libc::c_int as libc::c_ushort,
         145 as libc::c_int as libc::c_ushort,
         1041 as libc::c_int as libc::c_ushort,
         144 as libc::c_int as libc::c_ushort,
         1035 as libc::c_int as libc::c_ushort,
         49 as libc::c_int as libc::c_ushort,
         1025 as libc::c_int as libc::c_ushort,
         142 as libc::c_int as libc::c_ushort,
         1256 as libc::c_int as libc::c_ushort,
         1545 as libc::c_int as libc::c_ushort,
         1239 as libc::c_int as libc::c_ushort,
         355 as libc::c_int as libc::c_ushort,
         342 as libc::c_int as libc::c_ushort,
         21 as libc::c_int as libc::c_ushort,
         1019 as libc::c_int as libc::c_ushort,
         14 as libc::c_int as libc::c_ushort,
         1233 as libc::c_int as libc::c_ushort,
         459 as libc::c_int as libc::c_ushort,
         843 as libc::c_int as libc::c_ushort,
         822 as libc::c_int as libc::c_ushort,
         740 as libc::c_int as libc::c_ushort,
         38 as libc::c_int as libc::c_ushort,
         553 as libc::c_int as libc::c_ushort,
         96 as libc::c_int as libc::c_ushort,
         448 as libc::c_int as libc::c_ushort,
         8 as libc::c_int as libc::c_ushort];
    return (asso_values[(onig_codes_byte_at(codes, 2 as libc::c_int) as
                             libc::c_uchar as libc::c_int + 35 as libc::c_int)
                            as usize] as libc::c_int +
                asso_values[(onig_codes_byte_at(codes, 1 as libc::c_int) as
                                 libc::c_uchar as libc::c_int +
                                 1 as libc::c_int) as usize] as libc::c_int +
                asso_values[onig_codes_byte_at(codes, 0 as libc::c_int) as
                                libc::c_uchar as usize] as libc::c_int) as
               libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_unfold_key(mut code: OnigCodePoint)
 -> *const ByUnfoldKey {
    static mut wordlist: [ByUnfoldKey; 1545] =
        [{
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1040a as libc::c_int as OnigCodePoint,
                             index: 3267 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e0a as libc::c_int as OnigCodePoint,
                             index: 1727 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x40a as libc::c_int as OnigCodePoint,
                             index: 1016 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a as libc::c_int as OnigCodePoint,
                             index: 186 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0a as libc::c_int as OnigCodePoint,
                             index: 2088 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c0a as libc::c_int as OnigCodePoint,
                             index: 2451 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x189 as libc::c_int as OnigCodePoint,
                             index: 619 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f89 as libc::c_int as OnigCodePoint,
                             index: 134 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f85 as libc::c_int as OnigCodePoint,
                             index: 154 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x389 as libc::c_int as OnigCodePoint,
                             index: 733 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3ff as libc::c_int as OnigCodePoint,
                             index: 724 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab89 as libc::c_int as OnigCodePoint,
                             index: 1523 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab85 as libc::c_int as OnigCodePoint,
                             index: 1511 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c89 as libc::c_int as OnigCodePoint,
                             index: 3384 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c85 as libc::c_int as OnigCodePoint,
                             index: 3372 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e84 as libc::c_int as OnigCodePoint,
                             index: 1911 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3f5 as libc::c_int as OnigCodePoint,
                             index: 752 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x184 as libc::c_int as OnigCodePoint,
                             index: 360 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f84 as libc::c_int as OnigCodePoint,
                             index: 149 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c84 as libc::c_int as OnigCodePoint,
                             index: 2592 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x17d as libc::c_int as OnigCodePoint,
                             index: 351 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff3 as libc::c_int as OnigCodePoint,
                             index: 96 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab84 as libc::c_int as OnigCodePoint,
                             index: 1508 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa784 as libc::c_int as OnigCodePoint,
                             index: 3105 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c84 as libc::c_int as OnigCodePoint,
                             index: 3369 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab7d as libc::c_int as OnigCodePoint,
                             index: 1487 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa77d as libc::c_int as OnigCodePoint,
                             index: 1706 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e98 as libc::c_int as OnigCodePoint,
                             index: 38 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x498 as libc::c_int as OnigCodePoint,
                             index: 1106 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x198 as libc::c_int as OnigCodePoint,
                             index: 375 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f98 as libc::c_int as OnigCodePoint,
                             index: 169 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c98 as libc::c_int as OnigCodePoint,
                             index: 2622 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x398 as libc::c_int as OnigCodePoint,
                             index: 762 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa684 as libc::c_int as OnigCodePoint,
                             index: 2940 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab98 as libc::c_int as OnigCodePoint,
                             index: 1568 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa798 as libc::c_int as OnigCodePoint,
                             index: 3123 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c98 as libc::c_int as OnigCodePoint,
                             index: 3429 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x50a as libc::c_int as OnigCodePoint,
                             index: 1277 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ffb as libc::c_int as OnigCodePoint,
                             index: 2265 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e96 as libc::c_int as OnigCodePoint,
                             index: 16 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x496 as libc::c_int as OnigCodePoint,
                             index: 1103 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x196 as libc::c_int as OnigCodePoint,
                             index: 652 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f96 as libc::c_int as OnigCodePoint,
                             index: 199 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c96 as libc::c_int as OnigCodePoint,
                             index: 2619 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x396 as libc::c_int as OnigCodePoint,
                             index: 756 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa698 as libc::c_int as OnigCodePoint,
                             index: 2970 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab96 as libc::c_int as OnigCodePoint,
                             index: 1562 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa796 as libc::c_int as OnigCodePoint,
                             index: 3120 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c96 as libc::c_int as OnigCodePoint,
                             index: 3423 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1feb as libc::c_int as OnigCodePoint,
                             index: 2259 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ceb as libc::c_int as OnigCodePoint,
                             index: 2736 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e90 as libc::c_int as OnigCodePoint,
                             index: 1929 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x490 as libc::c_int as OnigCodePoint,
                             index: 1094 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x190 as libc::c_int as OnigCodePoint,
                             index: 628 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f90 as libc::c_int as OnigCodePoint,
                             index: 169 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c90 as libc::c_int as OnigCodePoint,
                             index: 2610 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x390 as libc::c_int as OnigCodePoint,
                             index: 25 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa696 as libc::c_int as OnigCodePoint,
                             index: 2967 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab90 as libc::c_int as OnigCodePoint,
                             index: 1544 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa790 as libc::c_int as OnigCodePoint,
                             index: 3114 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c90 as libc::c_int as OnigCodePoint,
                             index: 3405 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1d7 as libc::c_int as OnigCodePoint,
                             index: 444 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fd7 as libc::c_int as OnigCodePoint,
                             index: 31 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ea6 as libc::c_int as OnigCodePoint,
                             index: 1947 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4a6 as libc::c_int as OnigCodePoint,
                             index: 1127 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1a6 as libc::c_int as OnigCodePoint,
                             index: 676 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa6 as libc::c_int as OnigCodePoint,
                             index: 239 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ca6 as libc::c_int as OnigCodePoint,
                             index: 2643 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a6 as libc::c_int as OnigCodePoint,
                             index: 810 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa690 as libc::c_int as OnigCodePoint,
                             index: 2958 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba6 as libc::c_int as OnigCodePoint,
                             index: 1610 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7a6 as libc::c_int as OnigCodePoint,
                             index: 3144 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca6 as libc::c_int as OnigCodePoint,
                             index: 3471 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ea4 as libc::c_int as OnigCodePoint,
                             index: 1944 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4a4 as libc::c_int as OnigCodePoint,
                             index: 1124 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1a4 as libc::c_int as OnigCodePoint,
                             index: 390 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa4 as libc::c_int as OnigCodePoint,
                             index: 229 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ca4 as libc::c_int as OnigCodePoint,
                             index: 2640 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a4 as libc::c_int as OnigCodePoint,
                             index: 804 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a6 as libc::c_int as OnigCodePoint,
                             index: 2763 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba4 as libc::c_int as OnigCodePoint,
                             index: 1604 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7a4 as libc::c_int as OnigCodePoint,
                             index: 3141 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca4 as libc::c_int as OnigCodePoint,
                             index: 3465 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ea0 as libc::c_int as OnigCodePoint,
                             index: 1938 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4a0 as libc::c_int as OnigCodePoint,
                             index: 1118 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1a0 as libc::c_int as OnigCodePoint,
                             index: 384 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa0 as libc::c_int as OnigCodePoint,
                             index: 209 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ca0 as libc::c_int as OnigCodePoint,
                             index: 2634 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a0 as libc::c_int as OnigCodePoint,
                             index: 792 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a4 as libc::c_int as OnigCodePoint,
                             index: 2757 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba0 as libc::c_int as OnigCodePoint,
                             index: 1592 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7a0 as libc::c_int as OnigCodePoint,
                             index: 3135 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca0 as libc::c_int as OnigCodePoint,
                             index: 3453 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eb2 as libc::c_int as OnigCodePoint,
                             index: 1965 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4b2 as libc::c_int as OnigCodePoint,
                             index: 1145 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1b2 as libc::c_int as OnigCodePoint,
                             index: 694 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb2 as libc::c_int as OnigCodePoint,
                             index: 249 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cb2 as libc::c_int as OnigCodePoint,
                             index: 2661 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3fd as libc::c_int as OnigCodePoint,
                             index: 718 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a0 as libc::c_int as OnigCodePoint,
                             index: 2745 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb2 as libc::c_int as OnigCodePoint,
                             index: 1646 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7b2 as libc::c_int as OnigCodePoint,
                             index: 703 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cb2 as libc::c_int as OnigCodePoint,
                             index: 3507 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eac as libc::c_int as OnigCodePoint,
                             index: 1956 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4ac as libc::c_int as OnigCodePoint,
                             index: 1136 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ac as libc::c_int as OnigCodePoint,
                             index: 396 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fac as libc::c_int as OnigCodePoint,
                             index: 229 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cac as libc::c_int as OnigCodePoint,
                             index: 2652 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x537 as libc::c_int as OnigCodePoint,
                             index: 1352 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b2 as libc::c_int as OnigCodePoint,
                             index: 2799 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabac as libc::c_int as OnigCodePoint,
                             index: 1628 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7ac as libc::c_int as OnigCodePoint,
                             index: 637 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cac as libc::c_int as OnigCodePoint,
                             index: 3489 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eaa as libc::c_int as OnigCodePoint,
                             index: 1953 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4aa as libc::c_int as OnigCodePoint,
                             index: 1133 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xdd as libc::c_int as OnigCodePoint,
                             index: 162 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1faa as libc::c_int as OnigCodePoint,
                             index: 219 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2caa as libc::c_int as OnigCodePoint,
                             index: 2649 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3aa as libc::c_int as OnigCodePoint,
                             index: 824 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ac as libc::c_int as OnigCodePoint,
                             index: 2781 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabaa as libc::c_int as OnigCodePoint,
                             index: 1622 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7aa as libc::c_int as OnigCodePoint,
                             index: 646 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10caa as libc::c_int as OnigCodePoint,
                             index: 3483 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ea8 as libc::c_int as OnigCodePoint,
                             index: 1950 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4a8 as libc::c_int as OnigCodePoint,
                             index: 1130 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x20a as libc::c_int as OnigCodePoint,
                             index: 517 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa8 as libc::c_int as OnigCodePoint,
                             index: 209 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ca8 as libc::c_int as OnigCodePoint,
                             index: 2646 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a8 as libc::c_int as OnigCodePoint,
                             index: 817 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10aa as libc::c_int as OnigCodePoint,
                             index: 2775 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba8 as libc::c_int as OnigCodePoint,
                             index: 1616 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7a8 as libc::c_int as OnigCodePoint,
                             index: 3147 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca8 as libc::c_int as OnigCodePoint,
                             index: 3477 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ea2 as libc::c_int as OnigCodePoint,
                             index: 1941 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4a2 as libc::c_int as OnigCodePoint,
                             index: 1121 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1a2 as libc::c_int as OnigCodePoint,
                             index: 387 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa2 as libc::c_int as OnigCodePoint,
                             index: 219 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ca2 as libc::c_int as OnigCodePoint,
                             index: 2637 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a6 as libc::c_int as OnigCodePoint,
                             index: 3528 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a8 as libc::c_int as OnigCodePoint,
                             index: 2769 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba2 as libc::c_int as OnigCodePoint,
                             index: 1598 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7a2 as libc::c_int as OnigCodePoint,
                             index: 3138 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca2 as libc::c_int as OnigCodePoint,
                             index: 3459 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ced as libc::c_int as OnigCodePoint,
                             index: 2739 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe9 as libc::c_int as OnigCodePoint,
                             index: 2283 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe7 as libc::c_int as OnigCodePoint,
                             index: 47 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eb0 as libc::c_int as OnigCodePoint,
                             index: 1962 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4b0 as libc::c_int as OnigCodePoint,
                             index: 1142 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a4 as libc::c_int as OnigCodePoint,
                             index: 3522 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a2 as libc::c_int as OnigCodePoint,
                             index: 2751 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cb0 as libc::c_int as OnigCodePoint,
                             index: 2658 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3b0 as libc::c_int as OnigCodePoint,
                             index: 41 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe3 as libc::c_int as OnigCodePoint,
                             index: 41 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb0 as libc::c_int as OnigCodePoint,
                             index: 1640 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7b0 as libc::c_int as OnigCodePoint,
                             index: 706 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cb0 as libc::c_int as OnigCodePoint,
                             index: 3501 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1d9 as libc::c_int as OnigCodePoint,
                             index: 447 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fd9 as libc::c_int as OnigCodePoint,
                             index: 2277 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a0 as libc::c_int as OnigCodePoint,
                             index: 3510 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xdf as libc::c_int as OnigCodePoint,
                             index: 24 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd9 as libc::c_int as OnigCodePoint,
                             index: 150 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab77 as libc::c_int as OnigCodePoint,
                             index: 1469 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b0 as libc::c_int as OnigCodePoint,
                             index: 2793 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eae as libc::c_int as OnigCodePoint,
                             index: 1959 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4ae as libc::c_int as OnigCodePoint,
                             index: 1139 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ae as libc::c_int as OnigCodePoint,
                             index: 685 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fae as libc::c_int as OnigCodePoint,
                             index: 239 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cae as libc::c_int as OnigCodePoint,
                             index: 2655 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b2 as libc::c_int as OnigCodePoint,
                             index: 3564 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab73 as libc::c_int as OnigCodePoint,
                             index: 1457 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabae as libc::c_int as OnigCodePoint,
                             index: 1634 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab71 as libc::c_int as OnigCodePoint,
                             index: 1451 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cae as libc::c_int as OnigCodePoint,
                             index: 3495 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e2a as libc::c_int as OnigCodePoint,
                             index: 1775 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42a as libc::c_int as OnigCodePoint,
                             index: 968 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x12a as libc::c_int as OnigCodePoint,
                             index: 234 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2a as libc::c_int as OnigCodePoint,
                             index: 2130 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c2a as libc::c_int as OnigCodePoint,
                             index: 2547 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118ac as libc::c_int as OnigCodePoint,
                             index: 3546 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ae as libc::c_int as OnigCodePoint,
                             index: 2787 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x535 as libc::c_int as OnigCodePoint,
                             index: 1346 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa72a as libc::c_int as OnigCodePoint,
                             index: 2988 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e9a as libc::c_int as OnigCodePoint,
                             index: 0 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x49a as libc::c_int as OnigCodePoint,
                             index: 1109 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff37 as libc::c_int as OnigCodePoint,
                             index: 3225 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f9a as libc::c_int as OnigCodePoint,
                             index: 179 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c9a as libc::c_int as OnigCodePoint,
                             index: 2625 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x39a as libc::c_int as OnigCodePoint,
                             index: 772 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118aa as libc::c_int as OnigCodePoint,
                             index: 3540 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab9a as libc::c_int as OnigCodePoint,
                             index: 1574 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa79a as libc::c_int as OnigCodePoint,
                             index: 3126 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c9a as libc::c_int as OnigCodePoint,
                             index: 3435 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e94 as libc::c_int as OnigCodePoint,
                             index: 1935 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x494 as libc::c_int as OnigCodePoint,
                             index: 1100 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x194 as libc::c_int as OnigCodePoint,
                             index: 640 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f94 as libc::c_int as OnigCodePoint,
                             index: 189 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c94 as libc::c_int as OnigCodePoint,
                             index: 2616 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x394 as libc::c_int as OnigCodePoint,
                             index: 749 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a8 as libc::c_int as OnigCodePoint,
                             index: 3534 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab94 as libc::c_int as OnigCodePoint,
                             index: 1556 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa69a as libc::c_int as OnigCodePoint,
                             index: 2973 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c94 as libc::c_int as OnigCodePoint,
                             index: 3417 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10402 as libc::c_int as OnigCodePoint,
                             index: 3243 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e02 as libc::c_int as OnigCodePoint,
                             index: 1715 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x402 as libc::c_int as OnigCodePoint,
                             index: 992 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x102 as libc::c_int as OnigCodePoint,
                             index: 174 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x533 as libc::c_int as OnigCodePoint,
                             index: 1340 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c02 as libc::c_int as OnigCodePoint,
                             index: 2427 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a2 as libc::c_int as OnigCodePoint,
                             index: 3516 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x52a as libc::c_int as OnigCodePoint,
                             index: 1325 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa694 as libc::c_int as OnigCodePoint,
                             index: 2964 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e92 as libc::c_int as OnigCodePoint,
                             index: 1932 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x492 as libc::c_int as OnigCodePoint,
                             index: 1097 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2165 as libc::c_int as OnigCodePoint,
                             index: 2307 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f92 as libc::c_int as OnigCodePoint,
                             index: 179 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c92 as libc::c_int as OnigCodePoint,
                             index: 2613 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x392 as libc::c_int as OnigCodePoint,
                             index: 742 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2161 as libc::c_int as OnigCodePoint,
                             index: 2295 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab92 as libc::c_int as OnigCodePoint,
                             index: 1550 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa792 as libc::c_int as OnigCodePoint,
                             index: 3117 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c92 as libc::c_int as OnigCodePoint,
                             index: 3411 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b0 as libc::c_int as OnigCodePoint,
                             index: 3558 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f5f as libc::c_int as OnigCodePoint,
                             index: 2199 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e8e as libc::c_int as OnigCodePoint,
                             index: 1926 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x48e as libc::c_int as OnigCodePoint,
                             index: 1091 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x18e as libc::c_int as OnigCodePoint,
                             index: 453 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8e as libc::c_int as OnigCodePoint,
                             index: 159 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c8e as libc::c_int as OnigCodePoint,
                             index: 2607 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x38e as libc::c_int as OnigCodePoint,
                             index: 833 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa692 as libc::c_int as OnigCodePoint,
                             index: 2961 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab8e as libc::c_int as OnigCodePoint,
                             index: 1538 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x55 as libc::c_int as OnigCodePoint,
                             index: 59 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c8e as libc::c_int as OnigCodePoint,
                             index: 3399 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f5d as libc::c_int as OnigCodePoint,
                             index: 2196 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x212a as libc::c_int as OnigCodePoint,
                             index: 27 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4cb as libc::c_int as OnigCodePoint,
                             index: 1181 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1cb as libc::c_int as OnigCodePoint,
                             index: 425 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fcb as libc::c_int as OnigCodePoint,
                             index: 2241 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118ae as libc::c_int as OnigCodePoint,
                             index: 3552 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x502 as libc::c_int as OnigCodePoint,
                             index: 1265 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xcb as libc::c_int as OnigCodePoint,
                             index: 111 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa68e as libc::c_int as OnigCodePoint,
                             index: 2955 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e8a as libc::c_int as OnigCodePoint,
                             index: 1920 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x48a as libc::c_int as OnigCodePoint,
                             index: 1085 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x18a as libc::c_int as OnigCodePoint,
                             index: 622 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8a as libc::c_int as OnigCodePoint,
                             index: 139 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c8a as libc::c_int as OnigCodePoint,
                             index: 2601 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x38a as libc::c_int as OnigCodePoint,
                             index: 736 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c67 as libc::c_int as OnigCodePoint,
                             index: 2571 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab8a as libc::c_int as OnigCodePoint,
                             index: 1526 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e86 as libc::c_int as OnigCodePoint,
                             index: 1914 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c8a as libc::c_int as OnigCodePoint,
                             index: 3387 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x186 as libc::c_int as OnigCodePoint,
                             index: 616 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f86 as libc::c_int as OnigCodePoint,
                             index: 159 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c86 as libc::c_int as OnigCodePoint,
                             index: 2595 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x386 as libc::c_int as OnigCodePoint,
                             index: 727 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff35 as libc::c_int as OnigCodePoint,
                             index: 3219 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab86 as libc::c_int as OnigCodePoint,
                             index: 1514 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa786 as libc::c_int as OnigCodePoint,
                             index: 3108 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c86 as libc::c_int as OnigCodePoint,
                             index: 3375 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa68a as libc::c_int as OnigCodePoint,
                             index: 2949 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x555 as libc::c_int as OnigCodePoint,
                             index: 1442 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ebc as libc::c_int as OnigCodePoint,
                             index: 1980 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4bc as libc::c_int as OnigCodePoint,
                             index: 1160 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1bc as libc::c_int as OnigCodePoint,
                             index: 411 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fbc as libc::c_int as OnigCodePoint,
                             index: 62 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cbc as libc::c_int as OnigCodePoint,
                             index: 2676 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f5b as libc::c_int as OnigCodePoint,
                             index: 2193 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa686 as libc::c_int as OnigCodePoint,
                             index: 2943 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabbc as libc::c_int as OnigCodePoint,
                             index: 1676 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eb8 as libc::c_int as OnigCodePoint,
                             index: 1974 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4b8 as libc::c_int as OnigCodePoint,
                             index: 1154 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1b8 as libc::c_int as OnigCodePoint,
                             index: 408 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb8 as libc::c_int as OnigCodePoint,
                             index: 2268 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cb8 as libc::c_int as OnigCodePoint,
                             index: 2670 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1db as libc::c_int as OnigCodePoint,
                             index: 450 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fdb as libc::c_int as OnigCodePoint,
                             index: 2247 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb8 as libc::c_int as OnigCodePoint,
                             index: 1664 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10bc as libc::c_int as OnigCodePoint,
                             index: 2829 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xdb as libc::c_int as OnigCodePoint,
                             index: 156 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eb6 as libc::c_int as OnigCodePoint,
                             index: 1971 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4b6 as libc::c_int as OnigCodePoint,
                             index: 1151 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff33 as libc::c_int as OnigCodePoint,
                             index: 3213 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb6 as libc::c_int as OnigCodePoint,
                             index: 58 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cb6 as libc::c_int as OnigCodePoint,
                             index: 2667 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff2a as libc::c_int as OnigCodePoint,
                             index: 3186 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b8 as libc::c_int as OnigCodePoint,
                             index: 2817 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb6 as libc::c_int as OnigCodePoint,
                             index: 1658 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7b6 as libc::c_int as OnigCodePoint,
                             index: 3153 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10426 as libc::c_int as OnigCodePoint,
                             index: 3351 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e26 as libc::c_int as OnigCodePoint,
                             index: 1769 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x426 as libc::c_int as OnigCodePoint,
                             index: 956 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x126 as libc::c_int as OnigCodePoint,
                             index: 228 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53 as libc::c_int as OnigCodePoint,
                             index: 52 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c26 as libc::c_int as OnigCodePoint,
                             index: 2535 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x57 as libc::c_int as OnigCodePoint,
                             index: 65 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b6 as libc::c_int as OnigCodePoint,
                             index: 2811 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x22a as libc::c_int as OnigCodePoint,
                             index: 562 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa726 as libc::c_int as OnigCodePoint,
                             index: 2982 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e2e as libc::c_int as OnigCodePoint,
                             index: 1781 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42e as libc::c_int as OnigCodePoint,
                             index: 980 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x12e as libc::c_int as OnigCodePoint,
                             index: 240 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2e as libc::c_int as OnigCodePoint,
                             index: 2142 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c2e as libc::c_int as OnigCodePoint,
                             index: 2559 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2167 as libc::c_int as OnigCodePoint,
                             index: 2313 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa72e as libc::c_int as OnigCodePoint,
                             index: 2994 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e2c as libc::c_int as OnigCodePoint,
                             index: 1778 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42c as libc::c_int as OnigCodePoint,
                             index: 974 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x12c as libc::c_int as OnigCodePoint,
                             index: 237 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2c as libc::c_int as OnigCodePoint,
                             index: 2136 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c2c as libc::c_int as OnigCodePoint,
                             index: 2553 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6f as libc::c_int as OnigCodePoint,
                             index: 2223 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c6f as libc::c_int as OnigCodePoint,
                             index: 604 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabbf as libc::c_int as OnigCodePoint,
                             index: 1685 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa72c as libc::c_int as OnigCodePoint,
                             index: 2991 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e28 as libc::c_int as OnigCodePoint,
                             index: 1772 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x428 as libc::c_int as OnigCodePoint,
                             index: 962 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x128 as libc::c_int as OnigCodePoint,
                             index: 231 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f28 as libc::c_int as OnigCodePoint,
                             index: 2124 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c28 as libc::c_int as OnigCodePoint,
                             index: 2541 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x553 as libc::c_int as OnigCodePoint,
                             index: 1436 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10bf as libc::c_int as OnigCodePoint,
                             index: 2838 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa728 as libc::c_int as OnigCodePoint,
                             index: 2985 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x526 as libc::c_int as OnigCodePoint,
                             index: 1319 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x202 as libc::c_int as OnigCodePoint,
                             index: 505 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e40 as libc::c_int as OnigCodePoint,
                             index: 1808 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10424 as libc::c_int as OnigCodePoint,
                             index: 3345 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e24 as libc::c_int as OnigCodePoint,
                             index: 1766 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x424 as libc::c_int as OnigCodePoint,
                             index: 950 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x124 as libc::c_int as OnigCodePoint,
                             index: 225 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c24 as libc::c_int as OnigCodePoint,
                             index: 2529 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x52e as libc::c_int as OnigCodePoint,
                             index: 1331 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa740 as libc::c_int as OnigCodePoint,
                             index: 3018 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118bc as libc::c_int as OnigCodePoint,
                             index: 3594 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa724 as libc::c_int as OnigCodePoint,
                             index: 2979 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ef2 as libc::c_int as OnigCodePoint,
                             index: 2061 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4f2 as libc::c_int as OnigCodePoint,
                             index: 1241 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2 as libc::c_int as OnigCodePoint,
                             index: 483 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff2 as libc::c_int as OnigCodePoint,
                             index: 257 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cf2 as libc::c_int as OnigCodePoint,
                             index: 2742 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x52c as libc::c_int as OnigCodePoint,
                             index: 1328 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b8 as libc::c_int as OnigCodePoint,
                             index: 3582 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa640 as libc::c_int as OnigCodePoint,
                             index: 2865 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10422 as libc::c_int as OnigCodePoint,
                             index: 3339 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e22 as libc::c_int as OnigCodePoint,
                             index: 1763 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x422 as libc::c_int as OnigCodePoint,
                             index: 944 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x122 as libc::c_int as OnigCodePoint,
                             index: 222 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2126 as libc::c_int as OnigCodePoint,
                             index: 820 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c22 as libc::c_int as OnigCodePoint,
                             index: 2523 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x528 as libc::c_int as OnigCodePoint,
                             index: 1322 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f1 as libc::c_int as OnigCodePoint,
                             index: 483 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b6 as libc::c_int as OnigCodePoint,
                             index: 3576 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa722 as libc::c_int as OnigCodePoint,
                             index: 2976 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3f1 as libc::c_int as OnigCodePoint,
                             index: 796 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ebe as libc::c_int as OnigCodePoint,
                             index: 1983 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4be as libc::c_int as OnigCodePoint,
                             index: 1163 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb02 as libc::c_int as OnigCodePoint,
                             index: 12 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fbe as libc::c_int as OnigCodePoint,
                             index: 767 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cbe as libc::c_int as OnigCodePoint,
                             index: 2679 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1b5 as libc::c_int as OnigCodePoint,
                             index: 405 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x540 as libc::c_int as OnigCodePoint,
                             index: 1379 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabbe as libc::c_int as OnigCodePoint,
                             index: 1682 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x524 as libc::c_int as OnigCodePoint,
                             index: 1316 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xb5 as libc::c_int as OnigCodePoint,
                             index: 779 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb5 as libc::c_int as OnigCodePoint,
                             index: 1655 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eba as libc::c_int as OnigCodePoint,
                             index: 1977 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4ba as libc::c_int as OnigCodePoint,
                             index: 1157 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216f as libc::c_int as OnigCodePoint,
                             index: 2337 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fba as libc::c_int as OnigCodePoint,
                             index: 2226 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cba as libc::c_int as OnigCodePoint,
                             index: 2673 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10be as libc::c_int as OnigCodePoint,
                             index: 2835 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x51 as libc::c_int as OnigCodePoint,
                             index: 46 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabba as libc::c_int as OnigCodePoint,
                             index: 1670 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b5 as libc::c_int as OnigCodePoint,
                             index: 2808 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e6e as libc::c_int as OnigCodePoint,
                             index: 1878 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x46e as libc::c_int as OnigCodePoint,
                             index: 1055 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x16e as libc::c_int as OnigCodePoint,
                             index: 330 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6e as libc::c_int as OnigCodePoint,
                             index: 2220 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c6e as libc::c_int as OnigCodePoint,
                             index: 664 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118bf as libc::c_int as OnigCodePoint,
                             index: 3603 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x522 as libc::c_int as OnigCodePoint,
                             index: 1313 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ba as libc::c_int as OnigCodePoint,
                             index: 2823 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa76e as libc::c_int as OnigCodePoint,
                             index: 3087 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eb4 as libc::c_int as OnigCodePoint,
                             index: 1968 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4b4 as libc::c_int as OnigCodePoint,
                             index: 1148 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c75 as libc::c_int as OnigCodePoint,
                             index: 2583 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb4 as libc::c_int as OnigCodePoint,
                             index: 50 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cb4 as libc::c_int as OnigCodePoint,
                             index: 2664 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab75 as libc::c_int as OnigCodePoint,
                             index: 1463 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ec2 as libc::c_int as OnigCodePoint,
                             index: 1989 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb4 as libc::c_int as OnigCodePoint,
                             index: 1652 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7b4 as libc::c_int as OnigCodePoint,
                             index: 3150 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc2 as libc::c_int as OnigCodePoint,
                             index: 253 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cc2 as libc::c_int as OnigCodePoint,
                             index: 2685 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3c2 as libc::c_int as OnigCodePoint,
                             index: 800 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc2 as libc::c_int as OnigCodePoint,
                             index: 83 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff26 as libc::c_int as OnigCodePoint,
                             index: 3174 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b4 as libc::c_int as OnigCodePoint,
                             index: 2805 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eca as libc::c_int as OnigCodePoint,
                             index: 2001 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x551 as libc::c_int as OnigCodePoint,
                             index: 1430 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ca as libc::c_int as OnigCodePoint,
                             index: 425 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fca as libc::c_int as OnigCodePoint,
                             index: 2238 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cca as libc::c_int as OnigCodePoint,
                             index: 2697 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c2 as libc::c_int as OnigCodePoint,
                             index: 2847 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xca as libc::c_int as OnigCodePoint,
                             index: 108 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff2e as libc::c_int as OnigCodePoint,
                             index: 3198 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e8c as libc::c_int as OnigCodePoint,
                             index: 1923 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x48c as libc::c_int as OnigCodePoint,
                             index: 1088 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x226 as libc::c_int as OnigCodePoint,
                             index: 556 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8c as libc::c_int as OnigCodePoint,
                             index: 149 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c8c as libc::c_int as OnigCodePoint,
                             index: 2604 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x38c as libc::c_int as OnigCodePoint,
                             index: 830 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab8c as libc::c_int as OnigCodePoint,
                             index: 1532 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff2c as libc::c_int as OnigCodePoint,
                             index: 3192 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c8c as libc::c_int as OnigCodePoint,
                             index: 3393 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ec4 as libc::c_int as OnigCodePoint,
                             index: 1992 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x22e as libc::c_int as OnigCodePoint,
                             index: 568 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1c4 as libc::c_int as OnigCodePoint,
                             index: 417 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc4 as libc::c_int as OnigCodePoint,
                             index: 54 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cc4 as libc::c_int as OnigCodePoint,
                             index: 2688 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc4 as libc::c_int as OnigCodePoint,
                             index: 89 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff28 as libc::c_int as OnigCodePoint,
                             index: 3180 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa68c as libc::c_int as OnigCodePoint,
                             index: 2952 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1cf as libc::c_int as OnigCodePoint,
                             index: 432 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x22c as libc::c_int as OnigCodePoint,
                             index: 565 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118be as libc::c_int as OnigCodePoint,
                             index: 3600 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3cf as libc::c_int as OnigCodePoint,
                             index: 839 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xcf as libc::c_int as OnigCodePoint,
                             index: 123 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b5 as libc::c_int as OnigCodePoint,
                             index: 3573 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c4 as libc::c_int as OnigCodePoint,
                             index: 2853 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216e as libc::c_int as OnigCodePoint,
                             index: 2334 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24cb as libc::c_int as OnigCodePoint,
                             index: 2406 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x228 as libc::c_int as OnigCodePoint,
                             index: 559 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff24 as libc::c_int as OnigCodePoint,
                             index: 3168 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118ba as libc::c_int as OnigCodePoint,
                             index: 3588 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1efe as libc::c_int as OnigCodePoint,
                             index: 2079 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4fe as libc::c_int as OnigCodePoint,
                             index: 1259 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe as libc::c_int as OnigCodePoint,
                             index: 499 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e9e as libc::c_int as OnigCodePoint,
                             index: 24 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x49e as libc::c_int as OnigCodePoint,
                             index: 1115 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3fe as libc::c_int as OnigCodePoint,
                             index: 721 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f9e as libc::c_int as OnigCodePoint,
                             index: 199 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c9e as libc::c_int as OnigCodePoint,
                             index: 2631 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x39e as libc::c_int as OnigCodePoint,
                             index: 786 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x224 as libc::c_int as OnigCodePoint,
                             index: 553 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab9e as libc::c_int as OnigCodePoint,
                             index: 1586 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa79e as libc::c_int as OnigCodePoint,
                             index: 3132 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c9e as libc::c_int as OnigCodePoint,
                             index: 3447 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f7 as libc::c_int as OnigCodePoint,
                             index: 414 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff7 as libc::c_int as OnigCodePoint,
                             index: 67 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff22 as libc::c_int as OnigCodePoint,
                             index: 3162 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3f7 as libc::c_int as OnigCodePoint,
                             index: 884 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b4 as libc::c_int as OnigCodePoint,
                             index: 3570 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x49c as libc::c_int as OnigCodePoint,
                             index: 1112 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x19c as libc::c_int as OnigCodePoint,
                             index: 661 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f9c as libc::c_int as OnigCodePoint,
                             index: 189 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c9c as libc::c_int as OnigCodePoint,
                             index: 2628 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x39c as libc::c_int as OnigCodePoint,
                             index: 779 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24bc as libc::c_int as OnigCodePoint,
                             index: 2361 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab9c as libc::c_int as OnigCodePoint,
                             index: 1580 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa79c as libc::c_int as OnigCodePoint,
                             index: 3129 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c9c as libc::c_int as OnigCodePoint,
                             index: 3441 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x222 as libc::c_int as OnigCodePoint,
                             index: 550 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e7c as libc::c_int as OnigCodePoint,
                             index: 1899 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x47c as libc::c_int as OnigCodePoint,
                             index: 1076 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e82 as libc::c_int as OnigCodePoint,
                             index: 1908 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24b8 as libc::c_int as OnigCodePoint,
                             index: 2349 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x182 as libc::c_int as OnigCodePoint,
                             index: 357 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f82 as libc::c_int as OnigCodePoint,
                             index: 139 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c82 as libc::c_int as OnigCodePoint,
                             index: 2589 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab7c as libc::c_int as OnigCodePoint,
                             index: 1484 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab82 as libc::c_int as OnigCodePoint,
                             index: 1502 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa782 as libc::c_int as OnigCodePoint,
                             index: 3102 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c82 as libc::c_int as OnigCodePoint,
                             index: 3363 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c63 as libc::c_int as OnigCodePoint,
                             index: 1709 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24b6 as libc::c_int as OnigCodePoint,
                             index: 2343 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e80 as libc::c_int as OnigCodePoint,
                             index: 1905 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x480 as libc::c_int as OnigCodePoint,
                             index: 1082 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f59 as libc::c_int as OnigCodePoint,
                             index: 2190 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f80 as libc::c_int as OnigCodePoint,
                             index: 129 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c80 as libc::c_int as OnigCodePoint,
                             index: 2586 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x59 as libc::c_int as OnigCodePoint,
                             index: 71 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa682 as libc::c_int as OnigCodePoint,
                             index: 2937 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab80 as libc::c_int as OnigCodePoint,
                             index: 1496 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa780 as libc::c_int as OnigCodePoint,
                             index: 3099 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c80 as libc::c_int as OnigCodePoint,
                             index: 3357 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e4c as libc::c_int as OnigCodePoint,
                             index: 1826 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x145 as libc::c_int as OnigCodePoint,
                             index: 270 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x14c as libc::c_int as OnigCodePoint,
                             index: 279 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f4c as libc::c_int as OnigCodePoint,
                             index: 2184 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x345 as libc::c_int as OnigCodePoint,
                             index: 767 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x45 as libc::c_int as OnigCodePoint,
                             index: 12 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c as libc::c_int as OnigCodePoint,
                             index: 31 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa680 as libc::c_int as OnigCodePoint,
                             index: 2934 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa74c as libc::c_int as OnigCodePoint,
                             index: 3036 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e4a as libc::c_int as OnigCodePoint,
                             index: 1823 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1d5 as libc::c_int as OnigCodePoint,
                             index: 441 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x14a as libc::c_int as OnigCodePoint,
                             index: 276 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f4a as libc::c_int as OnigCodePoint,
                             index: 2178 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3d5 as libc::c_int as OnigCodePoint,
                             index: 810 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd5 as libc::c_int as OnigCodePoint,
                             index: 141 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4a as libc::c_int as OnigCodePoint,
                             index: 24 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24bf as libc::c_int as OnigCodePoint,
                             index: 2370 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa74a as libc::c_int as OnigCodePoint,
                             index: 3033 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa64c as libc::c_int as OnigCodePoint,
                             index: 2883 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1041c as libc::c_int as OnigCodePoint,
                             index: 3321 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e1c as libc::c_int as OnigCodePoint,
                             index: 1754 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41c as libc::c_int as OnigCodePoint,
                             index: 926 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x11c as libc::c_int as OnigCodePoint,
                             index: 213 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f1c as libc::c_int as OnigCodePoint,
                             index: 2118 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c1c as libc::c_int as OnigCodePoint,
                             index: 2505 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa64a as libc::c_int as OnigCodePoint,
                             index: 2880 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1041a as libc::c_int as OnigCodePoint,
                             index: 3315 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e1a as libc::c_int as OnigCodePoint,
                             index: 1751 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41a as libc::c_int as OnigCodePoint,
                             index: 920 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x11a as libc::c_int as OnigCodePoint,
                             index: 210 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f1a as libc::c_int as OnigCodePoint,
                             index: 2112 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c1a as libc::c_int as OnigCodePoint,
                             index: 2499 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabbd as libc::c_int as OnigCodePoint,
                             index: 1679 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x545 as libc::c_int as OnigCodePoint,
                             index: 1394 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54c as libc::c_int as OnigCodePoint,
                             index: 1415 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10418 as libc::c_int as OnigCodePoint,
                             index: 3309 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e18 as libc::c_int as OnigCodePoint,
                             index: 1748 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x418 as libc::c_int as OnigCodePoint,
                             index: 914 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118 as libc::c_int as OnigCodePoint,
                             index: 207 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f18 as libc::c_int as OnigCodePoint,
                             index: 2106 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c18 as libc::c_int as OnigCodePoint,
                             index: 2493 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10bd as libc::c_int as OnigCodePoint,
                             index: 2832 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2163 as libc::c_int as OnigCodePoint,
                             index: 2301 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54a as libc::c_int as OnigCodePoint,
                             index: 1409 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1040e as libc::c_int as OnigCodePoint,
                             index: 3279 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e0e as libc::c_int as OnigCodePoint,
                             index: 1733 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x40e as libc::c_int as OnigCodePoint,
                             index: 1028 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10e as libc::c_int as OnigCodePoint,
                             index: 192 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0e as libc::c_int as OnigCodePoint,
                             index: 2100 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c0e as libc::c_int as OnigCodePoint,
                             index: 2463 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1efc as libc::c_int as OnigCodePoint,
                             index: 2076 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4fc as libc::c_int as OnigCodePoint,
                             index: 1256 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc as libc::c_int as OnigCodePoint,
                             index: 496 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ffc as libc::c_int as OnigCodePoint,
                             index: 96 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x51c as libc::c_int as OnigCodePoint,
                             index: 1304 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1040c as libc::c_int as OnigCodePoint,
                             index: 3273 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e0c as libc::c_int as OnigCodePoint,
                             index: 1730 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x40c as libc::c_int as OnigCodePoint,
                             index: 1022 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c as libc::c_int as OnigCodePoint,
                             index: 189 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0c as libc::c_int as OnigCodePoint,
                             index: 2094 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c0c as libc::c_int as OnigCodePoint,
                             index: 2457 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6d as libc::c_int as OnigCodePoint,
                             index: 2217 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c6d as libc::c_int as OnigCodePoint,
                             index: 607 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x51a as libc::c_int as OnigCodePoint,
                             index: 1301 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24be as libc::c_int as OnigCodePoint,
                             index: 2367 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10408 as libc::c_int as OnigCodePoint,
                             index: 3261 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e08 as libc::c_int as OnigCodePoint,
                             index: 1724 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x408 as libc::c_int as OnigCodePoint,
                             index: 1010 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x108 as libc::c_int as OnigCodePoint,
                             index: 183 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f08 as libc::c_int as OnigCodePoint,
                             index: 2082 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c08 as libc::c_int as OnigCodePoint,
                             index: 2445 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c9 as libc::c_int as OnigCodePoint,
                             index: 1178 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x518 as libc::c_int as OnigCodePoint,
                             index: 1298 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc9 as libc::c_int as OnigCodePoint,
                             index: 2235 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24ba as libc::c_int as OnigCodePoint,
                             index: 2355 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc9 as libc::c_int as OnigCodePoint,
                             index: 105 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10416 as libc::c_int as OnigCodePoint,
                             index: 3303 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e16 as libc::c_int as OnigCodePoint,
                             index: 1745 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x416 as libc::c_int as OnigCodePoint,
                             index: 908 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x116 as libc::c_int as OnigCodePoint,
                             index: 204 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x50e as libc::c_int as OnigCodePoint,
                             index: 1283 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c16 as libc::c_int as OnigCodePoint,
                             index: 2487 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10414 as libc::c_int as OnigCodePoint,
                             index: 3297 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e14 as libc::c_int as OnigCodePoint,
                             index: 1742 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x414 as libc::c_int as OnigCodePoint,
                             index: 902 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x114 as libc::c_int as OnigCodePoint,
                             index: 201 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42b as libc::c_int as OnigCodePoint,
                             index: 971 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c14 as libc::c_int as OnigCodePoint,
                             index: 2481 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2b as libc::c_int as OnigCodePoint,
                             index: 2133 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c2b as libc::c_int as OnigCodePoint,
                             index: 2550 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x50c as libc::c_int as OnigCodePoint,
                             index: 1280 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10406 as libc::c_int as OnigCodePoint,
                             index: 3255 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e06 as libc::c_int as OnigCodePoint,
                             index: 1721 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x406 as libc::c_int as OnigCodePoint,
                             index: 1004 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x106 as libc::c_int as OnigCodePoint,
                             index: 180 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13fb as libc::c_int as OnigCodePoint,
                             index: 1697 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c06 as libc::c_int as OnigCodePoint,
                             index: 2439 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c2 as libc::c_int as OnigCodePoint,
                             index: 2379 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118bd as libc::c_int as OnigCodePoint,
                             index: 3597 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x508 as libc::c_int as OnigCodePoint,
                             index: 1274 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10404 as libc::c_int as OnigCodePoint,
                             index: 3249 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e04 as libc::c_int as OnigCodePoint,
                             index: 1718 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x404 as libc::c_int as OnigCodePoint,
                             index: 998 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x104 as libc::c_int as OnigCodePoint,
                             index: 177 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f95 as libc::c_int as OnigCodePoint,
                             index: 194 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c04 as libc::c_int as OnigCodePoint,
                             index: 2433 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x395 as libc::c_int as OnigCodePoint,
                             index: 752 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24ca as libc::c_int as OnigCodePoint,
                             index: 2403 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab95 as libc::c_int as OnigCodePoint,
                             index: 1559 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x531 as libc::c_int as OnigCodePoint,
                             index: 1334 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c95 as libc::c_int as OnigCodePoint,
                             index: 3420 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x516 as libc::c_int as OnigCodePoint,
                             index: 1295 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e6c as libc::c_int as OnigCodePoint,
                             index: 1875 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x46c as libc::c_int as OnigCodePoint,
                             index: 1052 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x16c as libc::c_int as OnigCodePoint,
                             index: 327 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6c as libc::c_int as OnigCodePoint,
                             index: 2214 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216d as libc::c_int as OnigCodePoint,
                             index: 2331 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x514 as libc::c_int as OnigCodePoint,
                             index: 1292 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x245 as libc::c_int as OnigCodePoint,
                             index: 697 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c as libc::c_int as OnigCodePoint,
                             index: 598 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa76c as libc::c_int as OnigCodePoint,
                             index: 3084 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10400 as libc::c_int as OnigCodePoint,
                             index: 3237 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e00 as libc::c_int as OnigCodePoint,
                             index: 1712 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x400 as libc::c_int as OnigCodePoint,
                             index: 986 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x100 as libc::c_int as OnigCodePoint,
                             index: 171 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c4 as libc::c_int as OnigCodePoint,
                             index: 2385 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c00 as libc::c_int as OnigCodePoint,
                             index: 2421 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x506 as libc::c_int as OnigCodePoint,
                             index: 1271 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24a as libc::c_int as OnigCodePoint,
                             index: 595 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fab as libc::c_int as OnigCodePoint,
                             index: 224 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa66c as libc::c_int as OnigCodePoint,
                             index: 2931 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3ab as libc::c_int as OnigCodePoint,
                             index: 827 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24cf as libc::c_int as OnigCodePoint,
                             index: 2418 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabab as libc::c_int as OnigCodePoint,
                             index: 1625 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7ab as libc::c_int as OnigCodePoint,
                             index: 631 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cab as libc::c_int as OnigCodePoint,
                             index: 3486 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x504 as libc::c_int as OnigCodePoint,
                             index: 1268 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x21c as libc::c_int as OnigCodePoint,
                             index: 544 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1a9 as libc::c_int as OnigCodePoint,
                             index: 679 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa9 as libc::c_int as OnigCodePoint,
                             index: 214 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ab as libc::c_int as OnigCodePoint,
                             index: 2778 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a9 as libc::c_int as OnigCodePoint,
                             index: 820 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x212b as libc::c_int as OnigCodePoint,
                             index: 92 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba9 as libc::c_int as OnigCodePoint,
                             index: 1619 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e88 as libc::c_int as OnigCodePoint,
                             index: 1917 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca9 as libc::c_int as OnigCodePoint,
                             index: 3480 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x21a as libc::c_int as OnigCodePoint,
                             index: 541 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f88 as libc::c_int as OnigCodePoint,
                             index: 129 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c88 as libc::c_int as OnigCodePoint,
                             index: 2598 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x388 as libc::c_int as OnigCodePoint,
                             index: 730 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13fd as libc::c_int as OnigCodePoint,
                             index: 1703 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab88 as libc::c_int as OnigCodePoint,
                             index: 1520 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a9 as libc::c_int as OnigCodePoint,
                             index: 2772 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c88 as libc::c_int as OnigCodePoint,
                             index: 3381 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x218 as libc::c_int as OnigCodePoint,
                             index: 538 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x500 as libc::c_int as OnigCodePoint,
                             index: 1262 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f4d as libc::c_int as OnigCodePoint,
                             index: 2187 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1a7 as libc::c_int as OnigCodePoint,
                             index: 393 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa7 as libc::c_int as OnigCodePoint,
                             index: 244 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4d as libc::c_int as OnigCodePoint,
                             index: 34 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a7 as libc::c_int as OnigCodePoint,
                             index: 814 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa688 as libc::c_int as OnigCodePoint,
                             index: 2946 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba7 as libc::c_int as OnigCodePoint,
                             index: 1613 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x20e as libc::c_int as OnigCodePoint,
                             index: 523 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca7 as libc::c_int as OnigCodePoint,
                             index: 3474 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e6a as libc::c_int as OnigCodePoint,
                             index: 1872 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x46a as libc::c_int as OnigCodePoint,
                             index: 1049 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x16a as libc::c_int as OnigCodePoint,
                             index: 324 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6a as libc::c_int as OnigCodePoint,
                             index: 2208 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216c as libc::c_int as OnigCodePoint,
                             index: 2328 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a7 as libc::c_int as OnigCodePoint,
                             index: 2766 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1d1 as libc::c_int as OnigCodePoint,
                             index: 435 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa76a as libc::c_int as OnigCodePoint,
                             index: 3081 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x20c as libc::c_int as OnigCodePoint,
                             index: 520 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3d1 as libc::c_int as OnigCodePoint,
                             index: 762 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd1 as libc::c_int as OnigCodePoint,
                             index: 129 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e68 as libc::c_int as OnigCodePoint,
                             index: 1869 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x468 as libc::c_int as OnigCodePoint,
                             index: 1046 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x168 as libc::c_int as OnigCodePoint,
                             index: 321 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f68 as libc::c_int as OnigCodePoint,
                             index: 2202 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff31 as libc::c_int as OnigCodePoint,
                             index: 3207 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa66a as libc::c_int as OnigCodePoint,
                             index: 2928 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x208 as libc::c_int as OnigCodePoint,
                             index: 514 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa768 as libc::c_int as OnigCodePoint,
                             index: 3078 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e64 as libc::c_int as OnigCodePoint,
                             index: 1863 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x464 as libc::c_int as OnigCodePoint,
                             index: 1040 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x164 as libc::c_int as OnigCodePoint,
                             index: 315 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54d as libc::c_int as OnigCodePoint,
                             index: 1418 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c64 as libc::c_int as OnigCodePoint,
                             index: 673 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff2b as libc::c_int as OnigCodePoint,
                             index: 3189 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa764 as libc::c_int as OnigCodePoint,
                             index: 3072 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa668 as libc::c_int as OnigCodePoint,
                             index: 2925 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216 as libc::c_int as OnigCodePoint,
                             index: 535 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118ab as libc::c_int as OnigCodePoint,
                             index: 3543 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e62 as libc::c_int as OnigCodePoint,
                             index: 1860 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x462 as libc::c_int as OnigCodePoint,
                             index: 1037 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x162 as libc::c_int as OnigCodePoint,
                             index: 312 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x214 as libc::c_int as OnigCodePoint,
                             index: 532 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c62 as libc::c_int as OnigCodePoint,
                             index: 655 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa664 as libc::c_int as OnigCodePoint,
                             index: 2919 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ed2 as libc::c_int as OnigCodePoint,
                             index: 2013 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4d2 as libc::c_int as OnigCodePoint,
                             index: 1193 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa762 as libc::c_int as OnigCodePoint,
                             index: 3069 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fd2 as libc::c_int as OnigCodePoint,
                             index: 20 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cd2 as libc::c_int as OnigCodePoint,
                             index: 2709 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a9 as libc::c_int as OnigCodePoint,
                             index: 3537 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd2 as libc::c_int as OnigCodePoint,
                             index: 132 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x206 as libc::c_int as OnigCodePoint,
                             index: 511 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10420 as libc::c_int as OnigCodePoint,
                             index: 3333 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e20 as libc::c_int as OnigCodePoint,
                             index: 1760 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x420 as libc::c_int as OnigCodePoint,
                             index: 938 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x120 as libc::c_int as OnigCodePoint,
                             index: 219 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa662 as libc::c_int as OnigCodePoint,
                             index: 2916 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c20 as libc::c_int as OnigCodePoint,
                             index: 2517 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e60 as libc::c_int as OnigCodePoint,
                             index: 1856 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x460 as libc::c_int as OnigCodePoint,
                             index: 1034 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x160 as libc::c_int as OnigCodePoint,
                             index: 309 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x204 as libc::c_int as OnigCodePoint,
                             index: 508 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c60 as libc::c_int as OnigCodePoint,
                             index: 2562 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24bd as libc::c_int as OnigCodePoint,
                             index: 2364 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216a as libc::c_int as OnigCodePoint,
                             index: 2322 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa760 as libc::c_int as OnigCodePoint,
                             index: 3066 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb16 as libc::c_int as OnigCodePoint,
                             index: 125 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a7 as libc::c_int as OnigCodePoint,
                             index: 3531 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1efa as libc::c_int as OnigCodePoint,
                             index: 2073 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4fa as libc::c_int as OnigCodePoint,
                             index: 1253 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa as libc::c_int as OnigCodePoint,
                             index: 493 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ffa as libc::c_int as OnigCodePoint,
                             index: 2262 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb14 as libc::c_int as OnigCodePoint,
                             index: 109 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3fa as libc::c_int as OnigCodePoint,
                             index: 887 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa660 as libc::c_int as OnigCodePoint,
                             index: 2913 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2168 as libc::c_int as OnigCodePoint,
                             index: 2316 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1b7 as libc::c_int as OnigCodePoint,
                             index: 700 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb7 as libc::c_int as OnigCodePoint,
                             index: 10 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6b as libc::c_int as OnigCodePoint,
                             index: 2211 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c6b as libc::c_int as OnigCodePoint,
                             index: 2577 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x200 as libc::c_int as OnigCodePoint,
                             index: 502 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb7 as libc::c_int as OnigCodePoint,
                             index: 1661 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb06 as libc::c_int as OnigCodePoint,
                             index: 29 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e56 as libc::c_int as OnigCodePoint,
                             index: 1841 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2164 as libc::c_int as OnigCodePoint,
                             index: 2304 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x156 as libc::c_int as OnigCodePoint,
                             index: 294 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f56 as libc::c_int as OnigCodePoint,
                             index: 62 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x520 as libc::c_int as OnigCodePoint,
                             index: 1310 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4f as libc::c_int as OnigCodePoint,
                             index: 40 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x56 as libc::c_int as OnigCodePoint,
                             index: 62 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b7 as libc::c_int as OnigCodePoint,
                             index: 2814 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa756 as libc::c_int as OnigCodePoint,
                             index: 3051 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb04 as libc::c_int as OnigCodePoint,
                             index: 5 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e78 as libc::c_int as OnigCodePoint,
                             index: 1893 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x478 as libc::c_int as OnigCodePoint,
                             index: 1070 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x178 as libc::c_int as OnigCodePoint,
                             index: 168 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e54 as libc::c_int as OnigCodePoint,
                             index: 1838 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2162 as libc::c_int as OnigCodePoint,
                             index: 2298 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x154 as libc::c_int as OnigCodePoint,
                             index: 291 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f54 as libc::c_int as OnigCodePoint,
                             index: 57 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab78 as libc::c_int as OnigCodePoint,
                             index: 1472 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa656 as libc::c_int as OnigCodePoint,
                             index: 2898 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54 as libc::c_int as OnigCodePoint,
                             index: 56 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e52 as libc::c_int as OnigCodePoint,
                             index: 1835 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa754 as libc::c_int as OnigCodePoint,
                             index: 3048 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x152 as libc::c_int as OnigCodePoint,
                             index: 288 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f52 as libc::c_int as OnigCodePoint,
                             index: 52 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c9 as libc::c_int as OnigCodePoint,
                             index: 2400 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e32 as libc::c_int as OnigCodePoint,
                             index: 1787 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x52 as libc::c_int as OnigCodePoint,
                             index: 49 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x132 as libc::c_int as OnigCodePoint,
                             index: 243 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa752 as libc::c_int as OnigCodePoint,
                             index: 3045 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb00 as libc::c_int as OnigCodePoint,
                             index: 4 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa654 as libc::c_int as OnigCodePoint,
                             index: 2895 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa732 as libc::c_int as OnigCodePoint,
                             index: 2997 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2160 as libc::c_int as OnigCodePoint,
                             index: 2292 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54f as libc::c_int as OnigCodePoint,
                             index: 1424 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x556 as libc::c_int as OnigCodePoint,
                             index: 1445 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e50 as libc::c_int as OnigCodePoint,
                             index: 1832 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa652 as libc::c_int as OnigCodePoint,
                             index: 2892 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x150 as libc::c_int as OnigCodePoint,
                             index: 285 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f50 as libc::c_int as OnigCodePoint,
                             index: 84 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x17b as libc::c_int as OnigCodePoint,
                             index: 348 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e4e as libc::c_int as OnigCodePoint,
                             index: 1829 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x50 as libc::c_int as OnigCodePoint,
                             index: 43 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x14e as libc::c_int as OnigCodePoint,
                             index: 282 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa750 as libc::c_int as OnigCodePoint,
                             index: 3042 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab7b as libc::c_int as OnigCodePoint,
                             index: 1481 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa77b as libc::c_int as OnigCodePoint,
                             index: 3093 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4e as libc::c_int as OnigCodePoint,
                             index: 37 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x554 as libc::c_int as OnigCodePoint,
                             index: 1439 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa74e as libc::c_int as OnigCodePoint,
                             index: 3039 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e48 as libc::c_int as OnigCodePoint,
                             index: 1820 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x216b as libc::c_int as OnigCodePoint,
                             index: 2325 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f48 as libc::c_int as OnigCodePoint,
                             index: 2172 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa650 as libc::c_int as OnigCodePoint,
                             index: 2889 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x552 as libc::c_int as OnigCodePoint,
                             index: 1433 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x48 as libc::c_int as OnigCodePoint,
                             index: 21 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa748 as libc::c_int as OnigCodePoint,
                             index: 3030 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa64e as libc::c_int as OnigCodePoint,
                             index: 2886 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x532 as libc::c_int as OnigCodePoint,
                             index: 1337 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1041e as libc::c_int as OnigCodePoint,
                             index: 3327 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e1e as libc::c_int as OnigCodePoint,
                             index: 1757 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41e as libc::c_int as OnigCodePoint,
                             index: 932 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x11e as libc::c_int as OnigCodePoint,
                             index: 216 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b7 as libc::c_int as OnigCodePoint,
                             index: 3579 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c1e as libc::c_int as OnigCodePoint,
                             index: 2511 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa648 as libc::c_int as OnigCodePoint,
                             index: 2877 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff9 as libc::c_int as OnigCodePoint,
                             index: 2253 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3f9 as libc::c_int as OnigCodePoint,
                             index: 878 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x550 as libc::c_int as OnigCodePoint,
                             index: 1427 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10412 as libc::c_int as OnigCodePoint,
                             index: 3291 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e12 as libc::c_int as OnigCodePoint,
                             index: 1739 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x412 as libc::c_int as OnigCodePoint,
                             index: 896 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x112 as libc::c_int as OnigCodePoint,
                             index: 198 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54e as libc::c_int as OnigCodePoint,
                             index: 1421 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c12 as libc::c_int as OnigCodePoint,
                             index: 2475 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10410 as libc::c_int as OnigCodePoint,
                             index: 3285 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e10 as libc::c_int as OnigCodePoint,
                             index: 1736 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x410 as libc::c_int as OnigCodePoint,
                             index: 890 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x110 as libc::c_int as OnigCodePoint,
                             index: 195 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c10 as libc::c_int as OnigCodePoint,
                             index: 2469 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2132 as libc::c_int as OnigCodePoint,
                             index: 2289 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x548 as libc::c_int as OnigCodePoint,
                             index: 1403 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ef8 as libc::c_int as OnigCodePoint,
                             index: 2070 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4f8 as libc::c_int as OnigCodePoint,
                             index: 1250 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8 as libc::c_int as OnigCodePoint,
                             index: 490 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff8 as libc::c_int as OnigCodePoint,
                             index: 2250 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x220 as libc::c_int as OnigCodePoint,
                             index: 381 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ee2 as libc::c_int as OnigCodePoint,
                             index: 2037 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4e2 as libc::c_int as OnigCodePoint,
                             index: 1217 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e2 as libc::c_int as OnigCodePoint,
                             index: 462 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe2 as libc::c_int as OnigCodePoint,
                             index: 36 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ce2 as libc::c_int as OnigCodePoint,
                             index: 2733 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3e2 as libc::c_int as OnigCodePoint,
                             index: 857 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x51e as libc::c_int as OnigCodePoint,
                             index: 1307 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ede as libc::c_int as OnigCodePoint,
                             index: 2031 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4de as libc::c_int as OnigCodePoint,
                             index: 1211 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1de as libc::c_int as OnigCodePoint,
                             index: 456 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cde as libc::c_int as OnigCodePoint,
                             index: 2727 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3de as libc::c_int as OnigCodePoint,
                             index: 851 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xde as libc::c_int as OnigCodePoint,
                             index: 165 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f69 as libc::c_int as OnigCodePoint,
                             index: 2205 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c69 as libc::c_int as OnigCodePoint,
                             index: 2574 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eda as libc::c_int as OnigCodePoint,
                             index: 2025 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4da as libc::c_int as OnigCodePoint,
                             index: 1205 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x512 as libc::c_int as OnigCodePoint,
                             index: 1289 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fda as libc::c_int as OnigCodePoint,
                             index: 2244 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cda as libc::c_int as OnigCodePoint,
                             index: 2721 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3da as libc::c_int as OnigCodePoint,
                             index: 845 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xda as libc::c_int as OnigCodePoint,
                             index: 153 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x510 as libc::c_int as OnigCodePoint,
                             index: 1286 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ed8 as libc::c_int as OnigCodePoint,
                             index: 2022 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4d8 as libc::c_int as OnigCodePoint,
                             index: 1202 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fd8 as libc::c_int as OnigCodePoint,
                             index: 2274 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cd8 as libc::c_int as OnigCodePoint,
                             index: 2718 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3d8 as libc::c_int as OnigCodePoint,
                             index: 842 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd8 as libc::c_int as OnigCodePoint,
                             index: 147 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ed6 as libc::c_int as OnigCodePoint,
                             index: 2019 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4d6 as libc::c_int as OnigCodePoint,
                             index: 1199 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fd6 as libc::c_int as OnigCodePoint,
                             index: 76 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cd6 as libc::c_int as OnigCodePoint,
                             index: 2715 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3d6 as libc::c_int as OnigCodePoint,
                             index: 792 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd6 as libc::c_int as OnigCodePoint,
                             index: 144 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ec8 as libc::c_int as OnigCodePoint,
                             index: 1998 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1c8 as libc::c_int as OnigCodePoint,
                             index: 421 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc8 as libc::c_int as OnigCodePoint,
                             index: 2232 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cc8 as libc::c_int as OnigCodePoint,
                             index: 2694 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff32 as libc::c_int as OnigCodePoint,
                             index: 3210 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc8 as libc::c_int as OnigCodePoint,
                             index: 102 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c7 as libc::c_int as OnigCodePoint,
                             index: 1175 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1c7 as libc::c_int as OnigCodePoint,
                             index: 421 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc7 as libc::c_int as OnigCodePoint,
                             index: 15 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ec0 as libc::c_int as OnigCodePoint,
                             index: 1986 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c0 as libc::c_int as OnigCodePoint,
                             index: 1187 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc7 as libc::c_int as OnigCodePoint,
                             index: 99 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cc0 as libc::c_int as OnigCodePoint,
                             index: 2682 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x179 as libc::c_int as OnigCodePoint,
                             index: 345 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc0 as libc::c_int as OnigCodePoint,
                             index: 77 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x232 as libc::c_int as OnigCodePoint,
                             index: 574 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1b3 as libc::c_int as OnigCodePoint,
                             index: 402 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb3 as libc::c_int as OnigCodePoint,
                             index: 62 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab79 as libc::c_int as OnigCodePoint,
                             index: 1475 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa779 as libc::c_int as OnigCodePoint,
                             index: 3090 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c7 as libc::c_int as OnigCodePoint,
                             index: 2859 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb3 as libc::c_int as OnigCodePoint,
                             index: 1649 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7b3 as libc::c_int as OnigCodePoint,
                             index: 3156 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa5 as libc::c_int as OnigCodePoint,
                             index: 234 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c0 as libc::c_int as OnigCodePoint,
                             index: 2841 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a5 as libc::c_int as OnigCodePoint,
                             index: 807 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba5 as libc::c_int as OnigCodePoint,
                             index: 1607 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1b1 as libc::c_int as OnigCodePoint,
                             index: 691 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca5 as libc::c_int as OnigCodePoint,
                             index: 3468 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b3 as libc::c_int as OnigCodePoint,
                             index: 2802 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2169 as libc::c_int as OnigCodePoint,
                             index: 2319 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24e as libc::c_int as OnigCodePoint,
                             index: 601 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb1 as libc::c_int as OnigCodePoint,
                             index: 1643 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7b1 as libc::c_int as OnigCodePoint,
                             index: 682 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cb1 as libc::c_int as OnigCodePoint,
                             index: 3504 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a5 as libc::c_int as OnigCodePoint,
                             index: 2760 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1af as libc::c_int as OnigCodePoint,
                             index: 399 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1faf as libc::c_int as OnigCodePoint,
                             index: 244 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x248 as libc::c_int as OnigCodePoint,
                             index: 592 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b1 as libc::c_int as OnigCodePoint,
                             index: 2796 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabaf as libc::c_int as OnigCodePoint,
                             index: 1637 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fad as libc::c_int as OnigCodePoint,
                             index: 234 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10caf as libc::c_int as OnigCodePoint,
                             index: 3498 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4cd as libc::c_int as OnigCodePoint,
                             index: 1184 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1cd as libc::c_int as OnigCodePoint,
                             index: 429 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabad as libc::c_int as OnigCodePoint,
                             index: 1631 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa7ad as libc::c_int as OnigCodePoint,
                             index: 658 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cad as libc::c_int as OnigCodePoint,
                             index: 3492 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xcd as libc::c_int as OnigCodePoint,
                             index: 117 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10af as libc::c_int as OnigCodePoint,
                             index: 2790 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x21e as libc::c_int as OnigCodePoint,
                             index: 547 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa3 as libc::c_int as OnigCodePoint,
                             index: 224 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a3 as libc::c_int as OnigCodePoint,
                             index: 800 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ad as libc::c_int as OnigCodePoint,
                             index: 2784 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba3 as libc::c_int as OnigCodePoint,
                             index: 1601 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca3 as libc::c_int as OnigCodePoint,
                             index: 3462 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10cd as libc::c_int as OnigCodePoint,
                             index: 2862 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fa1 as libc::c_int as OnigCodePoint,
                             index: 214 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24b7 as libc::c_int as OnigCodePoint,
                             index: 2346 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3a1 as libc::c_int as OnigCodePoint,
                             index: 796 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x212 as libc::c_int as OnigCodePoint,
                             index: 529 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xaba1 as libc::c_int as OnigCodePoint,
                             index: 1595 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a3 as libc::c_int as OnigCodePoint,
                             index: 2754 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10ca1 as libc::c_int as OnigCodePoint,
                             index: 3456 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1d3 as libc::c_int as OnigCodePoint,
                             index: 438 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fd3 as libc::c_int as OnigCodePoint,
                             index: 25 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x210 as libc::c_int as OnigCodePoint,
                             index: 526 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd3 as libc::c_int as OnigCodePoint,
                             index: 135 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e97 as libc::c_int as OnigCodePoint,
                             index: 34 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10a1 as libc::c_int as OnigCodePoint,
                             index: 2748 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x197 as libc::c_int as OnigCodePoint,
                             index: 649 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f97 as libc::c_int as OnigCodePoint,
                             index: 204 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x397 as libc::c_int as OnigCodePoint,
                             index: 759 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1041d as libc::c_int as OnigCodePoint,
                             index: 3324 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab97 as libc::c_int as OnigCodePoint,
                             index: 1565 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41d as libc::c_int as OnigCodePoint,
                             index: 929 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c97 as libc::c_int as OnigCodePoint,
                             index: 3426 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f1d as libc::c_int as OnigCodePoint,
                             index: 2121 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c1d as libc::c_int as OnigCodePoint,
                             index: 2508 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e72 as libc::c_int as OnigCodePoint,
                             index: 1884 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x472 as libc::c_int as OnigCodePoint,
                             index: 1061 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x172 as libc::c_int as OnigCodePoint,
                             index: 336 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b3 as libc::c_int as OnigCodePoint,
                             index: 3567 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c72 as libc::c_int as OnigCodePoint,
                             index: 2580 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x372 as libc::c_int as OnigCodePoint,
                             index: 712 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1041b as libc::c_int as OnigCodePoint,
                             index: 3318 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab72 as libc::c_int as OnigCodePoint,
                             index: 1454 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41b as libc::c_int as OnigCodePoint,
                             index: 923 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a5 as libc::c_int as OnigCodePoint,
                             index: 3525 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f1b as libc::c_int as OnigCodePoint,
                             index: 2115 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c1b as libc::c_int as OnigCodePoint,
                             index: 2502 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e70 as libc::c_int as OnigCodePoint,
                             index: 1881 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x470 as libc::c_int as OnigCodePoint,
                             index: 1058 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x170 as libc::c_int as OnigCodePoint,
                             index: 333 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b1 as libc::c_int as OnigCodePoint,
                             index: 3561 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c70 as libc::c_int as OnigCodePoint,
                             index: 610 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x370 as libc::c_int as OnigCodePoint,
                             index: 709 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e46 as libc::c_int as OnigCodePoint,
                             index: 1817 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab70 as libc::c_int as OnigCodePoint,
                             index: 1448 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e66 as libc::c_int as OnigCodePoint,
                             index: 1866 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x466 as libc::c_int as OnigCodePoint,
                             index: 1043 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x166 as libc::c_int as OnigCodePoint,
                             index: 318 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e44 as libc::c_int as OnigCodePoint,
                             index: 1814 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x46 as libc::c_int as OnigCodePoint,
                             index: 15 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118af as libc::c_int as OnigCodePoint,
                             index: 3555 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa746 as libc::c_int as OnigCodePoint,
                             index: 3027 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa766 as libc::c_int as OnigCodePoint,
                             index: 3075 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x44 as libc::c_int as OnigCodePoint,
                             index: 9 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118ad as libc::c_int as OnigCodePoint,
                             index: 3549 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa744 as libc::c_int as OnigCodePoint,
                             index: 3024 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e7a as libc::c_int as OnigCodePoint,
                             index: 1896 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x47a as libc::c_int as OnigCodePoint,
                             index: 1073 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e3a as libc::c_int as OnigCodePoint,
                             index: 1799 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa646 as libc::c_int as OnigCodePoint,
                             index: 2874 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f3a as libc::c_int as OnigCodePoint,
                             index: 2154 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa666 as libc::c_int as OnigCodePoint,
                             index: 2922 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab7a as libc::c_int as OnigCodePoint,
                             index: 1478 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a3 as libc::c_int as OnigCodePoint,
                             index: 3519 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa644 as libc::c_int as OnigCodePoint,
                             index: 2871 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa73a as libc::c_int as OnigCodePoint,
                             index: 3009 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ef4 as libc::c_int as OnigCodePoint,
                             index: 2064 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4f4 as libc::c_int as OnigCodePoint,
                             index: 1244 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f4 as libc::c_int as OnigCodePoint,
                             index: 487 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff4 as libc::c_int as OnigCodePoint,
                             index: 101 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118a1 as libc::c_int as OnigCodePoint,
                             index: 3513 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3f4 as libc::c_int as OnigCodePoint,
                             index: 762 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eec as libc::c_int as OnigCodePoint,
                             index: 2052 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4ec as libc::c_int as OnigCodePoint,
                             index: 1232 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ec as libc::c_int as OnigCodePoint,
                             index: 477 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fec as libc::c_int as OnigCodePoint,
                             index: 2286 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x546 as libc::c_int as OnigCodePoint,
                             index: 1397 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3ec as libc::c_int as OnigCodePoint,
                             index: 872 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13f as libc::c_int as OnigCodePoint,
                             index: 261 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f3f as libc::c_int as OnigCodePoint,
                             index: 2169 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x544 as libc::c_int as OnigCodePoint,
                             index: 1391 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eea as libc::c_int as OnigCodePoint,
                             index: 2049 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4ea as libc::c_int as OnigCodePoint,
                             index: 1229 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ea as libc::c_int as OnigCodePoint,
                             index: 474 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fea as libc::c_int as OnigCodePoint,
                             index: 2256 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3ea as libc::c_int as OnigCodePoint,
                             index: 869 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ee8 as libc::c_int as OnigCodePoint,
                             index: 2046 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4e8 as libc::c_int as OnigCodePoint,
                             index: 1226 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e8 as libc::c_int as OnigCodePoint,
                             index: 471 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe8 as libc::c_int as OnigCodePoint,
                             index: 2280 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53a as libc::c_int as OnigCodePoint,
                             index: 1361 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3e8 as libc::c_int as OnigCodePoint,
                             index: 866 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ee6 as libc::c_int as OnigCodePoint,
                             index: 2043 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4e6 as libc::c_int as OnigCodePoint,
                             index: 1223 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e6 as libc::c_int as OnigCodePoint,
                             index: 468 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe6 as libc::c_int as OnigCodePoint,
                             index: 88 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f4b as libc::c_int as OnigCodePoint,
                             index: 2181 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3e6 as libc::c_int as OnigCodePoint,
                             index: 863 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e5e as libc::c_int as OnigCodePoint,
                             index: 1853 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4b as libc::c_int as OnigCodePoint,
                             index: 27 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x15e as libc::c_int as OnigCodePoint,
                             index: 306 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2166 as libc::c_int as OnigCodePoint,
                             index: 2310 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ee4 as libc::c_int as OnigCodePoint,
                             index: 2040 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4e4 as libc::c_int as OnigCodePoint,
                             index: 1220 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e4 as libc::c_int as OnigCodePoint,
                             index: 465 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fe4 as libc::c_int as OnigCodePoint,
                             index: 80 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa75e as libc::c_int as OnigCodePoint,
                             index: 3063 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3e4 as libc::c_int as OnigCodePoint,
                             index: 860 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ee0 as libc::c_int as OnigCodePoint,
                             index: 2034 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4e0 as libc::c_int as OnigCodePoint,
                             index: 1214 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e0 as libc::c_int as OnigCodePoint,
                             index: 459 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53f as libc::c_int as OnigCodePoint,
                             index: 1376 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ce0 as libc::c_int as OnigCodePoint,
                             index: 2730 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3e0 as libc::c_int as OnigCodePoint,
                             index: 854 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1edc as libc::c_int as OnigCodePoint,
                             index: 2028 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4dc as libc::c_int as OnigCodePoint,
                             index: 1208 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa65e as libc::c_int as OnigCodePoint,
                             index: 2910 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cdc as libc::c_int as OnigCodePoint,
                             index: 2724 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3dc as libc::c_int as OnigCodePoint,
                             index: 848 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xdc as libc::c_int as OnigCodePoint,
                             index: 159 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ed0 as libc::c_int as OnigCodePoint,
                             index: 2010 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4d0 as libc::c_int as OnigCodePoint,
                             index: 1190 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cd0 as libc::c_int as OnigCodePoint,
                             index: 2706 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3d0 as libc::c_int as OnigCodePoint,
                             index: 742 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd0 as libc::c_int as OnigCodePoint,
                             index: 126 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ecc as libc::c_int as OnigCodePoint,
                             index: 2004 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x54b as libc::c_int as OnigCodePoint,
                             index: 1412 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fcc as libc::c_int as OnigCodePoint,
                             index: 71 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2ccc as libc::c_int as OnigCodePoint,
                             index: 2700 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ec6 as libc::c_int as OnigCodePoint,
                             index: 1995 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xcc as libc::c_int as OnigCodePoint,
                             index: 114 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc6 as libc::c_int as OnigCodePoint,
                             index: 67 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cc6 as libc::c_int as OnigCodePoint,
                             index: 2691 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c8 as libc::c_int as OnigCodePoint,
                             index: 2397 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc6 as libc::c_int as OnigCodePoint,
                             index: 96 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c5 as libc::c_int as OnigCodePoint,
                             index: 1172 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1c5 as libc::c_int as OnigCodePoint,
                             index: 417 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fbb as libc::c_int as OnigCodePoint,
                             index: 2229 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c7 as libc::c_int as OnigCodePoint,
                             index: 2394 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc5 as libc::c_int as OnigCodePoint,
                             index: 92 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fb9 as libc::c_int as OnigCodePoint,
                             index: 2271 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabbb as libc::c_int as OnigCodePoint,
                             index: 1673 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c0 as libc::c_int as OnigCodePoint,
                             index: 2373 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c3 as libc::c_int as OnigCodePoint,
                             index: 1169 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xabb9 as libc::c_int as OnigCodePoint,
                             index: 1667 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1fc3 as libc::c_int as OnigCodePoint,
                             index: 71 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc3 as libc::c_int as OnigCodePoint,
                             index: 86 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c5 as libc::c_int as OnigCodePoint,
                             index: 2856 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10bb as libc::c_int as OnigCodePoint,
                             index: 2826 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ed4 as libc::c_int as OnigCodePoint,
                             index: 2016 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4d4 as libc::c_int as OnigCodePoint,
                             index: 1196 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10b9 as libc::c_int as OnigCodePoint,
                             index: 2820 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13fc as libc::c_int as OnigCodePoint,
                             index: 1700 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cd4 as libc::c_int as OnigCodePoint,
                             index: 2712 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x246 as libc::c_int as OnigCodePoint,
                             index: 589 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xd4 as libc::c_int as OnigCodePoint,
                             index: 138 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c3 as libc::c_int as OnigCodePoint,
                             index: 2850 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff3a as libc::c_int as OnigCodePoint,
                             index: 3234 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x244 as libc::c_int as OnigCodePoint,
                             index: 688 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x19f as libc::c_int as OnigCodePoint,
                             index: 670 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f9f as libc::c_int as OnigCodePoint,
                             index: 204 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x39f as libc::c_int as OnigCodePoint,
                             index: 789 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab9f as libc::c_int as OnigCodePoint,
                             index: 1589 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c9f as libc::c_int as OnigCodePoint,
                             index: 3450 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x19d as libc::c_int as OnigCodePoint,
                             index: 667 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f9d as libc::c_int as OnigCodePoint,
                             index: 194 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x23a as libc::c_int as OnigCodePoint,
                             index: 2565 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x39d as libc::c_int as OnigCodePoint,
                             index: 783 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e5a as libc::c_int as OnigCodePoint,
                             index: 1847 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab9d as libc::c_int as OnigCodePoint,
                             index: 1583 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x15a as libc::c_int as OnigCodePoint,
                             index: 300 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c9d as libc::c_int as OnigCodePoint,
                             index: 3444 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e9b as libc::c_int as OnigCodePoint,
                             index: 1856 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24cd as libc::c_int as OnigCodePoint,
                             index: 2412 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x5a as libc::c_int as OnigCodePoint,
                             index: 74 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f9b as libc::c_int as OnigCodePoint,
                             index: 184 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa75a as libc::c_int as OnigCodePoint,
                             index: 3057 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x39b as libc::c_int as OnigCodePoint,
                             index: 776 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ece as libc::c_int as OnigCodePoint,
                             index: 2007 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab9b as libc::c_int as OnigCodePoint,
                             index: 1577 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e99 as libc::c_int as OnigCodePoint,
                             index: 42 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c9b as libc::c_int as OnigCodePoint,
                             index: 3438 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2cce as libc::c_int as OnigCodePoint,
                             index: 2703 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f99 as libc::c_int as OnigCodePoint,
                             index: 174 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xce as libc::c_int as OnigCodePoint,
                             index: 120 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x399 as libc::c_int as OnigCodePoint,
                             index: 767 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa65a as libc::c_int as OnigCodePoint,
                             index: 2904 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab99 as libc::c_int as OnigCodePoint,
                             index: 1571 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c99 as libc::c_int as OnigCodePoint,
                             index: 3432 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x193 as libc::c_int as OnigCodePoint,
                             index: 634 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f93 as libc::c_int as OnigCodePoint,
                             index: 184 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e58 as libc::c_int as OnigCodePoint,
                             index: 1844 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x393 as libc::c_int as OnigCodePoint,
                             index: 746 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x158 as libc::c_int as OnigCodePoint,
                             index: 297 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab93 as libc::c_int as OnigCodePoint,
                             index: 1553 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c93 as libc::c_int as OnigCodePoint,
                             index: 3414 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x58 as libc::c_int as OnigCodePoint,
                             index: 68 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42d as libc::c_int as OnigCodePoint,
                             index: 977 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa758 as libc::c_int as OnigCodePoint,
                             index: 3054 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2d as libc::c_int as OnigCodePoint,
                             index: 2139 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c2d as libc::c_int as OnigCodePoint,
                             index: 2556 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118bb as libc::c_int as OnigCodePoint,
                             index: 3591 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x191 as libc::c_int as OnigCodePoint,
                             index: 369 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f91 as libc::c_int as OnigCodePoint,
                             index: 174 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x118b9 as libc::c_int as OnigCodePoint,
                             index: 3585 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x391 as libc::c_int as OnigCodePoint,
                             index: 739 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab91 as libc::c_int as OnigCodePoint,
                             index: 1547 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa658 as libc::c_int as OnigCodePoint,
                             index: 2901 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c91 as libc::c_int as OnigCodePoint,
                             index: 3408 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x18f as libc::c_int as OnigCodePoint,
                             index: 625 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8f as libc::c_int as OnigCodePoint,
                             index: 164 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x38f as libc::c_int as OnigCodePoint,
                             index: 836 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab8f as libc::c_int as OnigCodePoint,
                             index: 1541 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c8f as libc::c_int as OnigCodePoint,
                             index: 3402 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x18b as libc::c_int as OnigCodePoint,
                             index: 366 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8b as libc::c_int as OnigCodePoint,
                             index: 144 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x187 as libc::c_int as OnigCodePoint,
                             index: 363 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f87 as libc::c_int as OnigCodePoint,
                             index: 164 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab8b as libc::c_int as OnigCodePoint,
                             index: 1529 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa78b as libc::c_int as OnigCodePoint,
                             index: 3111 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c8b as libc::c_int as OnigCodePoint,
                             index: 3390 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab87 as libc::c_int as OnigCodePoint,
                             index: 1517 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4c1 as libc::c_int as OnigCodePoint,
                             index: 1166 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c87 as libc::c_int as OnigCodePoint,
                             index: 3378 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e7e as libc::c_int as OnigCodePoint,
                             index: 1902 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x47e as libc::c_int as OnigCodePoint,
                             index: 1079 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xc1 as libc::c_int as OnigCodePoint,
                             index: 80 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c7e as libc::c_int as OnigCodePoint,
                             index: 580 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab7e as libc::c_int as OnigCodePoint,
                             index: 1490 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa77e as libc::c_int as OnigCodePoint,
                             index: 3096 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e76 as libc::c_int as OnigCodePoint,
                             index: 1890 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x476 as libc::c_int as OnigCodePoint,
                             index: 1067 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x176 as libc::c_int as OnigCodePoint,
                             index: 342 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e42 as libc::c_int as OnigCodePoint,
                             index: 1811 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c1 as libc::c_int as OnigCodePoint,
                             index: 2844 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x376 as libc::c_int as OnigCodePoint,
                             index: 715 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e36 as libc::c_int as OnigCodePoint,
                             index: 1793 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab76 as libc::c_int as OnigCodePoint,
                             index: 1466 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x136 as libc::c_int as OnigCodePoint,
                             index: 249 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42 as libc::c_int as OnigCodePoint,
                             index: 3 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e3e as libc::c_int as OnigCodePoint,
                             index: 1805 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa742 as libc::c_int as OnigCodePoint,
                             index: 3021 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e38 as libc::c_int as OnigCodePoint,
                             index: 1796 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f3e as libc::c_int as OnigCodePoint,
                             index: 2166 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa736 as libc::c_int as OnigCodePoint,
                             index: 3003 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f38 as libc::c_int as OnigCodePoint,
                             index: 2148 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x587 as libc::c_int as OnigCodePoint,
                             index: 105 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa73e as libc::c_int as OnigCodePoint,
                             index: 3015 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa738 as libc::c_int as OnigCodePoint,
                             index: 3006 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa642 as libc::c_int as OnigCodePoint,
                             index: 2868 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e5c as libc::c_int as OnigCodePoint,
                             index: 1850 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e34 as libc::c_int as OnigCodePoint,
                             index: 1790 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x15c as libc::c_int as OnigCodePoint,
                             index: 303 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x134 as libc::c_int as OnigCodePoint,
                             index: 246 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ef6 as libc::c_int as OnigCodePoint,
                             index: 2067 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4f6 as libc::c_int as OnigCodePoint,
                             index: 1247 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f6 as libc::c_int as OnigCodePoint,
                             index: 372 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ff6 as libc::c_int as OnigCodePoint,
                             index: 92 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa75c as libc::c_int as OnigCodePoint,
                             index: 3060 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa734 as libc::c_int as OnigCodePoint,
                             index: 3000 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ef0 as libc::c_int as OnigCodePoint,
                             index: 2058 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4f0 as libc::c_int as OnigCodePoint,
                             index: 1238 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0 as libc::c_int as OnigCodePoint,
                             index: 20 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e30 as libc::c_int as OnigCodePoint,
                             index: 1784 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3f0 as libc::c_int as OnigCodePoint,
                             index: 772 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x130 as libc::c_int as OnigCodePoint,
                             index: 261 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x542 as libc::c_int as OnigCodePoint,
                             index: 1385 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa65c as libc::c_int as OnigCodePoint,
                             index: 2907 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f83 as libc::c_int as OnigCodePoint,
                             index: 144 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x536 as libc::c_int as OnigCodePoint,
                             index: 1349 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab83 as libc::c_int as OnigCodePoint,
                             index: 1505 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53e as libc::c_int as OnigCodePoint,
                             index: 1373 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c83 as libc::c_int as OnigCodePoint,
                             index: 3366 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x538 as libc::c_int as OnigCodePoint,
                             index: 1355 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1eee as libc::c_int as OnigCodePoint,
                             index: 2055 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x4ee as libc::c_int as OnigCodePoint,
                             index: 1235 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1ee as libc::c_int as OnigCodePoint,
                             index: 480 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f8d as libc::c_int as OnigCodePoint,
                             index: 154 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x3ee as libc::c_int as OnigCodePoint,
                             index: 875 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab8d as libc::c_int as OnigCodePoint,
                             index: 1535 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa78d as libc::c_int as OnigCodePoint,
                             index: 643 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c8d as libc::c_int as OnigCodePoint,
                             index: 3396 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x534 as libc::c_int as OnigCodePoint,
                             index: 1343 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x181 as libc::c_int as OnigCodePoint,
                             index: 613 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f81 as libc::c_int as OnigCodePoint,
                             index: 134 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13d as libc::c_int as OnigCodePoint,
                             index: 258 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f3d as libc::c_int as OnigCodePoint,
                             index: 2163 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab81 as libc::c_int as OnigCodePoint,
                             index: 1499 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x17f as libc::c_int as OnigCodePoint,
                             index: 52 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10c81 as libc::c_int as OnigCodePoint,
                             index: 3360 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c7f as libc::c_int as OnigCodePoint,
                             index: 583 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x37f as libc::c_int as OnigCodePoint,
                             index: 881 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff2d as libc::c_int as OnigCodePoint,
                             index: 3195 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab7f as libc::c_int as OnigCodePoint,
                             index: 1493 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e74 as libc::c_int as OnigCodePoint,
                             index: 1887 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x474 as libc::c_int as OnigCodePoint,
                             index: 1064 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x174 as libc::c_int as OnigCodePoint,
                             index: 339 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1e3c as libc::c_int as OnigCodePoint,
                             index: 1802 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x149 as libc::c_int as OnigCodePoint,
                             index: 46 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f49 as libc::c_int as OnigCodePoint,
                             index: 2175 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f3c as libc::c_int as OnigCodePoint,
                             index: 2160 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xab74 as libc::c_int as OnigCodePoint,
                             index: 1460 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x49 as libc::c_int as OnigCodePoint,
                             index: 3606 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x143 as libc::c_int as OnigCodePoint,
                             index: 267 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24cc as libc::c_int as OnigCodePoint,
                             index: 2409 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xa73c as libc::c_int as OnigCodePoint,
                             index: 3012 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x43 as libc::c_int as OnigCodePoint,
                             index: 6 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x141 as libc::c_int as OnigCodePoint,
                             index: 264 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c6 as libc::c_int as OnigCodePoint,
                             index: 2391 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13b as libc::c_int as OnigCodePoint,
                             index: 255 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f3b as libc::c_int as OnigCodePoint,
                             index: 2157 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41 as libc::c_int as OnigCodePoint,
                             index: 0 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x139 as libc::c_int as OnigCodePoint,
                             index: 252 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f39 as libc::c_int as OnigCodePoint,
                             index: 2151 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c5 as libc::c_int as OnigCodePoint,
                             index: 2388 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24bb as libc::c_int as OnigCodePoint,
                             index: 2358 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13fa as libc::c_int as OnigCodePoint,
                             index: 1694 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53d as libc::c_int as OnigCodePoint,
                             index: 1370 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24b9 as libc::c_int as OnigCodePoint,
                             index: 2352 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x429 as libc::c_int as OnigCodePoint,
                             index: 965 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2183 as libc::c_int as OnigCodePoint,
                             index: 2340 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f29 as libc::c_int as OnigCodePoint,
                             index: 2127 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c29 as libc::c_int as OnigCodePoint,
                             index: 2544 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c3 as libc::c_int as OnigCodePoint,
                             index: 2382 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10427 as libc::c_int as OnigCodePoint,
                             index: 3354 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10425 as libc::c_int as OnigCodePoint,
                             index: 3348 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x427 as libc::c_int as OnigCodePoint,
                             index: 959 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x425 as libc::c_int as OnigCodePoint,
                             index: 953 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c27 as libc::c_int as OnigCodePoint,
                             index: 2538 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c25 as libc::c_int as OnigCodePoint,
                             index: 2532 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x549 as libc::c_int as OnigCodePoint,
                             index: 1406 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53c as libc::c_int as OnigCodePoint,
                             index: 1367 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10423 as libc::c_int as OnigCodePoint,
                             index: 3342 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x423 as libc::c_int as OnigCodePoint,
                             index: 947 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x543 as libc::c_int as OnigCodePoint,
                             index: 1388 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c23 as libc::c_int as OnigCodePoint,
                             index: 2526 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff36 as libc::c_int as OnigCodePoint,
                             index: 3222 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x541 as libc::c_int as OnigCodePoint,
                             index: 1382 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10421 as libc::c_int as OnigCodePoint,
                             index: 3336 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x53b as libc::c_int as OnigCodePoint,
                             index: 1364 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x421 as libc::c_int as OnigCodePoint,
                             index: 941 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff38 as libc::c_int as OnigCodePoint,
                             index: 3228 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x539 as libc::c_int as OnigCodePoint,
                             index: 1358 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c21 as libc::c_int as OnigCodePoint,
                             index: 2520 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10419 as libc::c_int as OnigCodePoint,
                             index: 3312 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10417 as libc::c_int as OnigCodePoint,
                             index: 3306 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x419 as libc::c_int as OnigCodePoint,
                             index: 917 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x417 as libc::c_int as OnigCodePoint,
                             index: 911 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f19 as libc::c_int as OnigCodePoint,
                             index: 2109 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c19 as libc::c_int as OnigCodePoint,
                             index: 2496 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c17 as libc::c_int as OnigCodePoint,
                             index: 2490 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x23e as libc::c_int as OnigCodePoint,
                             index: 2568 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff34 as libc::c_int as OnigCodePoint,
                             index: 3216 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10415 as libc::c_int as OnigCodePoint,
                             index: 3300 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10413 as libc::c_int as OnigCodePoint,
                             index: 3294 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x415 as libc::c_int as OnigCodePoint,
                             index: 905 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x413 as libc::c_int as OnigCodePoint,
                             index: 899 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c15 as libc::c_int as OnigCodePoint,
                             index: 2484 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c13 as libc::c_int as OnigCodePoint,
                             index: 2478 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24ce as libc::c_int as OnigCodePoint,
                             index: 2415 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1040f as libc::c_int as OnigCodePoint,
                             index: 3282 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x40f as libc::c_int as OnigCodePoint,
                             index: 1031 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff30 as libc::c_int as OnigCodePoint,
                             index: 3204 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0f as libc::c_int as OnigCodePoint,
                             index: 2103 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c0f as libc::c_int as OnigCodePoint,
                             index: 2466 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1040d as libc::c_int as OnigCodePoint,
                             index: 3276 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x40d as libc::c_int as OnigCodePoint,
                             index: 1025 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x147 as libc::c_int as OnigCodePoint,
                             index: 273 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0d as libc::c_int as OnigCodePoint,
                             index: 2097 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c0d as libc::c_int as OnigCodePoint,
                             index: 2460 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1040b as libc::c_int as OnigCodePoint,
                             index: 3270 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x47 as libc::c_int as OnigCodePoint,
                             index: 18 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x40b as libc::c_int as OnigCodePoint,
                             index: 1019 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x230 as libc::c_int as OnigCodePoint,
                             index: 571 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f0b as libc::c_int as OnigCodePoint,
                             index: 2091 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c0b as libc::c_int as OnigCodePoint,
                             index: 2454 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10409 as libc::c_int as OnigCodePoint,
                             index: 3264 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10405 as libc::c_int as OnigCodePoint,
                             index: 3252 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x409 as libc::c_int as OnigCodePoint,
                             index: 1013 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x405 as libc::c_int as OnigCodePoint,
                             index: 1001 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f09 as libc::c_int as OnigCodePoint,
                             index: 2085 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c09 as libc::c_int as OnigCodePoint,
                             index: 2448 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c05 as libc::c_int as OnigCodePoint,
                             index: 2436 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10403 as libc::c_int as OnigCodePoint,
                             index: 3246 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10401 as libc::c_int as OnigCodePoint,
                             index: 3240 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x403 as libc::c_int as OnigCodePoint,
                             index: 995 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x401 as libc::c_int as OnigCodePoint,
                             index: 989 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c03 as libc::c_int as OnigCodePoint,
                             index: 2430 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c01 as libc::c_int as OnigCodePoint,
                             index: 2424 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13f9 as libc::c_int as OnigCodePoint,
                             index: 1691 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x42f as libc::c_int as OnigCodePoint,
                             index: 983 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1f2f as libc::c_int as OnigCodePoint,
                             index: 2145 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x1041f as libc::c_int as OnigCodePoint,
                             index: 3330 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x41f as libc::c_int as OnigCodePoint,
                             index: 935 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x23d as libc::c_int as OnigCodePoint,
                             index: 378 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10411 as libc::c_int as OnigCodePoint,
                             index: 3288 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c1f as libc::c_int as OnigCodePoint,
                             index: 2514 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x411 as libc::c_int as OnigCodePoint,
                             index: 893 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x547 as libc::c_int as OnigCodePoint,
                             index: 1400 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c11 as libc::c_int as OnigCodePoint,
                             index: 2472 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x10407 as libc::c_int as OnigCodePoint,
                             index: 3258 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x407 as libc::c_int as OnigCodePoint,
                             index: 1007 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x24c1 as libc::c_int as OnigCodePoint,
                             index: 2376 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x2c07 as libc::c_int as OnigCodePoint,
                             index: 2442 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x13f8 as libc::c_int as OnigCodePoint,
                             index: 1688 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff39 as libc::c_int as OnigCodePoint,
                             index: 3231 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x243 as libc::c_int as OnigCodePoint,
                             index: 354 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x241 as libc::c_int as OnigCodePoint,
                             index: 586 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff29 as libc::c_int as OnigCodePoint,
                             index: 3183 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0x23b as libc::c_int as OnigCodePoint,
                             index: 577 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff27 as libc::c_int as OnigCodePoint,
                             index: 3177 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff25 as libc::c_int as OnigCodePoint,
                             index: 3171 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff23 as libc::c_int as OnigCodePoint,
                             index: 3165 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff21 as libc::c_int as OnigCodePoint,
                             index: 3159 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb17 as libc::c_int as OnigCodePoint,
                             index: 117 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xff2f as libc::c_int as OnigCodePoint,
                             index: 3201 as libc::c_int,
                             fold_len: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb15 as libc::c_int as OnigCodePoint,
                             index: 113 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb13 as libc::c_int as OnigCodePoint,
                             index: 121 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb05 as libc::c_int as OnigCodePoint,
                             index: 29 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xffffffff as libc::c_uint,
                             index: -(1 as libc::c_int),
                             fold_len: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb03 as libc::c_int as OnigCodePoint,
                             index: 0 as libc::c_int,
                             fold_len: 3 as libc::c_int,};
             init
         },
         {
             let mut init =
                 ByUnfoldKey{code: 0xfb01 as libc::c_int as OnigCodePoint,
                             index: 8 as libc::c_int,
                             fold_len: 2 as libc::c_int,};
             init
         }];
    if 0 as libc::c_int == 0 as libc::c_int {
        let mut key: libc::c_int = hash(&mut code) as libc::c_int;
        if key <= 1544 as libc::c_int {
            let mut gcode: OnigCodePoint = wordlist[key as usize].code;
            if code == gcode {
                return &*wordlist.as_ptr().offset(key as isize) as
                           *const ByUnfoldKey
            }
        }
    }
    return 0 as *const ByUnfoldKey;
}
