#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    static mut OnigSyntaxRuby: OnigSyntaxType;
    #[no_mangle]
    static mut OnigDefaultSyntax: *mut OnigSyntaxType;
}
pub type OnigCodePoint = libc::c_uint;
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
/* *********************************************************************
  regsyntax.c -  Oniguruma (regular expression library)
**********************************************************************/
/*-
 * Copyright (c) 2002-2006  K.Kosako  <sndgk393 AT ybb DOT ne DOT jp>
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
pub static mut OnigSyntaxASIS: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op: 0 as libc::c_int as libc::c_uint,
                           op2: (1 as libc::c_uint) << 20 as libc::c_int,
                           behavior: 0 as libc::c_int as libc::c_uint,
                           options: 0 as libc::c_uint,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxPosixBasic: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 24 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 26 as libc::c_int |
                                   (1 as libc::c_uint) << 13 as libc::c_int |
                                   (1 as libc::c_uint) << 9 as libc::c_int,
                           op2: 0 as libc::c_int as libc::c_uint,
                           behavior: 0 as libc::c_int as libc::c_uint,
                           options:
                               (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                    1 as libc::c_int) << 1 as libc::c_int |
                                   ((1 as libc::c_uint) << 1 as libc::c_int)
                                       << 1 as libc::c_int,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxPosixExtended: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 24 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 26 as libc::c_int |
                                   (1 as libc::c_uint) << 12 as libc::c_int |
                                   (1 as libc::c_uint) << 8 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 10 as libc::c_int,
                           op2: 0 as libc::c_int as libc::c_uint,
                           behavior:
                               (1 as libc::c_uint) << 31 as libc::c_int |
                                   (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int,
                           options:
                               (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                    1 as libc::c_int) << 1 as libc::c_int |
                                   ((1 as libc::c_uint) << 1 as libc::c_int)
                                       << 1 as libc::c_int,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxEmacs: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 9 as libc::c_int |
                                   (1 as libc::c_uint) << 13 as libc::c_int |
                                   (1 as libc::c_uint) << 11 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 26 as libc::c_int,
                           op2: (1 as libc::c_uint) << 15 as libc::c_int,
                           behavior: (1 as libc::c_uint) << 22 as libc::c_int,
                           options: 0 as libc::c_uint,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxGrep: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 24 as libc::c_int |
                                   (1 as libc::c_uint) << 9 as libc::c_int |
                                   (1 as libc::c_uint) << 13 as libc::c_int |
                                   (1 as libc::c_uint) << 11 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 5 as libc::c_int |
                                   (1 as libc::c_uint) << 7 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 18 as libc::c_int |
                                   (1 as libc::c_uint) << 20 as libc::c_int |
                                   (1 as libc::c_uint) << 19 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int,
                           op2: 0 as libc::c_int as libc::c_uint,
                           behavior:
                               (1 as libc::c_uint) << 22 as libc::c_int |
                                   (1 as libc::c_uint) << 20 as libc::c_int,
                           options: 0 as libc::c_uint,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxGnuRegex: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 24 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 8 as libc::c_int |
                                   (1 as libc::c_uint) << 12 as libc::c_int |
                                   (1 as libc::c_uint) << 10 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 14 as libc::c_int |
                                   (1 as libc::c_uint) << 15 as libc::c_int |
                                   (1 as libc::c_uint) << 18 as libc::c_int |
                                   (1 as libc::c_uint) << 20 as libc::c_int |
                                   (1 as libc::c_uint) << 19 as libc::c_int |
                                   (1 as libc::c_uint) << 21 as libc::c_int |
                                   (1 as libc::c_uint) << 22 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int,
                           op2: 0 as libc::c_int as libc::c_uint,
                           behavior:
                               (1 as libc::c_uint) << 31 as libc::c_int |
                                   (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 3 as libc::c_int |
                                   (1 as libc::c_uint) << 21 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int,
                           options: 0 as libc::c_uint,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxJava: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               ((1 as libc::c_uint) << 1 as libc::c_int |
                                    (1 as libc::c_uint) << 17 as libc::c_int |
                                    (1 as libc::c_uint) << 24 as libc::c_int |
                                    (1 as libc::c_uint) << 16 as libc::c_int |
                                    (1 as libc::c_uint) << 8 as libc::c_int |
                                    (1 as libc::c_uint) << 12 as libc::c_int |
                                    (1 as libc::c_uint) << 10 as libc::c_int |
                                    (1 as libc::c_uint) << 2 as libc::c_int |
                                    (1 as libc::c_uint) << 4 as libc::c_int |
                                    (1 as libc::c_uint) << 6 as libc::c_int |
                                    (1 as libc::c_uint) << 14 as libc::c_int |
                                    (1 as libc::c_uint) << 15 as libc::c_int |
                                    (1 as libc::c_uint) << 18 as libc::c_int |
                                    (1 as libc::c_uint) << 20 as libc::c_int |
                                    (1 as libc::c_uint) << 19 as libc::c_int |
                                    (1 as libc::c_uint) << 21 as libc::c_int |
                                    (1 as libc::c_uint) << 22 as libc::c_int |
                                    (1 as libc::c_uint) << 23 as libc::c_int |
                                    (1 as libc::c_uint) << 25 as libc::c_int |
                                    (1 as libc::c_uint) << 26 as libc::c_int |
                                    (1 as libc::c_uint) << 27 as libc::c_int |
                                    (1 as libc::c_uint) << 28 as libc::c_int |
                                    (1 as libc::c_uint) << 29 as libc::c_int)
                                   &
                                   !((1 as libc::c_uint) <<
                                         19 as libc::c_int),
                           op2:
                               (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 5 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 13 as libc::c_int |
                                   (1 as libc::c_uint) << 14 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int,
                           behavior:
                               (1 as libc::c_uint) << 31 as libc::c_int |
                                   (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 3 as libc::c_int |
                                   (1 as libc::c_uint) << 21 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int,
                           options:
                               (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                    1 as libc::c_int) << 1 as libc::c_int,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub static mut OnigSyntaxPerl: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               ((1 as libc::c_uint) << 1 as libc::c_int |
                                    (1 as libc::c_uint) << 17 as libc::c_int |
                                    (1 as libc::c_uint) << 24 as libc::c_int |
                                    (1 as libc::c_uint) << 16 as libc::c_int |
                                    (1 as libc::c_uint) << 8 as libc::c_int |
                                    (1 as libc::c_uint) << 12 as libc::c_int |
                                    (1 as libc::c_uint) << 10 as libc::c_int |
                                    (1 as libc::c_uint) << 2 as libc::c_int |
                                    (1 as libc::c_uint) << 4 as libc::c_int |
                                    (1 as libc::c_uint) << 6 as libc::c_int |
                                    (1 as libc::c_uint) << 14 as libc::c_int |
                                    (1 as libc::c_uint) << 15 as libc::c_int |
                                    (1 as libc::c_uint) << 18 as libc::c_int |
                                    (1 as libc::c_uint) << 20 as libc::c_int |
                                    (1 as libc::c_uint) << 19 as libc::c_int |
                                    (1 as libc::c_uint) << 21 as libc::c_int |
                                    (1 as libc::c_uint) << 22 as libc::c_int |
                                    (1 as libc::c_uint) << 23 as libc::c_int |
                                    (1 as libc::c_uint) << 25 as libc::c_int |
                                    (1 as libc::c_uint) << 28 as libc::c_int |
                                    (1 as libc::c_uint) << 29 as libc::c_int |
                                    (1 as libc::c_uint) << 30 as libc::c_int |
                                    (1 as libc::c_uint) << 26 as libc::c_int |
                                    (1 as libc::c_uint) << 27 as libc::c_int)
                                   &
                                   !((1 as libc::c_uint) <<
                                         19 as libc::c_int),
                           op2:
                               (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int,
                           behavior:
                               (1 as libc::c_uint) << 31 as libc::c_int |
                                   (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 3 as libc::c_int |
                                   (1 as libc::c_uint) << 21 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int,
                           options:
                               (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                    1 as libc::c_int) << 1 as libc::c_int,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
/* Perl + named group */
#[no_mangle]
pub static mut OnigSyntaxPerl_NG: OnigSyntaxType =
    {
        let mut init =
            OnigSyntaxType{op:
                               ((1 as libc::c_uint) << 1 as libc::c_int |
                                    (1 as libc::c_uint) << 17 as libc::c_int |
                                    (1 as libc::c_uint) << 24 as libc::c_int |
                                    (1 as libc::c_uint) << 16 as libc::c_int |
                                    (1 as libc::c_uint) << 8 as libc::c_int |
                                    (1 as libc::c_uint) << 12 as libc::c_int |
                                    (1 as libc::c_uint) << 10 as libc::c_int |
                                    (1 as libc::c_uint) << 2 as libc::c_int |
                                    (1 as libc::c_uint) << 4 as libc::c_int |
                                    (1 as libc::c_uint) << 6 as libc::c_int |
                                    (1 as libc::c_uint) << 14 as libc::c_int |
                                    (1 as libc::c_uint) << 15 as libc::c_int |
                                    (1 as libc::c_uint) << 18 as libc::c_int |
                                    (1 as libc::c_uint) << 20 as libc::c_int |
                                    (1 as libc::c_uint) << 19 as libc::c_int |
                                    (1 as libc::c_uint) << 21 as libc::c_int |
                                    (1 as libc::c_uint) << 22 as libc::c_int |
                                    (1 as libc::c_uint) << 23 as libc::c_int |
                                    (1 as libc::c_uint) << 25 as libc::c_int |
                                    (1 as libc::c_uint) << 28 as libc::c_int |
                                    (1 as libc::c_uint) << 29 as libc::c_int |
                                    (1 as libc::c_uint) << 30 as libc::c_int |
                                    (1 as libc::c_uint) << 26 as libc::c_int |
                                    (1 as libc::c_uint) << 27 as libc::c_int)
                                   &
                                   !((1 as libc::c_uint) <<
                                         19 as libc::c_int),
                           op2:
                               (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 2 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 7 as libc::c_int |
                                   (1 as libc::c_uint) << 8 as libc::c_int |
                                   (1 as libc::c_uint) << 9 as libc::c_int,
                           behavior:
                               (1 as libc::c_uint) << 31 as libc::c_int |
                                   (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 3 as libc::c_int |
                                   (1 as libc::c_uint) << 21 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 7 as libc::c_int |
                                   (1 as libc::c_uint) << 8 as libc::c_int,
                           options:
                               (((1 as libc::c_uint) << 1 as libc::c_int) <<
                                    1 as libc::c_int) << 1 as libc::c_int,
                           meta_char_table:
                               {
                                   let mut init =
                                       OnigMetaCharTableType{esc:
                                                                 '\\' as i32
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             zero_or_one_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             one_or_more_time:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,
                                                             anychar_anytime:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigCodePoint,};
                                   init
                               },};
        init
    };
#[no_mangle]
pub unsafe extern "C" fn onig_set_default_syntax(mut syntax:
                                                     *mut OnigSyntaxType)
 -> libc::c_int {
    if (syntax as *mut libc::c_void).is_null() {
        syntax = &mut OnigSyntaxRuby
    }
    OnigDefaultSyntax = syntax;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_copy_syntax(mut to: *mut OnigSyntaxType,
                                          mut from: *mut OnigSyntaxType) {
    *to = *from;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_syntax_op(mut syntax: *mut OnigSyntaxType,
                                            mut op: libc::c_uint) {
    (*syntax).op = op;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_syntax_op2(mut syntax: *mut OnigSyntaxType,
                                             mut op2: libc::c_uint) {
    (*syntax).op2 = op2;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_syntax_behavior(mut syntax:
                                                      *mut OnigSyntaxType,
                                                  mut behavior:
                                                      libc::c_uint) {
    (*syntax).behavior = behavior;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_syntax_options(mut syntax:
                                                     *mut OnigSyntaxType,
                                                 mut options:
                                                     OnigOptionType) {
    (*syntax).options = options;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_syntax_op(mut syntax: *mut OnigSyntaxType)
 -> libc::c_uint {
    return (*syntax).op;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_syntax_op2(mut syntax: *mut OnigSyntaxType)
 -> libc::c_uint {
    return (*syntax).op2;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_syntax_behavior(mut syntax:
                                                      *mut OnigSyntaxType)
 -> libc::c_uint {
    return (*syntax).behavior;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_syntax_options(mut syntax:
                                                     *mut OnigSyntaxType)
 -> OnigOptionType {
    return (*syntax).options;
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
#[no_mangle]
pub unsafe extern "C" fn onig_set_meta_char(mut enc: *mut OnigSyntaxType,
                                            mut what: libc::c_uint,
                                            mut code: OnigCodePoint)
 -> libc::c_int {
    match what {
        0 => { (*enc).meta_char_table.esc = code }
        1 => { (*enc).meta_char_table.anychar = code }
        2 => { (*enc).meta_char_table.anytime = code }
        3 => { (*enc).meta_char_table.zero_or_one_time = code }
        4 => { (*enc).meta_char_table.one_or_more_time = code }
        5 => { (*enc).meta_char_table.anychar_anytime = code }
        _ => { return -(30 as libc::c_int) }
    }
    return 0 as libc::c_int;
}
/* USE_VARIABLE_META_CHARS */
