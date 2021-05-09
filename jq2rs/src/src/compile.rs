#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jq_state;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __isinf(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __isinfl(__value: f128::f128) -> libc::c_int;
    #[no_mangle]
    fn __isinff(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __strdup(__string: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_invalid() -> jv;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_true() -> jv;
    #[no_mangle]
    fn jv_false() -> jv;
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_sized(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_has(object: jv, key: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_object_delete(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_merge(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_object_iter(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_next(_: jv, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_valid(_: jv, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_key(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_keys_unsorted(_: jv) -> jv;
    #[no_mangle]
    fn opcode_describe(op: opcode) -> *const opcode_description;
    #[no_mangle]
    fn locfile_retain(_: *mut locfile) -> *mut locfile;
    #[no_mangle]
    fn locfile_free(_: *mut locfile);
    #[no_mangle]
    fn locfile_locate(_: *mut locfile, _: location, _: *const libc::c_char,
                      _: ...);
    #[no_mangle]
    fn bytecode_free(bc: *mut bytecode);
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type uint16_t = libc::c_ushort;
pub type jv_kind = libc::c_uint;
pub const JV_KIND_OBJECT: jv_kind = 7;
pub const JV_KIND_ARRAY: jv_kind = 6;
pub const JV_KIND_STRING: jv_kind = 5;
pub const JV_KIND_NUMBER: jv_kind = 4;
pub const JV_KIND_TRUE: jv_kind = 3;
pub const JV_KIND_FALSE: jv_kind = 2;
pub const JV_KIND_NULL: jv_kind = 1;
pub const JV_KIND_INVALID: jv_kind = 0;
/* All of the fields of this struct are private.
   Really. Do not play with them. */
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
pub const OP_BIND_WILDCARD: C2RustUnnamed_0 = 2048;
pub const OP_HAS_BINDING: C2RustUnnamed_0 = 1024;
pub const OP_IS_CALL_PSEUDO: C2RustUnnamed_0 = 128;
pub const OP_HAS_UFUNC: C2RustUnnamed_0 = 64;
pub const OP_HAS_CFUNC: C2RustUnnamed_0 = 32;
pub const OP_HAS_BRANCH: C2RustUnnamed_0 = 8;
pub const OP_HAS_VARIABLE: C2RustUnnamed_0 = 4;
pub const OP_HAS_CONSTANT: C2RustUnnamed_0 = 2;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct location {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locfile {
    pub fname: jv,
    pub data: *const libc::c_char,
    pub length: libc::c_int,
    pub linemap: *mut libc::c_int,
    pub nlines: libc::c_int,
    pub error: *mut libc::c_char,
    pub jq: *mut jq_state,
    pub refct: libc::c_int,
}
// for strdup
/*
  The intermediate representation for jq filters is as a sequence of
  struct inst, which form a doubly-linked list via the next and prev
  pointers.

  A "block" represents a sequence of "struct inst", which may be
  empty.

  Blocks are generated by the parser bottom-up, so may have free
  variables (refer to things not defined). See inst.bound_by and
  inst.symbol.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inst {
    pub next: *mut inst,
    pub prev: *mut inst,
    pub op: opcode,
    pub imm: C2RustUnnamed_1,
    pub locfile: *mut locfile,
    pub source: location,
    pub bound_by: *mut inst,
    pub symbol: *mut libc::c_char,
    pub any_unbound: libc::c_int,
    pub referenced: libc::c_int,
    pub nformals: libc::c_int,
    pub nactuals: libc::c_int,
    pub subfn: block,
    pub arglist: block,
    pub compiled: *mut bytecode,
    pub bytecode_pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub first: *mut inst,
    pub last: *mut inst,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub intval: uint16_t,
    pub target: *mut inst,
    pub constant: jv,
    pub cfunc: *const cfunction,
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
static mut UNKNOWN_LOCATION: location =
    {
        let mut init =
            location{start: -(1 as libc::c_int), end: -(1 as libc::c_int),};
        init
    };
// position just after this insn
unsafe extern "C" fn inst_new(mut op: opcode) -> *mut inst {
    let mut i: *mut inst =
        jv_mem_alloc(::std::mem::size_of::<inst>() as libc::c_ulong) as
            *mut inst;
    (*i).prev = 0 as *mut inst;
    (*i).next = (*i).prev;
    (*i).op = op;
    (*i).bytecode_pos = -(1 as libc::c_int);
    (*i).bound_by = 0 as *mut inst;
    (*i).symbol = 0 as *mut libc::c_char;
    (*i).any_unbound = 0 as libc::c_int;
    (*i).referenced = 0 as libc::c_int;
    (*i).nformals = -(1 as libc::c_int);
    (*i).nactuals = -(1 as libc::c_int);
    (*i).subfn = gen_noop();
    (*i).arglist = gen_noop();
    (*i).source = UNKNOWN_LOCATION;
    (*i).locfile = 0 as *mut locfile;
    return i;
}
unsafe extern "C" fn inst_free(mut i: *mut inst) {
    jv_mem_free((*i).symbol as *mut libc::c_void);
    block_free((*i).subfn);
    block_free((*i).arglist);
    if !(*i).locfile.is_null() { locfile_free((*i).locfile); }
    if (*opcode_describe((*i).op)).flags & OP_HAS_CONSTANT as libc::c_int != 0
       {
        jv_free((*i).imm.constant);
    }
    jv_mem_free(i as *mut libc::c_void);
}
unsafe extern "C" fn inst_block(mut i: *mut inst) -> block {
    let mut b: block = { let mut init = block{first: i, last: i,}; init };
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn block_is_single(mut b: block) -> libc::c_int {
    return (!b.first.is_null() && b.first == b.last) as libc::c_int;
}
unsafe extern "C" fn block_take(mut b: *mut block) -> *mut inst {
    if (*b).first.is_null() { return 0 as *mut inst }
    let mut i: *mut inst = (*b).first;
    if !(*i).next.is_null() {
        (*(*i).next).prev = 0 as *mut inst;
        (*b).first = (*i).next;
        (*i).next = 0 as *mut inst
    } else { (*b).first = 0 as *mut inst; (*b).last = 0 as *mut inst }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn gen_location(mut loc: location, mut l: *mut locfile,
                                      mut b: block) -> block {
    let mut i: *mut inst = b.first;
    while !i.is_null() {
        if (*i).source.start == UNKNOWN_LOCATION.start &&
               (*i).source.end == UNKNOWN_LOCATION.end {
            (*i).source = loc;
            (*i).locfile = locfile_retain(l)
        }
        i = (*i).next
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gen_noop() -> block {
    let mut b: block =
        {
            let mut init =
                block{first: 0 as *mut inst, last: 0 as *mut inst,};
            init
        };
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn block_is_noop(mut b: block) -> libc::c_int {
    return (b.first.is_null() && b.last.is_null()) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_simple(mut op: opcode) -> block {
    if (*opcode_describe(op)).length == 1 as libc::c_int {
    } else {
        __assert_fail(b"opcode_describe(op)->length == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      145 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"block gen_op_simple(opcode)\x00")).as_ptr());
    };
    return inst_block(inst_new(op));
}
#[no_mangle]
pub unsafe extern "C" fn gen_const(mut constant: jv) -> block {
    if (*opcode_describe(LOADK)).flags & OP_HAS_CONSTANT as libc::c_int != 0 {
    } else {
        __assert_fail(b"opcode_describe(LOADK)->flags & OP_HAS_CONSTANT\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      151 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"block gen_const(jv)\x00")).as_ptr());
    };
    let mut i: *mut inst = inst_new(LOADK);
    (*i).imm.constant = constant;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn gen_const_global(mut constant: jv,
                                          mut name: *const libc::c_char)
 -> block {
    if (*opcode_describe(STORE_GLOBAL)).flags &
           (OP_HAS_CONSTANT as libc::c_int | OP_HAS_VARIABLE as libc::c_int |
                OP_HAS_BINDING as libc::c_int) ==
           OP_HAS_CONSTANT as libc::c_int | OP_HAS_VARIABLE as libc::c_int |
               OP_HAS_BINDING as libc::c_int {
    } else {
        __assert_fail(b"(opcode_describe(STORE_GLOBAL)->flags & (OP_HAS_CONSTANT | OP_HAS_VARIABLE | OP_HAS_BINDING)) == (OP_HAS_CONSTANT | OP_HAS_VARIABLE | OP_HAS_BINDING)\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      159 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"block gen_const_global(jv, const char *)\x00")).as_ptr());
    };
    let mut i: *mut inst = inst_new(STORE_GLOBAL);
    (*i).imm.constant = constant;
    (*i).symbol =
        if 0 != 0 &&
               (name.offset(1 as libc::c_int as isize) as *const libc::c_void
                    as
                    size_t).wrapping_sub(name as *const libc::c_void as
                                             size_t) ==
                   1 as libc::c_int as libc::c_ulong {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '\u{0}' as i32 {
                calloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
                    as *mut libc::c_char
            } else {
                ({
                     let mut __len: size_t =
                         strlen(name).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong);
                     let mut __retval: *mut libc::c_char =
                         malloc(__len) as *mut libc::c_char;
                     if !__retval.is_null() {
                         __retval =
                             memcpy(__retval as *mut libc::c_void,
                                    name as *const libc::c_void, __len) as
                                 *mut libc::c_char
                     }
                     __retval
                 })
            }
        } else { __strdup(name) };
    (*i).any_unbound = 0 as libc::c_int;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_pushk_under(mut constant: jv) -> block {
    if (*opcode_describe(PUSHK_UNDER)).flags & OP_HAS_CONSTANT as libc::c_int
           != 0 {
    } else {
        __assert_fail(b"opcode_describe(PUSHK_UNDER)->flags & OP_HAS_CONSTANT\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      168 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"block gen_op_pushk_under(jv)\x00")).as_ptr());
    };
    let mut i: *mut inst = inst_new(PUSHK_UNDER);
    (*i).imm.constant = constant;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn block_is_const(mut b: block) -> libc::c_int {
    return (block_is_single(b) != 0 &&
                ((*b.first).op as libc::c_uint ==
                     LOADK as libc::c_int as libc::c_uint ||
                     (*b.first).op as libc::c_uint ==
                         PUSHK_UNDER as libc::c_int as libc::c_uint)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn block_is_const_inf(mut b: block) -> libc::c_int {
    return (block_is_single(b) != 0 &&
                (*b.first).op as libc::c_uint ==
                    LOADK as libc::c_int as libc::c_uint &&
                jv_get_kind((*b.first).imm.constant) as libc::c_uint ==
                    JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
                (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                        ==
                        ::std::mem::size_of::<libc::c_float>() as
                            libc::c_ulong {
                     __isinff(jv_number_value((*b.first).imm.constant) as
                                  libc::c_float)
                 } else {
                     (if ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong ==
                             ::std::mem::size_of::<libc::c_double>() as
                                 libc::c_ulong {
                          __isinf(jv_number_value((*b.first).imm.constant))
                      } else {
                          __isinfl(f128::f128::new(jv_number_value((*b.first).imm.constant)))
                      })
                 }) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn block_const_kind(mut b: block) -> jv_kind {
    if block_is_const(b) != 0 {
    } else {
        __assert_fail(b"block_is_const(b)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      185 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"jv_kind block_const_kind(block)\x00")).as_ptr());
    };
    return jv_get_kind((*b.first).imm.constant);
}
#[no_mangle]
pub unsafe extern "C" fn block_const(mut b: block) -> jv {
    if block_is_const(b) != 0 {
    } else {
        __assert_fail(b"block_is_const(b)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      190 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"jv block_const(block)\x00")).as_ptr());
    };
    return jv_copy((*b.first).imm.constant);
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_target(mut op: opcode, mut target: block)
 -> block {
    if (*opcode_describe(op)).flags & OP_HAS_BRANCH as libc::c_int != 0 {
    } else {
        __assert_fail(b"opcode_describe(op)->flags & OP_HAS_BRANCH\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      195 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"block gen_op_target(opcode, block)\x00")).as_ptr());
    };
    if !target.last.is_null() {
    } else {
        __assert_fail(b"target.last\x00" as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      196 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"block gen_op_target(opcode, block)\x00")).as_ptr());
    };
    let mut i: *mut inst = inst_new(op);
    (*i).imm.target = target.last;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_targetlater(mut op: opcode) -> block {
    if (*opcode_describe(op)).flags & OP_HAS_BRANCH as libc::c_int != 0 {
    } else {
        __assert_fail(b"opcode_describe(op)->flags & OP_HAS_BRANCH\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      203 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"block gen_op_targetlater(opcode)\x00")).as_ptr());
    };
    let mut i: *mut inst = inst_new(op);
    (*i).imm.target = 0 as *mut inst;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn inst_set_target(mut b: block, mut target: block) {
    if block_is_single(b) != 0 {
    } else {
        __assert_fail(b"block_is_single(b)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      209 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void inst_set_target(block, block)\x00")).as_ptr());
    };
    if (*opcode_describe((*b.first).op)).flags & OP_HAS_BRANCH as libc::c_int
           != 0 {
    } else {
        __assert_fail(b"opcode_describe(b.first->op)->flags & OP_HAS_BRANCH\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      210 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void inst_set_target(block, block)\x00")).as_ptr());
    };
    if !target.last.is_null() {
    } else {
        __assert_fail(b"target.last\x00" as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      211 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void inst_set_target(block, block)\x00")).as_ptr());
    };
    (*b.first).imm.target = target.last;
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_unbound(mut op: opcode,
                                        mut name: *const libc::c_char)
 -> block {
    if (*opcode_describe(op)).flags & OP_HAS_BINDING as libc::c_int != 0 {
    } else {
        __assert_fail(b"opcode_describe(op)->flags & OP_HAS_BINDING\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      216 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"block gen_op_unbound(opcode, const char *)\x00")).as_ptr());
    };
    let mut i: *mut inst = inst_new(op);
    (*i).symbol =
        if 0 != 0 &&
               (name.offset(1 as libc::c_int as isize) as *const libc::c_void
                    as
                    size_t).wrapping_sub(name as *const libc::c_void as
                                             size_t) ==
                   1 as libc::c_int as libc::c_ulong {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '\u{0}' as i32 {
                calloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
                    as *mut libc::c_char
            } else {
                ({
                     let mut __len: size_t =
                         strlen(name).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong);
                     let mut __retval: *mut libc::c_char =
                         malloc(__len) as *mut libc::c_char;
                     if !__retval.is_null() {
                         __retval =
                             memcpy(__retval as *mut libc::c_void,
                                    name as *const libc::c_void, __len) as
                                 *mut libc::c_char
                     }
                     __retval
                 })
            }
        } else { __strdup(name) };
    (*i).any_unbound = 1 as libc::c_int;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_var_fresh(mut op: opcode,
                                          mut name: *const libc::c_char)
 -> block {
    if (*opcode_describe(op)).flags & OP_HAS_VARIABLE as libc::c_int != 0 {
    } else {
        __assert_fail(b"opcode_describe(op)->flags & OP_HAS_VARIABLE\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      224 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"block gen_op_var_fresh(opcode, const char *)\x00")).as_ptr());
    };
    let mut b: block = gen_op_unbound(op, name);
    (*b.first).bound_by = b.first;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gen_op_bound(mut op: opcode, mut binder: block)
 -> block {
    if block_is_single(binder) != 0 {
    } else {
        __assert_fail(b"block_is_single(binder)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      231 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"block gen_op_bound(opcode, block)\x00")).as_ptr());
    };
    let mut b: block = gen_op_unbound(op, (*binder.first).symbol);
    (*b.first).bound_by = binder.first;
    (*b.first).any_unbound = 0 as libc::c_int;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gen_dictpair(mut k: block, mut v: block) -> block {
    return block_join(block_join(gen_subexp(k), gen_subexp(v)),
                      gen_op_simple(INSERT));
}
unsafe extern "C" fn inst_join(mut a: *mut inst, mut b: *mut inst) {
    if !a.is_null() && !b.is_null() {
    } else {
        __assert_fail(b"a && b\x00" as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      244 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void inst_join(inst *, inst *)\x00")).as_ptr());
    };
    if (*a).next.is_null() {
    } else {
        __assert_fail(b"!a->next\x00" as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      245 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void inst_join(inst *, inst *)\x00")).as_ptr());
    };
    if (*b).prev.is_null() {
    } else {
        __assert_fail(b"!b->prev\x00" as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      246 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void inst_join(inst *, inst *)\x00")).as_ptr());
    };
    (*a).next = b;
    (*b).prev = a;
}
#[no_mangle]
pub unsafe extern "C" fn block_append(mut b: *mut block, mut b2: block) {
    if !b2.first.is_null() {
        if !(*b).last.is_null() {
            inst_join((*b).last, b2.first);
        } else { (*b).first = b2.first }
        (*b).last = b2.last
    };
}
#[no_mangle]
pub unsafe extern "C" fn block_join(mut a: block, mut b: block) -> block {
    let mut c: block = a;
    block_append(&mut c, b);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn block_has_only_binders_and_imports(mut binders:
                                                                block,
                                                            mut bindflags:
                                                                libc::c_int)
 -> libc::c_int {
    bindflags |= OP_HAS_BINDING as libc::c_int;
    let mut curr: *mut inst = binders.first;
    while !curr.is_null() {
        if (*opcode_describe((*curr).op)).flags & bindflags != bindflags &&
               (*curr).op as libc::c_uint !=
                   DEPS as libc::c_int as libc::c_uint &&
               (*curr).op as libc::c_uint !=
                   MODULEMETA as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
        curr = (*curr).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn block_has_only_binders(mut binders: block,
                                                mut bindflags: libc::c_int)
 -> libc::c_int {
    bindflags |= OP_HAS_BINDING as libc::c_int;
    bindflags &= !(OP_BIND_WILDCARD as libc::c_int);
    let mut curr: *mut inst = binders.first;
    while !curr.is_null() {
        if (*opcode_describe((*curr).op)).flags & bindflags != bindflags &&
               (*curr).op as libc::c_uint !=
                   MODULEMETA as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
        curr = (*curr).next
    }
    return 1 as libc::c_int;
}
// Count a call site's actual params
unsafe extern "C" fn block_count_actuals(mut b: block) -> libc::c_int {
    let mut args: libc::c_int = 0 as libc::c_int;
    let mut i: *mut inst = b.first;
    while !i.is_null() {
        match (*i).op as libc::c_uint {
            32 | 30 | 33 => { args += 1 }
            _ => {
                if 0 as libc::c_int != 0 &&
                       !(b"Unknown function type\x00" as *const u8 as
                             *const libc::c_char).is_null() {
                } else {
                    __assert_fail(b"0 && \"Unknown function type\"\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"src/compile.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  298 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 31],
                                                            &[libc::c_char; 31]>(b"int block_count_actuals(block)\x00")).as_ptr());
                };
            }
        }
        i = (*i).next
    }
    return args;
}
unsafe extern "C" fn block_bind_subblock(mut binder: block, mut body: block,
                                         mut bindflags: libc::c_int,
                                         mut break_distance: libc::c_int)
 -> libc::c_int {
    let mut any_unbound: libc::c_int = 0;
    return block_bind_subblock_inner(&mut any_unbound, binder, body,
                                     bindflags, break_distance);
}
unsafe extern "C" fn block_bind_each(mut binder: block, mut body: block,
                                     mut bindflags: libc::c_int)
 -> libc::c_int {
    if block_has_only_binders(binder, bindflags) != 0 {
    } else {
        __assert_fail(b"block_has_only_binders(binder, bindflags)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      361 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int block_bind_each(block, block, int)\x00")).as_ptr());
    };
    bindflags |= OP_HAS_BINDING as libc::c_int;
    let mut nrefs: libc::c_int = 0 as libc::c_int;
    let mut curr: *mut inst = binder.first;
    while !curr.is_null() {
        nrefs +=
            block_bind_subblock(inst_block(curr), body, bindflags,
                                0 as libc::c_int);
        curr = (*curr).next
    }
    return nrefs;
}
unsafe extern "C" fn block_bind(mut binder: block, mut body: block,
                                mut bindflags: libc::c_int) -> block {
    block_bind_each(binder, body, bindflags);
    return block_join(binder, body);
}
#[no_mangle]
pub unsafe extern "C" fn block_bind_library(mut binder: block,
                                            mut body: block,
                                            mut bindflags: libc::c_int,
                                            mut libname: *const libc::c_char)
 -> block {
    bindflags |= OP_HAS_BINDING as libc::c_int;
    let mut nrefs: libc::c_int = 0 as libc::c_int;
    let mut matchlen: libc::c_int =
        if libname.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else { strlen(libname) } as libc::c_int;
    let mut matchname: *mut libc::c_char =
        jv_mem_alloc((matchlen + 2 as libc::c_int + 1 as libc::c_int) as
                         size_t) as *mut libc::c_char;
    *matchname.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    if !libname.is_null() &&
           *libname.offset(0 as libc::c_int as isize) as libc::c_int !=
               '\u{0}' as i32 {
        strcpy(matchname, libname);
        strcpy(matchname.offset(matchlen as isize),
               b"::\x00" as *const u8 as *const libc::c_char);
        matchlen += 2 as libc::c_int
    }
    if block_has_only_binders(binder, bindflags) != 0 {
    } else {
        __assert_fail(b"block_has_only_binders(binder, bindflags)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      386 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"block block_bind_library(block, block, int, const char *)\x00")).as_ptr());
    };
    let mut curr: *mut inst = binder.last;
    while !curr.is_null() {
        let mut bindflags2: libc::c_int = bindflags;
        let mut cname: *mut libc::c_char = (*curr).symbol;
        let mut tname: *mut libc::c_char =
            jv_mem_alloc(strlen((*curr).symbol).wrapping_add(matchlen as
                                                                 libc::c_ulong).wrapping_add(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                as *mut libc::c_char;
        strcpy(tname, matchname);
        strcpy(tname.offset(matchlen as isize), (*curr).symbol);
        // Ew
        if (*opcode_describe((*curr).op)).flags &
               (OP_HAS_VARIABLE as libc::c_int |
                    OP_HAS_CONSTANT as libc::c_int) != 0 {
            bindflags2 =
                OP_HAS_VARIABLE as libc::c_int | OP_HAS_BINDING as libc::c_int
        }
        // This mutation is ugly, even if we undo it
        (*curr).symbol = tname;
        nrefs +=
            block_bind_subblock(inst_block(curr), body, bindflags2,
                                0 as libc::c_int);
        (*curr).symbol = cname;
        free(tname as *mut libc::c_void);
        curr = (*curr).prev
    }
    free(matchname as *mut libc::c_void);
    return body;
    // We don't return a join because we don't want those sticking around...
}
unsafe extern "C" fn block_take_last(mut b: *mut block) -> *mut inst {
    let mut i: *mut inst = (*b).last;
    if i.is_null() { return 0 as *mut inst }
    if !(*i).prev.is_null() {
        (*(*i).prev).next = (*i).next;
        (*b).last = (*i).prev;
        (*i).prev = 0 as *mut inst
    } else { (*b).first = 0 as *mut inst; (*b).last = 0 as *mut inst }
    return i;
}
// Binds a sequence of binders, which *must not* alrady be bound to each other,
// to body, throwing away unreferenced defs
#[no_mangle]
pub unsafe extern "C" fn block_bind_referenced(mut binder: block,
                                               mut body: block,
                                               mut bindflags: libc::c_int)
 -> block {
    if block_has_only_binders(binder, bindflags) != 0 {
    } else {
        __assert_fail(b"block_has_only_binders(binder, bindflags)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      426 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"block block_bind_referenced(block, block, int)\x00")).as_ptr());
    };
    bindflags |= OP_HAS_BINDING as libc::c_int;
    let mut curr: *mut inst = 0 as *mut inst;
    loop  {
        curr = block_take_last(&mut binder);
        if curr.is_null() { break ; }
        let mut b: block = inst_block(curr);
        if block_bind_subblock(b, body, bindflags, 0 as libc::c_int) ==
               0 as libc::c_int {
            block_free(b);
        } else { body = block_join(b, body) }
    }
    return body;
}
#[no_mangle]
pub unsafe extern "C" fn block_bind_self(mut binder: block,
                                         mut bindflags: libc::c_int)
 -> block {
    if block_has_only_binders(binder, bindflags) != 0 {
    } else {
        __assert_fail(b"block_has_only_binders(binder, bindflags)\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      442 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"block block_bind_self(block, int)\x00")).as_ptr());
    };
    bindflags |= OP_HAS_BINDING as libc::c_int;
    let mut body: block = gen_noop();
    let mut curr: *mut inst = 0 as *mut inst;
    loop  {
        curr = block_take_last(&mut binder);
        if curr.is_null() { break ; }
        let mut b: block = inst_block(curr);
        block_bind_subblock(b, body, bindflags, 0 as libc::c_int);
        body = block_join(b, body)
    }
    return body;
}
unsafe extern "C" fn block_mark_referenced(mut body: block) {
    let mut saw_top: libc::c_int = 0 as libc::c_int;
    let mut i: *mut inst = body.last;
    while !i.is_null() {
        if !(saw_top != 0 && (*i).bound_by == i && (*i).referenced == 0) {
            if (*i).op as libc::c_uint == TOP as libc::c_int as libc::c_uint {
                saw_top = 1 as libc::c_int
            }
            if !(*i).bound_by.is_null() {
                (*(*i).bound_by).referenced = 1 as libc::c_int
            }
            block_mark_referenced((*i).arglist);
            block_mark_referenced((*i).subfn);
        }
        i = (*i).prev
    };
}
#[no_mangle]
pub unsafe extern "C" fn block_drop_unreferenced(mut body: block) -> block {
    block_mark_referenced(body);
    let mut refd: block = gen_noop();
    let mut curr: *mut inst = 0 as *mut inst;
    loop  {
        curr = block_take(&mut body);
        if curr.is_null() { break ; }
        if (*curr).bound_by == curr && (*curr).referenced == 0 {
            inst_free(curr);
        } else { refd = block_join(refd, inst_block(curr)) }
    }
    return refd;
}
#[no_mangle]
pub unsafe extern "C" fn block_take_imports(mut body: *mut block) -> jv {
    let mut imports: jv = jv_array();
    /* Parser should never generate TOP before imports */
    if !(!(*body).first.is_null() &&
             (*(*body).first).op as libc::c_uint ==
                 TOP as libc::c_int as libc::c_uint &&
             !(*(*body).first).next.is_null() &&
             ((*(*(*body).first).next).op as libc::c_uint ==
                  MODULEMETA as libc::c_int as libc::c_uint ||
                  (*(*(*body).first).next).op as libc::c_uint ==
                      DEPS as libc::c_int as libc::c_uint)) {
    } else {
        __assert_fail(b"!(body->first && body->first->op == TOP && body->first->next && (body->first->next->op == MODULEMETA || body->first->next->op == DEPS))\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      492 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"jv block_take_imports(block *)\x00")).as_ptr()); // Use the keys for set semantics.
    };
    while !(*body).first.is_null() &&
              ((*(*body).first).op as libc::c_uint ==
                   MODULEMETA as libc::c_int as libc::c_uint ||
                   (*(*body).first).op as libc::c_uint ==
                       DEPS as libc::c_int as libc::c_uint) {
        let mut dep: *mut inst = block_take(body);
        if (*dep).op as libc::c_uint == DEPS as libc::c_int as libc::c_uint {
            imports = jv_array_append(imports, jv_copy((*dep).imm.constant))
        }
        inst_free(dep);
    }
    return imports;
}
#[no_mangle]
pub unsafe extern "C" fn block_list_funcs(mut body: block,
                                          mut omit_underscores: libc::c_int)
 -> jv {
    let mut funcs: jv = jv_object();
    let mut pos: *mut inst = body.first;
    while !pos.is_null() {
        if (*pos).op as libc::c_uint ==
               CLOSURE_CREATE as libc::c_int as libc::c_uint ||
               (*pos).op as libc::c_uint ==
                   CLOSURE_CREATE_C as libc::c_int as libc::c_uint {
            if !(*pos).symbol.is_null() &&
                   (omit_underscores == 0 ||
                        *(*pos).symbol.offset(0 as libc::c_int as isize) as
                            libc::c_int != '_' as i32) {
                funcs =
                    jv_object_set(funcs,
                                  jv_string_fmt(b"%s/%i\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (*pos).symbol,
                                                (*pos).nformals), jv_null())
            }
        }
        pos = (*pos).next
    }
    return jv_keys_unsorted(funcs);
}
#[no_mangle]
pub unsafe extern "C" fn gen_module(mut metadata: block) -> block {
    let mut i: *mut inst = inst_new(MODULEMETA);
    (*i).imm.constant = block_const(metadata);
    if jv_get_kind((*i).imm.constant) as libc::c_uint !=
           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        (*i).imm.constant =
            jv_object_set(jv_object(),
                          jv_string(b"metadata\x00" as *const u8 as
                                        *const libc::c_char),
                          (*i).imm.constant)
    }
    block_free(metadata);
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn block_module_meta(mut b: block) -> jv {
    if !b.first.is_null() &&
           (*b.first).op as libc::c_uint ==
               MODULEMETA as libc::c_int as libc::c_uint {
        return jv_copy((*b.first).imm.constant)
    }
    return jv_null();
}
#[no_mangle]
pub unsafe extern "C" fn gen_import(mut name: *const libc::c_char,
                                    mut as_0: *const libc::c_char,
                                    mut is_data: libc::c_int) -> block {
    let mut i: *mut inst = inst_new(DEPS);
    let mut meta: jv = jv_object();
    if !as_0.is_null() {
        meta =
            jv_object_set(meta,
                          jv_string(b"as\x00" as *const u8 as
                                        *const libc::c_char), jv_string(as_0))
    }
    meta =
        jv_object_set(meta,
                      jv_string(b"is_data\x00" as *const u8 as
                                    *const libc::c_char),
                      if is_data != 0 { jv_true() } else { jv_false() });
    meta =
        jv_object_set(meta,
                      jv_string(b"relpath\x00" as *const u8 as
                                    *const libc::c_char), jv_string(name));
    (*i).imm.constant = meta;
    return inst_block(i);
}
#[no_mangle]
pub unsafe extern "C" fn gen_import_meta(mut import: block,
                                         mut metadata: block) -> block {
    if block_is_single(import) != 0 &&
           (*import.first).op as libc::c_uint ==
               DEPS as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"block_is_single(import) && import.first->op == DEPS\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      543 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"block gen_import_meta(block, block)\x00")).as_ptr());
    };
    if block_is_const(metadata) != 0 &&
           block_const_kind(metadata) as libc::c_uint ==
               JV_KIND_OBJECT as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"block_is_const(metadata) && block_const_kind(metadata) == JV_KIND_OBJECT\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      544 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"block gen_import_meta(block, block)\x00")).as_ptr());
    };
    let mut i: *mut inst = import.first;
    (*i).imm.constant =
        jv_object_merge(block_const(metadata), (*i).imm.constant);
    block_free(metadata);
    return import;
}
#[no_mangle]
pub unsafe extern "C" fn gen_function(mut name: *const libc::c_char,
                                      mut formals: block, mut body: block)
 -> block {
    let mut i: *mut inst = inst_new(CLOSURE_CREATE);
    let mut nformals: libc::c_int = 0 as libc::c_int;
    let mut i_0: *mut inst = formals.last;
    while !i_0.is_null() {
        nformals += 1;
        (*i_0).nformals = 0 as libc::c_int;
        if (*i_0).op as libc::c_uint ==
               CLOSURE_PARAM_REGULAR as libc::c_int as libc::c_uint {
            (*i_0).op = CLOSURE_PARAM;
            body =
                gen_var_binding(gen_call((*i_0).symbol, gen_noop()),
                                (*i_0).symbol, body)
        }
        block_bind_subblock(inst_block(i_0), body,
                            OP_IS_CALL_PSEUDO as libc::c_int |
                                OP_HAS_BINDING as libc::c_int,
                            0 as libc::c_int);
        i_0 = (*i_0).prev
    }
    (*i).subfn = body;
    (*i).symbol =
        if 0 != 0 &&
               (name.offset(1 as libc::c_int as isize) as *const libc::c_void
                    as
                    size_t).wrapping_sub(name as *const libc::c_void as
                                             size_t) ==
                   1 as libc::c_int as libc::c_ulong {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '\u{0}' as i32 {
                calloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
                    as *mut libc::c_char
            } else {
                ({
                     let mut __len: size_t =
                         strlen(name).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong);
                     let mut __retval: *mut libc::c_char =
                         malloc(__len) as *mut libc::c_char;
                     if !__retval.is_null() {
                         __retval =
                             memcpy(__retval as *mut libc::c_void,
                                    name as *const libc::c_void, __len) as
                                 *mut libc::c_char
                     }
                     __retval
                 })
            }
        } else { __strdup(name) };
    (*i).any_unbound = -(1 as libc::c_int);
    (*i).nformals = nformals;
    (*i).arglist = formals;
    let mut b: block = inst_block(i);
    block_bind_subblock(b, b,
                        OP_IS_CALL_PSEUDO as libc::c_int |
                            OP_HAS_BINDING as libc::c_int, 0 as libc::c_int);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gen_param_regular(mut name: *const libc::c_char)
 -> block {
    return gen_op_unbound(CLOSURE_PARAM_REGULAR, name);
}
#[no_mangle]
pub unsafe extern "C" fn gen_param(mut name: *const libc::c_char) -> block {
    return gen_op_unbound(CLOSURE_PARAM, name);
}
#[no_mangle]
pub unsafe extern "C" fn gen_lambda(mut body: block) -> block {
    return gen_function(b"@lambda\x00" as *const u8 as *const libc::c_char,
                        gen_noop(), body);
}
#[no_mangle]
pub unsafe extern "C" fn gen_call(mut name: *const libc::c_char,
                                  mut args: block) -> block {
    let mut b: block = gen_op_unbound(CALL_JQ, name);
    (*b.first).arglist = args;
    (*b.first).nactuals = block_count_actuals((*b.first).arglist);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gen_subexp(mut a: block) -> block {
    if block_is_noop(a) != 0 { return gen_op_simple(DUP) }
    if block_is_single(a) != 0 &&
           (*a.first).op as libc::c_uint ==
               LOADK as libc::c_int as libc::c_uint {
        let mut c: jv = block_const(a);
        block_free(a);
        return gen_op_pushk_under(c)
    }
    return block_join(block_join(gen_op_simple(SUBEXP_BEGIN), a),
                      gen_op_simple(SUBEXP_END));
}
#[no_mangle]
pub unsafe extern "C" fn gen_both(mut a: block, mut b: block) -> block {
    let mut jump: block = gen_op_targetlater(JUMP);
    let mut fork: block = gen_op_target(FORK, jump);
    let mut c: block = block_join(block_join(block_join(fork, a), jump), b);
    inst_set_target(jump, c);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn gen_const_object(mut expr: block) -> block {
    let mut is_const: libc::c_int = 1 as libc::c_int;
    let mut o: jv = jv_object();
    let mut k: jv = jv_null();
    let mut v: jv = jv_null();
    let mut i: *mut inst = expr.first;
    while !i.is_null() {
        if (*i).op as libc::c_uint ==
               PUSHK_UNDER as libc::c_int as libc::c_uint {
            k = jv_copy((*i).imm.constant);
            i = (*i).next
        } else if (*i).op as libc::c_uint !=
                      SUBEXP_BEGIN as libc::c_int as libc::c_uint ||
                      (*i).next.is_null() ||
                      (*(*i).next).op as libc::c_uint !=
                          LOADK as libc::c_int as libc::c_uint ||
                      (*(*i).next).next.is_null() ||
                      (*(*(*i).next).next).op as libc::c_uint !=
                          SUBEXP_END as libc::c_int as libc::c_uint {
            is_const = 0 as libc::c_int;
            break ;
        } else {
            k = jv_copy((*(*i).next).imm.constant);
            i = (*(*(*i).next).next).next
        }
        if !i.is_null() &&
               (*i).op as libc::c_uint ==
                   PUSHK_UNDER as libc::c_int as libc::c_uint {
            v = jv_copy((*i).imm.constant);
            i = (*i).next
        } else if i.is_null() ||
                      (*i).op as libc::c_uint !=
                          SUBEXP_BEGIN as libc::c_int as libc::c_uint ||
                      (*i).next.is_null() ||
                      (*(*i).next).op as libc::c_uint !=
                          LOADK as libc::c_int as libc::c_uint ||
                      (*(*i).next).next.is_null() ||
                      (*(*(*i).next).next).op as libc::c_uint !=
                          SUBEXP_END as libc::c_int as libc::c_uint {
            is_const = 0 as libc::c_int;
            break ;
        } else {
            v = jv_copy((*(*i).next).imm.constant);
            i = (*(*(*i).next).next).next
        }
        if i.is_null() ||
               (*i).op as libc::c_uint !=
                   INSERT as libc::c_int as libc::c_uint {
            is_const = 0 as libc::c_int;
            break ;
        } else if jv_get_kind(k) as libc::c_uint !=
                      JV_KIND_STRING as libc::c_int as libc::c_uint {
            is_const = 0 as libc::c_int;
            break ;
        } else {
            o = jv_object_set(o, k, v);
            k = jv_null();
            v = jv_null();
            i = (*i).next
        }
    }
    if is_const == 0 {
        jv_free(o);
        jv_free(k);
        jv_free(v);
        let mut b: block =
            {
                let mut init =
                    block{first: 0 as *mut inst, last: 0 as *mut inst,};
                init
            };
        return b
    }
    block_free(expr);
    return gen_const(o);
}
unsafe extern "C" fn gen_const_array(mut expr: block) -> block {
    /*
   * An expr of all constant elements looks like this:
   *
   * 0009 FORK 0027
   * 0011 FORK 0023
   * 0013 FORK 0019
   * 0015 LOADK 1
   * 0017 JUMP 0021
   * 0019 LOADK 2
   * 0021 JUMP 0025
   * 0023 LOADK 3
   * 0025 JUMP 0029
   * 0027 LOADK 4
   *
   * That's: N-1 commas for N elements, N LOADKs, and a JUMP between
   * every LOADK.  The sequence ends in a LOADK.  Any deviation and it's
   * not a list of constants.
   *
   * Here we check for this pattern almost exactly.  We don't check that
   * the targets of the FORK and JUMP instructions are in the right
   * sequence.
   */
    let mut all_const: libc::c_int = 1 as libc::c_int;
    let mut commas: libc::c_int = 0 as libc::c_int;
    let mut normal: libc::c_int = 1 as libc::c_int;
    let mut a: jv = jv_array();
    let mut i: *mut inst = expr.first;
    while !i.is_null() {
        if (*i).op as libc::c_uint == FORK as libc::c_int as libc::c_uint {
            commas += 1;
            if (*i).imm.target.is_null() ||
                   (*(*i).imm.target).op as libc::c_uint !=
                       JUMP as libc::c_int as libc::c_uint ||
                   jv_array_length(jv_copy(a)) > 0 as libc::c_int {
                normal = 0 as libc::c_int;
                break ;
            }
        } else if all_const != 0 &&
                      (*i).op as libc::c_uint ==
                          LOADK as libc::c_int as libc::c_uint {
            if !(*i).next.is_null() &&
                   (*(*i).next).op as libc::c_uint !=
                       JUMP as libc::c_int as libc::c_uint {
                normal = 0 as libc::c_int;
                break ;
            } else { a = jv_array_append(a, jv_copy((*i).imm.constant)) }
        } else if (*i).op as libc::c_uint !=
                      JUMP as libc::c_int as libc::c_uint ||
                      (*i).imm.target.is_null() ||
                      (*(*i).imm.target).op as libc::c_uint !=
                          LOADK as libc::c_int as libc::c_uint {
            all_const = 0 as libc::c_int
        }
        i = (*i).next
    }
    if all_const != 0 && normal != 0 &&
           (expr.last.is_null() ||
                (*expr.last).op as libc::c_uint ==
                    LOADK as libc::c_int as libc::c_uint) &&
           jv_array_length(jv_copy(a)) == commas + 1 as libc::c_int {
        block_free(expr);
        return gen_const(a)
    }
    jv_free(a);
    let mut b: block =
        {
            let mut init =
                block{first: 0 as *mut inst, last: 0 as *mut inst,};
            init
        };
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn gen_collect(mut expr: block) -> block {
    let mut const_array: block = gen_const_array(expr);
    if !const_array.first.is_null() { return const_array }
    let mut array_var: block =
        gen_op_var_fresh(STOREV,
                         b"collect\x00" as *const u8 as *const libc::c_char);
    let mut c: block =
        block_join(block_join(gen_op_simple(DUP), gen_const(jv_array())),
                   array_var);
    let mut tail: block =
        block_join(gen_op_bound(APPEND, array_var), gen_op_simple(BACKTRACK));
    return block_join(block_join(block_join(block_join(c,
                                                       gen_op_target(FORK,
                                                                     tail)),
                                            expr), tail),
                      gen_op_bound(LOADVN, array_var));
}
unsafe extern "C" fn bind_matcher(mut matcher: block, mut body: block)
 -> block {
    // cannot call block_bind(matcher, body) because that requires
  // block_has_only_binders(matcher), which is not true here as matchers
  // may also contain code to extract the correct elements
    let mut i: *mut inst = matcher.first;
    while !i.is_null() {
        if ((*i).op as libc::c_uint == STOREV as libc::c_int as libc::c_uint
                ||
                (*i).op as libc::c_uint ==
                    STOREVN as libc::c_int as libc::c_uint) &&
               (*i).bound_by.is_null() {
            block_bind_subblock(inst_block(i), body,
                                OP_HAS_VARIABLE as libc::c_int,
                                0 as libc::c_int);
        }
        i = (*i).next
    }
    return block_join(matcher, body);
}
// Extract destructuring var names from the block
// *vars should be a jv_object (for set semantics)
unsafe extern "C" fn block_get_unbound_vars(mut b: block, mut vars: *mut jv) {
    if !vars.is_null() {
    } else {
        __assert_fail(b"vars != NULL\x00" as *const u8 as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      762 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void block_get_unbound_vars(block, jv *)\x00")).as_ptr());
    };
    if jv_get_kind(*vars) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(*vars) == JV_KIND_OBJECT\x00" as *const u8
                          as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      763 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void block_get_unbound_vars(block, jv *)\x00")).as_ptr());
    };
    let mut i: *mut inst = b.first;
    while !i.is_null() {
        if !(*i).subfn.first.is_null() {
            block_get_unbound_vars((*i).subfn, vars);
        } else if ((*i).op as libc::c_uint ==
                       STOREV as libc::c_int as libc::c_uint ||
                       (*i).op as libc::c_uint ==
                           STOREVN as libc::c_int as libc::c_uint) &&
                      (*i).bound_by.is_null() {
            *vars = jv_object_set(*vars, jv_string((*i).symbol), jv_true())
        }
        i = (*i).next
    };
}
/* Build wrappers around destructuring matchers so that we can chain them
 * when we have errors.  The approach is as follows:
 * DESTRUCTURE_ALT NEXT_MATCHER (unless last matcher)
 * existing_matcher_block
 * JUMP BODY
 */
unsafe extern "C" fn bind_alternation_matchers(mut matchers: block,
                                               mut body: block) -> block {
    let mut preamble: block =
        {
            let mut init =
                block{first: 0 as *mut inst, last: 0 as *mut inst,};
            init
        };
    let mut altmatchers: block =
        {
            let mut init =
                block{first: 0 as *mut inst, last: 0 as *mut inst,};
            init
        };
    let mut mb: block =
        {
            let mut init =
                block{first: 0 as *mut inst, last: 0 as *mut inst,};
            init
        };
    let mut final_matcher: block = matchers;
    // Pass through the matchers to find all destructured names.
    while !final_matcher.first.is_null() &&
              (*final_matcher.first).op as libc::c_uint ==
                  DESTRUCTURE_ALT as libc::c_int as libc::c_uint {
        block_append(&mut altmatchers,
                     inst_block(block_take(&mut final_matcher)));
    }
    // We don't have any alternations here, so we can use the simplest case.
    if altmatchers.first.is_null() {
        return bind_matcher(final_matcher, body)
    }
    // Collect var names
    let mut all_vars: jv = jv_object();
    block_get_unbound_vars(altmatchers, &mut all_vars);
    block_get_unbound_vars(final_matcher, &mut all_vars);
    // We need a preamble of STOREVs to which to bind the matchers and the body.
    let mut jv_i__: libc::c_int = jv_object_iter(all_vars);
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut key: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if jv_object_iter_valid(all_vars, jv_i__) != 0 {
                  key = jv_object_iter_key(all_vars, jv_i__);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            preamble =
                block_join(block_join(block_join(preamble,
                                                 gen_op_simple(DUP)),
                                      gen_const(jv_null())),
                           gen_op_unbound(STOREV, jv_string_value(key)));
            jv_free(key);
            jv_i__ = jv_object_iter_next(all_vars, jv_i__)
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(all_vars);
    // Now we build each matcher in turn
    let mut i: *mut inst = altmatchers.first;
    while !i.is_null() {
        let mut submatcher: block = (*i).subfn;
        // If we're successful, jump to the end of the matchers
        submatcher =
            block_join(submatcher, gen_op_target(JUMP, final_matcher));
        // DESTRUCTURE_ALT to the end of this submatcher so we can skip to the next one on error
        mb =
            block_join(block_join(mb,
                                  gen_op_target(DESTRUCTURE_ALT, submatcher)),
                       submatcher);
        // We're done with this inst and we don't want it anymore
    // But we can't let it free the submatcher block.
        (*i).subfn.last = 0 as *mut inst;
        (*i).subfn.first = (*i).subfn.last;
        i = (*i).next
    }
    // We're done with these insts now.
    block_free(altmatchers); // make that JUMP go bast the BACKTRACK at the end of the loop
    return bind_matcher(preamble,
                        block_join(block_join(mb, final_matcher), body));
}
#[no_mangle]
pub unsafe extern "C" fn gen_reduce(mut source: block, mut matcher: block,
                                    mut init: block, mut body: block)
 -> block {
    let mut res_var: block =
        gen_op_var_fresh(STOREV,
                         b"reduce\x00" as *const u8 as *const libc::c_char);
    let mut loop_0: block =
        block_join(block_join(block_join(gen_op_simple(DUPN), source),
                              bind_alternation_matchers(matcher,
                                                        block_join(block_join(gen_op_bound(LOADVN,
                                                                                           res_var),
                                                                              body),
                                                                   gen_op_bound(STOREV,
                                                                                res_var)))),
                   gen_op_simple(BACKTRACK));
    return block_join(block_join(block_join(block_join(block_join(gen_op_simple(DUP),
                                                                  init),
                                                       res_var),
                                            gen_op_target(FORK, loop_0)),
                                 loop_0), gen_op_bound(LOADVN, res_var));
}
#[no_mangle]
pub unsafe extern "C" fn gen_foreach(mut source: block, mut matcher: block,
                                     mut init: block, mut update: block,
                                     mut extract: block) -> block {
    let mut output: block = gen_op_targetlater(JUMP);
    let mut state_var: block =
        gen_op_var_fresh(STOREV,
                         b"foreach\x00" as *const u8 as *const libc::c_char);
    let mut loop_0: block =
        block_join(block_join(gen_op_simple(DUPN), source),
                   bind_alternation_matchers(matcher,
                                             block_join(block_join(block_join(block_join(block_join(gen_op_bound(LOADVN,
                                                                                                                 state_var),
                                                                                                    update),
                                                                                         gen_op_simple(DUP)),
                                                                              gen_op_bound(STOREV,
                                                                                           state_var)),
                                                                   extract),
                                                        output)));
    let mut foreach: block =
        block_join(block_join(block_join(block_join(block_join(gen_op_simple(DUP),
                                                               init),
                                                    state_var),
                                         gen_op_target(FORK, loop_0)),
                              loop_0), gen_op_simple(BACKTRACK));
    inst_set_target(output, foreach);
    return foreach;
}
#[no_mangle]
pub unsafe extern "C" fn gen_definedor(mut a: block, mut b: block) -> block {
    // var found := false
    let mut found_var: block =
        gen_op_var_fresh(STOREV,
                         b"found\x00" as *const u8 as *const libc::c_char);
    let mut init: block =
        block_join(block_join(gen_op_simple(DUP), gen_const(jv_false())),
                   found_var);
    // if found, backtrack. Otherwise execute b
    let mut backtrack: block = gen_op_simple(BACKTRACK);
    let mut tail: block =
        block_join(block_join(block_join(block_join(block_join(gen_op_simple(DUP),
                                                               gen_op_bound(LOADV,
                                                                            found_var)),
                                                    gen_op_target(JUMP_F,
                                                                  backtrack)),
                                         backtrack), gen_op_simple(POP)), b);
    // try again
    let mut if_notfound: block = gen_op_simple(BACKTRACK);
    // found := true, produce result
    let mut if_found: block =
        block_join(block_join(block_join(gen_op_simple(DUP),
                                         gen_const(jv_true())),
                              gen_op_bound(STOREV, found_var)),
                   gen_op_target(JUMP, tail));
    return block_join(block_join(block_join(block_join(block_join(block_join(init,
                                                                             gen_op_target(FORK,
                                                                                           if_notfound)),
                                                                  a),
                                                       gen_op_target(JUMP_F,
                                                                     if_found)),
                                            if_found), if_notfound), tail);
}
#[no_mangle]
pub unsafe extern "C" fn block_has_main(mut top: block) -> libc::c_int {
    let mut c: *mut inst = top.first;
    while !c.is_null() {
        if (*c).op as libc::c_uint == TOP as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        c = (*c).next
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn block_is_funcdef(mut b: block) -> libc::c_int {
    if !b.first.is_null() &&
           (*b.first).op as libc::c_uint ==
               CLOSURE_CREATE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gen_condbranch(mut iftrue: block, mut iffalse: block)
 -> block {
    iftrue = block_join(iftrue, gen_op_target(JUMP, iffalse));
    return block_join(block_join(gen_op_target(JUMP_F, iftrue), iftrue),
                      iffalse);
}
#[no_mangle]
pub unsafe extern "C" fn gen_and(mut a: block, mut b: block) -> block {
    // a and b = if a then (if b then true else false) else false
    return block_join(block_join(gen_op_simple(DUP), a),
                      gen_condbranch(block_join(block_join(gen_op_simple(POP),
                                                           b),
                                                gen_condbranch(gen_const(jv_true()),
                                                               gen_const(jv_false()))),
                                     block_join(gen_op_simple(POP),
                                                gen_const(jv_false()))));
}
#[no_mangle]
pub unsafe extern "C" fn gen_or(mut a: block, mut b: block) -> block {
    // a or b = if a then true else (if b then true else false)
    return block_join(block_join(gen_op_simple(DUP), a),
                      gen_condbranch(block_join(gen_op_simple(POP),
                                                gen_const(jv_true())),
                                     block_join(block_join(gen_op_simple(POP),
                                                           b),
                                                gen_condbranch(gen_const(jv_true()),
                                                               gen_const(jv_false())))));
}
#[no_mangle]
pub unsafe extern "C" fn gen_destructure_alt(mut matcher: block) -> block {
    let mut i: *mut inst = matcher.first;
    while !i.is_null() {
        if (*i).op as libc::c_uint == STOREV as libc::c_int as libc::c_uint {
            (*i).op = STOREVN
        }
        i = (*i).next
    }
    let mut i_0: *mut inst = inst_new(DESTRUCTURE_ALT);
    (*i_0).subfn = matcher;
    return inst_block(i_0);
}
#[no_mangle]
pub unsafe extern "C" fn gen_var_binding(mut var: block,
                                         mut name: *const libc::c_char,
                                         mut body: block) -> block {
    return gen_destructure(var, gen_op_unbound(STOREV, name), body);
}
#[no_mangle]
pub unsafe extern "C" fn gen_array_matcher(mut left: block, mut curr: block)
 -> block {
    let mut index: libc::c_int = 0;
    if block_is_noop(left) != 0 {
        index = 0 as libc::c_int
    } else {
        // `left` was returned by this function, so the third inst is the
    // constant containing the previously used index
        if (*left.first).op as libc::c_uint ==
               DUP as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"left.first->op == DUP\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/compile.c\x00" as *const u8 as
                              *const libc::c_char,
                          985 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"block gen_array_matcher(block, block)\x00")).as_ptr());
        };
        if !(*left.first).next.is_null() {
        } else {
            __assert_fail(b"left.first->next != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/compile.c\x00" as *const u8 as
                              *const libc::c_char,
                          986 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"block gen_array_matcher(block, block)\x00")).as_ptr());
        };
        let mut i: *mut inst = 0 as *mut inst;
        if (*(*left.first).next).op as libc::c_uint ==
               PUSHK_UNDER as libc::c_int as libc::c_uint {
            i = (*left.first).next
        } else {
            if (*(*left.first).next).op as libc::c_uint ==
                   SUBEXP_BEGIN as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"left.first->next->op == SUBEXP_BEGIN\x00" as
                                  *const u8 as *const libc::c_char,
                              b"src/compile.c\x00" as *const u8 as
                                  *const libc::c_char,
                              991 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"block gen_array_matcher(block, block)\x00")).as_ptr());
            };
            if (*(*(*left.first).next).next).op as libc::c_uint ==
                   LOADK as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"left.first->next->next->op == LOADK\x00" as
                                  *const u8 as *const libc::c_char,
                              b"src/compile.c\x00" as *const u8 as
                                  *const libc::c_char,
                              992 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"block gen_array_matcher(block, block)\x00")).as_ptr());
            };
            i = (*(*left.first).next).next
        }
        index =
            1 as libc::c_int +
                jv_number_value((*i).imm.constant) as libc::c_int
    }
    // `left` goes at the end so that the const index is in a predictable place
    return block_join(block_join(block_join(block_join(gen_op_simple(DUP),
                                                       gen_subexp(gen_const(jv_number(index
                                                                                          as
                                                                                          libc::c_double)))),
                                            gen_op_simple(INDEX)), curr),
                      left);
}
#[no_mangle]
pub unsafe extern "C" fn gen_object_matcher(mut name: block, mut curr: block)
 -> block {
    return block_join(block_join(block_join(gen_op_simple(DUP),
                                            gen_subexp(name)),
                                 gen_op_simple(INDEX)), curr);
}
#[no_mangle]
pub unsafe extern "C" fn gen_destructure(mut var: block, mut matchers: block,
                                         mut body: block) -> block {
    // var bindings can be added after coding the program; leave the TOP first.
    let mut top: block = gen_noop();
    if !body.first.is_null() &&
           (*body.first).op as libc::c_uint ==
               TOP as libc::c_int as libc::c_uint {
        top = inst_block(block_take(&mut body))
    }
    if !matchers.first.is_null() &&
           (*matchers.first).op as libc::c_uint ==
               DESTRUCTURE_ALT as libc::c_int as libc::c_uint {
        block_append(&mut var, gen_op_simple(DUP));
    } else { top = block_join(top, gen_op_simple(DUP)) }
    return block_join(block_join(block_join(top, gen_subexp(var)),
                                 gen_op_simple(POP)),
                      bind_alternation_matchers(matchers, body));
}
// Like gen_var_binding(), but bind `break`'s wildcard unbound variable
unsafe extern "C" fn gen_wildvar_binding(mut var: block,
                                         mut name: *const libc::c_char,
                                         mut body: block) -> block {
    return block_join(block_join(gen_op_simple(DUP), var),
                      block_bind(gen_op_unbound(STOREV, name), body,
                                 OP_HAS_VARIABLE as libc::c_int |
                                     OP_BIND_WILDCARD as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn gen_cond(mut cond: block, mut iftrue: block,
                                  mut iffalse: block) -> block {
    return block_join(block_join(gen_op_simple(DUP),
                                 block_join(gen_subexp(cond),
                                            gen_op_simple(POP))),
                      gen_condbranch(block_join(gen_op_simple(POP), iftrue),
                                     block_join(gen_op_simple(POP),
                                                iffalse)));
}
#[no_mangle]
pub unsafe extern "C" fn gen_try_handler(mut handler: block) -> block {
    // Quite a pain just to hide jq's internal errors.
    return gen_cond(gen_and(gen_call(b"_equal\x00" as *const u8 as
                                         *const libc::c_char,
                                     block_join(gen_lambda(gen_const(jv_string(b"object\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char))),
                                                gen_lambda(gen_call(b"type\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    gen_noop())))),
                            block_join(block_join(gen_subexp(gen_const(jv_string(b"__jq\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char))),
                                                  gen_noop()),
                                       gen_op_simple(INDEX))),
                    gen_call(b"error\x00" as *const u8 as *const libc::c_char,
                             gen_noop()), handler);
}
#[no_mangle]
pub unsafe extern "C" fn gen_try(mut exp: block, mut handler: block)
 -> block {
    /*
   * Produce something like:
   *  FORK_OPT <address of handler>
   *  <exp>
   *  JUMP <end of handler>
   *  <handler>
   *
   * If this is not an internal try/catch, then catch and re-raise
   * internal errors to prevent them from leaking.
   *
   * The handler will only execute if we backtrack to the FORK_OPT with
   * an error (exception).  If <exp> produces no value then FORK_OPT
   * will backtrack (propagate the `empty`, as it were.  If <exp>
   * produces a value then we'll execute whatever bytecode follows this
   * sequence.
   */
    if handler.first.is_null() && handler.last.is_null() {
        // A hack to deal with `.` as the handler; we could use a real NOOP here
        handler =
            block_join(block_join(gen_op_simple(DUP), gen_op_simple(POP)),
                       handler)
    }
    exp = block_join(exp, gen_op_target(JUMP, handler));
    return block_join(block_join(gen_op_target(FORK_OPT, exp), exp), handler);
}
#[no_mangle]
pub unsafe extern "C" fn gen_label(mut label: *const libc::c_char,
                                   mut exp: block) -> block {
    let mut cond: block =
        gen_call(b"_equal\x00" as *const u8 as *const libc::c_char,
                 block_join(gen_lambda(gen_noop()),
                            gen_lambda(gen_op_unbound(LOADV, label))));
    return gen_wildvar_binding(gen_op_simple(GENLABEL), label,
                               block_join(gen_op_simple(POP),
                                          gen_try(exp,
                                                  gen_cond(cond,
                                                           gen_op_simple(BACKTRACK),
                                                           gen_call(b"error\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    gen_noop())))));
}
#[no_mangle]
pub unsafe extern "C" fn gen_cbinding(mut cfunctions: *const cfunction,
                                      mut ncfunctions: libc::c_int,
                                      mut code: block) -> block {
    let mut cfunc: libc::c_int = 0 as libc::c_int;
    while cfunc < ncfunctions {
        let mut i: *mut inst = inst_new(CLOSURE_CREATE_C);
        (*i).imm.cfunc =
            &*cfunctions.offset(cfunc as isize) as *const cfunction;
        (*i).symbol =
            if 0 != 0 &&
                   ((*cfunctions.offset(cfunc as
                                            isize)).name.offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                        as *const libc::c_void as
                        size_t).wrapping_sub((*cfunctions.offset(cfunc as
                                                                     isize)).name
                                                 as *const libc::c_void as
                                                 size_t) ==
                       1 as libc::c_int as libc::c_ulong {
                if *(*cfunctions.offset(cfunc as
                                            isize)).name.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                       as libc::c_int == '\u{0}' as i32 {
                    calloc(1 as libc::c_int as size_t,
                           1 as libc::c_int as size_t) as *mut libc::c_char
                } else {
                    ({
                         let mut __len: size_t =
                             strlen((*cfunctions.offset(cfunc as
                                                            isize)).name).wrapping_add(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong);
                         let mut __retval: *mut libc::c_char =
                             malloc(__len) as *mut libc::c_char;
                         if !__retval.is_null() {
                             __retval =
                                 memcpy(__retval as *mut libc::c_void,
                                        (*cfunctions.offset(cfunc as
                                                                isize)).name
                                            as *const libc::c_void, __len) as
                                     *mut libc::c_char
                         }
                         __retval
                     })
                }
            } else { __strdup((*cfunctions.offset(cfunc as isize)).name) };
        (*i).nformals =
            (*cfunctions.offset(cfunc as isize)).nargs - 1 as libc::c_int;
        (*i).any_unbound = 0 as libc::c_int;
        code = block_join(inst_block(i), code);
        cfunc += 1
    }
    return code;
}
unsafe extern "C" fn nesting_level(mut bc: *mut bytecode,
                                   mut target: *mut inst) -> uint16_t {
    let mut level: uint16_t = 0 as libc::c_int as uint16_t;
    if !bc.is_null() && !target.is_null() && !(*target).compiled.is_null() {
    } else {
        __assert_fail(b"bc && target && target->compiled\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      1108 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"uint16_t nesting_level(struct bytecode *, inst *)\x00")).as_ptr());
    };
    while !bc.is_null() && (*target).compiled != bc {
        level = level.wrapping_add(1);
        bc = (*bc).parent
    }
    if !bc.is_null() && bc == (*target).compiled {
    } else {
        __assert_fail(b"bc && bc == target->compiled\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      1113 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"uint16_t nesting_level(struct bytecode *, inst *)\x00")).as_ptr());
    };
    return level;
}
unsafe extern "C" fn count_cfunctions(mut b: block) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut i: *mut inst = b.first;
    while !i.is_null() {
        if (*i).op as libc::c_uint ==
               CLOSURE_CREATE_C as libc::c_int as libc::c_uint {
            n += 1
        }
        n += count_cfunctions((*i).subfn);
        i = (*i).next
    }
    return n;
}
unsafe extern "C" fn compile(mut bc: *mut bytecode, mut b: block,
                             mut lf: *mut locfile, mut args: jv,
                             mut env: *mut jv) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut var_frame_idx: libc::c_int = 0 as libc::c_int;
    (*bc).nsubfunctions = 0 as libc::c_int;
    errors += expand_call_arglist(&mut b, args, env);
    b = block_join(b, gen_op_simple(RET));
    let mut localnames: jv = jv_array();
    let mut curr: *mut inst = b.first;
    while !curr.is_null() {
        if (*curr).next.is_null() {
            if curr == b.last {
            } else {
                __assert_fail(b"curr == b.last\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/compile.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1249 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 66],
                                                        &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
            };
        }
        let mut length: libc::c_int = (*opcode_describe((*curr).op)).length;
        if (*curr).op as libc::c_uint ==
               CALL_JQ as libc::c_int as libc::c_uint {
            let mut arg: *mut inst = (*curr).arglist.first;
            while !arg.is_null() {
                length += 2 as libc::c_int;
                arg = (*arg).next
            }
        }
        pos += length;
        (*curr).bytecode_pos = pos;
        (*curr).compiled = bc;
        if (*curr).op as libc::c_uint !=
               CLOSURE_REF as libc::c_int as libc::c_uint &&
               (*curr).op as libc::c_uint !=
                   CLOSURE_PARAM as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"curr->op != CLOSURE_REF && curr->op != CLOSURE_PARAM\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/compile.c\x00" as *const u8 as
                              *const libc::c_char,
                          1260 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
        };
        if (*opcode_describe((*curr).op)).flags &
               OP_HAS_VARIABLE as libc::c_int != 0 && (*curr).bound_by == curr
           {
            let fresh0 = var_frame_idx;
            var_frame_idx = var_frame_idx + 1;
            (*curr).imm.intval = fresh0 as uint16_t;
            localnames =
                jv_array_append(localnames, jv_string((*curr).symbol))
        }
        if (*curr).op as libc::c_uint ==
               CLOSURE_CREATE as libc::c_int as libc::c_uint {
            if (*curr).bound_by == curr {
            } else {
                __assert_fail(b"curr->bound_by == curr\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/compile.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1269 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 66],
                                                        &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
            };
            let fresh1 = (*bc).nsubfunctions;
            (*bc).nsubfunctions = (*bc).nsubfunctions + 1;
            (*curr).imm.intval = fresh1 as uint16_t
        }
        if (*curr).op as libc::c_uint ==
               CLOSURE_CREATE_C as libc::c_int as libc::c_uint {
            if (*curr).bound_by == curr {
            } else {
                __assert_fail(b"curr->bound_by == curr\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/compile.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1273 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 66],
                                                        &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
            };
            let fresh2 = (*(*bc).globals).ncfunctions;
            (*(*bc).globals).ncfunctions = (*(*bc).globals).ncfunctions + 1;
            let mut idx: libc::c_int = fresh2;
            (*(*bc).globals).cfunc_names =
                jv_array_append((*(*bc).globals).cfunc_names,
                                jv_string((*curr).symbol));
            *(*(*bc).globals).cfunctions.offset(idx as isize) =
                *(*curr).imm.cfunc;
            (*curr).imm.intval = idx as uint16_t
        }
        curr = (*curr).next
    }
    if pos > 0xffff as libc::c_int {
        // too long for program counter to fit in uint16_t
        locfile_locate(lf, UNKNOWN_LOCATION,
                       b"function compiled to %d bytes which is too long\x00"
                           as *const u8 as *const libc::c_char, pos);
        errors += 1
    }
    (*bc).codelen = pos;
    (*bc).debuginfo =
        jv_object_set((*bc).debuginfo,
                      jv_string(b"locals\x00" as *const u8 as
                                    *const libc::c_char), localnames);
    if (*bc).nsubfunctions != 0 {
        (*bc).subfunctions =
            jv_mem_calloc(::std::mem::size_of::<*mut bytecode>() as
                              libc::c_ulong, (*bc).nsubfunctions as size_t) as
                *mut *mut bytecode;
        let mut curr_0: *mut inst = b.first;
        while !curr_0.is_null() {
            if (*curr_0).op as libc::c_uint ==
                   CLOSURE_CREATE as libc::c_int as libc::c_uint {
                let mut subfn: *mut bytecode =
                    jv_mem_alloc(::std::mem::size_of::<bytecode>() as
                                     libc::c_ulong) as *mut bytecode;
                let ref mut fresh3 =
                    *(*bc).subfunctions.offset((*curr_0).imm.intval as isize);
                *fresh3 = subfn;
                (*subfn).globals = (*bc).globals;
                (*subfn).parent = bc;
                (*subfn).nclosures = 0 as libc::c_int;
                (*subfn).debuginfo =
                    jv_object_set(jv_object(),
                                  jv_string(b"name\x00" as *const u8 as
                                                *const libc::c_char),
                                  jv_string((*curr_0).symbol));
                let mut params: jv = jv_array();
                let mut param: *mut inst = (*curr_0).arglist.first;
                while !param.is_null() {
                    if (*param).op as libc::c_uint ==
                           CLOSURE_PARAM as libc::c_int as libc::c_uint {
                    } else {
                        __assert_fail(b"param->op == CLOSURE_PARAM\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1301 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    if (*param).bound_by == param {
                    } else {
                        __assert_fail(b"param->bound_by == param\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1302 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    let fresh4 = (*subfn).nclosures;
                    (*subfn).nclosures = (*subfn).nclosures + 1;
                    (*param).imm.intval = fresh4 as uint16_t;
                    (*param).compiled = subfn;
                    params =
                        jv_array_append(params, jv_string((*param).symbol));
                    param = (*param).next
                }
                (*subfn).debuginfo =
                    jv_object_set((*subfn).debuginfo,
                                  jv_string(b"params\x00" as *const u8 as
                                                *const libc::c_char), params);
                errors += compile(subfn, (*curr_0).subfn, lf, args, env);
                (*curr_0).subfn = gen_noop()
            }
            curr_0 = (*curr_0).next
        }
    } else { (*bc).subfunctions = 0 as *mut *mut bytecode }
    let mut code: *mut uint16_t =
        jv_mem_calloc(::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                      (*bc).codelen as size_t) as *mut uint16_t;
    (*bc).code = code;
    pos = 0 as libc::c_int;
    let mut constant_pool: jv = jv_array();
    let mut maxvar: libc::c_int = -(1 as libc::c_int);
    if errors == 0 {
        let mut curr_1: *mut inst = b.first;
        while !curr_1.is_null() {
            let mut op: *const opcode_description =
                opcode_describe((*curr_1).op);
            if !((*op).length == 0 as libc::c_int) {
                let fresh5 = pos;
                pos = pos + 1;
                *code.offset(fresh5 as isize) = (*curr_1).op as uint16_t;
                if (*curr_1).op as libc::c_uint !=
                       CLOSURE_REF as libc::c_int as libc::c_uint &&
                       (*curr_1).op as libc::c_uint !=
                           CLOSURE_PARAM as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"curr->op != CLOSURE_REF && curr->op != CLOSURE_PARAM\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"src/compile.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1325 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 66],
                                                            &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                };
                if (*curr_1).op as libc::c_uint ==
                       CALL_BUILTIN as libc::c_int as libc::c_uint {
                    if (*(*curr_1).bound_by).op as libc::c_uint ==
                           CLOSURE_CREATE_C as libc::c_int as libc::c_uint {
                    } else {
                        __assert_fail(b"curr->bound_by->op == CLOSURE_CREATE_C\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1327 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    if (*curr_1).arglist.first.is_null() {
                    } else {
                        __assert_fail(b"!curr->arglist.first\x00" as *const u8
                                          as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1328 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    let fresh6 = pos;
                    pos = pos + 1;
                    *code.offset(fresh6 as isize) = (*curr_1).imm.intval;
                    let fresh7 = pos;
                    pos = pos + 1;
                    *code.offset(fresh7 as isize) =
                        (*(*curr_1).bound_by).imm.intval
                } else if (*curr_1).op as libc::c_uint ==
                              CALL_JQ as libc::c_int as libc::c_uint {
                    if (*(*curr_1).bound_by).op as libc::c_uint ==
                           CLOSURE_CREATE as libc::c_int as libc::c_uint ||
                           (*(*curr_1).bound_by).op as libc::c_uint ==
                               CLOSURE_PARAM as libc::c_int as libc::c_uint {
                    } else {
                        __assert_fail(b"curr->bound_by->op == CLOSURE_CREATE || curr->bound_by->op == CLOSURE_PARAM\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1333 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    let fresh8 = pos;
                    pos = pos + 1;
                    *code.offset(fresh8 as isize) = (*curr_1).imm.intval;
                    let fresh9 = pos;
                    pos = pos + 1;
                    *code.offset(fresh9 as isize) =
                        nesting_level(bc, (*curr_1).bound_by);
                    let fresh10 = pos;
                    pos = pos + 1;
                    *code.offset(fresh10 as isize) =
                        ((*(*curr_1).bound_by).imm.intval as libc::c_int |
                             (if (*(*curr_1).bound_by).op as libc::c_uint ==
                                     CLOSURE_CREATE as libc::c_int as
                                         libc::c_uint {
                                  0x1000 as libc::c_int
                              } else { 0 as libc::c_int })) as uint16_t;
                    let mut arg_0: *mut inst = (*curr_1).arglist.first;
                    while !arg_0.is_null() {
                        if (*arg_0).op as libc::c_uint ==
                               CLOSURE_REF as libc::c_int as libc::c_uint &&
                               (*(*arg_0).bound_by).op as libc::c_uint ==
                                   CLOSURE_CREATE as libc::c_int as
                                       libc::c_uint {
                        } else {
                            __assert_fail(b"arg->op == CLOSURE_REF && arg->bound_by->op == CLOSURE_CREATE\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"src/compile.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1339 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 66],
                                                                    &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                        };
                        let fresh11 = pos;
                        pos = pos + 1;
                        *code.offset(fresh11 as isize) =
                            nesting_level(bc, (*arg_0).bound_by);
                        let fresh12 = pos;
                        pos = pos + 1;
                        *code.offset(fresh12 as isize) =
                            ((*(*arg_0).bound_by).imm.intval as libc::c_int |
                                 0x1000 as libc::c_int) as uint16_t;
                        arg_0 = (*arg_0).next
                    }
                } else if (*op).flags & OP_HAS_CONSTANT as libc::c_int != 0 &&
                              (*op).flags & OP_HAS_VARIABLE as libc::c_int !=
                                  0 {
                    // STORE_GLOBAL: constant global, basically
                    let fresh13 = pos; // only forward branches
                    pos = pos + 1; // FIXME: frames of size zero?
                    *code.offset(fresh13 as isize) =
                        jv_array_length(jv_copy(constant_pool)) as uint16_t;
                    constant_pool =
                        jv_array_append(constant_pool,
                                        jv_copy((*curr_1).imm.constant));
                    let fresh14 = pos;
                    pos = pos + 1;
                    *code.offset(fresh14 as isize) =
                        nesting_level(bc, (*curr_1).bound_by);
                    let mut var: uint16_t = (*(*curr_1).bound_by).imm.intval;
                    let fresh15 = pos;
                    pos = pos + 1;
                    *code.offset(fresh15 as isize) = var
                } else if (*op).flags & OP_HAS_CONSTANT as libc::c_int != 0 {
                    let fresh16 = pos;
                    pos = pos + 1;
                    *code.offset(fresh16 as isize) =
                        jv_array_length(jv_copy(constant_pool)) as uint16_t;
                    constant_pool =
                        jv_array_append(constant_pool,
                                        jv_copy((*curr_1).imm.constant))
                } else if (*op).flags & OP_HAS_VARIABLE as libc::c_int != 0 {
                    let fresh17 = pos;
                    pos = pos + 1;
                    *code.offset(fresh17 as isize) =
                        nesting_level(bc, (*curr_1).bound_by);
                    let mut var_0: uint16_t =
                        (*(*curr_1).bound_by).imm.intval;
                    let fresh18 = pos;
                    pos = pos + 1;
                    *code.offset(fresh18 as isize) = var_0;
                    if var_0 as libc::c_int > maxvar {
                        maxvar = var_0 as libc::c_int
                    }
                } else if (*op).flags & OP_HAS_BRANCH as libc::c_int != 0 {
                    if (*(*curr_1).imm.target).bytecode_pos !=
                           -(1 as libc::c_int) {
                    } else {
                        __assert_fail(b"curr->imm.target->bytecode_pos != -1\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1359 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    if (*(*curr_1).imm.target).bytecode_pos > pos {
                    } else {
                        __assert_fail(b"curr->imm.target->bytecode_pos > pos\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1360 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                    *code.offset(pos as isize) =
                        ((*(*curr_1).imm.target).bytecode_pos -
                             (pos + 1 as libc::c_int)) as uint16_t;
                    pos += 1
                } else if (*op).length > 1 as libc::c_int {
                    if 0 as libc::c_int != 0 &&
                           !(b"codegen not implemented for this operation\x00"
                                 as *const u8 as
                                 *const libc::c_char).is_null() {
                    } else {
                        __assert_fail(b"0 && \"codegen not implemented for this operation\"\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/compile.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1364 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"int compile(struct bytecode *, block, struct locfile *, jv, jv *)\x00")).as_ptr());
                    };
                }
            }
            curr_1 = (*curr_1).next
        }
    }
    (*bc).constants = constant_pool;
    (*bc).nlocals = maxvar + 2 as libc::c_int;
    block_free(b);
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn block_compile(mut b: block,
                                       mut out: *mut *mut bytecode,
                                       mut lf: *mut locfile, mut args: jv)
 -> libc::c_int {
    let mut bc: *mut bytecode =
        jv_mem_alloc(::std::mem::size_of::<bytecode>() as libc::c_ulong) as
            *mut bytecode;
    (*bc).parent = 0 as *mut bytecode;
    (*bc).nclosures = 0 as libc::c_int;
    (*bc).globals =
        jv_mem_alloc(::std::mem::size_of::<symbol_table>() as libc::c_ulong)
            as *mut symbol_table;
    let mut ncfunc: libc::c_int = count_cfunctions(b);
    (*(*bc).globals).ncfunctions = 0 as libc::c_int;
    (*(*bc).globals).cfunctions =
        jv_mem_calloc(::std::mem::size_of::<cfunction>() as libc::c_ulong,
                      ncfunc as size_t) as *mut cfunction;
    (*(*bc).globals).cfunc_names = jv_array();
    (*bc).debuginfo =
        jv_object_set(jv_object(),
                      jv_string(b"name\x00" as *const u8 as
                                    *const libc::c_char), jv_null());
    let mut env: jv = jv_invalid();
    let mut nerrors: libc::c_int = compile(bc, b, lf, args, &mut env);
    jv_free(args);
    jv_free(env);
    if (*(*bc).globals).ncfunctions == ncfunc {
    } else {
        __assert_fail(b"bc->globals->ncfunctions == ncfunc\x00" as *const u8
                          as *const libc::c_char,
                      b"src/compile.c\x00" as *const u8 as
                          *const libc::c_char,
                      1387 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"int block_compile(block, struct bytecode **, struct locfile *, jv)\x00")).as_ptr());
    };
    if nerrors > 0 as libc::c_int {
        bytecode_free(bc);
        *out = 0 as *mut bytecode
    } else { *out = bc }
    return nerrors;
}
#[no_mangle]
pub unsafe extern "C" fn block_free(mut b: block) {
    let mut next: *mut inst = 0 as *mut inst;
    let mut curr: *mut inst = b.first;
    while !curr.is_null() {
        next = (*curr).next;
        inst_free(curr);
        curr = next
    };
}
