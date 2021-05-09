#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jq_state;
    pub type inst;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* All of the fields of this struct are private.
   Really. Do not play with them. */
    /* array offsets */
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn locfile_get_line(_: *mut locfile, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn locfile_locate(_: *mut locfile, _: location, _: *const libc::c_char,
                      _: ...);
    #[no_mangle]
    fn gen_location(_: location, _: *mut locfile, _: block) -> block;
    #[no_mangle]
    fn gen_noop() -> block;
    #[no_mangle]
    fn block_is_noop(b: block) -> libc::c_int;
    #[no_mangle]
    fn gen_op_simple(op: opcode) -> block;
    #[no_mangle]
    fn gen_const(constant: jv) -> block;
    #[no_mangle]
    fn block_is_const(b: block) -> libc::c_int;
    #[no_mangle]
    fn block_is_const_inf(b: block) -> libc::c_int;
    #[no_mangle]
    fn block_const_kind(b: block) -> jv_kind;
    #[no_mangle]
    fn block_const(b: block) -> jv;
    #[no_mangle]
    fn gen_op_unbound(op: opcode, name: *const libc::c_char) -> block;
    #[no_mangle]
    fn gen_op_bound(op: opcode, binder: block) -> block;
    #[no_mangle]
    fn gen_op_var_fresh(op: opcode, name: *const libc::c_char) -> block;
    #[no_mangle]
    fn gen_module(metadata: block) -> block;
    #[no_mangle]
    fn gen_import(name: *const libc::c_char, as_0: *const libc::c_char,
                  is_data: libc::c_int) -> block;
    #[no_mangle]
    fn gen_import_meta(import: block, metadata: block) -> block;
    #[no_mangle]
    fn gen_function(name: *const libc::c_char, formals: block, body: block)
     -> block;
    #[no_mangle]
    fn gen_param_regular(name: *const libc::c_char) -> block;
    #[no_mangle]
    fn gen_param(name: *const libc::c_char) -> block;
    #[no_mangle]
    fn gen_lambda(body: block) -> block;
    #[no_mangle]
    fn gen_call(name: *const libc::c_char, body: block) -> block;
    #[no_mangle]
    fn gen_subexp(a: block) -> block;
    #[no_mangle]
    fn gen_both(a: block, b: block) -> block;
    #[no_mangle]
    fn gen_const_object(expr: block) -> block;
    #[no_mangle]
    fn gen_collect(expr: block) -> block;
    #[no_mangle]
    fn gen_reduce(source: block, matcher: block, init: block, body: block)
     -> block;
    #[no_mangle]
    fn gen_foreach(source: block, matcher: block, init: block, update: block,
                   extract: block) -> block;
    #[no_mangle]
    fn gen_definedor(a: block, b: block) -> block;
    #[no_mangle]
    fn gen_and(a: block, b: block) -> block;
    #[no_mangle]
    fn gen_or(a: block, b: block) -> block;
    #[no_mangle]
    fn gen_dictpair(k: block, v: block) -> block;
    #[no_mangle]
    fn gen_array_matcher(left: block, curr: block) -> block;
    #[no_mangle]
    fn gen_object_matcher(name: block, curr: block) -> block;
    #[no_mangle]
    fn gen_destructure(var: block, matcher: block, body: block) -> block;
    #[no_mangle]
    fn gen_destructure_alt(matcher: block) -> block;
    #[no_mangle]
    fn gen_cond(cond: block, iftrue: block, iffalse: block) -> block;
    #[no_mangle]
    fn gen_try_handler(handler: block) -> block;
    #[no_mangle]
    fn gen_try(exp: block, handler: block) -> block;
    #[no_mangle]
    fn gen_label(label: *const libc::c_char, exp: block) -> block;
    #[no_mangle]
    fn block_join(a: block, b: block) -> block;
    #[no_mangle]
    fn block_has_only_binders_and_imports(_: block, bindflags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn block_has_main(_: block) -> libc::c_int;
    #[no_mangle]
    fn block_is_single(b: block) -> libc::c_int;
    #[no_mangle]
    fn block_bind_referenced(binder: block, body: block,
                             bindflags: libc::c_int) -> block;
    #[no_mangle]
    fn block_free(_: block);
    #[no_mangle]
    fn jv_false() -> jv;
    #[no_mangle]
    fn jv_true() -> jv;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid() -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_kind_name(_: jv_kind) -> *const libc::c_char;
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_string_concat(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_dump_string_trunc(x: jv, outbuf: *mut libc::c_char, bufsize: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn jv_cmp(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jq_yy_delete_buffer(b: YY_BUFFER_STATE, yyscanner: yyscan_t);
    /* Macros after this point can all be overridden by user definitions in
 * section 1.
 */
    /* Amount of stuff to slurp up with each read. */
    /* __ia64__ */
    /* Number of entries by which start-condition stack grows. */
    /* Default declaration of generated scanner - a define so the user can
 * easily add parameters.
 */
    #[no_mangle]
    fn jq_yylex(yylval_param: *mut YYSTYPE, yylloc_param: *mut location,
                yyscanner: yyscan_t) -> libc::c_int;
    #[no_mangle]
    fn jq_yylex_destroy(yyscanner: yyscan_t) -> libc::c_int;
    #[no_mangle]
    fn jq_yylex_init_extra(user_defined: libc::c_int, scanner: *mut yyscan_t)
     -> libc::c_int;
    #[no_mangle]
    fn jq_yy_scan_bytes(bytes: *const libc::c_char, len: libc::c_int,
                        yyscanner: yyscan_t) -> YY_BUFFER_STATE;
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
// NOTE: Not actually part of any op -- a pseudo-op flag for special
  //       handling of `break`.
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub first: *mut inst,
    pub last: *mut inst,
}
/* Enabling verbose error messages.  */
/* In a future release of Bison, this section will be replaced
   by #include "y.tab.h".  */
/* Debug traces.  */
/* "%code requires" blocks.  */
/* yacc.c:354  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lexer_param {
    pub lexer: yyscan_t,
}
/* ! C99 */
/* ! FLEXINT_H */
/* begin standard C++ headers. */
/* TODO: this is always defined, so inline it */
/* An opaque pointer. */
pub type yyscan_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub literal: jv,
    pub blk: block,
}
/* yacc.c:1667  */
pub type yytype_int16 = libc::c_short;
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
//$$ = BLOCK(gen_op_target(FORK_OPT, $2), $2, $4);
/* yacc.c:1667  */
//$$ = BLOCK(gen_op_target(FORK_OPT, $2), $2, gen_op_simple(BACKTRACK));
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:354  */
/* yacc.c:1667  */
/* yacc.c:1667  */
pub type yytype_uint8 = libc::c_uchar;
pub const YYERROR_VERBOSE_ARGS_MAXIMUM: C2RustUnnamed_1 = 5;
pub type C2RustUnnamed_1 = libc::c_uint;
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* ! defined yyoverflow || YYERROR_VERBOSE */
/* yacc.c:1667  */
/* yacc.c:1667  */
/* A type that is properly aligned for any stack member.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: YYSTYPE,
    pub yyls_alloc: location,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
/* For convenience, these vars (plus the bison vars far below)
   are macros in the reentrant scanner. */
/* Size of default input buffer. */
/* __ia64__ */
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
static mut UNKNOWN_LOCATION: location =
    {
        let mut init =
            location{start: -(1 as libc::c_int), end: -(1 as libc::c_int),};
        init
    };
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yyerror(mut loc: *mut location,
                                 mut answer: *mut block,
                                 mut errors: *mut libc::c_int,
                                 mut locations: *mut locfile,
                                 mut lexer_param_ptr: *mut lexer_param,
                                 mut s: *const libc::c_char) {
    *errors += 1;
    if !strstr(s,
               b"unexpected\x00" as *const u8 as
                   *const libc::c_char).is_null() {
        locfile_locate(locations, *loc,
                       b"jq: error: %s (Unix shell quoting issues?)\x00" as
                           *const u8 as *const libc::c_char, s);
    } else {
        locfile_locate(locations, *loc,
                       b"jq: error: %s\x00" as *const u8 as
                           *const libc::c_char, s);
    };
}
#[no_mangle]
pub unsafe extern "C" fn yylex(mut yylval: *mut YYSTYPE,
                               mut yylloc: *mut location,
                               mut answer: *mut block,
                               mut errors: *mut libc::c_int,
                               mut locations: *mut locfile,
                               mut lexer_param_ptr: *mut lexer_param)
 -> libc::c_int {
    let mut lexer: yyscan_t = (*lexer_param_ptr).lexer;
    let mut tok: libc::c_int = jq_yylex(yylval, yylloc, lexer);
    if (tok == 261 as libc::c_int || tok == 297 as libc::c_int) &&
           jv_is_valid((*yylval).literal) == 0 {
        let mut msg: jv = jv_invalid_get_msg(jv_copy((*yylval).literal));
        if jv_get_kind(msg) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            let mut l: location = *yylloc;
            yyerror(&mut l, answer, errors, locations, lexer_param_ptr,
                    jv_string_value(msg));
        } else {
            let mut l_0: location = *yylloc;
            yyerror(&mut l_0, answer, errors, locations, lexer_param_ptr,
                    b"Invalid literal\x00" as *const u8 as
                        *const libc::c_char);
        }
        jv_free(msg);
        jv_free((*yylval).literal);
        (*yylval).literal = jv_null()
    }
    return tok;
}
unsafe extern "C" fn check_object_key(mut k: block) -> jv {
    if block_is_const(k) != 0 &&
           block_const_kind(k) as libc::c_uint !=
               JV_KIND_STRING as libc::c_int as libc::c_uint {
        let mut errbuf: [libc::c_char; 15] = [0; 15];
        return jv_string_fmt(b"Cannot use %s (%s) as object key\x00" as
                                 *const u8 as *const libc::c_char,
                             jv_kind_name(block_const_kind(k)),
                             jv_dump_string_trunc(block_const(k),
                                                  errbuf.as_mut_ptr(),
                                                  ::std::mem::size_of::<[libc::c_char; 15]>()
                                                      as libc::c_ulong))
    }
    return jv_invalid();
}
unsafe extern "C" fn gen_index(mut obj: block, mut key: block) -> block {
    return block_join(block_join(gen_subexp(key), obj), gen_op_simple(INDEX));
}
unsafe extern "C" fn gen_index_opt(mut obj: block, mut key: block) -> block {
    return block_join(block_join(gen_subexp(key), obj),
                      gen_op_simple(INDEX_OPT));
}
unsafe extern "C" fn gen_slice_index(mut obj: block, mut start: block,
                                     mut end: block, mut idx_op: opcode)
 -> block {
    let mut key: block =
        block_join(block_join(block_join(block_join(block_join(block_join(gen_subexp(gen_const(jv_object())),
                                                                          gen_subexp(gen_const(jv_string(b"start\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char)))),
                                                               gen_subexp(start)),
                                                    gen_op_simple(INSERT)),
                                         gen_subexp(gen_const(jv_string(b"end\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char)))),
                              gen_subexp(end)), gen_op_simple(INSERT));
    return block_join(block_join(key, obj), gen_op_simple(idx_op));
}
unsafe extern "C" fn constant_fold(mut a: block, mut b: block,
                                   mut op: libc::c_int) -> block {
    if block_is_single(a) == 0 || block_is_const(a) == 0 ||
           block_is_single(b) == 0 || block_is_const(b) == 0 {
        return gen_noop()
    }
    if op == '+' as i32 {
        if block_const_kind(a) as libc::c_uint ==
               JV_KIND_NULL as libc::c_int as libc::c_uint {
            block_free(a);
            return b
        }
        if block_const_kind(b) as libc::c_uint ==
               JV_KIND_NULL as libc::c_int as libc::c_uint {
            block_free(b);
            return a
        }
    }
    if block_const_kind(a) as libc::c_uint !=
           block_const_kind(b) as libc::c_uint {
        return gen_noop()
    }
    let mut res: jv = jv_invalid();
    if block_const_kind(a) as libc::c_uint ==
           JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        let mut jv_a: jv = block_const(a);
        let mut jv_b: jv = block_const(b);
        let mut na: libc::c_double = jv_number_value(jv_a);
        let mut nb: libc::c_double = jv_number_value(jv_b);
        let mut cmp: libc::c_int = jv_cmp(jv_a, jv_b);
        match op {
            43 => { res = jv_number(na + nb) }
            45 => { res = jv_number(na - nb) }
            42 => { res = jv_number(na * nb) }
            47 => {
                /* Value type.  */
                res = jv_number(na / nb)
            }
            265 => {
                res =
                    if cmp == 0 as libc::c_int {
                        jv_true()
                    } else { jv_false() }
            }
            266 => {
                res =
                    if cmp != 0 as libc::c_int {
                        jv_true()
                    } else { jv_false() }
            }
            60 => {
                res =
                    if cmp < 0 as libc::c_int {
                        jv_true()
                    } else { jv_false() }
            }
            62 => {
                res =
                    if cmp > 0 as libc::c_int {
                        jv_true()
                    } else { jv_false() }
            }
            293 => {
                res =
                    if cmp <= 0 as libc::c_int {
                        jv_true()
                    } else { jv_false() }
            }
            294 => {
                res =
                    if cmp >= 0 as libc::c_int {
                        jv_true()
                    } else { jv_false() }
            }
            _ => { }
        }
    } else if op == '+' as i32 &&
                  block_const_kind(a) as libc::c_uint ==
                      JV_KIND_STRING as libc::c_int as libc::c_uint {
        res = jv_string_concat(block_const(a), block_const(b))
    } else { return gen_noop() }
    if jv_get_kind(res) as libc::c_uint ==
           JV_KIND_INVALID as libc::c_int as libc::c_uint {
        return gen_noop()
    }
    /* Location type.  */
    block_free(a);
    block_free(b);
    return gen_const(res);
}
unsafe extern "C" fn gen_binop(mut a: block, mut b: block,
                               mut op: libc::c_int) -> block {
    let mut folded: block = constant_fold(a, b, op);
    if block_is_noop(folded) == 0 { return folded }
    let mut funcname: *const libc::c_char = 0 as *const libc::c_char;
    match op {
        43 => { funcname = b"_plus\x00" as *const u8 as *const libc::c_char }
        45 => { funcname = b"_minus\x00" as *const u8 as *const libc::c_char }
        42 => {
            funcname = b"_multiply\x00" as *const u8 as *const libc::c_char
        }
        47 => {
            funcname = b"_divide\x00" as *const u8 as *const libc::c_char
        }
        37 => { funcname = b"_mod\x00" as *const u8 as *const libc::c_char }
        265 => {
            funcname = b"_equal\x00" as *const u8 as *const libc::c_char
        }
        266 => {
            /* !YY_YY_SRC_PARSER_H_INCLUDED  */
            funcname = b"_notequal\x00" as *const u8 as *const libc::c_char
        }
        60 => { funcname = b"_less\x00" as *const u8 as *const libc::c_char }
        62 => {
            /* Second part of user prologue.  */
            funcname = b"_greater\x00" as *const u8 as *const libc::c_char
        }
        293 => {
            funcname = b"_lesseq\x00" as *const u8 as *const libc::c_char
        }
        294 => {
            funcname = b"_greatereq\x00" as *const u8 as *const libc::c_char
        }
        _ => { }
    }
    if !funcname.is_null() {
    } else {
        __assert_fail(b"funcname\x00" as *const u8 as *const libc::c_char,
                      b"src/parser.y\x00" as *const u8 as *const libc::c_char,
                      273 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"block gen_binop(block, block, int)\x00")).as_ptr());
    };
    return gen_call(funcname, block_join(gen_lambda(a), gen_lambda(b)));
}
unsafe extern "C" fn gen_format(mut a: block, mut fmt: jv) -> block {
    return block_join(a,
                      gen_call(b"format\x00" as *const u8 as
                                   *const libc::c_char,
                               gen_lambda(gen_const(fmt))));
}
unsafe extern "C" fn gen_definedor_assign(mut object: block, mut val: block)
 -> block {
    let mut tmp: block =
        gen_op_var_fresh(STOREV,
                         b"tmp\x00" as *const u8 as *const libc::c_char);
    return block_join(block_join(block_join(gen_op_simple(DUP), val), tmp),
                      gen_call(b"_modify\x00" as *const u8 as
                                   *const libc::c_char,
                               block_join(gen_lambda(object),
                                          gen_lambda(gen_definedor(gen_noop(),
                                                                   gen_op_bound(LOADV,
                                                                                tmp))))));
}
unsafe extern "C" fn gen_update(mut object: block, mut val: block,
                                mut optype: libc::c_int) -> block {
    let mut tmp: block =
        gen_op_var_fresh(STOREV,
                         b"tmp\x00" as *const u8 as *const libc::c_char);
    return block_join(block_join(block_join(gen_op_simple(DUP), val), tmp),
                      gen_call(b"_modify\x00" as *const u8 as
                                   *const libc::c_char,
                               block_join(gen_lambda(object),
                                          gen_lambda(gen_binop(gen_noop(),
                                                               gen_op_bound(LOADV,
                                                                            tmp),
                                                               optype)))));
}
/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
/* yacc.c:1667  */
static mut yytranslate: [yytype_uint8; 303] =
    [0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     62 as libc::c_int as yytype_uint8, 56 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8,
     54 as libc::c_int as yytype_uint8, 52 as libc::c_int as yytype_uint8,
     48 as libc::c_int as yytype_uint8, 53 as libc::c_int as yytype_uint8,
     64 as libc::c_int as yytype_uint8, 55 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     50 as libc::c_int as yytype_uint8, 49 as libc::c_int as yytype_uint8,
     51 as libc::c_int as yytype_uint8, 58 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 65 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 66 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 67 as libc::c_int as yytype_uint8,
     47 as libc::c_int as yytype_uint8, 68 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 8 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8,
     11 as libc::c_int as yytype_uint8, 12 as libc::c_int as yytype_uint8,
     13 as libc::c_int as yytype_uint8, 14 as libc::c_int as yytype_uint8,
     15 as libc::c_int as yytype_uint8, 16 as libc::c_int as yytype_uint8,
     17 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 20 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 22 as libc::c_int as yytype_uint8,
     23 as libc::c_int as yytype_uint8, 24 as libc::c_int as yytype_uint8,
     25 as libc::c_int as yytype_uint8, 26 as libc::c_int as yytype_uint8,
     27 as libc::c_int as yytype_uint8, 28 as libc::c_int as yytype_uint8,
     29 as libc::c_int as yytype_uint8, 30 as libc::c_int as yytype_uint8,
     31 as libc::c_int as yytype_uint8, 32 as libc::c_int as yytype_uint8,
     33 as libc::c_int as yytype_uint8, 34 as libc::c_int as yytype_uint8,
     35 as libc::c_int as yytype_uint8, 36 as libc::c_int as yytype_uint8,
     37 as libc::c_int as yytype_uint8, 38 as libc::c_int as yytype_uint8,
     39 as libc::c_int as yytype_uint8, 40 as libc::c_int as yytype_uint8,
     41 as libc::c_int as yytype_uint8, 42 as libc::c_int as yytype_uint8,
     43 as libc::c_int as yytype_uint8, 44 as libc::c_int as yytype_uint8,
     45 as libc::c_int as yytype_uint8, 46 as libc::c_int as yytype_uint8,
     57 as libc::c_int as yytype_uint8];
/* yacc.c:1667  */
/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static mut yytname: [*const libc::c_char; 100] =
    [b"$end\x00" as *const u8 as *const libc::c_char,
     b"error\x00" as *const u8 as *const libc::c_char,
     b"$undefined\x00" as *const u8 as *const libc::c_char,
     b"INVALID_CHARACTER\x00" as *const u8 as *const libc::c_char,
     b"IDENT\x00" as *const u8 as *const libc::c_char,
     b"FIELD\x00" as *const u8 as *const libc::c_char,
     b"LITERAL\x00" as *const u8 as *const libc::c_char,
     b"FORMAT\x00" as *const u8 as *const libc::c_char,
     b"\"..\"\x00" as *const u8 as *const libc::c_char,
     b"\"%=\"\x00" as *const u8 as *const libc::c_char,
     b"\"==\"\x00" as *const u8 as *const libc::c_char,
     b"\"!=\"\x00" as *const u8 as *const libc::c_char,
     b"\"//\"\x00" as *const u8 as *const libc::c_char,
     b"\"as\"\x00" as *const u8 as *const libc::c_char,
     b"\"def\"\x00" as *const u8 as *const libc::c_char,
     b"\"module\"\x00" as *const u8 as *const libc::c_char,
     b"\"import\"\x00" as *const u8 as *const libc::c_char,
     b"\"include\"\x00" as *const u8 as *const libc::c_char,
     b"\"if\"\x00" as *const u8 as *const libc::c_char,
     b"\"then\"\x00" as *const u8 as *const libc::c_char,
     b"\"else\"\x00" as *const u8 as *const libc::c_char,
     b"\"elif\"\x00" as *const u8 as *const libc::c_char,
     b"\"reduce\"\x00" as *const u8 as *const libc::c_char,
     b"\"foreach\"\x00" as *const u8 as *const libc::c_char,
     b"\"end\"\x00" as *const u8 as *const libc::c_char,
     b"\"and\"\x00" as *const u8 as *const libc::c_char,
     b"\"or\"\x00" as *const u8 as *const libc::c_char,
     b"\"try\"\x00" as *const u8 as *const libc::c_char,
     b"\"catch\"\x00" as *const u8 as *const libc::c_char,
     b"\"label\"\x00" as *const u8 as *const libc::c_char,
     b"\"break\"\x00" as *const u8 as *const libc::c_char,
     b"\"__loc__\"\x00" as *const u8 as *const libc::c_char,
     b"\"|=\"\x00" as *const u8 as *const libc::c_char,
     b"\"+=\"\x00" as *const u8 as *const libc::c_char,
     b"\"-=\"\x00" as *const u8 as *const libc::c_char,
     b"\"*=\"\x00" as *const u8 as *const libc::c_char,
     b"\"/=\"\x00" as *const u8 as *const libc::c_char,
     b"\"//=\"\x00" as *const u8 as *const libc::c_char,
     b"\"<=\"\x00" as *const u8 as *const libc::c_char,
     b"\">=\"\x00" as *const u8 as *const libc::c_char,
     b"\"?//\"\x00" as *const u8 as *const libc::c_char,
     b"QQSTRING_START\x00" as *const u8 as *const libc::c_char,
     b"QQSTRING_TEXT\x00" as *const u8 as *const libc::c_char,
     b"QQSTRING_INTERP_START\x00" as *const u8 as *const libc::c_char,
     b"QQSTRING_INTERP_END\x00" as *const u8 as *const libc::c_char,
     b"QQSTRING_END\x00" as *const u8 as *const libc::c_char,
     b"FUNCDEF\x00" as *const u8 as *const libc::c_char,
     b"\'|\'\x00" as *const u8 as *const libc::c_char,
     b"\',\'\x00" as *const u8 as *const libc::c_char,
     b"\'=\'\x00" as *const u8 as *const libc::c_char,
     b"\'<\'\x00" as *const u8 as *const libc::c_char,
     b"\'>\'\x00" as *const u8 as *const libc::c_char,
     b"\'+\'\x00" as *const u8 as *const libc::c_char,
     b"\'-\'\x00" as *const u8 as *const libc::c_char,
     b"\'*\'\x00" as *const u8 as *const libc::c_char,
     b"\'/\'\x00" as *const u8 as *const libc::c_char,
     b"\'%\'\x00" as *const u8 as *const libc::c_char,
     b"NONOPT\x00" as *const u8 as *const libc::c_char,
     b"\'?\'\x00" as *const u8 as *const libc::c_char,
     b"\';\'\x00" as *const u8 as *const libc::c_char,
     b"\'(\'\x00" as *const u8 as *const libc::c_char,
     b"\')\'\x00" as *const u8 as *const libc::c_char,
     b"\'$\'\x00" as *const u8 as *const libc::c_char,
     b"\':\'\x00" as *const u8 as *const libc::c_char,
     b"\'.\'\x00" as *const u8 as *const libc::c_char,
     b"\'[\'\x00" as *const u8 as *const libc::c_char,
     b"\']\'\x00" as *const u8 as *const libc::c_char,
     b"\'{\'\x00" as *const u8 as *const libc::c_char,
     b"\'}\'\x00" as *const u8 as *const libc::c_char,
     b"$accept\x00" as *const u8 as *const libc::c_char,
     b"TopLevel\x00" as *const u8 as *const libc::c_char,
     b"Module\x00" as *const u8 as *const libc::c_char,
     b"Imports\x00" as *const u8 as *const libc::c_char,
     b"FuncDefs\x00" as *const u8 as *const libc::c_char,
     b"Exp\x00" as *const u8 as *const libc::c_char,
     b"Import\x00" as *const u8 as *const libc::c_char,
     b"ImportWhat\x00" as *const u8 as *const libc::c_char,
     b"ImportFrom\x00" as *const u8 as *const libc::c_char,
     b"FuncDef\x00" as *const u8 as *const libc::c_char,
     b"Params\x00" as *const u8 as *const libc::c_char,
     b"Param\x00" as *const u8 as *const libc::c_char,
     b"String\x00" as *const u8 as *const libc::c_char,
     b"@1\x00" as *const u8 as *const libc::c_char,
     b"@2\x00" as *const u8 as *const libc::c_char,
     b"QQString\x00" as *const u8 as *const libc::c_char,
     b"ElseBody\x00" as *const u8 as *const libc::c_char,
     b"ExpD\x00" as *const u8 as *const libc::c_char,
     b"Term\x00" as *const u8 as *const libc::c_char,
     b"Args\x00" as *const u8 as *const libc::c_char,
     b"Arg\x00" as *const u8 as *const libc::c_char,
     b"RepPatterns\x00" as *const u8 as *const libc::c_char,
     b"Patterns\x00" as *const u8 as *const libc::c_char,
     b"Pattern\x00" as *const u8 as *const libc::c_char,
     b"ArrayPats\x00" as *const u8 as *const libc::c_char,
     b"ObjPats\x00" as *const u8 as *const libc::c_char,
     b"ObjPat\x00" as *const u8 as *const libc::c_char,
     b"Keyword\x00" as *const u8 as *const libc::c_char,
     b"MkDict\x00" as *const u8 as *const libc::c_char,
     b"MkDictPair\x00" as *const u8 as *const libc::c_char,
     0 as *const libc::c_char];
/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
/* yacc.c:1667  */
static mut yypact: [yytype_int16; 322] =
    [43 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     75 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 66 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     108 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     161 as libc::c_int as yytype_int16, 161 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     1 as libc::c_int as yytype_int16, -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16, 588 as libc::c_int as yytype_int16,
     2133 as libc::c_int as yytype_int16, 288 as libc::c_int as yytype_int16,
     521 as libc::c_int as yytype_int16, 354 as libc::c_int as yytype_int16,
     1406 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 14 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     28 as libc::c_int as yytype_int16, 746 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(9 as libc::c_int) as yytype_int16, 1812 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 126 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     127 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(6 as libc::c_int) as yytype_int16, 72 as libc::c_int as yytype_int16,
     1236 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     140 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 78 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     983 as libc::c_int as yytype_int16, -(45 as libc::c_int) as yytype_int16,
     82 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     2161 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 83 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1980 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     -(26 as libc::c_int) as yytype_int16,
     -(3 as libc::c_int) as yytype_int16, 454 as libc::c_int as yytype_int16,
     142 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1980 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1457 as libc::c_int as yytype_int16, 1980 as libc::c_int as yytype_int16,
     -(19 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 10 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 653 as libc::c_int as yytype_int16,
     -(26 as libc::c_int) as yytype_int16,
     -(26 as libc::c_int) as yytype_int16, 718 as libc::c_int as yytype_int16,
     109 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 24 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     411 as libc::c_int as yytype_int16, 130 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     130 as libc::c_int as yytype_int16, 1270 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, -(157 as libc::c_int) as yytype_int16,
     130 as libc::c_int as yytype_int16, 130 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     411 as libc::c_int as yytype_int16, 2048 as libc::c_int as yytype_int16,
     209 as libc::c_int as yytype_int16, 209 as libc::c_int as yytype_int16,
     2014 as libc::c_int as yytype_int16, 349 as libc::c_int as yytype_int16,
     2080 as libc::c_int as yytype_int16, 2048 as libc::c_int as yytype_int16,
     2048 as libc::c_int as yytype_int16, 2048 as libc::c_int as yytype_int16,
     2048 as libc::c_int as yytype_int16, 2048 as libc::c_int as yytype_int16,
     2048 as libc::c_int as yytype_int16, 209 as libc::c_int as yytype_int16,
     209 as libc::c_int as yytype_int16, 1980 as libc::c_int as yytype_int16,
     2014 as libc::c_int as yytype_int16, 2048 as libc::c_int as yytype_int16,
     209 as libc::c_int as yytype_int16, 209 as libc::c_int as yytype_int16,
     -(6 as libc::c_int) as yytype_int16, -(6 as libc::c_int) as yytype_int16,
     101 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     157 as libc::c_int as yytype_int16, -(26 as libc::c_int) as yytype_int16,
     900 as libc::c_int as yytype_int16, 122 as libc::c_int as yytype_int16,
     117 as libc::c_int as yytype_int16, 132 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, 112 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     116 as libc::c_int as yytype_int16, 933 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 81 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     2189 as libc::c_int as yytype_int16, -(2 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1508 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1712 as libc::c_int as yytype_int16, 115 as libc::c_int as yytype_int16,
     118 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(20 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     130 as libc::c_int as yytype_int16, 129 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 129 as libc::c_int as yytype_int16,
     114 as libc::c_int as yytype_int16, 130 as libc::c_int as yytype_int16,
     129 as libc::c_int as yytype_int16, 129 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(13 as libc::c_int) as yytype_int16, 119 as libc::c_int as yytype_int16,
     125 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     175 as libc::c_int as yytype_int16, 133 as libc::c_int as yytype_int16,
     -(22 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     135 as libc::c_int as yytype_int16, -(26 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16, 1033 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1083 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     810 as libc::c_int as yytype_int16, 123 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     181 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 10 as libc::c_int as yytype_int16,
     136 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     1980 as libc::c_int as yytype_int16, 1846 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     130 as libc::c_int as yytype_int16, 130 as libc::c_int as yytype_int16,
     129 as libc::c_int as yytype_int16, -(26 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(26 as libc::c_int) as yytype_int16,
     -(26 as libc::c_int) as yytype_int16,
     1304 as libc::c_int as yytype_int16, 137 as libc::c_int as yytype_int16,
     -(26 as libc::c_int) as yytype_int16, 900 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(26 as libc::c_int) as yytype_int16, 149 as libc::c_int as yytype_int16,
     1980 as libc::c_int as yytype_int16, 143 as libc::c_int as yytype_int16,
     145 as libc::c_int as yytype_int16, 146 as libc::c_int as yytype_int16,
     1133 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16, 1896 as libc::c_int as yytype_int16,
     1946 as libc::c_int as yytype_int16, 1559 as libc::c_int as yytype_int16,
     1610 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     129 as libc::c_int as yytype_int16, 129 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     148 as libc::c_int as yytype_int16, -(26 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     147 as libc::c_int as yytype_int16, 1661 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16, 838 as libc::c_int as yytype_int16,
     838 as libc::c_int as yytype_int16, -(26 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1762 as libc::c_int as yytype_int16, 1338 as libc::c_int as yytype_int16,
     1183 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     838 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     1372 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16];
/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
/* yacc.c:1667  */
/* yacc.c:1667  */
static mut yydefact: [yytype_uint8; 322] =
    [4 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     109 as libc::c_int as yytype_uint8, 83 as libc::c_int as yytype_uint8,
     100 as libc::c_int as yytype_uint8, 102 as libc::c_int as yytype_uint8,
     75 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     62 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 101 as libc::c_int as yytype_uint8,
     48 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     8 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     79 as libc::c_int as yytype_uint8, 64 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     77 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 33 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     107 as libc::c_int as yytype_uint8, 136 as libc::c_int as yytype_uint8,
     137 as libc::c_int as yytype_uint8, 138 as libc::c_int as yytype_uint8,
     139 as libc::c_int as yytype_uint8, 140 as libc::c_int as yytype_uint8,
     141 as libc::c_int as yytype_uint8, 142 as libc::c_int as yytype_uint8,
     143 as libc::c_int as yytype_uint8, 144 as libc::c_int as yytype_uint8,
     145 as libc::c_int as yytype_uint8, 146 as libc::c_int as yytype_uint8,
     147 as libc::c_int as yytype_uint8, 148 as libc::c_int as yytype_uint8,
     149 as libc::c_int as yytype_uint8, 150 as libc::c_int as yytype_uint8,
     151 as libc::c_int as yytype_uint8, 152 as libc::c_int as yytype_uint8,
     153 as libc::c_int as yytype_uint8, 154 as libc::c_int as yytype_uint8,
     108 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     85 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     105 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 166 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     162 as libc::c_int as yytype_uint8, 167 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 156 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     22 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     10 as libc::c_int as yytype_uint8, 82 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     54 as libc::c_int as yytype_uint8, 53 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     8 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     49 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     117 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     115 as libc::c_int as yytype_uint8, 66 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 76 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 111 as libc::c_int as yytype_uint8,
     103 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     81 as libc::c_int as yytype_uint8, 112 as libc::c_int as yytype_uint8,
     104 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 114 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     164 as libc::c_int as yytype_uint8, 165 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     106 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     41 as libc::c_int as yytype_uint8, 42 as libc::c_int as yytype_uint8,
     43 as libc::c_int as yytype_uint8, 26 as libc::c_int as yytype_uint8,
     25 as libc::c_int as yytype_uint8, 24 as libc::c_int as yytype_uint8,
     28 as libc::c_int as yytype_uint8, 32 as libc::c_int as yytype_uint8,
     35 as libc::c_int as yytype_uint8, 37 as libc::c_int as yytype_uint8,
     40 as libc::c_int as yytype_uint8, 27 as libc::c_int as yytype_uint8,
     46 as libc::c_int as yytype_uint8, 47 as libc::c_int as yytype_uint8,
     29 as libc::c_int as yytype_uint8, 30 as libc::c_int as yytype_uint8,
     23 as libc::c_int as yytype_uint8, 44 as libc::c_int as yytype_uint8,
     45 as libc::c_int as yytype_uint8, 31 as libc::c_int as yytype_uint8,
     34 as libc::c_int as yytype_uint8, 36 as libc::c_int as yytype_uint8,
     38 as libc::c_int as yytype_uint8, 39 as libc::c_int as yytype_uint8,
     78 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     121 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     84 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 93 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 50 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 110 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     57 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     17 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 18 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 67 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 158 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 169 as libc::c_int as yytype_uint8,
     73 as libc::c_int as yytype_uint8, 159 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     161 as libc::c_int as yytype_uint8, 160 as libc::c_int as yytype_uint8,
     157 as libc::c_int as yytype_uint8, 122 as libc::c_int as yytype_uint8,
     125 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     127 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 80 as libc::c_int as yytype_uint8,
     113 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     89 as libc::c_int as yytype_uint8, 52 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 116 as libc::c_int as yytype_uint8,
     65 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 55 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     16 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     21 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     72 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 163 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 123 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 129 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     124 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     120 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     91 as libc::c_int as yytype_uint8, 99 as libc::c_int as yytype_uint8,
     98 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     88 as libc::c_int as yytype_uint8, 51 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     68 as libc::c_int as yytype_uint8, 71 as libc::c_int as yytype_uint8,
     168 as libc::c_int as yytype_uint8, 126 as libc::c_int as yytype_uint8,
     135 as libc::c_int as yytype_uint8, 131 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     133 as libc::c_int as yytype_uint8, 128 as libc::c_int as yytype_uint8,
     132 as libc::c_int as yytype_uint8, 90 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 95 as libc::c_int as yytype_uint8,
     97 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     70 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 130 as libc::c_int as yytype_uint8,
     94 as libc::c_int as yytype_uint8, 56 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 134 as libc::c_int as yytype_uint8,
     69 as libc::c_int as yytype_uint8, 12 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 14 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 13 as libc::c_int as yytype_uint8];
/* yacc.c:1667  */
/* YYPGOTO[NTERM-NUM].  */
/* yacc.c:1667  */
static mut yypgoto: [yytype_int16; 30] =
    [-(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     177 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     184 as libc::c_int as yytype_int16, -(11 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(43 as libc::c_int) as yytype_int16, 5 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 89 as libc::c_int as yytype_int16,
     -(98 as libc::c_int) as yytype_int16,
     -(140 as libc::c_int) as yytype_int16,
     -(4 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(61 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16,
     -(53 as libc::c_int) as yytype_int16,
     -(18 as libc::c_int) as yytype_int16,
     -(106 as libc::c_int) as yytype_int16,
     -(157 as libc::c_int) as yytype_int16];
/* yacc.c:1667  */
/* YYDEFGOTO[NTERM-NUM].  */
static mut yydefgoto: [yytype_int16; 30] =
    [-(1 as libc::c_int) as yytype_int16, 2 as libc::c_int as yytype_int16,
     3 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     118 as libc::c_int as yytype_int16, 110 as libc::c_int as yytype_int16,
     31 as libc::c_int as yytype_int16, 32 as libc::c_int as yytype_int16,
     115 as libc::c_int as yytype_int16, 24 as libc::c_int as yytype_int16,
     199 as libc::c_int as yytype_int16, 200 as libc::c_int as yytype_int16,
     25 as libc::c_int as yytype_int16, 44 as libc::c_int as yytype_int16,
     127 as libc::c_int as yytype_int16, 136 as libc::c_int as yytype_int16,
     255 as libc::c_int as yytype_int16, 215 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 125 as libc::c_int as yytype_int16,
     126 as libc::c_int as yytype_int16, 182 as libc::c_int as yytype_int16,
     183 as libc::c_int as yytype_int16, 184 as libc::c_int as yytype_int16,
     225 as libc::c_int as yytype_int16, 231 as libc::c_int as yytype_int16,
     232 as libc::c_int as yytype_int16, 81 as libc::c_int as yytype_int16,
     82 as libc::c_int as yytype_int16, 83 as libc::c_int as yytype_int16];
/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
/* yacc.c:1667  */
/* yacc.c:1667  */
static mut yytable: [yytype_int16; 2221] =
    [23 as libc::c_int as yytype_int16, 68 as libc::c_int as yytype_int16,
     42 as libc::c_int as yytype_int16, 143 as libc::c_int as yytype_int16,
     71 as libc::c_int as yytype_int16, 111 as libc::c_int as yytype_int16,
     217 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     220 as libc::c_int as yytype_int16, 221 as libc::c_int as yytype_int16,
     40 as libc::c_int as yytype_int16, 112 as libc::c_int as yytype_int16,
     197 as libc::c_int as yytype_int16, 243 as libc::c_int as yytype_int16,
     45 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     144 as libc::c_int as yytype_int16, 120 as libc::c_int as yytype_int16,
     75 as libc::c_int as yytype_int16, 71 as libc::c_int as yytype_int16,
     111 as libc::c_int as yytype_int16, 145 as libc::c_int as yytype_int16,
     224 as libc::c_int as yytype_int16, 72 as libc::c_int as yytype_int16,
     271 as libc::c_int as yytype_int16, 80 as libc::c_int as yytype_int16,
     143 as libc::c_int as yytype_int16, 119 as libc::c_int as yytype_int16,
     131 as libc::c_int as yytype_int16, 123 as libc::c_int as yytype_int16,
     124 as libc::c_int as yytype_int16, 116 as libc::c_int as yytype_int16,
     116 as libc::c_int as yytype_int16, 264 as libc::c_int as yytype_int16,
     179 as libc::c_int as yytype_int16, 213 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 180 as libc::c_int as yytype_int16,
     194 as libc::c_int as yytype_int16, 181 as libc::c_int as yytype_int16,
     195 as libc::c_int as yytype_int16, 144 as libc::c_int as yytype_int16,
     28 as libc::c_int as yytype_int16, 29 as libc::c_int as yytype_int16,
     272 as libc::c_int as yytype_int16, 222 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 128 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 265 as libc::c_int as yytype_int16,
     129 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     111 as libc::c_int as yytype_int16, 249 as libc::c_int as yytype_int16,
     1 as libc::c_int as yytype_int16, 250 as libc::c_int as yytype_int16,
     111 as libc::c_int as yytype_int16, 149 as libc::c_int as yytype_int16,
     185 as libc::c_int as yytype_int16, 43 as libc::c_int as yytype_int16,
     113 as libc::c_int as yytype_int16, 114 as libc::c_int as yytype_int16,
     209 as libc::c_int as yytype_int16, 210 as libc::c_int as yytype_int16,
     132 as libc::c_int as yytype_int16, 211 as libc::c_int as yytype_int16,
     204 as libc::c_int as yytype_int16, 205 as libc::c_int as yytype_int16,
     198 as libc::c_int as yytype_int16, 244 as libc::c_int as yytype_int16,
     260 as libc::c_int as yytype_int16, 27 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 147 as libc::c_int as yytype_int16,
     274 as libc::c_int as yytype_int16, 263 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 113 as libc::c_int as yytype_int16,
     114 as libc::c_int as yytype_int16, 154 as libc::c_int as yytype_int16,
     155 as libc::c_int as yytype_int16, 156 as libc::c_int as yytype_int16,
     157 as libc::c_int as yytype_int16, 158 as libc::c_int as yytype_int16,
     159 as libc::c_int as yytype_int16, 160 as libc::c_int as yytype_int16,
     161 as libc::c_int as yytype_int16, 162 as libc::c_int as yytype_int16,
     163 as libc::c_int as yytype_int16, 164 as libc::c_int as yytype_int16,
     165 as libc::c_int as yytype_int16, 166 as libc::c_int as yytype_int16,
     167 as libc::c_int as yytype_int16, 168 as libc::c_int as yytype_int16,
     169 as libc::c_int as yytype_int16, 170 as libc::c_int as yytype_int16,
     171 as libc::c_int as yytype_int16, 172 as libc::c_int as yytype_int16,
     173 as libc::c_int as yytype_int16, 174 as libc::c_int as yytype_int16,
     175 as libc::c_int as yytype_int16, 176 as libc::c_int as yytype_int16,
     177 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     291 as libc::c_int as yytype_int16, 120 as libc::c_int as yytype_int16,
     292 as libc::c_int as yytype_int16, 293 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 190 as libc::c_int as yytype_int16,
     296 as libc::c_int as yytype_int16, 113 as libc::c_int as yytype_int16,
     114 as libc::c_int as yytype_int16, 298 as libc::c_int as yytype_int16,
     186 as libc::c_int as yytype_int16, 113 as libc::c_int as yytype_int16,
     114 as libc::c_int as yytype_int16, 289 as libc::c_int as yytype_int16,
     290 as libc::c_int as yytype_int16, 209 as libc::c_int as yytype_int16,
     210 as libc::c_int as yytype_int16, 41 as libc::c_int as yytype_int16,
     246 as libc::c_int as yytype_int16, 133 as libc::c_int as yytype_int16,
     201 as libc::c_int as yytype_int16, 203 as libc::c_int as yytype_int16,
     134 as libc::c_int as yytype_int16, 135 as libc::c_int as yytype_int16,
     207 as libc::c_int as yytype_int16, 137 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 309 as libc::c_int as yytype_int16,
     216 as libc::c_int as yytype_int16, 139 as libc::c_int as yytype_int16,
     216 as libc::c_int as yytype_int16, 140 as libc::c_int as yytype_int16,
     141 as libc::c_int as yytype_int16, 146 as libc::c_int as yytype_int16,
     216 as libc::c_int as yytype_int16, 216 as libc::c_int as yytype_int16,
     80 as libc::c_int as yytype_int16, 150 as libc::c_int as yytype_int16,
     151 as libc::c_int as yytype_int16, 152 as libc::c_int as yytype_int16,
     315 as libc::c_int as yytype_int16, 153 as libc::c_int as yytype_int16,
     178 as libc::c_int as yytype_int16, 191 as libc::c_int as yytype_int16,
     208 as libc::c_int as yytype_int16, 219 as libc::c_int as yytype_int16,
     80 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 223 as libc::c_int as yytype_int16,
     234 as libc::c_int as yytype_int16, 233 as libc::c_int as yytype_int16,
     235 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 6 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 8 as libc::c_int as yytype_int16,
     237 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     -(119 as libc::c_int) as yytype_int16,
     238 as libc::c_int as yytype_int16, 240 as libc::c_int as yytype_int16,
     256 as libc::c_int as yytype_int16, 261 as libc::c_int as yytype_int16,
     262 as libc::c_int as yytype_int16, 257 as libc::c_int as yytype_int16,
     269 as libc::c_int as yytype_int16, 248 as libc::c_int as yytype_int16,
     280 as libc::c_int as yytype_int16, 266 as libc::c_int as yytype_int16,
     214 as libc::c_int as yytype_int16, 236 as libc::c_int as yytype_int16,
     281 as libc::c_int as yytype_int16, 230 as libc::c_int as yytype_int16,
     239 as libc::c_int as yytype_int16, 267 as libc::c_int as yytype_int16,
     -(118 as libc::c_int) as yytype_int16, 18 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     124 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 270 as libc::c_int as yytype_int16,
     22 as libc::c_int as yytype_int16, 273 as libc::c_int as yytype_int16,
     283 as libc::c_int as yytype_int16, 295 as libc::c_int as yytype_int16,
     299 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     300 as libc::c_int as yytype_int16, 301 as libc::c_int as yytype_int16,
     310 as libc::c_int as yytype_int16, 282 as libc::c_int as yytype_int16,
     258 as libc::c_int as yytype_int16, 121 as libc::c_int as yytype_int16,
     259 as libc::c_int as yytype_int16, 216 as libc::c_int as yytype_int16,
     308 as libc::c_int as yytype_int16, 192 as libc::c_int as yytype_int16,
     117 as libc::c_int as yytype_int16, 316 as libc::c_int as yytype_int16,
     216 as libc::c_int as yytype_int16, 196 as libc::c_int as yytype_int16,
     245 as libc::c_int as yytype_int16, 297 as libc::c_int as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16, 18 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 268 as libc::c_int as yytype_int16,
     22 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     275 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     279 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 284 as libc::c_int as yytype_int16,
     285 as libc::c_int as yytype_int16, 233 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 286 as libc::c_int as yytype_int16,
     287 as libc::c_int as yytype_int16, 216 as libc::c_int as yytype_int16,
     216 as libc::c_int as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 230 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 303 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, -(74 as libc::c_int) as yytype_int16,
     69 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 70 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     71 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 312 as libc::c_int as yytype_int16,
     313 as libc::c_int as yytype_int16, 314 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 320 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, -(74 as libc::c_int) as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 76 as libc::c_int as yytype_int16,
     -(74 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     77 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 71 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, 57 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     62 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     66 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     212 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     78 as libc::c_int as yytype_int16, 77 as libc::c_int as yytype_int16,
     79 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     71 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(155 as libc::c_int) as yytype_int16, 0 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     57 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     61 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 187 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 78 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 79 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, -(155 as libc::c_int) as yytype_int16,
     0 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 188 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     189 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     73 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 6 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 8 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     13 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, 15 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 74 as libc::c_int as yytype_int16,
     22 as libc::c_int as yytype_int16, 46 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     202 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 6 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 8 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     13 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, 15 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 206 as libc::c_int as yytype_int16,
     22 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 122 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     278 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 226 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     227 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 71 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, 57 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     62 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     66 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     228 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     229 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     241 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 242 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 142 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 276 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 277 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 302 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     318 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     319 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 138 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 218 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 294 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 317 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 321 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 109 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     193 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 251 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     306 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 307 as libc::c_int as yytype_int16,
     84 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 87 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     88 as libc::c_int as yytype_int16, 89 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     98 as libc::c_int as yytype_int16, 99 as libc::c_int as yytype_int16,
     100 as libc::c_int as yytype_int16, 101 as libc::c_int as yytype_int16,
     102 as libc::c_int as yytype_int16, 103 as libc::c_int as yytype_int16,
     104 as libc::c_int as yytype_int16, 105 as libc::c_int as yytype_int16,
     106 as libc::c_int as yytype_int16, 107 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 108 as libc::c_int as yytype_int16,
     311 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     252 as libc::c_int as yytype_int16, 253 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     254 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     252 as libc::c_int as yytype_int16, 253 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 130 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     288 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     304 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 305 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     90 as libc::c_int as yytype_int16, 91 as libc::c_int as yytype_int16,
     92 as libc::c_int as yytype_int16, 93 as libc::c_int as yytype_int16,
     94 as libc::c_int as yytype_int16, 95 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, -(156 as libc::c_int) as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     -(156 as libc::c_int) as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 85 as libc::c_int as yytype_int16,
     86 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     -(156 as libc::c_int) as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     96 as libc::c_int as yytype_int16, 97 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     108 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     57 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     61 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16, 148 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     57 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     61 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16, 247 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     0 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     57 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     61 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16];
#[no_mangle]
pub unsafe extern "C" fn jq_parse(mut locations: *mut locfile,
                                  mut answer: *mut block) -> libc::c_int {
    let mut scanner: lexer_param =
        lexer_param{lexer: 0 as *mut libc::c_void,};
    let mut buf: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    jq_yylex_init_extra(0 as libc::c_int, &mut scanner.lexer);
    buf =
        jq_yy_scan_bytes((*locations).data, (*locations).length,
                         scanner.lexer);
    let mut errors: libc::c_int = 0 as libc::c_int;
    *answer = gen_noop();
    yyparse(answer, &mut errors, locations, &mut scanner);
    jq_yy_delete_buffer(buf, scanner.lexer);
    jq_yylex_destroy(scanner.lexer);
    if errors > 0 as libc::c_int { block_free(*answer); *answer = gen_noop() }
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn jq_parse_library(mut locations: *mut locfile,
                                          mut answer: *mut block)
 -> libc::c_int {
    let mut errs: libc::c_int = jq_parse(locations, answer);
    if errs != 0 { return errs }
    if block_has_main(*answer) != 0 {
        locfile_locate(locations, UNKNOWN_LOCATION,
                       b"jq: error: library should only have function definitions, not a main expression\x00"
                           as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    if block_has_only_binders_and_imports(*answer,
                                          OP_IS_CALL_PSEUDO as libc::c_int) !=
           0 {
    } else {
        __assert_fail(b"block_has_only_binders_and_imports(*answer, OP_IS_CALL_PSEUDO)\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/parser.y\x00" as *const u8 as *const libc::c_char,
                      1012 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"int jq_parse_library(struct locfile *, block *)\x00")).as_ptr());
    };
    return 0 as libc::c_int;
}
static mut yycheck: [yytype_int16; 2221] =
    [1 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     1 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     146 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     150 as libc::c_int as yytype_int16, 151 as libc::c_int as yytype_int16,
     13 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 68 as libc::c_int as yytype_int16,
     180 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     13 as libc::c_int as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     29 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     62 as libc::c_int as yytype_int16, 143 as libc::c_int as yytype_int16,
     41 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     61 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     68 as libc::c_int as yytype_int16, 153 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 41 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 79 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     42 as libc::c_int as yytype_int16, 43 as libc::c_int as yytype_int16,
     13 as libc::c_int as yytype_int16, 45 as libc::c_int as yytype_int16,
     131 as libc::c_int as yytype_int16, 132 as libc::c_int as yytype_int16,
     62 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     214 as libc::c_int as yytype_int16, 0 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, 78 as libc::c_int as yytype_int16,
     234 as libc::c_int as yytype_int16, 219 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 84 as libc::c_int as yytype_int16,
     85 as libc::c_int as yytype_int16, 86 as libc::c_int as yytype_int16,
     87 as libc::c_int as yytype_int16, 88 as libc::c_int as yytype_int16,
     89 as libc::c_int as yytype_int16, 90 as libc::c_int as yytype_int16,
     91 as libc::c_int as yytype_int16, 92 as libc::c_int as yytype_int16,
     93 as libc::c_int as yytype_int16, 94 as libc::c_int as yytype_int16,
     95 as libc::c_int as yytype_int16, 96 as libc::c_int as yytype_int16,
     97 as libc::c_int as yytype_int16, 98 as libc::c_int as yytype_int16,
     99 as libc::c_int as yytype_int16, 100 as libc::c_int as yytype_int16,
     101 as libc::c_int as yytype_int16, 102 as libc::c_int as yytype_int16,
     103 as libc::c_int as yytype_int16, 104 as libc::c_int as yytype_int16,
     105 as libc::c_int as yytype_int16, 106 as libc::c_int as yytype_int16,
     107 as libc::c_int as yytype_int16, 41 as libc::c_int as yytype_int16,
     264 as libc::c_int as yytype_int16, 120 as libc::c_int as yytype_int16,
     266 as libc::c_int as yytype_int16, 267 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 114 as libc::c_int as yytype_int16,
     270 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 273 as libc::c_int as yytype_int16,
     113 as libc::c_int as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 261 as libc::c_int as yytype_int16,
     262 as libc::c_int as yytype_int16, 42 as libc::c_int as yytype_int16,
     43 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     45 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     129 as libc::c_int as yytype_int16, 130 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     133 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, 295 as libc::c_int as yytype_int16,
     144 as libc::c_int as yytype_int16, 1 as libc::c_int as yytype_int16,
     146 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     66 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     150 as libc::c_int as yytype_int16, 151 as libc::c_int as yytype_int16,
     143 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 68 as libc::c_int as yytype_int16,
     308 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     47 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     153 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     40 as libc::c_int as yytype_int16, 181 as libc::c_int as yytype_int16,
     47 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 6 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 8 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 41 as libc::c_int as yytype_int16,
     40 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     47 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     198 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     185 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     181 as libc::c_int as yytype_int16, 188 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 40 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     62 as libc::c_int as yytype_int16, 194 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     41 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, 58 as libc::c_int as yytype_int16,
     249 as libc::c_int as yytype_int16, 208 as libc::c_int as yytype_int16,
     31 as libc::c_int as yytype_int16, 210 as libc::c_int as yytype_int16,
     214 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     120 as libc::c_int as yytype_int16, 29 as libc::c_int as yytype_int16,
     312 as libc::c_int as yytype_int16, 219 as libc::c_int as yytype_int16,
     127 as libc::c_int as yytype_int16, 194 as libc::c_int as yytype_int16,
     271 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 60 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 62 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 228 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     235 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     241 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 252 as libc::c_int as yytype_int16,
     253 as libc::c_int as yytype_int16, 271 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 256 as libc::c_int as yytype_int16,
     257 as libc::c_int as yytype_int16, 261 as libc::c_int as yytype_int16,
     262 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     271 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     283 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     0 as libc::c_int as yytype_int16, 1 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 7 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 13 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     305 as libc::c_int as yytype_int16, 306 as libc::c_int as yytype_int16,
     307 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     24 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     28 as libc::c_int as yytype_int16, 318 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     44 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 61 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 63 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     66 as libc::c_int as yytype_int16, 1 as libc::c_int as yytype_int16,
     68 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 13 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, 15 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     24 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 27 as libc::c_int as yytype_int16,
     28 as libc::c_int as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, 31 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     1 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     60 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     62 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     7 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     68 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     13 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     23 as libc::c_int as yytype_int16, 24 as libc::c_int as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     27 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     29 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     31 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     41 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 1 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     14 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 60 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 62 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 68 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 27 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 53 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, 63 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     66 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     1 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 4 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 6 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 8 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 14 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 18 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 22 as libc::c_int as yytype_int16,
     23 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     27 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     29 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     41 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     53 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 60 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 62 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 66 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16, 1 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     14 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     18 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 27 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 53 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     1 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 4 as libc::c_int as yytype_int16,
     5 as libc::c_int as yytype_int16, 6 as libc::c_int as yytype_int16,
     7 as libc::c_int as yytype_int16, 8 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 14 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 18 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 22 as libc::c_int as yytype_int16,
     23 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     27 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     29 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     41 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     53 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 60 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 62 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 64 as libc::c_int as yytype_int16,
     65 as libc::c_int as yytype_int16, 1 as libc::c_int as yytype_int16,
     67 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     14 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     18 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 27 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     18 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 53 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 27 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 67 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 53 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 59 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 67 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     14 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     18 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 27 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     4 as libc::c_int as yytype_int16, 5 as libc::c_int as yytype_int16,
     6 as libc::c_int as yytype_int16, 7 as libc::c_int as yytype_int16,
     8 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     18 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 53 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 27 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     66 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 53 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, 1 as libc::c_int as yytype_int16,
     64 as libc::c_int as yytype_int16, 65 as libc::c_int as yytype_int16,
     4 as libc::c_int as yytype_int16, 67 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 7 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 13 as libc::c_int as yytype_int16,
     14 as libc::c_int as yytype_int16, 15 as libc::c_int as yytype_int16,
     16 as libc::c_int as yytype_int16, 17 as libc::c_int as yytype_int16,
     18 as libc::c_int as yytype_int16, 19 as libc::c_int as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     22 as libc::c_int as yytype_int16, 23 as libc::c_int as yytype_int16,
     24 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 27 as libc::c_int as yytype_int16,
     28 as libc::c_int as yytype_int16, 29 as libc::c_int as yytype_int16,
     30 as libc::c_int as yytype_int16, 31 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 41 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     60 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     62 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     63 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 66 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 66 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 66 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 66 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 66 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     61 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, 61 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 59 as libc::c_int as yytype_int16,
     9 as libc::c_int as yytype_int16, 10 as libc::c_int as yytype_int16,
     11 as libc::c_int as yytype_int16, 12 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 32 as libc::c_int as yytype_int16,
     33 as libc::c_int as yytype_int16, 34 as libc::c_int as yytype_int16,
     35 as libc::c_int as yytype_int16, 36 as libc::c_int as yytype_int16,
     37 as libc::c_int as yytype_int16, 38 as libc::c_int as yytype_int16,
     39 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     47 as libc::c_int as yytype_int16, 48 as libc::c_int as yytype_int16,
     49 as libc::c_int as yytype_int16, 50 as libc::c_int as yytype_int16,
     51 as libc::c_int as yytype_int16, 52 as libc::c_int as yytype_int16,
     53 as libc::c_int as yytype_int16, 54 as libc::c_int as yytype_int16,
     55 as libc::c_int as yytype_int16, 56 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 58 as libc::c_int as yytype_int16,
     59 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     24 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     20 as libc::c_int as yytype_int16, 21 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 19 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     44 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     24 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 19 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     12 as libc::c_int as yytype_int16, 47 as libc::c_int as yytype_int16,
     48 as libc::c_int as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 9 as libc::c_int as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     58 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     26 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     32 as libc::c_int as yytype_int16, 33 as libc::c_int as yytype_int16,
     34 as libc::c_int as yytype_int16, 35 as libc::c_int as yytype_int16,
     36 as libc::c_int as yytype_int16, 37 as libc::c_int as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     10 as libc::c_int as yytype_int16, 11 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, 49 as libc::c_int as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, 25 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     38 as libc::c_int as yytype_int16, 39 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     50 as libc::c_int as yytype_int16, 51 as libc::c_int as yytype_int16,
     52 as libc::c_int as yytype_int16, 53 as libc::c_int as yytype_int16,
     54 as libc::c_int as yytype_int16, 55 as libc::c_int as yytype_int16,
     56 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     58 as libc::c_int as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     13 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     23 as libc::c_int as yytype_int16, 24 as libc::c_int as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     27 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     29 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     31 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     13 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     23 as libc::c_int as yytype_int16, 24 as libc::c_int as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     27 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     29 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     31 as libc::c_int as yytype_int16, 4 as libc::c_int as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     -(1 as libc::c_int) as yytype_int16, -(1 as libc::c_int) as yytype_int16,
     13 as libc::c_int as yytype_int16, 14 as libc::c_int as yytype_int16,
     15 as libc::c_int as yytype_int16, 16 as libc::c_int as yytype_int16,
     17 as libc::c_int as yytype_int16, 18 as libc::c_int as yytype_int16,
     19 as libc::c_int as yytype_int16, 20 as libc::c_int as yytype_int16,
     21 as libc::c_int as yytype_int16, 22 as libc::c_int as yytype_int16,
     23 as libc::c_int as yytype_int16, 24 as libc::c_int as yytype_int16,
     25 as libc::c_int as yytype_int16, 26 as libc::c_int as yytype_int16,
     27 as libc::c_int as yytype_int16, 28 as libc::c_int as yytype_int16,
     29 as libc::c_int as yytype_int16, 30 as libc::c_int as yytype_int16,
     31 as libc::c_int as yytype_int16];
/* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static mut yystos: [yytype_uint8; 322] =
    [0 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     70 as libc::c_int as yytype_uint8, 71 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     6 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     8 as libc::c_int as yytype_uint8, 14 as libc::c_int as yytype_uint8,
     18 as libc::c_int as yytype_uint8, 22 as libc::c_int as yytype_uint8,
     23 as libc::c_int as yytype_uint8, 27 as libc::c_int as yytype_uint8,
     29 as libc::c_int as yytype_uint8, 30 as libc::c_int as yytype_uint8,
     41 as libc::c_int as yytype_uint8, 53 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 62 as libc::c_int as yytype_uint8,
     64 as libc::c_int as yytype_uint8, 65 as libc::c_int as yytype_uint8,
     67 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     78 as libc::c_int as yytype_uint8, 81 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     16 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     72 as libc::c_int as yytype_uint8, 75 as libc::c_int as yytype_uint8,
     76 as libc::c_int as yytype_uint8, 60 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 41 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 62 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 62 as libc::c_int as yytype_uint8,
     82 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 13 as libc::c_int as yytype_uint8,
     14 as libc::c_int as yytype_uint8, 15 as libc::c_int as yytype_uint8,
     16 as libc::c_int as yytype_uint8, 17 as libc::c_int as yytype_uint8,
     18 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 21 as libc::c_int as yytype_uint8,
     22 as libc::c_int as yytype_uint8, 23 as libc::c_int as yytype_uint8,
     24 as libc::c_int as yytype_uint8, 25 as libc::c_int as yytype_uint8,
     26 as libc::c_int as yytype_uint8, 27 as libc::c_int as yytype_uint8,
     28 as libc::c_int as yytype_uint8, 29 as libc::c_int as yytype_uint8,
     30 as libc::c_int as yytype_uint8, 31 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 7 as libc::c_int as yytype_uint8,
     81 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 62 as libc::c_int as yytype_uint8,
     81 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     97 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 10 as libc::c_int as yytype_uint8,
     11 as libc::c_int as yytype_uint8, 12 as libc::c_int as yytype_uint8,
     25 as libc::c_int as yytype_uint8, 26 as libc::c_int as yytype_uint8,
     32 as libc::c_int as yytype_uint8, 33 as libc::c_int as yytype_uint8,
     34 as libc::c_int as yytype_uint8, 35 as libc::c_int as yytype_uint8,
     36 as libc::c_int as yytype_uint8, 37 as libc::c_int as yytype_uint8,
     38 as libc::c_int as yytype_uint8, 39 as libc::c_int as yytype_uint8,
     47 as libc::c_int as yytype_uint8, 48 as libc::c_int as yytype_uint8,
     49 as libc::c_int as yytype_uint8, 50 as libc::c_int as yytype_uint8,
     51 as libc::c_int as yytype_uint8, 52 as libc::c_int as yytype_uint8,
     53 as libc::c_int as yytype_uint8, 54 as libc::c_int as yytype_uint8,
     55 as libc::c_int as yytype_uint8, 56 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     13 as libc::c_int as yytype_uint8, 64 as libc::c_int as yytype_uint8,
     65 as libc::c_int as yytype_uint8, 77 as libc::c_int as yytype_uint8,
     81 as libc::c_int as yytype_uint8, 77 as libc::c_int as yytype_uint8,
     73 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     78 as libc::c_int as yytype_uint8, 72 as libc::c_int as yytype_uint8,
     59 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 88 as libc::c_int as yytype_uint8,
     89 as libc::c_int as yytype_uint8, 83 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     19 as libc::c_int as yytype_uint8, 13 as libc::c_int as yytype_uint8,
     13 as libc::c_int as yytype_uint8, 28 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     84 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8,
     61 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 66 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 48 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 68 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     68 as libc::c_int as yytype_uint8, 48 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 62 as libc::c_int as yytype_uint8,
     65 as libc::c_int as yytype_uint8, 67 as libc::c_int as yytype_uint8,
     90 as libc::c_int as yytype_uint8, 91 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 65 as libc::c_int as yytype_uint8,
     81 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 66 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 13 as libc::c_int as yytype_uint8,
     73 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     59 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8,
     84 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     62 as libc::c_int as yytype_uint8, 79 as libc::c_int as yytype_uint8,
     80 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     91 as libc::c_int as yytype_uint8, 91 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     47 as libc::c_int as yytype_uint8, 42 as libc::c_int as yytype_uint8,
     43 as libc::c_int as yytype_uint8, 45 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 97 as libc::c_int as yytype_uint8,
     53 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     61 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     86 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     97 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 93 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 62 as libc::c_int as yytype_uint8,
     81 as libc::c_int as yytype_uint8, 94 as libc::c_int as yytype_uint8,
     95 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     40 as libc::c_int as yytype_uint8, 47 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 58 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     62 as libc::c_int as yytype_uint8, 89 as libc::c_int as yytype_uint8,
     45 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     61 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     20 as libc::c_int as yytype_uint8, 21 as libc::c_int as yytype_uint8,
     24 as libc::c_int as yytype_uint8, 85 as libc::c_int as yytype_uint8,
     60 as libc::c_int as yytype_uint8, 60 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     86 as libc::c_int as yytype_uint8, 47 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     48 as libc::c_int as yytype_uint8, 66 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 48 as libc::c_int as yytype_uint8,
     68 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 66 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     80 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     44 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     86 as libc::c_int as yytype_uint8, 92 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 92 as libc::c_int as yytype_uint8,
     61 as libc::c_int as yytype_uint8, 63 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 95 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 58 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 58 as libc::c_int as yytype_uint8,
     66 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     24 as libc::c_int as yytype_uint8, 19 as libc::c_int as yytype_uint8,
     59 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     63 as libc::c_int as yytype_uint8, 92 as libc::c_int as yytype_uint8,
     58 as libc::c_int as yytype_uint8, 59 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 92 as libc::c_int as yytype_uint8,
     85 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8,
     59 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 61 as libc::c_int as yytype_uint8];
/* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static mut yyr1: [yytype_uint8; 170] =
    [0 as libc::c_int as yytype_uint8, 69 as libc::c_int as yytype_uint8,
     70 as libc::c_int as yytype_uint8, 70 as libc::c_int as yytype_uint8,
     71 as libc::c_int as yytype_uint8, 71 as libc::c_int as yytype_uint8,
     72 as libc::c_int as yytype_uint8, 72 as libc::c_int as yytype_uint8,
     73 as libc::c_int as yytype_uint8, 73 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 74 as libc::c_int as yytype_uint8,
     74 as libc::c_int as yytype_uint8, 75 as libc::c_int as yytype_uint8,
     75 as libc::c_int as yytype_uint8, 76 as libc::c_int as yytype_uint8,
     76 as libc::c_int as yytype_uint8, 76 as libc::c_int as yytype_uint8,
     77 as libc::c_int as yytype_uint8, 78 as libc::c_int as yytype_uint8,
     78 as libc::c_int as yytype_uint8, 79 as libc::c_int as yytype_uint8,
     79 as libc::c_int as yytype_uint8, 80 as libc::c_int as yytype_uint8,
     80 as libc::c_int as yytype_uint8, 80 as libc::c_int as yytype_uint8,
     82 as libc::c_int as yytype_uint8, 81 as libc::c_int as yytype_uint8,
     83 as libc::c_int as yytype_uint8, 81 as libc::c_int as yytype_uint8,
     84 as libc::c_int as yytype_uint8, 84 as libc::c_int as yytype_uint8,
     84 as libc::c_int as yytype_uint8, 85 as libc::c_int as yytype_uint8,
     85 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     86 as libc::c_int as yytype_uint8, 86 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 87 as libc::c_int as yytype_uint8,
     87 as libc::c_int as yytype_uint8, 88 as libc::c_int as yytype_uint8,
     88 as libc::c_int as yytype_uint8, 89 as libc::c_int as yytype_uint8,
     90 as libc::c_int as yytype_uint8, 90 as libc::c_int as yytype_uint8,
     91 as libc::c_int as yytype_uint8, 91 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 92 as libc::c_int as yytype_uint8,
     92 as libc::c_int as yytype_uint8, 93 as libc::c_int as yytype_uint8,
     93 as libc::c_int as yytype_uint8, 94 as libc::c_int as yytype_uint8,
     94 as libc::c_int as yytype_uint8, 95 as libc::c_int as yytype_uint8,
     95 as libc::c_int as yytype_uint8, 95 as libc::c_int as yytype_uint8,
     95 as libc::c_int as yytype_uint8, 95 as libc::c_int as yytype_uint8,
     95 as libc::c_int as yytype_uint8, 95 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 96 as libc::c_int as yytype_uint8,
     96 as libc::c_int as yytype_uint8, 97 as libc::c_int as yytype_uint8,
     97 as libc::c_int as yytype_uint8, 97 as libc::c_int as yytype_uint8,
     97 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8,
     98 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8,
     98 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8,
     98 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8,
     98 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8,
     98 as libc::c_int as yytype_uint8, 98 as libc::c_int as yytype_uint8];
/* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static mut yyr2: [yytype_uint8; 170] =
    [0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 11 as libc::c_int as yytype_uint8,
     9 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     8 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     0 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     6 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     7 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     6 as libc::c_int as yytype_uint8, 6 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 5 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     4 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 0 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     3 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 4 as libc::c_int as yytype_uint8,
     2 as libc::c_int as yytype_uint8, 2 as libc::c_int as yytype_uint8,
     1 as libc::c_int as yytype_uint8, 1 as libc::c_int as yytype_uint8,
     5 as libc::c_int as yytype_uint8, 3 as libc::c_int as yytype_uint8];
/* Copy YYSRC to YYDEST, returning the address of the terminating '\0' in
   YYDEST.  */
unsafe extern "C" fn yystpcpy(mut yydest: *mut libc::c_char,
                              mut yysrc: *const libc::c_char)
 -> *mut libc::c_char {
    let mut yyd: *mut libc::c_char = yydest;
    let mut yys: *const libc::c_char = yysrc;
    loop  {
        let fresh0 = yys;
        yys = yys.offset(1);
        let fresh1 = yyd;
        yyd = yyd.offset(1);
        *fresh1 = *fresh0;
        if !(*fresh1 as libc::c_int != '\u{0}' as i32) { break ; }
    }
    return yyd.offset(-(1 as libc::c_int as isize));
}
/* Copy to YYRES the contents of YYSTR after stripping away unnecessary
   quotes and backslashes, so that it's suitable for yyerror.  The
   heuristic is that double-quoting is unnecessary unless the string
   contains an apostrophe, a comma, or backslash (other than
   backslash-backslash).  YYSTR is taken from yytname.  If YYRES is
   null, do not copy; instead, return the length of what the result
   would have been.  */
unsafe extern "C" fn yytnamerr(mut yyres: *mut libc::c_char,
                               mut yystr: *const libc::c_char)
 -> libc::c_ulong {
    if *yystr as libc::c_int == '\"' as i32 {
        let mut yyn: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut yyp: *const libc::c_char = yystr;
        loop  {
            yyp = yyp.offset(1);
            match *yyp as libc::c_int {
                39 | 44 => { break ; }
                92 => {
                    yyp = yyp.offset(1);
                    if *yyp as libc::c_int != '\\' as i32 { break ; }
                }
                34 => {
                    if !yyres.is_null() {
                        *yyres.offset(yyn as isize) =
                            '\u{0}' as i32 as libc::c_char
                    }
                    return yyn
                }
                _ => { }
            }
            if !yyres.is_null() { *yyres.offset(yyn as isize) = *yyp }
            yyn = yyn.wrapping_add(1)
        }
    }
    if yyres.is_null() { return strlen(yystr) }
    return yystpcpy(yyres, yystr).wrapping_offset_from(yyres) as libc::c_long
               as libc::c_ulong;
}
/* Copy into *YYMSG, which is of size *YYMSG_ALLOC, an error message
   about the unexpected token YYTOKEN for the state stack whose top is
   YYSSP.

   Return 0 if *YYMSG was successfully written.  Return 1 if *YYMSG is
   not large enough to hold the message.  In that case, also set
   *YYMSG_ALLOC to the required number of bytes.  Return 2 if the
   required number of bytes is too large to store.  */
unsafe extern "C" fn yysyntax_error(mut yymsg_alloc: *mut libc::c_ulong,
                                    mut yymsg: *mut *mut libc::c_char,
                                    mut yyssp: *mut yytype_int16,
                                    mut yytoken: libc::c_int) -> libc::c_int {
    let mut yysize0: libc::c_ulong =
        yytnamerr(0 as *mut libc::c_char, yytname[yytoken as usize]);
    let mut yysize: libc::c_ulong = yysize0;
    /* Internationalized format string. */
    let mut yyformat: *const libc::c_char = 0 as *const libc::c_char;
    /* Arguments of yyformat. */
    let mut yyarg: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5];
    /* Number of reported tokens (one for the "unexpected", one per
     "expected"). */
    let mut yycount: libc::c_int = 0 as libc::c_int;
    /* There are many possibilities here to consider:
     - If this state is a consistent state with a default action, then
       the only way this function was invoked is if the default action
       is an error action.  In that case, don't check for expected
       tokens because there are none.
     - The only way there can be no lookahead present (in yychar) is if
       this state is a consistent state with a default action.  Thus,
       detecting the absence of a lookahead is sufficient to determine
       that there is no unexpected or expected token to report.  In that
       case, just report a simple "syntax error".
     - Don't assume there isn't a lookahead just because this state is a
       consistent state with a default action.  There might have been a
       previous inconsistent state, consistent state with a non-default
       action, or user semantic action that manipulated yychar.
     - Of course, the expected token list depends on states to have
       correct lookahead information, and it depends on the parser not
       to perform extra reductions after fetching a lookahead from the
       scanner and before detecting a syntax error.  Thus, state merging
       (from LALR or IELR) and default reductions corrupt the expected
       token list.  However, the list is correct for canonical LR with
       one exception: it will still contain any token that will not be
       accepted due to an error action in a later state.
  */
    if yytoken != -(2 as libc::c_int) {
        let mut yyn: libc::c_int = yypact[*yyssp as usize] as libc::c_int;
        let fresh2 = yycount;
        yycount = yycount + 1;
        yyarg[fresh2 as usize] = yytname[yytoken as usize];
        if !(yyn == -(157 as libc::c_int)) {
            /* Start YYX at -YYN if negative to avoid negative indexes in
             YYCHECK.  In other words, skip the first -YYN actions for
             this state because they are default actions.  */
            let mut yyxbegin: libc::c_int =
                if yyn < 0 as libc::c_int { -yyn } else { 0 as libc::c_int };
            /* Stay within bounds of both yycheck and yytname.  */
            let mut yychecklim: libc::c_int =
                2220 as libc::c_int - yyn + 1 as libc::c_int;
            let mut yyxend: libc::c_int =
                if yychecklim < 69 as libc::c_int {
                    yychecklim
                } else { 69 as libc::c_int };
            let mut yyx: libc::c_int = 0;
            yyx = yyxbegin;
            while yyx < yyxend {
                if yycheck[(yyx + yyn) as usize] as libc::c_int == yyx &&
                       yyx != 1 as libc::c_int &&
                       !(yytable[(yyx + yyn) as usize] as libc::c_int ==
                             -(156 as libc::c_int)) {
                    if yycount == YYERROR_VERBOSE_ARGS_MAXIMUM as libc::c_int
                       {
                        yycount = 1 as libc::c_int;
                        yysize = yysize0;
                        break ;
                    } else {
                        let fresh3 = yycount;
                        yycount = yycount + 1;
                        yyarg[fresh3 as usize] = yytname[yyx as usize];
                        let mut yysize1: libc::c_ulong =
                            yysize.wrapping_add(yytnamerr(0 as
                                                              *mut libc::c_char,
                                                          yytname[yyx as
                                                                      usize]));
                        if yysize <= yysize1 &&
                               yysize1 <= -(1 as libc::c_int) as libc::c_ulong
                           {
                            yysize = yysize1
                        } else { return 2 as libc::c_int }
                    }
                }
                yyx += 1
            }
        }
    }
    match yycount {
        1 => {
            yyformat =
                b"syntax error, unexpected %s\x00" as *const u8 as
                    *const libc::c_char
        }
        2 => {
            yyformat =
                b"syntax error, unexpected %s, expecting %s\x00" as *const u8
                    as *const libc::c_char
        }
        3 => {
            yyformat =
                b"syntax error, unexpected %s, expecting %s or %s\x00" as
                    *const u8 as *const libc::c_char
        }
        4 => {
            yyformat =
                b"syntax error, unexpected %s, expecting %s or %s or %s\x00"
                    as *const u8 as *const libc::c_char
        }
        5 => {
            yyformat =
                b"syntax error, unexpected %s, expecting %s or %s or %s or %s\x00"
                    as *const u8 as *const libc::c_char
        }
        0 | _ => {
            /* Avoid compiler warnings. */
            yyformat = b"syntax error\x00" as *const u8 as *const libc::c_char
        }
    }
    let mut yysize1_0: libc::c_ulong = yysize.wrapping_add(strlen(yyformat));
    if yysize <= yysize1_0 &&
           yysize1_0 <= -(1 as libc::c_int) as libc::c_ulong {
        yysize = yysize1_0
    } else { return 2 as libc::c_int }
    if *yymsg_alloc < yysize {
        *yymsg_alloc =
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(yysize);
        if !(yysize <= *yymsg_alloc &&
                 *yymsg_alloc <= -(1 as libc::c_int) as libc::c_ulong) {
            *yymsg_alloc = -(1 as libc::c_int) as libc::c_ulong
        }
        return 1 as libc::c_int
    }
    /* Avoid sprintf, as that infringes on the user's name space.
     Don't have undefined behavior even if the translation
     produced a string with the wrong number of "%s"s.  */
    let mut yyp: *mut libc::c_char = *yymsg;
    let mut yyi: libc::c_int = 0 as libc::c_int;
    loop  {
        *yyp = *yyformat;
        if !(*yyp as libc::c_int != '\u{0}' as i32) { break ; }
        if *yyp as libc::c_int == '%' as i32 &&
               *yyformat.offset(1 as libc::c_int as isize) as libc::c_int ==
                   's' as i32 && yyi < yycount {
            let fresh4 = yyi;
            yyi = yyi + 1;
            yyp = yyp.offset(yytnamerr(yyp, yyarg[fresh4 as usize]) as isize);
            yyformat = yyformat.offset(2 as libc::c_int as isize)
        } else { yyp = yyp.offset(1); yyformat = yyformat.offset(1) }
    }
    return 0 as libc::c_int;
}
/* YYERROR_VERBOSE */
/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/
unsafe extern "C" fn yydestruct(mut yymsg: *const libc::c_char,
                                mut yytype: libc::c_int,
                                mut yyvaluep: *mut YYSTYPE,
                                mut yylocationp: *mut location,
                                mut answer: *mut block,
                                mut errors: *mut libc::c_int,
                                mut locations: *mut locfile,
                                mut lexer_param_ptr: *mut lexer_param) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    }
    match yytype {
        4 => { jv_free((*yyvaluep).literal); }
        5 => { jv_free((*yyvaluep).literal); }
        6 => { jv_free((*yyvaluep).literal); }
        7 => { jv_free((*yyvaluep).literal); }
        42 => { jv_free((*yyvaluep).literal); }
        71 => { block_free((*yyvaluep).blk); }
        72 => { block_free((*yyvaluep).blk); }
        73 => { block_free((*yyvaluep).blk); }
        74 => { block_free((*yyvaluep).blk); }
        75 => { block_free((*yyvaluep).blk); }
        76 => { block_free((*yyvaluep).blk); }
        77 => { block_free((*yyvaluep).blk); }
        78 => { block_free((*yyvaluep).blk); }
        79 => { block_free((*yyvaluep).blk); }
        80 => { block_free((*yyvaluep).blk); }
        81 => { block_free((*yyvaluep).blk); }
        84 => { block_free((*yyvaluep).blk); }
        85 => { block_free((*yyvaluep).blk); }
        86 => { block_free((*yyvaluep).blk); }
        87 => { block_free((*yyvaluep).blk); }
        88 => { block_free((*yyvaluep).blk); }
        89 => { block_free((*yyvaluep).blk); }
        90 => { block_free((*yyvaluep).blk); }
        91 => { block_free((*yyvaluep).blk); }
        92 => { block_free((*yyvaluep).blk); }
        93 => { block_free((*yyvaluep).blk); }
        94 => { block_free((*yyvaluep).blk); }
        95 => { block_free((*yyvaluep).blk); }
        96 => { jv_free((*yyvaluep).literal); }
        97 => { block_free((*yyvaluep).blk); }
        98 => { block_free((*yyvaluep).blk); }
        _ => { }
    };
}
