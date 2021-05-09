#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn onigenc_init() -> libc::c_int;
    #[no_mangle]
    fn onig_initialize_encoding(enc: OnigEncoding) -> libc::c_int;
    #[no_mangle]
    fn onigenc_strlen(enc: OnigEncoding, p: *const OnigUChar,
                      end: *const OnigUChar) -> libc::c_int;
    #[no_mangle]
    fn onig_names_free(reg: *mut regex_t) -> libc::c_int;
    #[no_mangle]
    fn onig_node_free(node: *mut Node);
    #[no_mangle]
    fn onig_name_to_group_numbers(reg: OnigRegex, name: *const OnigUChar,
                                  name_end: *const OnigUChar,
                                  nums: *mut *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn onig_node_new_anchor(type_0: libc::c_int) -> *mut Node;
    #[no_mangle]
    fn onig_warning(s: *const libc::c_char);
    #[no_mangle]
    fn onig_node_str_cat(node: *mut Node, s: *const OnigUChar,
                         end: *const OnigUChar) -> libc::c_int;
    #[no_mangle]
    fn onig_node_conv_to_str_node(node: *mut Node, raw: libc::c_int);
    #[no_mangle]
    fn onig_parse_make_tree(root: *mut *mut Node, pattern: *const OnigUChar,
                            end: *const OnigUChar, reg: *mut regex_t,
                            env: *mut ScanEnv) -> libc::c_int;
    #[no_mangle]
    fn onig_node_list_add(list: *mut Node, x: *mut Node) -> *mut Node;
    #[no_mangle]
    fn onig_reduce_nested_quantifier(pnode: *mut Node, cnode: *mut Node);
    #[no_mangle]
    fn onig_node_str_set(node: *mut Node, s: *const OnigUChar,
                         end: *const OnigUChar) -> libc::c_int;
    #[no_mangle]
    fn onig_node_new_str(s: *const OnigUChar, end: *const OnigUChar)
     -> *mut Node;
    #[no_mangle]
    fn onig_node_new_alt(left: *mut Node, right: *mut Node) -> *mut Node;
    #[no_mangle]
    fn onig_node_new_list(left: *mut Node, right: *mut Node) -> *mut Node;
    #[no_mangle]
    fn onig_node_new_enclose(type_0: libc::c_int) -> *mut Node;
    #[no_mangle]
    fn onig_renumber_name_table(reg: *mut regex_t, map: *mut GroupNumRemap)
     -> libc::c_int;
    #[no_mangle]
    fn onig_scan_env_set_error_string(env: *mut ScanEnv, ecode: libc::c_int,
                                      arg: *mut OnigUChar,
                                      arg_end: *mut OnigUChar);
}
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
pub struct OnigErrorInfo {
    pub enc: OnigEncoding,
    pub par: *mut OnigUChar,
    pub par_end: *mut OnigUChar,
}
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
pub type Node = _Node;
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
pub struct NodeBase {
    pub type_0: libc::c_int,
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
pub type AbsAddrType = libc::c_int;
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
/* char map */
/* bit status */
pub type BitStatusType = libc::c_uint;
pub const OP_END: OpCode = 1;
pub const OP_FAIL_LOOK_BEHIND_NOT: OpCode = 78;
pub type LengthType = libc::c_int;
pub const OP_PUSH_LOOK_BEHIND_NOT: OpCode = 77;
pub type RelAddrType = libc::c_int;
pub type MemNumType = libc::c_short;
pub type BitSetRef = *mut Bits;
pub type PointerType = *mut libc::c_void;
pub const OP_EXACTN_IC: OpCode = 15;
pub const OP_EXACTMBN: OpCode = 13;
pub const OP_EXACTMB3N: OpCode = 12;
pub const OP_EXACTMB2N: OpCode = 11;
pub const OP_EXACTMB2N3: OpCode = 10;
pub const OP_EXACTMB2N2: OpCode = 9;
pub const OP_EXACTMB2N1: OpCode = 8;
pub const OP_EXACTN: OpCode = 7;
pub const OP_EXACT5: OpCode = 6;
pub const OP_EXACT4: OpCode = 5;
pub const OP_EXACT3: OpCode = 4;
pub const OP_EXACT2: OpCode = 3;
pub const OP_EXACT1: OpCode = 2;
pub const OP_EXACT1_IC: OpCode = 14;
pub const OP_LOOK_BEHIND: OpCode = 76;
pub const OP_FAIL_POS: OpCode = 73;
pub const OP_PUSH_POS_NOT: OpCode = 72;
pub const OP_POP_POS: OpCode = 71;
pub const OP_PUSH_POS: OpCode = 70;
pub const OP_WORD_END: OpCode = 34;
pub const OP_WORD_BEGIN: OpCode = 33;
pub const OP_NOT_WORD_BOUND: OpCode = 32;
pub const OP_WORD_BOUND: OpCode = 31;
pub const OP_BEGIN_POSITION: OpCode = 40;
pub const OP_SEMI_END_BUF: OpCode = 39;
pub const OP_END_LINE: OpCode = 38;
pub const OP_BEGIN_LINE: OpCode = 37;
pub const OP_END_BUF: OpCode = 36;
pub const OP_BEGIN_BUF: OpCode = 35;
pub const OP_POP_STOP_BT: OpCode = 75;
pub const OP_PUSH_STOP_BT: OpCode = 74;
pub const OP_JUMP: OpCode = 55;
pub const OP_POP: OpCode = 57;
pub const OP_PUSH: OpCode = 56;
pub const OP_MEMORY_END: OpCode = 52;
pub const OP_MEMORY_END_PUSH: OpCode = 50;
pub const OP_MEMORY_END_REC: OpCode = 53;
pub const OP_MEMORY_END_PUSH_REC: OpCode = 51;
pub const OP_RETURN: OpCode = 80;
pub const OP_MEMORY_START: OpCode = 48;
pub const OP_MEMORY_START_PUSH: OpCode = 49;
pub const OP_CALL: OpCode = 79;
pub const OP_SET_OPTION: OpCode = 87;
pub const OP_FAIL: OpCode = 54;
pub const OP_SET_OPTION_PUSH: OpCode = 86;
pub const OP_REPEAT_INC_NG: OpCode = 63;
pub const OP_REPEAT_INC: OpCode = 62;
pub const OP_REPEAT_INC_NG_SG: OpCode = 65;
pub const OP_REPEAT_INC_SG: OpCode = 64;
pub const OP_NULL_CHECK_END_MEMST_PUSH: OpCode = 69;
pub const OP_NULL_CHECK_END_MEMST: OpCode = 68;
pub const OP_NULL_CHECK_END: OpCode = 67;
pub const OP_NULL_CHECK_START: OpCode = 66;
pub const OP_REPEAT_NG: OpCode = 61;
pub const OP_REPEAT: OpCode = 60;
pub const OP_PUSH_IF_PEEK_NEXT: OpCode = 59;
pub const OP_PUSH_OR_JUMP_EXACT1: OpCode = 58;
pub const OP_ANYCHAR_STAR: OpCode = 25;
pub const OP_ANYCHAR_ML_STAR: OpCode = 26;
pub const OP_ANYCHAR_STAR_PEEK_NEXT: OpCode = 27;
pub const OP_ANYCHAR_ML_STAR_PEEK_NEXT: OpCode = 28;
pub const OP_BACKREF_MULTI: OpCode = 45;
pub const OP_BACKREF_MULTI_IC: OpCode = 46;
pub const OP_BACKREFN: OpCode = 43;
pub const OP_BACKREF2: OpCode = 42;
pub const OP_BACKREF1: OpCode = 41;
pub const OP_BACKREFN_IC: OpCode = 44;
pub const OP_BACKREF_WITH_LEVEL: OpCode = 47;
pub const OP_ANYCHAR: OpCode = 23;
pub const OP_ANYCHAR_ML: OpCode = 24;
pub const OP_WORD: OpCode = 29;
pub const OP_NOT_WORD: OpCode = 30;
pub const OP_CCLASS_MIX: OpCode = 18;
pub const OP_CCLASS_MIX_NOT: OpCode = 21;
pub const OP_CCLASS_MB: OpCode = 17;
pub const OP_CCLASS_MB_NOT: OpCode = 20;
pub const OP_CCLASS: OpCode = 16;
pub const OP_CCLASS_NOT: OpCode = 19;
pub const OP_CCLASS_NODE: OpCode = 22;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OptAncInfo {
    pub left_anchor: libc::c_int,
    pub right_anchor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeOptInfo {
    pub len: MinMaxLen,
    pub anc: OptAncInfo,
    pub exb: OptExactInfo,
    pub exm: OptExactInfo,
    pub expr: OptExactInfo,
    pub map: OptMapInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OptMapInfo {
    pub mmd: MinMaxLen,
    pub anc: OptAncInfo,
    pub value: libc::c_int,
    pub map: [OnigUChar; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinMaxLen {
    pub min: OnigLen,
    pub max: OnigLen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OptExactInfo {
    pub mmd: MinMaxLen,
    pub anc: OptAncInfo,
    pub reach_end: libc::c_int,
    pub ignore_case: libc::c_int,
    pub len: libc::c_int,
    pub s: [OnigUChar; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OptEnv {
    pub mmd: MinMaxLen,
    pub enc: OnigEncoding,
    pub options: OnigOptionType,
    pub case_fold_flag: OnigCaseFoldType,
    pub scan_env: *mut ScanEnv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupNumRemap {
    pub new_val: libc::c_int,
}
pub type OnigEndCallListItemType = OnigEndCallListItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnigEndCallListItem {
    pub next: *mut OnigEndCallListItem,
    pub func: Option<unsafe extern "C" fn() -> ()>,
}
pub type OpCode = libc::c_uint;
pub const OP_STATE_CHECK_ANYCHAR_ML_STAR: OpCode = 85;
pub const OP_STATE_CHECK_ANYCHAR_STAR: OpCode = 84;
pub const OP_STATE_CHECK: OpCode = 83;
pub const OP_STATE_CHECK_PUSH_OR_JUMP: OpCode = 82;
pub const OP_STATE_CHECK_PUSH: OpCode = 81;
pub const OP_FINISH: OpCode = 0;
/* *********************************************************************
  regcomp.c -  Oniguruma (regular expression library)
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
pub static mut OnigDefaultCaseFoldFlag: OnigCaseFoldType =
    ((1 as libc::c_int) << 30 as libc::c_int) as OnigCaseFoldType;
#[no_mangle]
pub unsafe extern "C" fn onig_get_default_case_fold_flag()
 -> OnigCaseFoldType {
    return OnigDefaultCaseFoldFlag;
}
#[no_mangle]
pub unsafe extern "C" fn onig_set_default_case_fold_flag(mut case_fold_flag:
                                                             OnigCaseFoldType)
 -> libc::c_int {
    OnigDefaultCaseFoldFlag = case_fold_flag;
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_dup(mut s: *mut OnigUChar, mut end: *mut OnigUChar)
 -> *mut OnigUChar {
    let mut len: libc::c_int =
        end.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    if len > 0 as libc::c_int {
        let mut r: *mut OnigUChar =
            malloc((len + 1 as libc::c_int) as libc::c_ulong) as
                *mut OnigUChar;
        if (r as *mut libc::c_void).is_null() { return 0 as *mut OnigUChar }
        memcpy(r as *mut libc::c_void, s as *const libc::c_void,
               len as libc::c_ulong);
        *r.offset(len as isize) = 0 as libc::c_int as OnigUChar;
        return r
    } else { return 0 as *mut OnigUChar };
}
unsafe extern "C" fn swap_node(mut a: *mut Node, mut b: *mut Node) {
    let mut c: Node = Node{u: C2RustUnnamed{base: NodeBase{type_0: 0,},},};
    c = *a;
    *a = *b;
    *b = c;
    if (*a).u.base.type_0 == 0 as libc::c_int {
        let mut sn: *mut StrNode = &mut (*a).u.str_0;
        if (*sn).capa == 0 as libc::c_int {
            let mut len: libc::c_int =
                (*sn).end.wrapping_offset_from((*sn).s) as libc::c_long as
                    libc::c_int;
            (*sn).s = (*sn).buf.as_mut_ptr();
            (*sn).end = (*sn).s.offset(len as isize)
        }
    }
    if (*b).u.base.type_0 == 0 as libc::c_int {
        let mut sn_0: *mut StrNode = &mut (*b).u.str_0;
        if (*sn_0).capa == 0 as libc::c_int {
            let mut len_0: libc::c_int =
                (*sn_0).end.wrapping_offset_from((*sn_0).s) as libc::c_long as
                    libc::c_int;
            (*sn_0).s = (*sn_0).buf.as_mut_ptr();
            (*sn_0).end = (*sn_0).s.offset(len_0 as isize)
        }
    };
}
unsafe extern "C" fn distance_add(mut d1: OnigLen, mut d2: OnigLen)
 -> OnigLen {
    if d1 == !(0 as libc::c_int as OnigLen) ||
           d2 == !(0 as libc::c_int as OnigLen) {
        return !(0 as libc::c_int as OnigLen)
    } else if d1 <= (!(0 as libc::c_int as OnigLen)).wrapping_sub(d2) {
        return d1.wrapping_add(d2)
    } else { return !(0 as libc::c_int as OnigLen) };
}
unsafe extern "C" fn distance_multiply(mut d: OnigLen, mut m: libc::c_int)
 -> OnigLen {
    if m == 0 as libc::c_int { return 0 as libc::c_int as OnigLen }
    if d < (!(0 as libc::c_int as OnigLen)).wrapping_div(m as libc::c_uint) {
        return d.wrapping_mul(m as libc::c_uint)
    } else { return !(0 as libc::c_int as OnigLen) };
}
unsafe extern "C" fn bitset_is_empty(mut bs: BitSetRef) -> libc::c_int {
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
        if *bs.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_bbuf_init(mut buf: *mut BBuf,
                                        mut size: libc::c_int)
 -> libc::c_int {
    if size <= 0 as libc::c_int {
        size = 0 as libc::c_int;
        (*buf).p = 0 as *mut OnigUChar
    } else {
        (*buf).p = malloc(size as libc::c_ulong) as *mut OnigUChar;
        if ((*buf).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    (*buf).alloc = size as libc::c_uint;
    (*buf).used = 0 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn unset_addr_list_init(mut uslist: *mut UnsetAddrList,
                                          mut size: libc::c_int)
 -> libc::c_int {
    let mut p: *mut UnsetAddr = 0 as *mut UnsetAddr;
    p =
        malloc((::std::mem::size_of::<UnsetAddr>() as
                    libc::c_ulong).wrapping_mul(size as libc::c_ulong)) as
            *mut UnsetAddr;
    if (p as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    (*uslist).num = 0 as libc::c_int;
    (*uslist).alloc = size;
    (*uslist).us = p;
    return 0 as libc::c_int;
}
unsafe extern "C" fn unset_addr_list_end(mut uslist: *mut UnsetAddrList) {
    if !((*uslist).us as *mut libc::c_void).is_null() {
        free((*uslist).us as *mut libc::c_void);
    };
}
unsafe extern "C" fn unset_addr_list_add(mut uslist: *mut UnsetAddrList,
                                         mut offset: libc::c_int,
                                         mut node: *mut _Node)
 -> libc::c_int {
    let mut p: *mut UnsetAddr = 0 as *mut UnsetAddr;
    let mut size: libc::c_int = 0;
    if (*uslist).num >= (*uslist).alloc {
        size = (*uslist).alloc * 2 as libc::c_int;
        p =
            realloc((*uslist).us as *mut libc::c_void,
                    (::std::mem::size_of::<UnsetAddr>() as
                         libc::c_ulong).wrapping_mul(size as libc::c_ulong))
                as *mut UnsetAddr;
        if (p as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
        (*uslist).alloc = size;
        (*uslist).us = p
    }
    (*(*uslist).us.offset((*uslist).num as isize)).offset = offset;
    let ref mut fresh0 =
        (*(*uslist).us.offset((*uslist).num as isize)).target;
    *fresh0 = node;
    (*uslist).num += 1;
    return 0 as libc::c_int;
}
/* USE_SUBEXP_CALL */
unsafe extern "C" fn add_opcode(mut reg: *mut regex_t,
                                mut opcode: libc::c_int) -> libc::c_int {
    let mut used: libc::c_int =
        (*reg).used.wrapping_add(1 as libc::c_int as libc::c_uint) as
            libc::c_int; /* NULL CHECK ID */
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    *(*reg).p.offset((*reg).used as isize) = opcode as libc::c_uchar;
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_rel_addr(mut reg: *mut regex_t,
                                  mut addr: libc::c_int) -> libc::c_int {
    let mut ra: RelAddrType = addr;
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                             as libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           &mut ra as *mut RelAddrType as *const libc::c_void,
           ::std::mem::size_of::<RelAddrType>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_abs_addr(mut reg: *mut regex_t,
                                  mut addr: libc::c_int) -> libc::c_int {
    let mut ra: AbsAddrType = addr;
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<AbsAddrType>()
                                             as libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           &mut ra as *mut AbsAddrType as *const libc::c_void,
           ::std::mem::size_of::<AbsAddrType>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_length(mut reg: *mut regex_t, mut len: libc::c_int)
 -> libc::c_int {
    let mut l: LengthType = len;
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                             as libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           &mut l as *mut LengthType as *const libc::c_void,
           ::std::mem::size_of::<LengthType>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_mem_num(mut reg: *mut regex_t, mut num: libc::c_int)
 -> libc::c_int {
    let mut n: MemNumType = num as MemNumType;
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                             as libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           &mut n as *mut MemNumType as *const libc::c_void,
           ::std::mem::size_of::<MemNumType>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_pointer(mut reg: *mut regex_t,
                                 mut addr: *mut libc::c_void) -> libc::c_int {
    let mut ptr: PointerType = addr;
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<PointerType>()
                                             as libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           &mut ptr as *mut PointerType as *const libc::c_void,
           ::std::mem::size_of::<PointerType>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_option(mut reg: *mut regex_t,
                                mut option: OnigOptionType) -> libc::c_int {
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigOptionType>()
                                             as libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           &mut option as *mut OnigOptionType as *const libc::c_void,
           ::std::mem::size_of::<OnigOptionType>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_opcode_rel_addr(mut reg: *mut regex_t,
                                         mut opcode: libc::c_int,
                                         mut addr: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = add_opcode(reg, opcode);
    if r != 0 { return r }
    r = add_rel_addr(reg, addr);
    return r;
}
unsafe extern "C" fn add_bytes(mut reg: *mut regex_t,
                               mut bytes: *mut OnigUChar,
                               mut len: libc::c_int) -> libc::c_int {
    let mut used: libc::c_int =
        (*reg).used.wrapping_add(len as libc::c_uint) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           bytes as *const libc::c_void, len as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_bitset(mut reg: *mut regex_t, mut bs: BitSetRef)
 -> libc::c_int {
    let mut used: libc::c_int =
        ((*reg).used as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<BitSet>() as
                                             libc::c_ulong) as libc::c_int;
    if (*reg).alloc < used as libc::c_uint {
        loop  {
            (*reg).alloc =
                (*reg).alloc.wrapping_mul(2 as libc::c_int as libc::c_uint);
            if !((*reg).alloc < used as libc::c_uint) { break ; }
        }
        (*reg).p =
            realloc((*reg).p as *mut libc::c_void,
                    (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).p as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
    }
    memcpy((*reg).p.offset((*reg).used as isize) as *mut libc::c_void,
           bs as *const libc::c_void,
           ::std::mem::size_of::<BitSet>() as libc::c_ulong);
    if (*reg).used < used as libc::c_uint {
        (*reg).used = used as libc::c_uint
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_opcode_option(mut reg: *mut regex_t,
                                       mut opcode: libc::c_int,
                                       mut option: OnigOptionType)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = add_opcode(reg, opcode);
    if r != 0 { return r }
    r = add_option(reg, option);
    return r;
}
unsafe extern "C" fn select_str_opcode(mut mb_len: libc::c_int,
                                       mut str_len: libc::c_int,
                                       mut ignore_case: libc::c_int)
 -> libc::c_int {
    let mut op: libc::c_int = 0;
    if ignore_case != 0 {
        match str_len {
            1 => { op = OP_EXACT1_IC as libc::c_int }
            _ => { op = OP_EXACTN_IC as libc::c_int }
        }
    } else {
        match mb_len {
            1 => {
                match str_len {
                    1 => { op = OP_EXACT1 as libc::c_int }
                    2 => { op = OP_EXACT2 as libc::c_int }
                    3 => { op = OP_EXACT3 as libc::c_int }
                    4 => { op = OP_EXACT4 as libc::c_int }
                    5 => { op = OP_EXACT5 as libc::c_int }
                    _ => { op = OP_EXACTN as libc::c_int }
                }
            }
            2 => {
                match str_len {
                    1 => { op = OP_EXACTMB2N1 as libc::c_int }
                    2 => { op = OP_EXACTMB2N2 as libc::c_int }
                    3 => { op = OP_EXACTMB2N3 as libc::c_int }
                    _ => { op = OP_EXACTMB2N as libc::c_int }
                }
            }
            3 => { op = OP_EXACTMB3N as libc::c_int }
            _ => { op = OP_EXACTMBN as libc::c_int }
        }
    }
    return op;
}
unsafe extern "C" fn compile_tree_empty_check(mut node: *mut Node,
                                              mut reg: *mut regex_t,
                                              mut empty_info: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut saved_num_null_check: libc::c_int = (*reg).num_null_check;
    if empty_info != 0 as libc::c_int {
        r = add_opcode(reg, OP_NULL_CHECK_START as libc::c_int);
        if r != 0 { return r }
        r = add_mem_num(reg, (*reg).num_null_check);
        if r != 0 { return r }
        (*reg).num_null_check += 1
    }
    r = compile_tree(node, reg);
    if r != 0 { return r }
    if empty_info != 0 as libc::c_int {
        if empty_info == 1 as libc::c_int {
            r = add_opcode(reg, OP_NULL_CHECK_END as libc::c_int)
        } else if empty_info == 2 as libc::c_int {
            r = add_opcode(reg, OP_NULL_CHECK_END_MEMST as libc::c_int)
        } else if empty_info == 3 as libc::c_int {
            r = add_opcode(reg, OP_NULL_CHECK_END_MEMST_PUSH as libc::c_int)
        }
        if r != 0 { return r }
        r = add_mem_num(reg, saved_num_null_check)
        /* NULL CHECK ID */
    } /* OP_REPEAT ID */
    return r; /* OP_REPEAT ID */
}
unsafe extern "C" fn compile_call(mut node: *mut CallNode,
                                  mut reg: *mut regex_t) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = add_opcode(reg, OP_CALL as libc::c_int);
    if r != 0 { return r }
    r =
        unset_addr_list_add((*node).unset_addr_list,
                            (*reg).used as libc::c_int, (*node).target);
    if r != 0 { return r }
    r = add_abs_addr(reg, 0 as libc::c_int);
    return r;
}
unsafe extern "C" fn compile_tree_n_times(mut node: *mut Node,
                                          mut n: libc::c_int,
                                          mut reg: *mut regex_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n { r = compile_tree(node, reg); if r != 0 { return r } i += 1 }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_compile_string_length(mut s: *mut OnigUChar,
                                               mut mb_len: libc::c_int,
                                               mut str_len: libc::c_int,
                                               mut reg: *mut regex_t,
                                               mut ignore_case: libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut op: libc::c_int = select_str_opcode(mb_len, str_len, ignore_case);
    len = 1 as libc::c_int;
    if op == OP_EXACTMBN as libc::c_int {
        len =
            (len as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                 as libc::c_ulong) as
                libc::c_int as libc::c_int
    }
    if op == OP_EXACTN as libc::c_int || op == OP_EXACTMB2N as libc::c_int ||
           op == OP_EXACTMB3N as libc::c_int ||
           op == OP_EXACTMBN as libc::c_int ||
           op == OP_EXACTN_IC as libc::c_int {
        len =
            (len as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                 as libc::c_ulong) as
                libc::c_int as libc::c_int
    }
    len += mb_len * str_len;
    return len;
}
unsafe extern "C" fn add_compile_string(mut s: *mut OnigUChar,
                                        mut mb_len: libc::c_int,
                                        mut str_len: libc::c_int,
                                        mut reg: *mut regex_t,
                                        mut ignore_case: libc::c_int)
 -> libc::c_int {
    let mut op: libc::c_int = select_str_opcode(mb_len, str_len, ignore_case);
    add_opcode(reg, op);
    if op == OP_EXACTMBN as libc::c_int { add_length(reg, mb_len); }
    if op == OP_EXACTN as libc::c_int || op == OP_EXACTMB2N as libc::c_int ||
           op == OP_EXACTMB3N as libc::c_int ||
           op == OP_EXACTMBN as libc::c_int ||
           op == OP_EXACTN_IC as libc::c_int {
        if op == OP_EXACTN_IC as libc::c_int {
            add_length(reg, mb_len * str_len);
        } else { add_length(reg, str_len); }
    }
    add_bytes(reg, s, mb_len * str_len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn compile_length_string_node(mut node: *mut Node,
                                                mut reg: *mut regex_t)
 -> libc::c_int {
    let mut rlen: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut prev_len: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut ambig: libc::c_int = 0;
    let mut enc: OnigEncoding = (*reg).enc;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut sn: *mut StrNode = 0 as *mut StrNode;
    sn = &mut (*node).u.str_0;
    if (*sn).end <= (*sn).s { return 0 as libc::c_int }
    ambig =
        ((*node).u.str_0.flag &
             ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int;
    prev = (*sn).s;
    p = prev;
    prev_len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
    p = p.offset(prev_len as isize);
    slen = 1 as libc::c_int;
    rlen = 0 as libc::c_int;
    while p < (*sn).end {
        len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
        if len == prev_len {
            slen += 1
        } else {
            r = add_compile_string_length(prev, prev_len, slen, reg, ambig);
            rlen += r;
            prev = p;
            slen = 1 as libc::c_int;
            prev_len = len
        }
        p = p.offset(len as isize)
    }
    r = add_compile_string_length(prev, prev_len, slen, reg, ambig);
    rlen += r;
    return rlen;
}
unsafe extern "C" fn compile_length_string_raw_node(mut sn: *mut StrNode,
                                                    mut reg: *mut regex_t)
 -> libc::c_int {
    if (*sn).end <= (*sn).s { return 0 as libc::c_int }
    return add_compile_string_length((*sn).s, 1 as libc::c_int,
                                     (*sn).end.wrapping_offset_from((*sn).s)
                                         as libc::c_long as libc::c_int, reg,
                                     0 as libc::c_int);
}
unsafe extern "C" fn compile_string_node(mut node: *mut Node,
                                         mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut prev_len: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut ambig: libc::c_int = 0;
    let mut enc: OnigEncoding = (*reg).enc;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut prev: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut sn: *mut StrNode = 0 as *mut StrNode;
    sn = &mut (*node).u.str_0;
    if (*sn).end <= (*sn).s { return 0 as libc::c_int }
    end = (*sn).end;
    ambig =
        ((*node).u.str_0.flag &
             ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int;
    prev = (*sn).s;
    p = prev;
    prev_len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
    p = p.offset(prev_len as isize);
    slen = 1 as libc::c_int;
    while p < end {
        len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
        if len == prev_len {
            slen += 1
        } else {
            r = add_compile_string(prev, prev_len, slen, reg, ambig);
            if r != 0 { return r }
            prev = p;
            slen = 1 as libc::c_int;
            prev_len = len
        }
        p = p.offset(len as isize)
    }
    return add_compile_string(prev, prev_len, slen, reg, ambig);
}
unsafe extern "C" fn compile_string_raw_node(mut sn: *mut StrNode,
                                             mut reg: *mut regex_t)
 -> libc::c_int {
    if (*sn).end <= (*sn).s { return 0 as libc::c_int }
    return add_compile_string((*sn).s, 1 as libc::c_int,
                              (*sn).end.wrapping_offset_from((*sn).s) as
                                  libc::c_long as libc::c_int, reg,
                              0 as libc::c_int);
}
unsafe extern "C" fn add_multi_byte_cclass(mut mbuf: *mut BBuf,
                                           mut reg: *mut regex_t)
 -> libc::c_int {
    add_length(reg, (*mbuf).used as libc::c_int);
    return add_bytes(reg, (*mbuf).p, (*mbuf).used as libc::c_int);
}
unsafe extern "C" fn compile_length_cclass_node(mut cc: *mut CClassNode,
                                                mut reg: *mut regex_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    if (*cc).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
           != 0 as libc::c_int as libc::c_uint {
        len =
            (1 as libc::c_int as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<PointerType>()
                                                 as libc::c_ulong) as
                libc::c_int;
        return len
    }
    if ((*cc).mbuf as *mut libc::c_void).is_null() {
        len =
            (1 as libc::c_int as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<BitSet>()
                                                 as libc::c_ulong) as
                libc::c_int
    } else {
        if (*(*reg).enc).min_enc_len > 1 as libc::c_int ||
               bitset_is_empty((*cc).bs.as_mut_ptr()) != 0 {
            len = 1 as libc::c_int
        } else {
            len =
                (1 as libc::c_int as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<BitSet>()
                                                     as libc::c_ulong) as
                    libc::c_int
        }
        len =
            (len as
                 libc::c_ulong).wrapping_add((::std::mem::size_of::<LengthType>()
                                                  as
                                                  libc::c_ulong).wrapping_add((*(*cc).mbuf).used
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int
    }
    return len;
}
unsafe extern "C" fn compile_cclass_node(mut cc: *mut CClassNode,
                                         mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*cc).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
           != 0 as libc::c_int as libc::c_uint {
        add_opcode(reg, OP_CCLASS_NODE as libc::c_int);
        r = add_pointer(reg, cc as *mut libc::c_void);
        return r
    }
    if ((*cc).mbuf as *mut libc::c_void).is_null() {
        if (*cc).flags &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            add_opcode(reg, OP_CCLASS_NOT as libc::c_int);
        } else { add_opcode(reg, OP_CCLASS as libc::c_int); }
        r = add_bitset(reg, (*cc).bs.as_mut_ptr())
    } else if (*(*reg).enc).min_enc_len > 1 as libc::c_int ||
                  bitset_is_empty((*cc).bs.as_mut_ptr()) != 0 {
        if (*cc).flags &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            add_opcode(reg, OP_CCLASS_MB_NOT as libc::c_int);
        } else { add_opcode(reg, OP_CCLASS_MB as libc::c_int); }
        r = add_multi_byte_cclass((*cc).mbuf, reg)
    } else {
        if (*cc).flags &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            add_opcode(reg, OP_CCLASS_MIX_NOT as libc::c_int);
        } else { add_opcode(reg, OP_CCLASS_MIX as libc::c_int); }
        r = add_bitset(reg, (*cc).bs.as_mut_ptr());
        if r != 0 { return r }
        r = add_multi_byte_cclass((*cc).mbuf, reg)
    }
    return r;
}
unsafe extern "C" fn entry_repeat_range(mut reg: *mut regex_t,
                                        mut id: libc::c_int,
                                        mut lower: libc::c_int,
                                        mut upper: libc::c_int)
 -> libc::c_int {
    let mut p: *mut OnigRepeatRange = 0 as *mut OnigRepeatRange;
    if (*reg).repeat_range_alloc == 0 as libc::c_int {
        p =
            malloc((::std::mem::size_of::<OnigRepeatRange>() as
                        libc::c_ulong).wrapping_mul(4 as libc::c_int as
                                                        libc::c_ulong)) as
                *mut OnigRepeatRange;
        if (p as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
        (*reg).repeat_range = p;
        (*reg).repeat_range_alloc = 4 as libc::c_int
    } else if (*reg).repeat_range_alloc <= id {
        let mut n: libc::c_int = 0;
        n = (*reg).repeat_range_alloc + 4 as libc::c_int;
        p =
            realloc((*reg).repeat_range as *mut libc::c_void,
                    (::std::mem::size_of::<OnigRepeatRange>() as
                         libc::c_ulong).wrapping_mul(n as libc::c_ulong)) as
                *mut OnigRepeatRange;
        if (p as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
        (*reg).repeat_range = p;
        (*reg).repeat_range_alloc = n
    } else { p = (*reg).repeat_range }
    (*p.offset(id as isize)).lower = lower;
    (*p.offset(id as isize)).upper =
        if upper == -(1 as libc::c_int) {
            0x7fffffff as libc::c_int
        } else { upper };
    return 0 as libc::c_int;
}
unsafe extern "C" fn compile_range_repeat_node(mut qn: *mut QtfrNode,
                                               mut target_len: libc::c_int,
                                               mut empty_info: libc::c_int,
                                               mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut num_repeat: libc::c_int = (*reg).num_repeat;
    r =
        add_opcode(reg,
                   if (*qn).greedy != 0 {
                       OP_REPEAT as libc::c_int
                   } else { OP_REPEAT_NG as libc::c_int });
    if r != 0 { return r }
    r = add_mem_num(reg, num_repeat);
    (*reg).num_repeat += 1;
    if r != 0 { return r }
    r =
        add_rel_addr(reg,
                     (target_len as
                          libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                           as
                                                                                           libc::c_ulong))
                         as libc::c_int);
    if r != 0 { return r }
    r = entry_repeat_range(reg, num_repeat, (*qn).lower, (*qn).upper);
    if r != 0 { return r }
    r = compile_tree_empty_check((*qn).target, reg, empty_info);
    if r != 0 { return r }
    if (*reg).num_call > 0 as libc::c_int ||
           (*qn).state & (1 as libc::c_int) << 12 as libc::c_int !=
               0 as libc::c_int {
        r =
            add_opcode(reg,
                       if (*qn).greedy != 0 {
                           OP_REPEAT_INC_SG as libc::c_int
                       } else { OP_REPEAT_INC_NG_SG as libc::c_int })
    } else {
        r =
            add_opcode(reg,
                       if (*qn).greedy != 0 {
                           OP_REPEAT_INC as libc::c_int
                       } else { OP_REPEAT_INC_NG as libc::c_int })
    }
    if r != 0 { return r }
    r = add_mem_num(reg, num_repeat);
    return r;
}
unsafe extern "C" fn is_anychar_star_quantifier(mut qn: *mut QtfrNode)
 -> libc::c_int {
    if (*qn).greedy != 0 && (*qn).upper == -(1 as libc::c_int) &&
           (*(*qn).target).u.base.type_0 == 3 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* USE_COMBINATION_EXPLOSION_CHECK */
unsafe extern "C" fn compile_length_quantifier_node(mut qn: *mut QtfrNode,
                                                    mut reg: *mut regex_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut mod_tlen: libc::c_int = 0;
    let mut infinite: libc::c_int =
        ((*qn).upper == -(1 as libc::c_int)) as libc::c_int;
    let mut empty_info: libc::c_int = (*qn).target_empty_info;
    let mut tlen: libc::c_int = compile_length_tree((*qn).target, reg);
    if tlen < 0 as libc::c_int { return tlen }
    /* anychar repeat */
    if (*(*qn).target).u.base.type_0 == 3 as libc::c_int {
        if (*qn).greedy != 0 && infinite != 0 {
            if !((*qn).next_head_exact as *mut libc::c_void).is_null() {
                return 1 as libc::c_int + 1 as libc::c_int +
                           tlen * (*qn).lower
            } else { return 1 as libc::c_int + tlen * (*qn).lower }
        }
    }
    if empty_info != 0 as libc::c_int {
        mod_tlen =
            (tlen as
                 libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add((1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)))
                as libc::c_int
    } else { mod_tlen = tlen }
    if infinite != 0 &&
           ((*qn).lower <= 1 as libc::c_int ||
                tlen * (*qn).lower <= 50 as libc::c_int) {
        if (*qn).lower == 1 as libc::c_int && tlen > 50 as libc::c_int {
            len =
                (1 as libc::c_int as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                     as libc::c_ulong) as
                    libc::c_int
        } else { len = tlen * (*qn).lower }
        if (*qn).greedy != 0 {
            if !((*qn).head_exact as *mut libc::c_void).is_null() {
                len =
                    (len as
                         libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_add(1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_add(mod_tlen
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong).wrapping_add((1
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           libc::c_ulong)))
                        as libc::c_int as libc::c_int
            } else if !((*qn).next_head_exact as *mut libc::c_void).is_null()
             {
                len =
                    (len as
                         libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_add(1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_add(mod_tlen
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong).wrapping_add((1
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                                                                                           as
                                                                                                                                                                                                                           libc::c_ulong)))
                        as libc::c_int as libc::c_int
            } else {
                len =
                    (len as
                         libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_add(mod_tlen
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_add((1
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_ulong)))
                        as libc::c_int as libc::c_int
            }
        } else {
            len =
                (len as
                     libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_add(mod_tlen
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_add((1
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                                                       as
                                                                                                                                                                                       libc::c_ulong)))
                    as libc::c_int as libc::c_int
        }
    } else if (*qn).upper == 0 as libc::c_int &&
                  (*qn).is_refered != 0 as libc::c_int {
        /* /(?<n>..){0}/ */
        len =
            (1 as libc::c_int as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                 as
                                                 libc::c_ulong).wrapping_add(tlen
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int
    } else if infinite == 0 && (*qn).greedy != 0 &&
                  ((*qn).upper == 1 as libc::c_int ||
                       (tlen as
                            libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                             as
                                                                                             libc::c_ulong)).wrapping_mul((*qn).upper
                                                                                                                              as
                                                                                                                              libc::c_ulong)
                           <= 50 as libc::c_int as libc::c_ulong) {
        len = tlen * (*qn).lower;
        len =
            (len as
                 libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add(tlen
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_mul(((*qn).upper
                                                                                                                                                   -
                                                                                                                                                   (*qn).lower)
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int
    } else if (*qn).greedy == 0 && (*qn).upper == 1 as libc::c_int &&
                  (*qn).lower == 0 as libc::c_int {
        /* '??' */
        len =
            (1 as libc::c_int as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                 as
                                                 libc::c_ulong).wrapping_add((1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong)).wrapping_add(tlen
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)
                as libc::c_int
    } else {
        len =
            (1 as libc::c_int as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                 as
                                                 libc::c_ulong).wrapping_add(mod_tlen
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_ulong)
                as libc::c_int
    }
    return len;
}
unsafe extern "C" fn compile_quantifier_node(mut qn: *mut QtfrNode,
                                             mut reg: *mut regex_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut mod_tlen: libc::c_int = 0;
    let mut infinite: libc::c_int =
        ((*qn).upper == -(1 as libc::c_int)) as libc::c_int;
    let mut empty_info: libc::c_int = (*qn).target_empty_info;
    let mut tlen: libc::c_int = compile_length_tree((*qn).target, reg);
    if tlen < 0 as libc::c_int { return tlen }
    if is_anychar_star_quantifier(qn) != 0 {
        r = compile_tree_n_times((*qn).target, (*qn).lower, reg);
        if r != 0 { return r }
        if !((*qn).next_head_exact as *mut libc::c_void).is_null() {
            if (*reg).options &
                   ((1 as libc::c_uint) << 1 as libc::c_int) <<
                       1 as libc::c_int != 0 {
                r =
                    add_opcode(reg,
                               OP_ANYCHAR_ML_STAR_PEEK_NEXT as libc::c_int)
            } else {
                r = add_opcode(reg, OP_ANYCHAR_STAR_PEEK_NEXT as libc::c_int)
            }
            if r != 0 { return r }
            return add_bytes(reg, (*(*qn).next_head_exact).u.str_0.s,
                             1 as libc::c_int)
        } else if (*reg).options &
                      ((1 as libc::c_uint) << 1 as libc::c_int) <<
                          1 as libc::c_int != 0 {
            return add_opcode(reg, OP_ANYCHAR_ML_STAR as libc::c_int)
        } else { return add_opcode(reg, OP_ANYCHAR_STAR as libc::c_int) }
    }
    if empty_info != 0 as libc::c_int {
        mod_tlen =
            (tlen as
                 libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add((1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)))
                as libc::c_int
    } else { mod_tlen = tlen }
    if infinite != 0 &&
           ((*qn).lower <= 1 as libc::c_int ||
                tlen * (*qn).lower <= 50 as libc::c_int) {
        if (*qn).lower == 1 as libc::c_int && tlen > 50 as libc::c_int {
            if (*qn).greedy != 0 {
                if !((*qn).head_exact as *mut libc::c_void).is_null() {
                    r =
                        add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                            (1 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                as libc::c_int)
                } else if !((*qn).next_head_exact as
                                *mut libc::c_void).is_null() {
                    r =
                        add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                            (1 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                as libc::c_int)
                } else {
                    r =
                        add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                            (1 as libc::c_int as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as libc::c_int)
                }
            } else {
                r =
                    add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                        (1 as libc::c_int as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                             as
                                                                             libc::c_ulong)
                                            as libc::c_int)
            }
            if r != 0 { return r }
        } else {
            r = compile_tree_n_times((*qn).target, (*qn).lower, reg);
            if r != 0 { return r }
        }
        if (*qn).greedy != 0 {
            if !((*qn).head_exact as *mut libc::c_void).is_null() {
                r =
                    add_opcode_rel_addr(reg,
                                        OP_PUSH_OR_JUMP_EXACT1 as libc::c_int,
                                        (mod_tlen as
                                             libc::c_ulong).wrapping_add((1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                              as
                                                                                                              libc::c_ulong))
                                            as libc::c_int);
                if r != 0 { return r }
                add_bytes(reg, (*(*qn).head_exact).u.str_0.s,
                          1 as libc::c_int);
                r = compile_tree_empty_check((*qn).target, reg, empty_info);
                if r != 0 { return r }
                r =
                    add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                        -(mod_tlen +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_add(1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                  as libc::c_int))
            } else if !((*qn).next_head_exact as *mut libc::c_void).is_null()
             {
                r =
                    add_opcode_rel_addr(reg,
                                        OP_PUSH_IF_PEEK_NEXT as libc::c_int,
                                        (mod_tlen as
                                             libc::c_ulong).wrapping_add((1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                              as
                                                                                                              libc::c_ulong))
                                            as libc::c_int);
                if r != 0 { return r }
                add_bytes(reg, (*(*qn).next_head_exact).u.str_0.s,
                          1 as libc::c_int);
                r = compile_tree_empty_check((*qn).target, reg, empty_info);
                if r != 0 { return r }
                r =
                    add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                        -(mod_tlen +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_add(1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                  as libc::c_int))
            } else {
                r =
                    add_opcode_rel_addr(reg, OP_PUSH as libc::c_int,
                                        (mod_tlen as
                                             libc::c_ulong).wrapping_add((1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                              as
                                                                                                              libc::c_ulong))
                                            as libc::c_int);
                if r != 0 { return r }
                r = compile_tree_empty_check((*qn).target, reg, empty_info);
                if r != 0 { return r }
                r =
                    add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                        -(mod_tlen +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int))
            }
        } else {
            r = add_opcode_rel_addr(reg, OP_JUMP as libc::c_int, mod_tlen);
            if r != 0 { return r }
            r = compile_tree_empty_check((*qn).target, reg, empty_info);
            if r != 0 { return r }
            r =
                add_opcode_rel_addr(reg, OP_PUSH as libc::c_int,
                                    -(mod_tlen +
                                          (1 as libc::c_int as
                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                               as
                                                                               libc::c_ulong)
                                              as libc::c_int))
        }
    } else if (*qn).upper == 0 as libc::c_int &&
                  (*qn).is_refered != 0 as libc::c_int {
        /* /(?<n>..){0}/ */
        r = add_opcode_rel_addr(reg, OP_JUMP as libc::c_int, tlen);
        if r != 0 { return r }
        r = compile_tree((*qn).target, reg)
    } else if infinite == 0 && (*qn).greedy != 0 &&
                  ((*qn).upper == 1 as libc::c_int ||
                       (tlen as
                            libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                             as
                                                                                             libc::c_ulong)).wrapping_mul((*qn).upper
                                                                                                                              as
                                                                                                                              libc::c_ulong)
                           <= 50 as libc::c_int as libc::c_ulong) {
        let mut n: libc::c_int = (*qn).upper - (*qn).lower;
        r = compile_tree_n_times((*qn).target, (*qn).lower, reg);
        if r != 0 { return r }
        i = 0 as libc::c_int;
        while i < n {
            r =
                add_opcode_rel_addr(reg, OP_PUSH as libc::c_int,
                                    (((n - i) * tlen) as
                                         libc::c_ulong).wrapping_add(((n - i -
                                                                           1
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_ulong).wrapping_mul((1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                           as
                                                                                                                                           libc::c_ulong)))
                                        as libc::c_int);
            if r != 0 { return r }
            r = compile_tree((*qn).target, reg);
            if r != 0 { return r }
            i += 1
        }
    } else if (*qn).greedy == 0 && (*qn).upper == 1 as libc::c_int &&
                  (*qn).lower == 0 as libc::c_int {
        /* '??' */
        r =
            add_opcode_rel_addr(reg, OP_PUSH as libc::c_int,
                                (1 as libc::c_int as
                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                     as
                                                                     libc::c_ulong)
                                    as libc::c_int);
        if r != 0 { return r }
        r = add_opcode_rel_addr(reg, OP_JUMP as libc::c_int, tlen);
        if r != 0 { return r }
        r = compile_tree((*qn).target, reg)
    } else { r = compile_range_repeat_node(qn, mod_tlen, empty_info, reg) }
    return r;
}
/* USE_COMBINATION_EXPLOSION_CHECK */
unsafe extern "C" fn compile_length_option_node(mut node: *mut EncloseNode,
                                                mut reg: *mut regex_t)
 -> libc::c_int {
    let mut tlen: libc::c_int = 0; /* goal position */
    let mut prev: OnigOptionType = (*reg).options;
    (*reg).options = (*node).option;
    tlen = compile_length_tree((*node).target, reg);
    (*reg).options = prev;
    if tlen < 0 as libc::c_int { return tlen }
    return tlen;
}
unsafe extern "C" fn compile_option_node(mut node: *mut EncloseNode,
                                         mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut prev: OnigOptionType = (*reg).options;
    (*reg).options = (*node).option;
    r = compile_tree((*node).target, reg);
    (*reg).options = prev;
    return r;
}
unsafe extern "C" fn compile_length_enclose_node(mut node: *mut EncloseNode,
                                                 mut reg: *mut regex_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tlen: libc::c_int = 0;
    if (*node).type_0 == (1 as libc::c_int) << 1 as libc::c_int {
        return compile_length_option_node(node, reg)
    }
    if !(*node).target.is_null() {
        tlen = compile_length_tree((*node).target, reg);
        if tlen < 0 as libc::c_int { return tlen }
    } else { tlen = 0 as libc::c_int }
    match (*node).type_0 {
        1 => {
            if (*node).state & (1 as libc::c_int) << 8 as libc::c_int !=
                   0 as libc::c_int {
                len =
                    (1 as libc::c_int as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                         as
                                                         libc::c_ulong).wrapping_add(tlen
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add((1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<AbsAddrType>()
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong)).wrapping_add((1
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_ulong)).wrapping_add(1
                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                             libc::c_ulong)
                        as libc::c_int;
                if if (*node).regnum <
                          (::std::mem::size_of::<BitStatusType>() as
                               libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_int {
                       ((*reg).bt_mem_end) &
                           ((1 as libc::c_int) << (*node).regnum) as
                               libc::c_uint
                   } else {
                       ((*reg).bt_mem_end) & 1 as libc::c_int as libc::c_uint
                   } != 0 {
                    len =
                        (len as
                             libc::c_ulong).wrapping_add(if (*node).state &
                                                                (1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    7 as
                                                                        libc::c_int
                                                                !=
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         } else {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         }) as libc::c_int as
                            libc::c_int
                } else {
                    len =
                        (len as
                             libc::c_ulong).wrapping_add(if (*node).state &
                                                                (1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    7 as
                                                                        libc::c_int
                                                                !=
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         } else {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         }) as libc::c_int as
                            libc::c_int
                }
            } else if (*node).state & (1 as libc::c_int) << 7 as libc::c_int
                          != 0 as libc::c_int {
                len =
                    (1 as libc::c_int as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                         as libc::c_ulong) as
                        libc::c_int;
                len =
                    (len as
                         libc::c_ulong).wrapping_add((tlen as
                                                          libc::c_ulong).wrapping_add((if (if (*node).regnum
                                                                                                  <
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
                                                                                                   ((1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                        <<
                                                                                                        (*node).regnum)
                                                                                                       as
                                                                                                       libc::c_uint
                                                                                           } else {
                                                                                               ((*reg).bt_mem_end)
                                                                                                   &
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint
                                                                                           })
                                                                                              !=
                                                                                              0
                                                                                          {
                                                                                           (1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                as
                                                                                                                                libc::c_ulong)
                                                                                       } else {
                                                                                           (1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                as
                                                                                                                                libc::c_ulong)
                                                                                       })))
                        as libc::c_int as libc::c_int
            } else {
                if if (*node).regnum <
                          (::std::mem::size_of::<BitStatusType>() as
                               libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_int {
                       ((*reg).bt_mem_start) &
                           ((1 as libc::c_int) << (*node).regnum) as
                               libc::c_uint
                   } else {
                       ((*reg).bt_mem_start) &
                           1 as libc::c_int as libc::c_uint
                   } != 0 {
                    len =
                        (1 as libc::c_int as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                             as libc::c_ulong)
                            as libc::c_int
                } else {
                    len =
                        (1 as libc::c_int as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                             as libc::c_ulong)
                            as libc::c_int
                }
                len =
                    (len as
                         libc::c_ulong).wrapping_add((tlen as
                                                          libc::c_ulong).wrapping_add((if (if (*node).regnum
                                                                                                  <
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
                                                                                                   ((1
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                        <<
                                                                                                        (*node).regnum)
                                                                                                       as
                                                                                                       libc::c_uint
                                                                                           } else {
                                                                                               ((*reg).bt_mem_end)
                                                                                                   &
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint
                                                                                           })
                                                                                              !=
                                                                                              0
                                                                                          {
                                                                                           (1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                as
                                                                                                                                libc::c_ulong)
                                                                                       } else {
                                                                                           (1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                                                as
                                                                                                                                libc::c_ulong)
                                                                                       })))
                        as libc::c_int as libc::c_int
            }
        }
        4 => {
            if (*node).state & (1 as libc::c_int) << 6 as libc::c_int !=
                   0 as libc::c_int {
                let mut qn: *mut QtfrNode = &mut (*(*node).target).u.qtfr;
                tlen = compile_length_tree((*qn).target, reg);
                if tlen < 0 as libc::c_int { return tlen }
                len =
                    ((tlen * (*qn).lower) as
                         libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                          as
                                                                                          libc::c_ulong)).wrapping_add(tlen
                                                                                                                           as
                                                                                                                           libc::c_ulong).wrapping_add(1
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           libc::c_ulong).wrapping_add((1
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_ulong))
                        as libc::c_int
            } else { len = 1 as libc::c_int + tlen + 1 as libc::c_int }
        }
        _ => { return -(6 as libc::c_int) }
    }
    return len;
}
unsafe extern "C" fn compile_enclose_node(mut node: *mut EncloseNode,
                                          mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if (*node).type_0 == (1 as libc::c_int) << 1 as libc::c_int {
        return compile_option_node(node, reg)
    }
    match (*node).type_0 {
        1 => {
            if (*node).state & (1 as libc::c_int) << 8 as libc::c_int !=
                   0 as libc::c_int {
                r = add_opcode(reg, OP_CALL as libc::c_int);
                if r != 0 { return r }
                (*node).call_addr =
                    ((*reg).used as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<AbsAddrType>()
                                                         as
                                                         libc::c_ulong).wrapping_add((1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                        as AbsAddrType;
                (*node).state |= (1 as libc::c_int) << 9 as libc::c_int;
                r = add_abs_addr(reg, (*node).call_addr);
                if r != 0 { return r }
                len = compile_length_tree((*node).target, reg);
                len =
                    (len as
                         libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_add(1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                        as libc::c_int as libc::c_int;
                if if (*node).regnum <
                          (::std::mem::size_of::<BitStatusType>() as
                               libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_int {
                       ((*reg).bt_mem_end) &
                           ((1 as libc::c_int) << (*node).regnum) as
                               libc::c_uint
                   } else {
                       ((*reg).bt_mem_end) & 1 as libc::c_int as libc::c_uint
                   } != 0 {
                    len =
                        (len as
                             libc::c_ulong).wrapping_add(if (*node).state &
                                                                (1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    7 as
                                                                        libc::c_int
                                                                !=
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         } else {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         }) as libc::c_int as
                            libc::c_int
                } else {
                    len =
                        (len as
                             libc::c_ulong).wrapping_add(if (*node).state &
                                                                (1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    7 as
                                                                        libc::c_int
                                                                !=
                                                                0 as
                                                                    libc::c_int
                                                            {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         } else {
                                                             (1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                         }) as libc::c_int as
                            libc::c_int
                }
                r = add_opcode_rel_addr(reg, OP_JUMP as libc::c_int, len);
                if r != 0 { return r }
            }
            if if (*node).regnum <
                      (::std::mem::size_of::<BitStatusType>() as
                           libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                           libc::c_ulong) as
                          libc::c_int {
                   ((*reg).bt_mem_start) &
                       ((1 as libc::c_int) << (*node).regnum) as libc::c_uint
               } else {
                   ((*reg).bt_mem_start) & 1 as libc::c_int as libc::c_uint
               } != 0 {
                r = add_opcode(reg, OP_MEMORY_START_PUSH as libc::c_int)
            } else { r = add_opcode(reg, OP_MEMORY_START as libc::c_int) }
            if r != 0 { return r }
            r = add_mem_num(reg, (*node).regnum);
            if r != 0 { return r }
            r = compile_tree((*node).target, reg);
            if r != 0 { return r }
            if (*node).state & (1 as libc::c_int) << 8 as libc::c_int !=
                   0 as libc::c_int {
                if if (*node).regnum <
                          (::std::mem::size_of::<BitStatusType>() as
                               libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_int {
                       ((*reg).bt_mem_end) &
                           ((1 as libc::c_int) << (*node).regnum) as
                               libc::c_uint
                   } else {
                       ((*reg).bt_mem_end) & 1 as libc::c_int as libc::c_uint
                   } != 0 {
                    r =
                        add_opcode(reg,
                                   if (*node).state &
                                          (1 as libc::c_int) <<
                                              7 as libc::c_int !=
                                          0 as libc::c_int {
                                       OP_MEMORY_END_PUSH_REC as libc::c_int
                                   } else {
                                       OP_MEMORY_END_PUSH as libc::c_int
                                   })
                } else {
                    r =
                        add_opcode(reg,
                                   if (*node).state &
                                          (1 as libc::c_int) <<
                                              7 as libc::c_int !=
                                          0 as libc::c_int {
                                       OP_MEMORY_END_REC as libc::c_int
                                   } else { OP_MEMORY_END as libc::c_int })
                }
                if r != 0 { return r }
                r = add_mem_num(reg, (*node).regnum);
                if r != 0 { return r }
                r = add_opcode(reg, OP_RETURN as libc::c_int)
            } else if (*node).state & (1 as libc::c_int) << 7 as libc::c_int
                          != 0 as libc::c_int {
                if if (*node).regnum <
                          (::std::mem::size_of::<BitStatusType>() as
                               libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_int {
                       ((*reg).bt_mem_end) &
                           ((1 as libc::c_int) << (*node).regnum) as
                               libc::c_uint
                   } else {
                       ((*reg).bt_mem_end) & 1 as libc::c_int as libc::c_uint
                   } != 0 {
                    r = add_opcode(reg, OP_MEMORY_END_PUSH_REC as libc::c_int)
                } else {
                    r = add_opcode(reg, OP_MEMORY_END_REC as libc::c_int)
                }
                if r != 0 { return r }
                r = add_mem_num(reg, (*node).regnum)
            } else {
                if if (*node).regnum <
                          (::std::mem::size_of::<BitStatusType>() as
                               libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_int {
                       ((*reg).bt_mem_end) &
                           ((1 as libc::c_int) << (*node).regnum) as
                               libc::c_uint
                   } else {
                       ((*reg).bt_mem_end) & 1 as libc::c_int as libc::c_uint
                   } != 0 {
                    r = add_opcode(reg, OP_MEMORY_END_PUSH as libc::c_int)
                } else { r = add_opcode(reg, OP_MEMORY_END as libc::c_int) }
                if r != 0 { return r }
                r = add_mem_num(reg, (*node).regnum)
            }
        }
        4 => {
            if (*node).state & (1 as libc::c_int) << 6 as libc::c_int !=
                   0 as libc::c_int {
                let mut qn: *mut QtfrNode = &mut (*(*node).target).u.qtfr;
                r = compile_tree_n_times((*qn).target, (*qn).lower, reg);
                if r != 0 { return r }
                len = compile_length_tree((*qn).target, reg);
                if len < 0 as libc::c_int { return len }
                r =
                    add_opcode_rel_addr(reg, OP_PUSH as libc::c_int,
                                        ((len + 1 as libc::c_int) as
                                             libc::c_ulong).wrapping_add((1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                              as
                                                                                                              libc::c_ulong))
                                            as libc::c_int);
                if r != 0 { return r }
                r = compile_tree((*qn).target, reg);
                if r != 0 { return r }
                r = add_opcode(reg, OP_POP as libc::c_int);
                if r != 0 { return r }
                r =
                    add_opcode_rel_addr(reg, OP_JUMP as libc::c_int,
                                        -((1 as libc::c_int as
                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                               as
                                                                               libc::c_ulong)
                                              as libc::c_int + len +
                                              1 as libc::c_int +
                                              (1 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                   as
                                                                                   libc::c_ulong)
                                                  as libc::c_int))
            } else {
                r = add_opcode(reg, OP_PUSH_STOP_BT as libc::c_int);
                if r != 0 { return r }
                r = compile_tree((*node).target, reg);
                if r != 0 { return r }
                r = add_opcode(reg, OP_POP_STOP_BT as libc::c_int)
            }
        }
        _ => { return -(6 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn compile_length_anchor_node(mut node: *mut AnchorNode,
                                                mut reg: *mut regex_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tlen: libc::c_int = 0 as libc::c_int;
    if !(*node).target.is_null() {
        tlen = compile_length_tree((*node).target, reg);
        if tlen < 0 as libc::c_int { return tlen }
    }
    match (*node).type_0 {
        1024 => { len = 1 as libc::c_int + tlen + 1 as libc::c_int }
        2048 => {
            len =
                (1 as libc::c_int as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                     as
                                                     libc::c_ulong).wrapping_add(tlen
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(1
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                    as libc::c_int
        }
        4096 => {
            len =
                (1 as libc::c_int as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                     as
                                                     libc::c_ulong).wrapping_add(tlen
                                                                                     as
                                                                                     libc::c_ulong)
                    as libc::c_int
        }
        8192 => {
            len =
                (1 as libc::c_int as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                     as
                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(tlen
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_add(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong)
                    as libc::c_int
        }
        _ => { len = 1 as libc::c_int }
    }
    return len;
}
unsafe extern "C" fn compile_anchor_node(mut node: *mut AnchorNode,
                                         mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    match (*node).type_0 {
        1 => { r = add_opcode(reg, OP_BEGIN_BUF as libc::c_int) }
        8 => { r = add_opcode(reg, OP_END_BUF as libc::c_int) }
        2 => { r = add_opcode(reg, OP_BEGIN_LINE as libc::c_int) }
        32 => { r = add_opcode(reg, OP_END_LINE as libc::c_int) }
        16 => { r = add_opcode(reg, OP_SEMI_END_BUF as libc::c_int) }
        4 => { r = add_opcode(reg, OP_BEGIN_POSITION as libc::c_int) }
        64 => { r = add_opcode(reg, OP_WORD_BOUND as libc::c_int) }
        128 => { r = add_opcode(reg, OP_NOT_WORD_BOUND as libc::c_int) }
        256 => { r = add_opcode(reg, OP_WORD_BEGIN as libc::c_int) }
        512 => { r = add_opcode(reg, OP_WORD_END as libc::c_int) }
        1024 => {
            r = add_opcode(reg, OP_PUSH_POS as libc::c_int);
            if r != 0 { return r }
            r = compile_tree((*node).target, reg);
            if r != 0 { return r }
            r = add_opcode(reg, OP_POP_POS as libc::c_int)
        }
        2048 => {
            len = compile_length_tree((*node).target, reg);
            if len < 0 as libc::c_int { return len }
            r =
                add_opcode_rel_addr(reg, OP_PUSH_POS_NOT as libc::c_int,
                                    len + 1 as libc::c_int);
            if r != 0 { return r }
            r = compile_tree((*node).target, reg);
            if r != 0 { return r }
            r = add_opcode(reg, OP_FAIL_POS as libc::c_int)
        }
        4096 => {
            let mut n: libc::c_int = 0;
            r = add_opcode(reg, OP_LOOK_BEHIND as libc::c_int);
            if r != 0 { return r }
            if (*node).char_len < 0 as libc::c_int {
                r = get_char_length_tree((*node).target, reg, &mut n);
                if r != 0 { return -(122 as libc::c_int) }
            } else { n = (*node).char_len }
            r = add_length(reg, n);
            if r != 0 { return r }
            r = compile_tree((*node).target, reg)
        }
        8192 => {
            let mut n_0: libc::c_int = 0;
            len = compile_length_tree((*node).target, reg);
            r =
                add_opcode_rel_addr(reg,
                                    OP_PUSH_LOOK_BEHIND_NOT as libc::c_int,
                                    len + 1 as libc::c_int);
            if r != 0 { return r }
            if (*node).char_len < 0 as libc::c_int {
                r = get_char_length_tree((*node).target, reg, &mut n_0);
                if r != 0 { return -(122 as libc::c_int) }
            } else { n_0 = (*node).char_len }
            r = add_length(reg, n_0);
            if r != 0 { return r }
            r = compile_tree((*node).target, reg);
            if r != 0 { return r }
            r = add_opcode(reg, OP_FAIL_LOOK_BEHIND_NOT as libc::c_int)
        }
        _ => { return -(6 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn compile_length_tree(mut node: *mut Node,
                                         mut reg: *mut regex_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    type_0 = (*node).u.base.type_0;
    match type_0 {
        8 => {
            len = 0 as libc::c_int;
            loop  {
                r = compile_length_tree((*node).u.cons.car, reg);
                if r < 0 as libc::c_int { return r }
                len += r;
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
            r = len
        }
        9 => {
            let mut n: libc::c_int = 0;
            r = 0 as libc::c_int;
            n = r;
            loop  {
                r += compile_length_tree((*node).u.cons.car, reg);
                n += 1;
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
            r =
                (r as
                     libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                      libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_add((1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                       as
                                                                                                                                                       libc::c_ulong)).wrapping_mul((n
                                                                                                                                                                                         -
                                                                                                                                                                                         1
                                                                                                                                                                                             as
                                                                                                                                                                                             libc::c_int)
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_ulong))
                    as libc::c_int as libc::c_int
        }
        0 => {
            if (*node).u.str_0.flag &
                   ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                r = compile_length_string_raw_node(&mut (*node).u.str_0, reg)
            } else { r = compile_length_string_node(node, reg) }
        }
        1 => { r = compile_length_cclass_node(&mut (*node).u.cclass, reg) }
        2 | 3 => { r = 1 as libc::c_int }
        4 => {
            let mut br: *mut BRefNode = &mut (*node).u.bref;
            if (*br).state & (1 as libc::c_int) << 13 as libc::c_int !=
                   0 as libc::c_int {
                r =
                    (1 as libc::c_int as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<OnigOptionType>()
                                                         as
                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                                                                                         as
                                                                                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<MemNumType>()
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong).wrapping_mul((*br).back_num
                                                                                                                                                                                          as
                                                                                                                                                                                          libc::c_ulong))
                        as libc::c_int
            } else if (*br).back_num == 1 as libc::c_int {
                r =
                    if (*reg).options & 1 as libc::c_uint == 0 &&
                           (*br).back_static[0 as libc::c_int as usize] <=
                               2 as libc::c_int {
                        1 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<MemNumType>()
                                                             as libc::c_ulong)
                    } as libc::c_int
            } else {
                r =
                    (1 as libc::c_int as
                         libc::c_ulong).wrapping_add(::std::mem::size_of::<LengthType>()
                                                         as
                                                         libc::c_ulong).wrapping_add((::std::mem::size_of::<MemNumType>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*br).back_num
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                        as libc::c_int
            }
        }
        10 => {
            r =
                (1 as libc::c_int as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<AbsAddrType>()
                                                     as libc::c_ulong) as
                    libc::c_int
        }
        5 => { r = compile_length_quantifier_node(&mut (*node).u.qtfr, reg) }
        6 => { r = compile_length_enclose_node(&mut (*node).u.enclose, reg) }
        7 => { r = compile_length_anchor_node(&mut (*node).u.anchor, reg) }
        _ => { return -(6 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn compile_tree(mut node: *mut Node, mut reg: *mut regex_t)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    match type_0 {
        8 => {
            loop  {
                r = compile_tree((*node).u.cons.car, reg);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            let mut x: *mut Node = node;
            len = 0 as libc::c_int;
            loop  {
                len += compile_length_tree((*x).u.cons.car, reg);
                if !(*x).u.cons.cdr.is_null() {
                    len =
                        (len as
                             libc::c_ulong).wrapping_add((1 as libc::c_int as
                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                              as
                                                                                              libc::c_ulong).wrapping_add((1
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                                                               as
                                                                                                                                                               libc::c_ulong)))
                            as libc::c_int as libc::c_int
                }
                x = (*x).u.cons.cdr;
                if (x as *mut libc::c_void).is_null() { break ; }
            }
            pos =
                (*reg).used.wrapping_add(len as libc::c_uint) as libc::c_int;
            loop  {
                len = compile_length_tree((*node).u.cons.car, reg);
                if !((*node).u.cons.cdr as *mut libc::c_void).is_null() {
                    r =
                        add_opcode_rel_addr(reg, OP_PUSH as libc::c_int,
                                            (len as
                                                 libc::c_ulong).wrapping_add((1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                  as
                                                                                                                  libc::c_ulong))
                                                as libc::c_int);
                    if r != 0 { break ; }
                }
                r = compile_tree((*node).u.cons.car, reg);
                if r != 0 { break ; }
                if !((*node).u.cons.cdr as *mut libc::c_void).is_null() {
                    len =
                        (pos as
                             libc::c_ulong).wrapping_sub(((*reg).used as
                                                              libc::c_ulong).wrapping_add((1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<RelAddrType>()
                                                                                                                               as
                                                                                                                               libc::c_ulong)))
                            as libc::c_int;
                    r = add_opcode_rel_addr(reg, OP_JUMP as libc::c_int, len);
                    if r != 0 { break ; }
                }
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
        }
        0 => {
            if (*node).u.str_0.flag &
                   ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                r = compile_string_raw_node(&mut (*node).u.str_0, reg)
            } else { r = compile_string_node(node, reg) }
        }
        1 => { r = compile_cclass_node(&mut (*node).u.cclass, reg) }
        2 => {
            let mut op: libc::c_int = 0;
            match (*node).u.ctype.ctype {
                12 => {
                    if (*node).u.ctype.not != 0 as libc::c_int {
                        op = OP_NOT_WORD as libc::c_int
                    } else { op = OP_WORD as libc::c_int }
                }
                _ => { return -(6 as libc::c_int) }
            }
            r = add_opcode(reg, op)
        }
        3 => {
            if (*reg).options &
                   ((1 as libc::c_uint) << 1 as libc::c_int) <<
                       1 as libc::c_int != 0 {
                r = add_opcode(reg, OP_ANYCHAR_ML as libc::c_int)
            } else { r = add_opcode(reg, OP_ANYCHAR as libc::c_int) }
        }
        4 => {
            let mut br: *mut BRefNode = &mut (*node).u.bref;
            let mut i: libc::c_int = 0;
            let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut current_block_75: u64;
            if (*br).state & (1 as libc::c_int) << 13 as libc::c_int !=
                   0 as libc::c_int {
                r = add_opcode(reg, OP_BACKREF_WITH_LEVEL as libc::c_int);
                if r != 0 { return r }
                r = add_option(reg, (*reg).options & 1 as libc::c_uint);
                if r != 0 { return r }
                r = add_length(reg, (*br).nest_level);
                if r != 0 { return r }
                current_block_75 = 10216368444052436211;
            } else if (*br).back_num == 1 as libc::c_int {
                n = (*br).back_static[0 as libc::c_int as usize];
                if (*reg).options & 1 as libc::c_uint != 0 {
                    r = add_opcode(reg, OP_BACKREFN_IC as libc::c_int);
                    if r != 0 { return r }
                    r = add_mem_num(reg, n)
                } else {
                    match n {
                        1 => {
                            r = add_opcode(reg, OP_BACKREF1 as libc::c_int)
                        }
                        2 => {
                            r = add_opcode(reg, OP_BACKREF2 as libc::c_int)
                        }
                        _ => {
                            r = add_opcode(reg, OP_BACKREFN as libc::c_int);
                            if r != 0 { return r }
                            r = add_mem_num(reg, n)
                        }
                    }
                }
                current_block_75 = 7072655752890836508;
            } else {
                i = 0;
                p = 0 as *mut libc::c_int;
                if (*reg).options & 1 as libc::c_uint != 0 {
                    r = add_opcode(reg, OP_BACKREF_MULTI_IC as libc::c_int)
                } else {
                    r = add_opcode(reg, OP_BACKREF_MULTI as libc::c_int)
                }
                if r != 0 { return r }
                current_block_75 = 10216368444052436211;
            }
            match current_block_75 {
                10216368444052436211 => {
                    r = add_length(reg, (*br).back_num);
                    if r != 0 { return r }
                    p =
                        if !((*br).back_dynamic as
                                 *mut libc::c_void).is_null() {
                            (*br).back_dynamic
                        } else { (*br).back_static.as_mut_ptr() };
                    i = (*br).back_num - 1 as libc::c_int;
                    while i >= 0 as libc::c_int {
                        r = add_mem_num(reg, *p.offset(i as isize));
                        if r != 0 { return r }
                        i -= 1
                    }
                }
                _ => { }
            }
        }
        10 => { r = compile_call(&mut (*node).u.call, reg) }
        5 => { r = compile_quantifier_node(&mut (*node).u.qtfr, reg) }
        6 => { r = compile_enclose_node(&mut (*node).u.enclose, reg) }
        7 => { r = compile_anchor_node(&mut (*node).u.anchor, reg) }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn noname_disable_map(mut plink: *mut *mut Node,
                                        mut map: *mut GroupNumRemap,
                                        mut counter: *mut libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut node: *mut Node = *plink;
    match (*node).u.base.type_0 {
        8 | 9 => {
            loop  {
                r = noname_disable_map(&mut (*node).u.cons.car, map, counter);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        5 => {
            let mut ptarget: *mut *mut Node = &mut (*node).u.qtfr.target;
            let mut old: *mut Node = *ptarget;
            r = noname_disable_map(ptarget, map, counter);
            if *ptarget != old &&
                   (**ptarget).u.base.type_0 == 5 as libc::c_int {
                onig_reduce_nested_quantifier(node, *ptarget);
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            if (*en).type_0 == (1 as libc::c_int) << 0 as libc::c_int {
                if (*en).state & (1 as libc::c_int) << 10 as libc::c_int !=
                       0 as libc::c_int {
                    *counter += 1;
                    (*map.offset((*en).regnum as isize)).new_val = *counter;
                    (*en).regnum = *counter;
                    r = noname_disable_map(&mut (*en).target, map, counter)
                } else {
                    *plink = (*en).target;
                    (*en).target = 0 as *mut Node;
                    onig_node_free(node);
                    r = noname_disable_map(plink, map, counter)
                }
            } else { r = noname_disable_map(&mut (*en).target, map, counter) }
        }
        7 => {
            if !(*node).u.anchor.target.is_null() {
                r =
                    noname_disable_map(&mut (*node).u.anchor.target, map,
                                       counter)
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn renumber_node_backref(mut node: *mut Node,
                                           mut map: *mut GroupNumRemap)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut old_num: libc::c_int = 0;
    let mut backs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bn: *mut BRefNode = &mut (*node).u.bref;
    if !((*bn).state & (1 as libc::c_int) << 11 as libc::c_int !=
             0 as libc::c_int) {
        return -(209 as libc::c_int)
    }
    old_num = (*bn).back_num;
    if ((*bn).back_dynamic as *mut libc::c_void).is_null() {
        backs = (*bn).back_static.as_mut_ptr()
    } else { backs = (*bn).back_dynamic }
    i = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    while i < old_num {
        n = (*map.offset(*backs.offset(i as isize) as isize)).new_val;
        if n > 0 as libc::c_int { *backs.offset(pos as isize) = n; pos += 1 }
        i += 1
    }
    (*bn).back_num = pos;
    return 0 as libc::c_int;
}
unsafe extern "C" fn renumber_by_map(mut node: *mut Node,
                                     mut map: *mut GroupNumRemap)
 -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    match (*node).u.base.type_0 {
        8 | 9 => {
            loop  {
                r = renumber_by_map((*node).u.cons.car, map);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        5 => { r = renumber_by_map((*node).u.qtfr.target, map) }
        6 => { r = renumber_by_map((*node).u.enclose.target, map) }
        4 => { r = renumber_node_backref(node, map) }
        7 => {
            if !(*node).u.anchor.target.is_null() {
                r = renumber_by_map((*node).u.anchor.target, map)
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn numbered_ref_check(mut node: *mut Node) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    match (*node).u.base.type_0 {
        8 | 9 => {
            loop  {
                r = numbered_ref_check((*node).u.cons.car);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        5 => { r = numbered_ref_check((*node).u.qtfr.target) }
        6 => { r = numbered_ref_check((*node).u.enclose.target) }
        4 => {
            if !((*node).u.bref.state &
                     (1 as libc::c_int) << 11 as libc::c_int !=
                     0 as libc::c_int) {
                return -(209 as libc::c_int)
            }
        }
        7 => {
            if !(*node).u.anchor.target.is_null() {
                r = numbered_ref_check((*node).u.anchor.target)
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn disable_noname_group_capture(mut root: *mut *mut Node,
                                                  mut reg: *mut regex_t,
                                                  mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut loc: BitStatusType = 0;
    let mut map: *mut GroupNumRemap = 0 as *mut GroupNumRemap;
    let mut fresh1 =
        ::std::vec::from_elem(0,
                              (::std::mem::size_of::<GroupNumRemap>() as
                                   libc::c_ulong).wrapping_mul(((*env).num_mem
                                                                    +
                                                                    1 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_ulong)
                                  as usize);
    map = fresh1.as_mut_ptr() as *mut GroupNumRemap;
    if (map as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    i = 1 as libc::c_int;
    while i <= (*env).num_mem {
        (*map.offset(i as isize)).new_val = 0 as libc::c_int;
        i += 1
    }
    counter = 0 as libc::c_int;
    r = noname_disable_map(root, map, &mut counter);
    if r != 0 as libc::c_int { return r }
    r = renumber_by_map(*root, map);
    if r != 0 as libc::c_int { return r }
    i = 1 as libc::c_int;
    pos = 1 as libc::c_int;
    while i <= (*env).num_mem {
        if (*map.offset(i as isize)).new_val > 0 as libc::c_int {
            let ref mut fresh2 =
                *if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null()
                    {
                     (*env).mem_nodes_dynamic
                 } else {
                     (*env).mem_nodes_static.as_mut_ptr()
                 }.offset(pos as isize);
            *fresh2 =
                *if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null()
                    {
                     (*env).mem_nodes_dynamic
                 } else {
                     (*env).mem_nodes_static.as_mut_ptr()
                 }.offset(i as isize);
            pos += 1
        }
        i += 1
    }
    loc = (*env).capture_history;
    (*env).capture_history = 0 as libc::c_int as BitStatusType;
    i = 1 as libc::c_int;
    while i <= 31 as libc::c_int {
        if if i <
                  (::std::mem::size_of::<BitStatusType>() as
                       libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                       libc::c_ulong) as
                      libc::c_int {
               (loc) & ((1 as libc::c_int) << i) as libc::c_uint
           } else { (loc) & 1 as libc::c_int as libc::c_uint } != 0 {
            if (*map.offset(i as isize)).new_val <
                   (::std::mem::size_of::<BitStatusType>() as
                        libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                        libc::c_ulong) as
                       libc::c_int {
                (*env).capture_history |=
                    ((1 as libc::c_int) << (*map.offset(i as isize)).new_val)
                        as libc::c_uint
            }
        }
        i += 1
    }
    (*env).num_mem = (*env).num_named;
    (*reg).num_mem = (*env).num_named;
    return onig_renumber_name_table(reg, map);
}
/* USE_NAMED_GROUP */
unsafe extern "C" fn unset_addr_list_fix(mut uslist: *mut UnsetAddrList,
                                         mut reg: *mut regex_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut en: *mut EncloseNode = 0 as *mut EncloseNode;
    let mut addr: AbsAddrType = 0;
    i = 0 as libc::c_int;
    while i < (*uslist).num {
        en = &mut (*(*(*uslist).us.offset(i as isize)).target).u.enclose;
        if !((*en).state & (1 as libc::c_int) << 9 as libc::c_int !=
                 0 as libc::c_int) {
            return -(11 as libc::c_int)
        }
        addr = (*en).call_addr;
        offset = (*(*uslist).us.offset(i as isize)).offset;
        let mut used: libc::c_int =
            (offset as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<AbsAddrType>()
                                                 as libc::c_ulong) as
                libc::c_int;
        if (*reg).alloc < used as libc::c_uint {
            loop  {
                (*reg).alloc =
                    (*reg).alloc.wrapping_mul(2 as libc::c_int as
                                                  libc::c_uint);
                if !((*reg).alloc < used as libc::c_uint) { break ; }
            }
            (*reg).p =
                realloc((*reg).p as *mut libc::c_void,
                        (*reg).alloc as libc::c_ulong) as *mut OnigUChar;
            if ((*reg).p as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
        }
        memcpy((*reg).p.offset(offset as isize) as *mut libc::c_void,
               &mut addr as *mut AbsAddrType as *const libc::c_void,
               ::std::mem::size_of::<AbsAddrType>() as libc::c_ulong);
        if (*reg).used < used as libc::c_uint {
            (*reg).used = used as libc::c_uint
        }
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn quantifiers_memory_node_info(mut node: *mut Node)
 -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    match (*node).u.base.type_0 {
        8 | 9 => {
            let mut v: libc::c_int = 0;
            loop  {
                v = quantifiers_memory_node_info((*node).u.cons.car);
                if v > r { r = v }
                if !(v >= 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        10 => {
            if (*node).u.call.state & (1 as libc::c_int) << 7 as libc::c_int
                   != 0 as libc::c_int {
                return 3 as libc::c_int
                /* tiny version */
            } else { r = quantifiers_memory_node_info((*node).u.call.target) }
        }
        5 => {
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            if (*qn).upper != 0 as libc::c_int {
                r = quantifiers_memory_node_info((*qn).target)
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en).type_0 {
                1 => { return 2 as libc::c_int }
                2 | 4 => { r = quantifiers_memory_node_info((*en).target) }
                _ => { }
            }
        }
        4 | 0 | 2 | 1 | 3 | 7 | _ => { }
    }
    return r;
}
/* fixed size pattern node only */
unsafe extern "C" fn get_char_length_tree1(mut node: *mut Node,
                                           mut reg: *mut regex_t,
                                           mut len: *mut libc::c_int,
                                           mut level: libc::c_int)
 -> libc::c_int {
    let mut tlen: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    level += 1;
    *len = 0 as libc::c_int;
    match (*node).u.base.type_0 {
        8 => {
            loop  {
                r =
                    get_char_length_tree1((*node).u.cons.car, reg, &mut tlen,
                                          level);
                if r == 0 as libc::c_int {
                    *len =
                        distance_add(*len as OnigLen, tlen as OnigLen) as
                            libc::c_int
                }
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            let mut tlen2: libc::c_int = 0;
            let mut varlen: libc::c_int = 0 as libc::c_int;
            r =
                get_char_length_tree1((*node).u.cons.car, reg, &mut tlen,
                                      level);
            while r == 0 as libc::c_int &&
                      {
                          node = (*node).u.cons.cdr;
                          !(node as *mut libc::c_void).is_null()
                      } {
                r =
                    get_char_length_tree1((*node).u.cons.car, reg, &mut tlen2,
                                          level);
                if r == 0 as libc::c_int {
                    if tlen != tlen2 { varlen = 1 as libc::c_int }
                }
            }
            if r == 0 as libc::c_int {
                if varlen != 0 as libc::c_int {
                    if level == 1 as libc::c_int {
                        r = -(2 as libc::c_int)
                    } else { r = -(1 as libc::c_int) }
                } else { *len = tlen }
            }
        }
        0 => {
            let mut sn: *mut StrNode = &mut (*node).u.str_0;
            let mut s: *mut OnigUChar = (*sn).s;
            while s < (*sn).end {
                s =
                    s.offset((*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(s)
                                 as isize);
                *len += 1
            }
        }
        5 => {
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            if (*qn).lower == (*qn).upper {
                r =
                    get_char_length_tree1((*qn).target, reg, &mut tlen,
                                          level);
                if r == 0 as libc::c_int {
                    *len =
                        distance_multiply(tlen as OnigLen, (*qn).lower) as
                            libc::c_int
                }
            } else { r = -(1 as libc::c_int) }
        }
        10 => {
            if !((*node).u.call.state & (1 as libc::c_int) << 7 as libc::c_int
                     != 0 as libc::c_int) {
                r =
                    get_char_length_tree1((*node).u.call.target, reg, len,
                                          level)
            } else { r = -(1 as libc::c_int) }
        }
        2 => { *len = 1 as libc::c_int }
        1 | 3 => { *len = 1 as libc::c_int }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en).type_0 {
                1 => {
                    if (*en).state & (1 as libc::c_int) << 2 as libc::c_int !=
                           0 as libc::c_int {
                        *len = (*en).char_len
                    } else {
                        r =
                            get_char_length_tree1((*en).target, reg, len,
                                                  level);
                        if r == 0 as libc::c_int {
                            (*en).char_len = *len;
                            (*node).u.enclose.state |=
                                (1 as libc::c_int) << 2 as libc::c_int
                        }
                    }
                }
                2 | 4 => {
                    r = get_char_length_tree1((*en).target, reg, len, level)
                }
                _ => { }
            }
        }
        7 => { }
        _ => { r = -(1 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn get_char_length_tree(mut node: *mut Node,
                                          mut reg: *mut regex_t,
                                          mut len: *mut libc::c_int)
 -> libc::c_int {
    return get_char_length_tree1(node, reg, len, 0 as libc::c_int);
}
/* x is not included y ==>  1 : 0 */
unsafe extern "C" fn is_not_included(mut x: *mut Node, mut y: *mut Node,
                                     mut reg: *mut regex_t) -> libc::c_int {
    let mut xs: *mut StrNode = 0 as *mut StrNode;
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut code: OnigCodePoint = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut ytype: libc::c_int = 0;
    loop  {
        ytype = (*y).u.base.type_0;
        match (*x).u.base.type_0 {
            2 => {
                match ytype {
                    2 => {
                        if (*y).u.ctype.ctype == (*x).u.ctype.ctype &&
                               (*y).u.ctype.not != (*x).u.ctype.not {
                            return 1 as libc::c_int
                        } else { return 0 as libc::c_int }
                    }
                    1 | 0 => { }
                    _ => { current_block = 3634396408142324656; break ; }
                }
            }
            1 => {
                let mut xc: *mut CClassNode = &mut (*x).u.cclass;
                match ytype {
                    2 => {
                        current_block = 9574586227985364372;
                        match current_block {
                            7628763681555618602 => {
                                let mut v: libc::c_int = 0;
                                let mut yc: *mut CClassNode =
                                    &mut (*y).u.cclass;
                                i = 0 as libc::c_int;
                                while i <
                                          (1 as libc::c_int) <<
                                              8 as libc::c_int {
                                    v =
                                        ((*xc).bs[(i as
                                                       libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                                                      as usize] &
                                             ((1 as libc::c_int) <<
                                                  (i as
                                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong)))
                                                 as libc::c_uint) as
                                            libc::c_int;
                                    if v != 0 as libc::c_int &&
                                           !((*xc).flags &
                                                 ((1 as libc::c_int) <<
                                                      0 as libc::c_int) as
                                                     libc::c_uint !=
                                                 0 as libc::c_int as
                                                     libc::c_uint) ||
                                           v == 0 as libc::c_int &&
                                               (*xc).flags &
                                                   ((1 as libc::c_int) <<
                                                        0 as libc::c_int) as
                                                       libc::c_uint !=
                                                   0 as libc::c_int as
                                                       libc::c_uint {
                                        v =
                                            ((*yc).bs[(i as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as usize] &
                                                 ((1 as libc::c_int) <<
                                                      (i as
                                                           libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong)))
                                                     as libc::c_uint) as
                                                libc::c_int;
                                        if v != 0 as libc::c_int &&
                                               !((*yc).flags &
                                                     ((1 as libc::c_int) <<
                                                          0 as libc::c_int) as
                                                         libc::c_uint !=
                                                     0 as libc::c_int as
                                                         libc::c_uint) ||
                                               v == 0 as libc::c_int &&
                                                   (*yc).flags &
                                                       ((1 as libc::c_int) <<
                                                            0 as libc::c_int)
                                                           as libc::c_uint !=
                                                       0 as libc::c_int as
                                                           libc::c_uint {
                                            return 0 as libc::c_int
                                        }
                                    }
                                    i += 1
                                }
                                if ((*xc).mbuf as *mut libc::c_void).is_null()
                                       &&
                                       !((*xc).flags &
                                             ((1 as libc::c_int) <<
                                                  0 as libc::c_int) as
                                                 libc::c_uint !=
                                             0 as libc::c_int as libc::c_uint)
                                       ||
                                       ((*yc).mbuf as
                                            *mut libc::c_void).is_null() &&
                                           !((*yc).flags &
                                                 ((1 as libc::c_int) <<
                                                      0 as libc::c_int) as
                                                     libc::c_uint !=
                                                 0 as libc::c_int as
                                                     libc::c_uint) {
                                    return 1 as libc::c_int
                                }
                                return 0 as libc::c_int
                            }
                            _ => {
                                match (*y).u.ctype.ctype {
                                    12 => {
                                        if (*y).u.ctype.not ==
                                               0 as libc::c_int {
                                            if ((*xc).mbuf as
                                                    *mut libc::c_void).is_null()
                                                   &&
                                                   !((*xc).flags &
                                                         ((1 as libc::c_int)
                                                              <<
                                                              0 as
                                                                  libc::c_int)
                                                             as libc::c_uint
                                                         !=
                                                         0 as libc::c_int as
                                                             libc::c_uint) {
                                                i = 0 as libc::c_int;
                                                while i <
                                                          (1 as libc::c_int)
                                                              <<
                                                              8 as libc::c_int
                                                      {
                                                    if (*xc).bs[(i as
                                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_ulong))
                                                                    as usize]
                                                           &
                                                           ((1 as libc::c_int)
                                                                <<
                                                                (i as
                                                                     libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_ulong)))
                                                               as libc::c_uint
                                                           != 0 {
                                                        if i <
                                                               128 as
                                                                   libc::c_int
                                                               &&
                                                               (*(*reg).enc).is_code_ctype.expect("non-null function pointer")(i
                                                                                                                                   as
                                                                                                                                   OnigCodePoint,
                                                                                                                               12
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   OnigCtype)
                                                                   != 0 {
                                                            return 0 as
                                                                       libc::c_int
                                                        }
                                                    }
                                                    i += 1
                                                }
                                                return 1 as libc::c_int
                                            }
                                            return 0 as libc::c_int
                                        } else {
                                            if !((*xc).mbuf as
                                                     *mut libc::c_void).is_null()
                                               {
                                                return 0 as libc::c_int
                                            }
                                            i = 0 as libc::c_int;
                                            while i <
                                                      (1 as libc::c_int) <<
                                                          8 as libc::c_int {
                                                if !(i < 128 as libc::c_int &&
                                                         (*(*reg).enc).is_code_ctype.expect("non-null function pointer")(i
                                                                                                                             as
                                                                                                                             OnigCodePoint,
                                                                                                                         12
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             OnigCtype)
                                                             != 0) {
                                                    if !((*xc).flags &
                                                             ((1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  0 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_uint
                                                             !=
                                                             0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                       {
                                                        if (*xc).bs[(i as
                                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong))
                                                                        as
                                                                        usize]
                                                               &
                                                               ((1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    (i as
                                                                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)))
                                                                   as
                                                                   libc::c_uint
                                                               != 0 {
                                                            return 0 as
                                                                       libc::c_int
                                                        }
                                                    } else if (*xc).bs[(i as
                                                                            libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong))
                                                                           as
                                                                           usize]
                                                                  &
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       (i as
                                                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong)))
                                                                      as
                                                                      libc::c_uint
                                                                  == 0 {
                                                        return 0 as
                                                                   libc::c_int
                                                    }
                                                }
                                                i += 1
                                            }
                                            return 1 as libc::c_int
                                        }
                                    }
                                    _ => { }
                                }
                                current_block = 3634396408142324656;
                                break ;
                            }
                        }
                    }
                    1 => {
                        current_block = 7628763681555618602;
                        match current_block {
                            7628763681555618602 => {
                                let mut v: libc::c_int = 0;
                                let mut yc: *mut CClassNode =
                                    &mut (*y).u.cclass;
                                i = 0 as libc::c_int;
                                while i <
                                          (1 as libc::c_int) <<
                                              8 as libc::c_int {
                                    v =
                                        ((*xc).bs[(i as
                                                       libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                                                      as usize] &
                                             ((1 as libc::c_int) <<
                                                  (i as
                                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong)))
                                                 as libc::c_uint) as
                                            libc::c_int;
                                    if v != 0 as libc::c_int &&
                                           !((*xc).flags &
                                                 ((1 as libc::c_int) <<
                                                      0 as libc::c_int) as
                                                     libc::c_uint !=
                                                 0 as libc::c_int as
                                                     libc::c_uint) ||
                                           v == 0 as libc::c_int &&
                                               (*xc).flags &
                                                   ((1 as libc::c_int) <<
                                                        0 as libc::c_int) as
                                                       libc::c_uint !=
                                                   0 as libc::c_int as
                                                       libc::c_uint {
                                        v =
                                            ((*yc).bs[(i as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as usize] &
                                                 ((1 as libc::c_int) <<
                                                      (i as
                                                           libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong)))
                                                     as libc::c_uint) as
                                                libc::c_int;
                                        if v != 0 as libc::c_int &&
                                               !((*yc).flags &
                                                     ((1 as libc::c_int) <<
                                                          0 as libc::c_int) as
                                                         libc::c_uint !=
                                                     0 as libc::c_int as
                                                         libc::c_uint) ||
                                               v == 0 as libc::c_int &&
                                                   (*yc).flags &
                                                       ((1 as libc::c_int) <<
                                                            0 as libc::c_int)
                                                           as libc::c_uint !=
                                                       0 as libc::c_int as
                                                           libc::c_uint {
                                            return 0 as libc::c_int
                                        }
                                    }
                                    i += 1
                                }
                                if ((*xc).mbuf as *mut libc::c_void).is_null()
                                       &&
                                       !((*xc).flags &
                                             ((1 as libc::c_int) <<
                                                  0 as libc::c_int) as
                                                 libc::c_uint !=
                                             0 as libc::c_int as libc::c_uint)
                                       ||
                                       ((*yc).mbuf as
                                            *mut libc::c_void).is_null() &&
                                           !((*yc).flags &
                                                 ((1 as libc::c_int) <<
                                                      0 as libc::c_int) as
                                                     libc::c_uint !=
                                                 0 as libc::c_int as
                                                     libc::c_uint) {
                                    return 1 as libc::c_int
                                }
                                return 0 as libc::c_int
                            }
                            _ => {
                                match (*y).u.ctype.ctype {
                                    12 => {
                                        if (*y).u.ctype.not ==
                                               0 as libc::c_int {
                                            if ((*xc).mbuf as
                                                    *mut libc::c_void).is_null()
                                                   &&
                                                   !((*xc).flags &
                                                         ((1 as libc::c_int)
                                                              <<
                                                              0 as
                                                                  libc::c_int)
                                                             as libc::c_uint
                                                         !=
                                                         0 as libc::c_int as
                                                             libc::c_uint) {
                                                i = 0 as libc::c_int;
                                                while i <
                                                          (1 as libc::c_int)
                                                              <<
                                                              8 as libc::c_int
                                                      {
                                                    if (*xc).bs[(i as
                                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_ulong))
                                                                    as usize]
                                                           &
                                                           ((1 as libc::c_int)
                                                                <<
                                                                (i as
                                                                     libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_ulong)))
                                                               as libc::c_uint
                                                           != 0 {
                                                        if i <
                                                               128 as
                                                                   libc::c_int
                                                               &&
                                                               (*(*reg).enc).is_code_ctype.expect("non-null function pointer")(i
                                                                                                                                   as
                                                                                                                                   OnigCodePoint,
                                                                                                                               12
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   OnigCtype)
                                                                   != 0 {
                                                            return 0 as
                                                                       libc::c_int
                                                        }
                                                    }
                                                    i += 1
                                                }
                                                return 1 as libc::c_int
                                            }
                                            return 0 as libc::c_int
                                        } else {
                                            if !((*xc).mbuf as
                                                     *mut libc::c_void).is_null()
                                               {
                                                return 0 as libc::c_int
                                            }
                                            i = 0 as libc::c_int;
                                            while i <
                                                      (1 as libc::c_int) <<
                                                          8 as libc::c_int {
                                                if !(i < 128 as libc::c_int &&
                                                         (*(*reg).enc).is_code_ctype.expect("non-null function pointer")(i
                                                                                                                             as
                                                                                                                             OnigCodePoint,
                                                                                                                         12
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             OnigCtype)
                                                             != 0) {
                                                    if !((*xc).flags &
                                                             ((1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  0 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_uint
                                                             !=
                                                             0 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                                                       {
                                                        if (*xc).bs[(i as
                                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong))
                                                                        as
                                                                        usize]
                                                               &
                                                               ((1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    (i as
                                                                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)))
                                                                   as
                                                                   libc::c_uint
                                                               != 0 {
                                                            return 0 as
                                                                       libc::c_int
                                                        }
                                                    } else if (*xc).bs[(i as
                                                                            libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong))
                                                                           as
                                                                           usize]
                                                                  &
                                                                  ((1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       (i as
                                                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_ulong)))
                                                                      as
                                                                      libc::c_uint
                                                                  == 0 {
                                                        return 0 as
                                                                   libc::c_int
                                                    }
                                                }
                                                i += 1
                                            }
                                            return 1 as libc::c_int
                                        }
                                    }
                                    _ => { }
                                }
                                current_block = 3634396408142324656;
                                break ;
                            }
                        }
                    }
                    0 => { }
                    _ => { current_block = 3634396408142324656; break ; }
                }
            }
            0 => {
                xs = &mut (*x).u.str_0;
                if (*x).u.str_0.end.wrapping_offset_from((*x).u.str_0.s) as
                       libc::c_long == 0 as libc::c_int as libc::c_long {
                    current_block = 3634396408142324656;
                    break ;
                } else { current_block = 5372832139739605200; break ; }
            }
            _ => { current_block = 3634396408142324656; break ; }
        }
        let mut tmp: *mut Node = 0 as *mut Node;
        tmp = x;
        x = y;
        y = tmp
    }
    match current_block {
        5372832139739605200 => {
            //c = *(xs->s);
            match ytype {
                2 => {
                    match (*y).u.ctype.ctype {
                        12 => {
                            if (*(*reg).enc).is_code_ctype.expect("non-null function pointer")((*(*reg).enc).mbc_to_code.expect("non-null function pointer")((*xs).s,
                                                                                                                                                             (*xs).end),
                                                                                               12
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   OnigCtype)
                                   != 0 {
                                return (*y).u.ctype.not
                            } else {
                                return ((*y).u.ctype.not == 0) as libc::c_int
                            }
                        }
                        _ => { }
                    }
                }
                1 => {
                    let mut cc: *mut CClassNode = &mut (*y).u.cclass;
                    code =
                        (*(*reg).enc).mbc_to_code.expect("non-null function pointer")((*xs).s,
                                                                                      (*xs).s.offset((*(*reg).enc).max_enc_len
                                                                                                         as
                                                                                                         isize));
                    return if onig_is_code_in_cc((*reg).enc, code, cc) !=
                                  0 as libc::c_int {
                               0 as libc::c_int
                           } else { 1 as libc::c_int }
                }
                0 => {
                    let mut q: *mut OnigUChar = 0 as *mut OnigUChar;
                    let mut ys: *mut StrNode = &mut (*y).u.str_0;
                    len =
                        (*x).u.str_0.end.wrapping_offset_from((*x).u.str_0.s)
                            as libc::c_long as libc::c_int;
                    if len as libc::c_long >
                           (*y).u.str_0.end.wrapping_offset_from((*y).u.str_0.s)
                               as libc::c_long {
                        len =
                            (*y).u.str_0.end.wrapping_offset_from((*y).u.str_0.s)
                                as libc::c_long as libc::c_int
                    }
                    if (*x).u.str_0.flag &
                           ((1 as libc::c_int) << 1 as libc::c_int) as
                               libc::c_uint !=
                           0 as libc::c_int as libc::c_uint ||
                           (*y).u.str_0.flag &
                               ((1 as libc::c_int) << 1 as libc::c_int) as
                                   libc::c_uint !=
                               0 as libc::c_int as libc::c_uint {
                        /* tiny version */
                        return 0 as libc::c_int
                    } else {
                        i = 0 as libc::c_int; // recursive
                        p = (*ys).s; /* recursion */
                        q = (*xs).s;
                        while i < len {
                            if *p as libc::c_int != *q as libc::c_int {
                                return 1 as libc::c_int
                            }
                            i += 1;
                            p = p.offset(1);
                            q = q.offset(1)
                        }
                    }
                }
                _ => { }
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_head_value_node(mut node: *mut Node,
                                         mut exact: libc::c_int,
                                         mut reg: *mut regex_t) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    match (*node).u.base.type_0 {
        2 | 1 => { if exact == 0 as libc::c_int { n = node } }
        8 => { n = get_head_value_node((*node).u.cons.car, exact, reg) }
        0 => {
            let mut sn: *mut StrNode = &mut (*node).u.str_0;
            if !((*sn).end <= (*sn).s) {
                if !(exact != 0 as libc::c_int &&
                         !((*node).u.str_0.flag &
                               ((1 as libc::c_int) << 0 as libc::c_int) as
                                   libc::c_uint !=
                               0 as libc::c_int as libc::c_uint) &&
                         (*reg).options & 1 as libc::c_uint != 0) {
                    n = node
                }
            }
        }
        5 => {
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            if (*qn).lower > 0 as libc::c_int {
                if !((*qn).head_exact as *mut libc::c_void).is_null() {
                    n = (*qn).head_exact
                } else { n = get_head_value_node((*qn).target, exact, reg) }
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en).type_0 {
                2 => {
                    let mut options: OnigOptionType = (*reg).options;
                    (*reg).options = (*node).u.enclose.option;
                    n =
                        get_head_value_node((*node).u.enclose.target, exact,
                                            reg);
                    (*reg).options = options
                }
                1 | 4 => { n = get_head_value_node((*en).target, exact, reg) }
                _ => { }
            }
        }
        7 => {
            if (*node).u.anchor.type_0 ==
                   (1 as libc::c_int) << 10 as libc::c_int {
                n = get_head_value_node((*node).u.anchor.target, exact, reg)
            }
        }
        4 | 9 | 3 | 10 | _ => { }
    }
    return n;
}
unsafe extern "C" fn check_type_tree(mut node: *mut Node,
                                     mut type_mask: libc::c_int,
                                     mut enclose_mask: libc::c_int,
                                     mut anchor_mask: libc::c_int)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    if (1 as libc::c_int) << type_0 & type_mask == 0 as libc::c_int {
        return 1 as libc::c_int
    }
    match type_0 {
        8 | 9 => {
            loop  {
                r =
                    check_type_tree((*node).u.cons.car, type_mask,
                                    enclose_mask, anchor_mask);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        5 => {
            r =
                check_type_tree((*node).u.qtfr.target, type_mask,
                                enclose_mask, anchor_mask)
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            if (*en).type_0 & enclose_mask == 0 as libc::c_int {
                return 1 as libc::c_int
            }
            r =
                check_type_tree((*en).target, type_mask, enclose_mask,
                                anchor_mask)
        }
        7 => {
            type_0 = (*node).u.anchor.type_0;
            if type_0 & anchor_mask == 0 as libc::c_int {
                return 1 as libc::c_int
            }
            if !(*node).u.anchor.target.is_null() {
                r =
                    check_type_tree((*node).u.anchor.target, type_mask,
                                    enclose_mask, anchor_mask)
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn get_min_len(mut node: *mut Node, mut min: *mut OnigLen,
                                 mut env: *mut ScanEnv) -> libc::c_int {
    let mut tmin: OnigLen = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    *min = 0 as libc::c_int as OnigLen;
    match (*node).u.base.type_0 {
        4 => {
            let mut i: libc::c_int = 0;
            let mut backs: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut nodes: *mut *mut Node =
                if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null()
                   {
                    (*env).mem_nodes_dynamic
                } else { (*env).mem_nodes_static.as_mut_ptr() };
            let mut br: *mut BRefNode = &mut (*node).u.bref;
            if !((*br).state & (1 as libc::c_int) << 7 as libc::c_int != 0) {
                backs =
                    if !((*br).back_dynamic as *mut libc::c_void).is_null() {
                        (*br).back_dynamic
                    } else { (*br).back_static.as_mut_ptr() };
                if *backs.offset(0 as libc::c_int as isize) > (*env).num_mem {
                    return -(208 as libc::c_int)
                }
                r =
                    get_min_len(*nodes.offset(*backs.offset(0 as libc::c_int
                                                                as isize) as
                                                  isize), min, env);
                if !(r != 0 as libc::c_int) {
                    i = 1 as libc::c_int;
                    while i < (*br).back_num {
                        if *backs.offset(i as isize) > (*env).num_mem {
                            return -(208 as libc::c_int)
                        }
                        r =
                            get_min_len(*nodes.offset(*backs.offset(i as
                                                                        isize)
                                                          as isize),
                                        &mut tmin, env);
                        if r != 0 as libc::c_int { break ; }
                        if *min > tmin { *min = tmin }
                        i += 1
                    }
                }
            }
        }
        10 => {
            if (*node).u.call.state & (1 as libc::c_int) << 7 as libc::c_int
                   != 0 as libc::c_int {
                let mut en: *mut EncloseNode =
                    &mut (*(*node).u.call.target).u.enclose;
                if (*en).state & (1 as libc::c_int) << 0 as libc::c_int !=
                       0 as libc::c_int {
                    *min = (*en).min_len
                }
            } else { r = get_min_len((*node).u.call.target, min, env) }
        }
        8 => {
            loop  {
                r = get_min_len((*node).u.cons.car, &mut tmin, env);
                if r == 0 as libc::c_int {
                    *min =
                        (*min as libc::c_uint).wrapping_add(tmin) as OnigLen
                            as OnigLen
                }
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            let mut x: *mut Node = 0 as *mut Node;
            let mut y: *mut Node = 0 as *mut Node;
            y = node;
            loop  {
                x = (*y).u.cons.car;
                r = get_min_len(x, &mut tmin, env);
                if r != 0 as libc::c_int { break ; }
                if y == node {
                    *min = tmin
                } else if *min > tmin { *min = tmin }
                if !(r == 0 as libc::c_int &&
                         {
                             y = (*y).u.cons.cdr;
                             !(y as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        0 => {
            let mut sn: *mut StrNode = &mut (*node).u.str_0;
            *min =
                (*sn).end.wrapping_offset_from((*sn).s) as libc::c_long as
                    OnigLen
        }
        2 => { *min = 1 as libc::c_int as OnigLen }
        1 | 3 => { *min = 1 as libc::c_int as OnigLen }
        5 => {
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            if (*qn).lower > 0 as libc::c_int {
                r = get_min_len((*qn).target, min, env);
                if r == 0 as libc::c_int {
                    *min = distance_multiply(*min, (*qn).lower)
                }
            }
        }
        6 => {
            let mut en_0: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en_0).type_0 {
                1 => {
                    if (*en_0).state & (1 as libc::c_int) << 0 as libc::c_int
                           != 0 as libc::c_int {
                        *min = (*en_0).min_len
                    } else if (*node).u.enclose.state &
                                  (1 as libc::c_int) << 3 as libc::c_int !=
                                  0 as libc::c_int {
                        *min = 0 as libc::c_int as OnigLen
                    } else {
                        (*node).u.enclose.state |=
                            (1 as libc::c_int) << 3 as libc::c_int;
                        r = get_min_len((*en_0).target, min, env);
                        (*node).u.enclose.state &=
                            !((1 as libc::c_int) << 3 as libc::c_int);
                        if r == 0 as libc::c_int {
                            (*en_0).min_len = *min;
                            (*node).u.enclose.state |=
                                (1 as libc::c_int) << 0 as libc::c_int
                        }
                    }
                }
                2 | 4 => { r = get_min_len((*en_0).target, min, env) }
                _ => { }
            }
        }
        7 | _ => { }
    }
    return r;
}
unsafe extern "C" fn get_max_len(mut node: *mut Node, mut max: *mut OnigLen,
                                 mut env: *mut ScanEnv) -> libc::c_int {
    let mut tmax: OnigLen = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    *max = 0 as libc::c_int as OnigLen;
    match (*node).u.base.type_0 {
        8 => {
            loop  {
                r = get_max_len((*node).u.cons.car, &mut tmax, env);
                if r == 0 as libc::c_int { *max = distance_add(*max, tmax) }
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            loop  {
                r = get_max_len((*node).u.cons.car, &mut tmax, env);
                if r == 0 as libc::c_int && *max < tmax { *max = tmax }
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        0 => {
            let mut sn: *mut StrNode = &mut (*node).u.str_0;
            *max =
                (*sn).end.wrapping_offset_from((*sn).s) as libc::c_long as
                    OnigLen
        }
        2 => { *max = (*(*env).enc).max_enc_len as OnigLen }
        1 | 3 => { *max = (*(*env).enc).max_enc_len as OnigLen }
        4 => {
            let mut i: libc::c_int = 0;
            let mut backs: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut nodes: *mut *mut Node =
                if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null()
                   {
                    (*env).mem_nodes_dynamic
                } else { (*env).mem_nodes_static.as_mut_ptr() };
            let mut br: *mut BRefNode = &mut (*node).u.bref;
            if (*br).state & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                *max = !(0 as libc::c_int as OnigLen)
            } else {
                backs =
                    if !((*br).back_dynamic as *mut libc::c_void).is_null() {
                        (*br).back_dynamic
                    } else { (*br).back_static.as_mut_ptr() };
                i = 0 as libc::c_int;
                while i < (*br).back_num {
                    if *backs.offset(i as isize) > (*env).num_mem {
                        return -(208 as libc::c_int)
                    }
                    r =
                        get_max_len(*nodes.offset(*backs.offset(i as isize) as
                                                      isize), &mut tmax, env);
                    if r != 0 as libc::c_int { break ; }
                    if *max < tmax { *max = tmax }
                    i += 1
                }
            }
        }
        10 => {
            if !((*node).u.call.state & (1 as libc::c_int) << 7 as libc::c_int
                     != 0 as libc::c_int) {
                r = get_max_len((*node).u.call.target, max, env)
            } else { *max = !(0 as libc::c_int as OnigLen) }
        }
        5 => {
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            if (*qn).upper != 0 as libc::c_int {
                r = get_max_len((*qn).target, max, env);
                if r == 0 as libc::c_int &&
                       *max != 0 as libc::c_int as libc::c_uint {
                    if !((*qn).upper == -(1 as libc::c_int)) {
                        *max = distance_multiply(*max, (*qn).upper)
                    } else { *max = !(0 as libc::c_int as OnigLen) }
                }
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en).type_0 {
                1 => {
                    if (*en).state & (1 as libc::c_int) << 1 as libc::c_int !=
                           0 as libc::c_int {
                        *max = (*en).max_len
                    } else if (*node).u.enclose.state &
                                  (1 as libc::c_int) << 3 as libc::c_int !=
                                  0 as libc::c_int {
                        *max = !(0 as libc::c_int as OnigLen)
                    } else {
                        (*node).u.enclose.state |=
                            (1 as libc::c_int) << 3 as libc::c_int;
                        r = get_max_len((*en).target, max, env);
                        (*node).u.enclose.state &=
                            !((1 as libc::c_int) << 3 as libc::c_int);
                        if r == 0 as libc::c_int {
                            (*en).max_len = *max;
                            (*node).u.enclose.state |=
                                (1 as libc::c_int) << 1 as libc::c_int
                        }
                    }
                }
                2 | 4 => { r = get_max_len((*en).target, max, env) }
                _ => { }
            }
        }
        7 | _ => { }
    }
    return r;
}
unsafe extern "C" fn subexp_inf_recursive_check(mut node: *mut Node,
                                                mut env: *mut ScanEnv,
                                                mut head: libc::c_int)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    match type_0 {
        8 => {
            let mut x: *mut Node = 0 as *mut Node;
            let mut min: OnigLen = 0;
            let mut ret: libc::c_int = 0;
            x = node;
            loop  {
                ret = subexp_inf_recursive_check((*x).u.cons.car, env, head);
                if ret < 0 as libc::c_int || ret == 2 as libc::c_int {
                    return ret
                }
                r |= ret;
                if head != 0 {
                    ret = get_min_len((*x).u.cons.car, &mut min, env);
                    if ret != 0 as libc::c_int { return ret }
                    if min != 0 as libc::c_int as libc::c_uint {
                        head = 0 as libc::c_int
                    }
                }
                x = (*x).u.cons.cdr;
                if (x as *mut libc::c_void).is_null() { break ; }
            }
        }
        9 => {
            let mut ret_0: libc::c_int = 0;
            r = 1 as libc::c_int;
            loop  {
                ret_0 =
                    subexp_inf_recursive_check((*node).u.cons.car, env, head);
                if ret_0 < 0 as libc::c_int || ret_0 == 2 as libc::c_int {
                    return ret_0
                }
                r &= ret_0;
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
        }
        5 => {
            r = subexp_inf_recursive_check((*node).u.qtfr.target, env, head);
            if r == 1 as libc::c_int {
                if (*node).u.qtfr.lower == 0 as libc::c_int {
                    r = 0 as libc::c_int
                }
            }
        }
        7 => {
            let mut an: *mut AnchorNode = &mut (*node).u.anchor;
            match (*an).type_0 {
                1024 | 2048 | 4096 | 8192 => {
                    r = subexp_inf_recursive_check((*an).target, env, head)
                }
                _ => { }
            }
        }
        10 => {
            r = subexp_inf_recursive_check((*node).u.call.target, env, head)
        }
        6 => {
            if (*node).u.enclose.state &
                   (1 as libc::c_int) << 4 as libc::c_int != 0 as libc::c_int
               {
                return 0 as libc::c_int
            } else {
                if (*node).u.enclose.state &
                       (1 as libc::c_int) << 3 as libc::c_int !=
                       0 as libc::c_int {
                    return if head == 0 as libc::c_int {
                               1 as libc::c_int
                           } else { 2 as libc::c_int }
                } else {
                    (*node).u.enclose.state |=
                        (1 as libc::c_int) << 4 as libc::c_int;
                    r =
                        subexp_inf_recursive_check((*node).u.enclose.target,
                                                   env, head);
                    (*node).u.enclose.state &=
                        !((1 as libc::c_int) << 4 as libc::c_int)
                }
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn subexp_inf_recursive_check_trav(mut node: *mut Node,
                                                     mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    match type_0 {
        8 | 9 => {
            loop  {
                r = subexp_inf_recursive_check_trav((*node).u.cons.car, env);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        5 => {
            r = subexp_inf_recursive_check_trav((*node).u.qtfr.target, env)
        }
        7 => {
            let mut an: *mut AnchorNode = &mut (*node).u.anchor;
            match (*an).type_0 {
                1024 | 2048 | 4096 | 8192 => {
                    r = subexp_inf_recursive_check_trav((*an).target, env)
                }
                _ => { }
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            if (*en).state & (1 as libc::c_int) << 7 as libc::c_int !=
                   0 as libc::c_int {
                (*node).u.enclose.state |=
                    (1 as libc::c_int) << 3 as libc::c_int;
                r =
                    subexp_inf_recursive_check((*en).target, env,
                                               1 as libc::c_int);
                if r > 0 as libc::c_int { return -(221 as libc::c_int) }
                (*node).u.enclose.state &=
                    !((1 as libc::c_int) << 3 as libc::c_int)
            }
            r = subexp_inf_recursive_check_trav((*en).target, env)
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn subexp_recursive_check(mut node: *mut Node)
 -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    match (*node).u.base.type_0 {
        8 | 9 => {
            loop  {
                r |= subexp_recursive_check((*node).u.cons.car);
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
        }
        5 => { r = subexp_recursive_check((*node).u.qtfr.target) }
        7 => {
            let mut an: *mut AnchorNode = &mut (*node).u.anchor;
            match (*an).type_0 {
                1024 | 2048 | 4096 | 8192 => {
                    r = subexp_recursive_check((*an).target)
                }
                _ => { }
            }
        }
        10 => {
            r = subexp_recursive_check((*node).u.call.target);
            if r != 0 as libc::c_int {
                (*node).u.call.state |= (1 as libc::c_int) << 7 as libc::c_int
            }
        }
        6 => {
            if (*node).u.enclose.state &
                   (1 as libc::c_int) << 4 as libc::c_int != 0 as libc::c_int
               {
                return 0 as libc::c_int
            } else {
                if (*node).u.enclose.state &
                       (1 as libc::c_int) << 3 as libc::c_int !=
                       0 as libc::c_int {
                    return 1 as libc::c_int
                } else {
                    (*node).u.enclose.state |=
                        (1 as libc::c_int) << 4 as libc::c_int;
                    r = subexp_recursive_check((*node).u.enclose.target);
                    (*node).u.enclose.state &=
                        !((1 as libc::c_int) << 4 as libc::c_int)
                }
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn subexp_recursive_check_trav(mut node: *mut Node,
                                                 mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    match type_0 {
        8 | 9 => {
            let mut ret: libc::c_int = 0;
            loop  {
                ret = subexp_recursive_check_trav((*node).u.cons.car, env);
                if ret == 1 as libc::c_int {
                    r = 1 as libc::c_int
                } else if ret < 0 as libc::c_int { return ret }
                node = (*node).u.cons.cdr;
                if (node as *mut libc::c_void).is_null() { break ; }
            }
        }
        5 => {
            r = subexp_recursive_check_trav((*node).u.qtfr.target, env);
            if (*node).u.qtfr.upper == 0 as libc::c_int {
                if r == 1 as libc::c_int {
                    (*node).u.qtfr.is_refered = 1 as libc::c_int
                }
            }
        }
        7 => {
            let mut an: *mut AnchorNode = &mut (*node).u.anchor;
            match (*an).type_0 {
                1024 | 2048 | 4096 | 8192 => {
                    r = subexp_recursive_check_trav((*an).target, env)
                }
                _ => { }
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            if !((*en).state & (1 as libc::c_int) << 7 as libc::c_int !=
                     0 as libc::c_int) {
                if (*en).state & (1 as libc::c_int) << 8 as libc::c_int !=
                       0 as libc::c_int {
                    (*node).u.enclose.state |=
                        (1 as libc::c_int) << 3 as libc::c_int;
                    r = subexp_recursive_check((*en).target);
                    if r != 0 as libc::c_int {
                        (*node).u.enclose.state |=
                            (1 as libc::c_int) << 7 as libc::c_int
                    }
                    (*node).u.enclose.state &=
                        !((1 as libc::c_int) << 3 as libc::c_int)
                }
            }
            r = subexp_recursive_check_trav((*en).target, env);
            if (*en).state & (1 as libc::c_int) << 8 as libc::c_int !=
                   0 as libc::c_int {
                r |= 1 as libc::c_int
            }
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn setup_subexp_call(mut node: *mut Node,
                                       mut env: *mut ScanEnv) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    match type_0 {
        8 => {
            loop  {
                r = setup_subexp_call((*node).u.cons.car, env);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            loop  {
                r = setup_subexp_call((*node).u.cons.car, env);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        5 => { r = setup_subexp_call((*node).u.qtfr.target, env) }
        6 => { r = setup_subexp_call((*node).u.enclose.target, env) }
        10 => {
            let mut cn: *mut CallNode = &mut (*node).u.call;
            let mut nodes: *mut *mut Node =
                if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null()
                   {
                    (*env).mem_nodes_dynamic
                } else { (*env).mem_nodes_static.as_mut_ptr() };
            let mut gnum: libc::c_int = 0;
            if (*cn).group_num != 0 as libc::c_int {
                gnum = (*cn).group_num;
                if (*env).num_named > 0 as libc::c_int &&
                       (*(*env).syntax).behavior &
                           (1 as libc::c_uint) << 7 as libc::c_int !=
                           0 as libc::c_int as libc::c_uint &&
                       (*env).option &
                           ((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                     1 as libc::c_int) << 1 as libc::c_int) <<
                                   1 as libc::c_int) << 1 as libc::c_int) <<
                                 1 as libc::c_int) << 1 as libc::c_int) <<
                               1 as libc::c_int == 0 {
                    return -(209 as libc::c_int)
                }
                if gnum > (*env).num_mem {
                    onig_scan_env_set_error_string(env, -(218 as libc::c_int),
                                                   (*cn).name,
                                                   (*cn).name_end);
                    return -(218 as libc::c_int)
                }
            } else {
                let mut refs: *mut libc::c_int = 0 as *mut libc::c_int;
                let mut n: libc::c_int =
                    onig_name_to_group_numbers((*env).reg, (*cn).name,
                                               (*cn).name_end, &mut refs);
                if n <= 0 as libc::c_int {
                    onig_scan_env_set_error_string(env, -(217 as libc::c_int),
                                                   (*cn).name,
                                                   (*cn).name_end);
                    return -(217 as libc::c_int)
                } else if n > 1 as libc::c_int {
                    onig_scan_env_set_error_string(env, -(220 as libc::c_int),
                                                   (*cn).name,
                                                   (*cn).name_end);
                    return -(220 as libc::c_int)
                } else {
                    (*cn).group_num = *refs.offset(0 as libc::c_int as isize)
                }
            }
            (*cn).target = *nodes.offset((*cn).group_num as isize);
            if ((*cn).target as *mut libc::c_void).is_null() {
                onig_scan_env_set_error_string(env, -(217 as libc::c_int),
                                               (*cn).name, (*cn).name_end);
                return -(217 as libc::c_int)
            }
            (*(*cn).target).u.enclose.state |=
                (1 as libc::c_int) << 8 as libc::c_int;
            if (*cn).group_num <
                   (::std::mem::size_of::<BitStatusType>() as
                        libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                        libc::c_ulong) as
                       libc::c_int {
                (*env).bt_mem_start |=
                    ((1 as libc::c_int) << (*cn).group_num) as libc::c_uint
            } else { (*env).bt_mem_start |= 1 as libc::c_int as libc::c_uint }
            (*cn).unset_addr_list = (*env).unset_addr_list
        }
        7 => {
            let mut an: *mut AnchorNode = &mut (*node).u.anchor;
            match (*an).type_0 {
                1024 | 2048 | 4096 | 8192 => {
                    r = setup_subexp_call((*an).target, env)
                }
                _ => { }
            }
        }
        _ => { }
    }
    return r;
}
/* divide different length alternatives in look-behind.
  (?<=A|B) ==> (?<=A)|(?<=B)
  (?<!A|B) ==> (?<!A)(?<!B)
*/
unsafe extern "C" fn divide_look_behind_alternatives(mut node: *mut Node)
 -> libc::c_int {
    let mut head: *mut Node = 0 as *mut Node;
    let mut np: *mut Node = 0 as *mut Node;
    let mut insert_node: *mut Node = 0 as *mut Node;
    let mut an: *mut AnchorNode = &mut (*node).u.anchor;
    let mut anc_type: libc::c_int = (*an).type_0;
    /* fprintf(stderr, "divide_look_behind: %d\n", (int )node); */
    head = (*an).target;
    np = (*head).u.cons.car;
    swap_node(node, head);
    (*node).u.cons.car = head;
    (*head).u.anchor.target = np;
    np = node;
    loop  {
        np = (*np).u.cons.cdr;
        if np.is_null() { break ; }
        insert_node = onig_node_new_anchor(anc_type);
        if (insert_node as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        (*insert_node).u.anchor.target = (*np).u.cons.car;
        (*np).u.cons.car = insert_node
    }
    if anc_type == (1 as libc::c_int) << 13 as libc::c_int {
        np = node;
        loop  {
            (*np).u.base.type_0 = 8 as libc::c_int;
            np = (*np).u.cons.cdr;
            if np.is_null() { break ; }
            /* alt -> list */
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setup_look_behind(mut node: *mut Node,
                                       mut reg: *mut regex_t,
                                       mut env: *mut ScanEnv) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut an: *mut AnchorNode = &mut (*node).u.anchor;
    /* fprintf(stderr, "setup_look_behind: %x\n", (int )node); */
    r = get_char_length_tree((*an).target, reg, &mut len);
    if r == 0 as libc::c_int {
        (*an).char_len = len
    } else if r == -(1 as libc::c_int) {
        r = -(122 as libc::c_int)
    } else if r == -(2 as libc::c_int) {
        if (*(*env).syntax).behavior & (1 as libc::c_uint) << 6 as libc::c_int
               != 0 as libc::c_int as libc::c_uint {
            r = divide_look_behind_alternatives(node)
        } else { r = -(122 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn next_setup(mut node: *mut Node, mut next_node: *mut Node,
                                mut reg: *mut regex_t) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    loop  {
        type_0 = (*node).u.base.type_0;
        if type_0 == 5 as libc::c_int {
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            if (*qn).greedy != 0 && (*qn).upper == -(1 as libc::c_int) {
                let mut n: *mut Node =
                    get_head_value_node(next_node, 1 as libc::c_int, reg);
                /* '\0': for UTF-16BE etc... */
                if !(n as *mut libc::c_void).is_null() &&
                       *(*n).u.str_0.s.offset(0 as libc::c_int as isize) as
                           libc::c_int != '\u{0}' as i32 {
                    (*qn).next_head_exact = n
                }
                /* automatic posseivation a*b ==> (?>a*)b */
                if (*qn).lower <= 1 as libc::c_int {
                    let mut ttype: libc::c_int =
                        (*(*qn).target).u.base.type_0;
                    if (1 as libc::c_int) << ttype &
                           ((1 as libc::c_int) << 0 as libc::c_int |
                                (1 as libc::c_int) << 1 as libc::c_int |
                                (1 as libc::c_int) << 2 as libc::c_int |
                                (1 as libc::c_int) << 3 as libc::c_int |
                                (1 as libc::c_int) << 4 as libc::c_int) !=
                           0 as libc::c_int {
                        let mut x: *mut Node = 0 as *mut Node;
                        let mut y: *mut Node = 0 as *mut Node;
                        x =
                            get_head_value_node((*qn).target,
                                                0 as libc::c_int, reg);
                        if !(x as *mut libc::c_void).is_null() {
                            y =
                                get_head_value_node(next_node,
                                                    0 as libc::c_int, reg);
                            if !(y as *mut libc::c_void).is_null() &&
                                   is_not_included(x, y, reg) != 0 {
                                let mut en: *mut Node =
                                    onig_node_new_enclose((1 as libc::c_int)
                                                              <<
                                                              2 as
                                                                  libc::c_int);
                                if (en as *mut libc::c_void).is_null() {
                                    return -(5 as libc::c_int)
                                }
                                (*en).u.enclose.state |=
                                    (1 as libc::c_int) << 6 as libc::c_int;
                                swap_node(node, en);
                                (*node).u.enclose.target = en
                            }
                        }
                    }
                }
            }
            break ;
        } else {
            if !(type_0 == 6 as libc::c_int) { break ; }
            let mut en_0: *mut EncloseNode = &mut (*node).u.enclose;
            if !((*en_0).type_0 == (1 as libc::c_int) << 0 as libc::c_int) {
                break ;
            }
            node = (*en_0).target
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn update_string_node_case_fold(mut reg: *mut regex_t,
                                                  mut node: *mut Node)
 -> libc::c_int {
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut buf: [OnigUChar; 18] = [0; 18];
    let mut sbuf: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut ebuf: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut sp: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut sbuf_size: libc::c_int = 0;
    let mut sn: *mut StrNode = &mut (*node).u.str_0;
    end = (*sn).end;
    sbuf_size =
        (end.wrapping_offset_from((*sn).s) as libc::c_long *
             2 as libc::c_int as libc::c_long) as libc::c_int;
    sbuf = malloc(sbuf_size as libc::c_ulong) as *mut OnigUChar;
    if (sbuf as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    ebuf = sbuf.offset(sbuf_size as isize);
    sp = sbuf;
    p = (*sn).s;
    while p < end {
        len =
            (*(*reg).enc).mbc_case_fold.expect("non-null function pointer")((*reg).case_fold_flag,
                                                                            &mut p
                                                                                as
                                                                                *mut *mut OnigUChar
                                                                                as
                                                                                *mut *const OnigUChar,
                                                                            end,
                                                                            buf.as_mut_ptr());
        i = 0 as libc::c_int;
        while i < len {
            if sp >= ebuf {
                sbuf =
                    realloc(sbuf as *mut libc::c_void,
                            (sbuf_size * 2 as libc::c_int) as libc::c_ulong)
                        as *mut OnigUChar;
                if (sbuf as *mut libc::c_void).is_null() {
                    return -(5 as libc::c_int)
                }
                sp = sbuf.offset(sbuf_size as isize);
                sbuf_size *= 2 as libc::c_int;
                ebuf = sbuf.offset(sbuf_size as isize)
            }
            let fresh3 = sp;
            sp = sp.offset(1);
            *fresh3 = buf[i as usize];
            i += 1
        }
    }
    r = onig_node_str_set(node, sbuf, sp);
    if r != 0 as libc::c_int { free(sbuf as *mut libc::c_void); return r }
    free(sbuf as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn expand_case_fold_make_rem_string(mut rnode:
                                                          *mut *mut Node,
                                                      mut s: *mut OnigUChar,
                                                      mut end: *mut OnigUChar,
                                                      mut reg: *mut regex_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    node = onig_node_new_str(s, end);
    if (node as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    r = update_string_node_case_fold(reg, node);
    if r != 0 as libc::c_int { onig_node_free(node); return r }
    (*node).u.str_0.flag |=
        ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
    (*node).u.str_0.flag |=
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    *rnode = node;
    return 0 as libc::c_int;
}
unsafe extern "C" fn expand_case_fold_string_alt(mut item_num: libc::c_int,
                                                 mut items:
                                                     *mut OnigCaseFoldCodeItem,
                                                 mut p: *mut OnigUChar,
                                                 mut slen: libc::c_int,
                                                 mut end: *mut OnigUChar,
                                                 mut reg: *mut regex_t,
                                                 mut rnode: *mut *mut Node)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut varlen: libc::c_int = 0;
    let mut anode: *mut Node = 0 as *mut Node;
    let mut var_anode: *mut Node = 0 as *mut Node;
    let mut snode: *mut Node = 0 as *mut Node;
    let mut xnode: *mut Node = 0 as *mut Node;
    let mut an: *mut Node = 0 as *mut Node;
    let mut buf: [OnigUChar; 7] = [0; 7];
    var_anode = 0 as *mut Node;
    *rnode = var_anode;
    varlen = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < item_num {
        if (*items.offset(i as isize)).byte_len != slen {
            varlen = 1 as libc::c_int;
            break ;
        } else { i += 1 }
    }
    if varlen != 0 as libc::c_int {
        var_anode = onig_node_new_alt(0 as *mut Node, 0 as *mut Node);
        *rnode = var_anode;
        if (var_anode as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        xnode = onig_node_new_list(0 as *mut Node, 0 as *mut Node);
        if (xnode as *mut libc::c_void).is_null() {
            current_block = 4242302208140097345;
        } else {
            (*var_anode).u.cons.car = xnode;
            anode = onig_node_new_alt(0 as *mut Node, 0 as *mut Node);
            if (anode as *mut libc::c_void).is_null() {
                current_block = 4242302208140097345;
            } else {
                (*xnode).u.cons.car = anode;
                current_block = 5783071609795492627;
            }
        }
    } else {
        anode = onig_node_new_alt(0 as *mut Node, 0 as *mut Node);
        *rnode = anode;
        if (anode as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        current_block = 5783071609795492627;
    }
    match current_block {
        5783071609795492627 => {
            snode = onig_node_new_str(p, p.offset(slen as isize));
            if !(snode as *mut libc::c_void).is_null() {
                (*anode).u.cons.car = snode;
                i = 0 as libc::c_int;
                's_127:
                    loop  {
                        if !(i < item_num) {
                            current_block = 3580086814630675314;
                            break ;
                        }
                        snode =
                            onig_node_new_str(0 as *const OnigUChar,
                                              0 as *const OnigUChar);
                        if (snode as *mut libc::c_void).is_null() {
                            current_block = 4242302208140097345;
                            break ;
                        }
                        j = 0 as libc::c_int;
                        while j < (*items.offset(i as isize)).code_len {
                            len =
                                (*(*reg).enc).code_to_mbc.expect("non-null function pointer")((*items.offset(i
                                                                                                                 as
                                                                                                                 isize)).code[j
                                                                                                                                  as
                                                                                                                                  usize],
                                                                                              buf.as_mut_ptr());
                            if len < 0 as libc::c_int {
                                r = len;
                                current_block = 14360672825188373104;
                                break 's_127 ;
                            } else {
                                r =
                                    onig_node_str_cat(snode, buf.as_mut_ptr(),
                                                      buf.as_mut_ptr().offset(len
                                                                                  as
                                                                                  isize));
                                if r != 0 as libc::c_int {
                                    current_block = 14360672825188373104;
                                    break 's_127 ;
                                }
                                j += 1
                            }
                        }
                        an =
                            onig_node_new_alt(0 as *mut Node, 0 as *mut Node);
                        if (an as *mut libc::c_void).is_null() {
                            current_block = 14360672825188373104;
                            break ;
                        }
                        if (*items.offset(i as isize)).byte_len != slen {
                            let mut rem: *mut Node = 0 as *mut Node;
                            let mut q: *mut OnigUChar =
                                p.offset((*items.offset(i as isize)).byte_len
                                             as isize);
                            if q < end {
                                r =
                                    expand_case_fold_make_rem_string(&mut rem,
                                                                     q, end,
                                                                     reg);
                                if r != 0 as libc::c_int {
                                    onig_node_free(an);
                                    current_block = 14360672825188373104;
                                    break ;
                                } else {
                                    xnode =
                                        onig_node_list_add(0 as *mut Node,
                                                           snode);
                                    if (xnode as *mut libc::c_void).is_null()
                                       {
                                        onig_node_free(an);
                                        onig_node_free(rem);
                                        current_block = 14360672825188373104;
                                        break ;
                                    } else if (onig_node_list_add(xnode, rem)
                                                   as
                                                   *mut libc::c_void).is_null()
                                     {
                                        onig_node_free(an);
                                        onig_node_free(xnode);
                                        onig_node_free(rem);
                                        current_block = 4242302208140097345;
                                        break ;
                                    } else { (*an).u.cons.car = xnode }
                                }
                            } else { (*an).u.cons.car = snode }
                            (*var_anode).u.cons.cdr = an;
                            var_anode = an
                        } else {
                            (*an).u.cons.car = snode;
                            (*anode).u.cons.cdr = an;
                            anode = an
                        }
                        i += 1
                    }
                match current_block {
                    4242302208140097345 => { }
                    _ => {
                        match current_block {
                            3580086814630675314 => { return varlen }
                            _ => { onig_node_free(snode); }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    onig_node_free(*rnode);
    return -(5 as libc::c_int);
}
unsafe extern "C" fn expand_case_fold_string(mut node: *mut Node,
                                             mut reg: *mut regex_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut alt_num: libc::c_int = 0;
    let mut start: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut top_root: *mut Node = 0 as *mut Node;
    let mut root: *mut Node = 0 as *mut Node;
    let mut snode: *mut Node = 0 as *mut Node;
    let mut prev_node: *mut Node = 0 as *mut Node;
    let mut items: [OnigCaseFoldCodeItem; 13] =
        [OnigCaseFoldCodeItem{byte_len: 0, code_len: 0, code: [0; 3],}; 13];
    let mut sn: *mut StrNode = &mut (*node).u.str_0;
    if (*node).u.str_0.flag &
           ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    start = (*sn).s;
    end = (*sn).end;
    if start >= end { return 0 as libc::c_int }
    r = 0 as libc::c_int;
    snode = 0 as *mut Node;
    prev_node = snode;
    root = prev_node;
    top_root = root;
    alt_num = 1 as libc::c_int;
    p = start;
    loop  {
        if !(p < end) { current_block = 5159818223158340697; break ; }
        n =
            (*(*reg).enc).get_case_fold_codes_by_str.expect("non-null function pointer")((*reg).case_fold_flag,
                                                                                         p,
                                                                                         end,
                                                                                         items.as_mut_ptr());
        if n < 0 as libc::c_int {
            r = n;
            current_block = 4640245532892785160;
            break ;
        } else {
            len =
                (*(*reg).enc).mbc_enc_len.expect("non-null function pointer")(p);
            if n == 0 as libc::c_int {
                if (snode as *mut libc::c_void).is_null() {
                    if (root as *mut libc::c_void).is_null() &&
                           !(prev_node as *mut libc::c_void).is_null() {
                        root = onig_node_list_add(0 as *mut Node, prev_node);
                        top_root = root;
                        if (root as *mut libc::c_void).is_null() {
                            onig_node_free(prev_node);
                            current_block = 2994717517321046937;
                            break ;
                        }
                    }
                    snode =
                        onig_node_new_str(0 as *const OnigUChar,
                                          0 as *const OnigUChar);
                    prev_node = snode;
                    if (snode as *mut libc::c_void).is_null() {
                        current_block = 2994717517321046937;
                        break ;
                    }
                    if !(root as *mut libc::c_void).is_null() {
                        if (onig_node_list_add(root, snode) as
                                *mut libc::c_void).is_null() {
                            onig_node_free(snode);
                            current_block = 2994717517321046937;
                            break ;
                        }
                    }
                }
                r = onig_node_str_cat(snode, p, p.offset(len as isize));
                if r != 0 as libc::c_int {
                    current_block = 4640245532892785160;
                    break ;
                }
            } else {
                alt_num *= n + 1 as libc::c_int;
                if alt_num > 8 as libc::c_int {
                    current_block = 5159818223158340697;
                    break ;
                }
                if (root as *mut libc::c_void).is_null() &&
                       !(prev_node as *mut libc::c_void).is_null() {
                    root = onig_node_list_add(0 as *mut Node, prev_node);
                    top_root = root;
                    if (root as *mut libc::c_void).is_null() {
                        onig_node_free(prev_node);
                        current_block = 2994717517321046937;
                        break ;
                    }
                }
                r =
                    expand_case_fold_string_alt(n, items.as_mut_ptr(), p, len,
                                                end, reg, &mut prev_node);
                if r < 0 as libc::c_int {
                    current_block = 2994717517321046937;
                    break ;
                }
                if r == 1 as libc::c_int {
                    if (root as *mut libc::c_void).is_null() {
                        top_root = prev_node
                    } else if (onig_node_list_add(root, prev_node) as
                                   *mut libc::c_void).is_null() {
                        onig_node_free(prev_node);
                        current_block = 2994717517321046937;
                        break ;
                    }
                    root = (*prev_node).u.cons.car
                } else if !(root as *mut libc::c_void).is_null() {
                    if (onig_node_list_add(root, prev_node) as
                            *mut libc::c_void).is_null() {
                        onig_node_free(prev_node);
                        current_block = 2994717517321046937;
                        break ;
                    }
                }
                snode = 0 as *mut Node
            }
            p = p.offset(len as isize)
        }
    }
    match current_block {
        5159818223158340697 => {
            if p < end {
                let mut srem: *mut Node = 0 as *mut Node;
                r = expand_case_fold_make_rem_string(&mut srem, p, end, reg);
                if r != 0 as libc::c_int {
                    current_block = 2994717517321046937;
                } else {
                    if !(prev_node as *mut libc::c_void).is_null() &&
                           (root as *mut libc::c_void).is_null() {
                        root = onig_node_list_add(0 as *mut Node, prev_node);
                        top_root = root;
                        if (root as *mut libc::c_void).is_null() {
                            onig_node_free(srem);
                            onig_node_free(prev_node);
                            current_block = 2994717517321046937;
                        } else { current_block = 15669289850109000831; }
                    } else { current_block = 15669289850109000831; }
                    match current_block {
                        2994717517321046937 => { }
                        _ => {
                            if (root as *mut libc::c_void).is_null() {
                                prev_node = srem;
                                current_block = 3229571381435211107;
                            } else if (onig_node_list_add(root, srem) as
                                           *mut libc::c_void).is_null() {
                                onig_node_free(srem);
                                current_block = 2994717517321046937;
                            } else { current_block = 3229571381435211107; }
                        }
                    }
                }
            } else { current_block = 3229571381435211107; }
            match current_block {
                2994717517321046937 => { }
                _ => {
                    /* r == 0 */
                    /* ending */
                    top_root =
                        if !(top_root as *mut libc::c_void).is_null() {
                            top_root
                        } else { prev_node };
                    swap_node(node, top_root);
                    onig_node_free(top_root);
                    return 0 as libc::c_int
                }
            }
        }
        _ => { }
    }
    match current_block {
        2994717517321046937 => { r = -(5 as libc::c_int) }
        _ => { }
    }
    onig_node_free(top_root);
    return r;
}
/* setup_tree does the following work.
 1. check empty loop. (set qn->target_empty_info)
 2. expand ignore-case in char class.
 3. set memory status bit flags. (reg->mem_stats)
 4. set qn->head_exact for [push, exact] -> [push_or_jump_exact1, exact].
 5. find invalid patterns in look-behind.
 6. expand repeated string.
 */
unsafe extern "C" fn setup_tree(mut node: *mut Node, mut reg: *mut regex_t,
                                mut state: libc::c_int, mut env: *mut ScanEnv)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    type_0 = (*node).u.base.type_0;
    let mut current_block_120: u64;
    match type_0 {
        8 => {
            let mut prev: *mut Node = 0 as *mut Node;
            loop  {
                r = setup_tree((*node).u.cons.car, reg, state, env);
                if !(prev as *mut libc::c_void).is_null() &&
                       r == 0 as libc::c_int {
                    r = next_setup(prev, (*node).u.cons.car, reg)
                }
                prev = (*node).u.cons.car;
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            loop  {
                r =
                    setup_tree((*node).u.cons.car, reg,
                               state | (1 as libc::c_int) << 0 as libc::c_int,
                               env);
                if !(r == 0 as libc::c_int &&
                         {
                             node = (*node).u.cons.cdr;
                             !(node as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        0 => {
            if (*reg).options & 1 as libc::c_uint != 0 &&
                   !((*node).u.str_0.flag &
                         ((1 as libc::c_int) << 0 as libc::c_int) as
                             libc::c_uint != 0 as libc::c_int as libc::c_uint)
               {
                r = expand_case_fold_string(node, reg)
            }
        }
        4 => {
            let mut i: libc::c_int = 0;
            let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut nodes: *mut *mut Node =
                if !((*env).mem_nodes_dynamic as *mut libc::c_void).is_null()
                   {
                    (*env).mem_nodes_dynamic
                } else { (*env).mem_nodes_static.as_mut_ptr() };
            let mut br: *mut BRefNode = &mut (*node).u.bref;
            p =
                if !((*br).back_dynamic as *mut libc::c_void).is_null() {
                    (*br).back_dynamic
                } else { (*br).back_static.as_mut_ptr() };
            i = 0 as libc::c_int;
            while i < (*br).back_num {
                if *p.offset(i as isize) > (*env).num_mem {
                    return -(208 as libc::c_int)
                }
                if *p.offset(i as isize) <
                       (::std::mem::size_of::<BitStatusType>() as
                            libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                            libc::c_ulong) as
                           libc::c_int {
                    (*env).backrefed_mem |=
                        ((1 as libc::c_int) << *p.offset(i as isize)) as
                            libc::c_uint
                } else {
                    (*env).backrefed_mem |= 1 as libc::c_int as libc::c_uint
                }
                if *p.offset(i as isize) <
                       (::std::mem::size_of::<BitStatusType>() as
                            libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                            libc::c_ulong) as
                           libc::c_int {
                    (*env).bt_mem_start |=
                        ((1 as libc::c_int) << *p.offset(i as isize)) as
                            libc::c_uint
                } else {
                    (*env).bt_mem_start |= 1 as libc::c_int as libc::c_uint
                }
                if (*br).state & (1 as libc::c_int) << 13 as libc::c_int !=
                       0 as libc::c_int {
                    if *p.offset(i as isize) <
                           (::std::mem::size_of::<BitStatusType>() as
                                libc::c_ulong).wrapping_mul(8 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                               as libc::c_int {
                        (*env).bt_mem_end |=
                            ((1 as libc::c_int) << *p.offset(i as isize)) as
                                libc::c_uint
                    } else {
                        (*env).bt_mem_end |= 1 as libc::c_int as libc::c_uint
                    }
                }
                (**nodes.offset(*p.offset(i as isize) as
                                    isize)).u.enclose.state |=
                    (1 as libc::c_int) << 5 as libc::c_int;
                i += 1
            }
        }
        5 => {
            let mut d: OnigLen = 0;
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            let mut target: *mut Node = (*qn).target;
            if state & (1 as libc::c_int) << 2 as libc::c_int !=
                   0 as libc::c_int {
                (*qn).state |= (1 as libc::c_int) << 12 as libc::c_int
            }
            if (*qn).upper == -(1 as libc::c_int) ||
                   (*qn).upper >= 1 as libc::c_int {
                r = get_min_len(target, &mut d, env);
                if r != 0 {
                    current_block_120 = 11735322225073324345;
                } else if d == 0 as libc::c_int as libc::c_uint {
                    (*qn).target_empty_info = 1 as libc::c_int;
                    r = quantifiers_memory_node_info(target);
                    if r < 0 as libc::c_int {
                        current_block_120 = 11735322225073324345;
                    } else {
                        if r > 0 as libc::c_int {
                            (*qn).target_empty_info = r
                        }
                        current_block_120 = 15594839951440953787;
                    }
                } else { current_block_120 = 15594839951440953787; }
            } else { current_block_120 = 15594839951440953787; }
            match current_block_120 {
                11735322225073324345 => { }
                _ => {
                    state |= (1 as libc::c_int) << 2 as libc::c_int;
                    if (*qn).lower != (*qn).upper {
                        state |= (1 as libc::c_int) << 3 as libc::c_int
                    }
                    r = setup_tree(target, reg, state, env);
                    if !(r != 0) {
                        /* expand string */
                        if (*target).u.base.type_0 == 0 as libc::c_int {
                            if !((*qn).lower == -(1 as libc::c_int)) &&
                                   (*qn).lower == (*qn).upper &&
                                   (*qn).lower > 1 as libc::c_int &&
                                   (*qn).lower <= 100 as libc::c_int {
                                let mut len: libc::c_int =
                                    (*target).u.str_0.end.wrapping_offset_from((*target).u.str_0.s)
                                        as libc::c_long as libc::c_int;
                                let mut sn: *mut StrNode =
                                    &mut (*target).u.str_0;
                                if len * (*qn).lower <= 100 as libc::c_int {
                                    let mut i_0: libc::c_int = 0;
                                    let mut n: libc::c_int = (*qn).lower;
                                    onig_node_conv_to_str_node(node,
                                                               (*target).u.str_0.flag
                                                                   as
                                                                   libc::c_int);
                                    i_0 = 0 as libc::c_int;
                                    while i_0 < n {
                                        r =
                                            onig_node_str_cat(node, (*sn).s,
                                                              (*sn).end);
                                        if r != 0 { break ; }
                                        i_0 += 1
                                    }
                                    onig_node_free(target);
                                    current_block_120 = 11735322225073324345;
                                    /* break case NT_QTFR: */
                                } else {
                                    current_block_120 = 54079586644752974;
                                }
                            } else { current_block_120 = 54079586644752974; }
                        } else { current_block_120 = 54079586644752974; }
                        match current_block_120 {
                            11735322225073324345 => { }
                            _ => {
                                if (*qn).greedy != 0 &&
                                       (*qn).target_empty_info !=
                                           0 as libc::c_int {
                                    if (*target).u.base.type_0 ==
                                           5 as libc::c_int {
                                        let mut tqn: *mut QtfrNode =
                                            &mut (*target).u.qtfr;
                                        if !((*tqn).head_exact as
                                                 *mut libc::c_void).is_null()
                                           {
                                            (*qn).head_exact =
                                                (*tqn).head_exact;
                                            (*tqn).head_exact =
                                                0 as *mut _Node
                                        }
                                    } else {
                                        (*qn).head_exact =
                                            get_head_value_node((*qn).target,
                                                                1 as
                                                                    libc::c_int,
                                                                reg)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en).type_0 {
                2 => {
                    let mut options: OnigOptionType = (*reg).options;
                    (*reg).options = (*node).u.enclose.option;
                    r = setup_tree((*node).u.enclose.target, reg, state, env);
                    (*reg).options = options
                }
                1 => {
                    if state &
                           ((1 as libc::c_int) << 0 as libc::c_int |
                                (1 as libc::c_int) << 1 as libc::c_int |
                                (1 as libc::c_int) << 3 as libc::c_int |
                                (1 as libc::c_int) << 4 as libc::c_int) !=
                           0 as libc::c_int {
                        if (*en).regnum <
                               (::std::mem::size_of::<BitStatusType>() as
                                    libc::c_ulong).wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                   as libc::c_int {
                            (*env).bt_mem_start |=
                                ((1 as libc::c_int) << (*en).regnum) as
                                    libc::c_uint
                        } else {
                            (*env).bt_mem_start |=
                                1 as libc::c_int as libc::c_uint
                        }
                        /* SET_ENCLOSE_STATUS(node, NST_MEM_IN_ALT_NOT); */
                    }
                    if (*en).state & (1 as libc::c_int) << 8 as libc::c_int !=
                           0 as libc::c_int {
                        state |= (1 as libc::c_int) << 4 as libc::c_int
                    }
                    if (*en).state & (1 as libc::c_int) << 7 as libc::c_int !=
                           0 as libc::c_int {
                        state |= (1 as libc::c_int) << 5 as libc::c_int
                    } else if state & (1 as libc::c_int) << 5 as libc::c_int
                                  != 0 as libc::c_int {
                        (*node).u.call.state |=
                            (1 as libc::c_int) << 7 as libc::c_int
                    }
                    r = setup_tree((*en).target, reg, state, env)
                }
                4 => {
                    let mut target_0: *mut Node = (*en).target;
                    r = setup_tree(target_0, reg, state, env);
                    if (*target_0).u.base.type_0 == 5 as libc::c_int {
                        let mut tqn_0: *mut QtfrNode =
                            &mut (*target_0).u.qtfr;
                        if (*tqn_0).upper == -(1 as libc::c_int) &&
                               (*tqn_0).lower <= 1 as libc::c_int &&
                               (*tqn_0).greedy != 0 as libc::c_int {
                            /* (?>a*), a*+ etc... */
                            let mut qtype: libc::c_int =
                                (*(*tqn_0).target).u.base.type_0;
                            if (1 as libc::c_int) << qtype &
                                   ((1 as libc::c_int) << 0 as libc::c_int |
                                        (1 as libc::c_int) << 1 as libc::c_int
                                        |
                                        (1 as libc::c_int) << 2 as libc::c_int
                                        |
                                        (1 as libc::c_int) << 3 as libc::c_int
                                        |
                                        (1 as libc::c_int) <<
                                            4 as libc::c_int) !=
                                   0 as libc::c_int {
                                (*node).u.enclose.state |=
                                    (1 as libc::c_int) << 6 as libc::c_int
                            }
                        }
                    }
                }
                _ => { }
            }
        }
        7 => {
            let mut an: *mut AnchorNode = &mut (*node).u.anchor;
            match (*an).type_0 {
                1024 => { r = setup_tree((*an).target, reg, state, env) }
                2048 => {
                    r =
                        setup_tree((*an).target, reg,
                                   state |
                                       (1 as libc::c_int) << 1 as libc::c_int,
                                   env)
                }
                4096 => {
                    /* allowed node types in look-behind */
                    r =
                        check_type_tree((*an).target,
                                        (1 as libc::c_int) << 8 as libc::c_int
                                            |
                                            (1 as libc::c_int) <<
                                                9 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                0 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                1 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                2 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                3 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                7 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                6 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                5 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                10 as libc::c_int,
                                        (1 as libc::c_int) << 0 as libc::c_int
                                            |
                                            (1 as libc::c_int) <<
                                                1 as libc::c_int,
                                        (1 as libc::c_int) <<
                                            12 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                1 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                5 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                0 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                2 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                6 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                7 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                8 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                9 as libc::c_int);
                    if r < 0 as libc::c_int { return r }
                    if r > 0 as libc::c_int { return -(122 as libc::c_int) }
                    r = setup_tree((*an).target, reg, state, env);
                    if r != 0 as libc::c_int { return r }
                    r = setup_look_behind(node, reg, env)
                }
                8192 => {
                    r =
                        check_type_tree((*an).target,
                                        (1 as libc::c_int) << 8 as libc::c_int
                                            |
                                            (1 as libc::c_int) <<
                                                9 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                0 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                1 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                2 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                3 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                7 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                6 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                5 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                10 as libc::c_int,
                                        (1 as libc::c_int) <<
                                            1 as libc::c_int,
                                        (1 as libc::c_int) <<
                                            12 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                13 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                1 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                5 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                0 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                2 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                6 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                7 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                8 as libc::c_int |
                                            (1 as libc::c_int) <<
                                                9 as libc::c_int);
                    if r < 0 as libc::c_int { return r }
                    if r > 0 as libc::c_int { return -(122 as libc::c_int) }
                    r =
                        setup_tree((*an).target, reg,
                                   state |
                                       (1 as libc::c_int) << 1 as libc::c_int,
                                   env);
                    if r != 0 as libc::c_int { return r }
                    r = setup_look_behind(node, reg, env)
                }
                _ => { }
            }
        }
        1 | 2 | 3 | 10 | _ => { }
    }
    return r;
}
/* set skip map for Boyer-Moore search */
unsafe extern "C" fn set_bm_skip(mut s: *mut OnigUChar,
                                 mut end: *mut OnigUChar,
                                 mut enc: OnigEncoding,
                                 mut skip: *mut OnigUChar,
                                 mut int_skip: *mut *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = end.wrapping_offset_from(s) as libc::c_long as libc::c_int;
    if len < 256 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *skip.offset(i as isize) = len as OnigUChar;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < len - 1 as libc::c_int {
            *skip.offset(*s.offset(i as isize) as isize) =
                (len - 1 as libc::c_int - i) as OnigUChar;
            i += 1
        }
    } else {
        if (*int_skip as *mut libc::c_void).is_null() {
            *int_skip =
                malloc((::std::mem::size_of::<libc::c_int>() as
                            libc::c_ulong).wrapping_mul(256 as libc::c_int as
                                                            libc::c_ulong)) as
                    *mut libc::c_int;
            if (*int_skip as *mut libc::c_void).is_null() {
                return -(5 as libc::c_int)
            }
        }
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *(*int_skip).offset(i as isize) = len;
            i += 1
        }
        i = 0 as libc::c_int;
        while i < len - 1 as libc::c_int {
            *(*int_skip).offset(*s.offset(i as isize) as isize) =
                len - 1 as libc::c_int - i;
            i += 1
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn map_position_value(mut enc: OnigEncoding,
                                        mut i: libc::c_int) -> libc::c_int {
    static mut ByteValTable: [libc::c_short; 128] =
        [5 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short, 1 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         4 as libc::c_int as libc::c_short, 7 as libc::c_int as libc::c_short,
         4 as libc::c_int as libc::c_short, 4 as libc::c_int as libc::c_short,
         4 as libc::c_int as libc::c_short, 4 as libc::c_int as libc::c_short,
         4 as libc::c_int as libc::c_short, 4 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         7 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         7 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         6 as libc::c_int as libc::c_short, 6 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         5 as libc::c_int as libc::c_short, 5 as libc::c_int as libc::c_short,
         1 as libc::c_int as libc::c_short];
    if i <
           (::std::mem::size_of::<[libc::c_short; 128]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_short>()
                                                as libc::c_ulong) as
               libc::c_int {
        if i == 0 as libc::c_int && (*enc).min_enc_len > 1 as libc::c_int {
            return 20 as libc::c_int
        } else { return ByteValTable[i as usize] as libc::c_int }
    } else { return 4 as libc::c_int };
    /* Take it easy. */
}
unsafe extern "C" fn distance_value(mut mm: *mut MinMaxLen) -> libc::c_int {
    /* 1000 / (min-max-dist + 1) */
    static mut dist_vals: [libc::c_short; 100] =
        [1000 as libc::c_int as libc::c_short,
         500 as libc::c_int as libc::c_short,
         333 as libc::c_int as libc::c_short,
         250 as libc::c_int as libc::c_short,
         200 as libc::c_int as libc::c_short,
         167 as libc::c_int as libc::c_short,
         143 as libc::c_int as libc::c_short,
         125 as libc::c_int as libc::c_short,
         111 as libc::c_int as libc::c_short,
         100 as libc::c_int as libc::c_short,
         91 as libc::c_int as libc::c_short,
         83 as libc::c_int as libc::c_short,
         77 as libc::c_int as libc::c_short,
         71 as libc::c_int as libc::c_short,
         67 as libc::c_int as libc::c_short,
         63 as libc::c_int as libc::c_short,
         59 as libc::c_int as libc::c_short,
         56 as libc::c_int as libc::c_short,
         53 as libc::c_int as libc::c_short,
         50 as libc::c_int as libc::c_short,
         48 as libc::c_int as libc::c_short,
         45 as libc::c_int as libc::c_short,
         43 as libc::c_int as libc::c_short,
         42 as libc::c_int as libc::c_short,
         40 as libc::c_int as libc::c_short,
         38 as libc::c_int as libc::c_short,
         37 as libc::c_int as libc::c_short,
         36 as libc::c_int as libc::c_short,
         34 as libc::c_int as libc::c_short,
         33 as libc::c_int as libc::c_short,
         32 as libc::c_int as libc::c_short,
         31 as libc::c_int as libc::c_short,
         30 as libc::c_int as libc::c_short,
         29 as libc::c_int as libc::c_short,
         29 as libc::c_int as libc::c_short,
         28 as libc::c_int as libc::c_short,
         27 as libc::c_int as libc::c_short,
         26 as libc::c_int as libc::c_short,
         26 as libc::c_int as libc::c_short,
         25 as libc::c_int as libc::c_short,
         24 as libc::c_int as libc::c_short,
         24 as libc::c_int as libc::c_short,
         23 as libc::c_int as libc::c_short,
         23 as libc::c_int as libc::c_short,
         22 as libc::c_int as libc::c_short,
         22 as libc::c_int as libc::c_short,
         21 as libc::c_int as libc::c_short,
         21 as libc::c_int as libc::c_short,
         20 as libc::c_int as libc::c_short,
         20 as libc::c_int as libc::c_short,
         20 as libc::c_int as libc::c_short,
         19 as libc::c_int as libc::c_short,
         19 as libc::c_int as libc::c_short,
         19 as libc::c_int as libc::c_short,
         18 as libc::c_int as libc::c_short,
         18 as libc::c_int as libc::c_short,
         18 as libc::c_int as libc::c_short,
         17 as libc::c_int as libc::c_short,
         17 as libc::c_int as libc::c_short,
         17 as libc::c_int as libc::c_short,
         16 as libc::c_int as libc::c_short,
         16 as libc::c_int as libc::c_short,
         16 as libc::c_int as libc::c_short,
         16 as libc::c_int as libc::c_short,
         15 as libc::c_int as libc::c_short,
         15 as libc::c_int as libc::c_short,
         15 as libc::c_int as libc::c_short,
         15 as libc::c_int as libc::c_short,
         14 as libc::c_int as libc::c_short,
         14 as libc::c_int as libc::c_short,
         14 as libc::c_int as libc::c_short,
         14 as libc::c_int as libc::c_short,
         14 as libc::c_int as libc::c_short,
         14 as libc::c_int as libc::c_short,
         13 as libc::c_int as libc::c_short,
         13 as libc::c_int as libc::c_short,
         13 as libc::c_int as libc::c_short,
         13 as libc::c_int as libc::c_short,
         13 as libc::c_int as libc::c_short,
         13 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         12 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         11 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short,
         10 as libc::c_int as libc::c_short];
    let mut d: OnigLen = 0;
    if (*mm).max == !(0 as libc::c_int as OnigLen) { return 0 as libc::c_int }
    d = (*mm).max.wrapping_sub((*mm).min);
    if d <
           (::std::mem::size_of::<[libc::c_short; 100]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_short>()
                                                as libc::c_ulong) as OnigLen {
        /* return dist_vals[d] * 16 / (mm->min + 12); */
        return dist_vals[d as usize] as libc::c_int
    } else { return 1 as libc::c_int }; /* avoid */
}
unsafe extern "C" fn comp_distance_value(mut d1: *mut MinMaxLen,
                                         mut d2: *mut MinMaxLen,
                                         mut v1: libc::c_int,
                                         mut v2: libc::c_int) -> libc::c_int {
    if v2 <= 0 as libc::c_int { return -(1 as libc::c_int) }
    if v1 <= 0 as libc::c_int { return 1 as libc::c_int }
    v1 *= distance_value(d1);
    v2 *= distance_value(d2);
    if v2 > v1 { return 1 as libc::c_int }
    if v2 < v1 { return -(1 as libc::c_int) }
    if (*d2).min < (*d1).min { return 1 as libc::c_int }
    if (*d2).min > (*d1).min { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_equal_mml(mut a: *mut MinMaxLen,
                                  mut b: *mut MinMaxLen) -> libc::c_int {
    return if (*a).min == (*b).min && (*a).max == (*b).max {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
unsafe extern "C" fn set_mml(mut mml: *mut MinMaxLen, mut min: OnigLen,
                             mut max: OnigLen) {
    (*mml).min = min;
    (*mml).max = max;
}
unsafe extern "C" fn clear_mml(mut mml: *mut MinMaxLen) {
    (*mml).max = 0 as libc::c_int as OnigLen;
    (*mml).min = (*mml).max;
}
unsafe extern "C" fn copy_mml(mut to: *mut MinMaxLen,
                              mut from: *mut MinMaxLen) {
    (*to).min = (*from).min;
    (*to).max = (*from).max;
}
unsafe extern "C" fn add_mml(mut to: *mut MinMaxLen,
                             mut from: *mut MinMaxLen) {
    (*to).min = distance_add((*to).min, (*from).min);
    (*to).max = distance_add((*to).max, (*from).max);
}
unsafe extern "C" fn alt_merge_mml(mut to: *mut MinMaxLen,
                                   mut from: *mut MinMaxLen) {
    if (*to).min > (*from).min { (*to).min = (*from).min }
    if (*to).max < (*from).max { (*to).max = (*from).max };
}
unsafe extern "C" fn copy_opt_env(mut to: *mut OptEnv,
                                  mut from: *mut OptEnv) {
    *to = *from;
}
unsafe extern "C" fn clear_opt_anc_info(mut anc: *mut OptAncInfo) {
    (*anc).left_anchor = 0 as libc::c_int;
    (*anc).right_anchor = 0 as libc::c_int;
}
unsafe extern "C" fn copy_opt_anc_info(mut to: *mut OptAncInfo,
                                       mut from: *mut OptAncInfo) {
    *to = *from;
}
unsafe extern "C" fn concat_opt_anc_info(mut to: *mut OptAncInfo,
                                         mut left: *mut OptAncInfo,
                                         mut right: *mut OptAncInfo,
                                         mut left_len: OnigLen,
                                         mut right_len: OnigLen) {
    clear_opt_anc_info(to);
    (*to).left_anchor = (*left).left_anchor;
    if left_len == 0 as libc::c_int as libc::c_uint {
        (*to).left_anchor |= (*right).left_anchor
    }
    (*to).right_anchor = (*right).right_anchor;
    if right_len == 0 as libc::c_int as libc::c_uint {
        (*to).right_anchor |= (*left).right_anchor
    } else {
        (*to).right_anchor |=
            (*left).right_anchor & (1 as libc::c_int) << 11 as libc::c_int
    };
}
unsafe extern "C" fn is_left_anchor(mut anc: libc::c_int) -> libc::c_int {
    if anc == (1 as libc::c_int) << 3 as libc::c_int ||
           anc == (1 as libc::c_int) << 4 as libc::c_int ||
           anc == (1 as libc::c_int) << 5 as libc::c_int ||
           anc == (1 as libc::c_int) << 10 as libc::c_int ||
           anc == (1 as libc::c_int) << 11 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_set_opt_anc_info(mut to: *mut OptAncInfo,
                                         mut anc: libc::c_int)
 -> libc::c_int {
    if (*to).left_anchor & anc != 0 as libc::c_int { return 1 as libc::c_int }
    return if (*to).right_anchor & anc != 0 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
unsafe extern "C" fn add_opt_anc_info(mut to: *mut OptAncInfo,
                                      mut anc: libc::c_int) {
    if is_left_anchor(anc) != 0 {
        (*to).left_anchor |= anc
    } else { (*to).right_anchor |= anc };
}
unsafe extern "C" fn remove_opt_anc_info(mut to: *mut OptAncInfo,
                                         mut anc: libc::c_int) {
    if is_left_anchor(anc) != 0 {
        (*to).left_anchor &= !anc
    } else { (*to).right_anchor &= !anc };
}
unsafe extern "C" fn alt_merge_opt_anc_info(mut to: *mut OptAncInfo,
                                            mut add: *mut OptAncInfo) {
    (*to).left_anchor &= (*add).left_anchor;
    (*to).right_anchor &= (*add).right_anchor;
}
unsafe extern "C" fn is_full_opt_exact_info(mut ex: *mut OptExactInfo)
 -> libc::c_int {
    return if (*ex).len >= 24 as libc::c_int {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
unsafe extern "C" fn clear_opt_exact_info(mut ex: *mut OptExactInfo) {
    clear_mml(&mut (*ex).mmd);
    clear_opt_anc_info(&mut (*ex).anc);
    (*ex).reach_end = 0 as libc::c_int;
    (*ex).ignore_case = 0 as libc::c_int;
    (*ex).len = 0 as libc::c_int;
    (*ex).s[0 as libc::c_int as usize] = '\u{0}' as i32 as OnigUChar;
}
unsafe extern "C" fn copy_opt_exact_info(mut to: *mut OptExactInfo,
                                         mut from: *mut OptExactInfo) {
    *to = *from;
}
unsafe extern "C" fn concat_opt_exact_info(mut to: *mut OptExactInfo,
                                           mut add: *mut OptExactInfo,
                                           mut enc: OnigEncoding) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut tanc: OptAncInfo = OptAncInfo{left_anchor: 0, right_anchor: 0,};
    if (*to).ignore_case == 0 && (*add).ignore_case != 0 {
        if (*to).len >= (*add).len { return }
        (*to).ignore_case = 1 as libc::c_int
    }
    p = (*add).s.as_mut_ptr();
    end = p.offset((*add).len as isize);
    i = (*to).len;
    while p < end {
        len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
        if i + len > 24 as libc::c_int { break ; }
        j = 0 as libc::c_int;
        while j < len && p < end {
            let fresh4 = p;
            p = p.offset(1);
            let fresh5 = i;
            i = i + 1;
            (*to).s[fresh5 as usize] = *fresh4;
            j += 1
        }
    }
    (*to).len = i;
    (*to).reach_end =
        if p == end { (*add).reach_end } else { 0 as libc::c_int };
    concat_opt_anc_info(&mut tanc, &mut (*to).anc, &mut (*add).anc,
                        1 as libc::c_int as OnigLen,
                        1 as libc::c_int as OnigLen);
    if (*to).reach_end == 0 { tanc.right_anchor = 0 as libc::c_int }
    copy_opt_anc_info(&mut (*to).anc, &mut tanc);
}
unsafe extern "C" fn concat_opt_exact_info_str(mut to: *mut OptExactInfo,
                                               mut s: *mut OnigUChar,
                                               mut end: *mut OnigUChar,
                                               mut raw: libc::c_int,
                                               mut enc: OnigEncoding) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut OnigUChar = 0 as *mut OnigUChar;
    i = (*to).len;
    p = s;
    while p < end && i < 24 as libc::c_int {
        len = (*enc).mbc_enc_len.expect("non-null function pointer")(p);
        if i + len > 24 as libc::c_int { break ; }
        j = 0 as libc::c_int;
        while j < len && p < end {
            let fresh6 = p;
            p = p.offset(1);
            let fresh7 = i;
            i = i + 1;
            (*to).s[fresh7 as usize] = *fresh6;
            j += 1
        }
    }
    (*to).len = i;
}
unsafe extern "C" fn alt_merge_opt_exact_info(mut to: *mut OptExactInfo,
                                              mut add: *mut OptExactInfo,
                                              mut env: *mut OptEnv) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if (*add).len == 0 as libc::c_int || (*to).len == 0 as libc::c_int {
        clear_opt_exact_info(to);
        return
    }
    if is_equal_mml(&mut (*to).mmd, &mut (*add).mmd) == 0 {
        clear_opt_exact_info(to);
        return
    }
    i = 0 as libc::c_int;
    while i < (*to).len && i < (*add).len {
        if (*to).s[i as usize] as libc::c_int !=
               (*add).s[i as usize] as libc::c_int {
            break ;
        }
        len =
            (*(*env).enc).mbc_enc_len.expect("non-null function pointer")((*to).s.as_mut_ptr().offset(i
                                                                                                          as
                                                                                                          isize));
        j = 1 as libc::c_int;
        while j < len {
            if (*to).s[(i + j) as usize] as libc::c_int !=
                   (*add).s[(i + j) as usize] as libc::c_int {
                break ;
            }
            j += 1
        }
        if j < len { break ; }
        i += len
    }
    if (*add).reach_end == 0 || i < (*add).len || i < (*to).len {
        (*to).reach_end = 0 as libc::c_int
    }
    (*to).len = i;
    (*to).ignore_case |= (*add).ignore_case;
    alt_merge_opt_anc_info(&mut (*to).anc, &mut (*add).anc);
    if (*to).reach_end == 0 { (*to).anc.right_anchor = 0 as libc::c_int };
}
unsafe extern "C" fn select_opt_exact_info(mut enc: OnigEncoding,
                                           mut now: *mut OptExactInfo,
                                           mut alt: *mut OptExactInfo) {
    let mut v1: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    v1 = (*now).len;
    v2 = (*alt).len;
    if v2 == 0 as libc::c_int {
        return
    } else {
        if v1 == 0 as libc::c_int {
            copy_opt_exact_info(now, alt);
            return
        } else {
            if v1 <= 2 as libc::c_int && v2 <= 2 as libc::c_int {
                /* ByteValTable[x] is big value --> low price */
                v2 =
                    map_position_value(enc,
                                       (*now).s[0 as libc::c_int as usize] as
                                           libc::c_int); /* 32768: something big value */
                v1 =
                    map_position_value(enc,
                                       (*alt).s[0 as libc::c_int as usize] as
                                           libc::c_int);
                if (*now).len > 1 as libc::c_int { v1 += 5 as libc::c_int }
                if (*alt).len > 1 as libc::c_int { v2 += 5 as libc::c_int }
            }
        }
    }
    if (*now).ignore_case == 0 as libc::c_int { v1 *= 2 as libc::c_int }
    if (*alt).ignore_case == 0 as libc::c_int { v2 *= 2 as libc::c_int }
    if comp_distance_value(&mut (*now).mmd, &mut (*alt).mmd, v1, v2) >
           0 as libc::c_int {
        copy_opt_exact_info(now, alt);
    };
}
unsafe extern "C" fn clear_opt_map_info(mut map: *mut OptMapInfo) {
    static mut clean_info: OptMapInfo =
        {
            let mut init =
                OptMapInfo{mmd:
                               {
                                   let mut init =
                                       MinMaxLen{min:
                                                     0 as libc::c_int as
                                                         OnigLen,
                                                 max:
                                                     0 as libc::c_int as
                                                         OnigLen,};
                                   init
                               },
                           anc:
                               {
                                   let mut init =
                                       OptAncInfo{left_anchor:
                                                      0 as libc::c_int,
                                                  right_anchor:
                                                      0 as libc::c_int,};
                                   init
                               },
                           value: 0 as libc::c_int,
                           map:
                               [0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar,
                                0 as libc::c_int as OnigUChar],};
            init
        };
    memcpy(map as *mut libc::c_void,
           &clean_info as *const OptMapInfo as *const libc::c_void,
           ::std::mem::size_of::<OptMapInfo>() as libc::c_ulong);
}
unsafe extern "C" fn copy_opt_map_info(mut to: *mut OptMapInfo,
                                       mut from: *mut OptMapInfo) {
    *to = *from;
}
unsafe extern "C" fn add_char_opt_map_info(mut map: *mut OptMapInfo,
                                           mut c: OnigUChar,
                                           mut enc: OnigEncoding) {
    if (*map).map[c as usize] as libc::c_int == 0 as libc::c_int {
        (*map).map[c as usize] = 1 as libc::c_int as OnigUChar;
        (*map).value += map_position_value(enc, c as libc::c_int)
    };
}
unsafe extern "C" fn add_char_amb_opt_map_info(mut map: *mut OptMapInfo,
                                               mut p: *mut OnigUChar,
                                               mut end: *mut OnigUChar,
                                               mut enc: OnigEncoding,
                                               mut case_fold_flag:
                                                   OnigCaseFoldType)
 -> libc::c_int {
    let mut items: [OnigCaseFoldCodeItem; 13] =
        [OnigCaseFoldCodeItem{byte_len: 0, code_len: 0, code: [0; 3],}; 13];
    let mut buf: [OnigUChar; 7] = [0; 7];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    add_char_opt_map_info(map, *p.offset(0 as libc::c_int as isize), enc);
    case_fold_flag =
        case_fold_flag &
            !((1 as libc::c_int) << 30 as libc::c_int) as libc::c_uint;
    n =
        (*enc).get_case_fold_codes_by_str.expect("non-null function pointer")(case_fold_flag,
                                                                              p,
                                                                              end,
                                                                              items.as_mut_ptr());
    if n < 0 as libc::c_int { return n }
    i = 0 as libc::c_int;
    while i < n {
        (*enc).code_to_mbc.expect("non-null function pointer")(items[i as
                                                                         usize].code[0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize],
                                                               buf.as_mut_ptr());
        add_char_opt_map_info(map, buf[0 as libc::c_int as usize], enc);
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn select_opt_map_info(mut now: *mut OptMapInfo,
                                         mut alt: *mut OptMapInfo) {
    static mut z: libc::c_int = (1 as libc::c_int) << 15 as libc::c_int;
    let mut v1: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    if (*alt).value == 0 as libc::c_int { return }
    if (*now).value == 0 as libc::c_int {
        copy_opt_map_info(now, alt);
        return
    }
    v1 = z / (*now).value;
    v2 = z / (*alt).value;
    if comp_distance_value(&mut (*now).mmd, &mut (*alt).mmd, v1, v2) >
           0 as libc::c_int {
        copy_opt_map_info(now, alt);
    };
}
unsafe extern "C" fn comp_opt_exact_or_map_info(mut e: *mut OptExactInfo,
                                                mut m: *mut OptMapInfo)
 -> libc::c_int {
    let mut ve: libc::c_int = 0;
    let mut vm: libc::c_int = 0;
    if (*m).value <= 0 as libc::c_int { return -(1 as libc::c_int) }
    ve =
        20 as libc::c_int * (*e).len *
            (if (*e).ignore_case != 0 {
                 1 as libc::c_int
             } else { 2 as libc::c_int });
    vm = 20 as libc::c_int * 5 as libc::c_int * 2 as libc::c_int / (*m).value;
    return comp_distance_value(&mut (*e).mmd, &mut (*m).mmd, ve, vm);
}
unsafe extern "C" fn alt_merge_opt_map_info(mut enc: OnigEncoding,
                                            mut to: *mut OptMapInfo,
                                            mut add: *mut OptMapInfo) {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    /* if (! is_equal_mml(&to->mmd, &add->mmd)) return ; */
    if (*to).value == 0 as libc::c_int { return }
    if (*add).value == 0 as libc::c_int || (*to).mmd.max < (*add).mmd.min {
        clear_opt_map_info(to);
        return
    }
    alt_merge_mml(&mut (*to).mmd, &mut (*add).mmd);
    val = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if (*add).map[i as usize] != 0 {
            (*to).map[i as usize] = 1 as libc::c_int as OnigUChar
        }
        if (*to).map[i as usize] != 0 { val += map_position_value(enc, i) }
        i += 1
    }
    (*to).value = val;
    alt_merge_opt_anc_info(&mut (*to).anc, &mut (*add).anc);
}
unsafe extern "C" fn set_bound_node_opt_info(mut opt: *mut NodeOptInfo,
                                             mut mmd: *mut MinMaxLen) {
    copy_mml(&mut (*opt).exb.mmd, mmd);
    copy_mml(&mut (*opt).expr.mmd, mmd);
    copy_mml(&mut (*opt).map.mmd, mmd);
}
unsafe extern "C" fn clear_node_opt_info(mut opt: *mut NodeOptInfo) {
    clear_mml(&mut (*opt).len);
    clear_opt_anc_info(&mut (*opt).anc);
    clear_opt_exact_info(&mut (*opt).exb);
    clear_opt_exact_info(&mut (*opt).exm);
    clear_opt_exact_info(&mut (*opt).expr);
    clear_opt_map_info(&mut (*opt).map);
}
unsafe extern "C" fn copy_node_opt_info(mut to: *mut NodeOptInfo,
                                        mut from: *mut NodeOptInfo) {
    *to = *from;
}
unsafe extern "C" fn concat_left_node_opt_info(mut enc: OnigEncoding,
                                               mut to: *mut NodeOptInfo,
                                               mut add: *mut NodeOptInfo) {
    let mut exb_reach: libc::c_int = 0;
    let mut exm_reach: libc::c_int = 0;
    let mut tanc: OptAncInfo = OptAncInfo{left_anchor: 0, right_anchor: 0,};
    concat_opt_anc_info(&mut tanc, &mut (*to).anc, &mut (*add).anc,
                        (*to).len.max, (*add).len.max);
    copy_opt_anc_info(&mut (*to).anc, &mut tanc);
    if (*add).exb.len > 0 as libc::c_int &&
           (*to).len.max == 0 as libc::c_int as libc::c_uint {
        concat_opt_anc_info(&mut tanc, &mut (*to).anc, &mut (*add).exb.anc,
                            (*to).len.max, (*add).len.max);
        copy_opt_anc_info(&mut (*add).exb.anc, &mut tanc);
    }
    if (*add).map.value > 0 as libc::c_int &&
           (*to).len.max == 0 as libc::c_int as libc::c_uint {
        if (*add).map.mmd.max == 0 as libc::c_int as libc::c_uint {
            (*add).map.anc.left_anchor |= (*to).anc.left_anchor
        }
    }
    exb_reach = (*to).exb.reach_end;
    exm_reach = (*to).exm.reach_end;
    if (*add).len.max != 0 as libc::c_int as libc::c_uint {
        (*to).exm.reach_end = 0 as libc::c_int;
        (*to).exb.reach_end = (*to).exm.reach_end
    }
    if (*add).exb.len > 0 as libc::c_int {
        if exb_reach != 0 {
            concat_opt_exact_info(&mut (*to).exb, &mut (*add).exb, enc);
            clear_opt_exact_info(&mut (*add).exb);
        } else if exm_reach != 0 {
            concat_opt_exact_info(&mut (*to).exm, &mut (*add).exb, enc);
            clear_opt_exact_info(&mut (*add).exb);
        }
    }
    select_opt_exact_info(enc, &mut (*to).exm, &mut (*add).exb);
    select_opt_exact_info(enc, &mut (*to).exm, &mut (*add).exm);
    if (*to).expr.len > 0 as libc::c_int {
        if (*add).len.max > 0 as libc::c_int as libc::c_uint {
            if (*to).expr.len > (*add).len.max as libc::c_int {
                (*to).expr.len = (*add).len.max as libc::c_int
            }
            if (*to).expr.mmd.max == 0 as libc::c_int as libc::c_uint {
                select_opt_exact_info(enc, &mut (*to).exb, &mut (*to).expr);
            } else {
                select_opt_exact_info(enc, &mut (*to).exm, &mut (*to).expr);
            }
        }
    } else if (*add).expr.len > 0 as libc::c_int {
        copy_opt_exact_info(&mut (*to).expr, &mut (*add).expr);
    }
    select_opt_map_info(&mut (*to).map, &mut (*add).map);
    add_mml(&mut (*to).len, &mut (*add).len);
}
unsafe extern "C" fn alt_merge_node_opt_info(mut to: *mut NodeOptInfo,
                                             mut add: *mut NodeOptInfo,
                                             mut env: *mut OptEnv) {
    alt_merge_opt_anc_info(&mut (*to).anc, &mut (*add).anc);
    alt_merge_opt_exact_info(&mut (*to).exb, &mut (*add).exb, env);
    alt_merge_opt_exact_info(&mut (*to).exm, &mut (*add).exm, env);
    alt_merge_opt_exact_info(&mut (*to).expr, &mut (*add).expr, env);
    alt_merge_opt_map_info((*env).enc, &mut (*to).map, &mut (*add).map);
    alt_merge_mml(&mut (*to).len, &mut (*add).len);
}
unsafe extern "C" fn optimize_node_left(mut node: *mut Node,
                                        mut opt: *mut NodeOptInfo,
                                        mut env: *mut OptEnv) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    clear_node_opt_info(opt);
    set_bound_node_opt_info(opt, &mut (*env).mmd);
    type_0 = (*node).u.base.type_0;
    let mut current_block_166: u64;
    match type_0 {
        8 => {
            let mut nenv: OptEnv =
                OptEnv{mmd: MinMaxLen{min: 0, max: 0,},
                       enc: 0 as *mut OnigEncodingType,
                       options: 0,
                       case_fold_flag: 0,
                       scan_env: 0 as *mut ScanEnv,};
            let mut nopt: NodeOptInfo =
                NodeOptInfo{len: MinMaxLen{min: 0, max: 0,},
                            anc: OptAncInfo{left_anchor: 0, right_anchor: 0,},
                            exb:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            exm:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            expr:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            map:
                                OptMapInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                           anc:
                                               OptAncInfo{left_anchor: 0,
                                                          right_anchor: 0,},
                                           value: 0,
                                           map: [0; 256],},};
            let mut nd: *mut Node = node;
            copy_opt_env(&mut nenv, env);
            loop  {
                r =
                    optimize_node_left((*nd).u.cons.car, &mut nopt,
                                       &mut nenv);
                if r == 0 as libc::c_int {
                    add_mml(&mut nenv.mmd, &mut nopt.len);
                    concat_left_node_opt_info((*env).enc, opt, &mut nopt);
                }
                if !(r == 0 as libc::c_int &&
                         {
                             nd = (*nd).u.cons.cdr;
                             !(nd as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        9 => {
            let mut nopt_0: NodeOptInfo =
                NodeOptInfo{len: MinMaxLen{min: 0, max: 0,},
                            anc: OptAncInfo{left_anchor: 0, right_anchor: 0,},
                            exb:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            exm:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            expr:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            map:
                                OptMapInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                           anc:
                                               OptAncInfo{left_anchor: 0,
                                                          right_anchor: 0,},
                                           value: 0,
                                           map: [0; 256],},};
            let mut nd_0: *mut Node = node;
            loop  {
                r = optimize_node_left((*nd_0).u.cons.car, &mut nopt_0, env);
                if r == 0 as libc::c_int {
                    if nd_0 == node {
                        copy_node_opt_info(opt, &mut nopt_0);
                    } else { alt_merge_node_opt_info(opt, &mut nopt_0, env); }
                }
                if !(r == 0 as libc::c_int &&
                         {
                             nd_0 = (*nd_0).u.cons.cdr;
                             !(nd_0 as *mut libc::c_void).is_null()
                         }) {
                    break ;
                }
            }
        }
        0 => {
            let mut sn: *mut StrNode = &mut (*node).u.str_0;
            let mut slen: libc::c_int =
                (*sn).end.wrapping_offset_from((*sn).s) as libc::c_long as
                    libc::c_int;
            let mut is_raw: libc::c_int =
                ((*node).u.str_0.flag &
                     ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                     != 0 as libc::c_int as libc::c_uint) as libc::c_int;
            if !((*node).u.str_0.flag &
                     ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                     != 0 as libc::c_int as libc::c_uint) {
                concat_opt_exact_info_str(&mut (*opt).exb, (*sn).s, (*sn).end,
                                          ((*node).u.str_0.flag &
                                               ((1 as libc::c_int) <<
                                                    0 as libc::c_int) as
                                                   libc::c_uint !=
                                               0 as libc::c_int as
                                                   libc::c_uint) as
                                              libc::c_int, (*env).enc);
                if slen > 0 as libc::c_int {
                    add_char_opt_map_info(&mut (*opt).map, *(*sn).s,
                                          (*env).enc);
                }
                set_mml(&mut (*opt).len, slen as OnigLen, slen as OnigLen);
                current_block_166 = 1434579379687443766;
            } else {
                let mut max: libc::c_int = 0;
                if (*node).u.str_0.flag &
                       ((1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint != 0 as libc::c_int as libc::c_uint {
                    let mut n: libc::c_int =
                        onigenc_strlen((*env).enc, (*sn).s, (*sn).end);
                    max = (*(*env).enc).max_enc_len * n;
                    current_block_166 = 15004371738079956865;
                } else {
                    concat_opt_exact_info_str(&mut (*opt).exb, (*sn).s,
                                              (*sn).end, is_raw, (*env).enc);
                    (*opt).exb.ignore_case = 1 as libc::c_int;
                    if slen > 0 as libc::c_int {
                        r =
                            add_char_amb_opt_map_info(&mut (*opt).map,
                                                      (*sn).s, (*sn).end,
                                                      (*env).enc,
                                                      (*env).case_fold_flag);
                        if r != 0 as libc::c_int {
                            current_block_166 = 12692154259096809850;
                        } else { current_block_166 = 2543120759711851213; }
                    } else { current_block_166 = 2543120759711851213; }
                    match current_block_166 {
                        12692154259096809850 => { }
                        _ => {
                            max = slen;
                            current_block_166 = 15004371738079956865;
                        }
                    }
                }
                match current_block_166 {
                    12692154259096809850 => { }
                    _ => {
                        set_mml(&mut (*opt).len, slen as OnigLen,
                                max as OnigLen);
                        current_block_166 = 1434579379687443766;
                    }
                }
            }
            match current_block_166 {
                12692154259096809850 => { }
                _ => {
                    if (*opt).exb.len == slen {
                        (*opt).exb.reach_end = 1 as libc::c_int
                    }
                }
            }
        }
        1 => {
            let mut i: libc::c_int = 0;
            let mut z: libc::c_int = 0;
            let mut cc: *mut CClassNode = &mut (*node).u.cclass;
            /* no need to check ignore case. (set in setup_tree()) */
            if !((*cc).mbuf as *mut libc::c_void).is_null() ||
                   (*cc).flags &
                       ((1 as libc::c_int) << 0 as libc::c_int) as
                           libc::c_uint != 0 as libc::c_int as libc::c_uint {
                let mut min: OnigLen = (*(*env).enc).min_enc_len as OnigLen;
                let mut max_0: OnigLen = (*(*env).enc).max_enc_len as OnigLen;
                set_mml(&mut (*opt).len, min, max_0);
            } else {
                i = 0 as libc::c_int;
                while i < (1 as libc::c_int) << 8 as libc::c_int {
                    z =
                        ((*cc).bs[(i as
                                       libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                                      as usize] &
                             ((1 as libc::c_int) <<
                                  (i as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)))
                                 as libc::c_uint) as libc::c_int;
                    if z != 0 &&
                           !((*cc).flags &
                                 ((1 as libc::c_int) << 0 as libc::c_int) as
                                     libc::c_uint !=
                                 0 as libc::c_int as libc::c_uint) ||
                           z == 0 &&
                               (*cc).flags &
                                   ((1 as libc::c_int) << 0 as libc::c_int) as
                                       libc::c_uint !=
                                   0 as libc::c_int as libc::c_uint {
                        add_char_opt_map_info(&mut (*opt).map, i as OnigUChar,
                                              (*env).enc);
                    }
                    i += 1
                }
                set_mml(&mut (*opt).len, 1 as libc::c_int as OnigLen,
                        1 as libc::c_int as OnigLen);
            }
        }
        2 => {
            let mut i_0: libc::c_int = 0;
            let mut min_0: libc::c_int = 0;
            let mut max_1: libc::c_int = 0;
            max_1 = (*(*env).enc).max_enc_len;
            if max_1 == 1 as libc::c_int {
                min_0 = 1 as libc::c_int;
                match (*node).u.ctype.ctype {
                    12 => {
                        if (*node).u.ctype.not != 0 as libc::c_int {
                            i_0 = 0 as libc::c_int;
                            while i_0 < (1 as libc::c_int) << 8 as libc::c_int
                                  {
                                if (*(*env).enc).is_code_ctype.expect("non-null function pointer")(i_0
                                                                                                       as
                                                                                                       OnigCodePoint,
                                                                                                   12
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       OnigCtype)
                                       == 0 {
                                    add_char_opt_map_info(&mut (*opt).map,
                                                          i_0 as OnigUChar,
                                                          (*env).enc);
                                }
                                i_0 += 1
                            }
                        } else {
                            i_0 = 0 as libc::c_int;
                            while i_0 < (1 as libc::c_int) << 8 as libc::c_int
                                  {
                                if (*(*env).enc).is_code_ctype.expect("non-null function pointer")(i_0
                                                                                                       as
                                                                                                       OnigCodePoint,
                                                                                                   12
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       OnigCtype)
                                       != 0 {
                                    add_char_opt_map_info(&mut (*opt).map,
                                                          i_0 as OnigUChar,
                                                          (*env).enc);
                                }
                                i_0 += 1
                            }
                        }
                    }
                    _ => { }
                }
            } else { min_0 = (*(*env).enc).min_enc_len }
            set_mml(&mut (*opt).len, min_0 as OnigLen, max_1 as OnigLen);
        }
        3 => {
            let mut min_1: OnigLen = (*(*env).enc).min_enc_len as OnigLen;
            let mut max_2: OnigLen = (*(*env).enc).max_enc_len as OnigLen;
            set_mml(&mut (*opt).len, min_1, max_2);
        }
        7 => {
            match (*node).u.anchor.type_0 {
                1 | 4 | 2 | 8 | 16 | 32 | 2048 | 4096 => {
                    add_opt_anc_info(&mut (*opt).anc,
                                     (*node).u.anchor.type_0);
                }
                1024 => {
                    let mut nopt_1: NodeOptInfo =
                        NodeOptInfo{len: MinMaxLen{min: 0, max: 0,},
                                    anc:
                                        OptAncInfo{left_anchor: 0,
                                                   right_anchor: 0,},
                                    exb:
                                        OptExactInfo{mmd:
                                                         MinMaxLen{min: 0,
                                                                   max: 0,},
                                                     anc:
                                                         OptAncInfo{left_anchor:
                                                                        0,
                                                                    right_anchor:
                                                                        0,},
                                                     reach_end: 0,
                                                     ignore_case: 0,
                                                     len: 0,
                                                     s: [0; 24],},
                                    exm:
                                        OptExactInfo{mmd:
                                                         MinMaxLen{min: 0,
                                                                   max: 0,},
                                                     anc:
                                                         OptAncInfo{left_anchor:
                                                                        0,
                                                                    right_anchor:
                                                                        0,},
                                                     reach_end: 0,
                                                     ignore_case: 0,
                                                     len: 0,
                                                     s: [0; 24],},
                                    expr:
                                        OptExactInfo{mmd:
                                                         MinMaxLen{min: 0,
                                                                   max: 0,},
                                                     anc:
                                                         OptAncInfo{left_anchor:
                                                                        0,
                                                                    right_anchor:
                                                                        0,},
                                                     reach_end: 0,
                                                     ignore_case: 0,
                                                     len: 0,
                                                     s: [0; 24],},
                                    map:
                                        OptMapInfo{mmd:
                                                       MinMaxLen{min: 0,
                                                                 max: 0,},
                                                   anc:
                                                       OptAncInfo{left_anchor:
                                                                      0,
                                                                  right_anchor:
                                                                      0,},
                                                   value: 0,
                                                   map: [0; 256],},};
                    r =
                        optimize_node_left((*node).u.anchor.target,
                                           &mut nopt_1, env);
                    if r == 0 as libc::c_int {
                        if nopt_1.exb.len > 0 as libc::c_int {
                            copy_opt_exact_info(&mut (*opt).expr,
                                                &mut nopt_1.exb);
                        } else if nopt_1.exm.len > 0 as libc::c_int {
                            copy_opt_exact_info(&mut (*opt).expr,
                                                &mut nopt_1.exm);
                        }
                        (*opt).expr.reach_end = 0 as libc::c_int;
                        if nopt_1.map.value > 0 as libc::c_int {
                            copy_opt_map_info(&mut (*opt).map,
                                              &mut nopt_1.map);
                        }
                    }
                }
                8192 | _ => { }
            }
        }
        4 => {
            let mut i_1: libc::c_int = 0;
            let mut backs: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut min_2: OnigLen = 0;
            let mut max_3: OnigLen = 0;
            let mut tmin: OnigLen = 0;
            let mut tmax: OnigLen = 0;
            let mut nodes: *mut *mut Node =
                if !((*(*env).scan_env).mem_nodes_dynamic as
                         *mut libc::c_void).is_null() {
                    (*(*env).scan_env).mem_nodes_dynamic
                } else { (*(*env).scan_env).mem_nodes_static.as_mut_ptr() };
            let mut br: *mut BRefNode = &mut (*node).u.bref;
            if (*br).state & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                set_mml(&mut (*opt).len, 0 as libc::c_int as OnigLen,
                        !(0 as libc::c_int as OnigLen));
            } else {
                backs =
                    if !((*br).back_dynamic as *mut libc::c_void).is_null() {
                        (*br).back_dynamic
                    } else { (*br).back_static.as_mut_ptr() };
                r =
                    get_min_len(*nodes.offset(*backs.offset(0 as libc::c_int
                                                                as isize) as
                                                  isize), &mut min_2,
                                (*env).scan_env);
                if !(r != 0 as libc::c_int) {
                    r =
                        get_max_len(*nodes.offset(*backs.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                                                      as isize), &mut max_3,
                                    (*env).scan_env);
                    if !(r != 0 as libc::c_int) {
                        i_1 = 1 as libc::c_int;
                        while i_1 < (*br).back_num {
                            r =
                                get_min_len(*nodes.offset(*backs.offset(i_1 as
                                                                            isize)
                                                              as isize),
                                            &mut tmin, (*env).scan_env);
                            if r != 0 as libc::c_int { break ; }
                            r =
                                get_max_len(*nodes.offset(*backs.offset(i_1 as
                                                                            isize)
                                                              as isize),
                                            &mut tmax, (*env).scan_env);
                            if r != 0 as libc::c_int { break ; }
                            if min_2 > tmin { min_2 = tmin }
                            if max_3 < tmax { max_3 = tmax }
                            i_1 += 1
                        }
                        if r == 0 as libc::c_int {
                            set_mml(&mut (*opt).len, min_2, max_3);
                        }
                    }
                }
            }
        }
        10 => {
            if (*node).u.call.state & (1 as libc::c_int) << 7 as libc::c_int
                   != 0 as libc::c_int {
                set_mml(&mut (*opt).len, 0 as libc::c_int as OnigLen,
                        !(0 as libc::c_int as OnigLen));
            } else {
                let mut save: OnigOptionType = (*env).options;
                (*env).options = (*(*node).u.call.target).u.enclose.option;
                r = optimize_node_left((*node).u.call.target, opt, env);
                (*env).options = save
            }
        }
        5 => {
            let mut i_2: libc::c_int = 0;
            let mut min_3: OnigLen = 0;
            let mut max_4: OnigLen = 0;
            let mut nopt_2: NodeOptInfo =
                NodeOptInfo{len: MinMaxLen{min: 0, max: 0,},
                            anc: OptAncInfo{left_anchor: 0, right_anchor: 0,},
                            exb:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            exm:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            expr:
                                OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                             anc:
                                                 OptAncInfo{left_anchor: 0,
                                                            right_anchor: 0,},
                                             reach_end: 0,
                                             ignore_case: 0,
                                             len: 0,
                                             s: [0; 24],},
                            map:
                                OptMapInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                           anc:
                                               OptAncInfo{left_anchor: 0,
                                                          right_anchor: 0,},
                                           value: 0,
                                           map: [0; 256],},};
            let mut qn: *mut QtfrNode = &mut (*node).u.qtfr;
            r = optimize_node_left((*qn).target, &mut nopt_2, env);
            if !(r != 0) {
                if (*qn).lower == 0 as libc::c_int &&
                       (*qn).upper == -(1 as libc::c_int) {
                    if (*env).mmd.max == 0 as libc::c_int as libc::c_uint &&
                           (*(*qn).target).u.base.type_0 == 3 as libc::c_int
                           && (*qn).greedy != 0 {
                        if (*env).options &
                               ((1 as libc::c_uint) << 1 as libc::c_int) <<
                                   1 as libc::c_int != 0 {
                            add_opt_anc_info(&mut (*opt).anc,
                                             (1 as libc::c_int) <<
                                                 15 as libc::c_int);
                        } else {
                            add_opt_anc_info(&mut (*opt).anc,
                                             (1 as libc::c_int) <<
                                                 14 as libc::c_int);
                        }
                    }
                } else if (*qn).lower > 0 as libc::c_int {
                    copy_node_opt_info(opt, &mut nopt_2);
                    if nopt_2.exb.len > 0 as libc::c_int {
                        if nopt_2.exb.reach_end != 0 {
                            i_2 = 2 as libc::c_int;
                            while i_2 <= (*qn).lower &&
                                      is_full_opt_exact_info(&mut (*opt).exb)
                                          == 0 {
                                concat_opt_exact_info(&mut (*opt).exb,
                                                      &mut nopt_2.exb,
                                                      (*env).enc);
                                i_2 += 1
                            }
                            if i_2 < (*qn).lower {
                                (*opt).exb.reach_end = 0 as libc::c_int
                            }
                        }
                    }
                    if (*qn).lower != (*qn).upper {
                        (*opt).exb.reach_end = 0 as libc::c_int;
                        (*opt).exm.reach_end = 0 as libc::c_int
                    }
                    if (*qn).lower > 1 as libc::c_int {
                        (*opt).exm.reach_end = 0 as libc::c_int
                    }
                }
                min_3 = distance_multiply(nopt_2.len.min, (*qn).lower);
                if (*qn).upper == -(1 as libc::c_int) {
                    max_4 =
                        if nopt_2.len.max > 0 as libc::c_int as libc::c_uint {
                            !(0 as libc::c_int as OnigLen)
                        } else { 0 as libc::c_int as libc::c_uint }
                } else {
                    max_4 = distance_multiply(nopt_2.len.max, (*qn).upper)
                }
                set_mml(&mut (*opt).len, min_3, max_4);
            }
        }
        6 => {
            let mut en: *mut EncloseNode = &mut (*node).u.enclose;
            match (*en).type_0 {
                2 => {
                    let mut save_0: OnigOptionType = (*env).options;
                    (*env).options = (*en).option;
                    r = optimize_node_left((*en).target, opt, env);
                    (*env).options = save_0
                }
                1 => {
                    (*en).opt_count += 1;
                    if (*en).opt_count > 5 as libc::c_int {
                        let mut min_4: OnigLen = 0;
                        let mut max_5: OnigLen = 0;
                        min_4 = 0 as libc::c_int as OnigLen;
                        max_5 = !(0 as libc::c_int as OnigLen);
                        if (*en).state &
                               (1 as libc::c_int) << 0 as libc::c_int !=
                               0 as libc::c_int {
                            min_4 = (*en).min_len
                        }
                        if (*en).state &
                               (1 as libc::c_int) << 1 as libc::c_int !=
                               0 as libc::c_int {
                            max_5 = (*en).max_len
                        }
                        set_mml(&mut (*opt).len, min_4, max_5);
                    } else {
                        r = optimize_node_left((*en).target, opt, env);
                        if is_set_opt_anc_info(&mut (*opt).anc,
                                               (1 as libc::c_int) <<
                                                   14 as libc::c_int |
                                                   (1 as libc::c_int) <<
                                                       15 as libc::c_int) != 0
                           {
                            if if (*en).regnum <
                                      (::std::mem::size_of::<BitStatusType>()
                                           as
                                           libc::c_ulong).wrapping_mul(8 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                          as libc::c_int {
                                   ((*(*env).scan_env).backrefed_mem) &
                                       ((1 as libc::c_int) << (*en).regnum) as
                                           libc::c_uint
                               } else {
                                   ((*(*env).scan_env).backrefed_mem) &
                                       1 as libc::c_int as libc::c_uint
                               } != 0 {
                                remove_opt_anc_info(&mut (*opt).anc,
                                                    (1 as libc::c_int) <<
                                                        14 as libc::c_int |
                                                        (1 as libc::c_int) <<
                                                            15 as
                                                                libc::c_int);
                            }
                        }
                    }
                }
                4 => { r = optimize_node_left((*en).target, opt, env) }
                _ => { }
            }
        }
        _ => { r = -(6 as libc::c_int) }
    }
    return r;
}
unsafe extern "C" fn set_optimize_exact_info(mut reg: *mut regex_t,
                                             mut e: *mut OptExactInfo)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*e).len == 0 as libc::c_int { return 0 as libc::c_int }
    if (*e).ignore_case != 0 {
        (*reg).exact = malloc((*e).len as libc::c_ulong) as *mut OnigUChar;
        if ((*reg).exact as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        memcpy((*reg).exact as *mut libc::c_void,
               (*e).s.as_mut_ptr() as *const libc::c_void,
               (*e).len as libc::c_ulong);
        (*reg).exact_end = (*reg).exact.offset((*e).len as isize);
        (*reg).optimize = 4 as libc::c_int
    } else {
        let mut allow_reverse: libc::c_int = 0;
        (*reg).exact =
            str_dup((*e).s.as_mut_ptr(),
                    (*e).s.as_mut_ptr().offset((*e).len as isize));
        if ((*reg).exact as *mut libc::c_void).is_null() {
            return -(5 as libc::c_int)
        }
        (*reg).exact_end = (*reg).exact.offset((*e).len as isize);
        allow_reverse =
            (*(*reg).enc).is_allowed_reverse_match.expect("non-null function pointer")((*reg).exact,
                                                                                       (*reg).exact_end);
        if (*e).len >= 3 as libc::c_int ||
               (*e).len >= 2 as libc::c_int && allow_reverse != 0 {
            r =
                set_bm_skip((*reg).exact, (*reg).exact_end, (*reg).enc,
                            (*reg).map.as_mut_ptr(), &mut (*reg).int_map);
            if r != 0 { return r }
            (*reg).optimize =
                if allow_reverse != 0 as libc::c_int {
                    2 as libc::c_int
                } else { 3 as libc::c_int }
        } else { (*reg).optimize = 1 as libc::c_int }
    }
    (*reg).dmin = (*e).mmd.min;
    (*reg).dmax = (*e).mmd.max;
    if (*reg).dmin != !(0 as libc::c_int as OnigLen) {
        (*reg).threshold_len =
            ((*reg).dmin as libc::c_long +
                 (*reg).exact_end.wrapping_offset_from((*reg).exact) as
                     libc::c_long) as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_optimize_map_info(mut reg: *mut regex_t,
                                           mut m: *mut OptMapInfo) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*reg).map[i as usize] = (*m).map[i as usize];
        i += 1
    }
    (*reg).optimize = 5 as libc::c_int;
    (*reg).dmin = (*m).mmd.min;
    (*reg).dmax = (*m).mmd.max;
    if (*reg).dmin != !(0 as libc::c_int as OnigLen) {
        (*reg).threshold_len =
            (*reg).dmin.wrapping_add(1 as libc::c_int as libc::c_uint) as
                libc::c_int
    };
}
unsafe extern "C" fn set_sub_anchor(mut reg: *mut regex_t,
                                    mut anc: *mut OptAncInfo) {
    (*reg).sub_anchor |=
        (*anc).left_anchor & (1 as libc::c_int) << 1 as libc::c_int;
    (*reg).sub_anchor |=
        (*anc).right_anchor & (1 as libc::c_int) << 5 as libc::c_int;
}
unsafe extern "C" fn set_optimize_info_from_tree(mut node: *mut Node,
                                                 mut reg: *mut regex_t,
                                                 mut scan_env: *mut ScanEnv)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut opt: NodeOptInfo =
        NodeOptInfo{len: MinMaxLen{min: 0, max: 0,},
                    anc: OptAncInfo{left_anchor: 0, right_anchor: 0,},
                    exb:
                        OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                     anc:
                                         OptAncInfo{left_anchor: 0,
                                                    right_anchor: 0,},
                                     reach_end: 0,
                                     ignore_case: 0,
                                     len: 0,
                                     s: [0; 24],},
                    exm:
                        OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                     anc:
                                         OptAncInfo{left_anchor: 0,
                                                    right_anchor: 0,},
                                     reach_end: 0,
                                     ignore_case: 0,
                                     len: 0,
                                     s: [0; 24],},
                    expr:
                        OptExactInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                     anc:
                                         OptAncInfo{left_anchor: 0,
                                                    right_anchor: 0,},
                                     reach_end: 0,
                                     ignore_case: 0,
                                     len: 0,
                                     s: [0; 24],},
                    map:
                        OptMapInfo{mmd: MinMaxLen{min: 0, max: 0,},
                                   anc:
                                       OptAncInfo{left_anchor: 0,
                                                  right_anchor: 0,},
                                   value: 0,
                                   map: [0; 256],},};
    let mut env: OptEnv =
        OptEnv{mmd: MinMaxLen{min: 0, max: 0,},
               enc: 0 as *mut OnigEncodingType,
               options: 0,
               case_fold_flag: 0,
               scan_env: 0 as *mut ScanEnv,};
    env.enc = (*reg).enc;
    env.options = (*reg).options;
    env.case_fold_flag = (*reg).case_fold_flag;
    env.scan_env = scan_env;
    clear_mml(&mut env.mmd);
    r = optimize_node_left(node, &mut opt, &mut env);
    if r != 0 { return r }
    (*reg).anchor =
        opt.anc.left_anchor &
            ((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int |
                 (1 as libc::c_int) << 14 as libc::c_int |
                 (1 as libc::c_int) << 15 as libc::c_int |
                 (1 as libc::c_int) << 12 as libc::c_int);
    if opt.anc.left_anchor &
           ((1 as libc::c_int) << 12 as libc::c_int |
                (1 as libc::c_int) << 11 as libc::c_int) != 0 as libc::c_int {
        (*reg).anchor &= !((1 as libc::c_int) << 15 as libc::c_int)
    }
    (*reg).anchor |=
        opt.anc.right_anchor &
            ((1 as libc::c_int) << 3 as libc::c_int |
                 (1 as libc::c_int) << 4 as libc::c_int |
                 (1 as libc::c_int) << 11 as libc::c_int);
    if (*reg).anchor &
           ((1 as libc::c_int) << 3 as libc::c_int |
                (1 as libc::c_int) << 4 as libc::c_int) != 0 {
        (*reg).anchor_dmin = opt.len.min;
        (*reg).anchor_dmax = opt.len.max
    }
    let mut current_block_25: u64;
    if opt.exb.len > 0 as libc::c_int || opt.exm.len > 0 as libc::c_int {
        select_opt_exact_info((*reg).enc, &mut opt.exb, &mut opt.exm);
        if opt.map.value > 0 as libc::c_int &&
               comp_opt_exact_or_map_info(&mut opt.exb, &mut opt.map) >
                   0 as libc::c_int {
            current_block_25 = 538028991732400653;
        } else {
            r = set_optimize_exact_info(reg, &mut opt.exb);
            set_sub_anchor(reg, &mut opt.exb.anc);
            current_block_25 = 14359455889292382949;
        }
    } else if opt.map.value > 0 as libc::c_int {
        current_block_25 = 538028991732400653;
    } else {
        (*reg).sub_anchor |=
            opt.anc.left_anchor & (1 as libc::c_int) << 1 as libc::c_int;
        if opt.len.max == 0 as libc::c_int as libc::c_uint {
            (*reg).sub_anchor |=
                opt.anc.right_anchor & (1 as libc::c_int) << 5 as libc::c_int
        }
        current_block_25 = 14359455889292382949;
    }
    match current_block_25 {
        538028991732400653 => {
            set_optimize_map_info(reg, &mut opt.map);
            set_sub_anchor(reg, &mut opt.map.anc);
        }
        _ => { }
    }
    return r;
}
unsafe extern "C" fn clear_optimize_info(mut reg: *mut regex_t) {
    (*reg).optimize = 0 as libc::c_int;
    (*reg).anchor = 0 as libc::c_int;
    (*reg).anchor_dmin = 0 as libc::c_int as OnigLen;
    (*reg).anchor_dmax = 0 as libc::c_int as OnigLen;
    (*reg).sub_anchor = 0 as libc::c_int;
    (*reg).exact_end = 0 as *mut libc::c_void as *mut OnigUChar;
    (*reg).threshold_len = 0 as libc::c_int;
    if !((*reg).exact as *mut libc::c_void).is_null() {
        free((*reg).exact as *mut libc::c_void);
        (*reg).exact = 0 as *mut libc::c_void as *mut OnigUChar
    };
}
/* ONIG_DEBUG */
#[no_mangle]
pub unsafe extern "C" fn onig_free_body(mut reg: *mut regex_t) {
    if !(reg as *mut libc::c_void).is_null() {
        if !((*reg).p as *mut libc::c_void).is_null() {
            free((*reg).p as *mut libc::c_void);
        }
        if !((*reg).exact as *mut libc::c_void).is_null() {
            free((*reg).exact as *mut libc::c_void);
        }
        if !((*reg).int_map as *mut libc::c_void).is_null() {
            free((*reg).int_map as *mut libc::c_void);
        }
        if !((*reg).int_map_backward as *mut libc::c_void).is_null() {
            free((*reg).int_map_backward as *mut libc::c_void);
        }
        if !((*reg).repeat_range as *mut libc::c_void).is_null() {
            free((*reg).repeat_range as *mut libc::c_void);
        }
        if !((*reg).chain as *mut libc::c_void).is_null() {
            onig_free((*reg).chain);
        }
        onig_names_free(reg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn onig_free(mut reg: *mut regex_t) {
    if !(reg as *mut libc::c_void).is_null() {
        onig_free_body(reg);
        free(reg as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn onig_transfer(mut to: *mut regex_t,
                                       mut from: *mut regex_t) {
    onig_free_body(to);
    memcpy(to as *mut libc::c_void, from as *const libc::c_void,
           ::std::mem::size_of::<regex_t>() as libc::c_ulong);
    free(from as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn onig_compile(mut reg: *mut regex_t,
                                      mut pattern: *const OnigUChar,
                                      mut pattern_end: *const OnigUChar,
                                      mut einfo: *mut OnigErrorInfo)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut init_size: libc::c_int = 0;
    let mut root: *mut Node = 0 as *mut Node;
    let mut scan_env: ScanEnv =
        ScanEnv{option: 0,
                case_fold_flag: 0,
                enc: 0 as *mut OnigEncodingType,
                syntax: 0 as *mut OnigSyntaxType,
                capture_history: 0,
                bt_mem_start: 0,
                bt_mem_end: 0,
                backrefed_mem: 0,
                pattern: 0 as *mut OnigUChar,
                pattern_end: 0 as *mut OnigUChar,
                error: 0 as *mut OnigUChar,
                error_end: 0 as *mut OnigUChar,
                reg: 0 as *mut regex_t,
                num_call: 0,
                unset_addr_list: 0 as *mut UnsetAddrList,
                num_mem: 0,
                num_named: 0,
                mem_alloc: 0,
                mem_nodes_static: [0 as *mut Node; 8],
                mem_nodes_dynamic: 0 as *mut *mut Node,
                parse_depth: 0,};
    let mut uslist: UnsetAddrList =
        UnsetAddrList{num: 0, alloc: 0, us: 0 as *mut UnsetAddr,};
    root = 0 as *mut Node;
    if !(einfo as *mut libc::c_void).is_null() {
        (*einfo).par = 0 as *mut libc::c_void as *mut OnigUChar
    }
    if (*reg).alloc == 0 as libc::c_int as libc::c_uint {
        init_size =
            (pattern_end.wrapping_offset_from(pattern) as libc::c_long *
                 2 as libc::c_int as libc::c_long) as libc::c_int;
        if init_size <= 0 as libc::c_int { init_size = 20 as libc::c_int }
        r = onig_bbuf_init(reg as *mut BBuf, init_size);
        if r != 0 as libc::c_int {
            current_block = 11685150444691505124;
        } else { current_block = 12209867499936983673; }
    } else {
        (*reg).used = 0 as libc::c_int as libc::c_uint;
        current_block = 12209867499936983673;
    }
    match current_block {
        12209867499936983673 => {
            (*reg).num_mem = 0 as libc::c_int;
            (*reg).num_repeat = 0 as libc::c_int;
            (*reg).num_null_check = 0 as libc::c_int;
            (*reg).repeat_range_alloc = 0 as libc::c_int;
            (*reg).repeat_range =
                0 as *mut libc::c_void as *mut OnigRepeatRange;
            r =
                onig_parse_make_tree(&mut root, pattern, pattern_end, reg,
                                     &mut scan_env);
            if r != 0 as libc::c_int {
                current_block = 9116806350784175767;
            } else {
                /* mixed use named group and no-named group */
                if scan_env.num_named > 0 as libc::c_int &&
                       (*scan_env.syntax).behavior &
                           (1 as libc::c_uint) << 7 as libc::c_int !=
                           0 as libc::c_int as libc::c_uint &&
                       (*reg).options &
                           ((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                                     1 as libc::c_int) << 1 as libc::c_int) <<
                                   1 as libc::c_int) << 1 as libc::c_int) <<
                                 1 as libc::c_int) << 1 as libc::c_int) <<
                               1 as libc::c_int == 0 {
                    if scan_env.num_named != scan_env.num_mem {
                        r =
                            disable_noname_group_capture(&mut root, reg,
                                                         &mut scan_env)
                    } else { r = numbered_ref_check(root) }
                    if r != 0 as libc::c_int {
                        current_block = 9116806350784175767;
                    } else { current_block = 5634871135123216486; }
                } else { current_block = 5634871135123216486; }
                match current_block {
                    9116806350784175767 => { }
                    _ => {
                        if scan_env.num_call > 0 as libc::c_int {
                            r =
                                unset_addr_list_init(&mut uslist,
                                                     scan_env.num_call);
                            if r != 0 as libc::c_int {
                                current_block = 9116806350784175767;
                            } else {
                                scan_env.unset_addr_list = &mut uslist;
                                r = setup_subexp_call(root, &mut scan_env);
                                if r != 0 as libc::c_int {
                                    current_block = 1146666095753440282;
                                } else {
                                    r =
                                        subexp_recursive_check_trav(root,
                                                                    &mut scan_env);
                                    if r < 0 as libc::c_int {
                                        current_block = 1146666095753440282;
                                    } else {
                                        r =
                                            subexp_inf_recursive_check_trav(root,
                                                                            &mut scan_env);
                                        if r != 0 as libc::c_int {
                                            current_block =
                                                1146666095753440282;
                                        } else {
                                            (*reg).num_call =
                                                scan_env.num_call;
                                            current_block =
                                                2873832966593178012;
                                        }
                                    }
                                }
                            }
                        } else {
                            (*reg).num_call = 0 as libc::c_int;
                            current_block = 2873832966593178012;
                        }
                        match current_block {
                            9116806350784175767 => { }
                            _ => {
                                match current_block {
                                    2873832966593178012 => {
                                        r =
                                            setup_tree(root, reg,
                                                       0 as libc::c_int,
                                                       &mut scan_env);
                                        if r != 0 as libc::c_int {
                                            current_block =
                                                1146666095753440282;
                                        } else {
                                            (*reg).capture_history =
                                                scan_env.capture_history;
                                            (*reg).bt_mem_start =
                                                scan_env.bt_mem_start;
                                            (*reg).bt_mem_start |=
                                                (*reg).capture_history;
                                            if (*reg).options &
                                                   (((((1 as libc::c_uint) <<
                                                           1 as libc::c_int)
                                                          << 1 as libc::c_int)
                                                         << 1 as libc::c_int)
                                                        << 1 as libc::c_int |
                                                        (((((1 as
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
                                                             1 as libc::c_int)
                                                            <<
                                                            1 as libc::c_int)
                                                   != 0 {
                                                (*reg).bt_mem_end =
                                                    !(0 as libc::c_int as
                                                          BitStatusType)
                                            } else {
                                                (*reg).bt_mem_end =
                                                    scan_env.bt_mem_end;
                                                (*reg).bt_mem_end |=
                                                    (*reg).capture_history
                                            }
                                            clear_optimize_info(reg);
                                            r =
                                                set_optimize_info_from_tree(root,
                                                                            reg,
                                                                            &mut scan_env);
                                            if r != 0 as libc::c_int {
                                                current_block =
                                                    1146666095753440282;
                                            } else {
                                                if !(scan_env.mem_nodes_dynamic
                                                         as
                                                         *mut libc::c_void).is_null()
                                                   {
                                                    free(scan_env.mem_nodes_dynamic
                                                             as
                                                             *mut libc::c_void);
                                                    scan_env.mem_nodes_dynamic
                                                        =
                                                        0 as *mut libc::c_void
                                                            as *mut *mut Node
                                                }
                                                r = compile_tree(root, reg);
                                                if r == 0 as libc::c_int {
                                                    r =
                                                        add_opcode(reg,
                                                                   OP_END as
                                                                       libc::c_int);
                                                    if scan_env.num_call >
                                                           0 as libc::c_int {
                                                        r =
                                                            unset_addr_list_fix(&mut uslist,
                                                                                reg);
                                                        unset_addr_list_end(&mut uslist);
                                                        if r != 0 {
                                                            current_block =
                                                                9116806350784175767;
                                                        } else {
                                                            current_block =
                                                                5235537862154438448;
                                                        }
                                                    } else {
                                                        current_block =
                                                            5235537862154438448;
                                                    }
                                                    match current_block {
                                                        9116806350784175767 =>
                                                        {
                                                        }
                                                        _ => {
                                                            if (*reg).num_repeat
                                                                   !=
                                                                   0 as
                                                                       libc::c_int
                                                                   ||
                                                                   (*reg).bt_mem_end
                                                                       !=
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                               {
                                                                (*reg).stack_pop_level
                                                                    =
                                                                    2 as
                                                                        libc::c_int
                                                            } else if (*reg).bt_mem_start
                                                                          !=
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint
                                                             {
                                                                (*reg).stack_pop_level
                                                                    =
                                                                    1 as
                                                                        libc::c_int
                                                            } else {
                                                                (*reg).stack_pop_level
                                                                    =
                                                                    0 as
                                                                        libc::c_int
                                                            }
                                                            current_block =
                                                                5372832139739605200;
                                                        }
                                                    }
                                                } else {
                                                    if scan_env.num_call >
                                                           0 as libc::c_int {
                                                        unset_addr_list_end(&mut uslist);
                                                    }
                                                    current_block =
                                                        5372832139739605200;
                                                }
                                                match current_block {
                                                    9116806350784175767 => { }
                                                    _ => {
                                                        onig_node_free(root);
                                                        current_block =
                                                            11685150444691505124;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => { }
                                }
                                match current_block {
                                    11685150444691505124 => { }
                                    9116806350784175767 => { }
                                    _ => {
                                        if scan_env.num_call >
                                               0 as libc::c_int {
                                            unset_addr_list_end(&mut uslist);
                                        }
                                        current_block = 9116806350784175767;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                11685150444691505124 => { }
                _ => {
                    if !(scan_env.error as *mut libc::c_void).is_null() {
                        if !(einfo as *mut libc::c_void).is_null() {
                            (*einfo).enc = scan_env.enc;
                            (*einfo).par = scan_env.error;
                            (*einfo).par_end = scan_env.error_end
                        }
                    }
                    onig_node_free(root);
                    if !(scan_env.mem_nodes_dynamic as
                             *mut libc::c_void).is_null() {
                        free(scan_env.mem_nodes_dynamic as *mut libc::c_void);
                    }
                    return r
                }
            }
        }
        _ => { }
    }
    return r;
}
static mut onig_inited: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn onig_reg_init(mut reg: *mut regex_t,
                                       mut option: OnigOptionType,
                                       mut case_fold_flag: OnigCaseFoldType,
                                       mut enc: OnigEncoding,
                                       mut syntax: *mut OnigSyntaxType)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    memset(reg as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<regex_t>() as libc::c_ulong);
    if onig_inited == 0 as libc::c_int {
        r = onig_initialize(0 as *mut OnigEncoding, 0 as libc::c_int);
        if r != 0 as libc::c_int { return -(23 as libc::c_int) }
        r = onig_initialize_encoding(enc);
        if r != 0 as libc::c_int { return -(23 as libc::c_int) }
        onig_warning(b"You didn\'t call onig_initialize() explicitly\x00" as
                         *const u8 as *const libc::c_char);
    }
    if (reg as *mut libc::c_void).is_null() { return -(30 as libc::c_int) }
    if enc.is_null() { return -(21 as libc::c_int) }
    if option &
           ((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int |
                ((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                          1 as libc::c_int) << 1 as libc::c_int) <<
                        1 as libc::c_int) << 1 as libc::c_int) <<
                      1 as libc::c_int) << 1 as libc::c_int) <<
                    1 as libc::c_int) ==
           (((((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                   << 1 as libc::c_int) << 1 as libc::c_int) <<
                 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int |
               ((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                         1 as libc::c_int) << 1 as libc::c_int) <<
                       1 as libc::c_int) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int {
        return -(403 as libc::c_int)
    }
    if option &
           ((((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                  << 1 as libc::c_int) << 1 as libc::c_int) <<
                1 as libc::c_int) << 1 as libc::c_int !=
           0 as libc::c_int as libc::c_uint {
        option |= (*syntax).options;
        option &=
            !((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                  << 1 as libc::c_int)
    } else { option |= (*syntax).options }
    (*reg).enc = enc;
    (*reg).options = option;
    (*reg).syntax = syntax;
    (*reg).optimize = 0 as libc::c_int;
    (*reg).exact = 0 as *mut libc::c_void as *mut OnigUChar;
    (*reg).int_map = 0 as *mut libc::c_void as *mut libc::c_int;
    (*reg).int_map_backward = 0 as *mut libc::c_void as *mut libc::c_int;
    (*reg).chain = 0 as *mut libc::c_void as *mut regex_t;
    (*reg).p = 0 as *mut libc::c_void as *mut OnigUChar;
    (*reg).alloc = 0 as libc::c_int as libc::c_uint;
    (*reg).used = 0 as libc::c_int as libc::c_uint;
    (*reg).name_table = 0 as *mut libc::c_void;
    (*reg).case_fold_flag = case_fold_flag;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_new_without_alloc(mut reg: *mut regex_t,
                                                mut pattern: *const OnigUChar,
                                                mut pattern_end:
                                                    *const OnigUChar,
                                                mut option: OnigOptionType,
                                                mut enc: OnigEncoding,
                                                mut syntax:
                                                    *mut OnigSyntaxType,
                                                mut einfo: *mut OnigErrorInfo)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = onig_reg_init(reg, option, OnigDefaultCaseFoldFlag, enc, syntax);
    if r != 0 { return r }
    r = onig_compile(reg, pattern, pattern_end, einfo);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn onig_new(mut reg: *mut *mut regex_t,
                                  mut pattern: *const OnigUChar,
                                  mut pattern_end: *const OnigUChar,
                                  mut option: OnigOptionType,
                                  mut enc: OnigEncoding,
                                  mut syntax: *mut OnigSyntaxType,
                                  mut einfo: *mut OnigErrorInfo)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    *reg =
        malloc(::std::mem::size_of::<regex_t>() as libc::c_ulong) as
            *mut regex_t;
    if (*reg as *mut libc::c_void).is_null() { return -(5 as libc::c_int) }
    r = onig_reg_init(*reg, option, OnigDefaultCaseFoldFlag, enc, syntax);
    if r != 0 {
        current_block = 15658198057350242478;
    } else {
        r = onig_compile(*reg, pattern, pattern_end, einfo);
        if r != 0 {
            current_block = 15658198057350242478;
        } else { current_block = 3276175668257526147; }
    }
    match current_block {
        15658198057350242478 => { onig_free(*reg); *reg = 0 as *mut regex_t }
        _ => { }
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn onig_initialize(mut encodings: *mut OnigEncoding,
                                         mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if onig_inited != 0 as libc::c_int { return 0 as libc::c_int }
    onigenc_init();
    onig_inited = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let mut enc: OnigEncoding = *encodings.offset(i as isize);
        r = onig_initialize_encoding(enc);
        if r != 0 as libc::c_int { return r }
        i += 1
    }
    return 0 as libc::c_int;
}
static mut EndCallTop: *mut OnigEndCallListItemType =
    0 as *const OnigEndCallListItemType as *mut OnigEndCallListItemType;
#[no_mangle]
pub unsafe extern "C" fn onig_add_end_call(mut func:
                                               Option<unsafe extern "C" fn()
                                                          -> ()>) {
    let mut item: *mut OnigEndCallListItemType =
        0 as *mut OnigEndCallListItemType;
    item =
        malloc(::std::mem::size_of::<OnigEndCallListItemType>() as
                   libc::c_ulong) as *mut OnigEndCallListItemType;
    if item.is_null() { return }
    (*item).next = EndCallTop;
    (*item).func = func;
    EndCallTop = item;
}
unsafe extern "C" fn exec_end_call_list() {
    let mut prev: *mut OnigEndCallListItemType =
        0 as *mut OnigEndCallListItemType;
    let mut func: Option<unsafe extern "C" fn() -> ()> = None;
    while !EndCallTop.is_null() {
        func = (*EndCallTop).func;
        Some(func.expect("non-null function pointer")).expect("non-null function pointer")();
        prev = EndCallTop;
        EndCallTop = (*EndCallTop).next;
        free(prev as *mut libc::c_void);
    };
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
pub unsafe extern "C" fn onig_end() -> libc::c_int {
    exec_end_call_list();
    onig_inited = 0 as libc::c_int;
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
/* ascii */
/* defined in regexec.c, but used in enc/xxx.c */
#[no_mangle]
pub unsafe extern "C" fn onig_is_in_code_range(mut p: *const OnigUChar,
                                               mut code: OnigCodePoint)
 -> libc::c_int {
    let mut n: OnigCodePoint = 0;
    let mut data: *mut OnigCodePoint = 0 as *mut OnigCodePoint;
    let mut low: OnigCodePoint = 0;
    let mut high: OnigCodePoint = 0;
    let mut x: OnigCodePoint = 0;
    n = *(p as *mut OnigCodePoint);
    data = p as *mut OnigCodePoint;
    data = data.offset(1);
    low = 0 as libc::c_int as OnigCodePoint;
    high = n;
    while low < high {
        x = low.wrapping_add(high) >> 1 as libc::c_int;
        if code >
               *data.offset(x.wrapping_mul(2 as libc::c_int as
                                               libc::c_uint).wrapping_add(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                as isize) {
            low = x.wrapping_add(1 as libc::c_int as libc::c_uint)
        } else { high = x }
    }
    return if low < n &&
                  code >=
                      *data.offset(low.wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint) as
                                       isize) {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn onig_is_code_in_cc_len(mut elen: libc::c_int,
                                                mut code: OnigCodePoint,
                                                mut cc: *mut CClassNode)
 -> libc::c_int {
    let mut found: libc::c_int = 0;
    if elen > 1 as libc::c_int ||
           code >= ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint {
        if ((*cc).mbuf as *mut libc::c_void).is_null() {
            found = 0 as libc::c_int
        } else {
            found =
                if onig_is_in_code_range((*(*cc).mbuf).p, code) !=
                       0 as libc::c_int {
                    1 as libc::c_int
                } else { 0 as libc::c_int }
        }
    } else {
        found =
            if (*cc).bs[(code as
                             libc::c_ulong).wrapping_div((::std::mem::size_of::<Bits>()
                                                              as
                                                              libc::c_ulong).wrapping_mul(8
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong))
                            as usize] &
                   ((1 as libc::c_int) <<
                        (code as
                             libc::c_ulong).wrapping_rem((::std::mem::size_of::<Bits>()
                                                              as
                                                              libc::c_ulong).wrapping_mul(8
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)))
                       as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else { 1 as libc::c_int }
    }
    if (*cc).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
           != 0 as libc::c_int as libc::c_uint {
        return (found == 0) as libc::c_int
    } else { return found };
}
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
pub unsafe extern "C" fn onig_is_code_in_cc(mut enc: OnigEncoding,
                                            mut code: OnigCodePoint,
                                            mut cc: *mut CClassNode)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    if (*enc).min_enc_len > 1 as libc::c_int {
        len = 2 as libc::c_int
    } else {
        len = (*enc).code_to_mbclen.expect("non-null function pointer")(code)
    }
    return onig_is_code_in_cc_len(len, code, cc);
}
/* ONIG_DEBUG */
