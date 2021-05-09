#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut OnigEncodingASCII: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF8: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF16_BE: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingUTF16_LE: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingEUC_JP: OnigEncodingType;
    #[no_mangle]
    static mut OnigEncodingSJIS: OnigEncodingType;
    #[no_mangle]
    fn onig_initialize_encoding(enc: OnigEncoding) -> libc::c_int;
    #[no_mangle]
    fn onigenc_set_default_encoding(enc: OnigEncoding) -> libc::c_int;
    #[no_mangle]
    fn onigenc_str_bytelen_null(enc: OnigEncoding, p: *const OnigUChar)
     -> libc::c_int;
    #[no_mangle]
    static mut OnigSyntaxPosixBasic: OnigSyntaxType;
    #[no_mangle]
    static mut OnigDefaultSyntax: *mut OnigSyntaxType;
    #[no_mangle]
    static mut OnigEncDefaultCharEncoding: OnigEncoding;
    #[no_mangle]
    fn onig_new(_: *mut OnigRegex, pattern: *const OnigUChar,
                pattern_end: *const OnigUChar, option: OnigOptionType,
                enc: OnigEncoding, syntax: *mut OnigSyntaxType,
                einfo: *mut OnigErrorInfo) -> libc::c_int;
    #[no_mangle]
    fn onig_free(_: OnigRegex);
    #[no_mangle]
    fn onig_search(_: OnigRegex, str: *const OnigUChar, end: *const OnigUChar,
                   start: *const OnigUChar, range: *const OnigUChar,
                   region: *mut OnigRegion, option: OnigOptionType)
     -> libc::c_int;
    #[no_mangle]
    fn onig_name_to_group_numbers(reg: OnigRegex, name: *const OnigUChar,
                                  name_end: *const OnigUChar,
                                  nums: *mut *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn onig_initialize(encodings: *mut OnigEncoding, n: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn onig_foreach_name(reg: OnigRegex,
                         func:
                             Option<unsafe extern "C" fn(_: *const OnigUChar,
                                                         _: *const OnigUChar,
                                                         _: libc::c_int,
                                                         _: *mut libc::c_int,
                                                         _: OnigRegex,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int>,
                         arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn onig_number_of_names(reg: OnigRegex) -> libc::c_int;
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
pub type onig_regex_t = OnigRegexType;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_t {
    pub onig: *mut libc::c_void,
    pub re_nsub: size_t,
    pub comp_options: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct O2PERR {
    pub onig_err: libc::c_int,
    pub posix_err: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i_wrap {
    pub func: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                          _: *const libc::c_uchar,
                                          _: libc::c_int, _: *mut libc::c_int,
                                          _: *mut regex_t,
                                          _: *mut libc::c_void)
                         -> libc::c_int>,
    pub reg: *mut regex_t,
    pub arg: *mut libc::c_void,
}
unsafe extern "C" fn onig2posix_error_code(mut code: libc::c_int)
 -> libc::c_int {
    static mut o2p: [O2PERR; 60] =
        [{
             let mut init =
                 O2PERR{onig_err: -(1 as libc::c_int),
                        posix_err: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(2 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(5 as libc::c_int),
                        posix_err: 12 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(15 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(6 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(11 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(12 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(13 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(14 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(21 as libc::c_int),
                        posix_err: 16 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(22 as libc::c_int),
                        posix_err: 16 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(23 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(30 as libc::c_int),
                        posix_err: 16 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(100 as libc::c_int),
                        posix_err: 9 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(101 as libc::c_int),
                        posix_err: 7 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(102 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(103 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(104 as libc::c_int),
                        posix_err: 5 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(105 as libc::c_int),
                        posix_err: 5 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(106 as libc::c_int),
                        posix_err: 5 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(108 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(109 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(110 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(111 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(112 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(113 as libc::c_int),
                        posix_err: 13 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(114 as libc::c_int),
                        posix_err: 13 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(115 as libc::c_int),
                        posix_err: 13 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(116 as libc::c_int),
                        posix_err: 8 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(117 as libc::c_int),
                        posix_err: 8 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(118 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(119 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(121 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(122 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(123 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(200 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(201 as libc::c_int),
                        posix_err: 10 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(202 as libc::c_int),
                        posix_err: 10 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(203 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(204 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(205 as libc::c_int),
                        posix_err: 4 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(206 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(207 as libc::c_int),
                        posix_err: 6 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(208 as libc::c_int),
                        posix_err: 6 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(209 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(401 as libc::c_int),
                        posix_err: 15 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(212 as libc::c_int),
                        posix_err: 15 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(400 as libc::c_int),
                        posix_err: 15 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(214 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(215 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(216 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(217 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(218 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(219 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(220 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(221 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(222 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(223 as libc::c_int),
                        posix_err: 2 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(402 as libc::c_int),
                        posix_err: 16 as libc::c_int,};
             init
         },
         {
             let mut init =
                 O2PERR{onig_err: -(500 as libc::c_int),
                        posix_err: 14 as libc::c_int,};
             init
         }];
    let mut i: libc::c_int = 0;
    if code >= 0 as libc::c_int { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[O2PERR; 60]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<O2PERR>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        if code == o2p[i as usize].onig_err {
            return o2p[i as usize].posix_err
        }
        i += 1
    }
    return 14 as libc::c_int;
    /* but, unknown error code */
}
/* ONIGURUMA_H */
#[no_mangle]
pub unsafe extern "C" fn regcomp(mut reg: *mut regex_t,
                                 mut pattern: *const libc::c_char,
                                 mut posix_options: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0; /* Match */
    let mut len: libc::c_int = 0;
    let mut syntax: *mut OnigSyntaxType = OnigDefaultSyntax;
    let mut options: OnigOptionType = 0;
    if posix_options & (1 as libc::c_int) << 4 as libc::c_int ==
           0 as libc::c_int {
        syntax = &mut OnigSyntaxPosixBasic
    }
    options = (*syntax).options;
    if posix_options & (1 as libc::c_int) << 0 as libc::c_int !=
           0 as libc::c_int {
        options |= 1 as libc::c_uint
    }
    if posix_options & (1 as libc::c_int) << 1 as libc::c_int !=
           0 as libc::c_int {
        options |=
            ((((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                   << 1 as libc::c_int) << 1 as libc::c_int) <<
                 1 as libc::c_int) << 1 as libc::c_int;
        options &=
            !((((1 as libc::c_uint) << 1 as libc::c_int) << 1 as libc::c_int)
                  << 1 as libc::c_int)
    }
    (*reg).comp_options = posix_options;
    if (*OnigEncDefaultCharEncoding).min_enc_len == 1 as libc::c_int {
        let mut tmps: *mut OnigUChar = pattern as *mut OnigUChar;
        while *tmps as libc::c_int != 0 as libc::c_int {
            tmps = tmps.offset(1)
        }
        len =
            tmps.wrapping_offset_from(pattern as *mut OnigUChar) as
                libc::c_long as libc::c_int
    } else {
        len =
            onigenc_str_bytelen_null(OnigEncDefaultCharEncoding,
                                     pattern as *mut OnigUChar)
    }
    r =
        onig_new(&mut (*reg).onig as *mut *mut libc::c_void as
                     *mut *mut onig_regex_t, pattern as *mut OnigUChar,
                 pattern.offset(len as isize) as *mut OnigUChar, options,
                 OnigEncDefaultCharEncoding, syntax,
                 0 as *mut libc::c_void as *mut OnigErrorInfo);
    if r != 0 as libc::c_int { return onig2posix_error_code(r) }
    (*reg).re_nsub = (*((*reg).onig as *mut onig_regex_t)).num_mem as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn regexec(mut reg: *mut regex_t,
                                 mut str: *const libc::c_char,
                                 mut nmatch: size_t,
                                 mut pmatch: *mut regmatch_t,
                                 mut posix_options: libc::c_int)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut end: *mut OnigUChar = 0 as *mut OnigUChar;
    let mut pm: *mut regmatch_t = 0 as *mut regmatch_t;
    let mut options: OnigOptionType = 0;
    options =
        (((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
             << 1 as libc::c_int) << 1 as libc::c_int;
    if posix_options & (1 as libc::c_int) << 2 as libc::c_int !=
           0 as libc::c_int {
        options |=
            (((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                       1 as libc::c_int) << 1 as libc::c_int) <<
                     1 as libc::c_int) << 1 as libc::c_int) <<
                   1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int
    }
    if posix_options & (1 as libc::c_int) << 3 as libc::c_int !=
           0 as libc::c_int {
        options |=
            ((((((((((1 as libc::c_uint) << 1 as libc::c_int) <<
                        1 as libc::c_int) << 1 as libc::c_int) <<
                      1 as libc::c_int) << 1 as libc::c_int) <<
                    1 as libc::c_int) << 1 as libc::c_int) <<
                  1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
    }
    if nmatch == 0 as libc::c_int as libc::c_ulong ||
           (*reg).comp_options & (1 as libc::c_int) << 5 as libc::c_int !=
               0 as libc::c_int {
        pm = 0 as *mut libc::c_void as *mut regmatch_t;
        nmatch = 0 as libc::c_int as size_t
    } else if (nmatch as libc::c_int) <
                  (*((*reg).onig as *mut onig_regex_t)).num_mem +
                      1 as libc::c_int {
        pm =
            malloc((::std::mem::size_of::<regmatch_t>() as
                        libc::c_ulong).wrapping_mul(((*((*reg).onig as
                                                            *mut onig_regex_t)).num_mem
                                                         + 1 as libc::c_int)
                                                        as libc::c_ulong)) as
                *mut regmatch_t;
        if pm.is_null() { return 12 as libc::c_int }
    } else { pm = pmatch }
    if (*(*((*reg).onig as *mut onig_regex_t)).enc).min_enc_len ==
           1 as libc::c_int {
        let mut tmps: *mut OnigUChar = str as *mut OnigUChar;
        while *tmps as libc::c_int != 0 as libc::c_int {
            tmps = tmps.offset(1)
        }
        len =
            tmps.wrapping_offset_from(str as *mut OnigUChar) as libc::c_long
                as libc::c_int
    } else {
        len =
            onigenc_str_bytelen_null((*((*reg).onig as
                                            *mut onig_regex_t)).enc,
                                     str as *mut OnigUChar)
    }
    end = str.offset(len as isize) as *mut OnigUChar;
    r =
        onig_search((*reg).onig as *mut onig_regex_t, str as *mut OnigUChar,
                    end, str as *mut OnigUChar, end, pm as *mut OnigRegion,
                    options);
    if r >= 0 as libc::c_int {
        r = 0 as libc::c_int;
        if pm != pmatch && !pm.is_null() {
            memcpy(pmatch as *mut libc::c_void, pm as *const libc::c_void,
                   (::std::mem::size_of::<regmatch_t>() as
                        libc::c_ulong).wrapping_mul(nmatch));
        }
    } else if r == -(1 as libc::c_int) {
        r = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nmatch as libc::c_int {
            let ref mut fresh0 = (*pmatch.offset(i as isize)).rm_eo;
            *fresh0 = -(1 as libc::c_int);
            (*pmatch.offset(i as isize)).rm_so = *fresh0;
            i += 1
        }
    } else { r = onig2posix_error_code(r) }
    if pm != pmatch && !pm.is_null() { free(pm as *mut libc::c_void); }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn regfree(mut reg: *mut regex_t) {
    onig_free((*reg).onig as *mut onig_regex_t);
}
/* extended API */
#[no_mangle]
pub unsafe extern "C" fn reg_set_encoding(mut mb_code: libc::c_int) {
    let mut enc: OnigEncoding = 0 as *mut OnigEncodingType;
    match mb_code {
        0 => { enc = &mut OnigEncodingASCII }
        1 => { enc = &mut OnigEncodingEUC_JP }
        2 => { enc = &mut OnigEncodingSJIS }
        3 => { enc = &mut OnigEncodingUTF8 }
        4 => { enc = &mut OnigEncodingUTF16_BE }
        5 => { enc = &mut OnigEncodingUTF16_LE }
        _ => { return }
    }
    onig_initialize(0 as *mut OnigEncoding, 0 as libc::c_int);
    onig_initialize_encoding(enc);
    onigenc_set_default_encoding(enc);
}
#[no_mangle]
pub unsafe extern "C" fn reg_name_to_group_numbers(mut reg: *mut regex_t,
                                                   mut name:
                                                       *const libc::c_uchar,
                                                   mut name_end:
                                                       *const libc::c_uchar,
                                                   mut nums:
                                                       *mut *mut libc::c_int)
 -> libc::c_int {
    return onig_name_to_group_numbers((*reg).onig as *mut onig_regex_t, name,
                                      name_end, nums);
}
unsafe extern "C" fn i_wrapper(mut name: *const OnigUChar,
                               mut name_end: *const OnigUChar,
                               mut ng: libc::c_int, mut gs: *mut libc::c_int,
                               mut reg: *mut onig_regex_t,
                               mut arg: *mut libc::c_void) -> libc::c_int {
    let mut warg: *mut i_wrap = arg as *mut i_wrap;
    return Some((*warg).func.expect("non-null function pointer")).expect("non-null function pointer")(name,
                                                                                                      name_end,
                                                                                                      ng,
                                                                                                      gs,
                                                                                                      (*warg).reg,
                                                                                                      (*warg).arg);
}
#[no_mangle]
pub unsafe extern "C" fn reg_foreach_name(mut reg: *mut regex_t,
                                          mut func:
                                              Option<unsafe extern "C" fn(_:
                                                                              *const libc::c_uchar,
                                                                          _:
                                                                              *const libc::c_uchar,
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
    let mut warg: i_wrap =
        i_wrap{func: None,
               reg: 0 as *mut regex_t,
               arg: 0 as *mut libc::c_void,};
    warg.func = func;
    warg.reg = reg;
    warg.arg = arg;
    return onig_foreach_name((*reg).onig as *mut onig_regex_t,
                             Some(i_wrapper as
                                      unsafe extern "C" fn(_:
                                                               *const OnigUChar,
                                                           _:
                                                               *const OnigUChar,
                                                           _: libc::c_int,
                                                           _:
                                                               *mut libc::c_int,
                                                           _:
                                                               *mut onig_regex_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                             &mut warg as *mut i_wrap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn reg_number_of_names(mut reg: *mut regex_t)
 -> libc::c_int {
    return onig_number_of_names((*reg).onig as *mut onig_regex_t);
}
