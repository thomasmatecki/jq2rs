#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type inst;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_kind_name(_: jv_kind) -> *const libc::c_char;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_get_refcnt(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_identical(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_invalid() -> jv;
    #[no_mangle]
    fn jv_invalid_with_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_has_msg(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_concat(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_slice(_: jv, _: libc::c_int, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_object_iter(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_next(_: jv, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_valid(_: jv, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_object_iter_key(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_object_iter_value(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_dump(_: jv, flags: libc::c_int);
    #[no_mangle]
    fn jv_dump_string(_: jv, flags: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_dump_string_trunc(x: jv, outbuf: *mut libc::c_char, bufsize: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn jv_nomem_handler(_: jv_nomem_handler_f, _: *mut libc::c_void);
    #[no_mangle]
    fn jv_get(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_mem_realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_alloc_unguarded(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn opcode_describe(op: opcode) -> *const opcode_description;
    #[no_mangle]
    fn dump_disassembly(_: libc::c_int, code: *mut bytecode);
    #[no_mangle]
    fn dump_operation(bc: *mut bytecode, op: *mut uint16_t);
    #[no_mangle]
    fn bytecode_operation_length(codeptr: *mut uint16_t) -> libc::c_int;
    #[no_mangle]
    fn bytecode_free(bc: *mut bytecode);
    #[no_mangle]
    fn block_compile(_: block, _: *mut *mut bytecode, _: *mut locfile, _: jv)
     -> libc::c_int;
    #[no_mangle]
    fn locfile_init(_: *mut jq_state, _: *const libc::c_char,
                    _: *const libc::c_char, _: libc::c_int) -> *mut locfile;
    #[no_mangle]
    fn locfile_free(_: *mut locfile);
    #[no_mangle]
    fn builtins_bind(_: *mut jq_state, _: *mut block) -> libc::c_int;
    #[no_mangle]
    fn load_program(jq: *mut jq_state, src: *mut locfile,
                    out_block: *mut block) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
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
pub type jv_print_flags = libc::c_uint;
pub const JV_PRINT_SPACE2: jv_print_flags = 1024;
pub const JV_PRINT_SPACE1: jv_print_flags = 512;
pub const JV_PRINT_SPACE0: jv_print_flags = 256;
pub const JV_PRINT_ISATTY: jv_print_flags = 128;
pub const JV_PRINT_TAB: jv_print_flags = 64;
pub const JV_PRINT_REFCOUNT: jv_print_flags = 32;
pub const JV_PRINT_INVALID: jv_print_flags = 16;
pub const JV_PRINT_SORTED: jv_print_flags = 8;
pub const JV_PRINT_COLOUR: jv_print_flags = 4;
pub const JV_PRINT_COLOR: jv_print_flags = 4;
pub const JV_PRINT_ASCII: jv_print_flags = 2;
pub const JV_PRINT_PRETTY: jv_print_flags = 1;
pub type jv_nomem_handler_f
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ALIGNMENT: C2RustUnnamed_0 = 8;
pub type stack_ptr = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub mem_end: *mut libc::c_char,
    pub bound: stack_ptr,
    pub limit: stack_ptr,
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NUM_OPCODES: C2RustUnnamed_1 = 41;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const JQ_DEBUG_TRACE_ALL: C2RustUnnamed_2 = 3;
pub const JQ_DEBUG_TRACE_DETAIL: C2RustUnnamed_2 = 2;
pub const JQ_DEBUG_TRACE: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jq_state {
    pub nomem_handler: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> ()>,
    pub nomem_handler_data: *mut libc::c_void,
    pub bc: *mut bytecode,
    pub err_cb: jq_msg_cb,
    pub err_cb_data: *mut libc::c_void,
    pub error: jv,
    pub stk: stack,
    pub curr_frame: stack_ptr,
    pub stk_top: stack_ptr,
    pub fork_top: stack_ptr,
    pub path: jv,
    pub value_at_path: jv,
    pub subexp_nest: libc::c_int,
    pub debug_trace_enabled: libc::c_int,
    pub initial_execution: libc::c_int,
    pub next_label: libc::c_uint,
    pub halted: libc::c_int,
    pub exit_code: jv,
    pub error_message: jv,
    pub attrs: jv,
    pub input_cb: jq_input_cb,
    pub input_cb_data: *mut libc::c_void,
    pub debug_cb: jq_msg_cb,
    pub debug_cb_data: *mut libc::c_void,
}
pub type jq_msg_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: jv) -> ()>;
pub type jq_input_cb
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: *mut libc::c_void)
               -> jv>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub first: *mut inst,
    pub last: *mut inst,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct forkpoint {
    pub saved_data_stack: stack_ptr,
    pub saved_curr_frame: stack_ptr,
    pub path_len: libc::c_int,
    pub subexp_nest: libc::c_int,
    pub value_at_path: jv,
    pub return_address: *mut uint16_t,
}
// NOTE: Not actually part of any op -- a pseudo-op flag for special
  //       handling of `break`.
// length in 16-bit units
// jq function call frame
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame {
    pub bc: *mut bytecode,
    pub env: stack_ptr,
    pub retdata: stack_ptr,
    pub retaddr: *mut uint16_t,
    pub entries: [frame_entry; 0],
}
// jq stack address of closed frame
// locals for any function called: either a closure or a local variable
#[derive(Copy, Clone)]
#[repr(C)]
pub union frame_entry {
    pub closure: closure,
    pub localvar: jv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct closure {
    pub bc: *mut bytecode,
    pub env: stack_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_pos {
    pub saved_data_stack: stack_ptr,
    pub saved_curr_frame: stack_ptr,
}
pub type func_5
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: jv, _: jv, _: jv, _: jv,
                                _: jv) -> jv>;
pub type func_4
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: jv, _: jv, _: jv, _: jv)
               -> jv>;
pub type func_3
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: jv, _: jv, _: jv) -> jv>;
pub type func_2
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: jv, _: jv) -> jv>;
pub type func_1 = Option<unsafe extern "C" fn(_: *mut jq_state, _: jv) -> jv>;
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/*
 * The stack is a directed forest of variably sized blocks. Each block has a
 * "next" block which is at a higher memory address, or 0 if the block has no
 * "next" block. More than one block may have no "next" block. A block may be
 * the "next" block of more than one other block. Pushed blocks are added at
 * the low-address end of the stack.
 *
 * Stack pointers are negative integers that are offsets relative to "mem_end",
 * the end of the allocated region. The stack "bound" is the stack pointer of
 * the last block that would be able to fit in the currently allocated region.
 * The stack "limit" is the stack pointer of the last block currently in the
 * stack. The stack pointer of the "next" block is stored directly below each
 * block.
 *
 *                      <- mem_end = 0x100
 * 0xF8  +------------+
 * 0xF0  |            |
 * 0xE8  +------------+ <- stack_ptr1 = -0x18
 * 0xE0  next = 0
 * 0xD8  +------------+
 * 0xD0  |            |
 * 0xC8  |            |
 * 0xC0  +------------+ <- stack_ptr2 = limit = -0x40
 * 0xB8  next = -0x18
 * 0xB0
 * 0xA8                 <- bound = -0x58
 * 0xA0
 */
// one-past-the-end of allocated region
// 0 - stack is empty
unsafe extern "C" fn stack_pop_block(mut s: *mut stack, mut p: stack_ptr,
                                     mut sz: size_t) -> stack_ptr {
    let mut r: stack_ptr = *stack_block_next(s, p);
    if p == (*s).limit {
        let mut alloc_sz: libc::c_int =
            align_round_up(sz).wrapping_add(ALIGNMENT as libc::c_int as
                                                libc::c_ulong) as libc::c_int;
        (*s).limit += alloc_sz
    }
    return r;
}
unsafe extern "C" fn stack_push_block(mut s: *mut stack, mut p: stack_ptr,
                                      mut sz: size_t) -> stack_ptr {
    let mut alloc_sz: libc::c_int =
        align_round_up(sz).wrapping_add(ALIGNMENT as libc::c_int as
                                            libc::c_ulong) as libc::c_int;
    let mut r: stack_ptr = (*s).limit - alloc_sz;
    if r < (*s).bound { stack_reallocate(s, alloc_sz as size_t); }
    (*s).limit = r;
    *stack_block_next(s, r) = p;
    return r;
}
unsafe extern "C" fn stack_reallocate(mut s: *mut stack, mut sz: size_t) {
    let mut old_mem_length: libc::c_int =
        -(*s).bound + ALIGNMENT as libc::c_int;
    let mut old_mem_start: *mut libc::c_char =
        (*s).mem_end.offset(-(old_mem_length as isize));
    let mut new_mem_length: libc::c_int =
        align_round_up((old_mem_length as
                            libc::c_ulong).wrapping_add(sz).wrapping_add(256
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(2
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
            as libc::c_int;
    let mut new_mem_start: *mut libc::c_char =
        jv_mem_realloc(old_mem_start as *mut libc::c_void,
                       new_mem_length as size_t) as *mut libc::c_char;
    memmove(new_mem_start.offset((new_mem_length - old_mem_length) as isize)
                as *mut libc::c_void, new_mem_start as *const libc::c_void,
            old_mem_length as libc::c_ulong);
    (*s).mem_end = new_mem_start.offset(new_mem_length as isize);
    (*s).bound = -(new_mem_length - ALIGNMENT as libc::c_int);
}
unsafe extern "C" fn stack_block_next(mut s: *mut stack, mut p: stack_ptr)
 -> *mut stack_ptr {
    return &mut *((stack_block as
                       unsafe extern "C" fn(_: *mut stack, _: stack_ptr)
                           -> *mut libc::c_void)(s, p) as
                      *mut stack_ptr).offset(-(1 as libc::c_int) as isize) as
               *mut stack_ptr;
}
unsafe extern "C" fn stack_block(mut s: *mut stack, mut p: stack_ptr)
 -> *mut libc::c_void {
    return (*s).mem_end.offset(p as isize) as *mut libc::c_void;
}
unsafe extern "C" fn stack_pop_will_free(mut s: *mut stack, mut p: stack_ptr)
 -> libc::c_int {
    return (p == (*s).limit) as libc::c_int;
}
unsafe extern "C" fn stack_reset(mut s: *mut stack) {
    if (*s).limit == 0 as libc::c_int &&
           !(b"stack freed while not empty\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"s->limit == 0 && \"stack freed while not empty\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"./src/exec_stack.h\x00" as *const u8 as
                          *const libc::c_char,
                      63 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void stack_reset(struct stack *)\x00")).as_ptr());
    };
    let mut mem_start: *mut libc::c_char =
        (*s).mem_end.offset(-((-(*s).bound + ALIGNMENT as libc::c_int) as
                                  isize));
    free(mem_start as *mut libc::c_void);
    stack_init(s);
}
unsafe extern "C" fn stack_init(mut s: *mut stack) {
    (*s).mem_end = 0 as *mut libc::c_char;
    (*s).bound = ALIGNMENT as libc::c_int;
    (*s).limit = 0 as libc::c_int;
}
unsafe extern "C" fn align_round_up(mut sz: size_t) -> size_t {
    return sz.wrapping_add((ALIGNMENT as libc::c_int - 1 as libc::c_int) as
                               libc::c_ulong).wrapping_div(ALIGNMENT as
                                                               libc::c_int as
                                                               libc::c_ulong).wrapping_mul(ALIGNMENT
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong);
}
// nclosures + nlocals
unsafe extern "C" fn frame_size(mut bc: *mut bytecode) -> libc::c_int {
    return (::std::mem::size_of::<frame>() as
                libc::c_ulong).wrapping_add((::std::mem::size_of::<frame_entry>()
                                                 as
                                                 libc::c_ulong).wrapping_mul(((*bc).nclosures
                                                                                  +
                                                                                  (*bc).nlocals)
                                                                                 as
                                                                                 libc::c_ulong))
               as libc::c_int;
}
unsafe extern "C" fn frame_current(mut jq: *mut jq_state) -> *mut frame {
    let mut fp: *mut frame =
        stack_block(&mut (*jq).stk, (*jq).curr_frame) as *mut frame;
    let mut next: stack_ptr =
        *stack_block_next(&mut (*jq).stk, (*jq).curr_frame);
    if next != 0 {
        let mut fpnext: *mut frame =
            stack_block(&mut (*jq).stk, next) as *mut frame;
        let mut bc: *mut bytecode = (*fpnext).bc;
        if (*fp).retaddr >= (*bc).code &&
               (*fp).retaddr < (*bc).code.offset((*bc).codelen as isize) {
        } else {
            __assert_fail(b"fp->retaddr >= bc->code && fp->retaddr < bc->code + bc->codelen\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          85 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"struct frame *frame_current(struct jq_state *)\x00")).as_ptr());
        };
    } else {
        if (*fp).retaddr.is_null() {
        } else {
            __assert_fail(b"fp->retaddr == 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          87 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"struct frame *frame_current(struct jq_state *)\x00")).as_ptr());
        };
    }
    return fp;
}
unsafe extern "C" fn frame_get_level(mut jq: *mut jq_state,
                                     mut level: libc::c_int) -> stack_ptr {
    let mut fr: stack_ptr = (*jq).curr_frame;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < level {
        let mut fp: *mut frame =
            stack_block(&mut (*jq).stk, fr) as *mut frame;
        fr = (*fp).env;
        i += 1
    }
    return fr;
}
unsafe extern "C" fn frame_local_var(mut jq: *mut jq_state,
                                     mut var: libc::c_int,
                                     mut level: libc::c_int) -> *mut jv {
    let mut fr: *mut frame =
        stack_block(&mut (*jq).stk, frame_get_level(jq, level)) as *mut frame;
    if var >= 0 as libc::c_int {
    } else {
        __assert_fail(b"var >= 0\x00" as *const u8 as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      103 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"jv *frame_local_var(struct jq_state *, int, int)\x00")).as_ptr());
    };
    if var < (*(*fr).bc).nlocals {
    } else {
        __assert_fail(b"var < fr->bc->nlocals\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      104 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"jv *frame_local_var(struct jq_state *, int, int)\x00")).as_ptr());
    };
    return &mut (*(*fr).entries.as_mut_ptr().offset(((*(*fr).bc).nclosures +
                                                         var) as
                                                        isize)).localvar;
}
unsafe extern "C" fn make_closure(mut jq: *mut jq_state,
                                  mut pc: *mut uint16_t) -> closure {
    let fresh0 = pc;
    pc = pc.offset(1);
    let mut level: uint16_t = *fresh0;
    let fresh1 = pc;
    pc = pc.offset(1);
    let mut idx: uint16_t = *fresh1;
    let mut fridx: stack_ptr = frame_get_level(jq, level as libc::c_int);
    let mut fr: *mut frame = stack_block(&mut (*jq).stk, fridx) as *mut frame;
    if idx as libc::c_int & 0x1000 as libc::c_int != 0 {
        // A new closure closing the frame identified by level, and with
    // the bytecode body of the idx'th subfunction of that frame
        let mut subfn_idx: libc::c_int =
            idx as libc::c_int & !(0x1000 as libc::c_int);
        if subfn_idx < (*(*fr).bc).nsubfunctions {
        } else {
            __assert_fail(b"subfn_idx < fr->bc->nsubfunctions\x00" as
                              *const u8 as *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          117 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 59],
                                                    &[libc::c_char; 59]>(b"struct closure make_closure(struct jq_state *, uint16_t *)\x00")).as_ptr());
        };
        let mut cl: closure =
            {
                let mut init =
                    closure{bc:
                                *(*(*fr).bc).subfunctions.offset(subfn_idx as
                                                                     isize),
                            env: fridx,};
                init
            };
        return cl
    } else {
        // A reference to a closure from the frame identified by level; copy
    // it as-is
        let mut closure: libc::c_int = idx as libc::c_int;
        if closure >= 0 as libc::c_int {
        } else {
            __assert_fail(b"closure >= 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          125 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 59],
                                                    &[libc::c_char; 59]>(b"struct closure make_closure(struct jq_state *, uint16_t *)\x00")).as_ptr());
        };
        if closure < (*(*fr).bc).nclosures {
        } else {
            __assert_fail(b"closure < fr->bc->nclosures\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          126 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 59],
                                                    &[libc::c_char; 59]>(b"struct closure make_closure(struct jq_state *, uint16_t *)\x00")).as_ptr());
        };
        return (*(*fr).entries.as_mut_ptr().offset(closure as isize)).closure
    };
}
unsafe extern "C" fn frame_push(mut jq: *mut jq_state, mut callee: closure,
                                mut argdef: *mut uint16_t,
                                mut nargs: libc::c_int) -> *mut frame {
    let mut new_frame_idx: stack_ptr =
        stack_push_block(&mut (*jq).stk, (*jq).curr_frame,
                         frame_size(callee.bc) as size_t);
    let mut new_frame: *mut frame =
        stack_block(&mut (*jq).stk, new_frame_idx) as *mut frame;
    (*new_frame).bc = callee.bc;
    (*new_frame).env = callee.env;
    if nargs == (*(*new_frame).bc).nclosures {
    } else {
        __assert_fail(b"nargs == new_frame->bc->nclosures\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      137 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"struct frame *frame_push(struct jq_state *, struct closure, uint16_t *, int)\x00")).as_ptr());
    };
    let mut entries: *mut frame_entry = (*new_frame).entries.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nargs {
        (*entries).closure =
            make_closure(jq, argdef.offset((i * 2 as libc::c_int) as isize));
        entries = entries.offset(1);
        i += 1
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*callee.bc).nlocals {
        (*entries).localvar = jv_invalid();
        entries = entries.offset(1);
        i_0 += 1
    }
    (*jq).curr_frame = new_frame_idx;
    return new_frame;
}
unsafe extern "C" fn frame_pop(mut jq: *mut jq_state) {
    if (*jq).curr_frame != 0 {
    } else {
        __assert_fail(b"jq->curr_frame\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      152 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void frame_pop(struct jq_state *)\x00")).as_ptr());
    };
    let mut fp: *mut frame = frame_current(jq);
    if stack_pop_will_free(&mut (*jq).stk, (*jq).curr_frame) != 0 {
        let mut nlocals: libc::c_int = (*(*fp).bc).nlocals;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < nlocals {
            jv_free(*frame_local_var(jq, i, 0 as libc::c_int));
            i += 1
        }
    }
    (*jq).curr_frame =
        stack_pop_block(&mut (*jq).stk, (*jq).curr_frame,
                        frame_size((*fp).bc) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn stack_push(mut jq: *mut jq_state, mut val: jv) {
    if jv_is_valid(val) != 0 {
    } else {
        __assert_fail(b"jv_is_valid(val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      164 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void stack_push(jq_state *, jv)\x00")).as_ptr());
    };
    (*jq).stk_top =
        stack_push_block(&mut (*jq).stk, (*jq).stk_top,
                         ::std::mem::size_of::<jv>() as libc::c_ulong);
    let mut sval: *mut jv =
        stack_block(&mut (*jq).stk, (*jq).stk_top) as *mut jv;
    *sval = val;
}
#[no_mangle]
pub unsafe extern "C" fn stack_pop(mut jq: *mut jq_state) -> jv {
    let mut sval: *mut jv =
        stack_block(&mut (*jq).stk, (*jq).stk_top) as *mut jv;
    let mut val: jv = *sval;
    if stack_pop_will_free(&mut (*jq).stk, (*jq).stk_top) == 0 {
        val = jv_copy(val)
    }
    (*jq).stk_top =
        stack_pop_block(&mut (*jq).stk, (*jq).stk_top,
                        ::std::mem::size_of::<jv>() as libc::c_ulong);
    if jv_is_valid(val) != 0 {
    } else {
        __assert_fail(b"jv_is_valid(val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      177 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"jv stack_pop(jq_state *)\x00")).as_ptr());
    };
    return val;
}
// Like stack_pop(), but assert !stack_pop_will_free() and replace with
// jv_null() on the stack.
#[no_mangle]
pub unsafe extern "C" fn stack_popn(mut jq: *mut jq_state) -> jv {
    let mut sval: *mut jv =
        stack_block(&mut (*jq).stk, (*jq).stk_top) as *mut jv;
    let mut val: jv = *sval;
    if stack_pop_will_free(&mut (*jq).stk, (*jq).stk_top) == 0 {
        *sval = jv_null()
    }
    (*jq).stk_top =
        stack_pop_block(&mut (*jq).stk, (*jq).stk_top,
                        ::std::mem::size_of::<jv>() as libc::c_ulong);
    if jv_is_valid(val) != 0 {
    } else {
        __assert_fail(b"jv_is_valid(val)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      190 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"jv stack_popn(jq_state *)\x00")).as_ptr());
    };
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn stack_get_pos(mut jq: *mut jq_state) -> stack_pos {
    let mut sp: stack_pos =
        {
            let mut init =
                stack_pos{saved_data_stack: (*jq).stk_top,
                          saved_curr_frame: (*jq).curr_frame,};
            init
        };
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn stack_save(mut jq: *mut jq_state,
                                    mut retaddr: *mut uint16_t,
                                    mut sp: stack_pos) {
    (*jq).fork_top =
        stack_push_block(&mut (*jq).stk, (*jq).fork_top,
                         ::std::mem::size_of::<forkpoint>() as libc::c_ulong);
    let mut fork: *mut forkpoint =
        stack_block(&mut (*jq).stk, (*jq).fork_top) as *mut forkpoint;
    (*fork).saved_data_stack = (*jq).stk_top;
    (*fork).saved_curr_frame = (*jq).curr_frame;
    (*fork).path_len =
        if jv_get_kind((*jq).path) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint {
            jv_array_length(jv_copy((*jq).path))
        } else { 0 as libc::c_int };
    (*fork).value_at_path = jv_copy((*jq).value_at_path);
    (*fork).subexp_nest = (*jq).subexp_nest;
    (*fork).return_address = retaddr;
    (*jq).stk_top = sp.saved_data_stack;
    (*jq).curr_frame = sp.saved_curr_frame;
}
unsafe extern "C" fn path_intact(mut jq: *mut jq_state, mut curr: jv)
 -> libc::c_int {
    if (*jq).subexp_nest == 0 as libc::c_int &&
           jv_get_kind((*jq).path) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_identical(curr, jv_copy((*jq).value_at_path))
    } else { jv_free(curr); return 1 as libc::c_int };
}
unsafe extern "C" fn path_append(mut jq: *mut jq_state, mut component: jv,
                                 mut value_at_path: jv) {
    if (*jq).subexp_nest == 0 as libc::c_int &&
           jv_get_kind((*jq).path) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        let mut n1: libc::c_int = jv_array_length(jv_copy((*jq).path));
        (*jq).path = jv_array_append((*jq).path, component);
        let mut n2: libc::c_int = jv_array_length(jv_copy((*jq).path));
        if n2 == n1 + 1 as libc::c_int {
        } else {
            __assert_fail(b"n2 == n1 + 1\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          240 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 37],
                                                    &[libc::c_char; 37]>(b"void path_append(jq_state *, jv, jv)\x00")).as_ptr());
        };
        jv_free((*jq).value_at_path);
        (*jq).value_at_path = value_at_path
    } else { jv_free(component); jv_free(value_at_path); };
}
/* For f_getpath() */
#[no_mangle]
pub unsafe extern "C" fn _jq_path_append(mut jq: *mut jq_state, mut v: jv,
                                         mut p: jv, mut value_at_path: jv)
 -> jv {
    if (*jq).subexp_nest != 0 as libc::c_int ||
           jv_get_kind((*jq).path) as libc::c_uint !=
               JV_KIND_ARRAY as libc::c_int as libc::c_uint ||
           jv_is_valid(value_at_path) == 0 {
        jv_free(v);
        jv_free(p);
        return value_at_path
    }
    if jv_identical(v, jv_copy((*jq).value_at_path)) == 0 {
        jv_free(p);
        return value_at_path
    }
    if jv_get_kind(p) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        (*jq).path = jv_array_concat((*jq).path, p)
    } else { (*jq).path = jv_array_append((*jq).path, p) }
    jv_free((*jq).value_at_path);
    (*jq).value_at_path = value_at_path;
    return jv_copy((*jq).value_at_path);
}
#[no_mangle]
pub unsafe extern "C" fn stack_restore(mut jq: *mut jq_state)
 -> *mut uint16_t {
    while stack_pop_will_free(&mut (*jq).stk, (*jq).fork_top) == 0 {
        if stack_pop_will_free(&mut (*jq).stk, (*jq).stk_top) != 0 {
            jv_free(stack_pop(jq));
        } else if stack_pop_will_free(&mut (*jq).stk, (*jq).curr_frame) != 0 {
            frame_pop(jq);
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"src/execute.c\x00" as *const u8 as
                                  *const libc::c_char,
                              278 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 36],
                                                        &[libc::c_char; 36]>(b"uint16_t *stack_restore(jq_state *)\x00")).as_ptr());
            };
        }
    }
    if (*jq).fork_top == 0 as libc::c_int { return 0 as *mut uint16_t }
    let mut fork: *mut forkpoint =
        stack_block(&mut (*jq).stk, (*jq).fork_top) as *mut forkpoint;
    let mut retaddr: *mut uint16_t = (*fork).return_address;
    (*jq).stk_top = (*fork).saved_data_stack;
    (*jq).curr_frame = (*fork).saved_curr_frame;
    let mut path_len: libc::c_int = (*fork).path_len;
    if jv_get_kind((*jq).path) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        if path_len >= 0 as libc::c_int {
        } else {
            __assert_fail(b"path_len >= 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/execute.c\x00" as *const u8 as
                              *const libc::c_char,
                          292 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_char; 36]>(b"uint16_t *stack_restore(jq_state *)\x00")).as_ptr());
        };
        (*jq).path = jv_array_slice((*jq).path, 0 as libc::c_int, path_len)
    } else { (*fork).path_len = 0 as libc::c_int }
    jv_free((*jq).value_at_path);
    (*jq).value_at_path = (*fork).value_at_path;
    (*jq).subexp_nest = (*fork).subexp_nest;
    (*jq).fork_top =
        stack_pop_block(&mut (*jq).stk, (*jq).fork_top,
                        ::std::mem::size_of::<forkpoint>() as libc::c_ulong);
    return retaddr;
}
unsafe extern "C" fn jq_reset(mut jq: *mut jq_state) {
    while !stack_restore(jq).is_null() { }
    if (*jq).stk_top == 0 as libc::c_int {
    } else {
        __assert_fail(b"jq->stk_top == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      307 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void jq_reset(jq_state *)\x00")).as_ptr());
    };
    if (*jq).fork_top == 0 as libc::c_int {
    } else {
        __assert_fail(b"jq->fork_top == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      308 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void jq_reset(jq_state *)\x00")).as_ptr());
    };
    if (*jq).curr_frame == 0 as libc::c_int {
    } else {
        __assert_fail(b"jq->curr_frame == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      309 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void jq_reset(jq_state *)\x00")).as_ptr());
    };
    stack_reset(&mut (*jq).stk);
    jv_free((*jq).error);
    (*jq).error = jv_null();
    (*jq).halted = 0 as libc::c_int;
    jv_free((*jq).exit_code);
    jv_free((*jq).error_message);
    if jv_get_kind((*jq).path) as libc::c_uint !=
           JV_KIND_INVALID as libc::c_int as libc::c_uint {
        jv_free((*jq).path);
    }
    (*jq).path = jv_null();
    jv_free((*jq).value_at_path);
    (*jq).value_at_path = jv_null();
    (*jq).subexp_nest = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jq_report_error(mut jq: *mut jq_state,
                                         mut value: jv) {
    if (*jq).err_cb.is_some() {
    } else {
        __assert_fail(b"jq->err_cb\x00" as *const u8 as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      326 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void jq_report_error(jq_state *, jv)\x00")).as_ptr());
    };
    // callback must jv_free() its jv argument
    (*jq).err_cb.expect("non-null function pointer")((*jq).err_cb_data,
                                                     value);
}
unsafe extern "C" fn set_error(mut jq: *mut jq_state, mut value: jv) {
    // Record so try/catch can find it.
    jv_free((*jq).error);
    (*jq).error = value;
}
#[no_mangle]
pub unsafe extern "C" fn jq_next(mut jq: *mut jq_state) -> jv {
    let mut current_block: u64;
    let mut cfunc_input: [jv; 10] =
        [jv{kind_flags: 0,
            pad_: 0,
            offset: 0,
            size: 0,
            u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},}; 10];
    jv_nomem_handler((*jq).nomem_handler, (*jq).nomem_handler_data);
    let mut pc: *mut uint16_t = stack_restore(jq);
    if !pc.is_null() {
    } else {
        __assert_fail(b"pc\x00" as *const u8 as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      345 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
    };
    let mut raising: libc::c_int = 0;
    let mut backtracking: libc::c_int =
        ((*jq).initial_execution == 0) as libc::c_int;
    (*jq).initial_execution = 0 as libc::c_int;
    if jv_get_kind((*jq).error) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(jq->error) == JV_KIND_NULL\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      350 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
    };
    loop  {
        if (*jq).halted != 0 {
            if (*jq).debug_trace_enabled != 0 {
                printf(b"\t<halted>\n\x00" as *const u8 as
                           *const libc::c_char);
            }
            return jv_invalid()
        }
        let mut opcode: uint16_t = *pc;
        raising = 0 as libc::c_int;
        if (*jq).debug_trace_enabled != 0 {
            dump_operation((*frame_current(jq)).bc, pc);
            printf(b"\t\x00" as *const u8 as *const libc::c_char);
            let mut opdesc: *const opcode_description =
                opcode_describe(opcode as opcode);
            let mut param: stack_ptr = 0 as libc::c_int;
            if backtracking == 0 {
                let mut stack_in: libc::c_int = (*opdesc).stack_in;
                if stack_in == -(1 as libc::c_int) {
                    stack_in =
                        *pc.offset(1 as libc::c_int as isize) as libc::c_int
                }
                param = (*jq).stk_top;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < stack_in {
                    if i != 0 as libc::c_int {
                        printf(b" | \x00" as *const u8 as
                                   *const libc::c_char);
                        param = *stack_block_next(&mut (*jq).stk, param)
                    }
                    if param == 0 { break ; }
                    jv_dump(jv_copy(*(stack_block(&mut (*jq).stk, param) as
                                          *mut jv)),
                            JV_PRINT_REFCOUNT as libc::c_int);
                    i += 1
                    //printf("<%d>", jv_get_refcnt(param->val));
          //printf(" -- ");
          //jv_dump(jv_copy(jq->path), 0);
                }
                if (*jq).debug_trace_enabled &
                       JQ_DEBUG_TRACE_DETAIL as libc::c_int != 0 {
                    loop  {
                        param = *stack_block_next(&mut (*jq).stk, param);
                        if !(param != 0) { break ; }
                        printf(b" || \x00" as *const u8 as
                                   *const libc::c_char);
                        jv_dump(jv_copy(*(stack_block(&mut (*jq).stk, param)
                                              as *mut jv)),
                                JV_PRINT_REFCOUNT as libc::c_int);
                    }
                }
            } else {
                printf(b"\t<backtracking>\x00" as *const u8 as
                           *const libc::c_char);
            }
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        if backtracking != 0 {
            opcode =
                (opcode as libc::c_int + NUM_OPCODES as libc::c_int) as
                    uint16_t;
            backtracking = 0 as libc::c_int;
            raising = (jv_is_valid((*jq).error) == 0) as libc::c_int
        }
        pc = pc.offset(1);
        match opcode as libc::c_int {
            34 => { continue ; }
            0 => {
                let fresh2 = pc;
                pc = pc.offset(1);
                let mut v: jv =
                    jv_array_get(jv_copy((*(*frame_current(jq)).bc).constants),
                                 *fresh2 as libc::c_int);
                if jv_is_valid(v) != 0 {
                } else {
                    __assert_fail(b"jv_is_valid(v)\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  407 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                jv_free(stack_pop(jq));
                stack_push(jq, v);
                continue ;
            }
            38 => {
                let fresh3 = (*jq).next_label;
                (*jq).next_label = (*jq).next_label.wrapping_add(1);
                stack_push(jq,
                           jv_object_set(jv_object(),
                                         jv_string(b"__jq\x00" as *const u8 as
                                                       *const libc::c_char),
                                         jv_number(fresh3 as
                                                       libc::c_double)));
                continue ;
            }
            1 => {
                let mut v_0: jv = stack_pop(jq);
                stack_push(jq, jv_copy(v_0));
                stack_push(jq, v_0);
                continue ;
            }
            2 => {
                let mut v_1: jv = stack_popn(jq);
                stack_push(jq, jv_copy(v_1));
                stack_push(jq, v_1);
                continue ;
            }
            3 => {
                let mut keep: jv = stack_pop(jq);
                let mut v_2: jv = stack_pop(jq);
                stack_push(jq, jv_copy(v_2));
                stack_push(jq, keep);
                stack_push(jq, v_2);
                continue ;
            }
            22 => {
                let mut v_3: jv = stack_pop(jq);
                stack_push(jq, jv_copy(v_3));
                stack_push(jq, v_3);
                (*jq).subexp_nest += 1;
                continue ;
            }
            23 => {
                if (*jq).subexp_nest > 0 as libc::c_int {
                } else {
                    __assert_fail(b"jq->subexp_nest > 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  450 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                (*jq).subexp_nest -= 1;
                let mut a: jv = stack_pop(jq);
                let mut b: jv = stack_pop(jq);
                stack_push(jq, a);
                stack_push(jq, b);
                continue ;
            }
            4 => {
                let fresh4 = pc;
                pc = pc.offset(1);
                let mut v_4: jv =
                    jv_array_get(jv_copy((*(*frame_current(jq)).bc).constants),
                                 *fresh4 as libc::c_int);
                if jv_is_valid(v_4) != 0 {
                } else {
                    __assert_fail(b"jv_is_valid(v)\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  461 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                let mut v2: jv = stack_pop(jq);
                stack_push(jq, v_4);
                stack_push(jq, v2);
                continue ;
            }
            5 => { jv_free(stack_pop(jq)); continue ; }
            19 => {
                let mut v_5: jv = stack_pop(jq);
                let fresh5 = pc;
                pc = pc.offset(1);
                let mut level: uint16_t = *fresh5;
                let fresh6 = pc;
                pc = pc.offset(1);
                let mut vidx: uint16_t = *fresh6;
                let mut var: *mut jv =
                    frame_local_var(jq, vidx as libc::c_int,
                                    level as libc::c_int);
                if jv_get_kind(*var) as libc::c_uint ==
                       JV_KIND_ARRAY as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"jv_get_kind(*var) == JV_KIND_ARRAY\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  478 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                *var = jv_array_append(*var, v_5);
                continue ;
            }
            20 => {
                let mut stktop: jv = stack_pop(jq);
                let mut v_6: jv = stack_pop(jq);
                let mut k: jv = stack_pop(jq);
                let mut objv: jv = stack_pop(jq);
                if jv_get_kind(objv) as libc::c_uint ==
                       JV_KIND_OBJECT as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"jv_get_kind(objv) == JV_KIND_OBJECT\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  488 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                if jv_get_kind(k) as libc::c_uint ==
                       JV_KIND_STRING as libc::c_int as libc::c_uint {
                    stack_push(jq, jv_object_set(objv, k, v_6));
                    stack_push(jq, stktop);
                    continue ;
                } else {
                    let mut errbuf: [libc::c_char; 15] = [0; 15];
                    set_error(jq,
                              jv_invalid_with_msg(jv_string_fmt(b"Cannot use %s (%s) as object key\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                jv_kind_name(jv_get_kind(k)),
                                                                jv_dump_string_trunc(jv_copy(k),
                                                                                     errbuf.as_mut_ptr(),
                                                                                     ::std::mem::size_of::<[libc::c_char; 15]>()
                                                                                         as
                                                                                         libc::c_ulong))));
                    jv_free(stktop);
                    jv_free(v_6);
                    jv_free(k);
                    jv_free(objv);
                }
                current_block = 13493279574219925475;
            }
            62 | 21 => {
                let fresh7 = pc;
                pc = pc.offset(1);
                let mut level_0: uint16_t = *fresh7;
                let fresh8 = pc;
                pc = pc.offset(1);
                let mut v_7: uint16_t = *fresh8;
                let mut var_0: *mut jv =
                    frame_local_var(jq, v_7 as libc::c_int,
                                    level_0 as libc::c_int);
                let mut max: jv = stack_pop(jq);
                if raising != 0 {
                    jv_free(max);
                } else if jv_get_kind(*var_0) as libc::c_uint !=
                              JV_KIND_NUMBER as libc::c_int as libc::c_uint ||
                              jv_get_kind(max) as libc::c_uint !=
                                  JV_KIND_NUMBER as libc::c_int as
                                      libc::c_uint {
                    set_error(jq,
                              jv_invalid_with_msg(jv_string_fmt(b"Range bounds must be numeric\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char)));
                    jv_free(max);
                } else if jv_number_value(*var_0) >= jv_number_value(max) {
                    /* finished iterating */
                    jv_free(max);
                } else {
                    let mut curr: jv = *var_0;
                    *var_0 =
                        jv_number(jv_number_value(*var_0) +
                                      1 as libc::c_int as libc::c_double);
                    let mut spos: stack_pos = stack_get_pos(jq);
                    stack_push(jq, max);
                    stack_save(jq, pc.offset(-(3 as libc::c_int as isize)),
                               spos);
                    stack_push(jq, curr);
                    continue ;
                }
                current_block = 13493279574219925475;
            }
            6 => {
                // FIXME: loadv/storev may do too much copying/freeing
                let fresh9 = pc;
                pc = pc.offset(1);
                let mut level_1: uint16_t = *fresh9;
                let fresh10 = pc;
                pc = pc.offset(1);
                let mut v_8: uint16_t = *fresh10;
                let mut var_1: *mut jv =
                    frame_local_var(jq, v_8 as libc::c_int,
                                    level_1 as libc::c_int);
                if (*jq).debug_trace_enabled != 0 {
                    printf(b"V%d = \x00" as *const u8 as *const libc::c_char,
                           v_8 as libc::c_int);
                    jv_dump(jv_copy(*var_1), 0 as libc::c_int);
                    printf(b" (%d)\n\x00" as *const u8 as *const libc::c_char,
                           jv_get_refcnt(*var_1));
                }
                jv_free(stack_pop(jq));
                stack_push(jq, jv_copy(*var_1));
                continue ;
            }
            7 => {
                // Does a load but replaces the variable with null
                let fresh11 = pc;
                pc = pc.offset(1);
                let mut level_2: uint16_t = *fresh11;
                let fresh12 = pc;
                pc = pc.offset(1);
                let mut v_9: uint16_t = *fresh12;
                let mut var_2: *mut jv =
                    frame_local_var(jq, v_9 as libc::c_int,
                                    level_2 as libc::c_int);
                if (*jq).debug_trace_enabled != 0 {
                    printf(b"V%d = \x00" as *const u8 as *const libc::c_char,
                           v_9 as libc::c_int);
                    jv_dump(jv_copy(*var_2), 0 as libc::c_int);
                    printf(b" (%d)\n\x00" as *const u8 as *const libc::c_char,
                           jv_get_refcnt(*var_2));
                }
                jv_free(stack_popn(jq));
                stack_push(jq, *var_2);
                *var_2 = jv_null();
                continue ;
            }
            40 => {
                stack_save(jq, pc.offset(-(1 as libc::c_int as isize)),
                           stack_get_pos(jq));
                current_block = 12693738997172594219;
            }
            8 => { current_block = 12693738997172594219; }
            81 => {
                let fresh15 = pc;
                pc = pc.offset(1);
                let mut level_4: uint16_t = *fresh15;
                let fresh16 = pc;
                pc = pc.offset(1);
                let mut v_11: uint16_t = *fresh16;
                let mut var_4: *mut jv =
                    frame_local_var(jq, v_11 as libc::c_int,
                                    level_4 as libc::c_int);
                jv_free(*var_4);
                *var_4 = jv_null();
                current_block = 13493279574219925475;
            }
            9 => {
                // Get the constant
                let fresh17 = pc;
                pc = pc.offset(1);
                let mut val_0: jv =
                    jv_array_get(jv_copy((*(*frame_current(jq)).bc).constants),
                                 *fresh17 as libc::c_int);
                if jv_is_valid(val_0) != 0 {
                } else {
                    __assert_fail(b"jv_is_valid(val)\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  599 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                // Store the var
                let fresh18 = pc; // next INDEX operation must index into v
                pc = pc.offset(1);
                let mut level_5: uint16_t = *fresh18;
                let fresh19 = pc;
                pc = pc.offset(1);
                let mut v_12: uint16_t = *fresh19;
                let mut var_5: *mut jv =
                    frame_local_var(jq, v_12 as libc::c_int,
                                    level_5 as libc::c_int);
                if (*jq).debug_trace_enabled != 0 {
                    printf(b"V%d = \x00" as *const u8 as *const libc::c_char,
                           v_12 as libc::c_int);
                    jv_dump(jv_copy(val_0), 0 as libc::c_int);
                    printf(b" (%d)\n\x00" as *const u8 as *const libc::c_char,
                           jv_get_refcnt(val_0));
                }
                jv_free(*var_5);
                *var_5 = val_0;
                continue ;
            }
            24 => {
                let mut v_13: jv = stack_pop(jq);
                stack_push(jq, (*jq).path);
                stack_save(jq, pc.offset(-(1 as libc::c_int as isize)),
                           stack_get_pos(jq));
                stack_push(jq,
                           jv_number((*jq).subexp_nest as libc::c_double));
                stack_push(jq, (*jq).value_at_path);
                stack_push(jq, jv_copy(v_13));
                (*jq).path = jv_array();
                (*jq).value_at_path = v_13;
                (*jq).subexp_nest = 0 as libc::c_int;
                continue ;
            }
            25 => {
                let mut v_14: jv = stack_pop(jq);
                // detect invalid path expression like path(.a | reverse)
                if path_intact(jq, jv_copy(v_14)) == 0 {
                    let mut errbuf_0: [libc::c_char; 30] =
                        [0; 30]; // discard value, only keep path
                    let mut msg: jv =
                        jv_string_fmt(b"Invalid path expression with result %s\x00"
                                          as *const u8 as *const libc::c_char,
                                      jv_dump_string_trunc(v_14,
                                                           errbuf_0.as_mut_ptr(),
                                                           ::std::mem::size_of::<[libc::c_char; 30]>()
                                                               as
                                                               libc::c_ulong));
                    set_error(jq, jv_invalid_with_msg(msg));
                } else {
                    jv_free(v_14);
                    let mut old_value_at_path: jv = stack_pop(jq);
                    let mut old_subexp_nest: libc::c_int =
                        jv_number_value(stack_pop(jq)) as libc::c_int;
                    let mut path: jv = (*jq).path;
                    (*jq).path = stack_pop(jq);
                    let mut spos_0: stack_pos = stack_get_pos(jq);
                    stack_push(jq, jv_copy(path));
                    stack_save(jq, pc.offset(-(1 as libc::c_int as isize)),
                               spos_0);
                    stack_push(jq, path);
                    (*jq).subexp_nest = old_subexp_nest;
                    jv_free((*jq).value_at_path);
                    (*jq).value_at_path = old_value_at_path;
                    continue ;
                }
                current_block = 13493279574219925475;
            }
            65 | 66 => {
                jv_free((*jq).path);
                (*jq).path = stack_pop(jq);
                current_block = 13493279574219925475;
            }
            10 | 11 => {
                let mut t: jv = stack_pop(jq);
                let mut k_0: jv = stack_pop(jq);
                // detect invalid path expression like path(reverse | .a)
                if path_intact(jq, jv_copy(t)) == 0 {
                    let mut keybuf: [libc::c_char; 15] =
                        [0; 15]; // FIXME do this better
                    let mut objbuf: [libc::c_char; 30] = [0; 30];
                    let mut msg_0: jv =
                        jv_string_fmt(b"Invalid path expression near attempt to access element %s of %s\x00"
                                          as *const u8 as *const libc::c_char,
                                      jv_dump_string_trunc(k_0,
                                                           keybuf.as_mut_ptr(),
                                                           ::std::mem::size_of::<[libc::c_char; 15]>()
                                                               as
                                                               libc::c_ulong),
                                      jv_dump_string_trunc(t,
                                                           objbuf.as_mut_ptr(),
                                                           ::std::mem::size_of::<[libc::c_char; 30]>()
                                                               as
                                                               libc::c_ulong));
                    set_error(jq, jv_invalid_with_msg(msg_0));
                } else {
                    let mut v_15: jv = jv_get(t, jv_copy(k_0));
                    if jv_is_valid(v_15) != 0 {
                        path_append(jq, k_0, jv_copy(v_15));
                        stack_push(jq, v_15);
                        continue ;
                    } else {
                        jv_free(k_0);
                        if opcode as libc::c_int == INDEX as libc::c_int {
                            set_error(jq, v_15);
                        } else { jv_free(v_15); }
                    }
                }
                current_block = 13493279574219925475;
            }
            16 => {
                let fresh20 = pc;
                pc = pc.offset(1);
                let mut offset: uint16_t = *fresh20;
                pc = pc.offset(offset as libc::c_int as isize);
                continue ;
            }
            17 => {
                let fresh21 = pc;
                pc = pc.offset(1);
                let mut offset_0: uint16_t = *fresh21;
                let mut t_0: jv = stack_pop(jq);
                let mut kind: jv_kind = jv_get_kind(t_0);
                if kind as libc::c_uint ==
                       JV_KIND_FALSE as libc::c_int as libc::c_uint ||
                       kind as libc::c_uint ==
                           JV_KIND_NULL as libc::c_int as libc::c_uint {
                    pc = pc.offset(offset_0 as libc::c_int as isize)
                }
                stack_push(jq, t_0);
                continue ;
            }
            12 | 13 => {
                let mut container: jv = stack_pop(jq);
                // fallthrough
                if path_intact(jq, jv_copy(container)) == 0 {
                    let mut errbuf_1: [libc::c_char; 30] = [0; 30];
                    let mut msg_1: jv =
                        jv_string_fmt(b"Invalid path expression near attempt to iterate through %s\x00"
                                          as *const u8 as *const libc::c_char,
                                      jv_dump_string_trunc(container,
                                                           errbuf_1.as_mut_ptr(),
                                                           ::std::mem::size_of::<[libc::c_char; 30]>()
                                                               as
                                                               libc::c_ulong));
                    set_error(jq, jv_invalid_with_msg(msg_1));
                    current_block = 13493279574219925475;
                } else {
                    stack_push(jq, container);
                    stack_push(jq,
                               jv_number(-(1 as libc::c_int) as
                                             libc::c_double));
                    current_block = 15689130159044723804;
                }
            }
            53 | 54 => { current_block = 15689130159044723804; }
            15 | 39 | 14 => {
                stack_save(jq, pc.offset(-(1 as libc::c_int as isize)),
                           stack_get_pos(jq));
                // detect invalid path expression like path(reverse | .[])
                pc = pc.offset(1); // skip offset this time
                continue ;
            }
            56 | 80 => {
                if jv_is_valid((*jq).error) != 0 {
                    // `try EXP ...` backtracked here (no value, `empty`), so we backtrack more
                    jv_free(stack_pop(jq));
                } else {
                    // `try EXP ...` exception caught in EXP
      // DESTRUCTURE_ALT doesn't want the error message on the stack,
      // as we would just want to throw it away anyway.
                    if opcode as libc::c_int !=
                           DESTRUCTURE_ALT as libc::c_int +
                               NUM_OPCODES as libc::c_int {
                        jv_free(stack_pop(jq));
                        stack_push(jq,
                                   jv_invalid_get_msg((*jq).error)); // free the input
                        // push the error's message
                    } else { jv_free((*jq).error); }
                    (*jq).error = jv_null();
                    let fresh22 = pc;
                    pc = pc.offset(1);
                    let mut offset_1: uint16_t = *fresh22;
                    pc = pc.offset(offset_1 as libc::c_int as isize);
                    continue ;
                }
                current_block = 13493279574219925475;
            }
            55 => {
                if raising != 0 {
                    current_block = 13493279574219925475;
                } else {
                    let fresh23 = pc;
                    pc = pc.offset(1);
                    let mut offset_2: uint16_t = *fresh23;
                    pc = pc.offset(offset_2 as libc::c_int as isize);
                    continue ;
                }
            }
            26 => {
                let fresh24 = pc;
                pc = pc.offset(1);
                let mut nargs: libc::c_int = *fresh24 as libc::c_int;
                let mut top: jv = stack_pop(jq);
                let mut in_0: *mut jv = cfunc_input.as_mut_ptr();
                *in_0.offset(0 as libc::c_int as isize) = top;
                let mut i_0: libc::c_int = 1 as libc::c_int;
                while i_0 < nargs {
                    *in_0.offset(i_0 as isize) = stack_pop(jq);
                    i_0 += 1
                }
                let fresh25 = pc;
                pc = pc.offset(1);
                let mut function: *mut cfunction =
                    &mut *(*(*(*(frame_current as
                                     unsafe extern "C" fn(_: *mut jq_state)
                                         ->
                                             *mut frame)(jq)).bc).globals).cfunctions.offset(*fresh25
                                                                                                 as
                                                                                                 isize)
                        as *mut cfunction;
                match (*function).nargs {
                    1 => {
                        top =
                            ::std::mem::transmute::<cfunction_ptr,
                                                    func_1>((*function).fptr).expect("non-null function pointer")(jq,
                                                                                                                  *in_0.offset(0
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize))
                    }
                    2 => {
                        top =
                            ::std::mem::transmute::<cfunction_ptr,
                                                    func_2>((*function).fptr).expect("non-null function pointer")(jq,
                                                                                                                  *in_0.offset(0
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize))
                    }
                    3 => {
                        top =
                            ::std::mem::transmute::<cfunction_ptr,
                                                    func_3>((*function).fptr).expect("non-null function pointer")(jq,
                                                                                                                  *in_0.offset(0
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(2
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize))
                    }
                    4 => {
                        top =
                            ::std::mem::transmute::<cfunction_ptr,
                                                    func_4>((*function).fptr).expect("non-null function pointer")(jq,
                                                                                                                  *in_0.offset(0
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(2
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(3
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize))
                    }
                    5 => {
                        top =
                            ::std::mem::transmute::<cfunction_ptr,
                                                    func_5>((*function).fptr).expect("non-null function pointer")(jq,
                                                                                                                  *in_0.offset(0
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(2
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(3
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize),
                                                                                                                  *in_0.offset(4
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize))
                    }
                    _ => {
                        // FIXME: a) up to 7 arguments (input + 6), b) should assert
      // because the compiler should not generate this error.
                        return jv_invalid_with_msg(jv_string(b"Function takes too many arguments\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char))
                    }
                }
                if jv_is_valid(top) != 0 {
                    stack_push(jq, top);
                    continue ;
                } else if jv_invalid_has_msg(jv_copy(top)) != 0 {
                    set_error(jq, top);
                }
                current_block = 13493279574219925475;
            }
            29 | 27 => {
                /*
       * Bytecode layout here:
       *
       *  CALL_JQ
       *  <nclosures>                       (i.e., number of call arguments)
       *  <callee closure>                  (what we're calling)
       *  <nclosures' worth of closures>    (frame reference + code pointer)
       *
       *  <next instruction (to return to)>
       *
       * Each closure consists of two uint16_t values: a "level"
       * identifying the frame to be closed over, and an index.
       *
       * The level is a relative number of call frames reachable from
       * the currently one; 0 -> current frame, 1 -> previous frame, and
       * so on.
       *
       * The index is either an index of the closed frame's subfunctions
       * or of the closed frame's parameter closures.  If the latter,
       * that closure will be passed, else the closed frame's pointer
       * and the subfunction's code will form the closure to be passed.
       *
       * See make_closure() for more information.
       */
                let mut input: jv = stack_pop(jq);
                let fresh26 = pc;
                pc = pc.offset(1);
                let mut nclosures: uint16_t = *fresh26;
                let mut retaddr: *mut uint16_t =
                    pc.offset(2 as libc::c_int as
                                  isize).offset((nclosures as libc::c_int *
                                                     2 as libc::c_int) as
                                                    isize);
                let mut retdata: stack_ptr = (*jq).stk_top;
                let mut new_frame: *mut frame = 0 as *mut frame;
                let mut cl: closure = make_closure(jq, pc);
                if opcode as libc::c_int == TAIL_CALL_JQ as libc::c_int {
                    retaddr = (*frame_current(jq)).retaddr;
                    retdata = (*frame_current(jq)).retdata;
                    frame_pop(jq);
                }
                new_frame =
                    frame_push(jq, cl, pc.offset(2 as libc::c_int as isize),
                               nclosures as libc::c_int);
                (*new_frame).retdata = retdata;
                (*new_frame).retaddr = retaddr;
                pc = (*(*new_frame).bc).code;
                stack_push(jq, input);
                continue ;
            }
            28 => {
                let mut value_0: jv = stack_pop(jq);
                if (*jq).stk_top == (*frame_current(jq)).retdata {
                } else {
                    __assert_fail(b"jq->stk_top == frame_current(jq)->retdata\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  926 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                let mut retaddr_0: *mut uint16_t =
                    (*frame_current(jq)).retaddr;
                if !retaddr_0.is_null() {
                    // function return
                    pc = retaddr_0;
                    frame_pop(jq);
                } else {
                    // top-level return, yielding value
                    let mut spos_2: stack_pos = stack_get_pos(jq);
                    stack_push(jq, jv_null());
                    stack_save(jq, pc.offset(-(1 as libc::c_int as isize)),
                               spos_2);
                    return value_0
                }
                stack_push(jq, value_0);
                continue ;
            }
            18 | 69 => { current_block = 13493279574219925475; }
            _ => {
                if 0 as libc::c_int != 0 &&
                       !(b"invalid instruction\x00" as *const u8 as
                             *const libc::c_char).is_null() {
                } else {
                    __assert_fail(b"0 && \"invalid instruction\"\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"src/execute.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  401 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 23],
                                                            &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                };
                continue ;
            }
        }
        match current_block {
            15689130159044723804 => {
                let mut idx: libc::c_int =
                    jv_number_value(stack_pop(jq)) as libc::c_int;
                let mut container_0: jv = stack_pop(jq);
                let mut keep_going: libc::c_int = 0;
                let mut is_last: libc::c_int = 0 as libc::c_int;
                let mut key: jv =
                    jv{kind_flags: 0,
                       pad_: 0,
                       offset: 0,
                       size: 0,
                       u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                let mut value: jv =
                    jv{kind_flags: 0,
                       pad_: 0,
                       offset: 0,
                       size: 0,
                       u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                if jv_get_kind(container_0) as libc::c_uint ==
                       JV_KIND_ARRAY as libc::c_int as libc::c_uint {
                    if opcode as libc::c_int == EACH as libc::c_int ||
                           opcode as libc::c_int == EACH_OPT as libc::c_int {
                        idx = 0 as libc::c_int
                    } else { idx = idx + 1 as libc::c_int }
                    let mut len: libc::c_int =
                        jv_array_length(jv_copy(container_0));
                    keep_going = (idx < len) as libc::c_int;
                    is_last = (idx == len - 1 as libc::c_int) as libc::c_int;
                    if keep_going != 0 {
                        key = jv_number(idx as libc::c_double);
                        value = jv_array_get(jv_copy(container_0), idx)
                    }
                } else if jv_get_kind(container_0) as libc::c_uint ==
                              JV_KIND_OBJECT as libc::c_int as libc::c_uint {
                    if opcode as libc::c_int == EACH as libc::c_int ||
                           opcode as libc::c_int == EACH_OPT as libc::c_int {
                        idx = jv_object_iter(container_0)
                    } else { idx = jv_object_iter_next(container_0, idx) }
                    keep_going = jv_object_iter_valid(container_0, idx);
                    if keep_going != 0 {
                        key = jv_object_iter_key(container_0, idx);
                        value = jv_object_iter_value(container_0, idx)
                    }
                } else {
                    if opcode as libc::c_int == EACH as libc::c_int ||
                           opcode as libc::c_int == EACH_OPT as libc::c_int {
                    } else {
                        __assert_fail(b"opcode == EACH || opcode == EACH_OPT\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/execute.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      758 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 23],
                                                                &[libc::c_char; 23]>(b"jv jq_next(jq_state *)\x00")).as_ptr());
                    };
                    if opcode as libc::c_int == EACH as libc::c_int {
                        let mut errbuf_2: [libc::c_char; 15] = [0; 15];
                        set_error(jq,
                                  jv_invalid_with_msg(jv_string_fmt(b"Cannot iterate over %s (%s)\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    jv_kind_name(jv_get_kind(container_0)),
                                                                    jv_dump_string_trunc(jv_copy(container_0),
                                                                                         errbuf_2.as_mut_ptr(),
                                                                                         ::std::mem::size_of::<[libc::c_char; 15]>()
                                                                                             as
                                                                                             libc::c_ulong))));
                    }
                    keep_going = 0 as libc::c_int
                }
                if keep_going == 0 || raising != 0 {
                    if keep_going != 0 { jv_free(value); }
                    jv_free(container_0);
                } else {
                    if is_last != 0 {
                        // we don't need to make a backtrack point
                        jv_free(container_0);
                        path_append(jq, key, jv_copy(value));
                        stack_push(jq, value);
                    } else {
                        let mut spos_1: stack_pos = stack_get_pos(jq);
                        stack_push(jq, container_0);
                        stack_push(jq, jv_number(idx as libc::c_double));
                        stack_save(jq,
                                   pc.offset(-(1 as libc::c_int as isize)),
                                   spos_1);
                        path_append(jq, key, jv_copy(value));
                        stack_push(jq, value);
                    }
                    continue ;
                }
            }
            12693738997172594219 => {
                let fresh13 = pc;
                pc = pc.offset(1);
                let mut level_3: uint16_t = *fresh13;
                let fresh14 = pc;
                pc = pc.offset(1);
                let mut v_10: uint16_t = *fresh14;
                let mut var_3: *mut jv =
                    frame_local_var(jq, v_10 as libc::c_int,
                                    level_3 as libc::c_int);
                let mut val: jv = stack_pop(jq);
                if (*jq).debug_trace_enabled != 0 {
                    printf(b"V%d = \x00" as *const u8 as *const libc::c_char,
                           v_10 as libc::c_int);
                    jv_dump(jv_copy(val), 0 as libc::c_int);
                    printf(b" (%d)\n\x00" as *const u8 as *const libc::c_char,
                           jv_get_refcnt(val));
                }
                jv_free(*var_3);
                *var_3 = val;
                continue ;
            }
            _ => { }
        }
        // resumed after top-level return
        pc = stack_restore(jq); // expected to already be formatted
        if pc.is_null() {
            if jv_is_valid((*jq).error) == 0 {
                let mut error: jv = (*jq).error; // ENOMEM
                (*jq).error = jv_null();
                return error
            }
            return jv_invalid()
        }
        backtracking = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn jq_format_error(mut msg: jv) -> jv {
    if jv_get_kind(msg) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint ||
           jv_get_kind(msg) as libc::c_uint ==
               JV_KIND_INVALID as libc::c_int as libc::c_uint &&
               jv_invalid_has_msg(jv_copy(msg)) == 0 {
        jv_free(msg);
        fprintf(stderr,
                b"jq: error: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return jv_null()
    }
    if jv_get_kind(msg) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint {
        return msg
    }
    if jv_get_kind(msg) as libc::c_uint ==
           JV_KIND_INVALID as libc::c_int as libc::c_uint {
        msg = jv_invalid_get_msg(msg)
    }
    if jv_get_kind(msg) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint {
        return jq_format_error(msg)
    }
    // Invalid with msg; prefix with "jq: error: "
    if jv_get_kind(msg) as libc::c_uint !=
           JV_KIND_INVALID as libc::c_int as libc::c_uint {
        if jv_get_kind(msg) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            return jv_string_fmt(b"jq: error: %s\x00" as *const u8 as
                                     *const libc::c_char,
                                 jv_string_value(msg))
        }
        msg = jv_dump_string(msg, JV_PRINT_INVALID as libc::c_int);
        if jv_get_kind(msg) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            return jv_string_fmt(b"jq: error: %s\x00" as *const u8 as
                                     *const libc::c_char,
                                 jv_string_value(msg))
        }
        return jq_format_error(jv_null())
        // ENOMEM
    }
    // An invalid inside an invalid!
    return jq_format_error(jv_invalid_get_msg(msg));
}
// XXX Refactor into a utility function that returns a jv and one that
// uses it and then prints that jv's string as the complete error
// message.
unsafe extern "C" fn default_err_cb(mut data: *mut libc::c_void,
                                    mut msg: jv) {
    msg = jq_format_error(msg);
    fprintf(data as *mut FILE,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            jv_string_value(msg));
    jv_free(msg);
}
#[no_mangle]
pub unsafe extern "C" fn jq_init() -> *mut jq_state {
    let mut jq: *mut jq_state = 0 as *mut jq_state;
    jq =
        jv_mem_alloc_unguarded(::std::mem::size_of::<jq_state>() as
                                   libc::c_ulong) as *mut jq_state;
    if jq.is_null() { return 0 as *mut jq_state }
    (*jq).bc = 0 as *mut bytecode;
    (*jq).next_label = 0 as libc::c_int as libc::c_uint;
    stack_init(&mut (*jq).stk);
    (*jq).stk_top = 0 as libc::c_int;
    (*jq).fork_top = 0 as libc::c_int;
    (*jq).curr_frame = 0 as libc::c_int;
    (*jq).error = jv_null();
    (*jq).halted = 0 as libc::c_int;
    (*jq).exit_code = jv_invalid();
    (*jq).error_message = jv_invalid();
    (*jq).err_cb =
        Some(default_err_cb as
                 unsafe extern "C" fn(_: *mut libc::c_void, _: jv) -> ());
    (*jq).err_cb_data = stderr as *mut libc::c_void;
    (*jq).attrs = jv_object();
    (*jq).path = jv_null();
    (*jq).value_at_path = jv_null();
    (*jq).nomem_handler = None;
    (*jq).nomem_handler_data = 0 as *mut libc::c_void;
    return jq;
}
#[no_mangle]
pub unsafe extern "C" fn jq_set_error_cb(mut jq: *mut jq_state,
                                         mut cb: jq_msg_cb,
                                         mut data: *mut libc::c_void) {
    if cb.is_none() {
        (*jq).err_cb =
            Some(default_err_cb as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: jv) -> ());
        (*jq).err_cb_data = stderr as *mut libc::c_void
    } else { (*jq).err_cb = cb; (*jq).err_cb_data = data };
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_error_cb(mut jq: *mut jq_state,
                                         mut cb: *mut jq_msg_cb,
                                         mut data: *mut *mut libc::c_void) {
    *cb = (*jq).err_cb;
    *data = (*jq).err_cb_data;
}
#[no_mangle]
pub unsafe extern "C" fn jq_set_nomem_handler(mut jq: *mut jq_state,
                                              mut nomem_handler:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void)
                                                             -> ()>,
                                              mut data: *mut libc::c_void) {
    jv_nomem_handler(nomem_handler, data);
    (*jq).nomem_handler = nomem_handler;
    (*jq).nomem_handler_data = data;
}
#[no_mangle]
pub unsafe extern "C" fn jq_start(mut jq: *mut jq_state, mut input: jv,
                                  mut flags: libc::c_int) {
    jv_nomem_handler((*jq).nomem_handler, (*jq).nomem_handler_data);
    jq_reset(jq);
    let mut top: closure =
        {
            let mut init = closure{bc: (*jq).bc, env: -(1 as libc::c_int),};
            init
        };
    let mut top_frame: *mut frame =
        frame_push(jq, top, 0 as *mut uint16_t, 0 as libc::c_int);
    (*top_frame).retdata = 0 as libc::c_int;
    (*top_frame).retaddr = 0 as *mut uint16_t;
    stack_push(jq, input);
    stack_save(jq, (*(*jq).bc).code, stack_get_pos(jq));
    (*jq).debug_trace_enabled = flags & JQ_DEBUG_TRACE_ALL as libc::c_int;
    (*jq).initial_execution = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jq_teardown(mut jq: *mut *mut jq_state) {
    let mut old_jq: *mut jq_state = *jq;
    if old_jq.is_null() { return }
    *jq = 0 as *mut jq_state;
    jq_reset(old_jq);
    bytecode_free((*old_jq).bc);
    (*old_jq).bc = 0 as *mut bytecode;
    jv_free((*old_jq).attrs);
    jv_mem_free(old_jq as *mut libc::c_void);
}
unsafe extern "C" fn ret_follows(mut pc: *mut uint16_t) -> libc::c_int {
    if *pc as libc::c_int == RET as libc::c_int { return 1 as libc::c_int }
    let fresh27 = pc;
    pc = pc.offset(1);
    if *fresh27 as libc::c_int != JUMP as libc::c_int {
        return 0 as libc::c_int
    }
    return ret_follows(pc.offset(*pc as libc::c_int as
                                     isize).offset(1 as libc::c_int as
                                                       isize));
    // FIXME, might be ironic
}
/*
 * Look for tail calls that can be optimized: tail calls with no
 * references left to the current frame.
 *
 * We're staring at this bytecode layout:
 *
 *   CALL_JQ
 *   <nclosures>
 *   <callee closure>       (2 units)
 *   <nclosures closures>   (2 units each)
 *   <next instruction>
 *
 * A closure is:
 *
 *   <level>    (a relative frame count chased via the current frame's env)
 *   <index>    (an index of a subfunction or closure in that frame)
 *
 * We're looking for:
 *
 * a) the next instruction is a RET or a chain of unconditional JUMPs
 * that ends in a RET, and
 *
 * b) none of the closures -callee included- have level == 0.
 */
unsafe extern "C" fn tail_call_analyze(mut pc: *mut uint16_t) -> uint16_t {
    if *pc as libc::c_int == CALL_JQ as libc::c_int {
    } else {
        __assert_fail(b"*pc == CALL_JQ\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      1107 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"uint16_t tail_call_analyze(uint16_t *)\x00")).as_ptr());
    };
    pc = pc.offset(1);
    // + 1 for the callee closure
    let fresh28 = pc;
    pc = pc.offset(1);
    let mut nclosures: uint16_t =
        (*fresh28 as libc::c_int + 1 as libc::c_int) as uint16_t;
    while nclosures as libc::c_int > 0 as libc::c_int {
        let fresh29 = pc;
        pc = pc.offset(1);
        if *fresh29 as libc::c_int == 0 as libc::c_int {
            return CALL_JQ as libc::c_int as uint16_t
        }
        pc = pc.offset(1);
        nclosures = nclosures.wrapping_sub(1)
    }
    if ret_follows(pc) != 0 { return TAIL_CALL_JQ as libc::c_int as uint16_t }
    return CALL_JQ as libc::c_int as uint16_t;
}
unsafe extern "C" fn optimize_code(mut bc: *mut bytecode) -> *mut bytecode {
    let mut pc: *mut uint16_t = (*bc).code;
    // FIXME: Don't mutate bc->code...
    while pc < (*bc).code.offset((*bc).codelen as isize) {
        match *pc as libc::c_int {
            27 => { *pc = tail_call_analyze(pc) }
            _ => { }
        }
        pc = pc.offset(bytecode_operation_length(pc) as isize)
    }
    return bc;
}
unsafe extern "C" fn optimize(mut bc: *mut bytecode) -> *mut bytecode {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*bc).nsubfunctions {
        let ref mut fresh30 = *(*bc).subfunctions.offset(i as isize);
        *fresh30 = optimize(*(*bc).subfunctions.offset(i as isize));
        i += 1
    }
    return optimize_code(bc);
}
unsafe extern "C" fn args2obj(mut args: jv) -> jv {
    if jv_get_kind(args) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        return args
    }
    if jv_get_kind(args) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(args) == JV_KIND_ARRAY\x00" as *const u8
                          as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      1149 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"jv args2obj(jv)\x00")).as_ptr());
    };
    let mut r: jv = jv_object();
    let mut kk: jv =
        jv_string(b"name\x00" as *const u8 as *const libc::c_char);
    let mut vk: jv =
        jv_string(b"value\x00" as *const u8 as *const libc::c_char);
    let mut jv_len__: libc::c_int = jv_array_length(jv_copy(args));
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut v: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if i < jv_len__ {
                  v = jv_array_get(jv_copy(args), i);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            r =
                jv_object_set(r, jv_object_get(jv_copy(v), kk),
                              jv_object_get(v, vk));
            i += 1
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(args);
    jv_free(kk);
    jv_free(vk);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn jq_compile_args(mut jq: *mut jq_state,
                                         mut str: *const libc::c_char,
                                         mut args: jv) -> libc::c_int {
    jv_nomem_handler((*jq).nomem_handler, (*jq).nomem_handler_data);
    if jv_get_kind(args) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint ||
           jv_get_kind(args) as libc::c_uint ==
               JV_KIND_OBJECT as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(args) == JV_KIND_ARRAY || jv_get_kind(args) == JV_KIND_OBJECT\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      1163 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"int jq_compile_args(jq_state *, const char *, jv)\x00")).as_ptr());
    };
    let mut locations: *mut locfile = 0 as *mut locfile;
    locations =
        locfile_init(jq,
                     b"<top-level>\x00" as *const u8 as *const libc::c_char,
                     str, strlen(str) as libc::c_int);
    let mut program: block =
        block{first: 0 as *mut inst, last: 0 as *mut inst,};
    jq_reset(jq);
    if !(*jq).bc.is_null() {
        bytecode_free((*jq).bc);
        (*jq).bc = 0 as *mut bytecode
    }
    let mut nerrors: libc::c_int = load_program(jq, locations, &mut program);
    if nerrors == 0 as libc::c_int {
        nerrors = builtins_bind(jq, &mut program);
        if nerrors == 0 as libc::c_int {
            args = args2obj(args);
            nerrors = block_compile(program, &mut (*jq).bc, locations, args)
        }
    } else { jv_free(args); }
    if nerrors != 0 {
        jq_report_error(jq,
                        jv_string_fmt(b"jq: %d compile %s\x00" as *const u8 as
                                          *const libc::c_char, nerrors,
                                      if nerrors > 1 as libc::c_int {
                                          b"errors\x00" as *const u8 as
                                              *const libc::c_char
                                      } else {
                                          b"error\x00" as *const u8 as
                                              *const libc::c_char
                                      }));
    }
    if !(*jq).bc.is_null() { (*jq).bc = optimize((*jq).bc) }
    locfile_free(locations);
    return ((*jq).bc != 0 as *mut libc::c_void as *mut bytecode) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jq_compile(mut jq: *mut jq_state,
                                    mut str: *const libc::c_char)
 -> libc::c_int {
    return jq_compile_args(jq, str, jv_object());
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_jq_origin(mut jq: *mut jq_state) -> jv {
    return jq_get_attr(jq,
                       jv_string(b"JQ_ORIGIN\x00" as *const u8 as
                                     *const libc::c_char));
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_prog_origin(mut jq: *mut jq_state) -> jv {
    return jq_get_attr(jq,
                       jv_string(b"PROGRAM_ORIGIN\x00" as *const u8 as
                                     *const libc::c_char));
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_lib_dirs(mut jq: *mut jq_state) -> jv {
    return jq_get_attr(jq,
                       jv_string(b"JQ_LIBRARY_PATH\x00" as *const u8 as
                                     *const libc::c_char));
}
#[no_mangle]
pub unsafe extern "C" fn jq_set_attrs(mut jq: *mut jq_state, mut attrs: jv) {
    if jv_get_kind(attrs) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(attrs) == JV_KIND_OBJECT\x00" as *const u8
                          as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      1205 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void jq_set_attrs(jq_state *, jv)\x00")).as_ptr());
    };
    jv_free((*jq).attrs);
    (*jq).attrs = attrs;
}
#[no_mangle]
pub unsafe extern "C" fn jq_set_attr(mut jq: *mut jq_state, mut attr: jv,
                                     mut val: jv) {
    (*jq).attrs = jv_object_set((*jq).attrs, attr, val);
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_attr(mut jq: *mut jq_state, mut attr: jv)
 -> jv {
    return jv_object_get(jv_copy((*jq).attrs), attr);
}
#[no_mangle]
pub unsafe extern "C" fn jq_dump_disassembly(mut jq: *mut jq_state,
                                             mut indent: libc::c_int) {
    dump_disassembly(indent, (*jq).bc);
}
#[no_mangle]
pub unsafe extern "C" fn jq_set_input_cb(mut jq: *mut jq_state,
                                         mut cb: jq_input_cb,
                                         mut data: *mut libc::c_void) {
    (*jq).input_cb = cb;
    (*jq).input_cb_data = data;
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_input_cb(mut jq: *mut jq_state,
                                         mut cb: *mut jq_input_cb,
                                         mut data: *mut *mut libc::c_void) {
    *cb = (*jq).input_cb;
    *data = (*jq).input_cb_data;
}
#[no_mangle]
pub unsafe extern "C" fn jq_set_debug_cb(mut jq: *mut jq_state,
                                         mut cb: jq_msg_cb,
                                         mut data: *mut libc::c_void) {
    (*jq).debug_cb = cb;
    (*jq).debug_cb_data = data;
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_debug_cb(mut jq: *mut jq_state,
                                         mut cb: *mut jq_msg_cb,
                                         mut data: *mut *mut libc::c_void) {
    *cb = (*jq).debug_cb;
    *data = (*jq).debug_cb_data;
}
#[no_mangle]
pub unsafe extern "C" fn jq_halt(mut jq: *mut jq_state, mut exit_code: jv,
                                 mut error_message: jv) {
    if (*jq).halted == 0 {
    } else {
        __assert_fail(b"!jq->halted\x00" as *const u8 as *const libc::c_char,
                      b"src/execute.c\x00" as *const u8 as
                          *const libc::c_char,
                      1245 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void jq_halt(jq_state *, jv, jv)\x00")).as_ptr());
    };
    (*jq).halted = 1 as libc::c_int;
    (*jq).exit_code = exit_code;
    (*jq).error_message = error_message;
}
#[no_mangle]
pub unsafe extern "C" fn jq_halted(mut jq: *mut jq_state) -> libc::c_int {
    return (*jq).halted;
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_exit_code(mut jq: *mut jq_state) -> jv {
    return jv_copy((*jq).exit_code);
}
#[no_mangle]
pub unsafe extern "C" fn jq_get_error_message(mut jq: *mut jq_state) -> jv {
    return jv_copy((*jq).error_message);
}
