#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn onigenc_step_back(enc: OnigEncoding, start: *const OnigUChar,
                         s: *const OnigUChar, n: libc::c_int)
     -> *mut OnigUChar;
    #[no_mangle]
    fn onigenc_get_right_adjust_char_head_with_prev(enc: OnigEncoding,
                                                    start: *const OnigUChar,
                                                    s: *const OnigUChar,
                                                    prev:
                                                        *mut *const OnigUChar)
     -> *mut OnigUChar;
    #[no_mangle]
    fn onigenc_get_prev_char_head(enc: OnigEncoding, start: *const OnigUChar,
                                  s: *const OnigUChar) -> *mut OnigUChar;
    #[no_mangle]
    fn onigenc_get_right_adjust_char_head(enc: OnigEncoding,
                                          start: *const OnigUChar,
                                          s: *const OnigUChar)
     -> *mut OnigUChar;
    #[no_mangle]
    fn onig_is_in_code_range(p: *const OnigUChar, code: OnigCodePoint)
     -> libc::c_int;
    #[no_mangle]
    fn onig_is_code_in_cc_len(enclen: libc::c_int, code: OnigCodePoint,
                              cc: *mut CClassNode) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub struct OnigMatchArg {
    pub stack_p: *mut libc::c_void,
    pub stack_n: libc::c_int,
    pub options: OnigOptionType,
    pub region: *mut OnigRegion,
    pub ptr_num: libc::c_int,
    pub start: *const OnigUChar,
    pub best_len: libc::c_int,
    pub best_s: *mut OnigUChar,
}
pub type OnigStackType = _OnigStackType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _OnigStackType {
    pub type_0: libc::c_uint,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub state: C2RustUnnamed_5,
    pub repeat: C2RustUnnamed_4,
    pub repeat_inc: C2RustUnnamed_3,
    pub mem: C2RustUnnamed_2,
    pub null_check: C2RustUnnamed_1,
    pub call_frame: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ret_addr: *mut OnigUChar,
    pub num: libc::c_int,
    pub pstr: *mut OnigUChar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub num: libc::c_int,
    pub pstr: *mut OnigUChar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub num: libc::c_int,
    pub pstr: *mut OnigUChar,
    pub start: OnigStackIndex,
    pub end: OnigStackIndex,
}
pub type OnigStackIndex = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si: OnigStackIndex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub count: libc::c_int,
    pub pcode: *mut OnigUChar,
    pub num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub pcode: *mut OnigUChar,
    pub pstr: *mut OnigUChar,
    pub pstr_prev: *mut OnigUChar,
}
pub const OP_FAIL: OpCode = 54;
pub const OP_FINISH: OpCode = 0;
pub const OP_RETURN: OpCode = 80;
pub type RelAddrType = libc::c_int;
pub type AbsAddrType = libc::c_int;
pub const OP_CALL: OpCode = 79;
pub const OP_FAIL_LOOK_BEHIND_NOT: OpCode = 78;
pub type LengthType = libc::c_int;
pub const OP_PUSH_LOOK_BEHIND_NOT: OpCode = 77;
pub const OP_LOOK_BEHIND: OpCode = 76;
pub const OP_POP_STOP_BT: OpCode = 75;
pub const OP_PUSH_STOP_BT: OpCode = 74;
pub const OP_FAIL_POS: OpCode = 73;
pub const OP_PUSH_POS_NOT: OpCode = 72;
pub const OP_POP_POS: OpCode = 71;
pub const OP_PUSH_POS: OpCode = 70;
pub type MemNumType = libc::c_short;
pub const OP_REPEAT_INC_NG_SG: OpCode = 65;
pub const OP_REPEAT_INC_NG: OpCode = 63;
pub const OP_REPEAT_INC_SG: OpCode = 64;
pub const OP_REPEAT_INC: OpCode = 62;
pub const OP_REPEAT_NG: OpCode = 61;
pub const OP_REPEAT: OpCode = 60;
pub const OP_PUSH_IF_PEEK_NEXT: OpCode = 59;
pub const OP_PUSH_OR_JUMP_EXACT1: OpCode = 58;
pub const OP_POP: OpCode = 57;
pub const OP_PUSH: OpCode = 56;
pub const OP_JUMP: OpCode = 55;
pub type BitStatusType = libc::c_uint;
pub const OP_NULL_CHECK_END_MEMST_PUSH: OpCode = 69;
pub const OP_NULL_CHECK_END_MEMST: OpCode = 68;
pub const OP_NULL_CHECK_END: OpCode = 67;
pub const OP_NULL_CHECK_START: OpCode = 66;
pub const OP_BACKREF_WITH_LEVEL: OpCode = 47;
pub const OP_BACKREF_MULTI_IC: OpCode = 46;
pub const OP_BACKREF_MULTI: OpCode = 45;
pub const OP_BACKREFN_IC: OpCode = 44;
pub const OP_BACKREFN: OpCode = 43;
pub const OP_BACKREF2: OpCode = 42;
pub const OP_BACKREF1: OpCode = 41;
pub const OP_MEMORY_END_REC: OpCode = 53;
pub const OP_MEMORY_END_PUSH_REC: OpCode = 51;
pub const OP_MEMORY_END: OpCode = 52;
pub const OP_MEMORY_END_PUSH: OpCode = 50;
pub const OP_MEMORY_START: OpCode = 48;
pub const OP_MEMORY_START_PUSH: OpCode = 49;
pub const OP_BEGIN_POSITION: OpCode = 40;
pub const OP_SEMI_END_BUF: OpCode = 39;
pub const OP_END_LINE: OpCode = 38;
pub const OP_BEGIN_LINE: OpCode = 37;
pub const OP_END_BUF: OpCode = 36;
pub const OP_BEGIN_BUF: OpCode = 35;
pub const OP_WORD_END: OpCode = 34;
pub const OP_WORD_BEGIN: OpCode = 33;
pub const OP_NOT_WORD_BOUND: OpCode = 32;
pub const OP_WORD_BOUND: OpCode = 31;
pub const OP_NOT_WORD: OpCode = 30;
pub const OP_WORD: OpCode = 29;
pub const OP_ANYCHAR_ML_STAR_PEEK_NEXT: OpCode = 28;
pub const OP_ANYCHAR_STAR_PEEK_NEXT: OpCode = 27;
pub const OP_ANYCHAR_ML_STAR: OpCode = 26;
pub const OP_ANYCHAR_STAR: OpCode = 25;
pub const OP_ANYCHAR_ML: OpCode = 24;
pub const OP_ANYCHAR: OpCode = 23;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClassNode {
    pub base: NodeBase,
    pub flags: libc::c_uint,
    pub bs: BitSet,
    pub mbuf: *mut BBuf,
}
pub type BBuf = _BBuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BBuf {
    pub p: *mut OnigUChar,
    pub used: libc::c_uint,
    pub alloc: libc::c_uint,
}
pub type BitSet = [Bits; 8];
pub type Bits = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeBase {
    pub type_0: libc::c_int,
}
pub type PointerType = *mut libc::c_void;
pub const OP_CCLASS_NODE: OpCode = 22;
pub type BitSetRef = *mut Bits;
pub const OP_CCLASS_MIX_NOT: OpCode = 21;
pub const OP_CCLASS_MB_NOT: OpCode = 20;
pub const OP_CCLASS_NOT: OpCode = 19;
pub const OP_CCLASS_MIX: OpCode = 18;
pub const OP_CCLASS_MB: OpCode = 17;
pub const OP_CCLASS: OpCode = 16;
pub const OP_EXACTMBN: OpCode = 13;
pub const OP_EXACTMB3N: OpCode = 12;
pub const OP_EXACTMB2N: OpCode = 11;
pub const OP_EXACTMB2N3: OpCode = 10;
pub const OP_EXACTMB2N2: OpCode = 9;
pub const OP_EXACTMB2N1: OpCode = 8;
pub const OP_EXACTN_IC: OpCode = 15;
pub const OP_EXACTN: OpCode = 7;
pub const OP_EXACT5: OpCode = 6;
pub const OP_EXACT4: OpCode = 5;
pub const OP_EXACT3: OpCode = 4;
pub const OP_EXACT2: OpCode = 3;
pub const OP_EXACT1_IC: OpCode = 14;
pub const OP_EXACT1: OpCode = 2;
/* USE_BACKREF_WITH_LEVEL */
/* matching region of POSIX API */
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub const OP_END: OpCode = 1;
pub type OpCode = libc::c_uint;
pub const OP_SET_OPTION: OpCode = 87;
pub const OP_SET_OPTION_PUSH: OpCode = 86;
pub const OP_STATE_CHECK_ANYCHAR_ML_STAR: OpCode = 85;
pub const OP_STATE_CHECK_ANYCHAR_STAR: OpCode = 84;
pub const OP_STATE_CHECK: OpCode = 83;
pub const OP_STATE_CHECK_PUSH_OR_JUMP: OpCode = 82;
pub const OP_STATE_CHECK_PUSH: OpCode = 81;
unsafe extern "C" fn history_tree_clear(mut node: *mut OnigCaptureTreeNode) {
    let mut i: libc::c_int = 0;
    if !(node as *mut libc::c_void).is_null() {
        i = 0 as libc::c_int;
        while i < (*node).num_childs {
            if !(*(*node).childs.offset(i as isize) as
                     *mut libc::c_void).is_null() {
                history_tree_free(*(*node).childs.offset(i as isize));
            }
            i += 1
        }
        i = 0 as libc::c_int;
        while i < (*node).allocated {
            let ref mut fresh0 = *(*node).childs.offset(i as isize);
            *fresh0 = 0 as *mut OnigCaptureTreeNode;
            i += 1
        }
        (*node).num_childs = 0 as libc::c_int;
        (*node).beg = -(1 as libc::c_int);
        (*node).end = -(1 as libc::c_int);
        (*node).group = -(1 as libc::c_int)
    };
}
/* *********************************************************************
  regexec.c -  Oniguruma (regular expression library)
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
unsafe extern "C" fn history_tree_free(mut node: *mut OnigCaptureTreeNode) {
    history_tree_clear(node);
    free(node as *mut libc::c_void);
}
unsafe extern "C" fn history_root_free(mut r: *mut OnigRegion) {
    if !((*r).history_root as *mut libc::c_void).is_null() {
        history_tree_free((*r).history_root);
        (*r).history_root = 0 as *mut OnigCaptureTreeNode
    };
}
unsafe extern "C" fn history_node_new() -> *mut OnigCaptureTreeNode {
    let mut node: *mut OnigCaptureTreeNode = 0 as *mut OnigCaptureTreeNode;
    node =
        malloc(::std::mem::size_of::<OnigCaptureTreeNode>() as libc::c_ulong)
            as *mut OnigCaptureTreeNode;
    if (node as *mut libc::c_void).is_null() {
        return 0 as *mut OnigCaptureTreeNode
    }
    (*node).childs = 0 as *mut *mut OnigCaptureTreeNode;
    (*node).allocated = 0 as libc::c_int;
    (*node).num_childs = 0 as libc::c_int;
    (*node).group = -(1 as libc::c_int);
    (*node).beg = -(1 as libc::c_int);
    (*node).end = -(1 as libc::c_int);
    return node;
}
unsafe extern "C" fn history_tree_add_child(mut parent:
                                                *mut OnigCaptureTreeNode,
                                            mut child:
                                                *mut OnigCaptureTreeNode)
 -> libc::c_int {
    if (*parent).num_childs >= (*parent).allocated {
        let mut n: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        if ((*parent).childs as *mut libc::c_void).is_null() {
            n = 8 as libc::c_int;
            (*parent).childs =
                malloc((::std::mem::size_of::<*mut OnigCaptureTreeNode>() as
                            libc::c_ulong).wrapping_mul(n as libc::c_ulong))
                    as *mut *mut OnigCaptureTreeNode
        } else {
            n = (*parent).allocated * 2 as libc::c_int;
            (*parent).childs =
                realloc((*parent).childs as *mut libc::c_void,
                        (::std::mem::size_of::<*mut OnigCaptureTreeNode>() as
                             libc::c_ulong).wrapping_mul(n as libc::c_ulong))
                    as *mut *mut OnigCaptureTreeNode
        }
        if ((*parent).childs as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        i = (*parent).allocated;
        while i < n {
            let ref mut fresh1 = *(*parent).childs.offset(i as isize);
            *fresh1 = 0 as *mut OnigCaptureTreeNode;
            i += 1
        }
        (*parent).allocated = n
    }
    let ref mut fresh2 =
        *(*parent).childs.offset((*parent).num_childs as isize);
    *fresh2 = child;
    (*parent).num_childs += 1;
    return 0 as libc::c_int;
}
unsafe extern "C" fn history_tree_clone(mut node: *mut OnigCaptureTreeNode)
 -> *mut OnigCaptureTreeNode {
    let mut i: libc::c_int = 0;
    let mut clone: *mut OnigCaptureTreeNode = 0 as *mut OnigCaptureTreeNode;
    let mut child: *mut OnigCaptureTreeNode = 0 as *mut OnigCaptureTreeNode;
    clone = history_node_new();
    if (clone as *mut libc::c_void).is_null() {
        return 0 as *mut OnigCaptureTreeNode
    }
    (*clone).beg = (*node).beg;
    (*clone).end = (*node).end;
    i = 0 as libc::c_int;
    while i < (*node).num_childs {
        child = history_tree_clone(*(*node).childs.offset(i as isize));
        if (child as *mut libc::c_void).is_null() {
            history_tree_free(clone);
            return 0 as *mut OnigCaptureTreeNode
        }
        history_tree_add_child(clone, child);
        i += 1
    }
    return clone;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_capture_tree(mut region: *mut OnigRegion)
 -> *mut OnigCaptureTreeNode {
    return (*region).history_root;
}
/* USE_CAPTURE_HISTORY */
#[no_mangle]
pub unsafe extern "C" fn onig_region_clear(mut region: *mut OnigRegion) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*region).num_regs {
        let ref mut fresh3 = *(*region).end.offset(i as isize);
        *fresh3 = -(1 as libc::c_int);
        *(*region).beg.offset(i as isize) = *fresh3;
        i += 1
    }
    history_root_free(region);
}
#[no_mangle]
pub unsafe extern "C" fn onig_region_resize(mut region: *mut OnigRegion,
                                            mut n: libc::c_int)
 -> libc::c_int {
    (*region).num_regs = n;
    if n < 10 as libc::c_int { n = 10 as libc::c_int }
    if (*region).allocated == 0 as libc::c_int {
        (*region).beg =
            malloc((n as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        (*region).end =
            malloc((n as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        if (*region).beg.is_null() || (*region).end.is_null() {
            return -(5 as libc::c_int)
        }
        (*region).allocated = n
    } else if (*region).allocated < n {
        (*region).beg =
            realloc((*region).beg as *mut libc::c_void,
                    (n as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
                *mut libc::c_int;
        (*region).end =
            realloc((*region).end as *mut libc::c_void,
                    (n as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
                *mut libc::c_int;
        if (*region).beg.is_null() || (*region).end.is_null() {
            return -(5 as libc::c_int)
        }
        (*region).allocated = n
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn onig_region_resize_clear(mut region: *mut OnigRegion,
                                              mut n: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = onig_region_resize(region, n);
    if r != 0 as libc::c_int { return r }
    onig_region_clear(region);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_region_set(mut region: *mut OnigRegion,
                                         mut at: libc::c_int,
                                         mut beg: libc::c_int,
                                         mut end: libc::c_int)
 -> libc::c_int {
    if at < 0 as libc::c_int { return -(30 as libc::c_int) }
    if at >= (*region).allocated {
        let mut r: libc::c_int =
            onig_region_resize(region, at + 1 as libc::c_int);
        if r < 0 as libc::c_int { return r }
    }
    *(*region).beg.offset(at as isize) = beg;
    *(*region).end.offset(at as isize) = end;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_region_init(mut region: *mut OnigRegion) {
    (*region).num_regs = 0 as libc::c_int;
    (*region).allocated = 0 as libc::c_int;
    (*region).beg = 0 as *mut libc::c_int;
    (*region).end = 0 as *mut libc::c_int;
    (*region).history_root = 0 as *mut OnigCaptureTreeNode;
}
#[no_mangle]
pub unsafe extern "C" fn onig_region_new() -> *mut OnigRegion {
    let mut r: *mut OnigRegion = 0 as *mut OnigRegion;
    r =
        malloc(::std::mem::size_of::<OnigRegion>() as libc::c_ulong) as
            *mut OnigRegion;
    onig_region_init(r);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn onig_region_free(mut r: *mut OnigRegion,
                                          mut free_self: libc::c_int) {
    if !r.is_null() {
        if (*r).allocated > 0 as libc::c_int {
            if !(*r).beg.is_null() { free((*r).beg as *mut libc::c_void); }
            if !(*r).end.is_null() { free((*r).end as *mut libc::c_void); }
            (*r).allocated = 0 as libc::c_int
        }
        history_root_free(r);
        if free_self != 0 { free(r as *mut libc::c_void); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn onig_region_copy(mut to: *mut OnigRegion,
                                          mut from: *mut OnigRegion) {
    let mut i: libc::c_int = 0;
    if to == from { return }
    if (*to).allocated == 0 as libc::c_int {
        if (*from).num_regs > 0 as libc::c_int {
            (*to).beg =
                malloc((::std::mem::size_of::<libc::c_int>() as
                            libc::c_ulong).wrapping_mul((*from).num_regs as
                                                            libc::c_ulong)) as
                    *mut libc::c_int;
            (*to).end =
                malloc((::std::mem::size_of::<libc::c_int>() as
                            libc::c_ulong).wrapping_mul((*from).num_regs as
                                                            libc::c_ulong)) as
                    *mut libc::c_int;
            (*to).allocated = (*from).num_regs
        }
    } else if (*to).allocated < (*from).num_regs {
        (*to).beg =
            realloc((*to).beg as *mut libc::c_void,
                    (::std::mem::size_of::<libc::c_int>() as
                         libc::c_ulong).wrapping_mul((*from).num_regs as
                                                         libc::c_ulong)) as
                *mut libc::c_int;
        (*to).end =
            realloc((*to).end as *mut libc::c_void,
                    (::std::mem::size_of::<libc::c_int>() as
                         libc::c_ulong).wrapping_mul((*from).num_regs as
                                                         libc::c_ulong)) as
                *mut libc::c_int;
        (*to).allocated = (*from).num_regs
    }
    i = 0 as libc::c_int;
    while i < (*from).num_regs {
        *(*to).beg.offset(i as isize) = *(*from).beg.offset(i as isize);
        *(*to).end.offset(i as isize) = *(*from).end.offset(i as isize);
        i += 1
    }
    (*to).num_regs = (*from).num_regs;
    history_root_free(to);
    if !((*from).history_root as *mut libc::c_void).is_null() {
        (*to).history_root = history_tree_clone((*from).history_root)
    };
}
/* for index start from 1 */
/* for index start from 1 */
static mut MatchStackLimitSize: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn onig_get_match_stack_limit_size() -> libc::c_uint {
    return MatchStackLimitSize;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_match_stack_limit_size(mut size:
                                                             libc::c_uint)
 -> libc::c_int {
    MatchStackLimitSize = size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stack_double(mut is_alloca: libc::c_int,
                                  mut arg_alloc_base: *mut *mut libc::c_char,
                                  mut arg_stk_base: *mut *mut OnigStackType,
                                  mut arg_stk_end: *mut *mut OnigStackType,
                                  mut arg_stk: *mut *mut OnigStackType,
                                  mut msa: *mut OnigMatchArg) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    let mut used: libc::c_int = 0;
    let mut size: size_t = 0;
    let mut new_size: size_t = 0;
    let mut alloc_base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_alloc_base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stk_base: *mut OnigStackType = 0 as *mut OnigStackType;
    let mut stk_end: *mut OnigStackType = 0 as *mut OnigStackType;
    let mut stk: *mut OnigStackType = 0 as *mut OnigStackType;
    alloc_base = *arg_alloc_base;
    stk_base = *arg_stk_base;
    stk_end = *arg_stk_end;
    stk = *arg_stk;
    n =
        stk_end.wrapping_offset_from(stk_base) as libc::c_long as
            libc::c_uint;
    size =
        (::std::mem::size_of::<OnigStackIndex>() as
             libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(n
                                                                                                              as
                                                                                                              libc::c_ulong));
    n = n.wrapping_mul(2 as libc::c_int as libc::c_uint);
    new_size =
        (::std::mem::size_of::<OnigStackIndex>() as
             libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(n
                                                                                                              as
                                                                                                              libc::c_ulong));
    if is_alloca != 0 as libc::c_int {
        new_alloc_base = malloc(new_size) as *mut libc::c_char;
        if (new_alloc_base as *mut libc::c_void).is_null() {
            (*msa).stack_n =
                stk_end.wrapping_offset_from(stk_base) as libc::c_long as
                    libc::c_int;
            if is_alloca != 0 as libc::c_int {
                let mut size_0: size_t =
                    (::std::mem::size_of::<OnigStackIndex>() as
                         libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                          as
                                                                                                                          libc::c_ulong));
                (*msa).stack_p = malloc(size_0);
                memcpy((*msa).stack_p, alloc_base as *const libc::c_void,
                       size_0);
            } else { (*msa).stack_p = alloc_base as *mut libc::c_void }
            return -(5 as libc::c_int)
        }
        memcpy(new_alloc_base as *mut libc::c_void,
               alloc_base as *const libc::c_void, size);
    } else {
        if MatchStackLimitSize != 0 as libc::c_int as libc::c_uint &&
               n > MatchStackLimitSize {
            if stk_end.wrapping_offset_from(stk_base) as libc::c_long as
                   libc::c_uint == MatchStackLimitSize {
                return -(15 as libc::c_int)
            } else { n = MatchStackLimitSize }
        }
        new_alloc_base =
            realloc(alloc_base as *mut libc::c_void, new_size) as
                *mut libc::c_char;
        if (new_alloc_base as *mut libc::c_void).is_null() {
            (*msa).stack_n =
                stk_end.wrapping_offset_from(stk_base) as libc::c_long as
                    libc::c_int;
            if is_alloca != 0 as libc::c_int {
                let mut size_1: size_t =
                    (::std::mem::size_of::<OnigStackIndex>() as
                         libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                          as
                                                                                                                          libc::c_ulong));
                (*msa).stack_p = malloc(size_1);
                memcpy((*msa).stack_p, alloc_base as *const libc::c_void,
                       size_1);
            } else { (*msa).stack_p = alloc_base as *mut libc::c_void }
            return -(5 as libc::c_int)
        }
    }
    alloc_base = new_alloc_base;
    used = stk.wrapping_offset_from(stk_base) as libc::c_long as libc::c_int;
    *arg_alloc_base = alloc_base;
    *arg_stk_base =
        alloc_base.offset((::std::mem::size_of::<OnigStackIndex>() as
                               libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                               libc::c_ulong)
                              as isize) as *mut OnigStackType;
    *arg_stk = (*arg_stk_base).offset(used as isize);
    *arg_stk_end = (*arg_stk_base).offset(n as isize);
    return 0 as libc::c_int;
}
/* USE_COMBINATION_EXPLOSION_CHECK */
/* USE_COMBINATION_EXPLOSION_CHECK */
/* empty, but position changed */
/* empty, but position changed */
unsafe extern "C" fn string_cmp_ic(mut enc: OnigEncoding,
                                   mut case_fold_flag: libc::c_int,
                                   mut s1: *mut OnigUChar,
                                   mut ps2: *mut *mut OnigUChar,
                                   mut mblen: libc::c_int) -> libc::c_int {
    let mut buf1: [OnigUChar; 18] = [0; 18];
    let mut buf2: [OnigUChar; 18] = [0; 18];
    let mut p1: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p2: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end1: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut s2: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end2: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    s2 = *ps2;
    end1 = s1.offset(mblen as isize);
    end2 = s2.offset(mblen as isize);
    while s1 < end1 {
        len1 =
            (*enc).mbc_case_fold.expect("non-null function pointer")(case_fold_flag
                                                                         as
                                                                         OnigCaseFoldType,
                                                                     &mut s1
                                                                         as
                                                                         *mut *mut OnigUChar
                                                                         as
                                                                         *mut *const OnigUChar,
                                                                     end1,
                                                                     buf1.as_mut_ptr());
        len2 =
            (*enc).mbc_case_fold.expect("non-null function pointer")(case_fold_flag
                                                                         as
                                                                         OnigCaseFoldType,
                                                                     &mut s2
                                                                         as
                                                                         *mut *mut OnigUChar
                                                                         as
                                                                         *mut *const OnigUChar,
                                                                     end2,
                                                                     buf2.as_mut_ptr());
        if len1 != len2 { return 0 as libc::c_int }
        p1 = buf1.as_mut_ptr();
        p2 = buf2.as_mut_ptr();
        loop  {
            let fresh4 = len1;
            len1 = len1 - 1;
            if !(fresh4 > 0 as libc::c_int) { break ; }
            if *p1 as libc::c_int != *p2 as libc::c_int {
                return 0 as libc::c_int
            }
            p1 = p1.offset(1);
            p2 = p2.offset(1)
        }
    }
    *ps2 = s2;
    return 1 as libc::c_int;
}
/* USE_MATCH_RANGE_MUST_BE_INSIDE_OF_SPECIFIED_RANGE */
unsafe extern "C" fn make_capture_history_tree(mut node:
                                                   *mut OnigCaptureTreeNode,
                                               mut kp:
                                                   *mut *mut OnigStackType,
                                               mut stk_top:
                                                   *mut OnigStackType,
                                               mut str: *mut OnigUChar,
                                               mut reg: *mut regex_t)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut child: *mut OnigCaptureTreeNode = 0 as *mut OnigCaptureTreeNode;
    let mut k: *mut OnigStackType = *kp;
    while k < stk_top {
        if (*k).type_0 == 0x100 as libc::c_int as libc::c_uint {
            n = (*k).u.mem.num;
            if n <= 31 as libc::c_int &&
                   (if n <
                           (::std::mem::size_of::<BitStatusType>() as
                                libc::c_ulong).wrapping_mul(8 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                               as libc::c_int {
                        ((*reg).capture_history) &
                            ((1 as libc::c_int) << n) as libc::c_uint
                    } else {
                        ((*reg).capture_history) &
                            1 as libc::c_int as libc::c_uint
                    }) != 0 as libc::c_int as libc::c_uint {
                child = history_node_new();
                if (child as *mut libc::c_void).is_null() {
                    return -(5 as libc::c_int)
                }
                (*child).group = n;
                (*child).beg =
                    (*k).u.mem.pstr.wrapping_offset_from(str) as libc::c_long
                        as libc::c_int;
                r = history_tree_add_child(node, child);
                if r != 0 as libc::c_int { return r }
                *kp = k.offset(1 as libc::c_int as isize);
                r = make_capture_history_tree(child, kp, stk_top, str, reg);
                if r != 0 as libc::c_int { return r }
                k = *kp;
                (*child).end =
                    (*k).u.mem.pstr.wrapping_offset_from(str) as libc::c_long
                        as libc::c_int
            }
        } else if (*k).type_0 == 0x8200 as libc::c_int as libc::c_uint {
            if (*k).u.mem.num == (*node).group {
                (*node).end =
                    (*k).u.mem.pstr.wrapping_offset_from(str) as libc::c_long
                        as libc::c_int;
                *kp = k;
                return 0 as libc::c_int
            }
        }
        k = k.offset(1)
    }
    return 1 as libc::c_int;
    /* 1: root node ending. */
}
unsafe extern "C" fn mem_is_in_memp(mut mem: libc::c_int,
                                    mut num: libc::c_int,
                                    mut memp: *mut OnigUChar) -> libc::c_int {
    let mut i: libc::c_int = 0; /* or goto next_mem; */
    let mut m: MemNumType = 0;
    i = 0 as libc::c_int;
    while i < num {
        m = *(memp as *mut MemNumType);
        memp =
            memp.offset(::std::mem::size_of::<MemNumType>() as libc::c_ulong
                            as isize);
        if mem == m as libc::c_int { return 1 as libc::c_int }
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn backref_match_at_nested_level(mut reg: *mut regex_t,
                                                   mut top:
                                                       *mut OnigStackType,
                                                   mut stk_base:
                                                       *mut OnigStackType,
                                                   mut ignore_case:
                                                       libc::c_int,
                                                   mut case_fold_flag:
                                                       libc::c_int,
                                                   mut nest: libc::c_int,
                                                   mut mem_num: libc::c_int,
                                                   mut memp: *mut OnigUChar,
                                                   mut s: *mut *mut OnigUChar,
                                                   mut send: *const OnigUChar)
 -> libc::c_int {
    let mut ss: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut pstart: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut pend: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut level: libc::c_int = 0;
    let mut k: *mut OnigStackType = 0 as *mut OnigStackType;
    level = 0 as libc::c_int;
    k = top;
    k = k.offset(-1);
    while k >= stk_base {
        if (*k).type_0 == 0x800 as libc::c_int as libc::c_uint {
            level -= 1
        } else if (*k).type_0 == 0x900 as libc::c_int as libc::c_uint {
            level += 1
        } else if level == nest {
            if (*k).type_0 == 0x100 as libc::c_int as libc::c_uint {
                if mem_is_in_memp((*k).u.mem.num, mem_num, memp) != 0 {
                    pstart = (*k).u.mem.pstr;
                    if !pend.is_null() {
                        if pend.wrapping_offset_from(pstart) as libc::c_long >
                               send.wrapping_offset_from(*s) as libc::c_long {
                            return 0 as libc::c_int
                        }
                        p = pstart;
                        ss = *s;
                        if ignore_case != 0 as libc::c_int {
                            if string_cmp_ic((*reg).enc, case_fold_flag,
                                             pstart, &mut ss,
                                             pend.wrapping_offset_from(pstart)
                                                 as libc::c_long as
                                                 libc::c_int) ==
                                   0 as libc::c_int {
                                return 0 as libc::c_int
                            }
                            /* or goto next_mem; */
                        } else {
                            while p < pend {
                                let fresh5 = p;
                                p = p.offset(1);
                                let fresh6 = ss;
                                ss = ss.offset(1);
                                if *fresh5 as libc::c_int !=
                                       *fresh6 as libc::c_int {
                                    return 0 as libc::c_int
                                }
                                /* or goto next_mem; */
                            }
                        }
                        *s = ss;
                        return 1 as libc::c_int
                    }
                }
            } else if (*k).type_0 == 0x8200 as libc::c_int as libc::c_uint {
                if mem_is_in_memp((*k).u.mem.num, mem_num, memp) != 0 {
                    pend = (*k).u.mem.pstr
                }
            }
        }
        k = k.offset(-1)
    }
    return 0 as libc::c_int;
}
/* match data(str - end) from position (sstart). */
/* if sstart == str then set sprev to NULL. */
unsafe extern "C" fn match_at(mut reg: *mut regex_t,
                              mut str: *const OnigUChar,
                              mut end: *const OnigUChar,
                              mut right_range: *const OnigUChar,
                              mut sstart: *const OnigUChar,
                              mut sprev: *mut OnigUChar,
                              mut msa: *mut OnigMatchArg) -> libc::c_int {
    let mut isnull: libc::c_int = 0; /* used as any purpose. */
    let mut current_block: u64;
    static mut FinishCode: [OnigUChar; 1] =
        [OP_FINISH as libc::c_int as OnigUChar];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut num_mem: libc::c_int = 0;
    let mut best_len: libc::c_int = 0;
    let mut pop_level: libc::c_int = 0;
    let mut tlen: LengthType = 0;
    let mut tlen2: LengthType = 0;
    let mut mem: MemNumType = 0;
    let mut addr: RelAddrType = 0;
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut sbegin: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut is_alloca: libc::c_int = 0;
    let mut alloc_base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stk_base: *mut OnigStackType = 0 as *mut OnigStackType;
    let mut stk: *mut OnigStackType = 0 as *mut OnigStackType;
    let mut stk_end: *mut OnigStackType = 0 as *mut OnigStackType;
    let mut stkp: *mut OnigStackType = 0 as *mut OnigStackType;
    let mut si: OnigStackIndex = 0;
    let mut repeat_stk: *mut OnigStackIndex = 0 as *mut OnigStackIndex;
    let mut mem_start_stk: *mut OnigStackIndex = 0 as *mut OnigStackIndex;
    let mut mem_end_stk: *mut OnigStackIndex = 0 as *mut OnigStackIndex;
    let mut p: *mut OnigUChar = (*reg).p;
    let mut option: OnigOptionType = (*reg).options;
    let mut encode: OnigEncoding = (*reg).enc;
    let mut case_fold_flag: OnigCaseFoldType = (*reg).case_fold_flag;
    //n = reg->num_repeat + reg->num_mem * 2;
    pop_level = (*reg).stack_pop_level; /* bottom stack */
    num_mem = (*reg).num_mem; /* end of while(1) */
    if !(*msa).stack_p.is_null() {
        is_alloca = 0 as libc::c_int; /* end of switch */
        alloc_base = (*msa).stack_p as *mut libc::c_char; /* n > best_len */
        stk_base =
            alloc_base.offset((::std::mem::size_of::<OnigStackIndex>() as
                                   libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                   as
                                                                   libc::c_ulong)
                                  as isize) as *mut OnigStackType;
        stk = stk_base;
        stk_end = stk_base.offset((*msa).stack_n as isize)
    } else if (*msa).ptr_num > 50 as libc::c_int {
        is_alloca = 0 as libc::c_int;
        alloc_base =
            malloc((::std::mem::size_of::<OnigStackIndex>() as
                        libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                        libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_mul(160
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_ulong)))
                as *mut libc::c_char;
        stk_base =
            alloc_base.offset((::std::mem::size_of::<OnigStackIndex>() as
                                   libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                   as
                                                                   libc::c_ulong)
                                  as isize) as *mut OnigStackType;
        stk = stk_base;
        stk_end = stk_base.offset(160 as libc::c_int as isize)
    } else {
        is_alloca = 1 as libc::c_int;
        let mut fresh7 =
            ::std::vec::from_elem(0,
                                  (::std::mem::size_of::<OnigStackIndex>() as
                                       libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                       as
                                                                       libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_mul(160
                                                                                                                                        as
                                                                                                                                        libc::c_int
                                                                                                                                        as
                                                                                                                                        libc::c_ulong))
                                      as usize);
        alloc_base = fresh7.as_mut_ptr() as *mut libc::c_char;
        stk_base =
            alloc_base.offset((::std::mem::size_of::<OnigStackIndex>() as
                                   libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                   as
                                                                   libc::c_ulong)
                                  as isize) as *mut OnigStackType;
        stk = stk_base;
        stk_end = stk_base.offset(160 as libc::c_int as isize)
    }
    repeat_stk = alloc_base as *mut OnigStackIndex;
    mem_start_stk = repeat_stk.offset((*reg).num_repeat as isize);
    mem_end_stk = mem_start_stk.offset(num_mem as isize);
    mem_start_stk = mem_start_stk.offset(-1);
    mem_end_stk = mem_end_stk.offset(-1);
    i = 1 as libc::c_int;
    while i <= num_mem {
        let ref mut fresh8 = *mem_end_stk.offset(i as isize);
        *fresh8 = -(1 as libc::c_int) as OnigStackIndex;
        *mem_start_stk.offset(i as isize) = *fresh8;
        i += 1
    }
    (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
    (*stk).u.state.pcode = FinishCode.as_mut_ptr();
    stk = stk.offset(1);
    best_len = -(1 as libc::c_int);
    s = sstart as *mut OnigUChar;
    's_246:
        loop  {
            sbegin = s;
            let fresh9 = p;
            p = p.offset(1);
            match *fresh9 as libc::c_int {
                1 => {
                    n =
                        s.wrapping_offset_from(sstart) as libc::c_long as
                            libc::c_int;
                    if n > best_len {
                        let mut region: *mut OnigRegion =
                            0 as *mut OnigRegion;
                        if option &
                               ((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                     1 as libc::c_int) << 1 as libc::c_int) <<
                                   1 as libc::c_int != 0 {
                            if n > (*msa).best_len {
                                (*msa).best_len = n;
                                (*msa).best_s = sstart as *mut OnigUChar;
                                current_block = 17728966195399430138;
                            } else { current_block = 4983594971376015098; }
                        } else { current_block = 17728966195399430138; }
                        match current_block {
                            4983594971376015098 => { }
                            _ => {
                                best_len = n;
                                region = (*msa).region;
                                if !region.is_null() {
                                    if (*msa).options &
                                           (((((((((((1 as libc::c_uint) <<
                                                         1 as libc::c_int) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int) <<
                                                   1 as libc::c_int) <<
                                                  1 as libc::c_int) <<
                                                 1 as libc::c_int) <<
                                                1 as libc::c_int) <<
                                               1 as libc::c_int != 0 {
                                        let mut rmt: *mut posix_regmatch_t =
                                            region as *mut posix_regmatch_t;
                                        (*rmt.offset(0 as libc::c_int as
                                                         isize)).rm_so =
                                            sstart.wrapping_offset_from(str)
                                                as libc::c_long as regoff_t;
                                        (*rmt.offset(0 as libc::c_int as
                                                         isize)).rm_eo =
                                            s.wrapping_offset_from(str) as
                                                libc::c_long as regoff_t;
                                        i = 1 as libc::c_int;
                                        while i <= num_mem {
                                            if *mem_end_stk.offset(i as isize)
                                                   !=
                                                   -(1 as libc::c_int) as
                                                       libc::c_long {
                                                if if i <
                                                          (::std::mem::size_of::<BitStatusType>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as libc::c_int {
                                                       ((*reg).bt_mem_start) &
                                                           ((1 as libc::c_int)
                                                                << i) as
                                                               libc::c_uint
                                                   } else {
                                                       ((*reg).bt_mem_start) &
                                                           1 as libc::c_int as
                                                               libc::c_uint
                                                   } != 0 {
                                                    (*rmt.offset(i as
                                                                     isize)).rm_so
                                                        =
                                                        (*stk_base.offset(*mem_start_stk.offset(i
                                                                                                    as
                                                                                                    isize)
                                                                              as
                                                                              isize)).u.mem.pstr.wrapping_offset_from(str)
                                                            as libc::c_long as
                                                            regoff_t
                                                } else {
                                                    (*rmt.offset(i as
                                                                     isize)).rm_so
                                                        =
                                                        (*mem_start_stk.offset(i
                                                                                   as
                                                                                   isize)
                                                             as
                                                             *mut libc::c_void
                                                             as
                                                             *mut OnigUChar).wrapping_offset_from(str)
                                                            as libc::c_long as
                                                            regoff_t
                                                }
                                                (*rmt.offset(i as
                                                                 isize)).rm_eo
                                                    =
                                                    (if (if i <
                                                                (::std::mem::size_of::<BitStatusType>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong)
                                                                    as
                                                                    libc::c_int
                                                            {
                                                             ((*reg).bt_mem_end)
                                                                 &
                                                                 ((1 as
                                                                       libc::c_int)
                                                                      << i) as
                                                                     libc::c_uint
                                                         } else {
                                                             ((*reg).bt_mem_end)
                                                                 &
                                                                 1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint
                                                         }) != 0 {
                                                         (*stk_base.offset(*mem_end_stk.offset(i
                                                                                                   as
                                                                                                   isize)
                                                                               as
                                                                               isize)).u.mem.pstr
                                                     } else {
                                                         *mem_end_stk.offset(i
                                                                                 as
                                                                                 isize)
                                                             as
                                                             *mut libc::c_void
                                                             as *mut OnigUChar
                                                     }).wrapping_offset_from(str)
                                                        as libc::c_long as
                                                        regoff_t
                                            } else {
                                                let ref mut fresh10 =
                                                    (*rmt.offset(i as
                                                                     isize)).rm_eo;
                                                *fresh10 =
                                                    -(1 as libc::c_int);
                                                (*rmt.offset(i as
                                                                 isize)).rm_so
                                                    = *fresh10
                                            }
                                            i += 1
                                        }
                                    } else {
                                        /* if (region) */
                                        /* USE_POSIX_API_REGION_OPTION */
                                        *(*region).beg.offset(0 as libc::c_int
                                                                  as isize) =
                                            sstart.wrapping_offset_from(str)
                                                as libc::c_long as
                                                libc::c_int;
                                        *(*region).end.offset(0 as libc::c_int
                                                                  as isize) =
                                            s.wrapping_offset_from(str) as
                                                libc::c_long as libc::c_int;
                                        i = 1 as libc::c_int;
                                        while i <= num_mem {
                                            if *mem_end_stk.offset(i as isize)
                                                   !=
                                                   -(1 as libc::c_int) as
                                                       libc::c_long {
                                                if if i <
                                                          (::std::mem::size_of::<BitStatusType>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as libc::c_int {
                                                       ((*reg).bt_mem_start) &
                                                           ((1 as libc::c_int)
                                                                << i) as
                                                               libc::c_uint
                                                   } else {
                                                       ((*reg).bt_mem_start) &
                                                           1 as libc::c_int as
                                                               libc::c_uint
                                                   } != 0 {
                                                    *(*region).beg.offset(i as
                                                                              isize)
                                                        =
                                                        (*stk_base.offset(*mem_start_stk.offset(i
                                                                                                    as
                                                                                                    isize)
                                                                              as
                                                                              isize)).u.mem.pstr.wrapping_offset_from(str)
                                                            as libc::c_long as
                                                            libc::c_int
                                                } else {
                                                    *(*region).beg.offset(i as
                                                                              isize)
                                                        =
                                                        (*mem_start_stk.offset(i
                                                                                   as
                                                                                   isize)
                                                             as
                                                             *mut libc::c_void
                                                             as
                                                             *mut OnigUChar).wrapping_offset_from(str)
                                                            as libc::c_long as
                                                            libc::c_int
                                                }
                                                *(*region).end.offset(i as
                                                                          isize)
                                                    =
                                                    (if (if i <
                                                                (::std::mem::size_of::<BitStatusType>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong)
                                                                    as
                                                                    libc::c_int
                                                            {
                                                             ((*reg).bt_mem_end)
                                                                 &
                                                                 ((1 as
                                                                       libc::c_int)
                                                                      << i) as
                                                                     libc::c_uint
                                                         } else {
                                                             ((*reg).bt_mem_end)
                                                                 &
                                                                 1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint
                                                         }) != 0 {
                                                         (*stk_base.offset(*mem_end_stk.offset(i
                                                                                                   as
                                                                                                   isize)
                                                                               as
                                                                               isize)).u.mem.pstr
                                                     } else {
                                                         *mem_end_stk.offset(i
                                                                                 as
                                                                                 isize)
                                                             as
                                                             *mut libc::c_void
                                                             as *mut OnigUChar
                                                     }).wrapping_offset_from(str)
                                                        as libc::c_long as
                                                        libc::c_int
                                            } else {
                                                let ref mut fresh11 =
                                                    *(*region).end.offset(i as
                                                                              isize);
                                                *fresh11 =
                                                    -(1 as libc::c_int);
                                                *(*region).beg.offset(i as
                                                                          isize)
                                                    = *fresh11
                                            }
                                            i += 1
                                        }
                                        if (*reg).capture_history !=
                                               0 as libc::c_int as
                                                   libc::c_uint {
                                            let mut r: libc::c_int = 0;
                                            let mut node:
                                                    *mut OnigCaptureTreeNode =
                                                0 as *mut OnigCaptureTreeNode;
                                            if ((*region).history_root as
                                                    *mut libc::c_void).is_null()
                                               {
                                                node = history_node_new();
                                                (*region).history_root = node;
                                                if (node as
                                                        *mut libc::c_void).is_null()
                                                   {
                                                    return -(5 as libc::c_int)
                                                }
                                            } else {
                                                node = (*region).history_root;
                                                history_tree_clear(node);
                                            }
                                            (*node).group = 0 as libc::c_int;
                                            (*node).beg =
                                                sstart.wrapping_offset_from(str)
                                                    as libc::c_long as
                                                    libc::c_int;
                                            (*node).end =
                                                s.wrapping_offset_from(str) as
                                                    libc::c_long as
                                                    libc::c_int;
                                            stkp = stk_base;
                                            r =
                                                make_capture_history_tree((*region).history_root,
                                                                          &mut stkp,
                                                                          stk,
                                                                          str
                                                                              as
                                                                              *mut OnigUChar,
                                                                          reg);
                                            if r < 0 as libc::c_int {
                                                /* USE_CAPTURE_HISTORY */
                                                best_len = r; /* error code */
                                                current_block =
                                                    10384894314999291408;
                                                break ;
                                            }
                                        }
                                    }
                                    /* else IS_POSIX_REGION() */
                                }
                            }
                        }
                    }
                    if !(option &
                             (((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                    1 as libc::c_int) << 1 as libc::c_int) <<
                                  1 as libc::c_int |
                                  (((((1 as libc::c_uint) << 1 as libc::c_int)
                                         << 1 as libc::c_int) <<
                                        1 as libc::c_int) << 1 as libc::c_int)
                                      << 1 as libc::c_int) != 0) {
                        current_block = 10384894314999291408;
                        break ;
                    }
                    if option &
                           (((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                  1 as libc::c_int) << 1 as libc::c_int) <<
                                1 as libc::c_int) << 1 as libc::c_int != 0 &&
                           s == sstart as *mut OnigUChar {
                        best_len = -(1 as libc::c_int)
                        /* for retry */
                    } else if !(option &
                                    ((((1 as libc::c_uint) <<
                                           1 as libc::c_int) <<
                                          1 as libc::c_int) <<
                                         1 as libc::c_int) << 1 as libc::c_int
                                    != 0 && s < right_range as *mut OnigUChar)
                     {
                        current_block = 10384894314999291408;
                        break ;
                    }
                    current_block = 10515973880943345494;
                }
                2 => {
                    let fresh12 = s;
                    s = s.offset(1);
                    if *p as libc::c_int != *fresh12 as libc::c_int {
                        current_block = 10515973880943345494;
                    } else if s.offset(0 as libc::c_int as isize) >
                                  right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        current_block = 15784326922594879266;
                    }
                }
                14 => {
                    let mut len: libc::c_int = 0;
                    let mut q_0: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut lowbuf: [OnigUChar; 18] = [0; 18];
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        len =
                            (*encode).mbc_case_fold.expect("non-null function pointer")(case_fold_flag,
                                                                                        &mut s
                                                                                            as
                                                                                            *mut *mut OnigUChar
                                                                                            as
                                                                                            *mut *const OnigUChar,
                                                                                        end,
                                                                                        lowbuf.as_mut_ptr());
                        if s.offset(0 as libc::c_int as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            q_0 = lowbuf.as_mut_ptr();
                            loop  {
                                let fresh13 = len;
                                len = len - 1;
                                if !(fresh13 > 0 as libc::c_int) {
                                    current_block = 15784326922594879266;
                                    break ;
                                }
                                if *p as libc::c_int != *q_0 as libc::c_int {
                                    current_block = 10515973880943345494;
                                    break ;
                                }
                                p = p.offset(1);
                                q_0 = q_0.offset(1)
                            }
                        }
                    }
                }
                3 => {
                    if s.offset(2 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            sprev = s;
                            p = p.offset(1);
                            s = s.offset(1);
                            continue ;
                        }
                    }
                }
                4 => {
                    if s.offset(3 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                            } else {
                                sprev = s;
                                p = p.offset(1);
                                s = s.offset(1);
                                continue ;
                            }
                        }
                    }
                }
                5 => {
                    if s.offset(4 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                            } else {
                                p = p.offset(1);
                                s = s.offset(1);
                                if *p as libc::c_int != *s as libc::c_int {
                                    current_block = 10515973880943345494;
                                } else {
                                    sprev = s;
                                    p = p.offset(1);
                                    s = s.offset(1);
                                    continue ;
                                }
                            }
                        }
                    }
                }
                6 => {
                    if s.offset(5 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                            } else {
                                p = p.offset(1);
                                s = s.offset(1);
                                if *p as libc::c_int != *s as libc::c_int {
                                    current_block = 10515973880943345494;
                                } else {
                                    p = p.offset(1);
                                    s = s.offset(1);
                                    if *p as libc::c_int != *s as libc::c_int
                                       {
                                        current_block = 10515973880943345494;
                                    } else {
                                        sprev = s;
                                        p = p.offset(1);
                                        s = s.offset(1);
                                        continue ;
                                    }
                                }
                            }
                        }
                    }
                }
                7 => {
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    if s.offset(tlen as isize) > right_range as *mut OnigUChar
                       {
                        current_block = 10515973880943345494;
                    } else {
                        loop  {
                            let fresh14 = tlen;
                            tlen = tlen - 1;
                            if !(fresh14 > 0 as libc::c_int) {
                                current_block = 981657943452992752;
                                break ;
                            }
                            let fresh15 = p;
                            p = p.offset(1);
                            let fresh16 = s;
                            s = s.offset(1);
                            if *fresh15 as libc::c_int !=
                                   *fresh16 as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                        }
                        match current_block {
                            10515973880943345494 => { }
                            _ => {
                                sprev =
                                    s.offset(-(1 as libc::c_int as isize));
                                continue ;
                            }
                        }
                    }
                }
                15 => {
                    let mut len_0: libc::c_int = 0;
                    let mut q_1: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut endp: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut lowbuf_0: [OnigUChar; 18] = [0; 18];
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    endp = p.offset(tlen as isize);
                    's_1061:
                        loop  {
                            if !(p < endp) { continue 's_246 ; }
                            sprev = s;
                            if s.offset(1 as libc::c_int as isize) >
                                   right_range as *mut OnigUChar {
                                break ;
                            }
                            len_0 =
                                (*encode).mbc_case_fold.expect("non-null function pointer")(case_fold_flag,
                                                                                            &mut s
                                                                                                as
                                                                                                *mut *mut OnigUChar
                                                                                                as
                                                                                                *mut *const OnigUChar,
                                                                                            end,
                                                                                            lowbuf_0.as_mut_ptr());
                            if s.offset(0 as libc::c_int as isize) >
                                   right_range as *mut OnigUChar {
                                break ;
                            }
                            q_1 = lowbuf_0.as_mut_ptr();
                            loop  {
                                let fresh17 = len_0;
                                len_0 = len_0 - 1;
                                if !(fresh17 > 0 as libc::c_int) { break ; }
                                if *p as libc::c_int != *q_1 as libc::c_int {
                                    break 's_1061 ;
                                }
                                p = p.offset(1);
                                q_1 = q_1.offset(1)
                            }
                        }
                    current_block = 10515973880943345494;
                }
                8 => {
                    if s.offset(2 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(1);
                            s = s.offset(1);
                            current_block = 15784326922594879266;
                        }
                    }
                }
                9 => {
                    if s.offset(4 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(1);
                            s = s.offset(1);
                            sprev = s;
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                            } else {
                                p = p.offset(1);
                                s = s.offset(1);
                                if *p as libc::c_int != *s as libc::c_int {
                                    current_block = 10515973880943345494;
                                } else {
                                    p = p.offset(1);
                                    s = s.offset(1);
                                    continue ;
                                }
                            }
                        }
                    }
                }
                10 => {
                    if s.offset(6 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *p as libc::c_int != *s as libc::c_int {
                        current_block = 10515973880943345494;
                    } else {
                        p = p.offset(1);
                        s = s.offset(1);
                        if *p as libc::c_int != *s as libc::c_int {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                            } else {
                                p = p.offset(1);
                                s = s.offset(1);
                                if *p as libc::c_int != *s as libc::c_int {
                                    current_block = 10515973880943345494;
                                } else {
                                    p = p.offset(1);
                                    s = s.offset(1);
                                    sprev = s;
                                    if *p as libc::c_int != *s as libc::c_int
                                       {
                                        current_block = 10515973880943345494;
                                    } else {
                                        p = p.offset(1);
                                        s = s.offset(1);
                                        if *p as libc::c_int !=
                                               *s as libc::c_int {
                                            current_block =
                                                10515973880943345494;
                                        } else {
                                            p = p.offset(1);
                                            s = s.offset(1);
                                            continue ;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                11 => {
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    if s.offset((tlen * 2 as libc::c_int) as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        loop  {
                            let fresh18 = tlen;
                            tlen = tlen - 1;
                            if !(fresh18 > 0 as libc::c_int) {
                                current_block = 6016358353929850055;
                                break ;
                            }
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            p = p.offset(1);
                            s = s.offset(1)
                        }
                        match current_block {
                            10515973880943345494 => { }
                            _ => {
                                sprev =
                                    s.offset(-(2 as libc::c_int as isize));
                                continue ;
                            }
                        }
                    }
                }
                12 => {
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    if s.offset((tlen * 3 as libc::c_int) as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        loop  {
                            let fresh19 = tlen;
                            tlen = tlen - 1;
                            if !(fresh19 > 0 as libc::c_int) {
                                current_block = 8969725751306813258;
                                break ;
                            }
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            p = p.offset(1);
                            s = s.offset(1);
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            p = p.offset(1);
                            s = s.offset(1)
                        }
                        match current_block {
                            10515973880943345494 => { }
                            _ => {
                                sprev =
                                    s.offset(-(3 as libc::c_int as isize));
                                continue ;
                            }
                        }
                    }
                }
                13 => {
                    /* for retry */
                    tlen = *(p as *mut LengthType); /* mb-len */
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize); /* string len */
                    tlen2 =
                        *(p as
                              *mut LengthType); /* OP_CCLASS can match mb-code. \D, \S */
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    tlen2 *= tlen;
                    if s.offset(tlen2 as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        loop  {
                            let fresh20 = tlen2;
                            tlen2 = tlen2 - 1;
                            if !(fresh20 > 0 as libc::c_int) {
                                current_block = 11271090240167667812;
                                break ;
                            }
                            if *p as libc::c_int != *s as libc::c_int {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            p = p.offset(1);
                            s = s.offset(1)
                        }
                        match current_block {
                            10515973880943345494 => { }
                            _ => {
                                sprev = s.offset(-(tlen as isize));
                                continue ;
                            }
                        }
                    }
                }
                16 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *(p as
                                    BitSetRef).offset((*s as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as isize) &
                                  ((1 as libc::c_int) <<
                                       (*s as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)))
                                      as libc::c_uint ==
                                  0 as libc::c_int as libc::c_uint {
                        current_block = 10515973880943345494;
                    } else {
                        p =
                            p.offset(::std::mem::size_of::<BitSet>() as
                                         libc::c_ulong as isize);
                        s =
                            s.offset((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                         as isize);
                        current_block = 15784326922594879266;
                    }
                }
                17 => {
                    if !((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                             != 1 as libc::c_int) {
                        current_block = 10515973880943345494;
                    } else { current_block = 3195428471348745422; }
                }
                18 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if (*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                  != 1 as libc::c_int {
                        p =
                            p.offset(::std::mem::size_of::<BitSet>() as
                                         libc::c_ulong as isize);
                        current_block = 3195428471348745422;
                    } else if *(p as
                                    BitSetRef).offset((*s as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as isize) &
                                  ((1 as libc::c_int) <<
                                       (*s as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)))
                                      as libc::c_uint ==
                                  0 as libc::c_int as libc::c_uint {
                        current_block = 10515973880943345494;
                    } else {
                        p =
                            p.offset(::std::mem::size_of::<BitSet>() as
                                         libc::c_ulong as isize);
                        tlen = *(p as *mut LengthType);
                        p =
                            p.offset(::std::mem::size_of::<LengthType>() as
                                         libc::c_ulong as isize);
                        p = p.offset(tlen as isize);
                        s = s.offset(1);
                        current_block = 15784326922594879266;
                    }
                }
                19 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if *(p as
                                    BitSetRef).offset((*s as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as isize) &
                                  ((1 as libc::c_int) <<
                                       (*s as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)))
                                      as libc::c_uint !=
                                  0 as libc::c_int as libc::c_uint {
                        current_block = 10515973880943345494;
                    } else {
                        p =
                            p.offset(::std::mem::size_of::<BitSet>() as
                                         libc::c_ulong as isize);
                        s =
                            s.offset((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                         as isize);
                        current_block = 15784326922594879266;
                    }
                }
                20 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if !((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                    != 1 as libc::c_int) {
                        s = s.offset(1);
                        tlen = *(p as *mut LengthType);
                        p =
                            p.offset(::std::mem::size_of::<LengthType>() as
                                         libc::c_ulong as isize);
                        p = p.offset(tlen as isize);
                        current_block = 15784326922594879266;
                    } else { current_block = 18129602335150433812; }
                }
                21 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if (*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                  != 1 as libc::c_int {
                        p =
                            p.offset(::std::mem::size_of::<BitSet>() as
                                         libc::c_ulong as isize);
                        current_block = 18129602335150433812;
                    } else if *(p as
                                    BitSetRef).offset((*s as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as isize) &
                                  ((1 as libc::c_int) <<
                                       (*s as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)))
                                      as libc::c_uint !=
                                  0 as libc::c_int as libc::c_uint {
                        current_block = 10515973880943345494;
                    } else {
                        p =
                            p.offset(::std::mem::size_of::<BitSet>() as
                                         libc::c_ulong as isize);
                        tlen = *(p as *mut LengthType);
                        p =
                            p.offset(::std::mem::size_of::<LengthType>() as
                                         libc::c_ulong as isize);
                        p = p.offset(tlen as isize);
                        s = s.offset(1);
                        current_block = 15784326922594879266;
                    }
                }
                22 => {
                    let mut code_1: OnigCodePoint = 0;
                    let mut node_0: *mut libc::c_void =
                        0 as *mut libc::c_void;
                    let mut mb_len_1: libc::c_int = 0;
                    let mut ss_1: *mut OnigUChar = 0 as *mut OnigUChar;
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        node_0 = *(p as *mut PointerType);
                        p =
                            p.offset(::std::mem::size_of::<PointerType>() as
                                         libc::c_ulong as isize);
                        mb_len_1 =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        ss_1 = s;
                        s = s.offset(mb_len_1 as isize);
                        if s.offset(0 as libc::c_int as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            code_1 =
                                (*encode).mbc_to_code.expect("non-null function pointer")(ss_1,
                                                                                          s);
                            if onig_is_code_in_cc_len(mb_len_1, code_1,
                                                      node_0 as
                                                          *mut CClassNode) ==
                                   0 as libc::c_int {
                                current_block = 10515973880943345494;
                            } else { current_block = 15784326922594879266; }
                        }
                    }
                }
                23 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        n =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if s.offset(n as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else if (*encode).is_mbc_newline.expect("non-null function pointer")(s,
                                                                                               end)
                                      != 0 {
                            current_block = 10515973880943345494;
                        } else {
                            s = s.offset(n as isize);
                            current_block = 15784326922594879266;
                        }
                    }
                }
                24 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        n =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if s.offset(n as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            s = s.offset(n as isize);
                            current_block = 15784326922594879266;
                        }
                    }
                }
                25 => {
                    loop  {
                        if !(s < right_range as *mut OnigUChar) {
                            current_block = 15784326922594879266;
                            break ;
                        }
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_0: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_0 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_0
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p;
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1);
                        n =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if s.offset(n as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                            break ;
                        }
                        if (*encode).is_mbc_newline.expect("non-null function pointer")(s,
                                                                                        end)
                               != 0 {
                            current_block = 10515973880943345494;
                            break ;
                        }
                        sprev = s;
                        s = s.offset(n as isize)
                    }
                }
                26 => {
                    loop  {
                        if !(s < right_range as *mut OnigUChar) {
                            current_block = 15784326922594879266;
                            break ;
                        }
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_1: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_1 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_0: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_0);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_0);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_1
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p;
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1);
                        n =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if n > 1 as libc::c_int {
                            if s.offset(n as isize) >
                                   right_range as *mut OnigUChar {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            sprev = s;
                            s = s.offset(n as isize)
                        } else { sprev = s; s = s.offset(1) }
                    }
                }
                27 => {
                    loop  {
                        if !(s < right_range as *mut OnigUChar) {
                            current_block = 2926860427235594157;
                            break ;
                        }
                        if *p as libc::c_int == *s as libc::c_int {
                            if (stk_end.wrapping_offset_from(stk) as
                                    libc::c_long) <
                                   1 as libc::c_int as libc::c_long {
                                let mut r_2: libc::c_int =
                                    stack_double(is_alloca, &mut alloc_base,
                                                 &mut stk_base, &mut stk_end,
                                                 &mut stk, msa);
                                if r_2 != 0 as libc::c_int {
                                    (*msa).stack_n =
                                        stk_end.wrapping_offset_from(stk_base)
                                            as libc::c_long as libc::c_int;
                                    if is_alloca != 0 as libc::c_int {
                                        let mut size_1: size_t =
                                            (::std::mem::size_of::<OnigStackIndex>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
                                        (*msa).stack_p = malloc(size_1);
                                        memcpy((*msa).stack_p,
                                               alloc_base as
                                                   *const libc::c_void,
                                               size_1);
                                    } else {
                                        (*msa).stack_p =
                                            alloc_base as *mut libc::c_void
                                    }
                                    return r_2
                                }
                                is_alloca = 0 as libc::c_int;
                                repeat_stk =
                                    alloc_base as *mut OnigStackIndex;
                                mem_start_stk =
                                    repeat_stk.offset((*reg).num_repeat as
                                                          isize);
                                mem_end_stk =
                                    mem_start_stk.offset(num_mem as isize);
                                mem_start_stk = mem_start_stk.offset(-1);
                                mem_end_stk = mem_end_stk.offset(-1)
                            }
                            (*stk).type_0 =
                                0x1 as libc::c_int as libc::c_uint;
                            (*stk).u.state.pcode =
                                p.offset(1 as libc::c_int as isize);
                            (*stk).u.state.pstr = s;
                            (*stk).u.state.pstr_prev = sprev;
                            stk = stk.offset(1)
                        }
                        n =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if s.offset(n as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                            break ;
                        }
                        if (*encode).is_mbc_newline.expect("non-null function pointer")(s,
                                                                                        end)
                               != 0 {
                            current_block = 10515973880943345494;
                            break ;
                        }
                        sprev = s;
                        s = s.offset(n as isize)
                    }
                    match current_block {
                        10515973880943345494 => { }
                        _ => {
                            p = p.offset(1);
                            current_block = 15784326922594879266;
                        }
                    }
                }
                28 => {
                    loop  {
                        if !(s < right_range as *mut OnigUChar) {
                            current_block = 12184831490747941883;
                            break ;
                        }
                        if *p as libc::c_int == *s as libc::c_int {
                            if (stk_end.wrapping_offset_from(stk) as
                                    libc::c_long) <
                                   1 as libc::c_int as libc::c_long {
                                let mut r_3: libc::c_int =
                                    stack_double(is_alloca, &mut alloc_base,
                                                 &mut stk_base, &mut stk_end,
                                                 &mut stk, msa);
                                if r_3 != 0 as libc::c_int {
                                    (*msa).stack_n =
                                        stk_end.wrapping_offset_from(stk_base)
                                            as libc::c_long as libc::c_int;
                                    if is_alloca != 0 as libc::c_int {
                                        let mut size_2: size_t =
                                            (::std::mem::size_of::<OnigStackIndex>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
                                        (*msa).stack_p = malloc(size_2);
                                        memcpy((*msa).stack_p,
                                               alloc_base as
                                                   *const libc::c_void,
                                               size_2);
                                    } else {
                                        (*msa).stack_p =
                                            alloc_base as *mut libc::c_void
                                    }
                                    return r_3
                                }
                                is_alloca = 0 as libc::c_int;
                                repeat_stk =
                                    alloc_base as *mut OnigStackIndex;
                                mem_start_stk =
                                    repeat_stk.offset((*reg).num_repeat as
                                                          isize);
                                mem_end_stk =
                                    mem_start_stk.offset(num_mem as isize);
                                mem_start_stk = mem_start_stk.offset(-1);
                                mem_end_stk = mem_end_stk.offset(-1)
                            }
                            (*stk).type_0 =
                                0x1 as libc::c_int as libc::c_uint;
                            (*stk).u.state.pcode =
                                p.offset(1 as libc::c_int as isize);
                            (*stk).u.state.pstr = s;
                            (*stk).u.state.pstr_prev = sprev;
                            stk = stk.offset(1)
                        }
                        n =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if n > 1 as libc::c_int {
                            if s.offset(n as isize) >
                                   right_range as *mut OnigUChar {
                                current_block = 10515973880943345494;
                                break ;
                            }
                            sprev = s;
                            s = s.offset(n as isize)
                        } else { sprev = s; s = s.offset(1) }
                    }
                    match current_block {
                        10515973880943345494 => { }
                        _ => {
                            p = p.offset(1);
                            current_block = 15784326922594879266;
                        }
                    }
                }
                29 => {
                    /* USE_COMBINATION_EXPLOSION_CHECK */
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block =
                            10515973880943345494; /* should be before push mem-end. */
                    } else if (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                    end),
                                                                                          12
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              OnigCtype)
                                  == 0 {
                        current_block = 10515973880943345494;
                    } else {
                        s =
                            s.offset((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                         as isize);
                        current_block = 15784326922594879266;
                    }
                }
                30 => {
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else if (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                    end),
                                                                                          12
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              OnigCtype)
                                  != 0 {
                        current_block = 10515973880943345494;
                    } else {
                        s =
                            s.offset((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                         as isize);
                        current_block = 15784326922594879266;
                    }
                }
                31 => {
                    if s == str as *mut OnigUChar {
                        if !(s.offset(1 as libc::c_int as isize) >
                                 right_range as *mut OnigUChar) {
                            if !((*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                       end),
                                                                                             12
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 OnigCtype)
                                     == 0) {
                                continue ;
                            }
                        }
                    } else if s == end as *mut OnigUChar {
                        if !((*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(sprev,
                                                                                                                                                   end),
                                                                                         12
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             OnigCtype)
                                 == 0) {
                            continue ;
                        }
                    } else if !((*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                      end),
                                                                                            12
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                OnigCtype)
                                    ==
                                    (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(sprev,
                                                                                                                                                          end),
                                                                                                12
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    OnigCtype))
                     {
                        continue ;
                    }
                    current_block = 10515973880943345494;
                }
                32 => {
                    if s == str as *mut OnigUChar {
                        if !(s < right_range as *mut OnigUChar &&
                                 (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                       end),
                                                                                             12
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 OnigCtype)
                                     != 0) {
                            continue ;
                        }
                    } else if s == end as *mut OnigUChar {
                        if !((*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(sprev,
                                                                                                                                                   end),
                                                                                         12
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             OnigCtype)
                                 != 0) {
                            continue ;
                        }
                    } else if !((*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                      end),
                                                                                            12
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                OnigCtype)
                                    !=
                                    (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(sprev,
                                                                                                                                                          end),
                                                                                                12
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    OnigCtype))
                     {
                        continue ;
                    }
                    current_block = 10515973880943345494;
                }
                33 => {
                    if s < right_range as *mut OnigUChar &&
                           (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                 end),
                                                                                       12
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           OnigCtype)
                               != 0 {
                        if s == str as *mut OnigUChar ||
                               (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(sprev,
                                                                                                                                                     end),
                                                                                           12
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               OnigCtype)
                                   == 0 {
                            continue ;
                        }
                        current_block = 10515973880943345494;
                    } else { current_block = 10515973880943345494; }
                }
                34 => {
                    if !(s == str as *mut OnigUChar) &&
                           (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(sprev,
                                                                                                                                                 end),
                                                                                       12
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           OnigCtype)
                               != 0 {
                        if s == end as *mut OnigUChar ||
                               (*encode).is_code_ctype.expect("non-null function pointer")((*encode).mbc_to_code.expect("non-null function pointer")(s,
                                                                                                                                                     end),
                                                                                           12
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               OnigCtype)
                                   == 0 {
                            continue ;
                        }
                        current_block = 10515973880943345494;
                    } else { current_block = 10515973880943345494; }
                }
                35 => {
                    if s == str as *mut OnigUChar { continue ; }
                    current_block = 10515973880943345494;
                }
                36 => {
                    if s == end as *mut OnigUChar { continue ; }
                    current_block = 10515973880943345494;
                }
                37 => {
                    if s == str as *mut OnigUChar {
                        if !((*msa).options &
                                 (((((((((1 as libc::c_uint) <<
                                             1 as libc::c_int) <<
                                            1 as libc::c_int) <<
                                           1 as libc::c_int) <<
                                          1 as libc::c_int) <<
                                         1 as libc::c_int) <<
                                        1 as libc::c_int) << 1 as libc::c_int)
                                      << 1 as libc::c_int) << 1 as libc::c_int
                                 != 0) {
                            continue ;
                        }
                    } else if (*encode).is_mbc_newline.expect("non-null function pointer")(sprev,
                                                                                           end)
                                  != 0 && !(s == end as *mut OnigUChar) {
                        continue ;
                    }
                    current_block = 10515973880943345494;
                }
                38 => {
                    if s == end as *mut OnigUChar {
                        if !((*msa).options &
                                 ((((((((((1 as libc::c_uint) <<
                                              1 as libc::c_int) <<
                                             1 as libc::c_int) <<
                                            1 as libc::c_int) <<
                                           1 as libc::c_int) <<
                                          1 as libc::c_int) <<
                                         1 as libc::c_int) <<
                                        1 as libc::c_int) << 1 as libc::c_int)
                                      << 1 as libc::c_int) << 1 as libc::c_int
                                 != 0) {
                            continue ;
                        }
                    } else if (*encode).is_mbc_newline.expect("non-null function pointer")(s,
                                                                                           end)
                                  != 0 {
                        continue ;
                    }
                    current_block = 10515973880943345494;
                }
                39 => {
                    if s == end as *mut OnigUChar {
                        if !((*msa).options &
                                 ((((((((((1 as libc::c_uint) <<
                                              1 as libc::c_int) <<
                                             1 as libc::c_int) <<
                                            1 as libc::c_int) <<
                                           1 as libc::c_int) <<
                                          1 as libc::c_int) <<
                                         1 as libc::c_int) <<
                                        1 as libc::c_int) << 1 as libc::c_int)
                                      << 1 as libc::c_int) << 1 as libc::c_int
                                 != 0) {
                            continue ;
                        }
                    } else if (*encode).is_mbc_newline.expect("non-null function pointer")(s,
                                                                                           end)
                                  != 0 &&
                                  s.offset((*encode).mbc_enc_len.expect("non-null function pointer")(s)
                                               as isize) ==
                                      end as *mut OnigUChar {
                        continue ;
                    }
                    current_block = 10515973880943345494;
                }
                40 => {
                    if !(s != (*msa).start as *mut OnigUChar) { continue ; }
                    current_block = 10515973880943345494;
                }
                49 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_4: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_4 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_3: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_3);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_3);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_4
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x100 as libc::c_int as libc::c_uint;
                    (*stk).u.mem.num = mem as libc::c_int;
                    (*stk).u.mem.pstr = s;
                    (*stk).u.mem.start = *mem_start_stk.offset(mem as isize);
                    (*stk).u.mem.end = *mem_end_stk.offset(mem as isize);
                    *mem_start_stk.offset(mem as isize) =
                        stk.wrapping_offset_from(stk_base) as libc::c_long;
                    *mem_end_stk.offset(mem as isize) =
                        -(1 as libc::c_int) as OnigStackIndex;
                    stk = stk.offset(1);
                    continue ;
                }
                48 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    *mem_start_stk.offset(mem as isize) =
                        s as *mut libc::c_void as OnigStackIndex;
                    continue ;
                }
                50 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_5: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_5 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_4: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_4);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_4);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_5
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x8200 as libc::c_int as libc::c_uint;
                    (*stk).u.mem.num = mem as libc::c_int;
                    (*stk).u.mem.pstr = s;
                    (*stk).u.mem.start = *mem_start_stk.offset(mem as isize);
                    (*stk).u.mem.end = *mem_end_stk.offset(mem as isize);
                    *mem_end_stk.offset(mem as isize) =
                        stk.wrapping_offset_from(stk_base) as libc::c_long;
                    stk = stk.offset(1);
                    continue ;
                }
                52 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    *mem_end_stk.offset(mem as isize) =
                        s as *mut libc::c_void as OnigStackIndex;
                    continue ;
                }
                51 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut level: libc::c_int = 0 as libc::c_int;
                    stkp = stk;
                    while stkp > stk_base {
                        stkp = stkp.offset(-1);
                        if (*stkp).type_0 &
                               0x8000 as libc::c_int as libc::c_uint !=
                               0 as libc::c_int as libc::c_uint &&
                               (*stkp).u.mem.num == mem as libc::c_int {
                            level += 1
                        } else {
                            if !((*stkp).type_0 ==
                                     0x100 as libc::c_int as libc::c_uint &&
                                     (*stkp).u.mem.num == mem as libc::c_int)
                               {
                                continue ;
                            }
                            if level == 0 as libc::c_int { break ; }
                            level -= 1
                        }
                    }
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_6: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_6 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_5: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_5);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_5);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_6
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x8200 as libc::c_int as libc::c_uint;
                    (*stk).u.mem.num = mem as libc::c_int;
                    (*stk).u.mem.pstr = s;
                    (*stk).u.mem.start = *mem_start_stk.offset(mem as isize);
                    (*stk).u.mem.end = *mem_end_stk.offset(mem as isize);
                    *mem_end_stk.offset(mem as isize) =
                        stk.wrapping_offset_from(stk_base) as libc::c_long;
                    stk = stk.offset(1);
                    *mem_start_stk.offset(mem as isize) =
                        stkp.wrapping_offset_from(stk_base) as libc::c_long;
                    continue ;
                }
                53 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    *mem_end_stk.offset(mem as isize) =
                        s as *mut libc::c_void as OnigStackIndex;
                    let mut level_0: libc::c_int = 0 as libc::c_int;
                    stkp = stk;
                    while stkp > stk_base {
                        stkp = stkp.offset(-1);
                        if (*stkp).type_0 &
                               0x8000 as libc::c_int as libc::c_uint !=
                               0 as libc::c_int as libc::c_uint &&
                               (*stkp).u.mem.num == mem as libc::c_int {
                            level_0 += 1
                        } else {
                            if !((*stkp).type_0 ==
                                     0x100 as libc::c_int as libc::c_uint &&
                                     (*stkp).u.mem.num == mem as libc::c_int)
                               {
                                continue ;
                            }
                            if level_0 == 0 as libc::c_int { break ; }
                            level_0 -= 1
                        }
                    }
                    if if (mem as libc::c_int) <
                              (::std::mem::size_of::<BitStatusType>() as
                                   libc::c_ulong).wrapping_mul(8 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong)
                                  as libc::c_int {
                           ((*reg).bt_mem_start) &
                               ((1 as libc::c_int) << mem as libc::c_int) as
                                   libc::c_uint
                       } else {
                           ((*reg).bt_mem_start) &
                               1 as libc::c_int as libc::c_uint
                       } != 0 {
                        *mem_start_stk.offset(mem as isize) =
                            stkp.wrapping_offset_from(stk_base) as
                                libc::c_long
                    } else {
                        *mem_start_stk.offset(mem as isize) =
                            (*stkp).u.mem.pstr as *mut libc::c_void as
                                OnigStackIndex
                    }
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_7: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_7 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_6: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_6);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_6);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_7
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x8400 as libc::c_int as libc::c_uint;
                    (*stk).u.mem.num = mem as libc::c_int;
                    stk = stk.offset(1);
                    continue ;
                }
                41 => {
                    mem = 1 as libc::c_int as MemNumType;
                    current_block = 17708152686995084742;
                }
                42 => {
                    mem = 2 as libc::c_int as MemNumType;
                    current_block = 17708152686995084742;
                }
                43 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    current_block = 17708152686995084742;
                }
                44 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut len_2: libc::c_int = 0;
                    let mut pstart_0: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut pend_0: *mut OnigUChar = 0 as *mut OnigUChar;
                    /* if you want to remove following line, 
           you should check in parse and compile time. */
                    if mem as libc::c_int > num_mem {
                        current_block = 10515973880943345494;
                    } else if *mem_end_stk.offset(mem as isize) ==
                                  -(1 as libc::c_int) as libc::c_long {
                        current_block = 10515973880943345494;
                    } else if *mem_start_stk.offset(mem as isize) ==
                                  -(1 as libc::c_int) as libc::c_long {
                        current_block = 10515973880943345494;
                    } else {
                        if if (mem as libc::c_int) <
                                  (::std::mem::size_of::<BitStatusType>() as
                                       libc::c_ulong).wrapping_mul(8 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
                                      as libc::c_int {
                               ((*reg).bt_mem_start) &
                                   ((1 as libc::c_int) << mem as libc::c_int)
                                       as libc::c_uint
                           } else {
                               ((*reg).bt_mem_start) &
                                   1 as libc::c_int as libc::c_uint
                           } != 0 {
                            pstart_0 =
                                (*stk_base.offset(*mem_start_stk.offset(mem as
                                                                            isize)
                                                      as isize)).u.mem.pstr
                        } else {
                            pstart_0 =
                                *mem_start_stk.offset(mem as isize) as
                                    *mut libc::c_void as *mut OnigUChar
                        }
                        pend_0 =
                            if if (mem as libc::c_int) <
                                      (::std::mem::size_of::<BitStatusType>()
                                           as
                                           libc::c_ulong).wrapping_mul(8 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                          as libc::c_int {
                                   ((*reg).bt_mem_end) &
                                       ((1 as libc::c_int) <<
                                            mem as libc::c_int) as
                                           libc::c_uint
                               } else {
                                   ((*reg).bt_mem_end) &
                                       1 as libc::c_int as libc::c_uint
                               } != 0 {
                                (*stk_base.offset(*mem_end_stk.offset(mem as
                                                                          isize)
                                                      as isize)).u.mem.pstr
                            } else {
                                *mem_end_stk.offset(mem as isize) as
                                    *mut libc::c_void as *mut OnigUChar
                            };
                        n =
                            pend_0.wrapping_offset_from(pstart_0) as
                                libc::c_long as libc::c_int;
                        if s.offset(n as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            sprev = s;
                            if string_cmp_ic(encode,
                                             case_fold_flag as libc::c_int,
                                             pstart_0, &mut s, n) ==
                                   0 as libc::c_int {
                                current_block = 10515973880943345494;
                            } else {
                                loop  {
                                    len_2 =
                                        (*encode).mbc_enc_len.expect("non-null function pointer")(sprev);
                                    if !(sprev.offset(len_2 as isize) < s) {
                                        break ;
                                    }
                                    sprev = sprev.offset(len_2 as isize)
                                }
                                continue ;
                            }
                        }
                    }
                }
                45 => {
                    let mut len_3: libc::c_int = 0;
                    let mut is_fail: libc::c_int = 0;
                    let mut pstart_1: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut pend_1: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut swork: *mut OnigUChar = 0 as *mut OnigUChar;
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    i = 0 as libc::c_int;
                    loop  {
                        if !(i < tlen) {
                            current_block = 8390308638985875266;
                            break ;
                        }
                        mem = *(p as *mut MemNumType);
                        p =
                            p.offset(::std::mem::size_of::<MemNumType>() as
                                         libc::c_ulong as isize);
                        if !(*mem_end_stk.offset(mem as isize) ==
                                 -(1 as libc::c_int) as libc::c_long) {
                            if !(*mem_start_stk.offset(mem as isize) ==
                                     -(1 as libc::c_int) as libc::c_long) {
                                if if (mem as libc::c_int) <
                                          (::std::mem::size_of::<BitStatusType>()
                                               as
                                               libc::c_ulong).wrapping_mul(8
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong)
                                              as libc::c_int {
                                       ((*reg).bt_mem_start) &
                                           ((1 as libc::c_int) <<
                                                mem as libc::c_int) as
                                               libc::c_uint
                                   } else {
                                       ((*reg).bt_mem_start) &
                                           1 as libc::c_int as libc::c_uint
                                   } != 0 {
                                    pstart_1 =
                                        (*stk_base.offset(*mem_start_stk.offset(mem
                                                                                    as
                                                                                    isize)
                                                              as
                                                              isize)).u.mem.pstr
                                } else {
                                    pstart_1 =
                                        *mem_start_stk.offset(mem as isize) as
                                            *mut libc::c_void as
                                            *mut OnigUChar
                                }
                                pend_1 =
                                    if if (mem as libc::c_int) <
                                              (::std::mem::size_of::<BitStatusType>()
                                                   as
                                                   libc::c_ulong).wrapping_mul(8
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int {
                                           ((*reg).bt_mem_end) &
                                               ((1 as libc::c_int) <<
                                                    mem as libc::c_int) as
                                                   libc::c_uint
                                       } else {
                                           ((*reg).bt_mem_end) &
                                               1 as libc::c_int as
                                                   libc::c_uint
                                       } != 0 {
                                        (*stk_base.offset(*mem_end_stk.offset(mem
                                                                                  as
                                                                                  isize)
                                                              as
                                                              isize)).u.mem.pstr
                                    } else {
                                        *mem_end_stk.offset(mem as isize) as
                                            *mut libc::c_void as
                                            *mut OnigUChar
                                    };
                                n =
                                    pend_1.wrapping_offset_from(pstart_1) as
                                        libc::c_long as libc::c_int;
                                if s.offset(n as isize) >
                                       right_range as *mut OnigUChar {
                                    current_block = 10515973880943345494;
                                    break ;
                                }
                                sprev = s;
                                swork = s;
                                is_fail = 0 as libc::c_int;
                                loop  {
                                    let fresh24 = n;
                                    n = n - 1;
                                    if !(fresh24 > 0 as libc::c_int) {
                                        break ;
                                    }
                                    let fresh25 = pstart_1;
                                    pstart_1 = pstart_1.offset(1);
                                    let fresh26 = swork;
                                    swork = swork.offset(1);
                                    if !(*fresh25 as libc::c_int !=
                                             *fresh26 as libc::c_int) {
                                        continue ;
                                    }
                                    is_fail = 1 as libc::c_int;
                                    break ;
                                }
                                if !(is_fail != 0) {
                                    s = swork;
                                    loop  {
                                        len_3 =
                                            (*encode).mbc_enc_len.expect("non-null function pointer")(sprev);
                                        if !(sprev.offset(len_3 as isize) < s)
                                           {
                                            break ;
                                        }
                                        sprev = sprev.offset(len_3 as isize)
                                    }
                                    p =
                                        p.offset((::std::mem::size_of::<MemNumType>()
                                                      as
                                                      libc::c_ulong).wrapping_mul((tlen
                                                                                       -
                                                                                       i
                                                                                       -
                                                                                       1
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_ulong)
                                                     as isize);
                                    current_block = 8390308638985875266;
                                    break ;
                                }
                            }
                        }
                        i += 1
                        /* success */
                    }
                    match current_block {
                        10515973880943345494 => { }
                        _ => {
                            if !(i == tlen) { continue ; }
                            current_block = 10515973880943345494;
                        }
                    }
                }
                46 => {
                    let mut len_4: libc::c_int = 0;
                    let mut is_fail_0: libc::c_int = 0;
                    let mut pstart_2: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut pend_2: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut swork_0: *mut OnigUChar = 0 as *mut OnigUChar;
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    i = 0 as libc::c_int;
                    loop  {
                        if !(i < tlen) {
                            current_block = 16368849584772035169;
                            break ;
                        }
                        mem = *(p as *mut MemNumType);
                        p =
                            p.offset(::std::mem::size_of::<MemNumType>() as
                                         libc::c_ulong as isize);
                        if !(*mem_end_stk.offset(mem as isize) ==
                                 -(1 as libc::c_int) as libc::c_long) {
                            if !(*mem_start_stk.offset(mem as isize) ==
                                     -(1 as libc::c_int) as libc::c_long) {
                                if if (mem as libc::c_int) <
                                          (::std::mem::size_of::<BitStatusType>()
                                               as
                                               libc::c_ulong).wrapping_mul(8
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong)
                                              as libc::c_int {
                                       ((*reg).bt_mem_start) &
                                           ((1 as libc::c_int) <<
                                                mem as libc::c_int) as
                                               libc::c_uint
                                   } else {
                                       ((*reg).bt_mem_start) &
                                           1 as libc::c_int as libc::c_uint
                                   } != 0 {
                                    pstart_2 =
                                        (*stk_base.offset(*mem_start_stk.offset(mem
                                                                                    as
                                                                                    isize)
                                                              as
                                                              isize)).u.mem.pstr
                                } else {
                                    pstart_2 =
                                        *mem_start_stk.offset(mem as isize) as
                                            *mut libc::c_void as
                                            *mut OnigUChar
                                }
                                pend_2 =
                                    if if (mem as libc::c_int) <
                                              (::std::mem::size_of::<BitStatusType>()
                                                   as
                                                   libc::c_ulong).wrapping_mul(8
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int {
                                           ((*reg).bt_mem_end) &
                                               ((1 as libc::c_int) <<
                                                    mem as libc::c_int) as
                                                   libc::c_uint
                                       } else {
                                           ((*reg).bt_mem_end) &
                                               1 as libc::c_int as
                                                   libc::c_uint
                                       } != 0 {
                                        (*stk_base.offset(*mem_end_stk.offset(mem
                                                                                  as
                                                                                  isize)
                                                              as
                                                              isize)).u.mem.pstr
                                    } else {
                                        *mem_end_stk.offset(mem as isize) as
                                            *mut libc::c_void as
                                            *mut OnigUChar
                                    };
                                n =
                                    pend_2.wrapping_offset_from(pstart_2) as
                                        libc::c_long as libc::c_int;
                                if s.offset(n as isize) >
                                       right_range as *mut OnigUChar {
                                    current_block = 10515973880943345494;
                                    break ;
                                }
                                sprev = s;
                                swork_0 = s;
                                if string_cmp_ic(encode,
                                                 case_fold_flag as
                                                     libc::c_int, pstart_2,
                                                 &mut swork_0, n) ==
                                       0 as libc::c_int {
                                    is_fail_0 = 1 as libc::c_int
                                } else { is_fail_0 = 0 as libc::c_int }
                                if !(is_fail_0 != 0) {
                                    s = swork_0;
                                    loop  {
                                        len_4 =
                                            (*encode).mbc_enc_len.expect("non-null function pointer")(sprev);
                                        if !(sprev.offset(len_4 as isize) < s)
                                           {
                                            break ;
                                        }
                                        sprev = sprev.offset(len_4 as isize)
                                    }
                                    p =
                                        p.offset((::std::mem::size_of::<MemNumType>()
                                                      as
                                                      libc::c_ulong).wrapping_mul((tlen
                                                                                       -
                                                                                       i
                                                                                       -
                                                                                       1
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      libc::c_ulong)
                                                     as isize);
                                    current_block = 16368849584772035169;
                                    break ;
                                }
                            }
                        }
                        i += 1
                        /* success */
                    }
                    match current_block {
                        10515973880943345494 => { }
                        _ => {
                            if !(i == tlen) { continue ; }
                            current_block = 10515973880943345494;
                        }
                    }
                }
                47 => {
                    let mut len_5: libc::c_int = 0;
                    let mut ic: OnigOptionType = 0;
                    let mut level_1: LengthType = 0;
                    ic = *(p as *mut OnigOptionType);
                    p =
                        p.offset(::std::mem::size_of::<OnigOptionType>() as
                                     libc::c_ulong as isize);
                    level_1 = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    sprev = s;
                    if backref_match_at_nested_level(reg, stk, stk_base,
                                                     ic as libc::c_int,
                                                     case_fold_flag as
                                                         libc::c_int, level_1,
                                                     tlen, p, &mut s, end) !=
                           0 {
                        loop  {
                            len_5 =
                                (*encode).mbc_enc_len.expect("non-null function pointer")(sprev);
                            if !(sprev.offset(len_5 as isize) < s) { break ; }
                            sprev = sprev.offset(len_5 as isize)
                        }
                        p =
                            p.offset((::std::mem::size_of::<MemNumType>() as
                                          libc::c_ulong).wrapping_mul(tlen as
                                                                          libc::c_ulong)
                                         as isize);
                        continue ;
                    } else { current_block = 10515973880943345494; }
                }
                66 => {
                    /* no need: IS_DYNAMIC_OPTION() == 0 */
                    mem = *(p as *mut MemNumType); /* mem: null check id */
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as
                                     isize); /* mem: null check id */
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_8: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk,
                                         msa); /* mem: null check id */
                        if r_8 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as
                                    libc::c_int; /* mem: null check id */
                            if is_alloca != 0 as libc::c_int {
                                let mut size_7: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_7);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_7);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_8
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x3000 as libc::c_int as libc::c_uint;
                    (*stk).u.null_check.num = mem as libc::c_int;
                    (*stk).u.null_check.pstr = s;
                    stk = stk.offset(1);
                    continue ;
                }
                67 => {
                    isnull = 0;
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut k: *mut OnigStackType = stk;
                    loop  {
                        k = k.offset(-1);
                        if !((*k).type_0 ==
                                 0x3000 as libc::c_int as libc::c_uint) {
                            continue ;
                        }
                        if !((*k).u.null_check.num == mem as libc::c_int) {
                            continue ;
                        }
                        isnull = ((*k).u.null_check.pstr == s) as libc::c_int;
                        break ;
                    }
                    if !(isnull != 0) { continue ; }
                    current_block = 16053602361063078949;
                }
                68 => {
                    let mut isnull_0: libc::c_int = 0;
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut k_0: *mut OnigStackType = stk;
                    loop  {
                        k_0 = k_0.offset(-1);
                        if !((*k_0).type_0 ==
                                 0x3000 as libc::c_int as libc::c_uint) {
                            continue ;
                        }
                        if !((*k_0).u.null_check.num == mem as libc::c_int) {
                            continue ;
                        }
                        if (*k_0).u.null_check.pstr != s {
                            isnull_0 = 0 as libc::c_int;
                            break ;
                        } else {
                            let mut endp_0: *mut OnigUChar =
                                0 as *mut OnigUChar;
                            isnull_0 = 1 as libc::c_int;
                            while k_0 < stk {
                                if (*k_0).type_0 ==
                                       0x100 as libc::c_int as libc::c_uint {
                                    if (*k_0).u.mem.end ==
                                           -(1 as libc::c_int) as libc::c_long
                                       {
                                        isnull_0 = 0 as libc::c_int;
                                        break ;
                                    } else {
                                        if if (*k_0).u.mem.num <
                                                  (::std::mem::size_of::<BitStatusType>()
                                                       as
                                                       libc::c_ulong).wrapping_mul(8
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                                      as libc::c_int {
                                               ((*reg).bt_mem_end) &
                                                   ((1 as libc::c_int) <<
                                                        (*k_0).u.mem.num) as
                                                       libc::c_uint
                                           } else {
                                               ((*reg).bt_mem_end) &
                                                   1 as libc::c_int as
                                                       libc::c_uint
                                           } != 0 {
                                            endp_0 =
                                                (*stk_base.offset((*k_0).u.mem.end
                                                                      as
                                                                      isize)).u.mem.pstr
                                        } else {
                                            endp_0 =
                                                (*k_0).u.mem.end as
                                                    *mut OnigUChar
                                        }
                                        if (*stk_base.offset((*k_0).u.mem.start
                                                                 as
                                                                 isize)).u.mem.pstr
                                               != endp_0 {
                                            isnull_0 = 0 as libc::c_int;
                                            break ;
                                        } else if endp_0 != s {
                                            isnull_0 = -(1 as libc::c_int)
                                        }
                                    }
                                }
                                k_0 = k_0.offset(1)
                            }
                            break ;
                        }
                    }
                    if !(isnull_0 != 0) { continue ; }
                    if isnull_0 == -(1 as libc::c_int) {
                        current_block = 10515973880943345494;
                    } else { current_block = 16053602361063078949; }
                }
                69 => {
                    let mut isnull_1: libc::c_int = 0;
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut level_2: libc::c_int = 0 as libc::c_int;
                    let mut k_1: *mut OnigStackType = stk;
                    loop  {
                        k_1 = k_1.offset(-1);
                        if (*k_1).type_0 ==
                               0x3000 as libc::c_int as libc::c_uint {
                            if !((*k_1).u.null_check.num ==
                                     mem as libc::c_int) {
                                continue ;
                            }
                            if level_2 == 0 as libc::c_int {
                                if (*k_1).u.null_check.pstr != s {
                                    isnull_1 = 0 as libc::c_int;
                                    break ;
                                } else {
                                    let mut endp_1: *mut OnigUChar =
                                        0 as *mut OnigUChar;
                                    isnull_1 = 1 as libc::c_int;
                                    while k_1 < stk {
                                        if (*k_1).type_0 ==
                                               0x100 as libc::c_int as
                                                   libc::c_uint {
                                            if (*k_1).u.mem.end ==
                                                   -(1 as libc::c_int) as
                                                       libc::c_long {
                                                isnull_1 = 0 as libc::c_int;
                                                break ;
                                            } else {
                                                if if (*k_1).u.mem.num <
                                                          (::std::mem::size_of::<BitStatusType>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as libc::c_int {
                                                       ((*reg).bt_mem_end) &
                                                           ((1 as libc::c_int)
                                                                <<
                                                                (*k_1).u.mem.num)
                                                               as libc::c_uint
                                                   } else {
                                                       ((*reg).bt_mem_end) &
                                                           1 as libc::c_int as
                                                               libc::c_uint
                                                   } != 0 {
                                                    endp_1 =
                                                        (*stk_base.offset((*k_1).u.mem.end
                                                                              as
                                                                              isize)).u.mem.pstr
                                                } else {
                                                    endp_1 =
                                                        (*k_1).u.mem.end as
                                                            *mut OnigUChar
                                                }
                                                if (*stk_base.offset((*k_1).u.mem.start
                                                                         as
                                                                         isize)).u.mem.pstr
                                                       != endp_1 {
                                                    isnull_1 =
                                                        0 as libc::c_int;
                                                    break ;
                                                } else if endp_1 != s {
                                                    isnull_1 =
                                                        -(1 as libc::c_int)
                                                }
                                            }
                                        }
                                        k_1 = k_1.offset(1)
                                    }
                                    break ;
                                }
                            } else { level_2 -= 1 }
                        } else if (*k_1).type_0 ==
                                      0x5000 as libc::c_int as libc::c_uint {
                            if (*k_1).u.null_check.num == mem as libc::c_int {
                                level_2 += 1
                            }
                        }
                    }
                    if isnull_1 != 0 {
                        if isnull_1 == -(1 as libc::c_int) {
                            current_block = 10515973880943345494;
                        } else { current_block = 16053602361063078949; }
                    } else {
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_9: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_9 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_8: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_8);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_8);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_9
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x5000 as libc::c_int as libc::c_uint;
                        (*stk).u.null_check.num = mem as libc::c_int;
                        stk = stk.offset(1);
                        continue ;
                    }
                }
                55 => {
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    p = p.offset(addr as isize);
                    continue ;
                }
                56 => {
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_10: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_10 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_9: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_9);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_9);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_10
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                    (*stk).u.state.pcode = p.offset(addr as isize);
                    (*stk).u.state.pstr = s;
                    (*stk).u.state.pstr_prev = sprev;
                    stk = stk.offset(1);
                    continue ;
                }
                57 => {
                    /* USE_COMBINATION_EXPLOSION_CHECK */
                    stk = stk.offset(-1); /* mem: OP_REPEAT ID */
                    continue ; /* mem: OP_REPEAT ID */
                }
                58 => {
                    addr = *(p as *mut RelAddrType); /* mem: OP_REPEAT ID */
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as
                                     isize); /* mem: OP_REPEAT ID */
                    if *p as libc::c_int == *s as libc::c_int &&
                           s < right_range as *mut OnigUChar {
                        p = p.offset(1); /* mem: OP_REPEAT ID */
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_11: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk,
                                             msa); /* mem: OP_REPEAT ID */
                            if r_11 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_10: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_10);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_10);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_11
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p.offset(addr as isize);
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1);
                        continue ;
                    } else {
                        p = p.offset((addr + 1 as libc::c_int) as isize);
                        continue ;
                    }
                }
                59 => {
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    if *p as libc::c_int == *s as libc::c_int {
                        p = p.offset(1);
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_12: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_12 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_11: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_11);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_11);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_12
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p.offset(addr as isize);
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1);
                        continue ;
                    } else { p = p.offset(1); continue ; }
                }
                60 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_13: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_13 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_12: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_12);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_12);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_13
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    *repeat_stk.offset(mem as isize) =
                        stk.wrapping_offset_from(stk_base) as libc::c_long;
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_14: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_14 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_13: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_13);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_13);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_14
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x700 as libc::c_int as libc::c_uint;
                    (*stk).u.repeat.num = mem as libc::c_int;
                    (*stk).u.repeat.pcode = p;
                    (*stk).u.repeat.count = 0 as libc::c_int;
                    stk = stk.offset(1);
                    if (*(*reg).repeat_range.offset(mem as isize)).lower ==
                           0 as libc::c_int {
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_15: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_15 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_14: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_14);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_14);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_15
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p.offset(addr as isize);
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1)
                    }
                    continue ;
                }
                61 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_16: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_16 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_15: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_15);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_15);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_16
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    *repeat_stk.offset(mem as isize) =
                        stk.wrapping_offset_from(stk_base) as libc::c_long;
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_17: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_17 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_16: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_16);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_16);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_17
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x700 as libc::c_int as libc::c_uint;
                    (*stk).u.repeat.num = mem as libc::c_int;
                    (*stk).u.repeat.pcode = p;
                    (*stk).u.repeat.count = 0 as libc::c_int;
                    stk = stk.offset(1);
                    if (*(*reg).repeat_range.offset(mem as isize)).lower ==
                           0 as libc::c_int {
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_18: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_18 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_17: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_17);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_17);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_18
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x1 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p;
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1);
                        p = p.offset(addr as isize)
                    }
                    continue ;
                }
                62 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    si = *repeat_stk.offset(mem as isize);
                    stkp = stk_base.offset(si as isize);
                    current_block = 4187025628485394788;
                }
                64 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut level_3: libc::c_int = 0 as libc::c_int;
                    stkp = stk;
                    loop  {
                        stkp = stkp.offset(-1);
                        if (*stkp).type_0 ==
                               0x700 as libc::c_int as libc::c_uint {
                            if !(level_3 == 0 as libc::c_int) { continue ; }
                            if (*stkp).u.repeat.num == mem as libc::c_int {
                                break ;
                            }
                        } else if (*stkp).type_0 ==
                                      0x800 as libc::c_int as libc::c_uint {
                            level_3 -= 1
                        } else if (*stkp).type_0 ==
                                      0x900 as libc::c_int as libc::c_uint {
                            level_3 += 1
                        }
                    }
                    si = stkp.wrapping_offset_from(stk_base) as libc::c_long;
                    current_block = 4187025628485394788;
                }
                63 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    si = *repeat_stk.offset(mem as isize);
                    stkp = stk_base.offset(si as isize);
                    current_block = 314002374245450226;
                }
                65 => {
                    mem = *(p as *mut MemNumType);
                    p =
                        p.offset(::std::mem::size_of::<MemNumType>() as
                                     libc::c_ulong as isize);
                    let mut level_4: libc::c_int = 0 as libc::c_int;
                    stkp = stk;
                    loop  {
                        stkp = stkp.offset(-1);
                        if (*stkp).type_0 ==
                               0x700 as libc::c_int as libc::c_uint {
                            if !(level_4 == 0 as libc::c_int) { continue ; }
                            if (*stkp).u.repeat.num == mem as libc::c_int {
                                break ;
                            }
                        } else if (*stkp).type_0 ==
                                      0x800 as libc::c_int as libc::c_uint {
                            level_4 -= 1
                        } else if (*stkp).type_0 ==
                                      0x900 as libc::c_int as libc::c_uint {
                            level_4 += 1
                        }
                    }
                    si = stkp.wrapping_offset_from(stk_base) as libc::c_long;
                    current_block = 314002374245450226;
                }
                70 => {
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_25: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_25 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_24: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_24);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_24);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_25
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x500 as libc::c_int as libc::c_uint;
                    (*stk).u.state.pcode = 0 as *mut OnigUChar;
                    (*stk).u.state.pstr = s;
                    (*stk).u.state.pstr_prev = sprev;
                    stk = stk.offset(1);
                    continue ;
                }
                71 => {
                    stkp = stk;
                    loop  {
                        stkp = stkp.offset(-1);
                        if (*stkp).type_0 &
                               0x10ff as libc::c_int as libc::c_uint !=
                               0 as libc::c_int as libc::c_uint {
                            (*stkp).type_0 =
                                0xa00 as libc::c_int as libc::c_uint
                        } else {
                            if !((*stkp).type_0 ==
                                     0x500 as libc::c_int as libc::c_uint) {
                                continue ;
                            }
                            (*stkp).type_0 =
                                0xa00 as libc::c_int as libc::c_uint;
                            break ;
                        }
                    }
                    s = (*stkp).u.state.pstr;
                    sprev = (*stkp).u.state.pstr_prev;
                    continue ;
                }
                72 => {
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_26: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_26 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_25: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_25);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_25);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_26
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x3 as libc::c_int as libc::c_uint;
                    (*stk).u.state.pcode = p.offset(addr as isize);
                    (*stk).u.state.pstr = s;
                    (*stk).u.state.pstr_prev = sprev;
                    stk = stk.offset(1);
                    continue ;
                }
                73 => {
                    loop  {
                        stk = stk.offset(-1);
                        if (*stk).type_0 == 0x3 as libc::c_int as libc::c_uint
                           {
                            break ;
                        }
                        if (*stk).type_0 ==
                               0x100 as libc::c_int as libc::c_uint {
                            *mem_start_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.start;
                            *mem_end_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.end
                        } else if (*stk).type_0 ==
                                      0x300 as libc::c_int as libc::c_uint {
                            let ref mut fresh28 =
                                (*stk_base.offset((*stk).u.repeat_inc.si as
                                                      isize)).u.repeat.count;
                            *fresh28 -= 1
                        } else if (*stk).type_0 ==
                                      0x8200 as libc::c_int as libc::c_uint {
                            *mem_start_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.start;
                            *mem_end_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.end
                        }
                    }
                    current_block = 10515973880943345494;
                }
                74 => {
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_27: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_27 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_26: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_26);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_26);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_27
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x600 as libc::c_int as libc::c_uint;
                    stk = stk.offset(1);
                    continue ;
                }
                75 => {
                    let mut k_2: *mut OnigStackType = stk;
                    loop  {
                        k_2 = k_2.offset(-1);
                        if (*k_2).type_0 &
                               0x10ff as libc::c_int as libc::c_uint !=
                               0 as libc::c_int as libc::c_uint {
                            (*k_2).type_0 =
                                0xa00 as libc::c_int as libc::c_uint
                        } else {
                            if !((*k_2).type_0 ==
                                     0x600 as libc::c_int as libc::c_uint) {
                                continue ;
                            }
                            (*k_2).type_0 =
                                0xa00 as libc::c_int as libc::c_uint;
                            break ;
                        }
                    }
                    continue ;
                }
                76 => {
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    s = onigenc_step_back(encode, str, s, tlen);
                    if (s as *mut libc::c_void).is_null() {
                        current_block = 10515973880943345494;
                    } else {
                        sprev = onigenc_get_prev_char_head(encode, str, s);
                        continue ;
                    }
                }
                77 => {
                    addr = *(p as *mut RelAddrType);
                    p =
                        p.offset(::std::mem::size_of::<RelAddrType>() as
                                     libc::c_ulong as isize);
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    q = onigenc_step_back(encode, str, s, tlen);
                    if (q as *mut libc::c_void).is_null() {
                        /* too short case -> success. ex. /(?<!XXX)a/.match("a")
           If you want to change to fail, replace following line. */
                        p = p.offset(addr as isize)
                        /* goto fail; */
                    } else {
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_28: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_28 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_27: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_27);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_27);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_28
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x2 as libc::c_int as libc::c_uint;
                        (*stk).u.state.pcode = p.offset(addr as isize);
                        (*stk).u.state.pstr = s;
                        (*stk).u.state.pstr_prev = sprev;
                        stk = stk.offset(1);
                        s = q;
                        sprev = onigenc_get_prev_char_head(encode, str, s)
                    }
                    continue ;
                }
                78 => {
                    loop  {
                        stk = stk.offset(-1);
                        if (*stk).type_0 == 0x2 as libc::c_int as libc::c_uint
                           {
                            break ;
                        }
                        if (*stk).type_0 ==
                               0x100 as libc::c_int as libc::c_uint {
                            *mem_start_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.start;
                            *mem_end_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.end
                        } else if (*stk).type_0 ==
                                      0x300 as libc::c_int as libc::c_uint {
                            let ref mut fresh29 =
                                (*stk_base.offset((*stk).u.repeat_inc.si as
                                                      isize)).u.repeat.count;
                            *fresh29 -= 1
                        } else if (*stk).type_0 ==
                                      0x8200 as libc::c_int as libc::c_uint {
                            *mem_start_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.start;
                            *mem_end_stk.offset((*stk).u.mem.num as isize) =
                                (*stk).u.mem.end
                        }
                    }
                    current_block = 10515973880943345494;
                }
                79 => {
                    addr = *(p as *mut AbsAddrType);
                    p =
                        p.offset(::std::mem::size_of::<AbsAddrType>() as
                                     libc::c_ulong as isize);
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_29: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_29 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_28: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_28);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_28);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_29
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x800 as libc::c_int as libc::c_uint;
                    (*stk).u.call_frame.ret_addr = p;
                    stk = stk.offset(1);
                    p = (*reg).p.offset(addr as isize);
                    continue ;
                }
                80 => {
                    let mut level_5: libc::c_int = 0 as libc::c_int;
                    let mut k_3: *mut OnigStackType = stk;
                    loop  {
                        k_3 = k_3.offset(-1);
                        if (*k_3).type_0 ==
                               0x800 as libc::c_int as libc::c_uint {
                            if level_5 == 0 as libc::c_int {
                                p = (*k_3).u.call_frame.ret_addr;
                                break ;
                            } else { level_5 -= 1 }
                        } else if (*k_3).type_0 ==
                                      0x900 as libc::c_int as libc::c_uint {
                            level_5 += 1
                        }
                    }
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_30: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_30 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_29: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_29);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_29);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_30
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x900 as libc::c_int as libc::c_uint;
                    stk = stk.offset(1);
                    continue ;
                }
                0 => { current_block = 10384894314999291408; break ; }
                54 => { current_block = 10515973880943345494; }
                _ => { current_block = 5846525744100762332; break ; }
            }
            match current_block {
                314002374245450226 => {
                    (*stkp).u.repeat.count += 1;
                    if (*stkp).u.repeat.count <
                           (*(*reg).repeat_range.offset(mem as isize)).upper {
                        if (*stkp).u.repeat.count >=
                               (*(*reg).repeat_range.offset(mem as
                                                                isize)).lower
                           {
                            let mut pcode: *mut OnigUChar =
                                (*stkp).u.repeat.pcode;
                            if (stk_end.wrapping_offset_from(stk) as
                                    libc::c_long) <
                                   1 as libc::c_int as libc::c_long {
                                let mut r_21: libc::c_int =
                                    stack_double(is_alloca, &mut alloc_base,
                                                 &mut stk_base, &mut stk_end,
                                                 &mut stk, msa);
                                if r_21 != 0 as libc::c_int {
                                    (*msa).stack_n =
                                        stk_end.wrapping_offset_from(stk_base)
                                            as libc::c_long as libc::c_int;
                                    if is_alloca != 0 as libc::c_int {
                                        let mut size_20: size_t =
                                            (::std::mem::size_of::<OnigStackIndex>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
                                        (*msa).stack_p = malloc(size_20);
                                        memcpy((*msa).stack_p,
                                               alloc_base as
                                                   *const libc::c_void,
                                               size_20);
                                    } else {
                                        (*msa).stack_p =
                                            alloc_base as *mut libc::c_void
                                    }
                                    return r_21
                                }
                                is_alloca = 0 as libc::c_int;
                                repeat_stk =
                                    alloc_base as *mut OnigStackIndex;
                                mem_start_stk =
                                    repeat_stk.offset((*reg).num_repeat as
                                                          isize);
                                mem_end_stk =
                                    mem_start_stk.offset(num_mem as isize);
                                mem_start_stk = mem_start_stk.offset(-1);
                                mem_end_stk = mem_end_stk.offset(-1)
                            }
                            (*stk).type_0 =
                                0x300 as libc::c_int as libc::c_uint;
                            (*stk).u.repeat_inc.si = si;
                            stk = stk.offset(1);
                            if (stk_end.wrapping_offset_from(stk) as
                                    libc::c_long) <
                                   1 as libc::c_int as libc::c_long {
                                let mut r_22: libc::c_int =
                                    stack_double(is_alloca, &mut alloc_base,
                                                 &mut stk_base, &mut stk_end,
                                                 &mut stk, msa);
                                if r_22 != 0 as libc::c_int {
                                    (*msa).stack_n =
                                        stk_end.wrapping_offset_from(stk_base)
                                            as libc::c_long as libc::c_int;
                                    if is_alloca != 0 as libc::c_int {
                                        let mut size_21: size_t =
                                            (::std::mem::size_of::<OnigStackIndex>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
                                        (*msa).stack_p = malloc(size_21);
                                        memcpy((*msa).stack_p,
                                               alloc_base as
                                                   *const libc::c_void,
                                               size_21);
                                    } else {
                                        (*msa).stack_p =
                                            alloc_base as *mut libc::c_void
                                    }
                                    return r_22
                                }
                                is_alloca = 0 as libc::c_int;
                                repeat_stk =
                                    alloc_base as *mut OnigStackIndex;
                                mem_start_stk =
                                    repeat_stk.offset((*reg).num_repeat as
                                                          isize);
                                mem_end_stk =
                                    mem_start_stk.offset(num_mem as isize);
                                mem_start_stk = mem_start_stk.offset(-1);
                                mem_end_stk = mem_end_stk.offset(-1)
                            }
                            (*stk).type_0 =
                                0x1 as libc::c_int as libc::c_uint;
                            (*stk).u.state.pcode = pcode;
                            (*stk).u.state.pstr = s;
                            (*stk).u.state.pstr_prev = sprev;
                            stk = stk.offset(1)
                        } else {
                            p = (*stkp).u.repeat.pcode;
                            if (stk_end.wrapping_offset_from(stk) as
                                    libc::c_long) <
                                   1 as libc::c_int as libc::c_long {
                                let mut r_23: libc::c_int =
                                    stack_double(is_alloca, &mut alloc_base,
                                                 &mut stk_base, &mut stk_end,
                                                 &mut stk, msa);
                                if r_23 != 0 as libc::c_int {
                                    (*msa).stack_n =
                                        stk_end.wrapping_offset_from(stk_base)
                                            as libc::c_long as libc::c_int;
                                    if is_alloca != 0 as libc::c_int {
                                        let mut size_22: size_t =
                                            (::std::mem::size_of::<OnigStackIndex>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
                                        (*msa).stack_p = malloc(size_22);
                                        memcpy((*msa).stack_p,
                                               alloc_base as
                                                   *const libc::c_void,
                                               size_22);
                                    } else {
                                        (*msa).stack_p =
                                            alloc_base as *mut libc::c_void
                                    }
                                    return r_23
                                }
                                is_alloca = 0 as libc::c_int;
                                repeat_stk =
                                    alloc_base as *mut OnigStackIndex;
                                mem_start_stk =
                                    repeat_stk.offset((*reg).num_repeat as
                                                          isize);
                                mem_end_stk =
                                    mem_start_stk.offset(num_mem as isize);
                                mem_start_stk = mem_start_stk.offset(-1);
                                mem_end_stk = mem_end_stk.offset(-1)
                            }
                            (*stk).type_0 =
                                0x300 as libc::c_int as libc::c_uint;
                            (*stk).u.repeat_inc.si = si;
                            stk = stk.offset(1)
                        }
                    } else if (*stkp).u.repeat.count ==
                                  (*(*reg).repeat_range.offset(mem as
                                                                   isize)).upper
                     {
                        if (stk_end.wrapping_offset_from(stk) as libc::c_long)
                               < 1 as libc::c_int as libc::c_long {
                            let mut r_24: libc::c_int =
                                stack_double(is_alloca, &mut alloc_base,
                                             &mut stk_base, &mut stk_end,
                                             &mut stk, msa);
                            if r_24 != 0 as libc::c_int {
                                (*msa).stack_n =
                                    stk_end.wrapping_offset_from(stk_base) as
                                        libc::c_long as libc::c_int;
                                if is_alloca != 0 as libc::c_int {
                                    let mut size_23: size_t =
                                        (::std::mem::size_of::<OnigStackIndex>()
                                             as
                                             libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                             as
                                                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                              as
                                                                                                                                              libc::c_ulong));
                                    (*msa).stack_p = malloc(size_23);
                                    memcpy((*msa).stack_p,
                                           alloc_base as *const libc::c_void,
                                           size_23);
                                } else {
                                    (*msa).stack_p =
                                        alloc_base as *mut libc::c_void
                                }
                                return r_24
                            }
                            is_alloca = 0 as libc::c_int;
                            repeat_stk = alloc_base as *mut OnigStackIndex;
                            mem_start_stk =
                                repeat_stk.offset((*reg).num_repeat as isize);
                            mem_end_stk =
                                mem_start_stk.offset(num_mem as isize);
                            mem_start_stk = mem_start_stk.offset(-1);
                            mem_end_stk = mem_end_stk.offset(-1)
                        }
                        (*stk).type_0 = 0x300 as libc::c_int as libc::c_uint;
                        (*stk).u.repeat_inc.si = si;
                        stk = stk.offset(1)
                    }
                    continue ;
                }
                4187025628485394788 => {
                    (*stkp).u.repeat.count += 1;
                    if !((*stkp).u.repeat.count >=
                             (*(*reg).repeat_range.offset(mem as
                                                              isize)).upper) {
                        if (*stkp).u.repeat.count >=
                               (*(*reg).repeat_range.offset(mem as
                                                                isize)).lower
                           {
                            if (stk_end.wrapping_offset_from(stk) as
                                    libc::c_long) <
                                   1 as libc::c_int as libc::c_long {
                                let mut r_19: libc::c_int =
                                    stack_double(is_alloca, &mut alloc_base,
                                                 &mut stk_base, &mut stk_end,
                                                 &mut stk, msa);
                                if r_19 != 0 as libc::c_int {
                                    (*msa).stack_n =
                                        stk_end.wrapping_offset_from(stk_base)
                                            as libc::c_long as libc::c_int;
                                    if is_alloca != 0 as libc::c_int {
                                        let mut size_18: size_t =
                                            (::std::mem::size_of::<OnigStackIndex>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
                                        (*msa).stack_p = malloc(size_18);
                                        memcpy((*msa).stack_p,
                                               alloc_base as
                                                   *const libc::c_void,
                                               size_18);
                                    } else {
                                        (*msa).stack_p =
                                            alloc_base as *mut libc::c_void
                                    }
                                    return r_19
                                }
                                is_alloca = 0 as libc::c_int;
                                repeat_stk =
                                    alloc_base as *mut OnigStackIndex;
                                mem_start_stk =
                                    repeat_stk.offset((*reg).num_repeat as
                                                          isize);
                                mem_end_stk =
                                    mem_start_stk.offset(num_mem as isize);
                                mem_start_stk = mem_start_stk.offset(-1);
                                mem_end_stk = mem_end_stk.offset(-1)
                            }
                            (*stk).type_0 =
                                0x1 as libc::c_int as libc::c_uint;
                            (*stk).u.state.pcode = p;
                            (*stk).u.state.pstr = s;
                            (*stk).u.state.pstr_prev = sprev;
                            stk = stk.offset(1);
                            p = (*stk_base.offset(si as isize)).u.repeat.pcode
                            /* Don't use stkp after PUSH. */
                        } else { p = (*stkp).u.repeat.pcode }
                    }
                    if (stk_end.wrapping_offset_from(stk) as libc::c_long) <
                           1 as libc::c_int as libc::c_long {
                        let mut r_20: libc::c_int =
                            stack_double(is_alloca, &mut alloc_base,
                                         &mut stk_base, &mut stk_end,
                                         &mut stk, msa);
                        if r_20 != 0 as libc::c_int {
                            (*msa).stack_n =
                                stk_end.wrapping_offset_from(stk_base) as
                                    libc::c_long as libc::c_int;
                            if is_alloca != 0 as libc::c_int {
                                let mut size_19: size_t =
                                    (::std::mem::size_of::<OnigStackIndex>()
                                         as
                                         libc::c_ulong).wrapping_mul((*msa).ptr_num
                                                                         as
                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                                          as
                                                                                                                                          libc::c_ulong));
                                (*msa).stack_p = malloc(size_19);
                                memcpy((*msa).stack_p,
                                       alloc_base as *const libc::c_void,
                                       size_19);
                            } else {
                                (*msa).stack_p =
                                    alloc_base as *mut libc::c_void
                            }
                            return r_20
                        }
                        is_alloca = 0 as libc::c_int;
                        repeat_stk = alloc_base as *mut OnigStackIndex;
                        mem_start_stk =
                            repeat_stk.offset((*reg).num_repeat as isize);
                        mem_end_stk = mem_start_stk.offset(num_mem as isize);
                        mem_start_stk = mem_start_stk.offset(-1);
                        mem_end_stk = mem_end_stk.offset(-1)
                    }
                    (*stk).type_0 = 0x300 as libc::c_int as libc::c_uint;
                    (*stk).u.repeat_inc.si = si;
                    stk = stk.offset(1);
                    continue ;
                }
                16053602361063078949 =>
                /* empty loop founded, skip next instruction */
                {
                    let fresh27 = p;
                    p = p.offset(1);
                    match *fresh27 as libc::c_int {
                        55 | 56 => {
                            p =
                                p.offset(::std::mem::size_of::<RelAddrType>()
                                             as libc::c_ulong as isize);
                            continue ;
                        }
                        62 | 63 | 64 | 65 => {
                            p =
                                p.offset(::std::mem::size_of::<MemNumType>()
                                             as libc::c_ulong as isize);
                            continue ;
                        }
                        _ => { current_block = 367271055897903750; break ; }
                    }
                }
                17708152686995084742 => {
                    let mut len_1: libc::c_int = 0;
                    let mut pstart: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut pend: *mut OnigUChar = 0 as *mut OnigUChar;
                    /* if you want to remove following line, 
           you should check in parse and compile time. */
                    if mem as libc::c_int > num_mem {
                        current_block = 10515973880943345494;
                    } else if *mem_end_stk.offset(mem as isize) ==
                                  -(1 as libc::c_int) as libc::c_long {
                        current_block = 10515973880943345494;
                    } else if *mem_start_stk.offset(mem as isize) ==
                                  -(1 as libc::c_int) as libc::c_long {
                        current_block = 10515973880943345494;
                    } else {
                        if if (mem as libc::c_int) <
                                  (::std::mem::size_of::<BitStatusType>() as
                                       libc::c_ulong).wrapping_mul(8 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
                                      as libc::c_int {
                               ((*reg).bt_mem_start) &
                                   ((1 as libc::c_int) << mem as libc::c_int)
                                       as libc::c_uint
                           } else {
                               ((*reg).bt_mem_start) &
                                   1 as libc::c_int as libc::c_uint
                           } != 0 {
                            pstart =
                                (*stk_base.offset(*mem_start_stk.offset(mem as
                                                                            isize)
                                                      as isize)).u.mem.pstr
                        } else {
                            pstart =
                                *mem_start_stk.offset(mem as isize) as
                                    *mut libc::c_void as *mut OnigUChar
                        }
                        pend =
                            if if (mem as libc::c_int) <
                                      (::std::mem::size_of::<BitStatusType>()
                                           as
                                           libc::c_ulong).wrapping_mul(8 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                          as libc::c_int {
                                   ((*reg).bt_mem_end) &
                                       ((1 as libc::c_int) <<
                                            mem as libc::c_int) as
                                           libc::c_uint
                               } else {
                                   ((*reg).bt_mem_end) &
                                       1 as libc::c_int as libc::c_uint
                               } != 0 {
                                (*stk_base.offset(*mem_end_stk.offset(mem as
                                                                          isize)
                                                      as isize)).u.mem.pstr
                            } else {
                                *mem_end_stk.offset(mem as isize) as
                                    *mut libc::c_void as *mut OnigUChar
                            };
                        n =
                            pend.wrapping_offset_from(pstart) as libc::c_long
                                as libc::c_int;
                        if s.offset(n as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            sprev = s;
                            loop  {
                                let fresh21 = n;
                                n = n - 1;
                                if !(fresh21 > 0 as libc::c_int) {
                                    current_block = 2068877374059494045;
                                    break ;
                                }
                                let fresh22 = pstart;
                                pstart = pstart.offset(1);
                                let fresh23 = s;
                                s = s.offset(1);
                                if *fresh22 as libc::c_int !=
                                       *fresh23 as libc::c_int {
                                    current_block = 10515973880943345494;
                                    break ;
                                }
                            }
                            match current_block {
                                10515973880943345494 => { }
                                _ => {
                                    loop  {
                                        len_1 =
                                            (*encode).mbc_enc_len.expect("non-null function pointer")(sprev);
                                        if !(sprev.offset(len_1 as isize) < s)
                                           {
                                            break ;
                                        }
                                        sprev = sprev.offset(len_1 as isize)
                                    }
                                    continue ;
                                }
                            }
                        }
                    }
                }
                18129602335150433812 => {
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    let mut code_0: OnigCodePoint = 0;
                    let mut ss_0: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut mb_len_0: libc::c_int =
                        (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                    if !(s.offset(mb_len_0 as isize) <=
                             right_range as *mut OnigUChar) {
                        if s.offset(1 as libc::c_int as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            s = end as *mut OnigUChar;
                            p = p.offset(tlen as isize);
                            current_block = 15784326922594879266;
                        }
                    } else {
                        ss_0 = s;
                        s = s.offset(mb_len_0 as isize);
                        code_0 =
                            (*encode).mbc_to_code.expect("non-null function pointer")(ss_0,
                                                                                      s);
                        if onig_is_in_code_range(p, code_0) != 0 {
                            current_block = 10515973880943345494;
                        } else {
                            p = p.offset(tlen as isize);
                            current_block = 15784326922594879266;
                        }
                    }
                }
                3195428471348745422 => {
                    tlen = *(p as *mut LengthType);
                    p =
                        p.offset(::std::mem::size_of::<LengthType>() as
                                     libc::c_ulong as isize);
                    let mut code: OnigCodePoint = 0;
                    let mut ss: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut mb_len: libc::c_int = 0;
                    if s.offset(1 as libc::c_int as isize) >
                           right_range as *mut OnigUChar {
                        current_block = 10515973880943345494;
                    } else {
                        mb_len =
                            (*encode).mbc_enc_len.expect("non-null function pointer")(s);
                        if s.offset(mb_len as isize) >
                               right_range as *mut OnigUChar {
                            current_block = 10515973880943345494;
                        } else {
                            ss = s;
                            s = s.offset(mb_len as isize);
                            code =
                                (*encode).mbc_to_code.expect("non-null function pointer")(ss,
                                                                                          s);
                            if onig_is_in_code_range(p, code) == 0 {
                                current_block = 10515973880943345494;
                            } else {
                                p = p.offset(tlen as isize);
                                current_block = 15784326922594879266;
                            }
                        }
                    }
                }
                _ => { }
            }
            match current_block {
                15784326922594879266 => { sprev = sbegin }
                _ =>
                /* fall */
                {
                    match pop_level {
                        0 => {
                            loop  {
                                stk = stk.offset(-1);
                                if (*stk).type_0 &
                                       0xff as libc::c_int as libc::c_uint !=
                                       0 as libc::c_int as libc::c_uint {
                                    break ;
                                }
                            }
                        }
                        1 => {
                            loop  {
                                stk = stk.offset(-1);
                                if (*stk).type_0 &
                                       0xff as libc::c_int as libc::c_uint !=
                                       0 as libc::c_int as libc::c_uint {
                                    break ;
                                }
                                if (*stk).type_0 ==
                                       0x100 as libc::c_int as libc::c_uint {
                                    *mem_start_stk.offset((*stk).u.mem.num as
                                                              isize) =
                                        (*stk).u.mem.start;
                                    *mem_end_stk.offset((*stk).u.mem.num as
                                                            isize) =
                                        (*stk).u.mem.end
                                }
                            }
                        }
                        _ => {
                            loop  {
                                stk = stk.offset(-1);
                                if (*stk).type_0 &
                                       0xff as libc::c_int as libc::c_uint !=
                                       0 as libc::c_int as libc::c_uint {
                                    break ;
                                }
                                if (*stk).type_0 ==
                                       0x100 as libc::c_int as libc::c_uint {
                                    *mem_start_stk.offset((*stk).u.mem.num as
                                                              isize) =
                                        (*stk).u.mem.start;
                                    *mem_end_stk.offset((*stk).u.mem.num as
                                                            isize) =
                                        (*stk).u.mem.end
                                } else if (*stk).type_0 ==
                                              0x300 as libc::c_int as
                                                  libc::c_uint {
                                    let ref mut fresh30 =
                                        (*stk_base.offset((*stk).u.repeat_inc.si
                                                              as
                                                              isize)).u.repeat.count;
                                    *fresh30 -= 1
                                } else if (*stk).type_0 ==
                                              0x8200 as libc::c_int as
                                                  libc::c_uint {
                                    *mem_start_stk.offset((*stk).u.mem.num as
                                                              isize) =
                                        (*stk).u.mem.start;
                                    *mem_end_stk.offset((*stk).u.mem.num as
                                                            isize) =
                                        (*stk).u.mem.end
                                }
                            }
                        }
                    }
                    p = (*stk).u.state.pcode;
                    s = (*stk).u.state.pstr;
                    sprev = (*stk).u.state.pstr_prev
                }
            }
        }
    match current_block {
        367271055897903750 => {
            (*msa).stack_n =
                stk_end.wrapping_offset_from(stk_base) as libc::c_long as
                    libc::c_int;
            if is_alloca != 0 as libc::c_int {
                let mut size_32: size_t =
                    (::std::mem::size_of::<OnigStackIndex>() as
                         libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                          as
                                                                                                                          libc::c_ulong));
                (*msa).stack_p = malloc(size_32);
                memcpy((*msa).stack_p, alloc_base as *const libc::c_void,
                       size_32);
            } else { (*msa).stack_p = alloc_base as *mut libc::c_void }
            return -(14 as libc::c_int)
        }
        5846525744100762332 => {
            (*msa).stack_n =
                stk_end.wrapping_offset_from(stk_base) as libc::c_long as
                    libc::c_int;
            if is_alloca != 0 as libc::c_int {
                let mut size_31: size_t =
                    (::std::mem::size_of::<OnigStackIndex>() as
                         libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                          as
                                                                                                                          libc::c_ulong));
                (*msa).stack_p = malloc(size_31);
                memcpy((*msa).stack_p, alloc_base as *const libc::c_void,
                       size_31);
            } else { (*msa).stack_p = alloc_base as *mut libc::c_void }
            return -(13 as libc::c_int)
        }
        _ =>
        /* default behavior: return first-matching result. */
        {
            (*msa).stack_n =
                stk_end.wrapping_offset_from(stk_base) as libc::c_long as
                    libc::c_int;
            if is_alloca != 0 as libc::c_int {
                let mut size_30: size_t =
                    (::std::mem::size_of::<OnigStackIndex>() as
                         libc::c_ulong).wrapping_mul((*msa).ptr_num as
                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigStackType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*msa).stack_n
                                                                                                                          as
                                                                                                                          libc::c_ulong));
                (*msa).stack_p = malloc(size_30);
                memcpy((*msa).stack_p, alloc_base as *const libc::c_void,
                       size_30);
            } else { (*msa).stack_p = alloc_base as *mut libc::c_void }
            return best_len
        }
    };
}
unsafe extern "C" fn slow_search(mut enc: OnigEncoding,
                                 mut target: *mut OnigUChar,
                                 mut target_end: *mut OnigUChar,
                                 mut text: *const OnigUChar,
                                 mut text_end: *const OnigUChar,
                                 mut text_range: *mut OnigUChar)
 -> *mut OnigUChar {
    let mut t: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    end = text_end as *mut OnigUChar;
    end =
        end.offset(-((target_end.wrapping_offset_from(target) as libc::c_long
                          - 1 as libc::c_int as libc::c_long) as isize));
    if end > text_range { end = text_range }
    s = text as *mut OnigUChar;
    while s < end {
        if *s as libc::c_int == *target as libc::c_int {
            p = s.offset(1 as libc::c_int as isize);
            t = target.offset(1 as libc::c_int as isize);
            while t < target_end {
                let fresh31 = p;
                p = p.offset(1);
                if *t as libc::c_int != *fresh31 as libc::c_int { break ; }
                t = t.offset(1)
            }
            if t == target_end { return s }
        }
        s =
            s.offset((*enc).mbc_enc_len.expect("non-null function pointer")(s)
                         as isize)
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn str_lower_case_match(mut enc: OnigEncoding,
                                          mut case_fold_flag: libc::c_int,
                                          mut t: *const OnigUChar,
                                          mut tend: *const OnigUChar,
                                          mut p: *const OnigUChar,
                                          mut end: *const OnigUChar)
 -> libc::c_int {
    let mut lowlen: libc::c_int = 0;
    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut lowbuf: [OnigUChar; 18] = [0; 18];
    while t < tend {
        lowlen =
            (*enc).mbc_case_fold.expect("non-null function pointer")(case_fold_flag
                                                                         as
                                                                         OnigCaseFoldType,
                                                                     &mut p as
                                                                         *mut *const OnigUChar,
                                                                     end,
                                                                     lowbuf.as_mut_ptr());
        q = lowbuf.as_mut_ptr();
        while lowlen > 0 as libc::c_int {
            let fresh32 = t;
            t = t.offset(1);
            let fresh33 = q;
            q = q.offset(1);
            if *fresh32 as libc::c_int != *fresh33 as libc::c_int {
                return 0 as libc::c_int
            }
            lowlen -= 1
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn slow_search_ic(mut enc: OnigEncoding,
                                    mut case_fold_flag: libc::c_int,
                                    mut target: *mut OnigUChar,
                                    mut target_end: *mut OnigUChar,
                                    mut text: *const OnigUChar,
                                    mut text_end: *const OnigUChar,
                                    mut text_range: *mut OnigUChar)
 -> *mut OnigUChar {
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    end = text_end as *mut OnigUChar;
    end =
        end.offset(-((target_end.wrapping_offset_from(target) as libc::c_long
                          - 1 as libc::c_int as libc::c_long) as isize));
    if end > text_range { end = text_range }
    s = text as *mut OnigUChar;
    while s < end {
        if str_lower_case_match(enc, case_fold_flag, target, target_end, s,
                                text_end) != 0 {
            return s
        }
        s =
            s.offset((*enc).mbc_enc_len.expect("non-null function pointer")(s)
                         as isize)
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn slow_search_backward(mut enc: OnigEncoding,
                                          mut target: *mut OnigUChar,
                                          mut target_end: *mut OnigUChar,
                                          mut text: *const OnigUChar,
                                          mut adjust_text: *const OnigUChar,
                                          mut text_end: *const OnigUChar,
                                          mut text_start: *const OnigUChar)
 -> *mut OnigUChar {
    let mut t: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    s = text_end as *mut OnigUChar;
    s =
        s.offset(-(target_end.wrapping_offset_from(target) as libc::c_long as
                       isize));
    if s > text_start as *mut OnigUChar {
        s = text_start as *mut OnigUChar
    } else {
        s =
            (*enc).left_adjust_char_head.expect("non-null function pointer")(adjust_text,
                                                                             s)
    }
    while s >= text as *mut OnigUChar {
        if *s as libc::c_int == *target as libc::c_int {
            p = s.offset(1 as libc::c_int as isize);
            t = target.offset(1 as libc::c_int as isize);
            while t < target_end {
                let fresh34 = p;
                p = p.offset(1);
                if *t as libc::c_int != *fresh34 as libc::c_int { break ; }
                t = t.offset(1)
            }
            if t == target_end { return s }
        }
        s = onigenc_get_prev_char_head(enc, adjust_text, s)
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn slow_search_backward_ic(mut enc: OnigEncoding,
                                             mut case_fold_flag: libc::c_int,
                                             mut target: *mut OnigUChar,
                                             mut target_end: *mut OnigUChar,
                                             mut text: *const OnigUChar,
                                             mut adjust_text:
                                                 *const OnigUChar,
                                             mut text_end: *const OnigUChar,
                                             mut text_start: *const OnigUChar)
 -> *mut OnigUChar {
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    s = text_end as *mut OnigUChar;
    s =
        s.offset(-(target_end.wrapping_offset_from(target) as libc::c_long as
                       isize));
    if s > text_start as *mut OnigUChar {
        s = text_start as *mut OnigUChar
    } else {
        s =
            (*enc).left_adjust_char_head.expect("non-null function pointer")(adjust_text,
                                                                             s)
    }
    while s >= text as *mut OnigUChar {
        if str_lower_case_match(enc, case_fold_flag, target, target_end, s,
                                text_end) != 0 {
            return s
        }
        s = onigenc_get_prev_char_head(enc, adjust_text, s)
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn bm_search_notrev(mut reg: *mut regex_t,
                                      mut target: *const OnigUChar,
                                      mut target_end: *const OnigUChar,
                                      mut text: *const OnigUChar,
                                      mut text_end: *const OnigUChar,
                                      mut text_range: *const OnigUChar)
 -> *mut OnigUChar {
    let mut s: *const OnigUChar = 0 as *const OnigUChar;
    let mut se: *const OnigUChar = 0 as *const OnigUChar;
    let mut t: *const OnigUChar = 0 as *const OnigUChar;
    let mut p: *const OnigUChar = 0 as *const OnigUChar;
    let mut end: *const OnigUChar = 0 as *const OnigUChar;
    let mut tail: *const OnigUChar = 0 as *const OnigUChar;
    let mut skip: libc::c_int = 0;
    let mut tlen1: libc::c_int = 0;
    tail = target_end.offset(-(1 as libc::c_int as isize));
    tlen1 = tail.wrapping_offset_from(target) as libc::c_long as libc::c_int;
    end = text_range;
    if end.offset(tlen1 as isize) > text_end {
        end = text_end.offset(-(tlen1 as isize))
    }
    s = text;
    if ((*reg).int_map as *mut libc::c_void).is_null() {
        while s < end {
            se = s.offset(tlen1 as isize);
            p = se;
            t = tail;
            while *p as libc::c_int == *t as libc::c_int {
                if t == target { return s as *mut OnigUChar }
                p = p.offset(-1);
                t = t.offset(-1)
            }
            skip = (*reg).map[*se as usize] as libc::c_int;
            t = s;
            loop  {
                s =
                    s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                 as isize);
                if !((s.wrapping_offset_from(t) as libc::c_long) <
                         skip as libc::c_long && s < end) {
                    break ;
                }
            }
        }
    } else {
        while s < end {
            se = s.offset(tlen1 as isize);
            p = se;
            t = tail;
            while *p as libc::c_int == *t as libc::c_int {
                if t == target { return s as *mut OnigUChar }
                p = p.offset(-1);
                t = t.offset(-1)
            }
            skip = *(*reg).int_map.offset(*se as isize);
            t = s;
            loop  {
                s =
                    s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                 as isize);
                if !((s.wrapping_offset_from(t) as libc::c_long) <
                         skip as libc::c_long && s < end) {
                    break ;
                }
            }
        }
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn bm_search(mut reg: *mut regex_t,
                               mut target: *const OnigUChar,
                               mut target_end: *const OnigUChar,
                               mut text: *const OnigUChar,
                               mut text_end: *const OnigUChar,
                               mut text_range: *const OnigUChar)
 -> *mut OnigUChar {
    let mut s: *const OnigUChar = 0 as *const OnigUChar;
    let mut t: *const OnigUChar = 0 as *const OnigUChar;
    let mut p: *const OnigUChar = 0 as *const OnigUChar;
    let mut end: *const OnigUChar = 0 as *const OnigUChar;
    let mut tail: *const OnigUChar = 0 as *const OnigUChar;
    end =
        text_range.offset(target_end.wrapping_offset_from(target) as
                              libc::c_long as
                              isize).offset(-(1 as libc::c_int as isize));
    if end > text_end { end = text_end }
    tail = target_end.offset(-(1 as libc::c_int as isize));
    s =
        text.offset(target_end.wrapping_offset_from(target) as libc::c_long as
                        isize).offset(-(1 as libc::c_int as isize));
    if ((*reg).int_map as *mut libc::c_void).is_null() {
        while s < end {
            p = s;
            t = tail;
            while *p as libc::c_int == *t as libc::c_int {
                if t == target { return p as *mut OnigUChar }
                p = p.offset(-1);
                t = t.offset(-1)
            }
            s = s.offset((*reg).map[*s as usize] as libc::c_int as isize)
        }
    } else {
        /* see int_map[] */
        while s < end {
            p = s;
            t = tail;
            while *p as libc::c_int == *t as libc::c_int {
                if t == target { return p as *mut OnigUChar }
                p = p.offset(-1);
                t = t.offset(-1)
            }
            s = s.offset(*(*reg).int_map.offset(*s as isize) as isize)
        }
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn map_search(mut enc: OnigEncoding,
                                mut map: *mut OnigUChar,
                                mut text: *const OnigUChar,
                                mut text_range: *const OnigUChar)
 -> *mut OnigUChar {
    let mut s: *const OnigUChar = text;
    while s < text_range {
        if *map.offset(*s as isize) != 0 { return s as *mut OnigUChar }
        s =
            s.offset((*enc).mbc_enc_len.expect("non-null function pointer")(s)
                         as isize)
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
unsafe extern "C" fn map_search_backward(mut enc: OnigEncoding,
                                         mut map: *mut OnigUChar,
                                         mut text: *const OnigUChar,
                                         mut adjust_text: *const OnigUChar,
                                         mut text_start: *const OnigUChar)
 -> *mut OnigUChar {
    let mut s: *const OnigUChar = text_start;
    while s >= text {
        if *map.offset(*s as isize) != 0 { return s as *mut OnigUChar }
        s = onigenc_get_prev_char_head(enc, adjust_text, s)
    }
    return 0 as *mut libc::c_void as *mut OnigUChar;
}
#[no_mangle]
pub unsafe extern "C" fn onig_match(mut reg: *mut regex_t,
                                    mut str: *const OnigUChar,
                                    mut end: *const OnigUChar,
                                    mut at: *const OnigUChar,
                                    mut region: *mut OnigRegion,
                                    mut option: OnigOptionType)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut msa: OnigMatchArg =
        OnigMatchArg{stack_p: 0 as *mut libc::c_void,
                     stack_n: 0,
                     options: 0,
                     region: 0 as *mut OnigRegion,
                     ptr_num: 0,
                     start: 0 as *const OnigUChar,
                     best_len: 0,
                     best_s: 0 as *mut OnigUChar,};
    msa.stack_p = 0 as *mut libc::c_void;
    msa.options = option;
    msa.region = region;
    msa.start = at;
    msa.best_len = -(1 as libc::c_int);
    msa.ptr_num = (*reg).num_repeat + (*reg).num_mem * 2 as libc::c_int;
    if !region.is_null() &&
           option &
               (((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                            1 as libc::c_int) << 1 as libc::c_int) <<
                          1 as libc::c_int) << 1 as libc::c_int) <<
                        1 as libc::c_int) << 1 as libc::c_int) <<
                      1 as libc::c_int) << 1 as libc::c_int) <<
                    1 as libc::c_int) << 1 as libc::c_int == 0 {
        r =
            onig_region_resize_clear(region,
                                     (*reg).num_mem + 1 as libc::c_int)
    } else { r = 0 as libc::c_int }
    if r == 0 as libc::c_int {
        if option &
               ((((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                             1 as libc::c_int) << 1 as libc::c_int) <<
                           1 as libc::c_int) << 1 as libc::c_int) <<
                         1 as libc::c_int) << 1 as libc::c_int) <<
                       1 as libc::c_int) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int != 0 {
            if (*(*reg).enc).is_valid_mbc_string.expect("non-null function pointer")(str,
                                                                                     end)
                   == 0 {
                r = -(400 as libc::c_int);
                current_block = 16328370901755691455;
            } else { current_block = 4956146061682418353; }
        } else { current_block = 4956146061682418353; }
        match current_block {
            16328370901755691455 => { }
            _ => {
                prev = onigenc_get_prev_char_head((*reg).enc, str, at);
                r = match_at(reg, str, end, end, at, prev, &mut msa)
            }
        }
    }
    if !msa.stack_p.is_null() { free(msa.stack_p); }
    return r;
}
unsafe extern "C" fn forward_search_range(mut reg: *mut regex_t,
                                          mut str: *const OnigUChar,
                                          mut end: *const OnigUChar,
                                          mut s: *mut OnigUChar,
                                          mut range: *mut OnigUChar,
                                          mut low: *mut *mut OnigUChar,
                                          mut high: *mut *mut OnigUChar,
                                          mut low_prev: *mut *mut OnigUChar)
 -> libc::c_int {
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut pprev: *mut OnigUChar = 0 as *mut libc::c_void as *mut OnigUChar;
    p = s;
    if (*reg).dmin > 0 as libc::c_int as libc::c_uint {
        if (*(*reg).enc).max_enc_len == 1 as libc::c_int {
            p = p.offset((*reg).dmin as isize)
        } else {
            let mut q: *mut OnigUChar = p.offset((*reg).dmin as isize);
            while p < q {
                p =
                    p.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize)
            }
        }
    }
    loop  {
        match (*reg).optimize {
            1 => {
                p =
                    slow_search((*reg).enc, (*reg).exact, (*reg).exact_end, p,
                                end, range)
            }
            4 => {
                p =
                    slow_search_ic((*reg).enc,
                                   (*reg).case_fold_flag as libc::c_int,
                                   (*reg).exact, (*reg).exact_end, p, end,
                                   range)
            }
            2 => {
                p =
                    bm_search(reg, (*reg).exact, (*reg).exact_end, p, end,
                              range)
            }
            3 => {
                p =
                    bm_search_notrev(reg, (*reg).exact, (*reg).exact_end, p,
                                     end, range)
            }
            5 => {
                p = map_search((*reg).enc, (*reg).map.as_mut_ptr(), p, range)
            }
            _ => { }
        }
        if !p.is_null() && p < range {
            if !(p.offset(-((*reg).dmin as isize)) < s) {
                if !((*reg).sub_anchor != 0) { break ; }
                let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
                match (*reg).sub_anchor {
                    2 => {
                        if p == str as *mut OnigUChar { break ; }
                        prev =
                            onigenc_get_prev_char_head((*reg).enc,
                                                       if !pprev.is_null() {
                                                           pprev
                                                       } else { str }, p);
                        if !((*(*reg).enc).is_mbc_newline.expect("non-null function pointer")(prev,
                                                                                              end)
                                 == 0) {
                            break ;
                        }
                    }
                    32 => {
                        if p == end as *mut OnigUChar { break ; }
                        if !((*(*reg).enc).is_mbc_newline.expect("non-null function pointer")(p,
                                                                                              end)
                                 == 0) {
                            break ;
                        }
                    }
                    _ => { break ; }
                }
            }
            pprev = p;
            p =
                p.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize)
            /* success */
        } else { return 0 as libc::c_int }
    }
    if (*reg).dmax == 0 as libc::c_int as libc::c_uint {
        *low = p;
        if !low_prev.is_null() {
            if *low > s {
                *low_prev = onigenc_get_prev_char_head((*reg).enc, s, p)
            } else {
                *low_prev =
                    onigenc_get_prev_char_head((*reg).enc,
                                               if !pprev.is_null() {
                                                   pprev
                                               } else { str }, p)
            }
        }
    } else if (*reg).dmax != !(0 as libc::c_int as OnigLen) {
        *low = p.offset(-((*reg).dmax as isize));
        if *low > s {
            *low =
                onigenc_get_right_adjust_char_head_with_prev((*reg).enc, s,
                                                             *low,
                                                             low_prev as
                                                                 *mut *const OnigUChar);
            if !low_prev.is_null() &&
                   (*low_prev as *mut libc::c_void).is_null() {
                *low_prev =
                    onigenc_get_prev_char_head((*reg).enc,
                                               if !pprev.is_null() {
                                                   pprev
                                               } else { s }, *low)
            }
        } else if !low_prev.is_null() {
            *low_prev =
                onigenc_get_prev_char_head((*reg).enc,
                                           if !pprev.is_null() {
                                               pprev
                                           } else { str }, *low)
        }
    }
    /* no needs to adjust *high, *high is used as range check only */
    *high = p.offset(-((*reg).dmin as isize));
    return 1 as libc::c_int;
    /* fail */
}
unsafe extern "C" fn backward_search_range(mut reg: *mut regex_t,
                                           mut str: *const OnigUChar,
                                           mut end: *const OnigUChar,
                                           mut s: *mut OnigUChar,
                                           mut range: *const OnigUChar,
                                           mut adjrange: *mut OnigUChar,
                                           mut low: *mut *mut OnigUChar,
                                           mut high: *mut *mut OnigUChar)
 -> libc::c_int {
    let mut current_block: u64;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    range = range.offset((*reg).dmin as isize);
    p = s;
    loop  {
        match (*reg).optimize {
            4 => {
                p =
                    slow_search_backward_ic((*reg).enc,
                                            (*reg).case_fold_flag as
                                                libc::c_int, (*reg).exact,
                                            (*reg).exact_end, range, adjrange,
                                            end, p)
            }
            1 | 2 | 3 => {
                p =
                    slow_search_backward((*reg).enc, (*reg).exact,
                                         (*reg).exact_end, range, adjrange,
                                         end, p)
            }
            5 => {
                p =
                    map_search_backward((*reg).enc, (*reg).map.as_mut_ptr(),
                                        range, adjrange, p)
            }
            _ => { }
        }
        if p.is_null() { current_block = 16025749893437322430; break ; }
        if !((*reg).sub_anchor != 0) {
            current_block = 15125582407903384992;
            break ;
            /* success */
        }
        let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
        match (*reg).sub_anchor {
            2 => {
                if p == str as *mut OnigUChar {
                    current_block = 15125582407903384992;
                    break ;
                }
                prev = onigenc_get_prev_char_head((*reg).enc, str, p);
                if !((*(*reg).enc).is_mbc_newline.expect("non-null function pointer")(prev,
                                                                                      end)
                         == 0) {
                    current_block = 15125582407903384992;
                    break ;
                }
                p = prev
            }
            32 => {
                if p == end as *mut OnigUChar {
                    current_block = 15125582407903384992;
                    break ;
                }
                if !((*(*reg).enc).is_mbc_newline.expect("non-null function pointer")(p,
                                                                                      end)
                         == 0) {
                    current_block = 15125582407903384992;
                    break ;
                }
                p = onigenc_get_prev_char_head((*reg).enc, adjrange, p);
                if (p as *mut libc::c_void).is_null() {
                    current_block = 16025749893437322430;
                    break ;
                }
            }
            _ => { current_block = 15125582407903384992; break ; }
        }
    }
    match current_block {
        16025749893437322430 => { return 0 as libc::c_int }
        _ => {
            /* no needs to adjust *high, *high is used as range check only */
            if (*reg).dmax != !(0 as libc::c_int as OnigLen) {
                *low = p.offset(-((*reg).dmax as isize));
                *high = p.offset(-((*reg).dmin as isize));
                *high =
                    onigenc_get_right_adjust_char_head((*reg).enc, adjrange,
                                                       *high)
            }
            return 1 as libc::c_int
        }
    };
    /* fail */
}
#[no_mangle]
pub unsafe extern "C" fn onig_search(mut reg: *mut regex_t,
                                     mut str: *const OnigUChar,
                                     mut end: *const OnigUChar,
                                     mut start: *const OnigUChar,
                                     mut range: *const OnigUChar,
                                     mut region: *mut OnigRegion,
                                     mut option: OnigOptionType)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut s: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut msa: OnigMatchArg =
        OnigMatchArg{stack_p: 0 as *mut libc::c_void,
                     stack_n: 0,
                     options: 0,
                     region: 0 as *mut OnigRegion,
                     ptr_num: 0,
                     start: 0 as *const OnigUChar,
                     best_len: 0,
                     best_s: 0 as *mut OnigUChar,};
    let mut orig_start: *const OnigUChar = start;
    let mut orig_range: *const OnigUChar = range;
    if !region.is_null() &&
           option &
               (((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                            1 as libc::c_int) << 1 as libc::c_int) <<
                          1 as libc::c_int) << 1 as libc::c_int) <<
                        1 as libc::c_int) << 1 as libc::c_int) <<
                      1 as libc::c_int) << 1 as libc::c_int) <<
                    1 as libc::c_int) << 1 as libc::c_int == 0 {
        r =
            onig_region_resize_clear(region,
                                     (*reg).num_mem + 1 as libc::c_int);
        if r != 0 {
            current_block = 1867614760774316006;
        } else { current_block = 6483416627284290920; }
    } else { current_block = 6483416627284290920; }
    match current_block {
        6483416627284290920 => {
            if start > end || start < str {
                current_block = 7323759974791646890;
            } else {
                if option &
                       ((((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                     1 as libc::c_int) << 1 as libc::c_int) <<
                                   1 as libc::c_int) << 1 as libc::c_int) <<
                                 1 as libc::c_int) << 1 as libc::c_int) <<
                               1 as libc::c_int) << 1 as libc::c_int) <<
                             1 as libc::c_int) << 1 as libc::c_int) <<
                           1 as libc::c_int != 0 {
                    if (*(*reg).enc).is_valid_mbc_string.expect("non-null function pointer")(str,
                                                                                             end)
                           == 0 {
                        r = -(400 as libc::c_int);
                        current_block = 1867614760774316006;
                    } else { current_block = 12209867499936983673; }
                } else { current_block = 12209867499936983673; }
                match current_block {
                    1867614760774316006 => { }
                    _ =>
                    /* error */
                    /* USE_FIND_LONGEST_SEARCH_ALL_OF_RANGE */
                    /* USE_MATCH_RANGE_MUST_BE_INSIDE_OF_SPECIFIED_RANGE */
                    /* anchor optimize: resume search range */
                    {
                        if (*reg).anchor != 0 as libc::c_int && str < end {
                            let mut min_semi_end: *mut OnigUChar =
                                0 as *mut OnigUChar;
                            let mut max_semi_end: *mut OnigUChar =
                                0 as *mut OnigUChar;
                            if (*reg).anchor &
                                   (1 as libc::c_int) << 2 as libc::c_int != 0
                               {
                                current_block = 17309678852997893213;
                            } else if (*reg).anchor &
                                          (1 as libc::c_int) <<
                                              0 as libc::c_int != 0 {
                                /* search str-position only */
                                if range > start {
                                    if start != str {
                                        current_block = 7323759974791646890;
                                    } else {
                                        range =
                                            str.offset(1 as libc::c_int as
                                                           isize);
                                        current_block = 6002151390280567665;
                                    }
                                } else if range <= str {
                                    start = str;
                                    range = str;
                                    current_block = 6002151390280567665;
                                } else {
                                    current_block = 7323759974791646890;
                                }
                            } else {
                                if (*reg).anchor &
                                       (1 as libc::c_int) << 3 as libc::c_int
                                       != 0 {
                                    max_semi_end = end as *mut OnigUChar;
                                    min_semi_end = max_semi_end;
                                    current_block = 8680878510672297395;
                                } else if (*reg).anchor &
                                              (1 as libc::c_int) <<
                                                  4 as libc::c_int != 0 {
                                    let mut pre_end: *mut OnigUChar =
                                        onigenc_step_back((*reg).enc, str,
                                                          end,
                                                          1 as libc::c_int);
                                    max_semi_end = end as *mut OnigUChar;
                                    if (*(*reg).enc).is_mbc_newline.expect("non-null function pointer")(pre_end,
                                                                                                        end)
                                           != 0 {
                                        min_semi_end = pre_end;
                                        if min_semi_end >
                                               str as *mut OnigUChar &&
                                               start <= min_semi_end {
                                            current_block =
                                                8680878510672297395;
                                        } else {
                                            current_block =
                                                6002151390280567665;
                                        }
                                    } else {
                                        min_semi_end = end as *mut OnigUChar;
                                        current_block = 8680878510672297395;
                                    }
                                } else if (*reg).anchor &
                                              (1 as libc::c_int) <<
                                                  15 as libc::c_int != 0 {
                                    current_block = 17309678852997893213;
                                } else {
                                    current_block = 6002151390280567665;
                                }
                                match current_block {
                                    17309678852997893213 => { }
                                    6002151390280567665 => { }
                                    _ => {
                                        if (max_semi_end.wrapping_offset_from(str)
                                                as libc::c_long as OnigLen) <
                                               (*reg).anchor_dmin {
                                            current_block =
                                                7323759974791646890;
                                        } else if range > start {
                                            if min_semi_end.wrapping_offset_from(start)
                                                   as libc::c_long as OnigLen
                                                   > (*reg).anchor_dmax {
                                                start =
                                                    min_semi_end.offset(-((*reg).anchor_dmax
                                                                              as
                                                                              isize));
                                                if start < end {
                                                    start =
                                                        onigenc_get_right_adjust_char_head((*reg).enc,
                                                                                           str,
                                                                                           start)
                                                }
                                            }
                                            if (max_semi_end.wrapping_offset_from(range.offset(-(1
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize)))
                                                    as libc::c_long as
                                                    OnigLen) <
                                                   (*reg).anchor_dmin {
                                                range =
                                                    max_semi_end.offset(-((*reg).anchor_dmin
                                                                              as
                                                                              isize)).offset(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 isize)
                                            }
                                            if start > range {
                                                current_block =
                                                    7323759974791646890;
                                            } else {
                                                current_block =
                                                    6002151390280567665;
                                            }
                                            /* If start == range, match with empty at end.
           Backward search is used. */
                                        } else {
                                            if min_semi_end.wrapping_offset_from(range)
                                                   as libc::c_long as OnigLen
                                                   > (*reg).anchor_dmax {
                                                range =
                                                    min_semi_end.offset(-((*reg).anchor_dmax
                                                                              as
                                                                              isize))
                                            }
                                            if (max_semi_end.wrapping_offset_from(start)
                                                    as libc::c_long as
                                                    OnigLen) <
                                                   (*reg).anchor_dmin {
                                                start =
                                                    max_semi_end.offset(-((*reg).anchor_dmin
                                                                              as
                                                                              isize));
                                                start =
                                                    (*(*reg).enc).left_adjust_char_head.expect("non-null function pointer")(str,
                                                                                                                            start)
                                            }
                                            if range > start {
                                                current_block =
                                                    7323759974791646890;
                                            } else {
                                                current_block =
                                                    6002151390280567665;
                                            }
                                        }
                                    }
                                }
                            }
                            match current_block {
                                6002151390280567665 => { }
                                7323759974791646890 => { }
                                _ =>
                                /* search start-position only */
                                {
                                    if range > start {
                                        range =
                                            start.offset(1 as libc::c_int as
                                                             isize)
                                    } else { range = start }
                                    current_block = 6002151390280567665;
                                }
                            }
                        } else if str == end {
                            /* empty string */
                            static mut address_for_empty_string:
                                   *const OnigUChar =
                                b"\x00" as *const u8 as *const libc::c_char as
                                    *mut OnigUChar;
                            if (*reg).threshold_len == 0 as libc::c_int {
                                str = address_for_empty_string;
                                end = str;
                                start = end;
                                s = start as *mut OnigUChar;
                                prev =
                                    0 as *mut libc::c_void as *mut OnigUChar;
                                msa.stack_p = 0 as *mut libc::c_void;
                                msa.options = option;
                                msa.region = region;
                                msa.start = start;
                                msa.best_len = -(1 as libc::c_int);
                                msa.ptr_num =
                                    (*reg).num_repeat +
                                        (*reg).num_mem * 2 as libc::c_int;
                                r =
                                    match_at(reg, str, end, end, s, prev,
                                             &mut msa);
                                if r != -(1 as libc::c_int) {
                                    if r >= 0 as libc::c_int {
                                        if (*reg).options &
                                               ((((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int) <<
                                                   1 as libc::c_int == 0 {
                                            current_block =
                                                14253232777612267885;
                                        } else {
                                            current_block =
                                                14514155837299642798;
                                        }
                                    } else {
                                        current_block = 4824798144739327838;
                                    }
                                } else {
                                    current_block = 14514155837299642798;
                                }
                            } else { current_block = 7323759974791646890; }
                        } else { current_block = 6002151390280567665; }
                        match current_block {
                            7323759974791646890 => { }
                            _ => {
                                match current_block {
                                    6002151390280567665 => {
                                        msa.stack_p = 0 as *mut libc::c_void;
                                        msa.options = option;
                                        msa.region = region;
                                        msa.start = orig_start;
                                        msa.best_len = -(1 as libc::c_int);
                                        msa.ptr_num =
                                            (*reg).num_repeat +
                                                (*reg).num_mem *
                                                    2 as libc::c_int;
                                        s = start as *mut OnigUChar;
                                        if range > start {
                                            /* forward search */
                                            if s > str as *mut OnigUChar {
                                                prev =
                                                    onigenc_get_prev_char_head((*reg).enc,
                                                                               str,
                                                                               s)
                                            } else {
                                                prev =
                                                    0 as *mut libc::c_void as
                                                        *mut OnigUChar
                                            }
                                            if (*reg).optimize !=
                                                   0 as libc::c_int {
                                                let mut sch_range:
                                                        *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                let mut low: *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                let mut high: *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                let mut low_prev:
                                                        *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                sch_range =
                                                    range as *mut OnigUChar;
                                                if (*reg).dmax !=
                                                       0 as libc::c_int as
                                                           libc::c_uint {
                                                    if (*reg).dmax ==
                                                           !(0 as libc::c_int
                                                                 as OnigLen) {
                                                        sch_range =
                                                            end as
                                                                *mut OnigUChar
                                                    } else {
                                                        sch_range =
                                                            sch_range.offset((*reg).dmax
                                                                                 as
                                                                                 isize);
                                                        if sch_range >
                                                               end as
                                                                   *mut OnigUChar
                                                           {
                                                            sch_range =
                                                                end as
                                                                    *mut OnigUChar
                                                        }
                                                    }
                                                }
                                                if (end.wrapping_offset_from(start)
                                                        as libc::c_long) <
                                                       (*reg).threshold_len as
                                                           libc::c_long {
                                                    current_block =
                                                        14514155837299642798;
                                                } else if (*reg).dmax !=
                                                              !(0 as
                                                                    libc::c_int
                                                                    as
                                                                    OnigLen) {
                                                    's_534:
                                                        loop  {
                                                            if forward_search_range(reg,
                                                                                    str,
                                                                                    end,
                                                                                    s,
                                                                                    sch_range,
                                                                                    &mut low,
                                                                                    &mut high,
                                                                                    &mut low_prev)
                                                                   == 0 {
                                                                current_block
                                                                    =
                                                                    14514155837299642798;
                                                                break ;
                                                            }
                                                            if s < low {
                                                                s = low;
                                                                prev =
                                                                    low_prev
                                                            }
                                                            while s <= high {
                                                                r =
                                                                    match_at(reg,
                                                                             str,
                                                                             end,
                                                                             orig_range,
                                                                             s,
                                                                             prev,
                                                                             &mut msa);
                                                                if r !=
                                                                       -(1 as
                                                                             libc::c_int)
                                                                   {
                                                                    if !(r >=
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                       {
                                                                        current_block
                                                                            =
                                                                            4824798144739327838;
                                                                        break
                                                                            's_534
                                                                            ;
                                                                    }
                                                                    if (*reg).options
                                                                           &
                                                                           ((((1
                                                                                   as
                                                                                   libc::c_uint)
                                                                                  <<
                                                                                  1
                                                                                      as
                                                                                      libc::c_int)
                                                                                 <<
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                <<
                                                                                1
                                                                                    as
                                                                                    libc::c_int)
                                                                               <<
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                           ==
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            14253232777612267885;
                                                                        break
                                                                            's_534
                                                                            ;
                                                                    }
                                                                }
                                                                prev = s;
                                                                s =
                                                                    s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                                                                 as
                                                                                 isize)
                                                            }
                                                            if !(s <
                                                                     range as
                                                                         *mut OnigUChar)
                                                               {
                                                                current_block
                                                                    =
                                                                    14514155837299642798;
                                                                break ;
                                                            }
                                                        }
                                                } else if forward_search_range(reg,
                                                                               str,
                                                                               end,
                                                                               s,
                                                                               sch_range,
                                                                               &mut low,
                                                                               &mut high,
                                                                               0
                                                                                   as
                                                                                   *mut libc::c_void
                                                                                   as
                                                                                   *mut *mut OnigUChar)
                                                              == 0 {
                                                    current_block =
                                                        14514155837299642798;
                                                } else if (*reg).anchor &
                                                              (1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  14 as
                                                                      libc::c_int
                                                              !=
                                                              0 as libc::c_int
                                                 {
                                                    loop  {
                                                        r =
                                                            match_at(reg, str,
                                                                     end,
                                                                     orig_range,
                                                                     s, prev,
                                                                     &mut msa);
                                                        if r !=
                                                               -(1 as
                                                                     libc::c_int)
                                                           {
                                                            if !(r >=
                                                                     0 as
                                                                         libc::c_int)
                                                               {
                                                                current_block
                                                                    =
                                                                    4824798144739327838;
                                                                break ;
                                                            }
                                                            if (*reg).options
                                                                   &
                                                                   ((((1 as
                                                                           libc::c_uint)
                                                                          <<
                                                                          1 as
                                                                              libc::c_int)
                                                                         <<
                                                                         1 as
                                                                             libc::c_int)
                                                                        <<
                                                                        1 as
                                                                            libc::c_int)
                                                                       <<
                                                                       1 as
                                                                           libc::c_int
                                                                   == 0 {
                                                                current_block
                                                                    =
                                                                    14253232777612267885;
                                                                break ;
                                                            }
                                                        }
                                                        prev = s;
                                                        s =
                                                            s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                                                         as
                                                                         isize);
                                                        if (*reg).anchor &
                                                               ((1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    12 as
                                                                        libc::c_int
                                                                    |
                                                                    (1 as
                                                                         libc::c_int)
                                                                        <<
                                                                        11 as
                                                                            libc::c_int)
                                                               ==
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            while (*(*reg).enc).is_mbc_newline.expect("non-null function pointer")(prev,
                                                                                                                                   end)
                                                                      == 0 &&
                                                                      s <
                                                                          range
                                                                              as
                                                                              *mut OnigUChar
                                                                  {
                                                                prev = s;
                                                                s =
                                                                    s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                                                                 as
                                                                                 isize)
                                                            }
                                                        }
                                                        if !(s <
                                                                 range as
                                                                     *mut OnigUChar)
                                                           {
                                                            current_block =
                                                                14514155837299642798;
                                                            break ;
                                                        }
                                                    }
                                                } else {
                                                    current_block =
                                                        5710330377809666066;
                                                }
                                            } else {
                                                current_block =
                                                    5710330377809666066;
                                            }
                                            match current_block {
                                                14514155837299642798 => { }
                                                4824798144739327838 => { }
                                                14253232777612267885 => { }
                                                _ => {
                                                    loop  {
                                                        r =
                                                            match_at(reg, str,
                                                                     end,
                                                                     orig_range,
                                                                     s, prev,
                                                                     &mut msa);
                                                        if r !=
                                                               -(1 as
                                                                     libc::c_int)
                                                           {
                                                            if !(r >=
                                                                     0 as
                                                                         libc::c_int)
                                                               {
                                                                current_block
                                                                    =
                                                                    4824798144739327838;
                                                                break ;
                                                            }
                                                            if (*reg).options
                                                                   &
                                                                   ((((1 as
                                                                           libc::c_uint)
                                                                          <<
                                                                          1 as
                                                                              libc::c_int)
                                                                         <<
                                                                         1 as
                                                                             libc::c_int)
                                                                        <<
                                                                        1 as
                                                                            libc::c_int)
                                                                       <<
                                                                       1 as
                                                                           libc::c_int
                                                                   == 0 {
                                                                current_block
                                                                    =
                                                                    14253232777612267885;
                                                                break ;
                                                            }
                                                        }
                                                        prev = s;
                                                        s =
                                                            s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                                                         as
                                                                         isize);
                                                        if !(s <
                                                                 range as
                                                                     *mut OnigUChar)
                                                           {
                                                            current_block =
                                                                10265166559088591044;
                                                            break ;
                                                        }
                                                    }
                                                    match current_block {
                                                        4824798144739327838 =>
                                                        {
                                                        }
                                                        14253232777612267885
                                                        => {
                                                        }
                                                        _ => {
                                                            if s ==
                                                                   range as
                                                                       *mut OnigUChar
                                                               {
                                                                /* check only. */
                                                                /* because empty match with /$/. */
                                                                r =
                                                                    match_at(reg,
                                                                             str,
                                                                             end,
                                                                             orig_range,
                                                                             s,
                                                                             prev,
                                                                             &mut msa);
                                                                if r !=
                                                                       -(1 as
                                                                             libc::c_int)
                                                                   {
                                                                    if r >=
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        if (*reg).options
                                                                               &
                                                                               ((((1
                                                                                       as
                                                                                       libc::c_uint)
                                                                                      <<
                                                                                      1
                                                                                          as
                                                                                          libc::c_int)
                                                                                     <<
                                                                                     1
                                                                                         as
                                                                                         libc::c_int)
                                                                                    <<
                                                                                    1
                                                                                        as
                                                                                        libc::c_int)
                                                                                   <<
                                                                                   1
                                                                                       as
                                                                                       libc::c_int
                                                                               ==
                                                                               0
                                                                           {
                                                                            current_block
                                                                                =
                                                                                14253232777612267885;
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                14514155837299642798;
                                                                        }
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            4824798144739327838;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        14514155837299642798;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    14514155837299642798;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            /* backward search */
                                            if orig_start < end {
                                                orig_start =
                                                    orig_start.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(orig_start)
                                                                          as
                                                                          isize)
                                            } /* is upper range */
                                            if (*reg).optimize !=
                                                   0 as libc::c_int {
                                                let mut low_0:
                                                        *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                let mut high_0:
                                                        *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                let mut adjrange:
                                                        *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                let mut sch_start:
                                                        *mut OnigUChar =
                                                    0 as *mut OnigUChar;
                                                if range < end {
                                                    adjrange =
                                                        (*(*reg).enc).left_adjust_char_head.expect("non-null function pointer")(str,
                                                                                                                                range)
                                                } else {
                                                    adjrange =
                                                        end as *mut OnigUChar
                                                }
                                                if (*reg).dmax !=
                                                       !(0 as libc::c_int as
                                                             OnigLen) &&
                                                       end.wrapping_offset_from(range)
                                                           as libc::c_long >=
                                                           (*reg).threshold_len
                                                               as libc::c_long
                                                   {
                                                    's_859:
                                                        loop  {
                                                            sch_start =
                                                                s.offset((*reg).dmax
                                                                             as
                                                                             isize);
                                                            if sch_start >
                                                                   end as
                                                                       *mut OnigUChar
                                                               {
                                                                sch_start =
                                                                    end as
                                                                        *mut OnigUChar
                                                            }
                                                            if backward_search_range(reg,
                                                                                     str,
                                                                                     end,
                                                                                     sch_start,
                                                                                     range,
                                                                                     adjrange,
                                                                                     &mut low_0,
                                                                                     &mut high_0)
                                                                   <=
                                                                   0 as
                                                                       libc::c_int
                                                               {
                                                                current_block
                                                                    =
                                                                    14514155837299642798;
                                                                break ;
                                                            }
                                                            if s > high_0 {
                                                                s = high_0
                                                            }
                                                            while s >= low_0 {
                                                                prev =
                                                                    onigenc_get_prev_char_head((*reg).enc,
                                                                                               str,
                                                                                               s);
                                                                r =
                                                                    match_at(reg,
                                                                             str,
                                                                             end,
                                                                             orig_start,
                                                                             s,
                                                                             prev,
                                                                             &mut msa);
                                                                if r !=
                                                                       -(1 as
                                                                             libc::c_int)
                                                                   {
                                                                    if !(r >=
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                       {
                                                                        current_block
                                                                            =
                                                                            4824798144739327838;
                                                                        break
                                                                            's_859
                                                                            ;
                                                                    }
                                                                    if (*reg).options
                                                                           &
                                                                           ((((1
                                                                                   as
                                                                                   libc::c_uint)
                                                                                  <<
                                                                                  1
                                                                                      as
                                                                                      libc::c_int)
                                                                                 <<
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                <<
                                                                                1
                                                                                    as
                                                                                    libc::c_int)
                                                                               <<
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                           ==
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            14253232777612267885;
                                                                        break
                                                                            's_859
                                                                            ;
                                                                    }
                                                                }
                                                                s = prev
                                                            }
                                                            if !(s >=
                                                                     range as
                                                                         *mut OnigUChar)
                                                               {
                                                                current_block
                                                                    =
                                                                    14514155837299642798;
                                                                break ;
                                                            }
                                                        }
                                                } else if (end.wrapping_offset_from(range)
                                                               as
                                                               libc::c_long) <
                                                              (*reg).threshold_len
                                                                  as
                                                                  libc::c_long
                                                 {
                                                    current_block =
                                                        14514155837299642798;
                                                } else {
                                                    sch_start = s;
                                                    if (*reg).dmax !=
                                                           0 as libc::c_int as
                                                               libc::c_uint {
                                                        if (*reg).dmax ==
                                                               !(0 as
                                                                     libc::c_int
                                                                     as
                                                                     OnigLen)
                                                           {
                                                            sch_start =
                                                                end as
                                                                    *mut OnigUChar
                                                        } else {
                                                            sch_start =
                                                                sch_start.offset((*reg).dmax
                                                                                     as
                                                                                     isize);
                                                            if sch_start >
                                                                   end as
                                                                       *mut OnigUChar
                                                               {
                                                                sch_start =
                                                                    end as
                                                                        *mut OnigUChar
                                                            } else {
                                                                sch_start =
                                                                    (*(*reg).enc).left_adjust_char_head.expect("non-null function pointer")(start,
                                                                                                                                            sch_start)
                                                            }
                                                        }
                                                    }
                                                    if backward_search_range(reg,
                                                                             str,
                                                                             end,
                                                                             sch_start,
                                                                             range,
                                                                             adjrange,
                                                                             &mut low_0,
                                                                             &mut high_0)
                                                           <= 0 as libc::c_int
                                                       {
                                                        current_block =
                                                            14514155837299642798;
                                                    } else {
                                                        current_block =
                                                            16477797002856340645;
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    16477797002856340645;
                                            }
                                            match current_block {
                                                14514155837299642798 => { }
                                                4824798144739327838 => { }
                                                14253232777612267885 => { }
                                                _ => {
                                                    loop  {
                                                        prev =
                                                            onigenc_get_prev_char_head((*reg).enc,
                                                                                       str,
                                                                                       s);
                                                        r =
                                                            match_at(reg, str,
                                                                     end,
                                                                     orig_start,
                                                                     s, prev,
                                                                     &mut msa);
                                                        if r !=
                                                               -(1 as
                                                                     libc::c_int)
                                                           {
                                                            if !(r >=
                                                                     0 as
                                                                         libc::c_int)
                                                               {
                                                                current_block
                                                                    =
                                                                    4824798144739327838;
                                                                break ;
                                                            }
                                                            if (*reg).options
                                                                   &
                                                                   ((((1 as
                                                                           libc::c_uint)
                                                                          <<
                                                                          1 as
                                                                              libc::c_int)
                                                                         <<
                                                                         1 as
                                                                             libc::c_int)
                                                                        <<
                                                                        1 as
                                                                            libc::c_int)
                                                                       <<
                                                                       1 as
                                                                           libc::c_int
                                                                   == 0 {
                                                                current_block
                                                                    =
                                                                    14253232777612267885;
                                                                break ;
                                                            }
                                                        }
                                                        s = prev;
                                                        if !(s >=
                                                                 range as
                                                                     *mut OnigUChar)
                                                           {
                                                            current_block =
                                                                14514155837299642798;
                                                            break ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => { }
                                }
                                match current_block {
                                    14514155837299642798 => {
                                        if (*reg).options &
                                               ((((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int) <<
                                                   1 as libc::c_int != 0 {
                                            if msa.best_len >=
                                                   0 as libc::c_int {
                                                s = msa.best_s;
                                                current_block =
                                                    14253232777612267885;
                                            } else {
                                                current_block =
                                                    10818670573987138623;
                                            }
                                        } else {
                                            current_block =
                                                10818670573987138623;
                                        }
                                        match current_block {
                                            14253232777612267885 => { }
                                            _ => {
                                                r = -(1 as libc::c_int);
                                                current_block =
                                                    4824798144739327838;
                                            }
                                        }
                                    }
                                    _ => { }
                                }
                                match current_block {
                                    14253232777612267885 => {
                                        if !msa.stack_p.is_null() {
                                            free(msa.stack_p);
                                        }
                                        return s.wrapping_offset_from(str) as
                                                   libc::c_long as libc::c_int
                                    }
                                    _ => {
                                        if !msa.stack_p.is_null() {
                                            free(msa.stack_p);
                                        }
                                        /* check only. */
                                        /* If result is mismatch and no FIND_NOT_EMPTY option,
     then the region is not set in match_at(). */
                                        if (*reg).options &
                                               (((((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int) <<
                                                   1 as libc::c_int != 0 &&
                                               !region.is_null() &&
                                               option &
                                                   (((((((((((1 as
                                                                  libc::c_uint)
                                                                 <<
                                                                 1 as
                                                                     libc::c_int)
                                                                <<
                                                                1 as
                                                                    libc::c_int)
                                                               <<
                                                               1 as
                                                                   libc::c_int)
                                                              <<
                                                              1 as
                                                                  libc::c_int)
                                                             <<
                                                             1 as libc::c_int)
                                                            <<
                                                            1 as libc::c_int)
                                                           <<
                                                           1 as libc::c_int)
                                                          << 1 as libc::c_int)
                                                         << 1 as libc::c_int)
                                                        << 1 as libc::c_int)
                                                       << 1 as libc::c_int ==
                                                   0 {
                                            onig_region_clear(region);
                                        }
                                        return r
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                1867614760774316006 => { }
                _ => { r = -(1 as libc::c_int) }
            }
        }
        _ => { }
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn onig_scan(mut reg: *mut regex_t,
                                   mut str: *const OnigUChar,
                                   mut end: *const OnigUChar,
                                   mut region: *mut OnigRegion,
                                   mut option: OnigOptionType,
                                   mut scan_callback:
                                       Option<unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut OnigRegion,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
                                   mut callback_arg: *mut libc::c_void)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut rs: libc::c_int = 0;
    let mut start: *const OnigUChar = 0 as *const OnigUChar;
    if option &
           ((((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                         1 as libc::c_int) << 1 as libc::c_int) <<
                       1 as libc::c_int) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int != 0 {
        if (*(*reg).enc).is_valid_mbc_string.expect("non-null function pointer")(str,
                                                                                 end)
               == 0 {
            return -(400 as libc::c_int)
        }
        option &=
            !(((((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                            1 as libc::c_int) << 1 as libc::c_int) <<
                          1 as libc::c_int) << 1 as libc::c_int) <<
                        1 as libc::c_int) << 1 as libc::c_int) <<
                      1 as libc::c_int) << 1 as libc::c_int) <<
                    1 as libc::c_int) << 1 as libc::c_int) <<
                  1 as libc::c_int)
    }
    n = 0 as libc::c_int;
    start = str;
    loop  {
        r = onig_search(reg, str, end, start, end, region, option);
        if r >= 0 as libc::c_int {
            rs =
                scan_callback.expect("non-null function pointer")(n, r,
                                                                  region,
                                                                  callback_arg);
            n += 1;
            if rs != 0 as libc::c_int { return rs }
            if *(*region).end.offset(0 as libc::c_int as isize) as
                   libc::c_long ==
                   start.wrapping_offset_from(str) as libc::c_long {
                start = start.offset(1)
            } else {
                start =
                    str.offset(*(*region).end.offset(0 as libc::c_int as
                                                         isize) as isize)
            }
            if start > end { break ; }
        } else {
            if r == -(1 as libc::c_int) { break ; }
            /* error */
            return r
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_encoding(mut reg: *mut regex_t)
 -> OnigEncoding {
    return (*reg).enc;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_options(mut reg: *mut regex_t)
 -> OnigOptionType {
    return (*reg).options;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_case_fold_flag(mut reg: *mut regex_t)
 -> OnigCaseFoldType {
    return (*reg).case_fold_flag;
}
#[no_mangle]
pub unsafe extern "C" fn onig_get_syntax(mut reg: *mut regex_t)
 -> *mut OnigSyntaxType {
    return (*reg).syntax;
}
#[no_mangle]
pub unsafe extern "C" fn onig_number_of_captures(mut reg: *mut regex_t)
 -> libc::c_int {
    return (*reg).num_mem;
}
#[no_mangle]
pub unsafe extern "C" fn onig_number_of_capture_histories(mut reg:
                                                              *mut regex_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= 31 as libc::c_int {
        if (if i <
                   (::std::mem::size_of::<BitStatusType>() as
                        libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                        libc::c_ulong) as
                       libc::c_int {
                ((*reg).capture_history) &
                    ((1 as libc::c_int) << i) as libc::c_uint
            } else {
                ((*reg).capture_history) & 1 as libc::c_int as libc::c_uint
            }) != 0 as libc::c_int as libc::c_uint {
            n += 1
        }
        i += 1
    }
    return n;
}
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
pub unsafe extern "C" fn onig_copy_encoding(mut to: OnigEncoding,
                                            mut from: OnigEncoding) {
    *to = *from;
}
