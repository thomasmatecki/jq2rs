#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           label_break_value, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type st_table_entry;
    #[no_mangle]
    fn onigenc_get_prev_char_head(enc: OnigEncoding, start: *const OnigUChar,
                                  s: *const OnigUChar) -> *mut OnigUChar;
    #[no_mangle]
    fn onigenc_strlen(enc: OnigEncoding, p: *const OnigUChar,
                      end: *const OnigUChar) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn onig_snprintf_with_pattern(buf: *mut OnigUChar, bufsize: libc::c_int,
                                  enc: OnigEncoding, pat: *mut OnigUChar,
                                  pat_end: *mut OnigUChar,
                                  fmt: *const OnigUChar, _: ...);
    #[no_mangle]
    fn onig_bbuf_init(buf: *mut BBuf, size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn onig_is_code_in_cc(enc: OnigEncoding, code: OnigCodePoint,
                          cc: *mut CClassNode) -> libc::c_int;
    #[no_mangle]
    fn onigenc_with_ascii_strncmp(enc: OnigEncoding, p: *const OnigUChar,
                                  end: *const OnigUChar,
                                  sascii: *const OnigUChar, n: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn onigenc_step(enc: OnigEncoding, p: *const OnigUChar,
                    end: *const OnigUChar, n: libc::c_int) -> *mut OnigUChar;
    #[no_mangle]
    fn onig_st_init_table_with_size(_: *mut st_hash_type, _: libc::c_int)
     -> *mut st_table;
    #[no_mangle]
    fn onig_st_insert(_: *mut st_table, _: st_data_t, _: st_data_t)
     -> libc::c_int;
    #[no_mangle]
    fn onig_st_lookup(_: *mut st_table, _: st_data_t, _: *mut st_data_t)
     -> libc::c_int;
    #[no_mangle]
    fn onig_st_foreach(_: *mut st_table,
                       _: Option<unsafe extern "C" fn() -> libc::c_int>,
                       _: st_data_t) -> libc::c_int;
    #[no_mangle]
    fn onig_st_free_table(_: *mut st_table);
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
/* constants */
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
/* errors related to thread */
/* #define ONIGERR_OVER_THREAD_PASS_LIMIT_COUNT                -1001 */
/* must be smaller than BIT_STATUS_BITS_NUM (unsigned int * 8) */
pub type OnigCaptureTreeNode = OnigCaptureTreeNodeStruct;
/* group number */
/* match result region type */
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
pub type OnigWarnFunc
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>;
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
pub struct NameEntry {
    pub name: *mut OnigUChar,
    pub name_len: libc::c_int,
    pub back_num: libc::c_int,
    pub back_alloc: libc::c_int,
    pub back_ref1: libc::c_int,
    pub back_refs: *mut libc::c_int,
}
pub type HashDataType = st_data_t;
/* escape compile-conflict */
/* This is a public domain general purpose hash table package written by Peter Moore @ UCB. */
/* @(#) st.h 5.1 89/12/14 */
pub type st_data_t = libc::c_ulong;
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
/* strend hash */
pub type hash_table_type = ();
pub type NameTable = st_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_table {
    pub type_0: *mut st_hash_type,
    pub num_bins: libc::c_int,
    pub num_entries: libc::c_int,
    pub bins: *mut *mut st_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_hash_type {
    pub compare: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub hash: Option<unsafe extern "C" fn() -> libc::c_int>,
}
pub type hash_data_type = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_str_end_key {
    pub s: *mut OnigUChar,
    pub end: *mut OnigUChar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INamesArg {
    pub func: Option<unsafe extern "C" fn(_: *const OnigUChar,
                                          _: *const OnigUChar, _: libc::c_int,
                                          _: *mut libc::c_int,
                                          _: *mut regex_t,
                                          _: *mut libc::c_void)
                         -> libc::c_int>,
    pub reg: *mut regex_t,
    pub arg: *mut libc::c_void,
    pub ret: libc::c_int,
    pub enc: OnigEncoding,
}
pub const ST_CONTINUE: st_retval = 0;
pub const ST_STOP: st_retval = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosixBracketEntryType {
    pub name: *mut OnigUChar,
    pub ctype: libc::c_int,
    pub len: libc::c_short,
}
pub type BitStatusType = libc::c_uint;
pub type Bits = libc::c_uint;
pub type BitSet = [Bits; 8];
pub type BitSetRef = *mut Bits;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BBuf {
    pub p: *mut OnigUChar,
    pub used: libc::c_uint,
    pub alloc: libc::c_uint,
}
pub type BBuf = _BBuf;
pub type AbsAddrType = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeBase {
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClassNode {
    pub base: NodeBase,
    pub flags: libc::c_uint,
    pub bs: BitSet,
    pub mbuf: *mut BBuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrNode {
    pub base: NodeBase,
    pub s: *mut OnigUChar,
    pub end: *mut OnigUChar,
    pub flag: libc::c_uint,
    pub capa: libc::c_int,
    pub buf: [OnigUChar; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QtfrNode {
    pub base: NodeBase,
    pub state: libc::c_int,
    pub target: *mut _Node,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
    pub greedy: libc::c_int,
    pub target_empty_info: libc::c_int,
    pub head_exact: *mut _Node,
    pub next_head_exact: *mut _Node,
    pub is_refered: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Node {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub base: NodeBase,
    pub str_0: StrNode,
    pub cclass: CClassNode,
    pub qtfr: QtfrNode,
    pub enclose: EncloseNode,
    pub bref: BRefNode,
    pub anchor: AnchorNode,
    pub cons: ConsAltNode,
    pub ctype: CtypeNode,
    pub call: CallNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallNode {
    pub base: NodeBase,
    pub state: libc::c_int,
    pub group_num: libc::c_int,
    pub name: *mut OnigUChar,
    pub name_end: *mut OnigUChar,
    pub target: *mut _Node,
    pub unset_addr_list: *mut UnsetAddrList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnsetAddrList {
    pub num: libc::c_int,
    pub alloc: libc::c_int,
    pub us: *mut UnsetAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnsetAddr {
    pub offset: libc::c_int,
    pub target: *mut _Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CtypeNode {
    pub base: NodeBase,
    pub ctype: libc::c_int,
    pub not: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConsAltNode {
    pub base: NodeBase,
    pub car: *mut _Node,
    pub cdr: *mut _Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnchorNode {
    pub base: NodeBase,
    pub type_0: libc::c_int,
    pub target: *mut _Node,
    pub char_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BRefNode {
    pub base: NodeBase,
    pub state: libc::c_int,
    pub back_num: libc::c_int,
    pub back_static: [libc::c_int; 6],
    pub back_dynamic: *mut libc::c_int,
    pub nest_level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncloseNode {
    pub base: NodeBase,
    pub state: libc::c_int,
    pub type_0: libc::c_int,
    pub regnum: libc::c_int,
    pub option: OnigOptionType,
    pub target: *mut _Node,
    pub call_addr: AbsAddrType,
    pub min_len: OnigLen,
    pub max_len: OnigLen,
    pub char_len: libc::c_int,
    pub opt_count: libc::c_int,
}
pub type Node = _Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScanEnv {
    pub option: OnigOptionType,
    pub case_fold_flag: OnigCaseFoldType,
    pub enc: OnigEncoding,
    pub syntax: *mut OnigSyntaxType,
    pub capture_history: BitStatusType,
    pub bt_mem_start: BitStatusType,
    pub bt_mem_end: BitStatusType,
    pub backrefed_mem: BitStatusType,
    pub pattern: *mut OnigUChar,
    pub pattern_end: *mut OnigUChar,
    pub error: *mut OnigUChar,
    pub error_end: *mut OnigUChar,
    pub reg: *mut regex_t,
    pub num_call: libc::c_int,
    pub unset_addr_list: *mut UnsetAddrList,
    pub num_mem: libc::c_int,
    pub num_named: libc::c_int,
    pub mem_alloc: libc::c_int,
    pub mem_nodes_static: [*mut Node; 8],
    pub mem_nodes_dynamic: *mut *mut Node,
    pub parse_depth: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupNumRemap {
    pub new_val: libc::c_int,
}
pub const RQ_ASIS: ReduceType = 0;
/* to '+)??' */
pub const RQ_PQ_Q: ReduceType = 6;
/* to '??'   */
pub const RQ_P_QQ: ReduceType = 5;
/* to '*?'   */
pub const RQ_QQ: ReduceType = 4;
/* to '*'    */
pub const RQ_AQ: ReduceType = 3;
/* delete parent */
pub const RQ_A: ReduceType = 2;
/* as is */
pub const RQ_DEL: ReduceType = 1;
pub type ReduceType = libc::c_uint;
pub const ST_DELETE: st_retval = 2;
pub const TK_EOT: TokenSyms = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigToken {
    pub type_0: TokenSyms,
    pub escaped: libc::c_int,
    pub base: libc::c_int,
    pub backp: *mut OnigUChar,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: *mut OnigUChar,
    pub c: libc::c_int,
    pub code: OnigCodePoint,
    pub anchor: libc::c_int,
    pub subtype: libc::c_int,
    pub repeat: C2RustUnnamed_4,
    pub backref: C2RustUnnamed_3,
    pub call: C2RustUnnamed_2,
    pub prop: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ctype: libc::c_int,
    pub not: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *mut OnigUChar,
    pub name_end: *mut OnigUChar,
    pub gnum: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub num: libc::c_int,
    pub ref1: libc::c_int,
    pub refs: *mut libc::c_int,
    pub by_name: libc::c_int,
    pub exist_level: libc::c_int,
    pub level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub lower: libc::c_int,
    pub upper: libc::c_int,
    pub greedy: libc::c_int,
    pub possessive: libc::c_int,
}
pub type TokenSyms = libc::c_uint;
/* to '+?)?' */
/* [ */
/* && */
pub const TK_CC_CC_OPEN: TokenSyms = 23;
pub const TK_CC_AND: TokenSyms = 22;
pub const TK_POSIX_BRACKET_OPEN: TokenSyms = 21;
pub const TK_CC_RANGE: TokenSyms = 20;
/* \p{...}, \P{...} */
/* in cc */
pub const TK_CC_CLOSE: TokenSyms = 19;
pub const TK_CHAR_PROPERTY: TokenSyms = 18;
pub const TK_QUOTE_OPEN: TokenSyms = 17;
pub const TK_CC_OPEN: TokenSyms = 16;
pub const TK_SUBEXP_CLOSE: TokenSyms = 15;
pub const TK_SUBEXP_OPEN: TokenSyms = 14;
/* SQL '%' == .* */
pub const TK_ALT: TokenSyms = 13;
pub const TK_ANYCHAR_ANYTIME: TokenSyms = 12;
pub const TK_INTERVAL: TokenSyms = 11;
pub const TK_OP_REPEAT: TokenSyms = 10;
pub const TK_ANCHOR: TokenSyms = 9;
pub const TK_CALL: TokenSyms = 8;
pub const TK_BACKREF: TokenSyms = 7;
pub const TK_CHAR_TYPE: TokenSyms = 6;
pub const TK_ANYCHAR: TokenSyms = 5;
pub const TK_CODE_POINT: TokenSyms = 4;
pub const TK_STRING: TokenSyms = 3;
pub const TK_CHAR: TokenSyms = 2;
/* end of token */
pub const TK_RAW_BYTE: TokenSyms = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IApplyCaseFoldArg {
    pub env: *mut ScanEnv,
    pub cc: *mut CClassNode,
    pub alt_root: *mut Node,
    pub ptail: *mut *mut Node,
}
pub type CCSTATE = libc::c_uint;
pub const CCS_START: CCSTATE = 3;
pub const CCS_COMPLETE: CCSTATE = 2;
pub const CCS_RANGE: CCSTATE = 1;
pub const CCS_VALUE: CCSTATE = 0;
pub type CCVALTYPE = libc::c_uint;
pub const CCV_CLASS: CCVALTYPE = 2;
pub const CCV_CODE_POINT: CCVALTYPE = 1;
pub const CCV_SB: CCVALTYPE = 0;
pub type st_retval = libc::c_uint;
pub const ST_CHECK: st_retval = 3;
#[no_mangle]
pub static mut OnigSyntaxRuby: OnigSyntaxType =
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
                               (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 3 as libc::c_int |
                                   (1 as libc::c_uint) << 7 as libc::c_int |
                                   (1 as libc::c_uint) << 8 as libc::c_int |
                                   (1 as libc::c_uint) << 9 as libc::c_int |
                                   (1 as libc::c_uint) << 16 as libc::c_int |
                                   (1 as libc::c_uint) << 17 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 11 as libc::c_int |
                                   (1 as libc::c_uint) << 12 as libc::c_int |
                                   (1 as libc::c_uint) << 13 as libc::c_int |
                                   (1 as libc::c_uint) << 19 as libc::c_int,
                           behavior:
                               (1 as libc::c_uint) << 31 as libc::c_int |
                                   (1 as libc::c_uint) << 0 as libc::c_int |
                                   (1 as libc::c_uint) << 1 as libc::c_int |
                                   (1 as libc::c_uint) << 3 as libc::c_int |
                                   (1 as libc::c_uint) << 21 as libc::c_int |
                                   (1 as libc::c_uint) << 23 as libc::c_int |
                                   (1 as libc::c_uint) << 4 as libc::c_int |
                                   (1 as libc::c_uint) << 6 as libc::c_int |
                                   (1 as libc::c_uint) << 7 as libc::c_int |
                                   (1 as libc::c_uint) << 8 as libc::c_int |
                                   (1 as libc::c_uint) << 9 as libc::c_int |
                                   (1 as libc::c_uint) << 24 as libc::c_int |
                                   (1 as libc::c_uint) << 25 as libc::c_int,
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
pub static mut OnigDefaultSyntax: *mut OnigSyntaxType =
    unsafe {
        &OnigSyntaxRuby as *const OnigSyntaxType as *mut OnigSyntaxType
    };
#[no_mangle]
pub unsafe extern "C" fn onig_null_warn(mut s: *const libc::c_char) { }
static mut onig_warn: OnigWarnFunc =
    unsafe {
        Some(onig_null_warn as
                 unsafe extern "C" fn(_: *const libc::c_char) -> ())
    };
static mut onig_verb_warn: OnigWarnFunc =
    unsafe {
        Some(onig_null_warn as
                 unsafe extern "C" fn(_: *const libc::c_char) -> ())
    };
#[no_mangle]
pub unsafe extern "C" fn onig_set_warn_func(mut f: OnigWarnFunc) {
    onig_warn = f;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_verb_warn_func(mut f: OnigWarnFunc) {
    onig_verb_warn = f;
}
#[no_mangle]
pub unsafe extern "C" fn onig_warning(mut s: *const libc::c_char) {
    if onig_warn ==
           Some(onig_null_warn as
                    unsafe extern "C" fn(_: *const libc::c_char) -> ()) {
        return
    }
    Some(onig_warn.expect("non-null function pointer")).expect("non-null function pointer")(s);
}
static mut ParseDepthLimit: libc::c_uint =
    4096 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn onig_get_parse_depth_limit() -> libc::c_uint {
    return ParseDepthLimit;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_parse_depth_limit(mut depth: libc::c_uint)
 -> libc::c_int {
    if depth == 0 as libc::c_int as libc::c_uint {
        ParseDepthLimit = 4096 as libc::c_int as libc::c_uint
    } else { ParseDepthLimit = depth }
    return 0 as libc::c_int;
}
unsafe extern "C" fn bbuf_free(mut bbuf: *mut BBuf) {
    if !(bbuf as *mut libc::c_void).is_null() {
        if !((*bbuf).p as *mut libc::c_void).is_null() {
            free((*bbuf).p as *mut libc::c_void);
        }
        free(bbuf as *mut libc::c_void);
    };
}
unsafe extern "C" fn bbuf_clone(mut rto: *mut *mut BBuf, mut from: *mut BBuf)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut to: *mut BBuf = 0 as *mut BBuf;
    to = malloc(::std::mem::size_of::<BBuf>() as libc::c_ulong) as *mut BBuf;
    *rto = to;
    if (to as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    r = onig_bbuf_init(to, (*from).alloc as libc::c_int);
    if r != 0 as libc::c_int { return r }
    (*to).used = (*from).used;
    memcpy((*to).p as *mut libc::c_void, (*from).p as *const libc::c_void,
           (*from).used as libc::c_ulong);
    return 0 as libc::c_int;
}
unsafe extern "C" fn bitset_set_range(mut bs: BitSetRef,
                                      mut from: libc::c_int,
                                      mut to: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = from;
    while i <= to && i < (1 as libc::c_int) << 8 as libc::c_int {
        let ref mut fresh0 =
            *bs.offset((i as
                            libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                           as isize);
        *fresh0 |=
            ((1 as libc::c_int) <<
                 (i as
                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                       as
                                                       libc::c_ulong).wrapping_mul(8
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)))
                as libc::c_uint;
        i += 1
    };
}
unsafe extern "C" fn bitset_invert(mut bs: BitSetRef) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (((1 as libc::c_int) << 8 as libc::c_int) as
                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_int {
        *bs.offset(i as isize) = !*bs.offset(i as isize);
        i += 1
    };
}
unsafe extern "C" fn bitset_invert_to(mut from: BitSetRef,
                                      mut to: BitSetRef) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (((1 as libc::c_int) << 8 as libc::c_int) as
                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_int {
        *to.offset(i as isize) = !*from.offset(i as isize);
        i += 1
    };
}
unsafe extern "C" fn bitset_and(mut dest: BitSetRef, mut bs: BitSetRef) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (((1 as libc::c_int) << 8 as libc::c_int) as
                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_int {
        let ref mut fresh1 = *dest.offset(i as isize);
        *fresh1 &= *bs.offset(i as isize);
        i += 1
    };
}
unsafe extern "C" fn bitset_or(mut dest: BitSetRef, mut bs: BitSetRef) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (((1 as libc::c_int) << 8 as libc::c_int) as
                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_int {
        let ref mut fresh2 = *dest.offset(i as isize);
        *fresh2 |= *bs.offset(i as isize);
        i += 1
    };
}
unsafe extern "C" fn bitset_copy(mut dest: BitSetRef, mut bs: BitSetRef) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (((1 as libc::c_int) << 8 as libc::c_int) as
                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_int {
        *dest.offset(i as isize) = *bs.offset(i as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn onig_strncmp(mut s1: *const OnigUChar,
                                      mut s2: *const OnigUChar,
                                      mut n: libc::c_int) -> libc::c_int {
    let mut x: libc::c_int = 0;
    loop  {
        let fresh3 = n;
        n = n - 1;
        if !(fresh3 > 0 as libc::c_int) { break ; }
        let fresh4 = s2;
        s2 = s2.offset(1);
        let fresh5 = s1;
        s1 = s1.offset(1);
        x = *fresh4 as libc::c_int - *fresh5 as libc::c_int;
        if x != 0 { return x }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_strcpy(mut dest: *mut OnigUChar,
                                     mut src: *const OnigUChar,
                                     mut end: *const OnigUChar) {
    let mut len: libc::c_int =
        end.wrapping_offset_from(src) as libc::c_long as libc::c_int;
    if len > 0 as libc::c_int {
        memcpy(dest as *mut libc::c_void, src as *const libc::c_void,
               len as libc::c_ulong);
        *dest.offset(len as isize) = 0 as libc::c_int as OnigUChar
    };
}
unsafe extern "C" fn strdup_with_null(mut enc: OnigEncoding,
                                      mut s: *mut OnigUChar,
                                      mut end: *mut OnigUChar)
 -> *mut OnigUChar {
    let mut slen: libc::c_int = 0;
    let mut term_len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: *mut OnigUChar = 0 as *mut OnigUChar;
    slen = end.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    term_len = (*enc).min_enc_len;
    r = malloc((slen + term_len) as libc::c_ulong) as *mut OnigUChar;
    if (r as *mut libc::c_void).is_null() { return 0 as *mut OnigUChar }
    memcpy(r as *mut libc::c_void, s as *const libc::c_void,
           slen as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < term_len {
        *r.offset((slen + i) as isize) = 0 as libc::c_int as OnigUChar;
        i += 1
    }
    return r;
}
unsafe extern "C" fn strcat_capa(mut dest: *mut OnigUChar,
                                 mut dest_end: *mut OnigUChar,
                                 mut src: *const OnigUChar,
                                 mut src_end: *const OnigUChar,
                                 mut capa: libc::c_int) -> *mut OnigUChar {
    let mut r: *mut OnigUChar = 0 as *mut OnigUChar;
    if !dest.is_null() {
        r =
            realloc(dest as *mut libc::c_void,
                    (capa + 1 as libc::c_int) as libc::c_ulong) as
                *mut OnigUChar
    } else {
        r =
            malloc((capa + 1 as libc::c_int) as libc::c_ulong) as
                *mut OnigUChar
    }
    if (r as *mut libc::c_void).is_null() { return 0 as *mut OnigUChar }
    onig_strcpy(r.offset(dest_end.wrapping_offset_from(dest) as libc::c_long
                             as isize), src, src_end);
    return r;
}
/* dest on static area */
unsafe extern "C" fn strcat_capa_from_static(mut dest: *mut OnigUChar,
                                             mut dest_end: *mut OnigUChar,
                                             mut src: *const OnigUChar,
                                             mut src_end: *const OnigUChar,
                                             mut capa: libc::c_int)
 -> *mut OnigUChar {
    let mut r: *mut OnigUChar = 0 as *mut OnigUChar;
    r = malloc((capa + 1 as libc::c_int) as libc::c_ulong) as *mut OnigUChar;
    if (r as *mut libc::c_void).is_null() { return 0 as *mut OnigUChar }
    onig_strcpy(r, dest, dest_end);
    onig_strcpy(r.offset(dest_end.wrapping_offset_from(dest) as libc::c_long
                             as isize), src, src_end);
    return r;
}
unsafe extern "C" fn str_end_cmp(mut x: *mut st_str_end_key,
                                 mut y: *mut st_str_end_key) -> libc::c_int {
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut c: libc::c_int = 0;
    if (*x).end.wrapping_offset_from((*x).s) as libc::c_long !=
           (*y).end.wrapping_offset_from((*y).s) as libc::c_long {
        return 1 as libc::c_int
    }
    p = (*x).s;
    q = (*y).s;
    while p < (*x).end {
        c = *p as libc::c_int - *q as libc::c_int;
        if c != 0 as libc::c_int { return c }
        p = p.offset(1);
        q = q.offset(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_end_hash(mut x: *mut st_str_end_key) -> libc::c_int {
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut val: libc::c_int = 0 as libc::c_int;
    p = (*x).s;
    while p < (*x).end {
        let fresh6 = p;
        p = p.offset(1);
        val = val * 997 as libc::c_int + *fresh6 as libc::c_int
    }
    return val + (val >> 5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_strend_table_with_size(mut size:
                                                                 libc::c_int)
 -> *mut libc::c_void {
    static mut hashType: st_hash_type =
        unsafe {
            {
                let mut init =
                    st_hash_type{compare:
                                     ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                             *mut st_str_end_key,
                                                                                         _:
                                                                                             *mut st_str_end_key)
                                                                        ->
                                                                            libc::c_int>,
                                                             Option<unsafe extern "C" fn()
                                                                        ->
                                                                            libc::c_int>>(Some(str_end_cmp
                                                                                                   as
                                                                                                   unsafe extern "C" fn(_:
                                                                                                                            *mut st_str_end_key,
                                                                                                                        _:
                                                                                                                            *mut st_str_end_key)
                                                                                                       ->
                                                                                                           libc::c_int)),
                                 hash:
                                     ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                             *mut st_str_end_key)
                                                                        ->
                                                                            libc::c_int>,
                                                             Option<unsafe extern "C" fn()
                                                                        ->
                                                                            libc::c_int>>(Some(str_end_hash
                                                                                                   as
                                                                                                   unsafe extern "C" fn(_:
                                                                                                                            *mut st_str_end_key)
                                                                                                       ->
                                                                                                           libc::c_int)),};
                init
            }
        };
    return onig_st_init_table_with_size(&mut hashType, size) as
               *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_lookup_strend(mut table: *mut libc::c_void,
                                               mut str_key: *const OnigUChar,
                                               mut end_key: *const OnigUChar,
                                               mut value: *mut hash_data_type)
 -> libc::c_int {
    let mut key: st_str_end_key =
        st_str_end_key{s: 0 as *mut OnigUChar, end: 0 as *mut OnigUChar,};
    key.s = str_key as *mut OnigUChar;
    key.end = end_key as *mut OnigUChar;
    return onig_st_lookup(table as *mut st_table,
                          &mut key as *mut st_str_end_key as st_data_t,
                          value);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_insert_strend(mut table: *mut libc::c_void,
                                               mut str_key: *const OnigUChar,
                                               mut end_key: *const OnigUChar,
                                               mut value: hash_data_type)
 -> libc::c_int {
    let mut key: *mut st_str_end_key = 0 as *mut st_str_end_key;
    let mut result: libc::c_int = 0;
    key =
        malloc(::std::mem::size_of::<st_str_end_key>() as libc::c_ulong) as
            *mut st_str_end_key;
    (*key).s = str_key as *mut OnigUChar;
    (*key).end = end_key as *mut OnigUChar;
    result = onig_st_insert(table as *mut st_table, key as st_data_t, value);
    if result != 0 { free(key as *mut libc::c_void); }
    return result;
}
/* 1.6 st.h doesn't define st_data_t type */
/* ONIG_DEBUG */
unsafe extern "C" fn i_free_name_entry(mut key: *mut OnigUChar,
                                       mut e: *mut NameEntry,
                                       mut arg: *mut libc::c_void)
 -> libc::c_int {
    free((*e).name as *mut libc::c_void); /* should be pattern encoding. */
    if !((*e).back_refs as *mut libc::c_void).is_null() {
        free((*e).back_refs as *mut libc::c_void);
    }
    free(key as *mut libc::c_void);
    free(e as *mut libc::c_void);
    return ST_DELETE as libc::c_int;
}
unsafe extern "C" fn names_clear(mut reg: *mut regex_t) -> libc::c_int {
    let mut t: *mut NameTable = (*reg).name_table as *mut NameTable;
    if !(t as *mut libc::c_void).is_null() {
        onig_st_foreach(t,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut OnigUChar,
                                                                            _:
                                                                                *mut NameEntry,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(i_free_name_entry
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut OnigUChar,
                                                                                                           _:
                                                                                                               *mut NameEntry,
                                                                                                           _:
                                                                                                               *mut libc::c_void)
                                                                                          ->
                                                                                              libc::c_int)),
                        0 as libc::c_int as st_data_t);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_names_free(mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut t: *mut NameTable = 0 as *mut NameTable;
    r = names_clear(reg);
    if r != 0 { return r }
    t = (*reg).name_table as *mut NameTable;
    if !(t as *mut libc::c_void).is_null() { onig_st_free_table(t); }
    (*reg).name_table = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn name_find(mut reg: *mut regex_t,
                               mut name: *const OnigUChar,
                               mut name_end: *const OnigUChar)
 -> *mut NameEntry {
    let mut e: *mut NameEntry = 0 as *mut NameEntry;
    let mut t: *mut NameTable = (*reg).name_table as *mut NameTable;
    e = 0 as *mut libc::c_void as *mut NameEntry;
    if !(t as *mut libc::c_void).is_null() {
        onig_st_lookup_strend(t as *mut libc::c_void, name, name_end,
                              &mut e as *mut *mut NameEntry as
                                  *mut libc::c_void as *mut HashDataType);
    }
    return e;
}
unsafe extern "C" fn i_names(mut key: *mut OnigUChar, mut e: *mut NameEntry,
                             mut arg: *mut INamesArg) -> libc::c_int {
    let mut r: libc::c_int =
        Some((*arg).func.expect("non-null function pointer")).expect("non-null function pointer")((*e).name,
                                                                                                  (*e).name.offset((*e).name_len
                                                                                                                       as
                                                                                                                       isize),
                                                                                                  (*e).back_num,
                                                                                                  if (*e).back_num
                                                                                                         >
                                                                                                         1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     {
                                                                                                      (*e).back_refs
                                                                                                  } else {
                                                                                                      &mut (*e).back_ref1
                                                                                                  },
                                                                                                  (*arg).reg,
                                                                                                  (*arg).arg);
    if r != 0 as libc::c_int { (*arg).ret = r; return ST_STOP as libc::c_int }
    return ST_CONTINUE as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_foreach_name(mut reg: *mut regex_t,
                                           mut func:
                                               Option<unsafe extern "C" fn(_:
                                                                               *const OnigUChar,
                                                                           _:
                                                                               *const OnigUChar,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               *mut libc::c_int,
                                                                           _:
                                                                               *mut regex_t,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> libc::c_int>,
                                           mut arg: *mut libc::c_void)
 -> libc::c_int {
    let mut narg: INamesArg =
        INamesArg{func: None,
                  reg: 0 as *mut regex_t,
                  arg: 0 as *mut libc::c_void,
                  ret: 0,
                  enc: 0 as *mut OnigEncodingType,};
    let mut t: *mut NameTable = (*reg).name_table as *mut NameTable;
    narg.ret = 0 as libc::c_int;
    if !(t as *mut libc::c_void).is_null() {
        narg.func = func;
        narg.reg = reg;
        narg.arg = arg;
        narg.enc = (*reg).enc;
        onig_st_foreach(t,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut OnigUChar,
                                                                            _:
                                                                                *mut NameEntry,
                                                                            _:
                                                                                *mut INamesArg)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(i_names
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut OnigUChar,
                                                                                                           _:
                                                                                                               *mut NameEntry,
                                                                                                           _:
                                                                                                               *mut INamesArg)
                                                                                          ->
                                                                                              libc::c_int)),
                        &mut narg as *mut INamesArg as HashDataType);
    }
    return narg.ret;
}
unsafe extern "C" fn i_renumber_name(mut key: *mut OnigUChar,
                                     mut e: *mut NameEntry,
                                     mut map: *mut GroupNumRemap)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*e).back_num > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*e).back_num {
            *(*e).back_refs.offset(i as isize) =
                (*map.offset(*(*e).back_refs.offset(i as isize) as
                                 isize)).new_val;
            i += 1
        }
    } else if (*e).back_num == 1 as libc::c_int {
        (*e).back_ref1 = (*map.offset((*e).back_ref1 as isize)).new_val
    }
    return ST_CONTINUE as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_renumber_name_table(mut reg: *mut regex_t,
                                                  mut map: *mut GroupNumRemap)
 -> libc::c_int {
    let mut t: *mut NameTable = (*reg).name_table as *mut NameTable;
    if !(t as *mut libc::c_void).is_null() {
        onig_st_foreach(t,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut OnigUChar,
                                                                            _:
                                                                                *mut NameEntry,
                                                                            _:
                                                                                *mut GroupNumRemap)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(i_renumber_name
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut OnigUChar,
                                                                                                           _:
                                                                                                               *mut NameEntry,
                                                                                                           _:
                                                                                                               *mut GroupNumRemap)
                                                                                          ->
                                                                                              libc::c_int)),
                        map as HashDataType);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_number_of_names(mut reg: *mut regex_t)
 -> libc::c_int {
    let mut t: *mut NameTable = (*reg).name_table as *mut NameTable;
    if !(t as *mut libc::c_void).is_null() {
        return (*t).num_entries
    } else { return 0 as libc::c_int };
}
/* USE_ST_LIBRARY */
/* else USE_ST_LIBRARY */
unsafe extern "C" fn name_add(mut reg: *mut regex_t, mut name: *mut OnigUChar,
                              mut name_end: *mut OnigUChar,
                              mut backref: libc::c_int, mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut alloc: libc::c_int = 0;
    let mut e: *mut NameEntry = 0 as *mut NameEntry;
    let mut t: *mut NameTable = (*reg).name_table as *mut NameTable;
    if name_end.wrapping_offset_from(name) as libc::c_long <=
           0 as libc::c_int as libc::c_long {
        return -(214 as libc::c_int)
    }
    e = name_find(reg, name, name_end);
    if (e as *mut libc::c_void).is_null() {
        if (t as *mut libc::c_void).is_null() {
            t =
                onig_st_init_strend_table_with_size(5 as libc::c_int) as
                    *mut NameTable;
            (*reg).name_table = t as *mut libc::c_void
        }
        e =
            malloc(::std::mem::size_of::<NameEntry>() as libc::c_ulong) as
                *mut NameEntry;
        if (e as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
        (*e).name = strdup_with_null((*reg).enc, name, name_end);
        if ((*e).name as *mut libc::c_void).is_null() {
            free(e as *mut libc::c_void);
            return -(5 as libc::c_int)
        }
        onig_st_insert_strend(t as *mut libc::c_void, (*e).name,
                              (*e).name.offset(name_end.wrapping_offset_from(name)
                                                   as libc::c_long as isize),
                              e as HashDataType);
        (*e).name_len =
            name_end.wrapping_offset_from(name) as libc::c_long as
                libc::c_int;
        (*e).back_num = 0 as libc::c_int;
        (*e).back_alloc = 0 as libc::c_int;
        (*e).back_refs = 0 as *mut libc::c_void as *mut libc::c_int
    }
    if (*e).back_num >= 1 as libc::c_int &&
           !((*(*env).syntax).behavior &
                 (1 as libc::c_uint) << 8 as libc::c_int !=
                 0 as libc::c_int as libc::c_uint) {
        onig_scan_env_set_error_string(env, -(219 as libc::c_int), name,
                                       name_end);
        return -(219 as libc::c_int)
    }
    (*e).back_num += 1;
    if (*e).back_num == 1 as libc::c_int {
        (*e).back_ref1 = backref
    } else if (*e).back_num == 2 as libc::c_int {
        alloc = 8 as libc::c_int;
        (*e).back_refs =
            malloc((::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(alloc as libc::c_ulong))
                as *mut libc::c_int;
        if ((*e).back_refs as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        (*e).back_alloc = alloc;
        *(*e).back_refs.offset(0 as libc::c_int as isize) = (*e).back_ref1;
        *(*e).back_refs.offset(1 as libc::c_int as isize) = backref
    } else {
        if (*e).back_num > (*e).back_alloc {
            alloc = (*e).back_alloc * 2 as libc::c_int;
            (*e).back_refs =
                realloc((*e).back_refs as *mut libc::c_void,
                        (::std::mem::size_of::<libc::c_int>() as
                             libc::c_ulong).wrapping_mul(alloc as
                                                             libc::c_ulong))
                    as *mut libc::c_int;
            if ((*e).back_refs as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            (*e).back_alloc = alloc
        }
        *(*e).back_refs.offset(((*e).back_num - 1 as libc::c_int) as isize) =
            backref
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_name_to_group_numbers(mut reg: *mut regex_t,
                                                    mut name:
                                                        *const OnigUChar,
                                                    mut name_end:
                                                        *const OnigUChar,
                                                    mut nums:
                                                        *mut *mut libc::c_int)
 -> libc::c_int {
    let mut e: *mut NameEntry = name_find(reg, name, name_end);
    if (e as *mut libc::c_void).is_null() { return -(217 as libc::c_int) }
    match (*e).back_num {
        0 => { }
        1 => { *nums = &mut (*e).back_ref1 }
        _ => { *nums = (*e).back_refs }
    }
    return (*e).back_num;
}
#[no_mangle]
pub unsafe extern "C" fn onig_name_to_backref_number(mut reg: *mut regex_t,
                                                     mut name:
                                                         *const OnigUChar,
                                                     mut name_end:
                                                         *const OnigUChar,
                                                     mut region:
                                                         *mut OnigRegion)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nums: *mut libc::c_int = 0 as *mut libc::c_int;
    n = onig_name_to_group_numbers(reg, name, name_end, &mut nums);
    if n < 0 as libc::c_int {
        return n
    } else if n == 0 as libc::c_int {
        return -(11 as libc::c_int)
    } else if n == 1 as libc::c_int {
        return *nums.offset(0 as libc::c_int as isize)
    } else {
        if !(region as *mut libc::c_void).is_null() {
            i = n - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if *(*region).beg.offset(*nums.offset(i as isize) as isize) !=
                       -(1 as libc::c_int) {
                    return *nums.offset(i as isize)
                }
                i -= 1
            }
        }
        return *nums.offset((n - 1 as libc::c_int) as isize)
    };
}
/* USE_NAMED_GROUP */
/* else USE_NAMED_GROUP */
#[no_mangle]
pub unsafe extern "C" fn onig_noname_group_capture_is_active(mut reg:
                                                                 *mut regex_t)
 -> libc::c_int {
    if (*reg).options &
           (((((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                   << 1 as libc::c_int) << 1 as libc::c_int) <<
                 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int !=
           0 {
        return 0 as libc::c_int
    }
    if onig_number_of_names(reg) > 0 as libc::c_int &&
           (*(*reg).syntax).behavior & (1 as libc::c_uint) << 7 as libc::c_int
               != 0 as libc::c_int as libc::c_uint &&
           (*reg).options &
               ((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                         1 as libc::c_int) << 1 as libc::c_int) <<
                       1 as libc::c_int) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn scan_env_clear(mut env: *mut ScanEnv) {
    let mut i: libc::c_int = 0;
    (*env).capture_history = 0 as libc::c_int as BitStatusType;
    (*env).bt_mem_start = 0 as libc::c_int as BitStatusType;
    (*env).bt_mem_end = 0 as libc::c_int as BitStatusType;
    (*env).backrefed_mem = 0 as libc::c_int as BitStatusType;
    (*env).error = 0 as *mut libc::c_void as *mut OnigUChar;
    (*env).error_end = 0 as *mut libc::c_void as *mut OnigUChar;
    (*env).num_call = 0 as libc::c_int;
    (*env).num_mem = 0 as libc::c_int;
    (*env).num_named = 0 as libc::c_int;
    (*env).mem_alloc = 0 as libc::c_int;
    (*env).mem_nodes_dynamic = 0 as *mut libc::c_void as *mut *mut Node;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        (*env).mem_nodes_static[i as usize] = 0 as *mut Node;
        i += 1
    }
    (*env).parse_depth = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn scan_env_add_mem_entry(mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut need: libc::c_int = 0;
    let mut alloc: libc::c_int = 0;
    let mut p: *mut *mut Node = 0 as *mut *mut Node;
    need = (*env).num_mem + 1 as libc::c_int;
    if need > 32767 as libc::c_int { return -(210 as libc::c_int) }
    if need >= 8 as libc::c_int {
        if (*env).mem_alloc <= need {
            if ((*env).mem_nodes_dynamic as *mut libc::c_void).is_null() {
                alloc = 16 as libc::c_int;
                p =
                    malloc((::std::mem::size_of::<*mut Node>() as
                                libc::c_ulong).wrapping_mul(alloc as
                                                                libc::c_ulong))
                        as *mut *mut Node;
                memcpy(p as *mut libc::c_void,
                       (*env).mem_nodes_static.as_mut_ptr() as
                           *const libc::c_void,
                       (::std::mem::size_of::<*mut Node>() as
                            libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                            libc::c_ulong));
            } else {
                alloc = (*env).mem_alloc * 2 as libc::c_int;
                p =
                    realloc((*env).mem_nodes_dynamic as *mut libc::c_void,
                            (::std::mem::size_of::<*mut Node>() as
                                 libc::c_ulong).wrapping_mul(alloc as
                                                                 libc::c_ulong))
                        as *mut *mut Node
            }
            if (p as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            i = (*env).num_mem + 1 as libc::c_int;
            while i < alloc {
                let ref mut fresh7 = *p.offset(i as isize);
                *fresh7 = 0 as *mut Node;
                i += 1
            }
            (*env).mem_nodes_dynamic = p;
            (*env).mem_alloc = alloc
        }
    }
    (*env).num_mem += 1;
    return (*env).num_mem;
}
unsafe extern "C" fn scan_env_set_mem_node(mut env: *mut ScanEnv,
                                           mut num: libc::c_int,
                                           mut node: *mut Node)
 -> libc::c_int {
    if (*env).num_mem >= num {
        let ref mut fresh8 =
            *if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null() {
                 (*env).mem_nodes_dynamic
             } else {
                 (*env).mem_nodes_static.as_mut_ptr()
             }.offset(num as isize);
        *fresh8 = node
    } else { return -(11 as libc::c_int) }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_free(mut node: *mut Node) {
    loop  {
        if (node as *mut libc::c_void).is_null() { return }
        match (*node).u.base.type_0 {
            0 => {
                if (*node).u.str_0.capa != 0 as libc::c_int &&
                       !((*node).u.str_0.s as *mut libc::c_void).is_null() &&
                       (*node).u.str_0.s != (*node).u.str_0.buf.as_mut_ptr() {
                    free((*node).u.str_0.s as *mut libc::c_void);
                }
                break ;
            }
            8 | 9 => {
                onig_node_free((*node).u.cons.car);
                let mut next_node: *mut Node = (*node).u.cons.cdr;
                free(node as *mut libc::c_void);
                node = next_node
            }
            1 => {
                let mut cc: *mut CClassNode = &mut (*node).u.cclass;
                if (*cc).flags &
                       ((1 as libc::c_int) << 1 as libc::c_int) as
                           libc::c_uint != 0 as libc::c_int as libc::c_uint {
                    return
                }
                if !(*cc).mbuf.is_null() { bbuf_free((*cc).mbuf); }
                break ;
            }
            5 => {
                if !(*node).u.qtfr.target.is_null() {
                    onig_node_free((*node).u.qtfr.target);
                }
                break ;
            }
            6 => {
                if !(*node).u.enclose.target.is_null() {
                    onig_node_free((*node).u.enclose.target);
                }
                break ;
            }
            4 => {
                if !((*node).u.bref.back_dynamic as
                         *mut libc::c_void).is_null() {
                    free((*node).u.bref.back_dynamic as *mut libc::c_void);
                }
                break ;
            }
            7 => {
                if !(*node).u.anchor.target.is_null() {
                    onig_node_free((*node).u.anchor.target);
                }
                break ;
            }
            _ => { break ; }
        }
    }
    free(node as *mut libc::c_void);
}
unsafe extern "C" fn node_new() -> *mut Node {
    let mut node: *mut Node = 0 as *mut Node;
    node =
        malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    /* xmemset(node, 0, sizeof(Node)); */
    return node;
}
unsafe extern "C" fn initialize_cclass(mut cc: *mut CClassNode) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (((1 as libc::c_int) << 8 as libc::c_int) as
                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_int {
        (*cc).bs[i as usize] = 0 as libc::c_int as Bits;
        i += 1
    }
    /* cc->base.flags = 0; */
    (*cc).flags = 0 as libc::c_int as libc::c_uint; /* /...(\1).../ */
    (*cc).mbuf = 0 as *mut BBuf; /* call by number if gnum != 0 */
}
unsafe extern "C" fn node_new_cclass() -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 1 as libc::c_int;
    initialize_cclass(&mut (*node).u.cclass);
    return node;
}
unsafe extern "C" fn node_new_ctype(mut type_0: libc::c_int,
                                    mut not: libc::c_int) -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 2 as libc::c_int;
    (*node).u.ctype.ctype = type_0;
    (*node).u.ctype.not = not;
    return node;
}
unsafe extern "C" fn node_new_anychar() -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 3 as libc::c_int;
    return node;
}
unsafe extern "C" fn node_new_list(mut left: *mut Node, mut right: *mut Node)
 -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 8 as libc::c_int;
    (*node).u.cons.car = left;
    (*node).u.cons.cdr = right;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_new_list(mut left: *mut Node,
                                            mut right: *mut Node)
 -> *mut Node {
    return node_new_list(left, right);
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_list_add(mut list: *mut Node,
                                            mut x: *mut Node) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    n = onig_node_new_list(x, 0 as *mut Node);
    if (n as *mut libc::c_void).is_null() { return 0 as *mut Node }
    if !(list as *mut libc::c_void).is_null() {
        while !((*list).u.cons.cdr as *mut libc::c_void).is_null() {
            list = (*list).u.cons.cdr
        }
        (*list).u.cons.cdr = n
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_new_alt(mut left: *mut Node,
                                           mut right: *mut Node)
 -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 9 as libc::c_int;
    (*node).u.cons.car = left;
    (*node).u.cons.cdr = right;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_new_anchor(mut type_0: libc::c_int)
 -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 7 as libc::c_int;
    (*node).u.anchor.type_0 = type_0;
    (*node).u.anchor.target = 0 as *mut _Node;
    (*node).u.anchor.char_len = -(1 as libc::c_int);
    return node;
}
unsafe extern "C" fn node_new_backref(mut back_num: libc::c_int,
                                      mut backrefs: *mut libc::c_int,
                                      mut by_name: libc::c_int,
                                      mut exist_level: libc::c_int,
                                      mut nest_level: libc::c_int,
                                      mut env: *mut ScanEnv) -> *mut Node {
    let mut i: libc::c_int = 0;
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 4 as libc::c_int;
    (*node).u.bref.state = 0 as libc::c_int;
    (*node).u.bref.back_num = back_num;
    (*node).u.bref.back_dynamic = 0 as *mut libc::c_void as *mut libc::c_int;
    if by_name != 0 as libc::c_int {
        (*node).u.bref.state |= (1 as libc::c_int) << 11 as libc::c_int
    }
    if exist_level != 0 as libc::c_int {
        (*node).u.bref.state |= (1 as libc::c_int) << 13 as libc::c_int;
        (*node).u.bref.nest_level = nest_level
    }
    i = 0 as libc::c_int;
    while i < back_num {
        if *backrefs.offset(i as isize) <= (*env).num_mem &&
               (*(if !((*env).mem_nodes_dynamic as
                           *mut libc::c_void).is_null() {
                      (*env).mem_nodes_dynamic
                  } else {
                      (*env).mem_nodes_static.as_mut_ptr()
                  }).offset(*backrefs.offset(i as isize) as isize) as
                    *mut libc::c_void).is_null() {
            (*node).u.bref.state |= (1 as libc::c_int) << 7 as libc::c_int;
            break ;
        } else { i += 1 }
    }
    if back_num <= 6 as libc::c_int {
        i = 0 as libc::c_int;
        while i < back_num {
            (*node).u.bref.back_static[i as usize] =
                *backrefs.offset(i as isize);
            i += 1
        }
    } else {
        let mut p: *mut libc::c_int =
            malloc((::std::mem::size_of::<libc::c_int>() as
                        libc::c_ulong).wrapping_mul(back_num as
                                                        libc::c_ulong)) as
                *mut libc::c_int;
        if (p as *mut libc::c_void).is_null() {
            onig_node_free(node);
            return 0 as *mut Node
        }
        (*node).u.bref.back_dynamic = p;
        i = 0 as libc::c_int;
        while i < back_num {
            *p.offset(i as isize) = *backrefs.offset(i as isize);
            i += 1
        }
    }
    return node;
}
unsafe extern "C" fn node_new_call(mut name: *mut OnigUChar,
                                   mut name_end: *mut OnigUChar,
                                   mut gnum: libc::c_int) -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 10 as libc::c_int;
    (*node).u.call.state = 0 as libc::c_int;
    (*node).u.call.target = 0 as *mut Node;
    (*node).u.call.name = name;
    (*node).u.call.name_end = name_end;
    (*node).u.call.group_num = gnum;
    return node;
}
unsafe extern "C" fn node_new_quantifier(mut lower: libc::c_int,
                                         mut upper: libc::c_int,
                                         mut by_number: libc::c_int)
 -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 5 as libc::c_int;
    (*node).u.qtfr.state = 0 as libc::c_int;
    (*node).u.qtfr.target = 0 as *mut _Node;
    (*node).u.qtfr.lower = lower;
    (*node).u.qtfr.upper = upper;
    (*node).u.qtfr.greedy = 1 as libc::c_int;
    (*node).u.qtfr.target_empty_info = 0 as libc::c_int;
    (*node).u.qtfr.head_exact = 0 as *mut Node;
    (*node).u.qtfr.next_head_exact = 0 as *mut Node;
    (*node).u.qtfr.is_refered = 0 as libc::c_int;
    if by_number != 0 as libc::c_int {
        (*node).u.qtfr.state |= (1 as libc::c_int) << 14 as libc::c_int
    }
    return node;
}
unsafe extern "C" fn node_new_enclose(mut type_0: libc::c_int) -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 6 as libc::c_int;
    (*node).u.enclose.type_0 = type_0;
    (*node).u.enclose.state = 0 as libc::c_int;
    (*node).u.enclose.regnum = 0 as libc::c_int;
    (*node).u.enclose.option = 0 as libc::c_int as OnigOptionType;
    (*node).u.enclose.target = 0 as *mut _Node;
    (*node).u.enclose.call_addr = -(1 as libc::c_int);
    (*node).u.enclose.opt_count = 0 as libc::c_int;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_new_enclose(mut type_0: libc::c_int)
 -> *mut Node {
    return node_new_enclose(type_0);
}
unsafe extern "C" fn node_new_enclose_memory(mut option: OnigOptionType,
                                             mut is_named: libc::c_int)
 -> *mut Node {
    let mut node: *mut Node =
        node_new_enclose((1 as libc::c_int) << 0 as libc::c_int);
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    if is_named != 0 as libc::c_int {
        (*node).u.enclose.state |= (1 as libc::c_int) << 10 as libc::c_int
    }
    (*node).u.enclose.option = option;
    return node;
}
unsafe extern "C" fn node_new_option(mut option: OnigOptionType)
 -> *mut Node {
    let mut node: *mut Node =
        node_new_enclose((1 as libc::c_int) << 1 as libc::c_int);
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.enclose.option = option;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_str_cat(mut node: *mut Node,
                                           mut s: *const OnigUChar,
                                           mut end: *const OnigUChar)
 -> libc::c_int {
    let mut addlen: libc::c_int =
        end.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    if addlen > 0 as libc::c_int {
        let mut len: libc::c_int =
            (*node).u.str_0.end.wrapping_offset_from((*node).u.str_0.s) as
                libc::c_long as libc::c_int;
        if (*node).u.str_0.capa > 0 as libc::c_int ||
               len + addlen > 24 as libc::c_int - 1 as libc::c_int {
            let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
            let mut capa: libc::c_int = len + addlen + 16 as libc::c_int;
            if capa <= (*node).u.str_0.capa {
                onig_strcpy((*node).u.str_0.s.offset(len as isize), s, end);
            } else {
                if (*node).u.str_0.s == (*node).u.str_0.buf.as_mut_ptr() {
                    p =
                        strcat_capa_from_static((*node).u.str_0.s,
                                                (*node).u.str_0.end, s, end,
                                                capa)
                } else {
                    p =
                        strcat_capa((*node).u.str_0.s, (*node).u.str_0.end, s,
                                    end, capa)
                }
                if (p as *mut libc::c_void).is_null() {
                    return -(5 as libc::c_int)
                }
                (*node).u.str_0.s = p;
                (*node).u.str_0.capa = capa
            }
        } else {
            onig_strcpy((*node).u.str_0.s.offset(len as isize), s, end);
        }
        (*node).u.str_0.end =
            (*node).u.str_0.s.offset(len as isize).offset(addlen as isize)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_str_set(mut node: *mut Node,
                                           mut s: *const OnigUChar,
                                           mut end: *const OnigUChar)
 -> libc::c_int {
    onig_node_str_clear(node);
    return onig_node_str_cat(node, s, end);
}
unsafe extern "C" fn node_str_cat_char(mut node: *mut Node, mut c: OnigUChar)
 -> libc::c_int {
    let mut s: [OnigUChar; 1] = [0; 1];
    s[0 as libc::c_int as usize] = c;
    return onig_node_str_cat(node, s.as_mut_ptr(),
                             s.as_mut_ptr().offset(1 as libc::c_int as
                                                       isize));
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_conv_to_str_node(mut node: *mut Node,
                                                    mut flag: libc::c_int) {
    (*node).u.base.type_0 = 0 as libc::c_int;
    (*node).u.str_0.flag = flag as libc::c_uint;
    (*node).u.str_0.capa = 0 as libc::c_int;
    (*node).u.str_0.s = (*node).u.str_0.buf.as_mut_ptr();
    (*node).u.str_0.end = (*node).u.str_0.buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_str_clear(mut node: *mut Node) {
    if (*node).u.str_0.capa != 0 as libc::c_int &&
           !((*node).u.str_0.s as *mut libc::c_void).is_null() &&
           (*node).u.str_0.s != (*node).u.str_0.buf.as_mut_ptr() {
        free((*node).u.str_0.s as *mut libc::c_void);
    }
    (*node).u.str_0.capa = 0 as libc::c_int;
    (*node).u.str_0.flag = 0 as libc::c_int as libc::c_uint;
    (*node).u.str_0.s = (*node).u.str_0.buf.as_mut_ptr();
    (*node).u.str_0.end = (*node).u.str_0.buf.as_mut_ptr();
}
unsafe extern "C" fn node_new_str(mut s: *const OnigUChar,
                                  mut end: *const OnigUChar) -> *mut Node {
    let mut node: *mut Node = node_new();
    if (node as *mut libc::c_void).is_null() { return 0 as *mut Node }
    (*node).u.base.type_0 = 0 as libc::c_int;
    (*node).u.str_0.capa = 0 as libc::c_int;
    (*node).u.str_0.flag = 0 as libc::c_int as libc::c_uint;
    (*node).u.str_0.s = (*node).u.str_0.buf.as_mut_ptr();
    (*node).u.str_0.end = (*node).u.str_0.buf.as_mut_ptr();
    if onig_node_str_cat(node, s, end) != 0 {
        onig_node_free(node);
        return 0 as *mut Node
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn onig_node_new_str(mut s: *const OnigUChar,
                                           mut end: *const OnigUChar)
 -> *mut Node {
    return node_new_str(s, end);
}
unsafe extern "C" fn node_new_str_raw(mut s: *mut OnigUChar,
                                      mut end: *mut OnigUChar) -> *mut Node {
    let mut node: *mut Node = node_new_str(s, end);
    (*node).u.str_0.flag |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    return node;
}
unsafe extern "C" fn node_new_empty() -> *mut Node {
    return node_new_str(0 as *const OnigUChar, 0 as *const OnigUChar);
}
unsafe extern "C" fn node_new_str_raw_char(mut c: OnigUChar) -> *mut Node {
    let mut p: [OnigUChar; 1] = [0; 1];
    p[0 as libc::c_int as usize] = c;
    return node_new_str_raw(p.as_mut_ptr(),
                            p.as_mut_ptr().offset(1 as libc::c_int as isize));
}
unsafe extern "C" fn str_node_split_last_char(mut sn: *mut StrNode,
                                              mut enc: OnigEncoding)
 -> *mut Node {
    let mut p: *const OnigUChar = 0 as *const OnigUChar;
    let mut n: *mut Node = 0 as *mut Node;
    if (*sn).end > (*sn).s {
        p = onigenc_get_prev_char_head(enc, (*sn).s, (*sn).end);
        if !p.is_null() && p > (*sn).s {
            /* can be split. */
            n = node_new_str(p, (*sn).end); /* overflow */
            if (*sn).flag &
                   ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                (*n).u.str_0.flag |=
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            } /* overflow */
            (*sn).end = p as *mut OnigUChar
        }
    } /* overflow */
    return n;
}
unsafe extern "C" fn str_node_can_be_split(mut sn: *mut StrNode,
                                           mut enc: OnigEncoding)
 -> libc::c_int {
    if (*sn).end > (*sn).s {
        return if ((*enc).mbc_enc_len.expect("non-null function pointer")((*sn).s)
                       as libc::c_long) <
                      (*sn).end.wrapping_offset_from((*sn).s) as libc::c_long
                  {
                   1 as libc::c_int
               } else { 0 as libc::c_int }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_scan_unsigned_number(mut src:
                                                       *mut *mut OnigUChar,
                                                   mut end: *const OnigUChar,
                                                   mut enc: OnigEncoding)
 -> libc::c_int {
    let mut num: libc::c_uint = 0;
    let mut val: libc::c_uint = 0;
    let mut c: OnigCodePoint = 0;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    num = 0 as libc::c_int as libc::c_uint;
    while if p < end as *mut OnigUChar {
              0 as libc::c_int
          } else { 1 as libc::c_int } == 0 {
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                    4 as
                                                                        libc::c_int
                                                                        as
                                                                        OnigCtype)
               != 0 {
            val = c.wrapping_sub('0' as i32 as libc::c_uint);
            if ((1 as libc::c_ulong) <<
                    4 as libc::c_int * 8 as libc::c_int -
                        1 as
                            libc::c_int).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong).wrapping_sub(val
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(10
                                                                                                                          as
                                                                                                                          libc::c_ulong)
                   < num as libc::c_ulong {
                return -(1 as libc::c_int)
            }
            num =
                num.wrapping_mul(10 as libc::c_int as
                                     libc::c_uint).wrapping_add(val)
        } else { p = pfetch_prev; break ; }
    }
    *src = p;
    return num as libc::c_int;
}
unsafe extern "C" fn scan_unsigned_hexadecimal_number(mut src:
                                                          *mut *mut OnigUChar,
                                                      mut end: *mut OnigUChar,
                                                      mut maxlen: libc::c_int,
                                                      mut enc: OnigEncoding)
 -> libc::c_int {
    let mut c: OnigCodePoint = 0;
    let mut num: libc::c_uint = 0;
    let mut val: libc::c_uint = 0;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    num = 0 as libc::c_int as libc::c_uint;
    while (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) == 0 &&
              {
                  let fresh9 = maxlen;
                  maxlen = maxlen - 1;
                  (fresh9) != 0 as libc::c_int
              } {
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                    11 as
                                                                        libc::c_int
                                                                        as
                                                                        OnigCtype)
               != 0 {
            val =
                if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                            4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                OnigCtype)
                       != 0 {
                    c.wrapping_sub('0' as i32 as libc::c_uint)
                } else if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                                   10
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       OnigCtype)
                              != 0 {
                    c.wrapping_sub('A' as i32 as
                                       libc::c_uint).wrapping_add(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                } else {
                    c.wrapping_sub('a' as i32 as
                                       libc::c_uint).wrapping_add(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                };
            if ((1 as libc::c_ulong) <<
                    4 as libc::c_int * 8 as libc::c_int -
                        1 as
                            libc::c_int).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong).wrapping_sub(val
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(16
                                                                                                                          as
                                                                                                                          libc::c_ulong)
                   < num as libc::c_ulong {
                return -(1 as libc::c_int)
            }
            num =
                (num <<
                     4 as
                         libc::c_int).wrapping_add((if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                                                                4
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    OnigCtype)
                                                           != 0 {
                                                        c.wrapping_sub('0' as
                                                                           i32
                                                                           as
                                                                           libc::c_uint)
                                                    } else {
                                                        (if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                                                                     10
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         OnigCtype)
                                                                != 0 {
                                                             c.wrapping_sub('A'
                                                                                as
                                                                                i32
                                                                                as
                                                                                libc::c_uint).wrapping_add(10
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint)
                                                         } else {
                                                             c.wrapping_sub('a'
                                                                                as
                                                                                i32
                                                                                as
                                                                                libc::c_uint).wrapping_add(10
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint)
                                                         })
                                                    }))
        } else { p = pfetch_prev; break ; }
    }
    *src = p;
    return num as libc::c_int;
}
unsafe extern "C" fn scan_unsigned_octal_number(mut src: *mut *mut OnigUChar,
                                                mut end: *mut OnigUChar,
                                                mut maxlen: libc::c_int,
                                                mut enc: OnigEncoding)
 -> libc::c_int {
    let mut c: OnigCodePoint = 0;
    let mut num: libc::c_uint = 0;
    let mut val: libc::c_uint = 0;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    num = 0 as libc::c_int as libc::c_uint;
    while (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) == 0 &&
              {
                  let fresh10 = maxlen;
                  maxlen = maxlen - 1;
                  (fresh10) != 0 as libc::c_int
              } {
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                    4 as
                                                                        libc::c_int
                                                                        as
                                                                        OnigCtype)
               != 0 && c < '8' as i32 as libc::c_uint {
            val = c.wrapping_sub('0' as i32 as libc::c_uint);
            if ((1 as libc::c_ulong) <<
                    4 as libc::c_int * 8 as libc::c_int -
                        1 as
                            libc::c_int).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong).wrapping_sub(val
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(8
                                                                                                                          as
                                                                                                                          libc::c_ulong)
                   < num as libc::c_ulong {
                return -(1 as libc::c_int)
            }
            num = (num << 3 as libc::c_int).wrapping_add(val)
        } else { p = pfetch_prev; break ; }
    }
    *src = p;
    return num as libc::c_int;
}
/* data format:
     [n][from-1][to-1][from-2][to-2] ... [from-n][to-n]
     (all data size is OnigCodePoint)
 */
unsafe extern "C" fn new_code_range(mut pbuf: *mut *mut BBuf) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut n: OnigCodePoint = 0;
    let mut bbuf: *mut BBuf = 0 as *mut BBuf;
    *pbuf =
        malloc(::std::mem::size_of::<BBuf>() as libc::c_ulong) as *mut BBuf;
    bbuf = *pbuf;
    if (*pbuf as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    r =
        onig_bbuf_init(*pbuf,
                       (::std::mem::size_of::<OnigCodePoint>() as
                            libc::c_ulong).wrapping_mul(5 as libc::c_int as
                                                            libc::c_ulong) as
                           libc::c_int);
    if r != 0 { return r }
    n = 0 as libc::c_int as OnigCodePoint;
    let mut used: libc::c_int =
        (0 as libc::c_int as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigCodePoint>()
                                             as libc::c_ulong) as libc::c_int;
    if (*bbuf).alloc < used as libc::c_uint {
        loop  {
            (*bbuf).alloc =
                (*bbuf).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*bbuf).alloc < used as libc::c_uint) { break ; }
        }
        (*bbuf).p =
            realloc((*bbuf).p as *mut libc::c_void,
                    (*bbuf).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*bbuf).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*bbuf).p.offset(0 as libc::c_int as isize) as *mut libc::c_void,
           &mut n as *mut OnigCodePoint as *const libc::c_void,
           ::std::mem::size_of::<OnigCodePoint>() as libc::c_ulong);
    if (*bbuf).used < used as libc::c_uint {
        (*bbuf).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_code_range_to_buf(mut pbuf: *mut *mut BBuf,
                                           mut from: OnigCodePoint,
                                           mut to: OnigCodePoint)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut inc_n: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut bound: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut n: OnigCodePoint = 0;
    let mut data: *mut OnigCodePoint = 0 as *mut OnigCodePoint;
    let mut bbuf: *mut BBuf = 0 as *mut BBuf;
    if from > to { n = from; from = to; to = n }
    if (*pbuf as *mut libc::c_void).is_null() {
        r = new_code_range(pbuf);
        if r != 0 { return r }
        bbuf = *pbuf;
        n = 0 as libc::c_int as OnigCodePoint
    } else { bbuf = *pbuf; n = *((*bbuf).p as *mut OnigCodePoint) }
    data = (*bbuf).p as *mut OnigCodePoint;
    data = data.offset(1);
    low = 0 as libc::c_int;
    bound = n as libc::c_int;
    while low < bound {
        x = low + bound >> 1 as libc::c_int;
        if from >
               *data.offset((x * 2 as libc::c_int + 1 as libc::c_int) as
                                isize) {
            low = x + 1 as libc::c_int
        } else { bound = x }
    }
    high =
        if to == !(0 as libc::c_int as OnigCodePoint) {
            n
        } else { low as libc::c_uint } as libc::c_int;
    bound = n as libc::c_int;
    while high < bound {
        x = high + bound >> 1 as libc::c_int;
        if to.wrapping_add(1 as libc::c_int as libc::c_uint) >=
               *data.offset((x * 2 as libc::c_int) as isize) {
            high = x + 1 as libc::c_int
        } else { bound = x }
    }
    inc_n = low + 1 as libc::c_int - high;
    if n.wrapping_add(inc_n as libc::c_uint) >
           10000 as libc::c_int as libc::c_uint {
        return -(205 as libc::c_int)
    }
    if inc_n != 1 as libc::c_int {
        if from > *data.offset((low * 2 as libc::c_int) as isize) {
            from = *data.offset((low * 2 as libc::c_int) as isize)
        }
        if to <
               *data.offset(((high - 1 as libc::c_int) * 2 as libc::c_int +
                                 1 as libc::c_int) as isize) {
            to =
                *data.offset(((high - 1 as libc::c_int) * 2 as libc::c_int +
                                  1 as libc::c_int) as isize)
        }
    }
    if inc_n != 0 as libc::c_int && (high as OnigCodePoint) < n {
        let mut from_pos: libc::c_int =
            (::std::mem::size_of::<OnigCodePoint>() as
                 libc::c_ulong).wrapping_mul((1 as libc::c_int +
                                                  high * 2 as libc::c_int) as
                                                 libc::c_ulong) as
                libc::c_int;
        let mut to_pos: libc::c_int =
            (::std::mem::size_of::<OnigCodePoint>() as
                 libc::c_ulong).wrapping_mul((1 as libc::c_int +
                                                  (low + 1 as libc::c_int) *
                                                      2 as libc::c_int) as
                                                 libc::c_ulong) as
                libc::c_int;
        let mut size: libc::c_int =
            (n.wrapping_sub(high as
                                libc::c_uint).wrapping_mul(2 as libc::c_int as
                                                               libc::c_uint)
                 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<OnigCodePoint>()
                                                 as libc::c_ulong) as
                libc::c_int;
        if inc_n > 0 as libc::c_int {
            if (to_pos + size) as libc::c_uint > (*bbuf).alloc {
                loop  {
                    (*bbuf).alloc =
                        (*bbuf).alloc.wrapping_mul(2 as libc::c_int as
                                                       libc::c_uint);
                    if !((*bbuf).alloc <
                             (to_pos as
                                  libc::c_uint).wrapping_add(size as
                                                                 libc::c_uint))
                       {
                        break ;
                    }
                }
                (*bbuf).p =
                    realloc((*bbuf).p as *mut libc::c_void,
                            (*bbuf).alloc as libc::c_ulong) as *mut OnigUChar;
                if ((*bbuf).p as *mut libc::c_void).is_null() {
                    return -(5 as libc::c_int)
                }
            }
            memmove((*bbuf).p.offset(to_pos as isize) as *mut libc::c_void,
                    (*bbuf).p.offset(from_pos as isize) as
                        *const libc::c_void, size as libc::c_ulong);
            if (to_pos + size) as libc::c_uint > (*bbuf).used {
                (*bbuf).used = (to_pos + size) as libc::c_uint
            }
        } else {
            memmove((*bbuf).p.offset(to_pos as isize) as *mut libc::c_void,
                    (*bbuf).p.offset(from_pos as isize) as
                        *const libc::c_void,
                    (*bbuf).used.wrapping_sub(from_pos as libc::c_uint) as
                        libc::c_ulong);
            (*bbuf).used =
                (*bbuf).used.wrapping_sub((from_pos - to_pos) as libc::c_uint)
        }
    }
    pos =
        (::std::mem::size_of::<OnigCodePoint>() as
             libc::c_ulong).wrapping_mul((1 as libc::c_int +
                                              low * 2 as libc::c_int) as
                                             libc::c_ulong) as libc::c_int;
    let mut new_alloc: libc::c_uint = (*bbuf).alloc;
    while new_alloc <
              (pos as
                   libc::c_ulong).wrapping_add((::std::mem::size_of::<OnigCodePoint>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong))
                  as libc::c_uint {
        new_alloc = new_alloc.wrapping_mul(2 as libc::c_int as libc::c_uint)
    }
    if (*bbuf).alloc != new_alloc {
        (*bbuf).p =
            realloc((*bbuf).p as *mut libc::c_void,
                    new_alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*bbuf).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        (*bbuf).alloc = new_alloc
    }
    let mut used: libc::c_int =
        (pos as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigCodePoint>()
                                             as libc::c_ulong) as libc::c_int;
    if (*bbuf).alloc < used as libc::c_uint {
        loop  {
            (*bbuf).alloc =
                (*bbuf).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*bbuf).alloc < used as libc::c_uint) { break ; }
        }
        (*bbuf).p =
            realloc((*bbuf).p as *mut libc::c_void,
                    (*bbuf).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*bbuf).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*bbuf).p.offset(pos as isize) as *mut libc::c_void,
           &mut from as *mut OnigCodePoint as *const libc::c_void,
           ::std::mem::size_of::<OnigCodePoint>() as libc::c_ulong);
    if (*bbuf).used < used as libc::c_uint {
        (*bbuf).used = used as libc::c_uint
    }
    let mut used_0: libc::c_int =
        (pos as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigCodePoint>()
                                             as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigCodePoint>()
                                                                             as
                                                                             libc::c_ulong)
            as libc::c_int;
    if (*bbuf).alloc < used_0 as libc::c_uint {
        loop  {
            (*bbuf).alloc =
                (*bbuf).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*bbuf).alloc < used_0 as libc::c_uint) { break ; }
        }
        (*bbuf).p =
            realloc((*bbuf).p as *mut libc::c_void,
                    (*bbuf).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*bbuf).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*bbuf).p.offset((pos as
                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigCodePoint>()
                                                                 as
                                                                 libc::c_ulong)
                                as isize) as *mut libc::c_void,
           &mut to as *mut OnigCodePoint as *const libc::c_void,
           ::std::mem::size_of::<OnigCodePoint>() as libc::c_ulong);
    if (*bbuf).used < used_0 as libc::c_uint {
        (*bbuf).used = used_0 as libc::c_uint
    }
    n =
        (n as libc::c_uint).wrapping_add(inc_n as libc::c_uint) as
            OnigCodePoint as OnigCodePoint;
    let mut used_1: libc::c_int =
        (0 as libc::c_int as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigCodePoint>()
                                             as libc::c_ulong) as libc::c_int;
    if (*bbuf).alloc < used_1 as libc::c_uint {
        loop  {
            (*bbuf).alloc =
                (*bbuf).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*bbuf).alloc < used_1 as libc::c_uint) { break ; }
        }
        (*bbuf).p =
            realloc((*bbuf).p as *mut libc::c_void,
                    (*bbuf).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*bbuf).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*bbuf).p.offset(0 as libc::c_int as isize) as *mut libc::c_void,
           &mut n as *mut OnigCodePoint as *const libc::c_void,
           ::std::mem::size_of::<OnigCodePoint>() as libc::c_ulong);
    if (*bbuf).used < used_1 as libc::c_uint {
        (*bbuf).used = used_1 as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_code_range(mut pbuf: *mut *mut BBuf,
                                    mut env: *mut ScanEnv,
                                    mut from: OnigCodePoint,
                                    mut to: OnigCodePoint) -> libc::c_int {
    if from > to {
        if (*(*env).syntax).behavior &
               (1 as libc::c_uint) << 22 as libc::c_int !=
               0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        } else { return -(203 as libc::c_int) }
    }
    return add_code_range_to_buf(pbuf, from, to);
}
unsafe extern "C" fn not_code_range_buf(mut enc: OnigEncoding,
                                        mut bbuf: *mut BBuf,
                                        mut pbuf: *mut *mut BBuf)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pre: OnigCodePoint = 0;
    let mut from: OnigCodePoint = 0;
    let mut data: *mut OnigCodePoint = 0 as *mut OnigCodePoint;
    let mut to: OnigCodePoint = 0 as libc::c_int as OnigCodePoint;
    *pbuf = 0 as *mut libc::c_void as *mut BBuf;
    if !(bbuf as *mut libc::c_void).is_null() {
        data = (*bbuf).p as *mut OnigCodePoint;
        n = *data as libc::c_int;
        data = data.offset(1);
        if !(n <= 0 as libc::c_int) {
            r = 0 as libc::c_int;
            pre =
                if (*enc).min_enc_len > 1 as libc::c_int {
                    0 as libc::c_int
                } else { 0x80 as libc::c_int } as OnigCodePoint;
            i = 0 as libc::c_int;
            while i < n {
                from = *data.offset((i * 2 as libc::c_int) as isize);
                to =
                    *data.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                     isize);
                if pre <= from.wrapping_sub(1 as libc::c_int as libc::c_uint)
                   {
                    r =
                        add_code_range_to_buf(pbuf, pre,
                                              from.wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint));
                    if r != 0 as libc::c_int { return r }
                }
                if to == !(0 as libc::c_int as OnigCodePoint) { break ; }
                pre = to.wrapping_add(1 as libc::c_int as libc::c_uint);
                i += 1
            }
            if to < !(0 as libc::c_int as OnigCodePoint) {
                r =
                    add_code_range_to_buf(pbuf,
                                          to.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint),
                                          !(0 as libc::c_int as
                                                OnigCodePoint))
            }
            return r
        }
    }
    return add_code_range_to_buf(pbuf,
                                 if (*enc).min_enc_len > 1 as libc::c_int {
                                     0 as libc::c_int
                                 } else { 0x80 as libc::c_int } as
                                     OnigCodePoint,
                                 !(0 as libc::c_int as OnigCodePoint));
}
unsafe extern "C" fn or_code_range_buf(mut enc: OnigEncoding,
                                       mut bbuf1: *mut BBuf,
                                       mut not1: libc::c_int,
                                       mut bbuf2: *mut BBuf,
                                       mut not2: libc::c_int,
                                       mut pbuf: *mut *mut BBuf)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: OnigCodePoint = 0;
    let mut n1: OnigCodePoint = 0;
    let mut data1: *mut OnigCodePoint = 0 as *mut OnigCodePoint;
    let mut from: OnigCodePoint = 0;
    let mut to: OnigCodePoint = 0;
    *pbuf = 0 as *mut libc::c_void as *mut BBuf;
    if (bbuf1 as *mut libc::c_void).is_null() &&
           (bbuf2 as *mut libc::c_void).is_null() {
        if not1 != 0 as libc::c_int || not2 != 0 as libc::c_int {
            return add_code_range_to_buf(pbuf,
                                         if (*enc).min_enc_len >
                                                1 as libc::c_int {
                                             0 as libc::c_int
                                         } else { 0x80 as libc::c_int } as
                                             OnigCodePoint,
                                         !(0 as libc::c_int as OnigCodePoint))
        }
        return 0 as libc::c_int
    }
    r = 0 as libc::c_int;
    if (bbuf2 as *mut libc::c_void).is_null() {
        let mut tbuf: *mut BBuf = 0 as *mut BBuf;
        let mut tnot: libc::c_int = 0;
        tnot = not1;
        not1 = not2;
        not2 = tnot;
        tbuf = bbuf1;
        bbuf1 = bbuf2;
        bbuf2 = tbuf
    }
    if (bbuf1 as *mut libc::c_void).is_null() {
        if not1 != 0 as libc::c_int {
            return add_code_range_to_buf(pbuf,
                                         if (*enc).min_enc_len >
                                                1 as libc::c_int {
                                             0 as libc::c_int
                                         } else { 0x80 as libc::c_int } as
                                             OnigCodePoint,
                                         !(0 as libc::c_int as OnigCodePoint))
        } else if not2 == 0 as libc::c_int {
            return bbuf_clone(pbuf, bbuf2)
        } else { return not_code_range_buf(enc, bbuf2, pbuf) }
    }
    if not1 != 0 as libc::c_int {
        let mut tbuf_0: *mut BBuf = 0 as *mut BBuf;
        let mut tnot_0: libc::c_int = 0;
        tnot_0 = not1;
        not1 = not2;
        not2 = tnot_0;
        tbuf_0 = bbuf1;
        bbuf1 = bbuf2;
        bbuf2 = tbuf_0
    }
    data1 = (*bbuf1).p as *mut OnigCodePoint;
    n1 = *data1;
    data1 = data1.offset(1);
    if not2 == 0 as libc::c_int && not1 == 0 as libc::c_int {
        /* 1 OR 2 */
        r = bbuf_clone(pbuf, bbuf2)
    } else if not1 == 0 as libc::c_int {
        /* 1 OR (not 2) */
        r = not_code_range_buf(enc, bbuf2, pbuf)
    }
    if r != 0 as libc::c_int { return r }
    i = 0 as libc::c_int as OnigCodePoint;
    while i < n1 {
        from =
            *data1.offset(i.wrapping_mul(2 as libc::c_int as libc::c_uint) as
                              isize);
        to =
            *data1.offset(i.wrapping_mul(2 as libc::c_int as
                                             libc::c_uint).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                              as isize);
        r = add_code_range_to_buf(pbuf, from, to);
        if r != 0 as libc::c_int { return r }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn and_code_range1(mut pbuf: *mut *mut BBuf,
                                     mut from1: OnigCodePoint,
                                     mut to1: OnigCodePoint,
                                     mut data: *mut OnigCodePoint,
                                     mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut from2: OnigCodePoint = 0;
    let mut to2: OnigCodePoint = 0;
    let mut current_block_18: u64;
    i = 0 as libc::c_int;
    while i < n {
        from2 = *data.offset((i * 2 as libc::c_int) as isize);
        to2 =
            *data.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
        if from2 < from1 {
            if to2 < from1 {
                current_block_18 = 16559507199688588974;
            } else {
                from1 = to2.wrapping_add(1 as libc::c_int as libc::c_uint);
                current_block_18 = 2668756484064249700;
            }
        } else {
            if from2 <= to1 {
                if to2 < to1 {
                    if from1 <=
                           from2.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) {
                        r =
                            add_code_range_to_buf(pbuf, from1,
                                                  from2.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint));
                        if r != 0 as libc::c_int { return r }
                    }
                    from1 = to2.wrapping_add(1 as libc::c_int as libc::c_uint)
                } else {
                    to1 = from2.wrapping_sub(1 as libc::c_int as libc::c_uint)
                }
            } else { from1 = from2 }
            current_block_18 = 2668756484064249700;
        }
        match current_block_18 {
            2668756484064249700 => { if from1 > to1 { break ; } }
            _ => { }
        }
        i += 1
    }
    if from1 <= to1 {
        r = add_code_range_to_buf(pbuf, from1, to1);
        if r != 0 as libc::c_int { return r }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn and_code_range_buf(mut bbuf1: *mut BBuf,
                                        mut not1: libc::c_int,
                                        mut bbuf2: *mut BBuf,
                                        mut not2: libc::c_int,
                                        mut pbuf: *mut *mut BBuf)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: OnigCodePoint = 0;
    let mut j: OnigCodePoint = 0;
    let mut n1: OnigCodePoint = 0;
    let mut n2: OnigCodePoint = 0;
    let mut data1: *mut OnigCodePoint = 0 as *mut OnigCodePoint;
    let mut data2: *mut OnigCodePoint = 0 as *mut OnigCodePoint;
    let mut from: OnigCodePoint = 0;
    let mut to: OnigCodePoint = 0;
    let mut from1: OnigCodePoint = 0;
    let mut to1: OnigCodePoint = 0;
    let mut from2: OnigCodePoint = 0;
    let mut to2: OnigCodePoint = 0;
    *pbuf = 0 as *mut libc::c_void as *mut BBuf;
    if (bbuf1 as *mut libc::c_void).is_null() {
        if not1 != 0 as libc::c_int && !(bbuf2 as *mut libc::c_void).is_null()
           {
            /* not1 != 0 -> not2 == 0 */
            return bbuf_clone(pbuf, bbuf2)
        }
        return 0 as libc::c_int
    } else {
        if (bbuf2 as *mut libc::c_void).is_null() {
            if not2 != 0 as libc::c_int { return bbuf_clone(pbuf, bbuf1) }
            return 0 as libc::c_int
        }
    }
    if not1 != 0 as libc::c_int {
        let mut tbuf: *mut BBuf = 0 as *mut BBuf;
        let mut tnot: libc::c_int = 0;
        tnot = not1;
        not1 = not2;
        not2 = tnot;
        tbuf = bbuf1;
        bbuf1 = bbuf2;
        bbuf2 = tbuf
    }
    data1 = (*bbuf1).p as *mut OnigCodePoint;
    data2 = (*bbuf2).p as *mut OnigCodePoint;
    n1 = *data1;
    n2 = *data2;
    data1 = data1.offset(1);
    data2 = data2.offset(1);
    if not2 == 0 as libc::c_int && not1 == 0 as libc::c_int {
        /* 1 AND 2 */
        i = 0 as libc::c_int as OnigCodePoint;
        while i < n1 {
            from1 =
                *data1.offset(i.wrapping_mul(2 as libc::c_int as libc::c_uint)
                                  as isize);
            to1 =
                *data1.offset(i.wrapping_mul(2 as libc::c_int as
                                                 libc::c_uint).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                  as isize);
            j = 0 as libc::c_int as OnigCodePoint;
            while j < n2 {
                from2 =
                    *data2.offset(j.wrapping_mul(2 as libc::c_int as
                                                     libc::c_uint) as isize);
                to2 =
                    *data2.offset(j.wrapping_mul(2 as libc::c_int as
                                                     libc::c_uint).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                      as isize);
                if from2 > to1 { break ; }
                if !(to2 < from1) {
                    from = if from1 < from2 { from2 } else { from1 };
                    to = if to1 > to2 { to2 } else { to1 };
                    r = add_code_range_to_buf(pbuf, from, to);
                    if r != 0 as libc::c_int { return r }
                }
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
    } else if not1 == 0 as libc::c_int {
        /* 1 AND (not 2) */
        i = 0 as libc::c_int as OnigCodePoint;
        while i < n1 {
            from1 =
                *data1.offset(i.wrapping_mul(2 as libc::c_int as libc::c_uint)
                                  as isize);
            to1 =
                *data1.offset(i.wrapping_mul(2 as libc::c_int as
                                                 libc::c_uint).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                  as isize);
            r = and_code_range1(pbuf, from1, to1, data2, n2 as libc::c_int);
            if r != 0 as libc::c_int { return r }
            i = i.wrapping_add(1)
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn and_cclass(mut dest: *mut CClassNode,
                                mut cc: *mut CClassNode,
                                mut enc: OnigEncoding) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut not1: libc::c_int = 0;
    let mut not2: libc::c_int = 0;
    let mut buf1: *mut BBuf = 0 as *mut BBuf;
    let mut buf2: *mut BBuf = 0 as *mut BBuf;
    let mut pbuf: *mut BBuf = 0 as *mut BBuf;
    let mut bsr1: BitSetRef = 0 as *mut Bits;
    let mut bsr2: BitSetRef = 0 as *mut Bits;
    let mut bs1: BitSet = [0; 8];
    let mut bs2: BitSet = [0; 8];
    not1 =
        ((*dest).flags &
             ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int;
    bsr1 = (*dest).bs.as_mut_ptr();
    buf1 = (*dest).mbuf;
    not2 =
        ((*cc).flags &
             ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int;
    bsr2 = (*cc).bs.as_mut_ptr();
    buf2 = (*cc).mbuf;
    if not1 != 0 as libc::c_int {
        bitset_invert_to(bsr1, bs1.as_mut_ptr());
        bsr1 = bs1.as_mut_ptr()
    }
    if not2 != 0 as libc::c_int {
        bitset_invert_to(bsr2, bs2.as_mut_ptr());
        bsr2 = bs2.as_mut_ptr()
    }
    bitset_and(bsr1, bsr2);
    if bsr1 != (*dest).bs.as_mut_ptr() {
        bitset_copy((*dest).bs.as_mut_ptr(), bsr1);
        bsr1 = (*dest).bs.as_mut_ptr()
    }
    if not1 != 0 as libc::c_int { bitset_invert((*dest).bs.as_mut_ptr()); }
    if !((*enc).max_enc_len == 1 as libc::c_int) {
        if not1 != 0 as libc::c_int && not2 != 0 as libc::c_int {
            r =
                or_code_range_buf(enc, buf1, 0 as libc::c_int, buf2,
                                  0 as libc::c_int, &mut pbuf)
        } else {
            r = and_code_range_buf(buf1, not1, buf2, not2, &mut pbuf);
            if r == 0 as libc::c_int && not1 != 0 as libc::c_int {
                let mut tbuf: *mut BBuf = 0 as *mut BBuf;
                r = not_code_range_buf(enc, pbuf, &mut tbuf);
                if r != 0 as libc::c_int { bbuf_free(pbuf); return r }
                bbuf_free(pbuf);
                pbuf = tbuf
            }
        }
        if r != 0 as libc::c_int { return r }
        (*dest).mbuf = pbuf;
        bbuf_free(buf1);
        return r
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn or_cclass(mut dest: *mut CClassNode,
                               mut cc: *mut CClassNode, mut enc: OnigEncoding)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut not1: libc::c_int = 0;
    let mut not2: libc::c_int = 0;
    let mut buf1: *mut BBuf = 0 as *mut BBuf;
    let mut buf2: *mut BBuf = 0 as *mut BBuf;
    let mut pbuf: *mut BBuf = 0 as *mut BBuf;
    let mut bsr1: BitSetRef = 0 as *mut Bits;
    let mut bsr2: BitSetRef = 0 as *mut Bits;
    let mut bs1: BitSet = [0; 8];
    let mut bs2: BitSet = [0; 8];
    not1 =
        ((*dest).flags &
             ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int;
    bsr1 = (*dest).bs.as_mut_ptr();
    buf1 = (*dest).mbuf;
    not2 =
        ((*cc).flags &
             ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int;
    bsr2 = (*cc).bs.as_mut_ptr();
    buf2 = (*cc).mbuf;
    if not1 != 0 as libc::c_int {
        bitset_invert_to(bsr1, bs1.as_mut_ptr());
        bsr1 = bs1.as_mut_ptr()
    }
    if not2 != 0 as libc::c_int {
        bitset_invert_to(bsr2, bs2.as_mut_ptr());
        bsr2 = bs2.as_mut_ptr()
    }
    bitset_or(bsr1, bsr2);
    if bsr1 != (*dest).bs.as_mut_ptr() {
        bitset_copy((*dest).bs.as_mut_ptr(), bsr1);
        bsr1 = (*dest).bs.as_mut_ptr()
    }
    if not1 != 0 as libc::c_int { bitset_invert((*dest).bs.as_mut_ptr()); }
    if !((*enc).max_enc_len == 1 as libc::c_int) {
        if not1 != 0 as libc::c_int && not2 != 0 as libc::c_int {
            r =
                and_code_range_buf(buf1, 0 as libc::c_int, buf2,
                                   0 as libc::c_int, &mut pbuf)
        } else {
            r = or_code_range_buf(enc, buf1, not1, buf2, not2, &mut pbuf);
            if r == 0 as libc::c_int && not1 != 0 as libc::c_int {
                let mut tbuf: *mut BBuf = 0 as *mut BBuf;
                r = not_code_range_buf(enc, pbuf, &mut tbuf);
                if r != 0 as libc::c_int { bbuf_free(pbuf); return r }
                bbuf_free(pbuf);
                pbuf = tbuf
            }
        }
        if r != 0 as libc::c_int { return r }
        (*dest).mbuf = pbuf;
        bbuf_free(buf1);
        return r
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn conv_backslash_value(mut c: OnigCodePoint,
                                          mut env: *mut ScanEnv)
 -> OnigCodePoint {
    if (*(*env).syntax).op & (1 as libc::c_uint) << 26 as libc::c_int !=
           0 as libc::c_int as libc::c_uint {
        match c {
            110 => { return '\n' as i32 as OnigCodePoint }
            116 => { return '\t' as i32 as OnigCodePoint }
            114 => { return '\r' as i32 as OnigCodePoint }
            102 => { return '\u{c}' as i32 as OnigCodePoint }
            97 => { return '\u{7}' as i32 as OnigCodePoint }
            98 => { return '\u{8}' as i32 as OnigCodePoint }
            101 => { return '\u{1b}' as i32 as OnigCodePoint }
            118 => {
                if (*(*env).syntax).op2 &
                       (1 as libc::c_uint) << 13 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                    return '\u{b}' as i32 as OnigCodePoint
                }
            }
            _ => { }
        }
    }
    return c;
}
unsafe extern "C" fn is_invalid_quantifier_target(mut node: *mut Node)
 -> libc::c_int {
    match (*node).u.base.type_0 {
        7 => { return 1 as libc::c_int }
        8 => {
            loop  {
                if is_invalid_quantifier_target((*node).u.cons.car) == 0 {
                    return 0 as libc::c_int
                }
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
            return 0 as libc::c_int
        }
        9 => {
            loop  {
                if is_invalid_quantifier_target((*node).u.cons.car) != 0 {
                    return 1 as libc::c_int
                }
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
        }
        6 | _ => { }
    }
    return 0 as libc::c_int;
}
/* ?:0, *:1, +:2, ??:3, *?:4, +?:5 */
unsafe extern "C" fn popular_quantifier_num(mut q: *mut QtfrNode)
 -> libc::c_int {
    if (*q).greedy != 0 {
        if (*q).lower == 0 as libc::c_int {
            if (*q).upper == 1 as libc::c_int {
                return 0 as libc::c_int
            } else {
                if (*q).upper == -(1 as libc::c_int) {
                    return 1 as libc::c_int
                }
            }
        } else if (*q).lower == 1 as libc::c_int {
            if (*q).upper == -(1 as libc::c_int) { return 2 as libc::c_int }
        }
    } else if (*q).lower == 0 as libc::c_int {
        if (*q).upper == 1 as libc::c_int {
            return 3 as libc::c_int
        } else {
            if (*q).upper == -(1 as libc::c_int) { return 4 as libc::c_int }
        }
    } else if (*q).lower == 1 as libc::c_int {
        if (*q).upper == -(1 as libc::c_int) { return 5 as libc::c_int }
    } /* "....{" : OK! */
    return -(1 as libc::c_int);
}
static mut ReduceTypeTable: [[ReduceType; 6]; 6] =
    [[RQ_DEL, RQ_A, RQ_A, RQ_QQ, RQ_AQ, RQ_ASIS],
     [RQ_DEL, RQ_DEL, RQ_DEL, RQ_P_QQ, RQ_P_QQ, RQ_DEL],
     [RQ_A, RQ_A, RQ_DEL, RQ_ASIS, RQ_P_QQ, RQ_DEL],
     [RQ_DEL, RQ_AQ, RQ_AQ, RQ_DEL, RQ_AQ, RQ_AQ],
     [RQ_DEL, RQ_DEL, RQ_DEL, RQ_DEL, RQ_DEL, RQ_DEL],
     [RQ_ASIS, RQ_PQ_Q, RQ_DEL, RQ_AQ, RQ_AQ, RQ_DEL]];
#[no_mangle]
pub unsafe extern "C" fn onig_reduce_nested_quantifier(mut pnode: *mut Node,
                                                       mut cnode: *mut Node) {
    let mut pnum: libc::c_int = 0;
    let mut cnum: libc::c_int = 0;
    let mut p: *mut QtfrNode = 0 as *mut QtfrNode;
    let mut c: *mut QtfrNode = 0 as *mut QtfrNode;
    p = &mut (*pnode).u.qtfr;
    c = &mut (*cnode).u.qtfr;
    pnum = popular_quantifier_num(p);
    cnum = popular_quantifier_num(c);
    if pnum < 0 as libc::c_int || cnum < 0 as libc::c_int { return }
    match ReduceTypeTable[cnum as usize][pnum as usize] as libc::c_uint {
        1 => { *pnode = *cnode }
        2 => {
            (*p).target = (*c).target;
            (*p).lower = 0 as libc::c_int;
            (*p).upper = -(1 as libc::c_int);
            (*p).greedy = 1 as libc::c_int
        }
        3 => {
            (*p).target = (*c).target;
            (*p).lower = 0 as libc::c_int;
            (*p).upper = -(1 as libc::c_int);
            (*p).greedy = 0 as libc::c_int
        }
        4 => {
            (*p).target = (*c).target;
            (*p).lower = 0 as libc::c_int;
            (*p).upper = 1 as libc::c_int;
            (*p).greedy = 0 as libc::c_int
        }
        5 => {
            (*p).target = cnode;
            (*p).lower = 0 as libc::c_int;
            (*p).upper = 1 as libc::c_int;
            (*p).greedy = 0 as libc::c_int;
            (*c).lower = 1 as libc::c_int;
            (*c).upper = -(1 as libc::c_int);
            (*c).greedy = 1 as libc::c_int;
            return
        }
        6 => {
            (*p).target = cnode;
            (*p).lower = 0 as libc::c_int;
            (*p).upper = 1 as libc::c_int;
            (*p).greedy = 1 as libc::c_int;
            (*c).lower = 1 as libc::c_int;
            (*c).upper = -(1 as libc::c_int);
            (*c).greedy = 0 as libc::c_int;
            return
        }
        0 => { (*p).target = cnode; return }
        _ => { }
    }
    (*c).target = 0 as *mut Node;
    onig_node_free(cnode);
}
unsafe extern "C" fn fetch_range_quantifier(mut src: *mut *mut OnigUChar,
                                            mut end: *mut OnigUChar,
                                            mut tok: *mut OnigToken,
                                            mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut current_block: u64;
    let mut low: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    let mut syn_allow: libc::c_int = 0;
    let mut non_low: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut c: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    syn_allow =
        ((*(*env).syntax).behavior & (1 as libc::c_uint) << 3 as libc::c_int
             != 0 as libc::c_int as libc::c_uint) as libc::c_int;
    if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
        if syn_allow != 0 {
            return 1 as libc::c_int
        } else { return -(100 as libc::c_int) }
        /* "....{" syntax error */
    }
    if syn_allow == 0 {
        c =
            if p < end {
                (*enc).mbc_to_code.expect("non-null function pointer")(p, end)
            } else { 0 as libc::c_int as libc::c_uint };
        if c == ')' as i32 as libc::c_uint || c == '(' as i32 as libc::c_uint
               || c == '|' as i32 as libc::c_uint {
            return -(100 as libc::c_int)
        }
    }
    low = onig_scan_unsigned_number(&mut p, end, (*env).enc);
    if low < 0 as libc::c_int { return -(201 as libc::c_int) }
    if low > 100000 as libc::c_int { return -(201 as libc::c_int) }
    if p == *src {
        /* can't read low */
        if (*(*env).syntax).behavior & (1 as libc::c_uint) << 4 as libc::c_int
               != 0 as libc::c_int as libc::c_uint {
            /* allow {,n} as {0,n} */
            low = 0 as libc::c_int;
            non_low = 1 as libc::c_int;
            current_block = 14576567515993809846;
        } else { current_block = 12401456848548100637; }
    } else { current_block = 14576567515993809846; }
    match current_block {
        14576567515993809846 => {
            if !(if p < end { 0 as libc::c_int } else { 1 as libc::c_int } !=
                     0) {
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if c == ',' as i32 as libc::c_uint {
                    let mut prev: *mut OnigUChar = p;
                    up = onig_scan_unsigned_number(&mut p, end, (*env).enc);
                    if up < 0 as libc::c_int { return -(201 as libc::c_int) }
                    if up > 100000 as libc::c_int {
                        return -(201 as libc::c_int)
                    }
                    if p == prev {
                        if non_low != 0 as libc::c_int {
                            current_block = 12401456848548100637;
                        } else {
                            up = -(1 as libc::c_int);
                            current_block = 15512526488502093901;
                        }
                        /* {n,} : {n,infinite} */
                    } else {
                        current_block =
                            15512526488502093901; /* {n} : exact n times */
                    }
                } else if non_low != 0 as libc::c_int {
                    current_block = 12401456848548100637;
                } else {
                    p = pfetch_prev;
                    up = low;
                    r = 2 as libc::c_int;
                    current_block = 15512526488502093901;
                }
                match current_block {
                    12401456848548100637 => { }
                    _ => {
                        if !(if p < end {
                                 0 as libc::c_int
                             } else { 1 as libc::c_int } != 0) {
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize);
                            if (*(*env).syntax).op &
                                   (1 as libc::c_uint) << 9 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                if c != (*(*env).syntax).meta_char_table.esc {
                                    current_block = 12401456848548100637;
                                } else {
                                    c =
                                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                               end);
                                    pfetch_prev = p;
                                    p =
                                        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                     as isize);
                                    current_block = 10753070352654377903;
                                }
                            } else { current_block = 10753070352654377903; }
                            match current_block {
                                12401456848548100637 => { }
                                _ => {
                                    if !(c != '}' as i32 as libc::c_uint) {
                                        if !(up == -(1 as libc::c_int)) &&
                                               low > up {
                                            return -(202 as libc::c_int)
                                        }
                                        (*tok).type_0 = TK_INTERVAL;
                                        (*tok).u.repeat.lower = low;
                                        (*tok).u.repeat.upper = up;
                                        *src = p;
                                        /* fixed */
                                        return r
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    } /* 0: normal {n,m}, 2: fixed {n} */
    if syn_allow != 0 {
        /* *src = p; */
        /* !!! Don't do this line !!! */
        return 1 as libc::c_int
        /* OK */
    } else { return -(123 as libc::c_int) };
}
/* \M-, \C-, \c, or \... */
unsafe extern "C" fn fetch_escaped_value(mut src: *mut *mut OnigUChar,
                                         mut end: *mut OnigUChar,
                                         mut env: *mut ScanEnv,
                                         mut val: *mut OnigCodePoint)
 -> libc::c_int {
    let mut current_block: u64;
    let mut v: libc::c_int = 0;
    let mut c: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut p: *mut OnigUChar = *src;
    if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
        return -(104 as libc::c_int)
    }
    c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
    p =
        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p) as
                     isize);
    match c {
        77 => {
            if (*(*env).syntax).op2 & (1 as libc::c_uint) << 12 as libc::c_int
                   != 0 as libc::c_int as libc::c_uint {
                if if p < end { 0 as libc::c_int } else { 1 as libc::c_int }
                       != 0 {
                    return -(105 as libc::c_int)
                }
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if c != '-' as i32 as libc::c_uint {
                    return -(108 as libc::c_int)
                }
                if if p < end { 0 as libc::c_int } else { 1 as libc::c_int }
                       != 0 {
                    return -(105 as libc::c_int)
                }
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if c == (*(*env).syntax).meta_char_table.esc {
                    v = fetch_escaped_value(&mut p, end, env, &mut c);
                    if v < 0 as libc::c_int { return v }
                }
                c =
                    c & 0xff as libc::c_int as libc::c_uint |
                        0x80 as libc::c_int as libc::c_uint;
                current_block = 168769493162332264;
            } else { current_block = 7187160828046810477; }
        }
        67 => {
            if (*(*env).syntax).op2 & (1 as libc::c_uint) << 11 as libc::c_int
                   != 0 as libc::c_int as libc::c_uint {
                if if p < end { 0 as libc::c_int } else { 1 as libc::c_int }
                       != 0 {
                    return -(106 as libc::c_int)
                }
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if c != '-' as i32 as libc::c_uint {
                    return -(109 as libc::c_int)
                }
                current_block = 17584930885384997743;
            } else { current_block = 7187160828046810477; }
        }
        99 => {
            if (*(*env).syntax).op & (1 as libc::c_uint) << 27 as libc::c_int
                   != 0 as libc::c_int as libc::c_uint {
                current_block = 17584930885384997743;
            } else { current_block = 7187160828046810477; }
        }
        _ => { current_block = 7187160828046810477; }
    }
    match current_block {
        17584930885384997743 => {
            if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0
               {
                return -(106 as libc::c_int)
            }
            c =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if c == '?' as i32 as libc::c_uint {
                c = 0o177 as libc::c_int as OnigCodePoint
            } else {
                if c == (*(*env).syntax).meta_char_table.esc {
                    v = fetch_escaped_value(&mut p, end, env, &mut c);
                    if v < 0 as libc::c_int { return v }
                }
                c &= 0x9f as libc::c_int as libc::c_uint
            }
        }
        7187160828046810477 =>
        /* fall through */
        {
            c = conv_backslash_value(c, env)
        }
        _ => { }
    }
    *src = p;
    *val = c;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_name_end_code_point(mut start: OnigCodePoint)
 -> OnigCodePoint {
    match start {
        60 => { return '>' as i32 as OnigCodePoint }
        39 => { return '\'' as i32 as OnigCodePoint }
        _ => { }
    }
    return 0 as libc::c_int as OnigCodePoint;
}
/*
   \k<name+n>, \k<name-n>
   \k<num+n>,  \k<num-n>
   \k<-num+n>, \k<-num-n>
*/
unsafe extern "C" fn fetch_name_with_level(mut start_code: OnigCodePoint,
                                           mut src: *mut *mut OnigUChar,
                                           mut end: *mut OnigUChar,
                                           mut rname_end: *mut *mut OnigUChar,
                                           mut env: *mut ScanEnv,
                                           mut rback_num: *mut libc::c_int,
                                           mut rlevel: *mut libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut is_num: libc::c_int = 0;
    let mut exist_level: libc::c_int = 0;
    let mut end_code: OnigCodePoint = 0;
    let mut c: OnigCodePoint = 0 as libc::c_int as OnigCodePoint;
    let mut enc: OnigEncoding = (*env).enc;
    let mut name_end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut pnum_head: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    *rback_num = 0 as libc::c_int;
    exist_level = 0 as libc::c_int;
    is_num = exist_level;
    sign = 1 as libc::c_int;
    pnum_head = *src;
    end_code = get_name_end_code_point(start_code);
    name_end = end;
    r = 0 as libc::c_int;
    if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
        return -(214 as libc::c_int)
    } else {
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if c == end_code { return -(214 as libc::c_int) }
        if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                    4 as
                                                                        libc::c_int
                                                                        as
                                                                        OnigCtype)
               != 0 {
            is_num = 1 as libc::c_int
        } else if c == '-' as i32 as libc::c_uint {
            is_num = 2 as libc::c_int;
            sign = -(1 as libc::c_int);
            pnum_head = p
        } else if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                           12
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               OnigCtype)
                      == 0 {
            r = -(216 as libc::c_int)
        }
    }
    while if p < end { 0 as libc::c_int } else { 1 as libc::c_int } == 0 {
        name_end = p;
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if c == end_code || c == ')' as i32 as libc::c_uint ||
               c == '+' as i32 as libc::c_uint ||
               c == '-' as i32 as libc::c_uint {
            if is_num == 2 as libc::c_int { r = -(215 as libc::c_int) }
            break ;
        } else if is_num != 0 as libc::c_int {
            if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                        4 as
                                                                            libc::c_int
                                                                            as
                                                                            OnigCtype)
                   != 0 {
                is_num = 1 as libc::c_int
            } else { r = -(215 as libc::c_int); is_num = 0 as libc::c_int }
        } else if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                           12
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               OnigCtype)
                      == 0 {
            r = -(216 as libc::c_int)
        }
    }
    if r == 0 as libc::c_int && c != end_code {
        if c == '+' as i32 as libc::c_uint || c == '-' as i32 as libc::c_uint
           {
            let mut level: libc::c_int = 0;
            let mut flag: libc::c_int =
                if c == '-' as i32 as libc::c_uint {
                    -(1 as libc::c_int)
                } else { 1 as libc::c_int };
            if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0
               {
                r = -(216 as libc::c_int);
                current_block = 10660635924059941024;
            } else {
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                            4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                OnigCtype)
                       == 0 {
                    current_block = 8719833738645569545;
                } else {
                    p = pfetch_prev;
                    level = onig_scan_unsigned_number(&mut p, end, enc);
                    if level < 0 as libc::c_int {
                        return -(200 as libc::c_int)
                    }
                    *rlevel = level * flag;
                    exist_level = 1 as libc::c_int;
                    if if p < end {
                           0 as libc::c_int
                       } else { 1 as libc::c_int } == 0 {
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == end_code {
                            current_block = 10660635924059941024;
                        } else { current_block = 8719833738645569545; }
                    } else { current_block = 8719833738645569545; }
                }
            }
        } else { current_block = 8719833738645569545; }
    } else { current_block = 10660635924059941024; }
    loop  {
        match current_block {
            10660635924059941024 => {
                if r == 0 as libc::c_int {
                    if !(is_num != 0 as libc::c_int) { break ; }
                    *rback_num =
                        onig_scan_unsigned_number(&mut pnum_head, name_end,
                                                  enc);
                    if *rback_num < 0 as libc::c_int {
                        return -(200 as libc::c_int)
                    } else {
                        if *rback_num == 0 as libc::c_int {
                            current_block = 8719833738645569545;
                            continue ;
                        }
                        *rback_num *= sign;
                        break ;
                    }
                } else {
                    onig_scan_env_set_error_string(env, r, *src, name_end);
                    return r
                }
            }
            _ => {
                r = -(215 as libc::c_int);
                name_end = end;
                current_block = 10660635924059941024;
            }
        }
    }
    *rname_end = name_end;
    *src = p;
    return if exist_level != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
/* USE_BACKREF_WITH_LEVEL */
/*
  ref: 0 -> define name    (don't allow number name)
       1 -> reference name (allow number name)
*/
unsafe extern "C" fn fetch_name(mut start_code: OnigCodePoint,
                                mut src: *mut *mut OnigUChar,
                                mut end: *mut OnigUChar,
                                mut rname_end: *mut *mut OnigUChar,
                                mut env: *mut ScanEnv,
                                mut rback_num: *mut libc::c_int,
                                mut ref_0: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut is_num: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut end_code: OnigCodePoint = 0;
    let mut c: OnigCodePoint = 0 as libc::c_int as OnigCodePoint;
    let mut enc: OnigEncoding = (*env).enc;
    let mut name_end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut pnum_head: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = *src;
    *rback_num = 0 as libc::c_int;
    end_code = get_name_end_code_point(start_code);
    name_end = end;
    pnum_head = *src;
    r = 0 as libc::c_int;
    is_num = 0 as libc::c_int;
    sign = 1 as libc::c_int;
    if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
        return -(214 as libc::c_int)
    } else {
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if c == end_code { return -(214 as libc::c_int) }
        if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                    4 as
                                                                        libc::c_int
                                                                        as
                                                                        OnigCtype)
               != 0 {
            if ref_0 == 1 as libc::c_int {
                is_num = 1 as libc::c_int
            } else { r = -(215 as libc::c_int); is_num = 0 as libc::c_int }
        } else if c == '-' as i32 as libc::c_uint {
            if ref_0 == 1 as libc::c_int {
                is_num = 2 as libc::c_int;
                sign = -(1 as libc::c_int);
                pnum_head = p
            } else { r = -(215 as libc::c_int); is_num = 0 as libc::c_int }
        } else if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                           12
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               OnigCtype)
                      == 0 {
            r = -(216 as libc::c_int)
        }
    }
    let mut current_block_79: u64;
    if r == 0 as libc::c_int {
        while if p < end { 0 as libc::c_int } else { 1 as libc::c_int } == 0 {
            name_end = p;
            c =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if c == end_code || c == ')' as i32 as libc::c_uint {
                if is_num == 2 as libc::c_int { r = -(215 as libc::c_int) }
                break ;
            } else if is_num != 0 as libc::c_int {
                if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                            4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                OnigCtype)
                       != 0 {
                    is_num = 1 as libc::c_int
                } else {
                    if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                                12
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    OnigCtype)
                           == 0 {
                        r = -(216 as libc::c_int)
                    } else { r = -(215 as libc::c_int) }
                    is_num = 0 as libc::c_int
                }
            } else if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                               12
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   OnigCtype)
                          == 0 {
                r = -(216 as libc::c_int)
            }
        }
        if c != end_code { r = -(215 as libc::c_int); name_end = end }
        if is_num != 0 as libc::c_int {
            *rback_num =
                onig_scan_unsigned_number(&mut pnum_head, name_end, enc);
            if *rback_num < 0 as libc::c_int {
                return -(200 as libc::c_int)
            } else if *rback_num == 0 as libc::c_int {
                r = -(215 as libc::c_int);
                current_block_79 = 3139427963964462121;
            } else {
                *rback_num *= sign;
                current_block_79 = 7178192492338286402;
            }
        } else { current_block_79 = 7178192492338286402; }
        match current_block_79 {
            3139427963964462121 => { }
            _ => { *rname_end = name_end; *src = p; return 0 as libc::c_int }
        }
    } else {
        while if p < end { 0 as libc::c_int } else { 1 as libc::c_int } == 0 {
            name_end = p;
            c =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if c == end_code || c == ')' as i32 as libc::c_uint { break ; }
        }
        if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
            name_end = end
        }
    }
    onig_scan_env_set_error_string(env, r, *src, name_end);
    return r;
}
/* USE_NAMED_GROUP */
unsafe extern "C" fn CC_ESC_WARN(mut env: *mut ScanEnv,
                                 mut c: *mut OnigUChar) {
    if onig_warn ==
           Some(onig_null_warn as
                    unsafe extern "C" fn(_: *const libc::c_char) -> ()) {
        return
    }
    if (*(*env).syntax).behavior & (1 as libc::c_uint) << 24 as libc::c_int !=
           0 as libc::c_int as libc::c_uint &&
           (*(*env).syntax).behavior &
               (1 as libc::c_uint) << 21 as libc::c_int !=
               0 as libc::c_int as libc::c_uint {
        let mut buf: [OnigUChar; 256] = [0; 256];
        onig_snprintf_with_pattern(buf.as_mut_ptr(), 256 as libc::c_int,
                                   (*env).enc, (*env).pattern,
                                   (*env).pattern_end,
                                   b"character class has \'%s\' without escape\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut OnigUChar, c);
        Some(onig_warn.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr()
                                                                                                    as
                                                                                                    *mut libc::c_char);
    };
}
unsafe extern "C" fn CLOSE_BRACKET_WITHOUT_ESC_WARN(mut env: *mut ScanEnv,
                                                    mut c: *mut OnigUChar) {
    if onig_warn ==
           Some(onig_null_warn as
                    unsafe extern "C" fn(_: *const libc::c_char) -> ()) {
        return
    }
    if (*(*env).syntax).behavior & (1 as libc::c_uint) << 24 as libc::c_int !=
           0 as libc::c_int as libc::c_uint {
        let mut buf: [OnigUChar; 256] = [0; 256];
        onig_snprintf_with_pattern(buf.as_mut_ptr(), 256 as libc::c_int,
                                   (*env).enc, (*env).pattern,
                                   (*env).pattern_end,
                                   b"regular expression has \'%s\' without escape\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut OnigUChar, c);
        Some(onig_warn.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr()
                                                                                                    as
                                                                                                    *mut libc::c_char);
    };
}
unsafe extern "C" fn find_str_position(mut s: *mut OnigCodePoint,
                                       mut n: libc::c_int,
                                       mut from: *mut OnigUChar,
                                       mut to: *mut OnigUChar,
                                       mut next: *mut *mut OnigUChar,
                                       mut enc: OnigEncoding)
 -> *mut OnigUChar {
    let mut i: libc::c_int = 0;
    let mut x: OnigCodePoint = 0;
    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = from;
    while p < to {
        x = (*enc).mbc_to_code.expect("non-null function pointer")(p, to);
        q =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if x == *s.offset(0 as libc::c_int as isize) {
            i = 1 as libc::c_int;
            while i < n && q < to {
                x =
                    (*enc).mbc_to_code.expect("non-null function pointer")(q,
                                                                           to);
                if x != *s.offset(i as isize) { break ; }
                q =
                    q.offset((*enc).mbc_enc_len.expect("non-null function pointer")(q)
                                 as isize);
                i += 1
            }
            if i >= n {
                if !(next as *mut libc::c_void).is_null() { *next = q }
                return p
            }
        }
        p = q
    }
    return 0 as *mut OnigUChar;
}
unsafe extern "C" fn str_exist_check_with_esc(mut s: *mut OnigCodePoint,
                                              mut n: libc::c_int,
                                              mut from: *mut OnigUChar,
                                              mut to: *mut OnigUChar,
                                              mut bad: OnigCodePoint,
                                              mut enc: OnigEncoding,
                                              mut syn: *mut OnigSyntaxType)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut in_esc: libc::c_int = 0;
    let mut x: OnigCodePoint = 0;
    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = from;
    in_esc = 0 as libc::c_int;
    while p < to {
        if in_esc != 0 {
            in_esc = 0 as libc::c_int;
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize)
        } else {
            x = (*enc).mbc_to_code.expect("non-null function pointer")(p, to);
            q =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if x == *s.offset(0 as libc::c_int as isize) {
                i = 1 as libc::c_int;
                while i < n && q < to {
                    x =
                        (*enc).mbc_to_code.expect("non-null function pointer")(q,
                                                                               to);
                    if x != *s.offset(i as isize) { break ; }
                    q =
                        q.offset((*enc).mbc_enc_len.expect("non-null function pointer")(q)
                                     as isize);
                    i += 1
                }
                if i >= n { return 1 as libc::c_int }
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize)
            } else {
                x =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           to);
                if x == bad {
                    return 0 as libc::c_int
                } else {
                    if x == (*syn).meta_char_table.esc {
                        in_esc = 1 as libc::c_int
                    }
                }
                p = q
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fetch_token_in_cc(mut tok: *mut OnigToken,
                                       mut src: *mut *mut OnigUChar,
                                       mut end: *mut OnigUChar,
                                       mut env: *mut ScanEnv) -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut c: OnigCodePoint = 0;
    let mut c2: OnigCodePoint = 0;
    let mut syn: *mut OnigSyntaxType = (*env).syntax;
    let mut enc: OnigEncoding = (*env).enc;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
        (*tok).type_0 = TK_EOT;
        return (*tok).type_0 as libc::c_int
    }
    c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
    pfetch_prev = p;
    p =
        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p) as
                     isize);
    (*tok).type_0 = TK_CHAR;
    (*tok).base = 0 as libc::c_int;
    (*tok).u.c = c as libc::c_int;
    (*tok).escaped = 0 as libc::c_int;
    if c == ']' as i32 as libc::c_uint {
        (*tok).type_0 = TK_CC_CLOSE
    } else if c == '-' as i32 as libc::c_uint {
        (*tok).type_0 = TK_CC_RANGE
    } else if c == (*syn).meta_char_table.esc {
        if (*syn).behavior & (1 as libc::c_uint) << 21 as libc::c_int !=
               0 as libc::c_int as libc::c_uint {
            if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0
               {
                return -(104 as libc::c_int)
            }
            c =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            pfetch_prev = p;
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            (*tok).escaped = 1 as libc::c_int;
            (*tok).u.c = c as libc::c_int;
            match c {
                119 => {
                    (*tok).type_0 = TK_CHAR_TYPE;
                    (*tok).u.prop.ctype = 12 as libc::c_int;
                    (*tok).u.prop.not = 0 as libc::c_int
                }
                87 => {
                    (*tok).type_0 = TK_CHAR_TYPE;
                    (*tok).u.prop.ctype = 12 as libc::c_int;
                    (*tok).u.prop.not = 1 as libc::c_int
                }
                100 => {
                    (*tok).type_0 = TK_CHAR_TYPE;
                    (*tok).u.prop.ctype = 4 as libc::c_int;
                    (*tok).u.prop.not = 0 as libc::c_int
                }
                68 => {
                    (*tok).type_0 = TK_CHAR_TYPE;
                    (*tok).u.prop.ctype = 4 as libc::c_int;
                    (*tok).u.prop.not = 1 as libc::c_int
                }
                115 => {
                    (*tok).type_0 = TK_CHAR_TYPE;
                    (*tok).u.prop.ctype = 9 as libc::c_int;
                    (*tok).u.prop.not = 0 as libc::c_int
                }
                83 => {
                    (*tok).type_0 = TK_CHAR_TYPE;
                    (*tok).u.prop.ctype = 9 as libc::c_int;
                    (*tok).u.prop.not = 1 as libc::c_int
                }
                104 => {
                    if (*syn).op2 & (1 as libc::c_uint) << 19 as libc::c_int
                           != 0 as libc::c_int as libc::c_uint {
                        (*tok).type_0 = TK_CHAR_TYPE;
                        (*tok).u.prop.ctype = 11 as libc::c_int;
                        (*tok).u.prop.not = 0 as libc::c_int
                    }
                }
                72 => {
                    if (*syn).op2 & (1 as libc::c_uint) << 19 as libc::c_int
                           != 0 as libc::c_int as libc::c_uint {
                        (*tok).type_0 = TK_CHAR_TYPE;
                        (*tok).u.prop.ctype = 11 as libc::c_int;
                        (*tok).u.prop.not = 1 as libc::c_int
                    }
                }
                112 | 80 => {
                    if !(if p < end {
                             0 as libc::c_int
                         } else { 1 as libc::c_int } != 0) {
                        c2 =
                            if p < end {
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end)
                            } else { 0 as libc::c_int as libc::c_uint };
                        if c2 == '{' as i32 as libc::c_uint &&
                               (*syn).op2 &
                                   (1 as libc::c_uint) << 16 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize);
                            (*tok).type_0 = TK_CHAR_PROPERTY;
                            (*tok).u.prop.not =
                                if c == 'P' as i32 as libc::c_uint {
                                    1 as libc::c_int
                                } else { 0 as libc::c_int };
                            if (if p < end {
                                    0 as libc::c_int
                                } else { 1 as libc::c_int }) == 0 &&
                                   (*syn).op2 &
                                       (1 as libc::c_uint) <<
                                           17 as libc::c_int !=
                                       0 as libc::c_int as libc::c_uint {
                                c2 =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c2 == '^' as i32 as libc::c_uint {
                                    (*tok).u.prop.not =
                                        if (*tok).u.prop.not ==
                                               0 as libc::c_int {
                                            1 as libc::c_int
                                        } else { 0 as libc::c_int }
                                } else { p = pfetch_prev }
                            }
                        }
                    }
                }
                120 => {
                    if !(if p < end {
                             0 as libc::c_int
                         } else { 1 as libc::c_int } != 0) {
                        prev = p;
                        if (if p < end {
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end)
                            } else { 0 as libc::c_int as libc::c_uint }) ==
                               '{' as i32 as OnigCodePoint &&
                               (*syn).op &
                                   (1 as libc::c_uint) << 30 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize);
                            num =
                                scan_unsigned_hexadecimal_number(&mut p, end,
                                                                 8 as
                                                                     libc::c_int,
                                                                 enc);
                            if num < 0 as libc::c_int {
                                return -(401 as libc::c_int)
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } == 0 {
                                c2 =
                                    if p < end {
                                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                               end)
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    };
                                if (*enc).is_code_ctype.expect("non-null function pointer")(c2,
                                                                                            11
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                OnigCtype)
                                       != 0 {
                                    return -(212 as libc::c_int)
                                }
                            }
                            if p >
                                   prev.offset((*enc).mbc_enc_len.expect("non-null function pointer")(prev)
                                                   as isize) &&
                                   (if p < end {
                                        0 as libc::c_int
                                    } else { 1 as libc::c_int }) == 0 &&
                                   (if p < end {
                                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                               end)
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) == '}' as i32 as OnigCodePoint {
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                (*tok).type_0 = TK_CODE_POINT;
                                (*tok).base = 16 as libc::c_int;
                                (*tok).u.code = num as OnigCodePoint
                            } else {
                                /* can't read nothing or invalid format */
                                p = prev
                            }
                        } else if (*syn).op &
                                      (1 as libc::c_uint) << 29 as libc::c_int
                                      != 0 as libc::c_int as libc::c_uint {
                            num =
                                scan_unsigned_hexadecimal_number(&mut p, end,
                                                                 2 as
                                                                     libc::c_int,
                                                                 enc);
                            if num < 0 as libc::c_int {
                                return -(200 as libc::c_int)
                            }
                            if p == prev {
                                /* can't read nothing. */
                                num = 0 as libc::c_int
                                /* but, it's not error */
                            }
                            (*tok).type_0 = TK_RAW_BYTE;
                            (*tok).base = 16 as libc::c_int;
                            (*tok).u.c = num
                        }
                    }
                }
                117 => {
                    if !(if p < end {
                             0 as libc::c_int
                         } else { 1 as libc::c_int } != 0) {
                        prev = p;
                        if (*syn).op2 &
                               (1 as libc::c_uint) << 14 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            num =
                                scan_unsigned_hexadecimal_number(&mut p, end,
                                                                 4 as
                                                                     libc::c_int,
                                                                 enc);
                            if num < 0 as libc::c_int {
                                return -(200 as libc::c_int)
                            }
                            if p == prev {
                                /* can't read nothing. */
                                num = 0 as libc::c_int
                                /* but, it's not error */
                            }
                            (*tok).type_0 = TK_CODE_POINT;
                            (*tok).base = 16 as libc::c_int;
                            (*tok).u.code = num as OnigCodePoint
                        }
                    }
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    if (*syn).op & (1 as libc::c_uint) << 28 as libc::c_int !=
                           0 as libc::c_int as libc::c_uint {
                        p = pfetch_prev;
                        prev = p;
                        num =
                            scan_unsigned_octal_number(&mut p, end,
                                                       3 as libc::c_int, enc);
                        if num < 0 as libc::c_int {
                            return -(200 as libc::c_int)
                        }
                        if p == prev {
                            /* can't read nothing. */
                            num = 0 as libc::c_int
                            /* but, it's not error */
                        } /* point at '[' is read */
                        (*tok).type_0 = TK_RAW_BYTE;
                        (*tok).base = 8 as libc::c_int;
                        (*tok).u.c = num
                    }
                }
                _ => {
                    p = pfetch_prev;
                    num = fetch_escaped_value(&mut p, end, env, &mut c2);
                    if num < 0 as libc::c_int { return num }
                    if (*tok).u.c as libc::c_uint != c2 {
                        (*tok).u.code = c2;
                        (*tok).type_0 = TK_CODE_POINT
                    }
                }
            }
        }
    } else if c == '[' as i32 as libc::c_uint {
        let mut current_block_153: u64;
        if (*syn).op & (1 as libc::c_uint) << 24 as libc::c_int !=
               0 as libc::c_int as libc::c_uint &&
               (if p < end {
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end)
                } else { 0 as libc::c_int as libc::c_uint }) ==
                   ':' as i32 as OnigCodePoint {
            let mut send: [OnigCodePoint; 2] =
                [':' as i32 as OnigCodePoint, ']' as i32 as OnigCodePoint];
            (*tok).backp = p;
            pfetch_prev = p;
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if str_exist_check_with_esc(send.as_mut_ptr(), 2 as libc::c_int,
                                        p, end, ']' as i32 as OnigCodePoint,
                                        enc, syn) != 0 {
                (*tok).type_0 = TK_POSIX_BRACKET_OPEN;
                current_block_153 = 14841185953239992141;
            } else {
                p = pfetch_prev;
                current_block_153 = 11867115373695062341;
            }
        } else { current_block_153 = 11867115373695062341; }
        match current_block_153 {
            11867115373695062341 => {
                if (*syn).op2 & (1 as libc::c_uint) << 6 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                    (*tok).type_0 = TK_CC_CC_OPEN
                } else {
                    CC_ESC_WARN(env,
                                b"[\x00" as *const u8 as *const libc::c_char
                                    as *mut OnigUChar);
                }
            }
            _ => { }
        }
    } else if c == '&' as i32 as libc::c_uint {
        if (*syn).op2 & (1 as libc::c_uint) << 6 as libc::c_int !=
               0 as libc::c_int as libc::c_uint &&
               (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) ==
                   0 &&
               (if p < end {
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end)
                } else { 0 as libc::c_int as libc::c_uint }) ==
                   '&' as i32 as OnigCodePoint {
            pfetch_prev = p;
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            (*tok).type_0 = TK_CC_AND
        }
    }
    *src = p;
    return (*tok).type_0 as libc::c_int;
}
unsafe extern "C" fn fetch_token(mut tok: *mut OnigToken,
                                 mut src: *mut *mut OnigUChar,
                                 mut end: *mut OnigUChar,
                                 mut env: *mut ScanEnv) -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut c: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut syn: *mut OnigSyntaxType = (*env).syntax;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    loop  {
        if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
            (*tok).type_0 = TK_EOT;
            return (*tok).type_0 as libc::c_int
        }
        (*tok).type_0 = TK_STRING;
        (*tok).base = 0 as libc::c_int;
        (*tok).backp = p;
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if c == (*syn).meta_char_table.esc &&
               !((*syn).op2 & (1 as libc::c_uint) << 20 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
            if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0
               {
                return -(104 as libc::c_int)
            }
            (*tok).backp = p;
            c =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            pfetch_prev = p;
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            (*tok).u.c = c as libc::c_int;
            (*tok).escaped = 1 as libc::c_int;
            match c {
                42 => { current_block = 16372213405346143698; break ; }
                43 => { current_block = 12740818325037147325; break ; }
                63 => { current_block = 11980851012772541874; break ; }
                123 => { current_block = 297784347553452316; break ; }
                124 => { current_block = 18258225697907100628; break ; }
                40 => { current_block = 15904406811757377787; break ; }
                41 => { current_block = 3494171831578997035; break ; }
                119 => { current_block = 16873236124228232483; break ; }
                87 => { current_block = 5674466540670887041; break ; }
                98 => { current_block = 8412118167407665105; break ; }
                66 => { current_block = 1898993302553076133; break ; }
                60 => { current_block = 17673280890339436511; break ; }
                62 => { current_block = 9325485969395579449; break ; }
                115 => { current_block = 3009217617782060917; break ; }
                83 => { current_block = 15453639749100233674; break ; }
                100 => { current_block = 7230447043056047703; break ; }
                68 => { current_block = 15952717276592120294; break ; }
                104 => { current_block = 9842434449443674853; break ; }
                72 => { current_block = 9636888125699845498; break ; }
                65 => { current_block = 8235112133255751888; break ; }
                90 => { current_block = 4158919382095016298; break ; }
                122 => { current_block = 14131958400073952123; break ; }
                71 => { current_block = 4724803196040624734; break ; }
                96 => { current_block = 15570335776229543684; break ; }
                39 => { current_block = 7455596087214207069; break ; }
                120 => { current_block = 8540457983839371850; break ; }
                117 => { current_block = 3334119875212657085; break ; }
                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 6740850349075033367;
                    break ;
                }
                48 => { current_block = 2916120083628436309; break ; }
                107 => { current_block = 14906809932406725840; break ; }
                103 => { current_block = 8048257192757430128; break ; }
                81 => { current_block = 4874723077730206021; break ; }
                112 | 80 => { current_block = 13228428744258393755; break ; }
                _ => { current_block = 6398212124985529790; break ; }
            }
        } else {
            (*tok).u.c = c as libc::c_int;
            (*tok).escaped = 0 as libc::c_int;
            if c != 0 as libc::c_int as libc::c_uint &&
                   (*syn).op & (1 as libc::c_uint) << 0 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                if c == (*syn).meta_char_table.anychar {
                    current_block = 15112282366752052665;
                    break ;
                }
                if c == (*syn).meta_char_table.anytime {
                    current_block = 17380491634721104092;
                    break ;
                }
                if c == (*syn).meta_char_table.zero_or_one_time {
                    current_block = 12824708355191203993;
                    break ;
                }
                if c == (*syn).meta_char_table.one_or_more_time {
                    current_block = 10905486111603547446;
                    break ;
                }
                if c == (*syn).meta_char_table.anychar_anytime {
                    (*tok).type_0 = TK_ANYCHAR_ANYTIME;
                    current_block = 11687719283784080910;
                    break ;
                }
            }
            match c {
                46 => {
                    if !((*syn).op & (1 as libc::c_uint) << 1 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 15112282366752052665; break ; }
                }
                42 => {
                    if !((*syn).op & (1 as libc::c_uint) << 2 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 17380491634721104092; break ; }
                }
                43 => {
                    if !((*syn).op & (1 as libc::c_uint) << 4 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 10905486111603547446; break ; }
                }
                63 => {
                    if !((*syn).op & (1 as libc::c_uint) << 6 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 12824708355191203993; break ; }
                }
                123 => {
                    if !((*syn).op & (1 as libc::c_uint) << 8 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 4713212954604576497; break ; }
                }
                124 => {
                    if !((*syn).op & (1 as libc::c_uint) << 10 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 8478868103403534984; break ; }
                }
                40 => {
                    if !((if p < end {
                              0 as libc::c_int
                          } else { 1 as libc::c_int }) == 0 &&
                             (if p < end {
                                  (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                         end)
                              } else { 0 as libc::c_int as libc::c_uint }) ==
                                 '?' as i32 as OnigCodePoint &&
                             (*syn).op2 &
                                 (1 as libc::c_uint) << 1 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                        current_block = 9416928054198617439;
                        break ;
                    }
                    pfetch_prev = p;
                    p =
                        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                     as isize);
                    if (if p < end {
                            0 as libc::c_int
                        } else { 1 as libc::c_int }) == 0 &&
                           (if p < end {
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end)
                            } else { 0 as libc::c_int as libc::c_uint }) ==
                               '#' as i32 as OnigCodePoint {
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        loop  {
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize);
                            if c == (*syn).meta_char_table.esc {
                                if if p < end {
                                       0 as libc::c_int
                                   } else { 1 as libc::c_int } == 0 {
                                    c =
                                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                               end);
                                    pfetch_prev = p;
                                    p =
                                        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                     as isize)
                                }
                            } else if c == ')' as i32 as libc::c_uint {
                                break ;
                            }
                        }
                    } else {
                        p = pfetch_prev;
                        current_block = 9416928054198617439;
                        break ;
                    }
                }
                41 => {
                    if !((*syn).op & (1 as libc::c_uint) << 12 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 15849589987095405551; break ; }
                }
                94 => {
                    if !((*syn).op & (1 as libc::c_uint) << 23 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 17058499098102203106; break ; }
                }
                36 => {
                    if !((*syn).op & (1 as libc::c_uint) << 23 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 4544312254017297231; break ; }
                }
                91 => {
                    if !((*syn).op & (1 as libc::c_uint) << 17 as libc::c_int
                             != 0 as libc::c_int as libc::c_uint) {
                        current_block = 11687719283784080910;
                        break ;
                    } else { current_block = 11297176373744872492; break ; }
                }
                93 => {
                    if *src > (*env).pattern {
                        /* /].../ is allowed. */
                        CLOSE_BRACKET_WITHOUT_ESC_WARN(env,
                                                       b"]\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut OnigUChar); /* error */
                    }
                    current_block = 11687719283784080910;
                    break ;
                }
                35 => {
                    if !((*env).option &
                             (1 as libc::c_uint) << 1 as libc::c_int != 0) {
                        current_block = 11687719283784080910;
                        break ;
                    }
                    while if p < end {
                              0 as libc::c_int
                          } else { 1 as libc::c_int } == 0 {
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if (*enc).is_code_ctype.expect("non-null function pointer")(c,
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        OnigCtype)
                               != 0 {
                            break ;
                        }
                    }
                }
                32 | 9 | 10 | 13 | 12 => {
                    if !((*env).option &
                             (1 as libc::c_uint) << 1 as libc::c_int != 0) {
                        current_block = 11687719283784080910;
                        break ;
                    }
                }
                _ => { current_block = 11687719283784080910; break ; }
            }
        }
    }
    match current_block {
        4713212954604576497 => {
            r = fetch_range_quantifier(&mut p, end, tok, env);
            if r < 0 as libc::c_int { return r }
            if r == 0 as libc::c_int {
                current_block = 10131319541265114523;
            } else if r == 2 as libc::c_int {
                /* {n} */
                if (*syn).behavior & (1 as libc::c_uint) << 9 as libc::c_int
                       != 0 as libc::c_int as libc::c_uint {
                    current_block = 6765407519951034582;
                } else { current_block = 10131319541265114523; }
            } else {
                /* r == 1 : normal char */
                current_block = 11687719283784080910;
            }
        }
        12824708355191203993 => {
            (*tok).type_0 = TK_OP_REPEAT;
            (*tok).u.repeat.lower = 0 as libc::c_int;
            (*tok).u.repeat.upper = 1 as libc::c_int;
            current_block = 10131319541265114523;
        }
        9416928054198617439 => {
            if !((*syn).op & (1 as libc::c_uint) << 12 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_SUBEXP_OPEN;
                current_block = 11687719283784080910;
            }
        }
        10905486111603547446 => {
            (*tok).type_0 = TK_OP_REPEAT;
            (*tok).u.repeat.lower = 1 as libc::c_int;
            (*tok).u.repeat.upper = -(1 as libc::c_int);
            current_block = 10131319541265114523;
        }
        17380491634721104092 => {
            (*tok).type_0 = TK_OP_REPEAT;
            (*tok).u.repeat.lower = 0 as libc::c_int;
            (*tok).u.repeat.upper = -(1 as libc::c_int);
            current_block = 10131319541265114523;
        }
        6740850349075033367 => {
            p = pfetch_prev;
            prev = p;
            num = onig_scan_unsigned_number(&mut p, end, enc);
            if num < 0 as libc::c_int || num > 1000 as libc::c_int {
                current_block = 4936445733123251449;
            } else if (*syn).op & (1 as libc::c_uint) << 16 as libc::c_int !=
                          0 as libc::c_int as libc::c_uint &&
                          (num <= (*env).num_mem || num <= 9 as libc::c_int) {
                /* This spec. from GNU regex */
                if (*syn).behavior & (1 as libc::c_uint) << 5 as libc::c_int
                       != 0 as libc::c_int as libc::c_uint {
                    if num > (*env).num_mem ||
                           (*(if !((*env).mem_nodes_dynamic as
                                       *mut libc::c_void).is_null() {
                                  (*env).mem_nodes_dynamic
                              } else {
                                  (*env).mem_nodes_static.as_mut_ptr()
                              }).offset(num as isize) as
                                *mut libc::c_void).is_null() {
                        return -(208 as libc::c_int)
                    }
                }
                (*tok).type_0 = TK_BACKREF;
                (*tok).u.backref.num = 1 as libc::c_int;
                (*tok).u.backref.ref1 = num;
                (*tok).u.backref.by_name = 0 as libc::c_int;
                (*tok).u.backref.exist_level = 0 as libc::c_int;
                current_block = 11687719283784080910;
            } else { current_block = 4936445733123251449; }
            match current_block {
                11687719283784080910 => { }
                _ => {
                    if c == '8' as i32 as libc::c_uint ||
                           c == '9' as i32 as libc::c_uint {
                        /* normal char */
                        p = prev;
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        current_block = 11687719283784080910;
                    } else { p = prev; current_block = 2916120083628436309; }
                }
            }
        }
        3334119875212657085 => {
            if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0
               {
                current_block = 11687719283784080910;
            } else {
                prev = p;
                if (*syn).op2 & (1 as libc::c_uint) << 14 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                    num =
                        scan_unsigned_hexadecimal_number(&mut p, end,
                                                         4 as libc::c_int,
                                                         enc);
                    if num < 0 as libc::c_int { return -(200 as libc::c_int) }
                    if p == prev {
                        /* can't read nothing. */
                        num = 0 as libc::c_int
                        /* but, it's not error */
                    }
                    (*tok).type_0 = TK_CODE_POINT;
                    (*tok).base = 16 as libc::c_int;
                    (*tok).u.code = num as OnigCodePoint
                }
                current_block = 11687719283784080910;
            }
        }
        8540457983839371850 => {
            if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0
               {
                current_block = 11687719283784080910;
            } else {
                prev = p;
                if (if p < end {
                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                               end)
                    } else { 0 as libc::c_int as libc::c_uint }) ==
                       '{' as i32 as OnigCodePoint &&
                       (*syn).op & (1 as libc::c_uint) << 30 as libc::c_int !=
                           0 as libc::c_int as libc::c_uint {
                    pfetch_prev = p;
                    p =
                        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                     as isize);
                    num =
                        scan_unsigned_hexadecimal_number(&mut p, end,
                                                         8 as libc::c_int,
                                                         enc);
                    if num < 0 as libc::c_int { return -(401 as libc::c_int) }
                    if if p < end {
                           0 as libc::c_int
                       } else { 1 as libc::c_int } == 0 {
                        if (*enc).is_code_ctype.expect("non-null function pointer")(if p
                                                                                           <
                                                                                           end
                                                                                       {
                                                                                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                                                                               end)
                                                                                    } else {
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint
                                                                                    },
                                                                                    11
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        OnigCtype)
                               != 0 {
                            return -(212 as libc::c_int)
                        }
                    }
                    if p >
                           prev.offset((*enc).mbc_enc_len.expect("non-null function pointer")(prev)
                                           as isize) &&
                           (if p < end {
                                0 as libc::c_int
                            } else { 1 as libc::c_int }) == 0 &&
                           (if p < end {
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end)
                            } else { 0 as libc::c_int as libc::c_uint }) ==
                               '}' as i32 as OnigCodePoint {
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        (*tok).type_0 = TK_CODE_POINT;
                        (*tok).u.code = num as OnigCodePoint
                    } else {
                        /* can't read nothing or invalid format */
                        p = prev
                    }
                } else if (*syn).op & (1 as libc::c_uint) << 29 as libc::c_int
                              != 0 as libc::c_int as libc::c_uint {
                    num =
                        scan_unsigned_hexadecimal_number(&mut p, end,
                                                         2 as libc::c_int,
                                                         enc);
                    if num < 0 as libc::c_int { return -(200 as libc::c_int) }
                    if p == prev {
                        /* can't read nothing. */
                        num = 0 as libc::c_int
                        /* but, it's not error */
                    } /* error */
                    (*tok).type_0 = TK_RAW_BYTE;
                    (*tok).base = 16 as libc::c_int;
                    (*tok).u.c = num
                }
                current_block = 11687719283784080910;
            }
        }
        4724803196040624734 => {
            if !((*syn).op & (1 as libc::c_uint) << 15 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ANCHOR;
                (*tok).u.subtype = (1 as libc::c_int) << 2 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        4158919382095016298 => {
            if !((*syn).op & (1 as libc::c_uint) << 14 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ANCHOR;
                (*tok).u.subtype = (1 as libc::c_int) << 4 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        9636888125699845498 => {
            if !((*syn).op2 & (1 as libc::c_uint) << 19 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 11 as libc::c_int;
                (*tok).u.prop.not = 1 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        9842434449443674853 => {
            if !((*syn).op2 & (1 as libc::c_uint) << 19 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 11 as libc::c_int;
                (*tok).u.prop.not = 0 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        15952717276592120294 => {
            if !((*syn).op & (1 as libc::c_uint) << 22 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 4 as libc::c_int;
                (*tok).u.prop.not = 1 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        7230447043056047703 => {
            if !((*syn).op & (1 as libc::c_uint) << 22 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 4 as libc::c_int;
                (*tok).u.prop.not = 0 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        15453639749100233674 => {
            if !((*syn).op & (1 as libc::c_uint) << 21 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 9 as libc::c_int;
                (*tok).u.prop.not = 1 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        3009217617782060917 => {
            if !((*syn).op & (1 as libc::c_uint) << 21 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 9 as libc::c_int;
                (*tok).u.prop.not = 0 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        9325485969395579449 => {
            if !((*syn).op & (1 as libc::c_uint) << 19 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ANCHOR;
                (*tok).u.anchor = (1 as libc::c_int) << 9 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        17673280890339436511 => {
            if !((*syn).op & (1 as libc::c_uint) << 19 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ANCHOR;
                (*tok).u.anchor = (1 as libc::c_int) << 8 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        1898993302553076133 => {
            if !((*syn).op & (1 as libc::c_uint) << 20 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ANCHOR;
                (*tok).u.anchor = (1 as libc::c_int) << 7 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        8412118167407665105 => {
            if !((*syn).op & (1 as libc::c_uint) << 20 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ANCHOR;
                (*tok).u.anchor = (1 as libc::c_int) << 6 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        5674466540670887041 => {
            if !((*syn).op & (1 as libc::c_uint) << 18 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 12 as libc::c_int;
                (*tok).u.prop.not = 1 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        16873236124228232483 => {
            if !((*syn).op & (1 as libc::c_uint) << 18 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_CHAR_TYPE;
                (*tok).u.prop.ctype = 12 as libc::c_int;
                (*tok).u.prop.not = 0 as libc::c_int;
                current_block = 11687719283784080910;
            }
        }
        3494171831578997035 => {
            if !((*syn).op & (1 as libc::c_uint) << 13 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_SUBEXP_CLOSE;
                current_block = 11687719283784080910;
            }
        }
        15904406811757377787 => {
            if !((*syn).op & (1 as libc::c_uint) << 13 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_SUBEXP_OPEN;
                current_block = 11687719283784080910;
            }
        }
        18258225697907100628 => {
            if !((*syn).op & (1 as libc::c_uint) << 11 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_ALT;
                current_block = 11687719283784080910;
            }
        }
        297784347553452316 => {
            if !((*syn).op & (1 as libc::c_uint) << 9 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                r = fetch_range_quantifier(&mut p, end, tok, env);
                if r < 0 as libc::c_int { return r }
                if r == 0 as libc::c_int {
                    current_block = 10131319541265114523;
                } else if r == 2 as libc::c_int {
                    /* {n} */
                    if (*syn).behavior &
                           (1 as libc::c_uint) << 9 as libc::c_int !=
                           0 as libc::c_int as libc::c_uint {
                        current_block = 6765407519951034582;
                    } else { current_block = 10131319541265114523; }
                } else {
                    /* r == 1 : normal char */
                    current_block = 11687719283784080910;
                }
            }
        }
        11980851012772541874 => {
            if !((*syn).op & (1 as libc::c_uint) << 7 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_OP_REPEAT;
                (*tok).u.repeat.lower = 0 as libc::c_int;
                (*tok).u.repeat.upper = 1 as libc::c_int;
                current_block = 10131319541265114523;
            }
        }
        12740818325037147325 => {
            if !((*syn).op & (1 as libc::c_uint) << 5 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_OP_REPEAT;
                (*tok).u.repeat.lower = 1 as libc::c_int;
                (*tok).u.repeat.upper = -(1 as libc::c_int);
                current_block = 10131319541265114523;
            }
        }
        16372213405346143698 => {
            if !((*syn).op & (1 as libc::c_uint) << 3 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else {
                (*tok).type_0 = TK_OP_REPEAT;
                (*tok).u.repeat.lower = 0 as libc::c_int;
                (*tok).u.repeat.upper = -(1 as libc::c_int);
                current_block = 10131319541265114523;
            }
        }
        17058499098102203106 => {
            (*tok).type_0 = TK_ANCHOR;
            (*tok).u.subtype =
                if (*env).option &
                       (((1 as libc::c_uint) << 1 as libc::c_int) <<
                            1 as libc::c_int) << 1 as libc::c_int != 0 {
                    ((1 as libc::c_int)) << 0 as libc::c_int
                } else { ((1 as libc::c_int)) << 1 as libc::c_int };
            current_block = 11687719283784080910;
        }
        4544312254017297231 => {
            (*tok).type_0 = TK_ANCHOR;
            (*tok).u.subtype =
                if (*env).option &
                       (((1 as libc::c_uint) << 1 as libc::c_int) <<
                            1 as libc::c_int) << 1 as libc::c_int != 0 {
                    ((1 as libc::c_int)) << 4 as libc::c_int
                } else { ((1 as libc::c_int)) << 5 as libc::c_int };
            current_block = 11687719283784080910;
        }
        15112282366752052665 => {
            (*tok).type_0 = TK_ANYCHAR;
            current_block = 11687719283784080910;
        }
        6398212124985529790 => {
            let mut c2: OnigCodePoint = 0;
            p = pfetch_prev;
            num = fetch_escaped_value(&mut p, end, env, &mut c2);
            if num < 0 as libc::c_int { return num }
            /* set_raw: */
            if (*tok).u.c as libc::c_uint != c2 {
                (*tok).type_0 = TK_CODE_POINT;
                (*tok).u.code = c2
            } else {
                /* string */
                p =
                    (*tok).backp.offset((*enc).mbc_enc_len.expect("non-null function pointer")((*tok).backp)
                                            as isize)
            } /* no need. escape gcc warning. */
            current_block = 11687719283784080910;
        }
        13228428744258393755 => {
            if (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) ==
                   0 &&
                   (if p < end {
                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                               end)
                    } else { 0 as libc::c_int as libc::c_uint }) ==
                       '{' as i32 as OnigCodePoint &&
                   (*syn).op2 & (1 as libc::c_uint) << 16 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                (*tok).type_0 = TK_CHAR_PROPERTY;
                (*tok).u.prop.not =
                    if c == 'P' as i32 as libc::c_uint {
                        1 as libc::c_int
                    } else { 0 as libc::c_int };
                if (if p < end { 0 as libc::c_int } else { 1 as libc::c_int })
                       == 0 &&
                       (*syn).op2 & (1 as libc::c_uint) << 17 as libc::c_int
                           != 0 as libc::c_int as libc::c_uint {
                    c =
                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                               end);
                    pfetch_prev = p;
                    p =
                        p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                     as isize);
                    if c == '^' as i32 as libc::c_uint {
                        (*tok).u.prop.not =
                            if (*tok).u.prop.not == 0 as libc::c_int {
                                1 as libc::c_int
                            } else { 0 as libc::c_int }
                    } else { p = pfetch_prev }
                }
            }
            current_block = 11687719283784080910;
        }
        4874723077730206021 => {
            if (*syn).op2 & (1 as libc::c_uint) << 0 as libc::c_int !=
                   0 as libc::c_int as libc::c_uint {
                (*tok).type_0 = TK_QUOTE_OPEN
            }
            current_block = 11687719283784080910;
        }
        8048257192757430128 => {
            if (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) ==
                   0 &&
                   (*syn).op2 & (1 as libc::c_uint) << 9 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if c == '<' as i32 as libc::c_uint ||
                       c == '\'' as i32 as libc::c_uint {
                    let mut gnum: libc::c_int = 0;
                    let mut name_end_0: *mut OnigUChar = 0 as *mut OnigUChar;
                    prev = p;
                    r =
                        fetch_name(c, &mut p, end, &mut name_end_0, env,
                                   &mut gnum, 1 as libc::c_int);
                    if r < 0 as libc::c_int { return r }
                    (*tok).type_0 = TK_CALL;
                    (*tok).u.call.name = prev;
                    (*tok).u.call.name_end = name_end_0;
                    (*tok).u.call.gnum = gnum
                } else { p = pfetch_prev }
            }
            current_block = 11687719283784080910;
        }
        14906809932406725840 => {
            if (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) ==
                   0 &&
                   (*syn).op2 & (1 as libc::c_uint) << 8 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                if c == '<' as i32 as libc::c_uint ||
                       c == '\'' as i32 as libc::c_uint {
                    let mut name_end: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut backs: *mut libc::c_int = 0 as *mut libc::c_int;
                    let mut back_num: libc::c_int = 0;
                    prev = p;
                    name_end = 0 as *mut OnigUChar;
                    r =
                        fetch_name_with_level(c, &mut p, end, &mut name_end,
                                              env, &mut back_num,
                                              &mut (*tok).u.backref.level);
                    if r == 1 as libc::c_int {
                        (*tok).u.backref.exist_level = 1 as libc::c_int
                    } else { (*tok).u.backref.exist_level = 0 as libc::c_int }
                    if r < 0 as libc::c_int { return r }
                    if back_num != 0 as libc::c_int {
                        if back_num < 0 as libc::c_int {
                            back_num =
                                (*env).num_mem + 1 as libc::c_int + back_num;
                            if back_num <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        if (*syn).behavior &
                               (1 as libc::c_uint) << 5 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if back_num > (*env).num_mem ||
                                   (*(if !((*env).mem_nodes_dynamic as
                                               *mut libc::c_void).is_null() {
                                          (*env).mem_nodes_dynamic
                                      } else {
                                          (*env).mem_nodes_static.as_mut_ptr()
                                      }).offset(back_num as isize) as
                                        *mut libc::c_void).is_null() {
                                return -(208 as libc::c_int)
                            }
                        }
                        (*tok).type_0 = TK_BACKREF;
                        (*tok).u.backref.by_name = 0 as libc::c_int;
                        (*tok).u.backref.num = 1 as libc::c_int;
                        (*tok).u.backref.ref1 = back_num
                    } else {
                        num =
                            onig_name_to_group_numbers((*env).reg, prev,
                                                       name_end, &mut backs);
                        if num <= 0 as libc::c_int {
                            onig_scan_env_set_error_string(env,
                                                           -(217 as
                                                                 libc::c_int),
                                                           prev, name_end);
                            return -(217 as libc::c_int)
                        }
                        if (*syn).behavior &
                               (1 as libc::c_uint) << 5 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            let mut i: libc::c_int = 0;
                            i = 0 as libc::c_int;
                            while i < num {
                                if *backs.offset(i as isize) > (*env).num_mem
                                       ||
                                       (*(if !((*env).mem_nodes_dynamic as
                                                   *mut libc::c_void).is_null()
                                             {
                                              (*env).mem_nodes_dynamic
                                          } else {
                                              (*env).mem_nodes_static.as_mut_ptr()
                                          }).offset(*backs.offset(i as isize)
                                                        as isize) as
                                            *mut libc::c_void).is_null() {
                                    return -(208 as libc::c_int)
                                }
                                i += 1
                            }
                        }
                        (*tok).type_0 = TK_BACKREF;
                        (*tok).u.backref.by_name = 1 as libc::c_int;
                        if num == 1 as libc::c_int {
                            (*tok).u.backref.num = 1 as libc::c_int;
                            (*tok).u.backref.ref1 =
                                *backs.offset(0 as libc::c_int as isize)
                        } else {
                            (*tok).u.backref.num = num;
                            (*tok).u.backref.refs = backs
                        }
                    }
                } else { p = pfetch_prev }
            }
            current_block = 11687719283784080910;
        }
        8478868103403534984 => {
            (*tok).type_0 = TK_ALT;
            current_block = 11687719283784080910;
        }
        15849589987095405551 => {
            (*tok).type_0 = TK_SUBEXP_CLOSE;
            current_block = 11687719283784080910;
        }
        11297176373744872492 => {
            (*tok).type_0 = TK_CC_OPEN;
            current_block = 11687719283784080910;
        }
        8235112133255751888 => {
            if !((*syn).op & (1 as libc::c_uint) << 14 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else { current_block = 9624946417733889048; }
        }
        14131958400073952123 => {
            if !((*syn).op & (1 as libc::c_uint) << 14 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else { current_block = 1149483608613653870; }
        }
        15570335776229543684 => {
            if !((*syn).op2 & (1 as libc::c_uint) << 15 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else { current_block = 9624946417733889048; }
        }
        7455596087214207069 => {
            if !((*syn).op2 & (1 as libc::c_uint) << 15 as libc::c_int !=
                     0 as libc::c_int as libc::c_uint) {
                current_block = 11687719283784080910;
            } else { current_block = 1149483608613653870; }
        }
        _ => { }
    }
    match current_block {
        1149483608613653870 => {
            (*tok).type_0 = TK_ANCHOR;
            (*tok).u.subtype = (1 as libc::c_int) << 3 as libc::c_int;
            current_block = 11687719283784080910;
        }
        9624946417733889048 => {
            (*tok).type_0 = TK_ANCHOR;
            (*tok).u.subtype = (1 as libc::c_int) << 0 as libc::c_int;
            current_block = 11687719283784080910;
        }
        10131319541265114523 => {
            if (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) ==
                   0 &&
                   (if p < end {
                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                               end)
                    } else { 0 as libc::c_int as libc::c_uint }) ==
                       '?' as i32 as OnigCodePoint &&
                   (*syn).op & (1 as libc::c_uint) << 25 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                (*tok).u.repeat.greedy = 0 as libc::c_int;
                (*tok).u.repeat.possessive = 0 as libc::c_int;
                current_block = 11687719283784080910;
            } else { current_block = 6765407519951034582; }
        }
        2916120083628436309 =>
        /* fall through */
        {
            if (*syn).op & (1 as libc::c_uint) << 28 as libc::c_int !=
                   0 as libc::c_int as libc::c_uint {
                prev = p;
                num =
                    scan_unsigned_octal_number(&mut p, end,
                                               if c ==
                                                      '0' as i32 as
                                                          libc::c_uint {
                                                   2 as libc::c_int
                                               } else { 3 as libc::c_int },
                                               enc);
                if num < 0 as libc::c_int { return -(200 as libc::c_int) }
                if p == prev {
                    /* can't read nothing. */
                    num = 0 as libc::c_int
                    /* but, it's not error */
                }
                (*tok).type_0 = TK_RAW_BYTE;
                (*tok).base = 8 as libc::c_int;
                (*tok).u.c = num
            } else if c != '0' as i32 as libc::c_uint {
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize)
            }
            current_block = 11687719283784080910;
        }
        _ => { }
    }
    match current_block {
        6765407519951034582 => {
            if (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) ==
                   0 &&
                   (if p < end {
                        (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                               end)
                    } else { 0 as libc::c_int as libc::c_uint }) ==
                       '+' as i32 as OnigCodePoint &&
                   ((*syn).op2 & (1 as libc::c_uint) << 4 as libc::c_int !=
                        0 as libc::c_int as libc::c_uint &&
                        (*tok).type_0 as libc::c_uint !=
                            TK_INTERVAL as libc::c_int as libc::c_uint ||
                        (*syn).op2 & (1 as libc::c_uint) << 5 as libc::c_int
                            != 0 as libc::c_int as libc::c_uint &&
                            (*tok).type_0 as libc::c_uint ==
                                TK_INTERVAL as libc::c_int as libc::c_uint) {
                c =
                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                           end);
                pfetch_prev = p;
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                (*tok).u.repeat.greedy = 1 as libc::c_int;
                (*tok).u.repeat.possessive = 1 as libc::c_int
            } else {
                (*tok).u.repeat.greedy = 1 as libc::c_int;
                (*tok).u.repeat.possessive = 0 as libc::c_int
            }
        }
        _ => { }
    }
    /* string */
    *src = p;
    return (*tok).type_0 as libc::c_int;
}
unsafe extern "C" fn add_ctype_to_cc_by_range(mut cc: *mut CClassNode,
                                              mut ctype: libc::c_int,
                                              mut not: libc::c_int,
                                              mut enc: OnigEncoding,
                                              mut sb_out: OnigCodePoint,
                                              mut mbr: *const OnigCodePoint)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut j: OnigCodePoint = 0;
    let mut n: libc::c_int =
        *mbr.offset(0 as libc::c_int as isize) as libc::c_int;
    if not == 0 as libc::c_int {
        i = 0 as libc::c_int;
        's_17:
            while i < n {
                j =
                    *mbr.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                    isize);
                while j <=
                          *mbr.offset((i * 2 as libc::c_int +
                                           2 as libc::c_int) as isize) {
                    if j >= sb_out {
                        if j >
                               *mbr.offset((i * 2 as libc::c_int +
                                                1 as libc::c_int) as isize) {
                            r =
                                add_code_range_to_buf(&mut (*cc).mbuf, j,
                                                      *mbr.offset((i *
                                                                       2 as
                                                                           libc::c_int
                                                                       +
                                                                       2 as
                                                                           libc::c_int)
                                                                      as
                                                                      isize));
                            if r != 0 as libc::c_int { return r }
                            i += 1
                        }
                        break 's_17 ;
                    } else {
                        (*cc).bs[(j as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (j as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint;
                        j = j.wrapping_add(1)
                    }
                }
                i += 1
            }
        while i < n {
            r =
                add_code_range_to_buf(&mut (*cc).mbuf,
                                      *mbr.offset((i * 2 as libc::c_int +
                                                       1 as libc::c_int) as
                                                      isize),
                                      *mbr.offset((i * 2 as libc::c_int +
                                                       2 as libc::c_int) as
                                                      isize));
            if r != 0 as libc::c_int { return r }
            i += 1
        }
    } else {
        let mut current_block_37: u64;
        let mut prev: OnigCodePoint = 0 as libc::c_int as OnigCodePoint;
        i = 0 as libc::c_int;
        's_118:
            loop  {
                if !(i < n) {
                    current_block_37 = 5494826135382683477;
                    break ;
                }
                j = prev;
                while j <
                          *mbr.offset((i * 2 as libc::c_int +
                                           1 as libc::c_int) as isize) {
                    if j >= sb_out {
                        current_block_37 = 9402490234393440343;
                        break 's_118 ;
                    }
                    (*cc).bs[(j as
                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                                 as usize] |=
                        ((1 as libc::c_int) <<
                             (j as
                                  libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)))
                            as libc::c_uint;
                    j = j.wrapping_add(1)
                }
                prev =
                    (*mbr.offset((i * 2 as libc::c_int + 2 as libc::c_int) as
                                     isize)).wrapping_add(1 as libc::c_int as
                                                              libc::c_uint);
                i += 1
            }
        match current_block_37 {
            5494826135382683477 => {
                j = prev;
                while j < sb_out {
                    (*cc).bs[(j as
                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                                 as usize] |=
                        ((1 as libc::c_int) <<
                             (j as
                                  libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)))
                            as libc::c_uint;
                    j = j.wrapping_add(1)
                }
            }
            _ => { }
        }
        prev = sb_out;
        i = 0 as libc::c_int;
        while i < n {
            if prev <
                   *mbr.offset((i * 2 as libc::c_int + 1 as libc::c_int) as
                                   isize) {
                r =
                    add_code_range_to_buf(&mut (*cc).mbuf, prev,
                                          (*mbr.offset((i * 2 as libc::c_int +
                                                            1 as libc::c_int)
                                                           as
                                                           isize)).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint));
                if r != 0 as libc::c_int { return r }
            }
            prev =
                (*mbr.offset((i * 2 as libc::c_int + 2 as libc::c_int) as
                                 isize)).wrapping_add(1 as libc::c_int as
                                                          libc::c_uint);
            i += 1
        }
        if prev < 0x7fffffff as libc::c_int as libc::c_uint {
            r =
                add_code_range_to_buf(&mut (*cc).mbuf, prev,
                                      0x7fffffff as libc::c_int as
                                          OnigCodePoint);
            if r != 0 as libc::c_int { return r }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_ctype_to_cc(mut cc: *mut CClassNode,
                                     mut ctype: libc::c_int,
                                     mut not: libc::c_int,
                                     mut env: *mut ScanEnv) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ranges: *const OnigCodePoint = 0 as *const OnigCodePoint;
    let mut sb_out: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    r =
        (*enc).get_ctype_code_range.expect("non-null function pointer")(ctype
                                                                            as
                                                                            OnigCtype,
                                                                        &mut sb_out,
                                                                        &mut ranges);
    if r == 0 as libc::c_int {
        return add_ctype_to_cc_by_range(cc, ctype, not, (*env).enc, sb_out,
                                        ranges)
    } else { if r != -(2 as libc::c_int) { return r } }
    r = 0 as libc::c_int;
    match ctype {
        1 | 2 | 3 | 4 | 6 | 8 | 9 | 10 | 11 | 14 | 13 => {
            if not != 0 as libc::c_int {
                c = 0 as libc::c_int;
                while c < (1 as libc::c_int) << 8 as libc::c_int {
                    if (*enc).is_code_ctype.expect("non-null function pointer")(c
                                                                                    as
                                                                                    OnigCodePoint,
                                                                                ctype
                                                                                    as
                                                                                    OnigCtype)
                           == 0 {
                        (*cc).bs[(c as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (c as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint
                    }
                    c += 1
                }
                if !((*enc).max_enc_len == 1 as libc::c_int) {
                    r =
                        add_code_range_to_buf(&mut (*cc).mbuf,
                                              if (*enc).min_enc_len >
                                                     1 as libc::c_int {
                                                  0 as libc::c_int
                                              } else { 0x80 as libc::c_int }
                                                  as OnigCodePoint,
                                              !(0 as libc::c_int as
                                                    OnigCodePoint));
                    if r != 0 { return r }
                }
            } else {
                c = 0 as libc::c_int;
                while c < (1 as libc::c_int) << 8 as libc::c_int {
                    if (*enc).is_code_ctype.expect("non-null function pointer")(c
                                                                                    as
                                                                                    OnigCodePoint,
                                                                                ctype
                                                                                    as
                                                                                    OnigCtype)
                           != 0 {
                        (*cc).bs[(c as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (c as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint
                    }
                    c += 1
                }
            }
        }
        5 | 7 => {
            if not != 0 as libc::c_int {
                c = 0 as libc::c_int;
                while c < (1 as libc::c_int) << 8 as libc::c_int {
                    if (*enc).is_code_ctype.expect("non-null function pointer")(c
                                                                                    as
                                                                                    OnigCodePoint,
                                                                                ctype
                                                                                    as
                                                                                    OnigCtype)
                           == 0 {
                        (*cc).bs[(c as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (c as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint
                    }
                    c += 1
                }
            } else {
                c = 0 as libc::c_int;
                while c < (1 as libc::c_int) << 8 as libc::c_int {
                    if (*enc).is_code_ctype.expect("non-null function pointer")(c
                                                                                    as
                                                                                    OnigCodePoint,
                                                                                ctype
                                                                                    as
                                                                                    OnigCtype)
                           != 0 {
                        (*cc).bs[(c as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (c as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint
                    }
                    c += 1
                }
                if !((*enc).max_enc_len == 1 as libc::c_int) {
                    r =
                        add_code_range_to_buf(&mut (*cc).mbuf,
                                              if (*enc).min_enc_len >
                                                     1 as libc::c_int {
                                                  0 as libc::c_int
                                              } else { 0x80 as libc::c_int }
                                                  as OnigCodePoint,
                                              !(0 as libc::c_int as
                                                    OnigCodePoint));
                    if r != 0 { return r }
                }
            }
        }
        12 => {
            if not == 0 as libc::c_int {
                c = 0 as libc::c_int;
                while c < (1 as libc::c_int) << 8 as libc::c_int {
                    if c < 128 as libc::c_int &&
                           (*enc).is_code_ctype.expect("non-null function pointer")(c
                                                                                        as
                                                                                        OnigCodePoint,
                                                                                    12
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        OnigCtype)
                               != 0 {
                        (*cc).bs[(c as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (c as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint
                    }
                    c += 1
                }
                if !((*enc).max_enc_len == 1 as libc::c_int) {
                    r =
                        add_code_range_to_buf(&mut (*cc).mbuf,
                                              if (*enc).min_enc_len >
                                                     1 as libc::c_int {
                                                  0 as libc::c_int
                                              } else { 0x80 as libc::c_int }
                                                  as OnigCodePoint,
                                              !(0 as libc::c_int as
                                                    OnigCodePoint));
                    if r != 0 { return r }
                }
            } else {
                c = 0 as libc::c_int;
                while c < (1 as libc::c_int) << 8 as libc::c_int {
                    if (*enc).code_to_mbclen.expect("non-null function pointer")(c
                                                                                     as
                                                                                     OnigCodePoint)
                           > 0 as libc::c_int &&
                           (*enc).is_code_ctype.expect("non-null function pointer")(c
                                                                                        as
                                                                                        OnigCodePoint,
                                                                                    12
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        OnigCtype)
                               == 0 {
                        (*cc).bs[(c as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] |=
                            ((1 as libc::c_int) <<
                                 (c as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                as libc::c_uint
                    }
                    c += 1
                }
            }
        }
        _ => { return -(11 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn parse_posix_bracket(mut cc: *mut CClassNode,
                                         mut src: *mut *mut OnigUChar,
                                         mut end: *mut OnigUChar,
                                         mut env: *mut ScanEnv)
 -> libc::c_int {
    static mut PBS: [PosixBracketEntryType; 15] =
        [{
             let mut init =
                 PosixBracketEntryType{name:
                                           b"alnum\x00" as *const u8 as
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
                                           b"alpha\x00" as *const u8 as
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
                                           b"blank\x00" as *const u8 as
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
                                           b"cntrl\x00" as *const u8 as
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
                                           b"digit\x00" as *const u8 as
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
                                           b"graph\x00" as *const u8 as
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
                                           b"lower\x00" as *const u8 as
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
                                           b"print\x00" as *const u8 as
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
                                           b"punct\x00" as *const u8 as
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
                                           b"space\x00" as *const u8 as
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
                                           b"upper\x00" as *const u8 as
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
                                           b"xdigit\x00" as *const u8 as
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
                                           b"ascii\x00" as *const u8 as
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
                                           b"word\x00" as *const u8 as
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
    let mut not: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut c: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut p: *mut OnigUChar = *src;
    if (if p < end {
            (*enc).mbc_to_code.expect("non-null function pointer")(p, end)
        } else { 0 as libc::c_int as libc::c_uint }) ==
           '^' as i32 as OnigCodePoint {
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        not = 1 as libc::c_int
    } else { not = 0 as libc::c_int }
    if !(onigenc_strlen(enc, p, end) < 4 as libc::c_int + 3 as libc::c_int) {
        pb = PBS.as_mut_ptr();
        while !((*pb).name as *mut libc::c_void).is_null() {
            if onigenc_with_ascii_strncmp(enc, p, end, (*pb).name,
                                          (*pb).len as libc::c_int) ==
                   0 as libc::c_int {
                p = onigenc_step(enc, p, end, (*pb).len as libc::c_int);
                if onigenc_with_ascii_strncmp(enc, p, end,
                                              b":]\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut OnigUChar,
                                              2 as libc::c_int) !=
                       0 as libc::c_int {
                    return -(121 as libc::c_int)
                }
                r = add_ctype_to_cc(cc, (*pb).ctype, not, env);
                if r != 0 as libc::c_int { return r }
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                p =
                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                 as isize);
                *src = p;
                return 0 as libc::c_int
            }
            pb = pb.offset(1)
        }
    }
    c = 0 as libc::c_int as OnigCodePoint;
    i = 0 as libc::c_int;
    while (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) == 0 &&
              {
                  c =
                      (if p < end {
                           (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                  end)
                       } else { 0 as libc::c_int as libc::c_uint });
                  (c) != ':' as i32 as libc::c_uint
              } && c != ']' as i32 as libc::c_uint {
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        i += 1;
        if i > 20 as libc::c_int { break ; }
    }
    if c == ':' as i32 as libc::c_uint &&
           (if p < end { 0 as libc::c_int } else { 1 as libc::c_int }) == 0 {
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } == 0 {
            c =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if c == ']' as i32 as libc::c_uint {
                return -(121 as libc::c_int)
            }
        }
    }
    return 1 as libc::c_int;
    /* 1: is not POSIX bracket, but no error. */
}
unsafe extern "C" fn fetch_char_property_to_ctype(mut src:
                                                      *mut *mut OnigUChar,
                                                  mut end: *mut OnigUChar,
                                                  mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut c: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut start: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = *src;
    r = 0 as libc::c_int;
    prev = p;
    start = prev;
    while if p < end { 0 as libc::c_int } else { 1 as libc::c_int } == 0 {
        prev = p;
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if c == '}' as i32 as libc::c_uint {
            r =
                (*enc).property_name_to_ctype.expect("non-null function pointer")(enc,
                                                                                  start,
                                                                                  prev);
            if r < 0 as libc::c_int { break ; }
            *src = p;
            return r
        } else {
            if !(c == '(' as i32 as libc::c_uint ||
                     c == ')' as i32 as libc::c_uint ||
                     c == '{' as i32 as libc::c_uint ||
                     c == '|' as i32 as libc::c_uint) {
                continue ;
            }
            r = -(223 as libc::c_int);
            break ;
        }
    }
    onig_scan_env_set_error_string(env, r, *src, prev);
    return r;
}
unsafe extern "C" fn parse_char_property(mut np: *mut *mut Node,
                                         mut tok: *mut OnigToken,
                                         mut src: *mut *mut OnigUChar,
                                         mut end: *mut OnigUChar,
                                         mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut ctype: libc::c_int = 0;
    let mut cc: *mut CClassNode = 0 as *mut CClassNode;
    ctype = fetch_char_property_to_ctype(src, end, env);
    if ctype < 0 as libc::c_int { return ctype }
    *np = node_new_cclass();
    if (*np as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    cc = &mut (**np).u.cclass;
    r = add_ctype_to_cc(cc, ctype, 0 as libc::c_int, env);
    if r != 0 as libc::c_int { return r }
    if (*tok).u.prop.not != 0 as libc::c_int {
        (*cc).flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_state_class(mut cc: *mut CClassNode,
                                      mut vs: *mut OnigCodePoint,
                                      mut type_0: *mut CCVALTYPE,
                                      mut state: *mut CCSTATE,
                                      mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if *state as libc::c_uint == CCS_RANGE as libc::c_int as libc::c_uint {
        return -(110 as libc::c_int)
    }
    if *state as libc::c_uint == CCS_VALUE as libc::c_int as libc::c_uint &&
           *type_0 as libc::c_uint != CCV_CLASS as libc::c_int as libc::c_uint
       {
        if *type_0 as libc::c_uint == CCV_SB as libc::c_int as libc::c_uint {
            (*cc).bs[(*vs as libc::c_int as
                          libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong))
                         as usize] |=
                ((1 as libc::c_int) <<
                     (*vs as libc::c_int as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)))
                    as libc::c_uint
        } else if *type_0 as libc::c_uint ==
                      CCV_CODE_POINT as libc::c_int as libc::c_uint {
            r = add_code_range(&mut (*cc).mbuf, env, *vs, *vs);
            if r < 0 as libc::c_int { return r }
        }
    }
    *state = CCS_VALUE;
    *type_0 = CCV_CLASS;
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_state_val(mut cc: *mut CClassNode,
                                    mut vs: *mut OnigCodePoint,
                                    mut v: OnigCodePoint,
                                    mut vs_israw: *mut libc::c_int,
                                    mut v_israw: libc::c_int,
                                    mut intype: CCVALTYPE,
                                    mut type_0: *mut CCVALTYPE,
                                    mut state: *mut CCSTATE,
                                    mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    match *state as libc::c_uint {
        0 => {
            if *type_0 as libc::c_uint ==
                   CCV_SB as libc::c_int as libc::c_uint {
                (*cc).bs[(*vs as libc::c_int as
                              libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong))
                             as usize] |=
                    ((1 as libc::c_int) <<
                         (*vs as libc::c_int as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)))
                        as libc::c_uint
            } else if *type_0 as libc::c_uint ==
                          CCV_CODE_POINT as libc::c_int as libc::c_uint {
                r = add_code_range(&mut (*cc).mbuf, env, *vs, *vs);
                if r < 0 as libc::c_int { return r }
            }
        }
        1 => {
            if intype as libc::c_uint == *type_0 as libc::c_uint {
                if intype as libc::c_uint ==
                       CCV_SB as libc::c_int as libc::c_uint {
                    if *vs > 0xff as libc::c_int as libc::c_uint ||
                           v > 0xff as libc::c_int as libc::c_uint {
                        return -(400 as libc::c_int)
                    }
                    if *vs > v {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 22 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(203 as libc::c_int)
                        }
                    } else {
                        bitset_set_range((*cc).bs.as_mut_ptr(),
                                         *vs as libc::c_int,
                                         v as libc::c_int);
                    }
                } else {
                    r = add_code_range(&mut (*cc).mbuf, env, *vs, v);
                    if r < 0 as libc::c_int { return r }
                }
            } else if *vs > v {
                if !((*(*env).syntax).behavior &
                         (1 as libc::c_uint) << 22 as libc::c_int !=
                         0 as libc::c_int as libc::c_uint) {
                    return -(203 as libc::c_int)
                }
            } else {
                bitset_set_range((*cc).bs.as_mut_ptr(), *vs as libc::c_int,
                                 if v < 0xff as libc::c_int as libc::c_uint {
                                     v
                                 } else {
                                     0xff as libc::c_int as libc::c_uint
                                 } as libc::c_int);
                r = add_code_range(&mut (*cc).mbuf, env, *vs, v);
                if r < 0 as libc::c_int { return r }
            }
            *state = CCS_COMPLETE
        }
        2 | 3 => { *state = CCS_VALUE }
        _ => { }
    }
    *vs_israw = v_israw;
    *vs = v;
    *type_0 = intype;
    return 0 as libc::c_int;
}
unsafe extern "C" fn code_exist_check(mut c: OnigCodePoint,
                                      mut from: *mut OnigUChar,
                                      mut end: *mut OnigUChar,
                                      mut ignore_escaped: libc::c_int,
                                      mut env: *mut ScanEnv) -> libc::c_int {
    let mut in_esc: libc::c_int = 0;
    let mut code: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut p: *mut OnigUChar = from;
    in_esc = 0 as libc::c_int;
    while if p < end { 0 as libc::c_int } else { 1 as libc::c_int } == 0 {
        if ignore_escaped != 0 && in_esc != 0 {
            in_esc = 0 as libc::c_int
        } else {
            code =
                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                       end);
            p =
                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                             as isize);
            if code == c { return 1 as libc::c_int }
            if code == (*(*env).syntax).meta_char_table.esc {
                in_esc = 1 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_char_class(mut np: *mut *mut Node,
                                      mut tok: *mut OnigToken,
                                      mut src: *mut *mut OnigUChar,
                                      mut end: *mut OnigUChar,
                                      mut env: *mut ScanEnv) -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut neg: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut fetched: libc::c_int = 0;
    let mut and_start: libc::c_int = 0;
    let mut v: OnigCodePoint = 0;
    let mut vs: OnigCodePoint = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut node: *mut Node = 0 as *mut Node;
    let mut cc: *mut CClassNode = 0 as *mut CClassNode;
    let mut prev_cc: *mut CClassNode = 0 as *mut CClassNode;
    let mut work_cc: CClassNode =
        CClassNode{base: NodeBase{type_0: 0,},
                   flags: 0,
                   bs: [0; 8],
                   mbuf: 0 as *mut BBuf,};
    let mut state: CCSTATE = CCS_VALUE;
    let mut val_type: CCVALTYPE = CCV_SB;
    let mut in_type: CCVALTYPE = CCV_SB;
    let mut val_israw: libc::c_int = 0;
    let mut in_israw: libc::c_int = 0;
    *np = 0 as *mut Node;
    (*env).parse_depth = (*env).parse_depth.wrapping_add(1);
    if (*env).parse_depth > ParseDepthLimit { return -(16 as libc::c_int) }
    prev_cc = 0 as *mut libc::c_void as *mut CClassNode;
    r = fetch_token_in_cc(tok, src, end, env);
    if r == TK_CHAR as libc::c_int && (*tok).u.c == '^' as i32 &&
           (*tok).escaped == 0 as libc::c_int {
        neg = 1 as libc::c_int;
        r = fetch_token_in_cc(tok, src, end, env)
    } else { neg = 0 as libc::c_int }
    if r < 0 as libc::c_int { return r }
    if r == TK_CC_CLOSE as libc::c_int {
        if code_exist_check(']' as i32 as OnigCodePoint, *src,
                            (*env).pattern_end, 1 as libc::c_int, env) == 0 {
            return -(102 as libc::c_int)
        }
        CC_ESC_WARN(env,
                    b"]\x00" as *const u8 as *const libc::c_char as
                        *mut OnigUChar);
        (*tok).type_0 = TK_CHAR;
        r = (*tok).type_0 as libc::c_int
        /* allow []...] */
    }
    node = node_new_cclass();
    *np = node;
    if (node as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    cc = &mut (*node).u.cclass;
    and_start = 0 as libc::c_int;
    state = CCS_START;
    p = *src;
    's_142:
        loop  {
            if !(r != TK_CC_CLOSE as libc::c_int) {
                current_block = 10996290961880923853;
                break ;
            }
            fetched = 0 as libc::c_int;
            match r {
                2 => { current_block = 1186222781174767224; }
                1 => {
                    /* tok->base != 0 : octal or hexadec. */
                    if !((*(*env).enc).max_enc_len == 1 as libc::c_int) &&
                           (*tok).base != 0 as libc::c_int {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut bufe: *mut OnigUChar =
                            buf.as_mut_ptr().offset(7 as libc::c_int as
                                                        isize);
                        let mut psave: *mut OnigUChar = p;
                        let mut i: libc::c_int = 0;
                        let mut base: libc::c_int = (*tok).base;
                        buf[0 as libc::c_int as usize] =
                            (*tok).u.c as OnigUChar;
                        i = 1 as libc::c_int;
                        while i < (*(*env).enc).max_enc_len {
                            r = fetch_token_in_cc(tok, &mut p, end, env);
                            if r < 0 as libc::c_int {
                                current_block = 17889361762285833695;
                                break 's_142 ;
                            }
                            if r != TK_RAW_BYTE as libc::c_int ||
                                   (*tok).base != base {
                                fetched = 1 as libc::c_int;
                                break ;
                            } else {
                                buf[i as usize] = (*tok).u.c as OnigUChar;
                                i += 1
                            }
                        }
                        if i < (*(*env).enc).min_enc_len {
                            r = -(206 as libc::c_int);
                            current_block = 17889361762285833695;
                            break ;
                        } else {
                            len =
                                (*(*env).enc).mbc_enc_len.expect("non-null function pointer")(buf.as_mut_ptr());
                            if i < len {
                                r = -(206 as libc::c_int);
                                current_block = 17889361762285833695;
                                break ;
                            } else {
                                if i > len {
                                    /* fetch back */
                                    p = psave;
                                    i = 1 as libc::c_int;
                                    while i < len {
                                        r =
                                            fetch_token_in_cc(tok, &mut p,
                                                              end, env);
                                        i += 1
                                    }
                                    fetched = 0 as libc::c_int
                                }
                                if i == 1 as libc::c_int {
                                    v =
                                        buf[0 as libc::c_int as usize] as
                                            OnigCodePoint;
                                    current_block = 3367568877686200745;
                                } else {
                                    v =
                                        (*(*env).enc).mbc_to_code.expect("non-null function pointer")(buf.as_mut_ptr(),
                                                                                                      bufe);
                                    in_type = CCV_CODE_POINT;
                                    current_block = 17441561948628420366;
                                }
                            }
                        }
                    } else {
                        v = (*tok).u.c as OnigCodePoint;
                        current_block = 3367568877686200745;
                    }
                    match current_block {
                        3367568877686200745 => { in_type = CCV_SB }
                        _ => { }
                    }
                    in_israw = 1 as libc::c_int;
                    current_block = 13477568710254637077;
                }
                4 => {
                    v = (*tok).u.code;
                    in_israw = 1 as libc::c_int;
                    current_block = 3735943025446157910;
                }
                21 => {
                    r = parse_posix_bracket(cc, &mut p, end, env);
                    if r < 0 as libc::c_int {
                        current_block = 17889361762285833695;
                        break ;
                    }
                    if r == 1 as libc::c_int {
                        /* is not POSIX bracket */
                        CC_ESC_WARN(env,
                                    b"[\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut OnigUChar);
                        p = (*tok).backp;
                        v = (*tok).u.c as OnigCodePoint;
                        in_israw = 0 as libc::c_int;
                        current_block = 3735943025446157910;
                    } else { current_block = 9690652989446465541; }
                }
                6 => {
                    r =
                        add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                        (*tok).u.prop.not, env);
                    if r != 0 as libc::c_int { return r }
                    current_block = 9690652989446465541;
                }
                18 => {
                    let mut ctype: libc::c_int = 0;
                    ctype = fetch_char_property_to_ctype(&mut p, end, env);
                    if ctype < 0 as libc::c_int { return ctype }
                    r = add_ctype_to_cc(cc, ctype, (*tok).u.prop.not, env);
                    if r != 0 as libc::c_int { return r }
                    current_block = 9690652989446465541;
                }
                20 => {
                    if state as libc::c_uint ==
                           CCS_VALUE as libc::c_int as libc::c_uint {
                        r = fetch_token_in_cc(tok, &mut p, end, env);
                        if r < 0 as libc::c_int {
                            current_block = 17889361762285833695;
                            break ;
                        }
                        fetched = 1 as libc::c_int;
                        if r == TK_CC_CLOSE as libc::c_int {
                            /* allow [x-] */
                            current_block = 6453482308595291318;
                        } else if r == TK_CC_AND as libc::c_int {
                            CC_ESC_WARN(env,
                                        b"-\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut OnigUChar);
                            current_block = 6453482308595291318;
                        } else {
                            state = CCS_RANGE;
                            current_block = 6880299496751257707;
                        }
                    } else if state as libc::c_uint ==
                                  CCS_START as libc::c_int as libc::c_uint {
                        /* [-xa] is allowed */
                        v = (*tok).u.c as OnigCodePoint;
                        in_israw = 0 as libc::c_int;
                        r = fetch_token_in_cc(tok, &mut p, end, env);
                        if r < 0 as libc::c_int {
                            current_block = 17889361762285833695;
                            break ;
                        }
                        fetched = 1 as libc::c_int;
                        /* [--x] or [a&&-x] is warned. */
                        if r == TK_CC_RANGE as libc::c_int ||
                               and_start != 0 as libc::c_int {
                            CC_ESC_WARN(env,
                                        b"-\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut OnigUChar);
                        }
                        current_block = 3735943025446157910;
                    } else if state as libc::c_uint ==
                                  CCS_RANGE as libc::c_int as libc::c_uint {
                        CC_ESC_WARN(env,
                                    b"-\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut OnigUChar);
                        current_block = 1186222781174767224;
                        /* [!--x] is allowed */
                    } else {
                        /* CCS_COMPLETE */
                        r = fetch_token_in_cc(tok, &mut p, end, env);
                        if r < 0 as libc::c_int {
                            current_block = 17889361762285833695;
                            break ;
                        }
                        fetched = 1 as libc::c_int;
                        if r == TK_CC_CLOSE as libc::c_int {
                            current_block = 6453482308595291318;
                        } else {
                            if r == TK_CC_AND as libc::c_int {
                                CC_ESC_WARN(env,
                                            b"-\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut OnigUChar);
                            } else if (*(*env).syntax).behavior &
                                          (1 as libc::c_uint) <<
                                              23 as libc::c_int !=
                                          0 as libc::c_int as libc::c_uint {
                                CC_ESC_WARN(env,
                                            b"-\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut OnigUChar);
                                /* [0-9-a] is allowed as [0-9\-a] */
                            } else {
                                r = -(112 as libc::c_int); /* allow [a-b-] */
                                current_block = 17889361762285833695;
                                break ;
                            }
                            current_block = 6453482308595291318;
                        }
                    }
                    match current_block {
                        3735943025446157910 => { }
                        1186222781174767224 => { }
                        6880299496751257707 => { }
                        _ => {
                            v = '-' as i32 as OnigCodePoint;
                            in_israw = 0 as libc::c_int;
                            current_block = 3735943025446157910;
                        }
                    }
                }
                23 => {
                    /* [ */
                    let mut anode: *mut Node = 0 as *mut Node;
                    let mut acc: *mut CClassNode = 0 as *mut CClassNode;
                    r = parse_char_class(&mut anode, tok, &mut p, end, env);
                    if r != 0 as libc::c_int {
                        onig_node_free(anode);
                    } else {
                        acc = &mut (*anode).u.cclass;
                        r = or_cclass(cc, acc, (*env).enc);
                        onig_node_free(anode);
                    }
                    if r != 0 as libc::c_int {
                        current_block = 17889361762285833695;
                        break ;
                    }
                    current_block = 6880299496751257707;
                }
                22 => {
                    /* && */
                    if state as libc::c_uint ==
                           CCS_VALUE as libc::c_int as libc::c_uint {
                        r =
                            next_state_val(cc, &mut vs,
                                           0 as libc::c_int as OnigCodePoint,
                                           &mut val_israw, 0 as libc::c_int,
                                           val_type, &mut val_type,
                                           &mut state, env);
                        if r != 0 as libc::c_int {
                            current_block = 17889361762285833695;
                            break ;
                        }
                    }
                    /* initialize local variables */
                    and_start = 1 as libc::c_int;
                    state = CCS_START;
                    if !(prev_cc as *mut libc::c_void).is_null() {
                        r = and_cclass(prev_cc, cc, (*env).enc);
                        if r != 0 as libc::c_int {
                            current_block = 17889361762285833695;
                            break ;
                        }
                        bbuf_free((*cc).mbuf);
                    } else { prev_cc = cc; cc = &mut work_cc }
                    initialize_cclass(cc);
                    current_block = 6880299496751257707;
                }
                0 => {
                    r = -(103 as libc::c_int);
                    current_block = 17889361762285833695;
                    break ;
                }
                _ => {
                    r = -(11 as libc::c_int);
                    current_block = 17889361762285833695;
                    break ;
                }
            }
            match current_block {
                9690652989446465541 => {
                    r =
                        next_state_class(cc, &mut vs, &mut val_type,
                                         &mut state, env);
                    if r != 0 as libc::c_int {
                        current_block = 17889361762285833695;
                        break ;
                    }
                    current_block = 6880299496751257707;
                }
                3735943025446157910 => {
                    len =
                        (*(*env).enc).code_to_mbclen.expect("non-null function pointer")(v);
                    if len < 0 as libc::c_int {
                        r = len;
                        current_block = 17889361762285833695;
                        break ;
                    } else {
                        in_type =
                            if len == 1 as libc::c_int {
                                CCV_SB as libc::c_int
                            } else { CCV_CODE_POINT as libc::c_int } as
                                CCVALTYPE
                    }
                    current_block = 13477568710254637077;
                }
                1186222781174767224 => {
                    len =
                        (*(*env).enc).code_to_mbclen.expect("non-null function pointer")((*tok).u.c
                                                                                             as
                                                                                             OnigCodePoint);
                    if len > 1 as libc::c_int {
                        in_type = CCV_CODE_POINT
                    } else if len < 0 as libc::c_int {
                        r = len;
                        current_block = 17889361762285833695;
                        break ;
                    } else {
                        /* sb_char: */
                        in_type = CCV_SB
                    }
                    v = (*tok).u.c as OnigCodePoint;
                    in_israw = 0 as libc::c_int;
                    current_block = 13477568710254637077;
                }
                _ => { }
            }
            match current_block {
                13477568710254637077 => {
                    r =
                        next_state_val(cc, &mut vs, v, &mut val_israw,
                                       in_israw, in_type, &mut val_type,
                                       &mut state, env);
                    if r != 0 as libc::c_int {
                        current_block = 17889361762285833695;
                        break ;
                    }
                }
                _ => { }
            }
            if fetched != 0 {
                r = (*tok).type_0 as libc::c_int
            } else {
                r = fetch_token_in_cc(tok, &mut p, end, env);
                if r < 0 as libc::c_int {
                    current_block = 17889361762285833695;
                    break ;
                }
            }
        }
    match current_block {
        10996290961880923853 => {
            if state as libc::c_uint ==
                   CCS_VALUE as libc::c_int as libc::c_uint {
                r =
                    next_state_val(cc, &mut vs,
                                   0 as libc::c_int as OnigCodePoint,
                                   &mut val_israw, 0 as libc::c_int, val_type,
                                   &mut val_type, &mut state, env);
                if r != 0 as libc::c_int {
                    current_block = 17889361762285833695;
                } else { current_block = 2346768750020253347; }
            } else { current_block = 2346768750020253347; }
            match current_block {
                17889361762285833695 => { }
                _ => {
                    if !(prev_cc as *mut libc::c_void).is_null() {
                        r = and_cclass(prev_cc, cc, (*env).enc);
                        if r != 0 as libc::c_int {
                            current_block = 17889361762285833695;
                        } else {
                            bbuf_free((*cc).mbuf);
                            cc = prev_cc;
                            current_block = 17322559809113443968;
                        }
                    } else { current_block = 17322559809113443968; }
                    match current_block {
                        17889361762285833695 => { }
                        _ => {
                            if neg != 0 as libc::c_int {
                                (*cc).flags |=
                                    ((1 as libc::c_int) << 0 as libc::c_int)
                                        as libc::c_uint
                            } else {
                                (*cc).flags &=
                                    !((1 as libc::c_int) << 0 as libc::c_int)
                                        as libc::c_uint
                            }
                            if (*cc).flags &
                                   ((1 as libc::c_int) << 0 as libc::c_int) as
                                       libc::c_uint !=
                                   0 as libc::c_int as libc::c_uint &&
                                   (*(*env).syntax).behavior &
                                       (1 as libc::c_uint) <<
                                           20 as libc::c_int !=
                                       0 as libc::c_int as libc::c_uint {
                                let mut is_empty: libc::c_int = 0;
                                is_empty =
                                    if ((*cc).mbuf as
                                            *mut libc::c_void).is_null() {
                                        1 as libc::c_int
                                    } else { 0 as libc::c_int };
                                if is_empty != 0 as libc::c_int {
                                    let mut i_0: libc::c_int = 0;
                                    is_empty = 1 as libc::c_int;
                                    i_0 = 0 as libc::c_int;
                                    while i_0 <
                                              (((1 as libc::c_int) <<
                                                    8 as libc::c_int) as
                                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_ulong))
                                                  as libc::c_int {
                                        if (*cc).bs[i_0 as usize] !=
                                               0 as libc::c_int as
                                                   libc::c_uint {
                                            is_empty = 0 as libc::c_int;
                                            break ;
                                        } else { i_0 += 1 }
                                    }
                                }
                                if is_empty == 0 as libc::c_int {
                                    if (*(*env).enc).is_code_ctype.expect("non-null function pointer")(0xa
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           OnigCodePoint,
                                                                                                       0
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           OnigCtype)
                                           != 0 {
                                        if (*(*env).enc).code_to_mbclen.expect("non-null function pointer")(0xa
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                OnigCodePoint)
                                               == 1 as libc::c_int {
                                            (*cc).bs[(0xa as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] |=
                                                ((1 as libc::c_int) <<
                                                     (0xa as libc::c_int as
                                                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong)))
                                                    as libc::c_uint
                                        } else {
                                            add_code_range(&mut (*cc).mbuf,
                                                           env,
                                                           0xa as libc::c_int
                                                               as
                                                               OnigCodePoint,
                                                           0xa as libc::c_int
                                                               as
                                                               OnigCodePoint);
                                        }
                                    }
                                }
                            }
                            *src = p;
                            (*env).parse_depth =
                                (*env).parse_depth.wrapping_sub(1);
                            return 0 as libc::c_int
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if cc != &mut (**np).u.cclass as *mut CClassNode {
        bbuf_free((*cc).mbuf);
    }
    return r;
}
unsafe extern "C" fn parse_enclose(mut np: *mut *mut Node,
                                   mut tok: *mut OnigToken,
                                   mut term: libc::c_int,
                                   mut src: *mut *mut OnigUChar,
                                   mut end: *mut OnigUChar,
                                   mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut target: *mut Node = 0 as *mut Node;
    let mut option: OnigOptionType = 0;
    let mut c: OnigCodePoint = 0;
    let mut enc: OnigEncoding = (*env).enc;
    let mut list_capture: libc::c_int = 0;
    let mut p: *mut OnigUChar = *src;
    let mut pfetch_prev: *mut OnigUChar = 0 as *mut OnigUChar;
    *np = 0 as *mut Node;
    if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
        return -(117 as libc::c_int)
    }
    option = (*env).option;
    let mut name: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut name_end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut current_block_145: u64;
    if (if p < end {
            (*enc).mbc_to_code.expect("non-null function pointer")(p, end)
        } else { 0 as libc::c_int as libc::c_uint }) ==
           '?' as i32 as OnigCodePoint &&
           (*(*env).syntax).op2 & (1 as libc::c_uint) << 1 as libc::c_int !=
               0 as libc::c_int as libc::c_uint {
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        if if p < end { 0 as libc::c_int } else { 1 as libc::c_int } != 0 {
            return -(118 as libc::c_int)
        }
        c = (*enc).mbc_to_code.expect("non-null function pointer")(p, end);
        pfetch_prev = p;
        p =
            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                         as isize);
        match c {
            58 => { current_block_145 = 746628236765849835; }
            61 => {
                current_block_145 = 4276137520343792367;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        /* look behind (?<=...), (?<!...) */
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        /*         preceding read */
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        /* (?>...) stop backtrack */
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                                /* option only */
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                    /* (?@<name>...) */
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            33 => {
                current_block_145 = 9846438544018066795;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            62 => {
                current_block_145 = 6242671573579299787;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            39 => {
                current_block_145 = 17308265447697084388;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            60 => {
                current_block_145 = 3187822828080989487;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            64 => {
                current_block_145 = 17787851924190067911;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            45 | 105 | 109 | 115 | 120 => {
                current_block_145 = 3010514237756320440;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
            _ => {
                current_block_145 = 17173725837442240300;
                match current_block_145 {
                    17173725837442240300 => { return -(119 as libc::c_int) }
                    3187822828080989487 => {
                        if if p < end {
                               0 as libc::c_int
                           } else { 1 as libc::c_int } != 0 {
                            return -(117 as libc::c_int)
                        }
                        c =
                            (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                   end);
                        pfetch_prev = p;
                        p =
                            p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                         as isize);
                        if c == '=' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         12 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else if c == '!' as i32 as libc::c_uint {
                            *np =
                                onig_node_new_anchor((1 as libc::c_int) <<
                                                         13 as libc::c_int);
                            current_block_145 = 1541960655964344436;
                        } else {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                name = 0 as *mut OnigUChar;
                                name_end = 0 as *mut OnigUChar;
                                p = pfetch_prev;
                                c = '<' as i32 as OnigCodePoint
                            } else { return -(119 as libc::c_int) }
                            current_block_145 = 1430194059259504117;
                        }
                    }
                    17308265447697084388 => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 7 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            current_block_145 = 1430194059259504117;
                        } else { return -(119 as libc::c_int) }
                    }
                    4276137520343792367 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     10 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    9846438544018066795 => {
                        *np =
                            onig_node_new_anchor((1 as libc::c_int) <<
                                                     11 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    6242671573579299787 => {
                        *np =
                            node_new_enclose((1 as libc::c_int) <<
                                                 2 as libc::c_int);
                        current_block_145 = 1541960655964344436;
                    }
                    3010514237756320440 => {
                        let mut neg: libc::c_int = 0 as libc::c_int;
                        loop  {
                            match c {
                                58 | 41 => { }
                                45 => { neg = 1 as libc::c_int }
                                120 => {
                                    if neg != 0 {
                                        option &=
                                            !((1 as libc::c_uint) <<
                                                  1 as libc::c_int)
                                    } else {
                                        option |=
                                            (1 as libc::c_uint) <<
                                                1 as libc::c_int
                                    };
                                }
                                105 => {
                                    if neg != 0 {
                                        option &= !(1 as libc::c_uint)
                                    } else { option |= 1 as libc::c_uint };
                                }
                                115 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                109 => {
                                    if (*(*env).syntax).op2 &
                                           (1 as libc::c_uint) <<
                                               2 as libc::c_int !=
                                           0 as libc::c_int as libc::c_uint {
                                        if if neg == 0 as libc::c_int {
                                               1 as libc::c_int
                                           } else { 0 as libc::c_int } != 0 {
                                            option &=
                                                !((((1 as libc::c_uint) <<
                                                        1 as libc::c_int) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                (((1 as libc::c_uint) <<
                                                      1 as libc::c_int) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else if (*(*env).syntax).op2 &
                                                  (1 as libc::c_uint) <<
                                                      3 as libc::c_int !=
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                        if neg != 0 {
                                            option &=
                                                !(((1 as libc::c_uint) <<
                                                       1 as libc::c_int) <<
                                                      1 as libc::c_int)
                                        } else {
                                            option |=
                                                ((1 as libc::c_uint) <<
                                                     1 as libc::c_int) <<
                                                    1 as libc::c_int
                                        };
                                    } else { return -(119 as libc::c_int) }
                                }
                                _ => { return -(119 as libc::c_int) }
                            }
                            if c == ')' as i32 as libc::c_uint {
                                *np = node_new_option(option);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                *src = p;
                                return 2 as libc::c_int
                            } else {
                                if c == ':' as i32 as libc::c_uint {
                                    let mut prev: OnigOptionType =
                                        (*env).option;
                                    (*env).option = option;
                                    r = fetch_token(tok, &mut p, end, env);
                                    if r < 0 as libc::c_int { return r }
                                    r =
                                        parse_subexp(&mut target, tok, term,
                                                     &mut p, end, env);
                                    (*env).option = prev;
                                    if r < 0 as libc::c_int {
                                        onig_node_free(target);
                                        return r
                                    }
                                    *np = node_new_option(option);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    (**np).u.enclose.target = target;
                                    *src = p;
                                    return 0 as libc::c_int
                                }
                            }
                            if if p < end {
                                   0 as libc::c_int
                               } else { 1 as libc::c_int } != 0 {
                                return -(118 as libc::c_int)
                            }
                            c =
                                (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                       end);
                            pfetch_prev = p;
                            p =
                                p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                             as isize)
                        }
                    }
                    _ => {
                        if (*(*env).syntax).op2 &
                               (1 as libc::c_uint) << 10 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).op2 &
                                   (1 as libc::c_uint) << 7 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                c =
                                    (*enc).mbc_to_code.expect("non-null function pointer")(p,
                                                                                           end);
                                pfetch_prev = p;
                                p =
                                    p.offset((*enc).mbc_enc_len.expect("non-null function pointer")(p)
                                                 as isize);
                                if c == '<' as i32 as libc::c_uint ||
                                       c == '\'' as i32 as libc::c_uint {
                                    list_capture = 1 as libc::c_int;
                                    current_block_145 = 12158049957618377742;
                                } else {
                                    p = pfetch_prev;
                                    current_block_145 = 54079586644752974;
                                }
                            } else { current_block_145 = 54079586644752974; }
                            match current_block_145 {
                                12158049957618377742 => { }
                                _ => {
                                    *np =
                                        node_new_enclose_memory((*env).option,
                                                                0 as
                                                                    libc::c_int);
                                    if (*np as *mut libc::c_void).is_null() {
                                        return -(5 as libc::c_int)
                                    }
                                    num = scan_env_add_mem_entry(env);
                                    if num < 0 as libc::c_int {
                                        return num
                                    } else {
                                        if num >=
                                               (::std::mem::size_of::<BitStatusType>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as libc::c_int {
                                            return -(222 as libc::c_int)
                                        }
                                    }
                                    (**np).u.enclose.regnum = num;
                                    if num <
                                           (::std::mem::size_of::<BitStatusType>()
                                                as
                                                libc::c_ulong).wrapping_mul(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int {
                                        (*env).capture_history |=
                                            ((1 as libc::c_int) << num) as
                                                libc::c_uint
                                    }
                                    current_block_145 = 1541960655964344436;
                                }
                            }
                        } else { return -(119 as libc::c_int) }
                    }
                }
                match current_block_145 {
                    1541960655964344436 => { }
                    _ => {
                        match current_block_145 {
                            1430194059259504117 => {
                                list_capture = 0 as libc::c_int
                            }
                            _ => { }
                        }
                        name = p;
                        r =
                            fetch_name(c, &mut p, end, &mut name_end, env,
                                       &mut num, 0 as libc::c_int);
                        if r < 0 as libc::c_int { return r }
                        num = scan_env_add_mem_entry(env);
                        if num < 0 as libc::c_int { return num }
                        if list_capture != 0 as libc::c_int &&
                               num >=
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                            return -(222 as libc::c_int)
                        }
                        r = name_add((*env).reg, name, name_end, num, env);
                        if r != 0 as libc::c_int { return r }
                        *np =
                            node_new_enclose_memory((*env).option,
                                                    1 as libc::c_int);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (**np).u.enclose.regnum = num;
                        if list_capture != 0 as libc::c_int {
                            if num <
                                   (::std::mem::size_of::<BitStatusType>() as
                                        libc::c_ulong).wrapping_mul(8 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                       as libc::c_int {
                                (*env).capture_history |=
                                    ((1 as libc::c_int) << num) as
                                        libc::c_uint
                            }
                        }
                        (*env).num_named += 1;
                        current_block_145 = 1541960655964344436;
                    }
                }
            }
        }
    } else if (*env).option &
                  (((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                           1 as libc::c_int) << 1 as libc::c_int) <<
                         1 as libc::c_int) << 1 as libc::c_int) <<
                       1 as libc::c_int) << 1 as libc::c_int != 0 {
        current_block_145 = 746628236765849835;
    } else {
        *np = node_new_enclose_memory((*env).option, 0 as libc::c_int);
        if (*np as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
        num = scan_env_add_mem_entry(env);
        if num < 0 as libc::c_int { return num }
        (**np).u.enclose.regnum = num;
        current_block_145 = 1541960655964344436;
    }
    match current_block_145 {
        1541960655964344436 => { }
        _ =>
        /* (?:...) grouping only */
        {
            r = fetch_token(tok, &mut p, end, env); /* group */
            if r < 0 as libc::c_int { return r }
            r = parse_subexp(np, tok, term, &mut p, end, env);
            if r < 0 as libc::c_int { return r }
            *src = p;
            return 1 as libc::c_int
        }
    }
    if (*np as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    r = fetch_token(tok, &mut p, end, env);
    if r < 0 as libc::c_int { return r }
    r = parse_subexp(&mut target, tok, term, &mut p, end, env);
    if r < 0 as libc::c_int { onig_node_free(target); return r }
    if (**np).u.base.type_0 == 7 as libc::c_int {
        (**np).u.anchor.target = target
    } else {
        (**np).u.enclose.target = target;
        if (**np).u.enclose.type_0 == (1 as libc::c_int) << 0 as libc::c_int {
            /* Don't move this to previous of parse_subexp() */
            r = scan_env_set_mem_node(env, (**np).u.enclose.regnum, *np);
            if r != 0 as libc::c_int { return r }
        }
    }
    *src = p;
    return 0 as libc::c_int;
}
static mut PopularQStr: [*const libc::c_char; 6] =
    [b"?\x00" as *const u8 as *const libc::c_char,
     b"*\x00" as *const u8 as *const libc::c_char,
     b"+\x00" as *const u8 as *const libc::c_char,
     b"??\x00" as *const u8 as *const libc::c_char,
     b"*?\x00" as *const u8 as *const libc::c_char,
     b"+?\x00" as *const u8 as *const libc::c_char];
static mut ReduceQStr: [*const libc::c_char; 7] =
    [b"\x00" as *const u8 as *const libc::c_char,
     b"\x00" as *const u8 as *const libc::c_char,
     b"*\x00" as *const u8 as *const libc::c_char,
     b"*?\x00" as *const u8 as *const libc::c_char,
     b"??\x00" as *const u8 as *const libc::c_char,
     b"+ and ??\x00" as *const u8 as *const libc::c_char,
     b"+? and ?\x00" as *const u8 as *const libc::c_char];
unsafe extern "C" fn set_quantifier(mut qnode: *mut Node,
                                    mut target: *mut Node,
                                    mut group: libc::c_int,
                                    mut env: *mut ScanEnv) -> libc::c_int {
    let mut current_block: u64;
    let mut qn: *mut QtfrNode = 0 as *mut QtfrNode;
    qn = &mut (*qnode).u.qtfr;
    if (*qn).lower == 1 as libc::c_int && (*qn).upper == 1 as libc::c_int {
        return 1 as libc::c_int
    }
    match (*target).u.base.type_0 {
        0 => {
            if group == 0 {
                let mut sn: *mut StrNode = &mut (*target).u.str_0;
                if str_node_can_be_split(sn, (*env).enc) != 0 {
                    let mut n: *mut Node =
                        str_node_split_last_char(sn, (*env).enc);
                    if !(n as *mut libc::c_void).is_null() {
                        (*qn).target = n;
                        return 2 as libc::c_int
                    }
                }
            }
            current_block = 17784502470059252271;
        }
        5 => {
            /* check redundant double repeat. */
      /* verbose warn (?:.?)? etc... but not warn (.?)? etc... */
            let mut qnt: *mut QtfrNode = &mut (*target).u.qtfr;
            let mut nestq_num: libc::c_int = popular_quantifier_num(qn);
            let mut targetq_num: libc::c_int = popular_quantifier_num(qnt);
            if !((*qn).state & (1 as libc::c_int) << 14 as libc::c_int !=
                     0 as libc::c_int) &&
                   !((*qnt).state & (1 as libc::c_int) << 14 as libc::c_int !=
                         0 as libc::c_int) &&
                   (*(*env).syntax).behavior &
                       (1 as libc::c_uint) << 25 as libc::c_int !=
                       0 as libc::c_int as libc::c_uint {
                let mut buf: [OnigUChar; 256] = [0; 256];
                match ReduceTypeTable[targetq_num as
                                          usize][nestq_num as usize] as
                          libc::c_uint {
                    0 => { }
                    1 => {
                        current_block = 10648125353650867138;
                        match current_block {
                            18244012544844943134 => {
                                if onig_verb_warn !=
                                       Some(onig_null_warn as
                                                unsafe extern "C" fn(_:
                                                                         *const libc::c_char)
                                                    -> ()) {
                                    onig_snprintf_with_pattern(buf.as_mut_ptr(),
                                                               256 as
                                                                   libc::c_int,
                                                               (*env).enc,
                                                               (*env).pattern,
                                                               (*env).pattern_end,
                                                               b"nested repeat operator %s and %s was replaced with \'%s\'\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut OnigUChar,
                                                               PopularQStr[targetq_num
                                                                               as
                                                                               usize],
                                                               PopularQStr[nestq_num
                                                                               as
                                                                               usize],
                                                               ReduceQStr[ReduceTypeTable[targetq_num
                                                                                              as
                                                                                              usize][nestq_num
                                                                                                         as
                                                                                                         usize]
                                                                              as
                                                                              usize]);
                                    Some(onig_verb_warn.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr()
                                                                                                                                     as
                                                                                                                                     *mut libc::c_char);
                                }
                            }
                            _ => {
                                if onig_verb_warn !=
                                       Some(onig_null_warn as
                                                unsafe extern "C" fn(_:
                                                                         *const libc::c_char)
                                                    -> ()) {
                                    onig_snprintf_with_pattern(buf.as_mut_ptr(),
                                                               256 as
                                                                   libc::c_int,
                                                               (*env).enc,
                                                               (*env).pattern,
                                                               (*env).pattern_end,
                                                               b"redundant nested repeat operator\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut OnigUChar);
                                    Some(onig_verb_warn.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr()
                                                                                                                                     as
                                                                                                                                     *mut libc::c_char);
                                }
                            }
                        }
                    }
                    _ => {
                        current_block = 18244012544844943134;
                        match current_block {
                            18244012544844943134 => {
                                if onig_verb_warn !=
                                       Some(onig_null_warn as
                                                unsafe extern "C" fn(_:
                                                                         *const libc::c_char)
                                                    -> ()) {
                                    onig_snprintf_with_pattern(buf.as_mut_ptr(),
                                                               256 as
                                                                   libc::c_int,
                                                               (*env).enc,
                                                               (*env).pattern,
                                                               (*env).pattern_end,
                                                               b"nested repeat operator %s and %s was replaced with \'%s\'\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut OnigUChar,
                                                               PopularQStr[targetq_num
                                                                               as
                                                                               usize],
                                                               PopularQStr[nestq_num
                                                                               as
                                                                               usize],
                                                               ReduceQStr[ReduceTypeTable[targetq_num
                                                                                              as
                                                                                              usize][nestq_num
                                                                                                         as
                                                                                                         usize]
                                                                              as
                                                                              usize]);
                                    Some(onig_verb_warn.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr()
                                                                                                                                     as
                                                                                                                                     *mut libc::c_char);
                                }
                            }
                            _ => {
                                if onig_verb_warn !=
                                       Some(onig_null_warn as
                                                unsafe extern "C" fn(_:
                                                                         *const libc::c_char)
                                                    -> ()) {
                                    onig_snprintf_with_pattern(buf.as_mut_ptr(),
                                                               256 as
                                                                   libc::c_int,
                                                               (*env).enc,
                                                               (*env).pattern,
                                                               (*env).pattern_end,
                                                               b"redundant nested repeat operator\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut OnigUChar);
                                    Some(onig_verb_warn.expect("non-null function pointer")).expect("non-null function pointer")(buf.as_mut_ptr()
                                                                                                                                     as
                                                                                                                                     *mut libc::c_char);
                                }
                            }
                        }
                    }
                }
            }
            if targetq_num >= 0 as libc::c_int {
                if nestq_num >= 0 as libc::c_int {
                    onig_reduce_nested_quantifier(qnode, target);
                    current_block = 9351478953302203588;
                } else {
                    if targetq_num == 1 as libc::c_int ||
                           targetq_num == 2 as libc::c_int {
                        /* * or + */
                        /* (?:a*){n,m}, (?:a+){n,m} => (?:a*){n,n}, (?:a+){n,n} */
                        if !((*qn).upper == -(1 as libc::c_int)) &&
                               (*qn).upper > 1 as libc::c_int &&
                               (*qn).greedy != 0 {
                            (*qn).upper =
                                if (*qn).lower == 0 as libc::c_int {
                                    1 as libc::c_int
                                } else { (*qn).lower }
                        }
                    }
                    current_block = 17784502470059252271;
                }
            } else { current_block = 17784502470059252271; }
        }
        _ => { current_block = 17784502470059252271; }
    }
    match current_block {
        17784502470059252271 => { (*qn).target = target }
        _ => { }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn i_apply_case_fold(mut from: OnigCodePoint,
                                       mut to: *mut OnigCodePoint,
                                       mut to_len: libc::c_int,
                                       mut arg: *mut libc::c_void)
 -> libc::c_int {
    let mut iarg: *mut IApplyCaseFoldArg = 0 as *mut IApplyCaseFoldArg;
    let mut env: *mut ScanEnv = 0 as *mut ScanEnv;
    let mut cc: *mut CClassNode = 0 as *mut CClassNode;
    let mut bs: BitSetRef = 0 as *mut Bits;
    iarg = arg as *mut IApplyCaseFoldArg;
    env = (*iarg).env;
    cc = (*iarg).cc;
    bs = (*cc).bs.as_mut_ptr();
    if to_len == 1 as libc::c_int {
        let mut is_in: libc::c_int = onig_is_code_in_cc((*env).enc, from, cc);
        if is_in != 0 as libc::c_int &&
               !((*cc).flags &
                     ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                     != 0 as libc::c_int as libc::c_uint) ||
               is_in == 0 as libc::c_int &&
                   (*cc).flags &
                       ((1 as libc::c_int) << 0 as libc::c_int) as
                           libc::c_uint != 0 as libc::c_int as libc::c_uint {
            if (*(*env).enc).min_enc_len > 1 as libc::c_int ||
                   *to >=
                       ((1 as libc::c_int) << 8 as libc::c_int) as
                           libc::c_uint {
                add_code_range(&mut (*cc).mbuf, env, *to, *to);
            } else {
                let ref mut fresh11 =
                    *bs.offset((*to as
                                    libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                                   as isize);
                *fresh11 |=
                    ((1 as libc::c_int) <<
                         (*to as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong)))
                        as libc::c_uint
            }
        }
        /* CASE_FOLD_IS_APPLIED_INSIDE_NEGATIVE_CCLASS */
    } else {
        let mut r: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut buf: [OnigUChar; 7] = [0; 7];
        let mut snode: *mut Node = 0 as *mut Node;
        if onig_is_code_in_cc((*env).enc, from, cc) != 0 &&
               !((*cc).flags &
                     ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                     != 0 as libc::c_int as libc::c_uint) {
            i = 0 as libc::c_int;
            while i < to_len {
                len =
                    (*(*env).enc).code_to_mbc.expect("non-null function pointer")(*to.offset(i
                                                                                                 as
                                                                                                 isize),
                                                                                  buf.as_mut_ptr());
                if i == 0 as libc::c_int {
                    snode =
                        onig_node_new_str(buf.as_mut_ptr(),
                                          buf.as_mut_ptr().offset(len as
                                                                      isize));
                    if (snode as *mut libc::c_void).is_null() {
                        return -(5 as libc::c_int)
                    }
                    /* char-class expanded multi-char only
             compare with string folded at match time. */
                    (*snode).u.str_0.flag |=
                        ((1 as libc::c_int) << 1 as libc::c_int) as
                            libc::c_uint
                } else {
                    r =
                        onig_node_str_cat(snode, buf.as_mut_ptr(),
                                          buf.as_mut_ptr().offset(len as
                                                                      isize));
                    if r < 0 as libc::c_int {
                        onig_node_free(snode);
                        return r
                    }
                }
                i += 1
            }
            *(*iarg).ptail = onig_node_new_alt(snode, 0 as *mut Node);
            if (*(*iarg).ptail as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
            (*iarg).ptail = &mut (**(*iarg).ptail).u.cons.cdr
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_exp(mut np: *mut *mut Node,
                               mut tok: *mut OnigToken, mut term: libc::c_int,
                               mut src: *mut *mut OnigUChar,
                               mut end: *mut OnigUChar, mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut group: libc::c_int = 0 as libc::c_int;
    let mut qn: *mut Node = 0 as *mut Node;
    let mut targetp: *mut *mut Node = 0 as *mut *mut Node;
    *np = 0 as *mut Node;
    if !((*tok).type_0 as libc::c_uint == term as TokenSyms as libc::c_uint) {
        match (*tok).type_0 as libc::c_uint {
            13 | 0 => { }
            14 => {
                current_block = 1863889588662849341;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            /* option only */
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    //should not enclen_end()
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                /* Don't use this, it is wrong for little endian encodings. */
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                /* split case: /abc+/ */
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            15 => {
                current_block = 6491452808681056906;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            3 => {
                current_block = 4775909272756257391;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            1 => {
                current_block = 15512526488502093901;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            4 => {
                current_block = 11009730405578905636;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            17 => {
                current_block = 6986091146223672350;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            6 => {
                current_block = 4238669146215718434;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            18 => {
                current_block = 10105986042003360997;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            16 => {
                current_block = 1192742893222450069;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            5 => {
                current_block = 6697987161864809334;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            12 => {
                current_block = 4532714271348953913;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            7 => {
                current_block = 17269102997289355865;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            8 => {
                current_block = 1459440320119951532;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            9 => {
                current_block = 905838268264890239;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            10 | 11 => {
                current_block = 14209905858590319049;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
            _ => {
                current_block = 3280871515879575837;
                match current_block {
                    14209905858590319049 => {
                        if (*(*env).syntax).behavior &
                               (1 as libc::c_uint) << 0 as libc::c_int !=
                               0 as libc::c_int as libc::c_uint {
                            if (*(*env).syntax).behavior &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 as libc::c_int as libc::c_uint {
                                return -(113 as libc::c_int)
                            } else { *np = node_new_empty() }
                            current_block = 6186816898867308296;
                        } else { current_block = 4775909272756257391; }
                    }
                    17269102997289355865 => {
                        len = (*tok).u.backref.num;
                        *np =
                            node_new_backref(len,
                                             if len > 1 as libc::c_int {
                                                 (*tok).u.backref.refs
                                             } else {
                                                 &mut (*tok).u.backref.ref1
                                             }, (*tok).u.backref.by_name,
                                             (*tok).u.backref.exist_level,
                                             (*tok).u.backref.level, env);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    4532714271348953913 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        qn =
                            node_new_quantifier(0 as libc::c_int,
                                                -(1 as libc::c_int),
                                                0 as libc::c_int);
                        if (qn as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*qn).u.qtfr.target = *np;
                        *np = qn;
                        current_block = 6186816898867308296;
                    }
                    6697987161864809334 => {
                        *np = node_new_anychar();
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    10105986042003360997 => {
                        r = parse_char_property(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        current_block = 6186816898867308296;
                    }
                    6491452808681056906 => {
                        if !((*(*env).syntax).behavior &
                                 (1 as libc::c_uint) << 2 as libc::c_int !=
                                 0 as libc::c_int as libc::c_uint) {
                            return -(116 as libc::c_int)
                        }
                        if (*tok).escaped != 0 {
                            current_block = 15512526488502093901;
                        } else { current_block = 4775909272756257391; }
                    }
                    1863889588662849341 => {
                        r =
                            parse_enclose(np, tok,
                                          TK_SUBEXP_CLOSE as libc::c_int, src,
                                          end, env);
                        if r < 0 as libc::c_int { return r }
                        if r == 1 as libc::c_int {
                            group = 1 as libc::c_int
                        } else if r == 2 as libc::c_int {
                            let mut target: *mut Node = 0 as *mut Node;
                            let mut prev: OnigOptionType = (*env).option;
                            (*env).option = (**np).u.enclose.option;
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            r =
                                parse_subexp(&mut target, tok, term, src, end,
                                             env);
                            (*env).option = prev;
                            if r < 0 as libc::c_int {
                                onig_node_free(target);
                                return r
                            }
                            (**np).u.enclose.target = target;
                            return (*tok).type_0 as libc::c_int
                        }
                        current_block = 6186816898867308296;
                    }
                    11009730405578905636 => {
                        let mut buf: [OnigUChar; 7] = [0; 7];
                        let mut num: libc::c_int =
                            (*(*env).enc).code_to_mbc.expect("non-null function pointer")((*tok).u.code,
                                                                                          buf.as_mut_ptr());
                        if num < 0 as libc::c_int { return num }
                        *np =
                            node_new_str(buf.as_mut_ptr(),
                                         buf.as_mut_ptr().offset(num as
                                                                     isize));
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        current_block = 6186816898867308296;
                    }
                    6986091146223672350 => {
                        let mut end_op: [OnigCodePoint; 2] = [0; 2];
                        let mut qstart: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut qend: *mut OnigUChar = 0 as *mut OnigUChar;
                        let mut nextp: *mut OnigUChar = 0 as *mut OnigUChar;
                        end_op[0 as libc::c_int as usize] =
                            (*(*env).syntax).meta_char_table.esc;
                        end_op[1 as libc::c_int as usize] =
                            'E' as i32 as OnigCodePoint;
                        qstart = *src;
                        qend =
                            find_str_position(end_op.as_mut_ptr(),
                                              2 as libc::c_int, qstart, end,
                                              &mut nextp, (*env).enc);
                        if (qend as *mut libc::c_void).is_null() {
                            qend = end;
                            nextp = qend
                        }
                        *np = node_new_str(qstart, qend);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        *src = nextp;
                        current_block = 6186816898867308296;
                    }
                    4238669146215718434 => {
                        match (*tok).u.prop.ctype {
                            12 => {
                                *np =
                                    node_new_ctype((*tok).u.prop.ctype,
                                                   (*tok).u.prop.not);
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                            }
                            9 | 4 | 11 => {
                                let mut cc: *mut CClassNode =
                                    0 as *mut CClassNode;
                                *np = node_new_cclass();
                                if (*np as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                cc = &mut (**np).u.cclass;
                                add_ctype_to_cc(cc, (*tok).u.prop.ctype,
                                                0 as libc::c_int, env);
                                if (*tok).u.prop.not != 0 as libc::c_int {
                                    (*cc).flags |=
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int) as libc::c_uint
                                }
                            }
                            _ => { return -(11 as libc::c_int) }
                        }
                        current_block = 6186816898867308296;
                    }
                    1192742893222450069 => {
                        let mut cc_0: *mut CClassNode = 0 as *mut CClassNode;
                        r = parse_char_class(np, tok, src, end, env);
                        if r != 0 as libc::c_int { return r }
                        cc_0 = &mut (**np).u.cclass;
                        if (*env).option & 1 as libc::c_uint != 0 {
                            let mut iarg: IApplyCaseFoldArg =
                                IApplyCaseFoldArg{env: 0 as *mut ScanEnv,
                                                  cc: 0 as *mut CClassNode,
                                                  alt_root: 0 as *mut Node,
                                                  ptail:
                                                      0 as *mut *mut Node,};
                            iarg.env = env;
                            iarg.cc = cc_0;
                            iarg.alt_root = 0 as *mut Node;
                            iarg.ptail = &mut iarg.alt_root;
                            r =
                                (*(*env).enc).apply_all_case_fold.expect("non-null function pointer")((*env).case_fold_flag,
                                                                                                      Some(i_apply_case_fold
                                                                                                               as
                                                                                                               unsafe extern "C" fn(_:
                                                                                                                                        OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        *mut OnigCodePoint,
                                                                                                                                    _:
                                                                                                                                        libc::c_int,
                                                                                                                                    _:
                                                                                                                                        *mut libc::c_void)
                                                                                                                   ->
                                                                                                                       libc::c_int),
                                                                                                      &mut iarg
                                                                                                          as
                                                                                                          *mut IApplyCaseFoldArg
                                                                                                          as
                                                                                                          *mut libc::c_void);
                            if r != 0 as libc::c_int {
                                onig_node_free(iarg.alt_root);
                                return r
                            }
                            if !(iarg.alt_root as *mut libc::c_void).is_null()
                               {
                                let mut work: *mut Node =
                                    onig_node_new_alt(*np, iarg.alt_root);
                                if (work as *mut libc::c_void).is_null() {
                                    onig_node_free(iarg.alt_root);
                                    return -(5 as libc::c_int)
                                }
                                *np = work
                            }
                        }
                        current_block = 6186816898867308296;
                    }
                    1459440320119951532 => {
                        let mut gnum: libc::c_int = (*tok).u.call.gnum;
                        if gnum < 0 as libc::c_int {
                            gnum = (*env).num_mem + 1 as libc::c_int + gnum;
                            if gnum <= 0 as libc::c_int {
                                return -(208 as libc::c_int)
                            }
                        }
                        *np =
                            node_new_call((*tok).u.call.name,
                                          (*tok).u.call.name_end, gnum);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        (*env).num_call += 1;
                        current_block = 6186816898867308296;
                    }
                    905838268264890239 => {
                        *np = onig_node_new_anchor((*tok).u.anchor);
                        current_block = 6186816898867308296;
                    }
                    3280871515879575837 => { return -(11 as libc::c_int) }
                    _ => { }
                }
                match current_block {
                    6186816898867308296 => {
                        targetp = np;
                        current_block = 10018013231291823552;
                    }
                    4775909272756257391 => {
                        *np = node_new_str((*tok).backp, *src);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        loop  {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_STRING as libc::c_int { break ; }
                            r = onig_node_str_cat(*np, (*tok).backp, *src);
                            if r < 0 as libc::c_int { return r }
                        }
                        current_block = 15576022034391501562;
                    }
                    _ => {
                        *np = node_new_str_raw_char((*tok).u.c as OnigUChar);
                        if (*np as *mut libc::c_void).is_null() {
                            return -(5 as libc::c_int)
                        }
                        len = 1 as libc::c_int;
                        loop  {
                            if len >= (*(*env).enc).min_enc_len {
                                if len ==
                                       (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((**np).u.str_0.s)
                                   {
                                    r = fetch_token(tok, src, end, env);
                                    (**np).u.str_0.flag &=
                                        !((1 as libc::c_int) <<
                                              0 as libc::c_int) as
                                            libc::c_uint;
                                    break ;
                                }
                            }
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            if r != TK_RAW_BYTE as libc::c_int {
                                return -(206 as libc::c_int)
                            }
                            r =
                                node_str_cat_char(*np,
                                                  (*tok).u.c as OnigUChar);
                            if r < 0 as libc::c_int { return r }
                            len += 1
                        }
                        current_block = 15576022034391501562;
                    }
                }
                match current_block {
                    15576022034391501562 => {
                        targetp = np;
                        current_block = 10974226118576674975;
                    }
                    _ => { }
                }
                loop  {
                    match current_block {
                        10018013231291823552 => {
                            r = fetch_token(tok, src, end, env);
                            if r < 0 as libc::c_int { return r }
                            current_block = 10974226118576674975;
                        }
                        _ => {
                            if !(r == TK_OP_REPEAT as libc::c_int ||
                                     r == TK_INTERVAL as libc::c_int) {
                                break ;
                            }
                            if is_invalid_quantifier_target(*targetp) != 0 {
                                return -(114 as libc::c_int)
                            }
                            qn =
                                node_new_quantifier((*tok).u.repeat.lower,
                                                    (*tok).u.repeat.upper,
                                                    if r ==
                                                           TK_INTERVAL as
                                                               libc::c_int {
                                                        1 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    });
                            if (qn as *mut libc::c_void).is_null() {
                                return -(5 as libc::c_int)
                            }
                            (*qn).u.qtfr.greedy = (*tok).u.repeat.greedy;
                            r = set_quantifier(qn, *targetp, group, env);
                            if r < 0 as libc::c_int {
                                onig_node_free(qn);
                                return r
                            }
                            if (*tok).u.repeat.possessive != 0 as libc::c_int
                               {
                                let mut en: *mut Node = 0 as *mut Node;
                                en =
                                    node_new_enclose((1 as libc::c_int) <<
                                                         2 as libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.target = qn;
                                qn = en
                            }
                            if r == 0 as libc::c_int {
                                *targetp = qn
                            } else if r == 1 as libc::c_int {
                                onig_node_free(qn);
                            } else if r == 2 as libc::c_int {
                                let mut tmp: *mut Node = 0 as *mut Node;
                                *targetp =
                                    node_new_list(*targetp, 0 as *mut Node);
                                if (*targetp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                (**targetp).u.cons.cdr =
                                    node_new_list(qn, 0 as *mut Node);
                                tmp = (**targetp).u.cons.cdr;
                                if (tmp as *mut libc::c_void).is_null() {
                                    onig_node_free(qn);
                                    return -(5 as libc::c_int)
                                }
                                targetp = &mut (*tmp).u.cons.car
                            }
                            current_block = 10018013231291823552;
                        }
                    }
                }
                return r
            }
        }
    }
    *np = node_new_empty();
    return (*tok).type_0 as libc::c_int;
}
unsafe extern "C" fn parse_branch(mut top: *mut *mut Node,
                                  mut tok: *mut OnigToken,
                                  mut term: libc::c_int,
                                  mut src: *mut *mut OnigUChar,
                                  mut end: *mut OnigUChar,
                                  mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    let mut headp: *mut *mut Node = 0 as *mut *mut Node;
    *top = 0 as *mut Node;
    r = parse_exp(&mut node, tok, term, src, end, env);
    if r < 0 as libc::c_int { onig_node_free(node); return r }
    if r == TK_EOT as libc::c_int || r == term || r == TK_ALT as libc::c_int {
        *top = node
    } else {
        *top = node_new_list(node, 0 as *mut Node);
        headp = &mut (**top).u.cons.cdr;
        while r != TK_EOT as libc::c_int && r != term &&
                  r != TK_ALT as libc::c_int {
            r = parse_exp(&mut node, tok, term, src, end, env);
            if r < 0 as libc::c_int { onig_node_free(node); return r }
            if (*node).u.base.type_0 == 8 as libc::c_int {
                *headp = node;
                while !((*node).u.cons.cdr as *mut libc::c_void).is_null() {
                    node = (*node).u.cons.cdr
                }
                headp = &mut (*node).u.cons.cdr
            } else {
                *headp = node_new_list(node, 0 as *mut Node);
                headp = &mut (**headp).u.cons.cdr
            }
        }
    }
    return r;
}
/* term_tok: TK_EOT or TK_SUBEXP_CLOSE */
unsafe extern "C" fn parse_subexp(mut top: *mut *mut Node,
                                  mut tok: *mut OnigToken,
                                  mut term: libc::c_int,
                                  mut src: *mut *mut OnigUChar,
                                  mut end: *mut OnigUChar,
                                  mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    let mut headp: *mut *mut Node = 0 as *mut *mut Node;
    *top = 0 as *mut Node;
    (*env).parse_depth = (*env).parse_depth.wrapping_add(1);
    if (*env).parse_depth > ParseDepthLimit { return -(16 as libc::c_int) }
    r = parse_branch(&mut node, tok, term, src, end, env);
    if r < 0 as libc::c_int { onig_node_free(node); return r }
    if r == term {
        *top = node
    } else {
        's_144:
            {
                if r == TK_ALT as libc::c_int {
                    *top = onig_node_new_alt(node, 0 as *mut Node);
                    headp = &mut (**top).u.cons.cdr;
                    while r == TK_ALT as libc::c_int {
                        r = fetch_token(tok, src, end, env);
                        if r < 0 as libc::c_int { return r }
                        r = parse_branch(&mut node, tok, term, src, end, env);
                        if r < 0 as libc::c_int {
                            onig_node_free(node);
                            return r
                        }
                        *headp = onig_node_new_alt(node, 0 as *mut Node);
                        headp = &mut (**headp).u.cons.cdr
                    }
                    if !((*tok).type_0 as libc::c_uint !=
                             term as TokenSyms as libc::c_uint) {
                        break 's_144 ;
                    }
                } else { onig_node_free(node); }
                if term == TK_SUBEXP_CLOSE as libc::c_int {
                    return -(117 as libc::c_int)
                } else { return -(11 as libc::c_int) }
            }
    }
    (*env).parse_depth = (*env).parse_depth.wrapping_sub(1);
    return r;
}
unsafe extern "C" fn parse_regexp(mut top: *mut *mut Node,
                                  mut src: *mut *mut OnigUChar,
                                  mut end: *mut OnigUChar,
                                  mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut tok: OnigToken =
        OnigToken{type_0: TK_EOT,
                  escaped: 0,
                  base: 0,
                  backp: 0 as *mut OnigUChar,
                  u: C2RustUnnamed_0{s: 0 as *mut OnigUChar,},};
    r = fetch_token(&mut tok, src, end, env);
    if r < 0 as libc::c_int { return r }
    r = parse_subexp(top, &mut tok, TK_EOT as libc::c_int, src, end, env);
    if r < 0 as libc::c_int { return r }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_parse_make_tree(mut root: *mut *mut Node,
                                              mut pattern: *const OnigUChar,
                                              mut end: *const OnigUChar,
                                              mut reg: *mut regex_t,
                                              mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    names_clear(reg);
    scan_env_clear(env);
    (*env).option = (*reg).options;
    (*env).case_fold_flag = (*reg).case_fold_flag;
    (*env).enc = (*reg).enc;
    (*env).syntax = (*reg).syntax;
    (*env).pattern = pattern as *mut OnigUChar;
    (*env).pattern_end = end as *mut OnigUChar;
    (*env).reg = reg;
    *root = 0 as *mut Node;
    if (*(*env).enc).is_valid_mbc_string.expect("non-null function pointer")(pattern,
                                                                             end)
           == 0 {
        return -(400 as libc::c_int)
    }
    p = pattern as *mut OnigUChar;
    r = parse_regexp(root, &mut p, end as *mut OnigUChar, env);
    (*reg).num_mem = (*env).num_mem;
    return r;
}
/* *********************************************************************
  regparse.h -  Oniguruma (regular expression library)
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
/* node type */
/* node type bit */
/* sizeof(CClassNode) - sizeof(int)*4 */
/* by backslashed number */
/* status bits */
/* STK_REPEAT is nested in stack. */
/* {n,m} */
/* (allocated size - 1) or 0: use buf[] */
/* include called node. don't eliminate even if {0} */
/* for multiple call reference */
/* min length (byte) */
/* max length (byte) */
/* character length  */
/* referenced count in optimize_node_left() */
/* EncloseNode : ENCLOSE_MEMORY */
/* for reg->names only */
#[no_mangle]
pub unsafe extern "C" fn onig_scan_env_set_error_string(mut env: *mut ScanEnv,
                                                        mut ecode:
                                                            libc::c_int,
                                                        mut arg:
                                                            *mut OnigUChar,
                                                        mut arg_end:
                                                            *mut OnigUChar) {
    (*env).error = arg;
    (*env).error_end = arg_end;
}
