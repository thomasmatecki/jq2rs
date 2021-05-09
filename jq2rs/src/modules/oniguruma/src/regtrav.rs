#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub allocated: libc::c_int,
    pub num_regs: libc::c_int,
    pub beg: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub history_root: *mut OnigCaptureTreeNode,
}
pub type OnigRegion = re_registers;
/* *********************************************************************
  regtrav.c -  Oniguruma (regular expression library)
**********************************************************************/
/*-
 * Copyright (c) 2002-2004  K.Kosako  <sndgk393 AT ybb DOT ne DOT jp>
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
unsafe extern "C" fn capture_tree_traverse(mut node: *mut OnigCaptureTreeNode,
                                           mut at: libc::c_int,
                                           mut callback_func:
                                               Option<unsafe extern "C" fn(_:
                                                                               libc::c_int,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> libc::c_int>,
                                           mut level: libc::c_int,
                                           mut arg: *mut libc::c_void)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if node.is_null() { return 0 as libc::c_int }
    if at & 1 as libc::c_int != 0 as libc::c_int {
        r =
            Some(callback_func.expect("non-null function pointer")).expect("non-null function pointer")((*node).group,
                                                                                                        (*node).beg,
                                                                                                        (*node).end,
                                                                                                        level,
                                                                                                        1
                                                                                                            as
                                                                                                            libc::c_int,
                                                                                                        arg);
        if r != 0 as libc::c_int { return r }
    }
    i = 0 as libc::c_int;
    while i < (*node).num_childs {
        r =
            capture_tree_traverse(*(*node).childs.offset(i as isize), at,
                                  callback_func, level + 1 as libc::c_int,
                                  arg);
        if r != 0 as libc::c_int { return r }
        i += 1
    }
    if at & 2 as libc::c_int != 0 as libc::c_int {
        r =
            Some(callback_func.expect("non-null function pointer")).expect("non-null function pointer")((*node).group,
                                                                                                        (*node).beg,
                                                                                                        (*node).end,
                                                                                                        level,
                                                                                                        2
                                                                                                            as
                                                                                                            libc::c_int,
                                                                                                        arg);
        if r != 0 as libc::c_int { return r }
    }
    return 0 as libc::c_int;
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
/* PART: regular expression */
/* config parameters */
/* constants */
/* options */
/* options (search time) */
/* limit */
/* syntax */
/* default option */
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
/* USE_CAPTURE_HISTORY */
#[no_mangle]
pub unsafe extern "C" fn onig_capture_tree_traverse(mut region:
                                                        *mut OnigRegion,
                                                    mut at: libc::c_int,
                                                    mut callback_func:
                                                        Option<unsafe extern "C" fn(_:
                                                                                        libc::c_int,
                                                                                    _:
                                                                                        libc::c_int,
                                                                                    _:
                                                                                        libc::c_int,
                                                                                    _:
                                                                                        libc::c_int,
                                                                                    _:
                                                                                        libc::c_int,
                                                                                    _:
                                                                                        *mut libc::c_void)
                                                                   ->
                                                                       libc::c_int>,
                                                    mut arg:
                                                        *mut libc::c_void)
 -> libc::c_int {
    return capture_tree_traverse((*region).history_root, at, callback_func,
                                 0 as libc::c_int, arg);
}
