#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, extern_types, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    pub type jv_refcnt;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_dump(_: jv, flags: libc::c_int);
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
}
pub type uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jv {
    pub kind_flags: libc::c_uchar,
    pub pad_: libc::c_uchar,
    pub offset: libc::c_ushort,
    pub size: libc::c_int,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *mut jv_refcnt,
    pub number: libc::c_double,
}
pub type opcode = libc::c_uint;
pub const STOREVN: opcode = 40;
pub const DESTRUCTURE_ALT: opcode = 39;
pub const GENLABEL: opcode = 38;
pub const MODULEMETA: opcode = 37;
pub const DEPS: opcode = 36;
pub const CLOSURE_PARAM_REGULAR: opcode = 35;
pub const TOP: opcode = 34;
pub const CLOSURE_CREATE_C: opcode = 33;
pub const CLOSURE_CREATE: opcode = 32;
pub const CLOSURE_REF: opcode = 31;
pub const CLOSURE_PARAM: opcode = 30;
pub const TAIL_CALL_JQ: opcode = 29;
pub const RET: opcode = 28;
pub const CALL_JQ: opcode = 27;
pub const CALL_BUILTIN: opcode = 26;
pub const PATH_END: opcode = 25;
pub const PATH_BEGIN: opcode = 24;
pub const SUBEXP_END: opcode = 23;
pub const SUBEXP_BEGIN: opcode = 22;
pub const RANGE: opcode = 21;
pub const INSERT: opcode = 20;
pub const APPEND: opcode = 19;
pub const BACKTRACK: opcode = 18;
pub const JUMP_F: opcode = 17;
pub const JUMP: opcode = 16;
pub const FORK_OPT: opcode = 15;
pub const FORK: opcode = 14;
pub const EACH_OPT: opcode = 13;
pub const EACH: opcode = 12;
pub const INDEX_OPT: opcode = 11;
pub const INDEX: opcode = 10;
pub const STORE_GLOBAL: opcode = 9;
pub const STOREV: opcode = 8;
pub const LOADVN: opcode = 7;
pub const LOADV: opcode = 6;
pub const POP: opcode = 5;
pub const PUSHK_UNDER: opcode = 4;
pub const DUP2: opcode = 3;
pub const DUPN: opcode = 2;
pub const DUP: opcode = 1;
pub const LOADK: opcode = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUM_OPCODES: C2RustUnnamed_0 = 41;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OP_BIND_WILDCARD: C2RustUnnamed_1 = 2048;
pub const OP_HAS_BINDING: C2RustUnnamed_1 = 1024;
pub const OP_IS_CALL_PSEUDO: C2RustUnnamed_1 = 128;
pub const OP_HAS_UFUNC: C2RustUnnamed_1 = 64;
pub const OP_HAS_CFUNC: C2RustUnnamed_1 = 32;
pub const OP_HAS_BRANCH: C2RustUnnamed_1 = 8;
pub const OP_HAS_VARIABLE: C2RustUnnamed_1 = 4;
pub const OP_HAS_CONSTANT: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opcode_description {
    pub op: opcode,
    pub name: *const libc::c_char,
    pub flags: libc::c_int,
    pub length: libc::c_int,
    pub stack_in: libc::c_int,
    pub stack_out: libc::c_int,
}
pub type cfunction_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfunction {
    pub fptr: cfunction_ptr,
    pub name: *const libc::c_char,
    pub nargs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol_table {
    pub cfunctions: *mut cfunction,
    pub ncfunctions: libc::c_int,
    pub cfunc_names: jv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytecode {
    pub code: *mut uint16_t,
    pub codelen: libc::c_int,
    pub nlocals: libc::c_int,
    pub nclosures: libc::c_int,
    pub constants: jv,
    pub globals: *mut symbol_table,
    pub subfunctions: *mut *mut bytecode,
    pub nsubfunctions: libc::c_int,
    pub parent: *mut bytecode,
    pub debuginfo: jv,
}
// flags, length
static mut opcode_descriptions: [opcode_description; 41] =
    [{
         let mut init =
             opcode_description{op: LOADK,
                                name:
                                    b"LOADK\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_CONSTANT as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: DUP,
                                name:
                                    b"DUP\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: DUPN,
                                name:
                                    b"DUPN\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: DUP2,
                                name:
                                    b"DUP2\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 2 as libc::c_int,
                                stack_out: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: PUSHK_UNDER,
                                name:
                                    b"PUSHK_UNDER\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_CONSTANT as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: POP,
                                name:
                                    b"POP\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: LOADV,
                                name:
                                    b"LOADV\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: LOADVN,
                                name:
                                    b"LOADVN\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: STOREV,
                                name:
                                    b"STOREV\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: STORE_GLOBAL,
                                name:
                                    b"STORE_GLOBAL\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_CONSTANT as libc::c_int |
                                        OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int |
                                        OP_IS_CALL_PSEUDO as libc::c_int,
                                length: 4 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: INDEX,
                                name:
                                    b"INDEX\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 2 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: INDEX_OPT,
                                name:
                                    b"INDEX_OPT\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 2 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: EACH,
                                name:
                                    b"EACH\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: EACH_OPT,
                                name:
                                    b"EACH_OPT\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: FORK,
                                name:
                                    b"FORK\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_BRANCH as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: FORK_OPT,
                                name:
                                    b"FORK_OPT\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_BRANCH as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: JUMP,
                                name:
                                    b"JUMP\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_BRANCH as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: JUMP_F,
                                name:
                                    b"JUMP_F\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_BRANCH as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: BACKTRACK,
                                name:
                                    b"BACKTRACK\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: APPEND,
                                name:
                                    b"APPEND\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: INSERT,
                                name:
                                    b"INSERT\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 4 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: RANGE,
                                name:
                                    b"RANGE\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: SUBEXP_BEGIN,
                                name:
                                    b"SUBEXP_BEGIN\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: SUBEXP_END,
                                name:
                                    b"SUBEXP_END\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 2 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: PATH_BEGIN,
                                name:
                                    b"PATH_BEGIN\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: PATH_END,
                                name:
                                    b"PATH_END\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 2 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CALL_BUILTIN,
                                name:
                                    b"CALL_BUILTIN\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_CFUNC as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: -(1 as libc::c_int),
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CALL_JQ,
                                name:
                                    b"CALL_JQ\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_UFUNC as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int |
                                        OP_IS_CALL_PSEUDO as libc::c_int,
                                length: 4 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: RET,
                                name:
                                    b"RET\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: TAIL_CALL_JQ,
                                name:
                                    b"TAIL_CALL_JQ\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_UFUNC as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int |
                                        OP_IS_CALL_PSEUDO as libc::c_int,
                                length: 4 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CLOSURE_PARAM,
                                name:
                                    b"CLOSURE_PARAM\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_IS_CALL_PSEUDO as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 0 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CLOSURE_REF,
                                name:
                                    b"CLOSURE_REF\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_IS_CALL_PSEUDO as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CLOSURE_CREATE,
                                name:
                                    b"CLOSURE_CREATE\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_IS_CALL_PSEUDO as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 0 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CLOSURE_CREATE_C,
                                name:
                                    b"CLOSURE_CREATE_C\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_IS_CALL_PSEUDO as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 0 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: TOP,
                                name:
                                    b"TOP\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: CLOSURE_PARAM_REGULAR,
                                name:
                                    b"CLOSURE_PARAM_REGULAR\x00" as *const u8
                                        as *const libc::c_char,
                                flags:
                                    OP_IS_CALL_PSEUDO as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 0 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: DEPS,
                                name:
                                    b"DEPS\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_CONSTANT as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: MODULEMETA,
                                name:
                                    b"MODULEMETA\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_CONSTANT as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: GENLABEL,
                                name:
                                    b"GENLABEL\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: 0 as libc::c_int,
                                length: 1 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: DESTRUCTURE_ALT,
                                name:
                                    b"DESTRUCTURE_ALT\x00" as *const u8 as
                                        *const libc::c_char,
                                flags: OP_HAS_BRANCH as libc::c_int,
                                length: 2 as libc::c_int,
                                stack_in: 0 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             opcode_description{op: STOREVN,
                                name:
                                    b"STOREVN\x00" as *const u8 as
                                        *const libc::c_char,
                                flags:
                                    OP_HAS_VARIABLE as libc::c_int |
                                        OP_HAS_BINDING as libc::c_int,
                                length: 3 as libc::c_int,
                                stack_in: 1 as libc::c_int,
                                stack_out: 0 as libc::c_int,};
         init
     }];
static mut invalid_opcode_description: opcode_description =
    {
        let mut init =
            opcode_description{op: 4294967295 as opcode,
                               name:
                                   b"#INVALID\x00" as *const u8 as
                                       *const libc::c_char,
                               flags: 0 as libc::c_int,
                               length: 0 as libc::c_int,
                               stack_in: 0 as libc::c_int,
                               stack_out: 0 as libc::c_int,};
        init
    };
// NOTE: Not actually part of any op -- a pseudo-op flag for special
  //       handling of `break`.
// length in 16-bit units
#[no_mangle]
pub unsafe extern "C" fn opcode_describe(mut op: opcode)
 -> *const opcode_description {
    if op as libc::c_int >= 0 as libc::c_int &&
           (op as libc::c_int) < NUM_OPCODES as libc::c_int {
        return &*opcode_descriptions.as_ptr().offset(op as isize) as
                   *const opcode_description
    } else { return &invalid_opcode_description };
}
#[no_mangle]
pub unsafe extern "C" fn bytecode_operation_length(mut codeptr: *mut uint16_t)
 -> libc::c_int {
    let mut length: libc::c_int =
        (*opcode_describe(*codeptr as opcode)).length;
    if *codeptr as libc::c_int == CALL_JQ as libc::c_int ||
           *codeptr as libc::c_int == TAIL_CALL_JQ as libc::c_int {
        length +=
            *codeptr.offset(1 as libc::c_int as isize) as libc::c_int *
                2 as libc::c_int
    }
    return length;
}
unsafe extern "C" fn dump_code(mut indent: libc::c_int,
                               mut bc: *mut bytecode) {
    let mut pc: libc::c_int = 0 as libc::c_int;
    while pc < (*bc).codelen {
        printf(b"%*s\x00" as *const u8 as *const libc::c_char, indent,
               b"\x00" as *const u8 as *const libc::c_char);
        dump_operation(bc, (*bc).code.offset(pc as isize));
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        pc += bytecode_operation_length((*bc).code.offset(pc as isize))
    };
}
unsafe extern "C" fn symbol_table_free(mut syms: *mut symbol_table) {
    jv_mem_free((*syms).cfunctions as *mut libc::c_void);
    jv_free((*syms).cfunc_names);
    jv_mem_free(syms as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dump_disassembly(mut indent: libc::c_int,
                                          mut bc: *mut bytecode) {
    if (*bc).nclosures > 0 as libc::c_int {
        printf(b"%*s[params: \x00" as *const u8 as *const libc::c_char,
               indent, b"\x00" as *const u8 as *const libc::c_char);
        let mut params: jv =
            jv_object_get(jv_copy((*bc).debuginfo),
                          jv_string(b"params\x00" as *const u8 as
                                        *const libc::c_char));
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*bc).nclosures {
            if i != 0 {
                printf(b", \x00" as *const u8 as *const libc::c_char);
            }
            let mut name: jv = jv_array_get(jv_copy(params), i);
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   jv_string_value(name));
            jv_free(name);
            i += 1
        }
        jv_free(params);
        printf(b"]\n\x00" as *const u8 as *const libc::c_char);
    }
    dump_code(indent, bc);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*bc).nsubfunctions {
        let mut subfn: *mut bytecode =
            *(*bc).subfunctions.offset(i_0 as isize);
        let mut name_0: jv =
            jv_object_get(jv_copy((*subfn).debuginfo),
                          jv_string(b"name\x00" as *const u8 as
                                        *const libc::c_char));
        printf(b"%*s%s:%d:\n\x00" as *const u8 as *const libc::c_char, indent,
               b"\x00" as *const u8 as *const libc::c_char,
               jv_string_value(name_0), i_0);
        jv_free(name_0);
        dump_disassembly(indent + 2 as libc::c_int, subfn);
        i_0 += 1
    };
}
unsafe extern "C" fn getlevel(mut bc: *mut bytecode, mut level: libc::c_int)
 -> *mut bytecode {
    while level > 0 as libc::c_int { bc = (*bc).parent; level -= 1 }
    return bc;
}
#[no_mangle]
pub unsafe extern "C" fn dump_operation(mut bc: *mut bytecode,
                                        mut codeptr: *mut uint16_t) {
    let mut pc: libc::c_int =
        codeptr.wrapping_offset_from((*bc).code) as libc::c_long as
            libc::c_int;
    printf(b"%04d \x00" as *const u8 as *const libc::c_char, pc);
    let fresh0 = pc;
    pc = pc + 1;
    let mut op: *const opcode_description =
        opcode_describe(*(*bc).code.offset(fresh0 as isize) as opcode);
    printf(b"%s\x00" as *const u8 as *const libc::c_char, (*op).name);
    if (*op).length > 1 as libc::c_int {
        let fresh1 = pc;
        pc = pc + 1;
        let mut imm: uint16_t = *(*bc).code.offset(fresh1 as isize);
        if (*op).op as libc::c_uint == CALL_JQ as libc::c_int as libc::c_uint
               ||
               (*op).op as libc::c_uint ==
                   TAIL_CALL_JQ as libc::c_int as libc::c_uint {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < imm as libc::c_int + 1 as libc::c_int {
                let fresh2 = pc;
                pc = pc + 1;
                let mut level: uint16_t = *(*bc).code.offset(fresh2 as isize);
                let fresh3 = pc;
                pc = pc + 1;
                let mut idx: uint16_t = *(*bc).code.offset(fresh3 as isize);
                let mut name: jv =
                    jv{kind_flags: 0,
                       pad_: 0,
                       offset: 0,
                       size: 0,
                       u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                if idx as libc::c_int & 0x1000 as libc::c_int != 0 {
                    idx =
                        (idx as libc::c_int & !(0x1000 as libc::c_int)) as
                            uint16_t;
                    name =
                        jv_object_get(jv_copy((**(*getlevel(bc,
                                                            level as
                                                                libc::c_int)).subfunctions.offset(idx
                                                                                                      as
                                                                                                      isize)).debuginfo),
                                      jv_string(b"name\x00" as *const u8 as
                                                    *const libc::c_char))
                } else {
                    name =
                        jv_array_get(jv_object_get(jv_copy((*getlevel(bc,
                                                                      level as
                                                                          libc::c_int)).debuginfo),
                                                   jv_string(b"params\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)),
                                     idx as libc::c_int)
                }
                printf(b" %s:%d\x00" as *const u8 as *const libc::c_char,
                       jv_string_value(name), idx as libc::c_int);
                jv_free(name);
                if level != 0 {
                    printf(b"^%d\x00" as *const u8 as *const libc::c_char,
                           level as libc::c_int);
                }
                i += 1
            }
        } else if (*op).op as libc::c_uint ==
                      CALL_BUILTIN as libc::c_int as libc::c_uint {
            let fresh4 = pc;
            pc = pc + 1;
            let mut func: libc::c_int =
                *(*bc).code.offset(fresh4 as isize) as libc::c_int;
            let mut name_0: jv =
                jv_array_get(jv_copy((*(*bc).globals).cfunc_names), func);
            printf(b" %s\x00" as *const u8 as *const libc::c_char,
                   jv_string_value(name_0));
            jv_free(name_0);
        } else if (*op).flags & OP_HAS_BRANCH as libc::c_int != 0 {
            printf(b" %04d\x00" as *const u8 as *const libc::c_char,
                   pc + imm as libc::c_int);
        } else if (*op).flags & OP_HAS_CONSTANT as libc::c_int != 0 {
            printf(b" \x00" as *const u8 as *const libc::c_char);
            jv_dump(jv_array_get(jv_copy((*bc).constants),
                                 imm as libc::c_int), 0 as libc::c_int);
        } else if (*op).flags & OP_HAS_VARIABLE as libc::c_int != 0 {
            let fresh5 = pc;
            pc = pc + 1;
            let mut v: uint16_t = *(*bc).code.offset(fresh5 as isize);
            let mut name_1: jv =
                jv_array_get(jv_object_get(jv_copy((*getlevel(bc,
                                                              imm as
                                                                  libc::c_int)).debuginfo),
                                           jv_string(b"locals\x00" as
                                                         *const u8 as
                                                         *const libc::c_char)),
                             v as libc::c_int);
            printf(b" $%s:%d\x00" as *const u8 as *const libc::c_char,
                   jv_string_value(name_1), v as libc::c_int);
            jv_free(name_1);
            if imm != 0 {
                printf(b"^%d\x00" as *const u8 as *const libc::c_char,
                       imm as libc::c_int);
            }
        } else {
            printf(b" %d\x00" as *const u8 as *const libc::c_char,
                   imm as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytecode_free(mut bc: *mut bytecode) {
    if bc.is_null() { return }
    jv_mem_free((*bc).code as *mut libc::c_void);
    jv_free((*bc).constants);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*bc).nsubfunctions {
        bytecode_free(*(*bc).subfunctions.offset(i as isize));
        i += 1
    }
    if (*bc).parent.is_null() { symbol_table_free((*bc).globals); }
    jv_mem_free((*bc).subfunctions as *mut libc::c_void);
    jv_free((*bc).debuginfo);
    jv_mem_free(bc as *mut libc::c_void);
}
