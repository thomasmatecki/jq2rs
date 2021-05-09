#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type inst;
    pub type jv_refcnt;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_sized(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_parse_sized(string: *const libc::c_char, length: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    fn clearerr(__stream: *mut FILE);
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
pub type int16_t = libc::c_short;
pub type uint8_t = libc::c_uchar;
pub type flex_uint8_t = uint8_t;
pub type flex_int16_t = int16_t;
/* An opaque pointer. */
pub type yyscan_t = *mut libc::c_void;
/* Return all but the first "n" matched characters back to the input stream. */
/* Undo effects of setting up yytext. */
/* set up yytext again */
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
/* __ia64__ */
/* The state buf must be large enough to hold one state per character in the main buffer.
 */
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type yy_size_t = size_t;
/* Special case for "unistd.h", since it is non-ANSI. We include it way
 * down here because we want the user's section 1 to have been scanned first.
 * The user has a chance to override it with an option.
 */
/* Holds the entire state of the reentrant scanner. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yyguts_t {
    pub yyextra_r: libc::c_int,
    pub yyin_r: *mut FILE,
    pub yyout_r: *mut FILE,
    pub yy_buffer_stack_top: size_t,
    pub yy_buffer_stack_max: size_t,
    pub yy_buffer_stack: *mut YY_BUFFER_STATE,
    pub yy_hold_char: libc::c_char,
    pub yy_n_chars: libc::c_int,
    pub yyleng_r: libc::c_int,
    pub yy_c_buf_p: *mut libc::c_char,
    pub yy_init: libc::c_int,
    pub yy_start: libc::c_int,
    pub yy_did_buffer_switch_on_eof: libc::c_int,
    pub yy_start_stack_ptr: libc::c_int,
    pub yy_start_stack_depth: libc::c_int,
    pub yy_start_stack: *mut libc::c_int,
    pub yy_last_accepting_state: yy_state_type,
    pub yy_last_accepting_cpos: *mut libc::c_char,
    pub yylineno_r: libc::c_int,
    pub yy_flex_debug_r: libc::c_int,
    pub yytext_r: *mut libc::c_char,
    pub yy_more_flag: libc::c_int,
    pub yy_more_len: libc::c_int,
    pub yylval_r: *mut YYSTYPE,
    pub yylloc_r: *mut location,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct location {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub literal: jv,
    pub blk: block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub first: *mut inst,
    pub last: *mut inst,
}
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
pub type yy_state_type = libc::c_int;
pub type YY_CHAR = flex_uint8_t;
unsafe extern "C" fn try_exit(mut c: libc::c_int, mut state: libc::c_int,
                              mut yyscanner: yyscan_t) -> libc::c_int {
    let mut match_0: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut ret: libc::c_int = 0;
    match state {
        1 => { ret = ')' as i32; match_0 = ret as libc::c_char }
        2 => { ret = ']' as i32; match_0 = ret as libc::c_char }
        3 => { ret = '}' as i32; match_0 = ret as libc::c_char }
        4 => {
            match_0 = ')' as i32 as libc::c_char;
            ret = 299 as libc::c_int
        }
        _ => {
            // may not be the best error to give
            return 258 as libc::c_int
        }
    }
    if match_0 as libc::c_int != 0 {
    } else {
        __assert_fail(b"match\x00" as *const u8 as *const libc::c_char,
                      b"src/lexer.l\x00" as *const u8 as *const libc::c_char,
                      154 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int try_exit(int, int, yyscan_t)\x00")).as_ptr());
    };
    if match_0 as libc::c_int == c {
        yy_pop_state(yyscanner);
        return ret
    } else {
        // FIXME: should we pop? Give a better error at least
        return 258 as libc::c_int
    };
}
unsafe extern "C" fn enter(mut c: libc::c_int, mut currstate: libc::c_int,
                           mut yyscanner: yyscan_t) -> libc::c_int {
    let mut state: libc::c_int = 0 as libc::c_int;
    match c {
        40 => { state = 1 as libc::c_int }
        91 => { state = 2 as libc::c_int }
        123 => { state = 3 as libc::c_int }
        298 => { state = 4 as libc::c_int }
        _ => { }
    }
    if state != 0 {
    } else {
        __assert_fail(b"state\x00" as *const u8 as *const libc::c_char,
                      b"src/lexer.l\x00" as *const u8 as *const libc::c_char,
                      172 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"int enter(int, int, yyscan_t)\x00")).as_ptr());
    };
    yy_push_state(state, yyscanner);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyalloc(mut sz: size_t,
                                    mut extra: *mut libc::c_void)
 -> *mut libc::c_void {
    return jv_mem_alloc(sz);
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyrealloc(mut p: *mut libc::c_void,
                                      mut sz: size_t,
                                      mut extra: *mut libc::c_void)
 -> *mut libc::c_void {
    return jv_mem_realloc(p, sz);
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyfree(mut p: *mut libc::c_void,
                                   mut extra: *mut libc::c_void) {
    jv_mem_free(p);
}
static mut yy_accept: [flex_int16_t; 157] =
    [0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 51 as libc::c_int as flex_int16_t,
     49 as libc::c_int as flex_int16_t, 48 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 49 as libc::c_int as flex_int16_t,
     40 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     36 as libc::c_int as flex_int16_t, 37 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 39 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     49 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     42 as libc::c_int as flex_int16_t, 45 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 2 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 29 as libc::c_int as flex_int16_t,
     27 as libc::c_int as flex_int16_t, 25 as libc::c_int as flex_int16_t,
     26 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 47 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 18 as libc::c_int as flex_int16_t,
     28 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     31 as libc::c_int as flex_int16_t, 3 as libc::c_int as flex_int16_t,
     32 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     38 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 4 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     9 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 14 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 24 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     41 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 30 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 34 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     13 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 8 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     15 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     19 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 12 as libc::c_int as flex_int16_t,
     11 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 10 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     22 as libc::c_int as flex_int16_t, 20 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 21 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 5 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 7 as libc::c_int as flex_int16_t,
     16 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     17 as libc::c_int as flex_int16_t, 6 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t];
static mut yy_ec: [YY_CHAR; 256] =
    [0 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 2 as libc::c_int as YY_CHAR,
     3 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     2 as libc::c_int as YY_CHAR, 5 as libc::c_int as YY_CHAR,
     6 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     8 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     10 as libc::c_int as YY_CHAR, 11 as libc::c_int as YY_CHAR,
     12 as libc::c_int as YY_CHAR, 13 as libc::c_int as YY_CHAR,
     14 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     16 as libc::c_int as YY_CHAR, 17 as libc::c_int as YY_CHAR,
     18 as libc::c_int as YY_CHAR, 18 as libc::c_int as YY_CHAR,
     18 as libc::c_int as YY_CHAR, 18 as libc::c_int as YY_CHAR,
     18 as libc::c_int as YY_CHAR, 18 as libc::c_int as YY_CHAR,
     18 as libc::c_int as YY_CHAR, 18 as libc::c_int as YY_CHAR,
     18 as libc::c_int as YY_CHAR, 18 as libc::c_int as YY_CHAR,
     19 as libc::c_int as YY_CHAR, 20 as libc::c_int as YY_CHAR,
     21 as libc::c_int as YY_CHAR, 22 as libc::c_int as YY_CHAR,
     23 as libc::c_int as YY_CHAR, 24 as libc::c_int as YY_CHAR,
     25 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 27 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 28 as libc::c_int as YY_CHAR,
     29 as libc::c_int as YY_CHAR, 30 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 31 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 32 as libc::c_int as YY_CHAR,
     33 as libc::c_int as YY_CHAR, 34 as libc::c_int as YY_CHAR,
     35 as libc::c_int as YY_CHAR, 36 as libc::c_int as YY_CHAR,
     37 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     38 as libc::c_int as YY_CHAR, 39 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 40 as libc::c_int as YY_CHAR,
     41 as libc::c_int as YY_CHAR, 42 as libc::c_int as YY_CHAR,
     43 as libc::c_int as YY_CHAR, 44 as libc::c_int as YY_CHAR,
     45 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     46 as libc::c_int as YY_CHAR, 47 as libc::c_int as YY_CHAR,
     48 as libc::c_int as YY_CHAR, 49 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 26 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 50 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 51 as libc::c_int as YY_CHAR,
     52 as libc::c_int as YY_CHAR, 53 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR];
static mut yy_meta: [YY_CHAR; 54] =
    [0 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 2 as libc::c_int as YY_CHAR,
     2 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     3 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     5 as libc::c_int as YY_CHAR, 6 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 8 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     7 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR];
static mut yy_base: [flex_int16_t; 170] =
    [0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 51 as libc::c_int as flex_int16_t,
     52 as libc::c_int as flex_int16_t, 320 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 57 as libc::c_int as flex_int16_t,
     59 as libc::c_int as flex_int16_t, 297 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 296 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     295 as libc::c_int as flex_int16_t, 294 as libc::c_int as flex_int16_t,
     293 as libc::c_int as flex_int16_t, 47 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     292 as libc::c_int as flex_int16_t, 291 as libc::c_int as flex_int16_t,
     290 as libc::c_int as flex_int16_t, 294 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 291 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 51 as libc::c_int as flex_int16_t,
     53 as libc::c_int as flex_int16_t, 52 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 59 as libc::c_int as flex_int16_t,
     57 as libc::c_int as flex_int16_t, 66 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 63 as libc::c_int as flex_int16_t,
     68 as libc::c_int as flex_int16_t, 70 as libc::c_int as flex_int16_t,
     72 as libc::c_int as flex_int16_t, 287 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 80 as libc::c_int as flex_int16_t,
     90 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     99 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     106 as libc::c_int as flex_int16_t, 286 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 110 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 290 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 285 as libc::c_int as flex_int16_t,
     281 as libc::c_int as flex_int16_t, 86 as libc::c_int as flex_int16_t,
     77 as libc::c_int as flex_int16_t, 277 as libc::c_int as flex_int16_t,
     97 as libc::c_int as flex_int16_t, 101 as libc::c_int as flex_int16_t,
     111 as libc::c_int as flex_int16_t, 113 as libc::c_int as flex_int16_t,
     115 as libc::c_int as flex_int16_t, 117 as libc::c_int as flex_int16_t,
     274 as libc::c_int as flex_int16_t, 119 as libc::c_int as flex_int16_t,
     120 as libc::c_int as flex_int16_t, 118 as libc::c_int as flex_int16_t,
     121 as libc::c_int as flex_int16_t, 270 as libc::c_int as flex_int16_t,
     122 as libc::c_int as flex_int16_t, 123 as libc::c_int as flex_int16_t,
     124 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 257 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 255 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 254 as libc::c_int as flex_int16_t,
     249 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     245 as libc::c_int as flex_int16_t, 321 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 125 as libc::c_int as flex_int16_t,
     239 as libc::c_int as flex_int16_t, 126 as libc::c_int as flex_int16_t,
     127 as libc::c_int as flex_int16_t, 237 as libc::c_int as flex_int16_t,
     128 as libc::c_int as flex_int16_t, 134 as libc::c_int as flex_int16_t,
     234 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     143 as libc::c_int as flex_int16_t, 147 as libc::c_int as flex_int16_t,
     148 as libc::c_int as flex_int16_t, 149 as libc::c_int as flex_int16_t,
     152 as libc::c_int as flex_int16_t, 154 as libc::c_int as flex_int16_t,
     232 as libc::c_int as flex_int16_t, 165 as libc::c_int as flex_int16_t,
     212 as libc::c_int as flex_int16_t, 210 as libc::c_int as flex_int16_t,
     157 as libc::c_int as flex_int16_t, 159 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     208 as libc::c_int as flex_int16_t, 160 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 162 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     166 as libc::c_int as flex_int16_t, 207 as libc::c_int as flex_int16_t,
     196 as libc::c_int as flex_int16_t, 171 as libc::c_int as flex_int16_t,
     205 as libc::c_int as flex_int16_t, 204 as libc::c_int as flex_int16_t,
     174 as libc::c_int as flex_int16_t, 167 as libc::c_int as flex_int16_t,
     175 as libc::c_int as flex_int16_t, 201 as libc::c_int as flex_int16_t,
     170 as libc::c_int as flex_int16_t, 176 as libc::c_int as flex_int16_t,
     190 as libc::c_int as flex_int16_t, 190 as libc::c_int as flex_int16_t,
     184 as libc::c_int as flex_int16_t, 199 as libc::c_int as flex_int16_t,
     194 as libc::c_int as flex_int16_t, 198 as libc::c_int as flex_int16_t,
     197 as libc::c_int as flex_int16_t, 85 as libc::c_int as flex_int16_t,
     78 as libc::c_int as flex_int16_t, 76 as libc::c_int as flex_int16_t,
     321 as libc::c_int as flex_int16_t, 230 as libc::c_int as flex_int16_t,
     239 as libc::c_int as flex_int16_t, 245 as libc::c_int as flex_int16_t,
     250 as libc::c_int as flex_int16_t, 255 as libc::c_int as flex_int16_t,
     264 as libc::c_int as flex_int16_t, 273 as libc::c_int as flex_int16_t,
     278 as libc::c_int as flex_int16_t, 283 as libc::c_int as flex_int16_t,
     285 as libc::c_int as flex_int16_t, 290 as libc::c_int as flex_int16_t,
     294 as libc::c_int as flex_int16_t, 298 as libc::c_int as flex_int16_t];
static mut yy_def: [flex_int16_t; 170] =
    [0 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     157 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 158 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 159 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     160 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     162 as libc::c_int as flex_int16_t, 162 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     164 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     160 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     162 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 165 as libc::c_int as flex_int16_t,
     164 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     164 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     166 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     167 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     168 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     169 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t];
static mut yy_nxt: [flex_int16_t; 375] =
    [0 as libc::c_int as flex_int16_t, 14 as libc::c_int as flex_int16_t,
     15 as libc::c_int as flex_int16_t, 16 as libc::c_int as flex_int16_t,
     14 as libc::c_int as flex_int16_t, 17 as libc::c_int as flex_int16_t,
     18 as libc::c_int as flex_int16_t, 19 as libc::c_int as flex_int16_t,
     20 as libc::c_int as flex_int16_t, 21 as libc::c_int as flex_int16_t,
     22 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     24 as libc::c_int as flex_int16_t, 25 as libc::c_int as flex_int16_t,
     20 as libc::c_int as flex_int16_t, 26 as libc::c_int as flex_int16_t,
     27 as libc::c_int as flex_int16_t, 28 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 20 as libc::c_int as flex_int16_t,
     20 as libc::c_int as flex_int16_t, 30 as libc::c_int as flex_int16_t,
     31 as libc::c_int as flex_int16_t, 32 as libc::c_int as flex_int16_t,
     33 as libc::c_int as flex_int16_t, 34 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     22 as libc::c_int as flex_int16_t, 14 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 36 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 38 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 40 as libc::c_int as flex_int16_t,
     41 as libc::c_int as flex_int16_t, 42 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     45 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 22 as libc::c_int as flex_int16_t,
     49 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     51 as libc::c_int as flex_int16_t, 51 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 52 as libc::c_int as flex_int16_t,
     52 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 61 as libc::c_int as flex_int16_t,
     65 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     62 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     62 as libc::c_int as flex_int16_t, 66 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 80 as libc::c_int as flex_int16_t,
     64 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 75 as libc::c_int as flex_int16_t,
     53 as libc::c_int as flex_int16_t, 53 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 64 as libc::c_int as flex_int16_t,
     79 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     67 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     87 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     96 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     76 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     77 as libc::c_int as flex_int16_t, 78 as libc::c_int as flex_int16_t,
     81 as libc::c_int as flex_int16_t, 83 as libc::c_int as flex_int16_t,
     82 as libc::c_int as flex_int16_t, 84 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     90 as libc::c_int as flex_int16_t, 88 as libc::c_int as flex_int16_t,
     85 as libc::c_int as flex_int16_t, 86 as libc::c_int as flex_int16_t,
     91 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     106 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     89 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     92 as libc::c_int as flex_int16_t, 99 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 99 as libc::c_int as flex_int16_t,
     67 as libc::c_int as flex_int16_t, 99 as libc::c_int as flex_int16_t,
     100 as libc::c_int as flex_int16_t, 99 as libc::c_int as flex_int16_t,
     67 as libc::c_int as flex_int16_t, 105 as libc::c_int as flex_int16_t,
     102 as libc::c_int as flex_int16_t, 97 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 107 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     109 as libc::c_int as flex_int16_t, 108 as libc::c_int as flex_int16_t,
     112 as libc::c_int as flex_int16_t, 116 as libc::c_int as flex_int16_t,
     110 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     115 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     117 as libc::c_int as flex_int16_t, 118 as libc::c_int as flex_int16_t,
     125 as libc::c_int as flex_int16_t, 119 as libc::c_int as flex_int16_t,
     111 as libc::c_int as flex_int16_t, 126 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 113 as libc::c_int as flex_int16_t,
     114 as libc::c_int as flex_int16_t, 127 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 124 as libc::c_int as flex_int16_t,
     128 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     129 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     120 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     132 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 130 as libc::c_int as flex_int16_t,
     131 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 137 as libc::c_int as flex_int16_t,
     140 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     139 as libc::c_int as flex_int16_t, 135 as libc::c_int as flex_int16_t,
     133 as libc::c_int as flex_int16_t, 138 as libc::c_int as flex_int16_t,
     145 as libc::c_int as flex_int16_t, 134 as libc::c_int as flex_int16_t,
     147 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     143 as libc::c_int as flex_int16_t, 144 as libc::c_int as flex_int16_t,
     151 as libc::c_int as flex_int16_t, 141 as libc::c_int as flex_int16_t,
     148 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     150 as libc::c_int as flex_int16_t, 142 as libc::c_int as flex_int16_t,
     152 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     97 as libc::c_int as flex_int16_t, 149 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 121 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 153 as libc::c_int as flex_int16_t,
     154 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 121 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     155 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 121 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 56 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 56 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 56 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 63 as libc::c_int as flex_int16_t,
     63 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     63 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     63 as libc::c_int as flex_int16_t, 72 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 72 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 72 as libc::c_int as flex_int16_t,
     73 as libc::c_int as flex_int16_t, 73 as libc::c_int as flex_int16_t,
     73 as libc::c_int as flex_int16_t, 102 as libc::c_int as flex_int16_t,
     73 as libc::c_int as flex_int16_t, 94 as libc::c_int as flex_int16_t,
     94 as libc::c_int as flex_int16_t, 100 as libc::c_int as flex_int16_t,
     94 as libc::c_int as flex_int16_t, 94 as libc::c_int as flex_int16_t,
     94 as libc::c_int as flex_int16_t, 94 as libc::c_int as flex_int16_t,
     102 as libc::c_int as flex_int16_t, 94 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 98 as libc::c_int as flex_int16_t,
     121 as libc::c_int as flex_int16_t, 98 as libc::c_int as flex_int16_t,
     121 as libc::c_int as flex_int16_t, 98 as libc::c_int as flex_int16_t,
     122 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     122 as libc::c_int as flex_int16_t, 122 as libc::c_int as flex_int16_t,
     123 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     123 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     136 as libc::c_int as flex_int16_t, 146 as libc::c_int as flex_int16_t,
     104 as libc::c_int as flex_int16_t, 146 as libc::c_int as flex_int16_t,
     146 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 95 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 103 as libc::c_int as flex_int16_t,
     101 as libc::c_int as flex_int16_t, 93 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 71 as libc::c_int as flex_int16_t,
     70 as libc::c_int as flex_int16_t, 69 as libc::c_int as flex_int16_t,
     68 as libc::c_int as flex_int16_t, 60 as libc::c_int as flex_int16_t,
     59 as libc::c_int as flex_int16_t, 58 as libc::c_int as flex_int16_t,
     57 as libc::c_int as flex_int16_t, 55 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 13 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t];
static mut yy_chk: [flex_int16_t; 375] =
    [0 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     11 as libc::c_int as flex_int16_t, 12 as libc::c_int as flex_int16_t,
     40 as libc::c_int as flex_int16_t, 11 as libc::c_int as flex_int16_t,
     12 as libc::c_int as flex_int16_t, 15 as libc::c_int as flex_int16_t,
     15 as libc::c_int as flex_int16_t, 16 as libc::c_int as flex_int16_t,
     16 as libc::c_int as flex_int16_t, 27 as libc::c_int as flex_int16_t,
     28 as libc::c_int as flex_int16_t, 27 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 36 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 28 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 39 as libc::c_int as flex_int16_t,
     38 as libc::c_int as flex_int16_t, 40 as libc::c_int as flex_int16_t,
     27 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     42 as libc::c_int as flex_int16_t, 29 as libc::c_int as flex_int16_t,
     41 as libc::c_int as flex_int16_t, 36 as libc::c_int as flex_int16_t,
     11 as libc::c_int as flex_int16_t, 12 as libc::c_int as flex_int16_t,
     45 as libc::c_int as flex_int16_t, 27 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 47 as libc::c_int as flex_int16_t,
     53 as libc::c_int as flex_int16_t, 48 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 155 as libc::c_int as flex_int16_t,
     76 as libc::c_int as flex_int16_t, 154 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 38 as libc::c_int as flex_int16_t,
     41 as libc::c_int as flex_int16_t, 42 as libc::c_int as flex_int16_t,
     41 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     153 as libc::c_int as flex_int16_t, 75 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 45 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 61 as libc::c_int as flex_int16_t,
     76 as libc::c_int as flex_int16_t, 61 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     78 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 64 as libc::c_int as flex_int16_t,
     79 as libc::c_int as flex_int16_t, 64 as libc::c_int as flex_int16_t,
     61 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     64 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     62 as libc::c_int as flex_int16_t, 75 as libc::c_int as flex_int16_t,
     67 as libc::c_int as flex_int16_t, 53 as libc::c_int as flex_int16_t,
     80 as libc::c_int as flex_int16_t, 61 as libc::c_int as flex_int16_t,
     81 as libc::c_int as flex_int16_t, 78 as libc::c_int as flex_int16_t,
     82 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     83 as libc::c_int as flex_int16_t, 87 as libc::c_int as flex_int16_t,
     85 as libc::c_int as flex_int16_t, 86 as libc::c_int as flex_int16_t,
     88 as libc::c_int as flex_int16_t, 90 as libc::c_int as flex_int16_t,
     91 as libc::c_int as flex_int16_t, 92 as libc::c_int as flex_int16_t,
     105 as libc::c_int as flex_int16_t, 107 as libc::c_int as flex_int16_t,
     108 as libc::c_int as flex_int16_t, 110 as libc::c_int as flex_int16_t,
     80 as libc::c_int as flex_int16_t, 79 as libc::c_int as flex_int16_t,
     82 as libc::c_int as flex_int16_t, 87 as libc::c_int as flex_int16_t,
     81 as libc::c_int as flex_int16_t, 111 as libc::c_int as flex_int16_t,
     86 as libc::c_int as flex_int16_t, 113 as libc::c_int as flex_int16_t,
     88 as libc::c_int as flex_int16_t, 90 as libc::c_int as flex_int16_t,
     107 as libc::c_int as flex_int16_t, 91 as libc::c_int as flex_int16_t,
     81 as libc::c_int as flex_int16_t, 108 as libc::c_int as flex_int16_t,
     114 as libc::c_int as flex_int16_t, 83 as libc::c_int as flex_int16_t,
     85 as libc::c_int as flex_int16_t, 110 as libc::c_int as flex_int16_t,
     115 as libc::c_int as flex_int16_t, 116 as libc::c_int as flex_int16_t,
     117 as libc::c_int as flex_int16_t, 105 as libc::c_int as flex_int16_t,
     111 as libc::c_int as flex_int16_t, 118 as libc::c_int as flex_int16_t,
     113 as libc::c_int as flex_int16_t, 119 as libc::c_int as flex_int16_t,
     92 as libc::c_int as flex_int16_t, 121 as libc::c_int as flex_int16_t,
     124 as libc::c_int as flex_int16_t, 126 as libc::c_int as flex_int16_t,
     125 as libc::c_int as flex_int16_t, 129 as libc::c_int as flex_int16_t,
     130 as libc::c_int as flex_int16_t, 131 as libc::c_int as flex_int16_t,
     132 as libc::c_int as flex_int16_t, 133 as libc::c_int as flex_int16_t,
     116 as libc::c_int as flex_int16_t, 134 as libc::c_int as flex_int16_t,
     141 as libc::c_int as flex_int16_t, 114 as libc::c_int as flex_int16_t,
     115 as libc::c_int as flex_int16_t, 144 as libc::c_int as flex_int16_t,
     137 as libc::c_int as flex_int16_t, 124 as libc::c_int as flex_int16_t,
     129 as libc::c_int as flex_int16_t, 140 as libc::c_int as flex_int16_t,
     142 as libc::c_int as flex_int16_t, 145 as libc::c_int as flex_int16_t,
     126 as libc::c_int as flex_int16_t, 119 as libc::c_int as flex_int16_t,
     117 as libc::c_int as flex_int16_t, 125 as libc::c_int as flex_int16_t,
     134 as libc::c_int as flex_int16_t, 118 as libc::c_int as flex_int16_t,
     137 as libc::c_int as flex_int16_t, 148 as libc::c_int as flex_int16_t,
     132 as libc::c_int as flex_int16_t, 133 as libc::c_int as flex_int16_t,
     144 as libc::c_int as flex_int16_t, 130 as libc::c_int as flex_int16_t,
     140 as libc::c_int as flex_int16_t, 147 as libc::c_int as flex_int16_t,
     142 as libc::c_int as flex_int16_t, 131 as libc::c_int as flex_int16_t,
     145 as libc::c_int as flex_int16_t, 150 as libc::c_int as flex_int16_t,
     121 as libc::c_int as flex_int16_t, 141 as libc::c_int as flex_int16_t,
     152 as libc::c_int as flex_int16_t, 151 as libc::c_int as flex_int16_t,
     149 as libc::c_int as flex_int16_t, 146 as libc::c_int as flex_int16_t,
     143 as libc::c_int as flex_int16_t, 147 as libc::c_int as flex_int16_t,
     148 as libc::c_int as flex_int16_t, 139 as libc::c_int as flex_int16_t,
     138 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     135 as libc::c_int as flex_int16_t, 128 as libc::c_int as flex_int16_t,
     127 as libc::c_int as flex_int16_t, 123 as libc::c_int as flex_int16_t,
     150 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     157 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     157 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     157 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     157 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 122 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 158 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 158 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 158 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 159 as libc::c_int as flex_int16_t,
     159 as libc::c_int as flex_int16_t, 120 as libc::c_int as flex_int16_t,
     159 as libc::c_int as flex_int16_t, 112 as libc::c_int as flex_int16_t,
     159 as libc::c_int as flex_int16_t, 160 as libc::c_int as flex_int16_t,
     109 as libc::c_int as flex_int16_t, 160 as libc::c_int as flex_int16_t,
     106 as libc::c_int as flex_int16_t, 160 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 102 as libc::c_int as flex_int16_t,
     161 as libc::c_int as flex_int16_t, 162 as libc::c_int as flex_int16_t,
     162 as libc::c_int as flex_int16_t, 100 as libc::c_int as flex_int16_t,
     162 as libc::c_int as flex_int16_t, 162 as libc::c_int as flex_int16_t,
     162 as libc::c_int as flex_int16_t, 162 as libc::c_int as flex_int16_t,
     99 as libc::c_int as flex_int16_t, 162 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     97 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     165 as libc::c_int as flex_int16_t, 89 as libc::c_int as flex_int16_t,
     165 as libc::c_int as flex_int16_t, 165 as libc::c_int as flex_int16_t,
     166 as libc::c_int as flex_int16_t, 84 as libc::c_int as flex_int16_t,
     166 as libc::c_int as flex_int16_t, 167 as libc::c_int as flex_int16_t,
     77 as libc::c_int as flex_int16_t, 167 as libc::c_int as flex_int16_t,
     167 as libc::c_int as flex_int16_t, 168 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 168 as libc::c_int as flex_int16_t,
     168 as libc::c_int as flex_int16_t, 169 as libc::c_int as flex_int16_t,
     73 as libc::c_int as flex_int16_t, 169 as libc::c_int as flex_int16_t,
     169 as libc::c_int as flex_int16_t, 71 as libc::c_int as flex_int16_t,
     65 as libc::c_int as flex_int16_t, 49 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     32 as libc::c_int as flex_int16_t, 31 as libc::c_int as flex_int16_t,
     30 as libc::c_int as flex_int16_t, 26 as libc::c_int as flex_int16_t,
     25 as libc::c_int as flex_int16_t, 24 as libc::c_int as flex_int16_t,
     21 as libc::c_int as flex_int16_t, 17 as libc::c_int as flex_int16_t,
     13 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 156 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t];
/* Report a fatal error. */
/* end tables serialization structures and prototypes */
/* Default declaration of generated scanner - a define so the user can
 * easily add parameters.
 */
/* !YY_DECL */
/* Code executed at the beginning of each rule, after yytext and yyleng
 * have been set up.
 */
/* Code executed at the end of each rule. */
/*LINTED*/
/* * The main scanner function which does all the work.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yylex(mut yylval_param: *mut YYSTYPE,
                                  mut yylloc_param: *mut location,
                                  mut yyscanner: yyscan_t) -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int =
        0; /* first start state */
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yylval_r = yylval_param;
    (*yyg).yylloc_r = yylloc_param;
    if (*yyg).yy_init == 0 {
        (*yyg).yy_init = 1 as libc::c_int;
        if (*yyg).yy_start == 0 { (*yyg).yy_start = 1 as libc::c_int }
        if (*yyg).yyin_r.is_null() { (*yyg).yyin_r = stdin }
        if (*yyg).yyout_r.is_null() { (*yyg).yyout_r = stdout }
        if if !(*yyg).yy_buffer_stack.is_null() {
               *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                  isize)
           } else { 0 as YY_BUFFER_STATE }.is_null() {
            jq_yyensure_buffer_stack(yyscanner);
            let ref mut fresh0 =
                *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                   isize);
            *fresh0 =
                jq_yy_create_buffer((*yyg).yyin_r, 16384 as libc::c_int,
                                    yyscanner)
        }
        jq_yy_load_buffer_state(yyscanner);
    }
    loop 
         /* loops until end-of-file is reached */
         {
        yy_cp = (*yyg).yy_c_buf_p;
        /* end of action switch */
        /* Support of yytext. */
        *yy_cp = (*yyg).yy_hold_char;
        /* yy_bp points to the position in yy_ch_buf of the start of
		 * the current run.
		 */
        yy_bp = yy_cp;
        yy_current_state = (*yyg).yy_start;
        'c_11379:
            loop  {
                loop  {
                    let mut yy_c: YY_CHAR = yy_ec[*yy_cp as YY_CHAR as usize];
                    if yy_accept[yy_current_state as usize] != 0 {
                        (*yyg).yy_last_accepting_state = yy_current_state;
                        (*yyg).yy_last_accepting_cpos = yy_cp
                    }
                    while yy_chk[(yy_base[yy_current_state as usize] as
                                      libc::c_int + yy_c as libc::c_int) as
                                     usize] as libc::c_int != yy_current_state
                          {
                        yy_current_state =
                            yy_def[yy_current_state as usize] as libc::c_int;
                        if yy_current_state >= 157 as libc::c_int {
                            yy_c = yy_meta[yy_c as usize]
                        }
                    }
                    yy_current_state =
                        yy_nxt[(yy_base[yy_current_state as usize] as
                                    libc::c_int + yy_c as libc::c_int) as
                                   usize] as yy_state_type;
                    yy_cp = yy_cp.offset(1);
                    if !(yy_base[yy_current_state as usize] as libc::c_int !=
                             321 as libc::c_int) {
                        break ;
                    }
                }
                'c_11380:
                    loop  {
                        yy_act =
                            yy_accept[yy_current_state as usize] as
                                libc::c_int;
                        if yy_act == 0 as libc::c_int {
                            /* have to back up */
                            yy_cp = (*yyg).yy_last_accepting_cpos;
                            yy_current_state = (*yyg).yy_last_accepting_state;
                            yy_act =
                                yy_accept[yy_current_state as usize] as
                                    libc::c_int
                        }
                        (*yyg).yytext_r = yy_bp;
                        (*yyg).yyleng_r =
                            yy_cp.wrapping_offset_from(yy_bp) as libc::c_long
                                as libc::c_int;
                        (*yyg).yy_hold_char = *yy_cp;
                        *yy_cp = '\u{0}' as i32 as libc::c_char;
                        (*yyg).yy_c_buf_p = yy_cp;
                        loop 
                             /* This label is used only to access EOF actions. */
                             {
                            match yy_act {
                                0 => {
                                    /* beginning of action switch */
                                    /* must back up */
                                    /* undo the effects of YY_DO_BEFORE_ACTION */
                                    *yy_cp = (*yyg).yy_hold_char;
                                    yy_cp = (*yyg).yy_last_accepting_cpos;
                                    yy_current_state =
                                        (*yyg).yy_last_accepting_state;
                                    continue 'c_11380 ;
                                }
                                1 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    break 'c_11379 ;
                                }
                                2 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 266 as libc::c_int
                                }
                                3 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 265 as libc::c_int
                                }
                                4 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 268 as libc::c_int
                                }
                                5 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 271 as libc::c_int
                                }
                                6 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 272 as libc::c_int
                                }
                                7 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 270 as libc::c_int
                                }
                                8 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 269 as libc::c_int
                                }
                                9 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 273 as libc::c_int
                                }
                                10 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 274 as libc::c_int
                                }
                                11 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 275 as libc::c_int
                                }
                                12 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 276 as libc::c_int
                                }
                                13 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 280 as libc::c_int
                                }
                                14 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 281 as libc::c_int
                                }
                                15 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 279 as libc::c_int
                                }
                                16 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 277 as libc::c_int
                                }
                                17 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 278 as libc::c_int
                                }
                                18 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 267 as libc::c_int
                                }
                                19 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 282 as libc::c_int
                                }
                                20 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 283 as libc::c_int
                                }
                                21 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 284 as libc::c_int
                                }
                                22 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 285 as libc::c_int
                                }
                                23 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 286 as libc::c_int
                                }
                                24 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 287 as libc::c_int
                                }
                                25 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 288 as libc::c_int
                                }
                                26 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 289 as libc::c_int
                                }
                                27 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 290 as libc::c_int
                                }
                                28 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 291 as libc::c_int
                                }
                                29 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 264 as libc::c_int
                                }
                                30 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 292 as libc::c_int
                                }
                                31 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 293 as libc::c_int
                                }
                                32 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 294 as libc::c_int
                                }
                                33 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 263 as libc::c_int
                                }
                                34 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 295 as libc::c_int
                                }
                                35 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return *(*yyg).yytext_r.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                               as libc::c_int
                                }
                                36 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return enter(*(*yyg).yytext_r.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                     as libc::c_int,
                                                 ((*yyg).yy_start -
                                                      1 as libc::c_int) /
                                                     2 as libc::c_int,
                                                 yyscanner)
                                }
                                37 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return try_exit(*(*yyg).yytext_r.offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                        as libc::c_int,
                                                    ((*yyg).yy_start -
                                                         1 as libc::c_int) /
                                                        2 as libc::c_int,
                                                    yyscanner)
                                }
                                38 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    (*(*yyg).yylval_r).literal =
                                        jv_string_sized((*yyg).yytext_r.offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                                        (*yyg).yyleng_r -
                                                            1 as libc::c_int);
                                    return 262 as libc::c_int
                                }
                                39 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    (*(*yyg).yylval_r).literal =
                                        jv_parse_sized((*yyg).yytext_r,
                                                       (*yyg).yyleng_r);
                                    return 261 as libc::c_int
                                }
                                40 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    yy_push_state(5 as libc::c_int,
                                                  yyscanner);
                                    return 296 as libc::c_int
                                }
                                41 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return enter(298 as libc::c_int,
                                                 ((*yyg).yy_start -
                                                      1 as libc::c_int) /
                                                     2 as libc::c_int,
                                                 yyscanner)
                                }
                                42 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    yy_pop_state(yyscanner);
                                    return 300 as libc::c_int
                                }
                                43 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    let mut escapes: jv =
                                        jv_string_fmt(b"\"%.*s\"\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      (*yyg).yyleng_r,
                                                      (*yyg).yytext_r);
                                    (*(*yyg).yylval_r).literal =
                                        jv_parse_sized(jv_string_value(escapes),
                                                       jv_string_length_bytes(jv_copy(escapes)));
                                    jv_free(escapes);
                                    return 297 as libc::c_int
                                }
                                44 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    (*(*yyg).yylval_r).literal =
                                        jv_string_sized((*yyg).yytext_r,
                                                        (*yyg).yyleng_r);
                                    return 297 as libc::c_int
                                }
                                45 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 258 as libc::c_int
                                }
                                46 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    (*(*yyg).yylval_r).literal =
                                        jv_string((*yyg).yytext_r);
                                    return 259 as libc::c_int
                                }
                                47 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    (*(*yyg).yylval_r).literal =
                                        jv_string((*yyg).yytext_r.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
                                    return 260 as libc::c_int
                                }
                                48 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    break 'c_11379 ;
                                }
                                49 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    return 258 as libc::c_int
                                }
                                50 => {
                                    (*(*yyg).yylloc_r).start =
                                        jq_yyget_extra(yyscanner);
                                    (*(*yyg).yylloc_r).end =
                                        (*(*yyg).yylloc_r).start +
                                            (*yyg).yyleng_r;
                                    jq_yyset_extra((*(*yyg).yylloc_r).end,
                                                   yyscanner);
                                    yy_fatal_error(b"flex scanner jammed\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   yyscanner);
                                }
                                52 | 53 | 54 | 55 | 56 | 57 => {
                                    return 0 as libc::c_int
                                }
                                51 => {
                                    /* Amount of text matched not including the EOB char. */
                                    yy_amount_of_matched_text =
                                        yy_cp.wrapping_offset_from((*yyg).yytext_r)
                                            as libc::c_long as libc::c_int -
                                            1 as libc::c_int;
                                    /* Undo the effects of YY_DO_BEFORE_ACTION. */
                                    *yy_cp = (*yyg).yy_hold_char;
                                    if (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                            as
                                                                            isize)).yy_buffer_status
                                           == 0 as libc::c_int {
                                        /* We're scanning a new file or input source.  It's
			 * possible that this happened because the user
			 * just pointed yyin at a new source and called
			 * yylex().  If so, then we have to assure
			 * consistency between YY_CURRENT_BUFFER and our
			 * globals.  Here is the right place to do so, because
			 * this is the first action (other than possibly a
			 * back-up) that will match for the new input source.
			 */
                                        (*yyg).yy_n_chars =
                                            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                                 as
                                                                                 isize)).yy_n_chars;
                                        let ref mut fresh1 =
                                            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                                 as
                                                                                 isize)).yy_input_file;
                                        *fresh1 = (*yyg).yyin_r;
                                        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                             as
                                                                             isize)).yy_buffer_status
                                            = 1 as libc::c_int
                                    }
                                    /* Note that here we test for yy_c_buf_p "<=" to the position
		 * of the first EOB in the buffer, since yy_c_buf_p will
		 * already have been incremented past the NUL character
		 * (since all states make transitions on EOB to the
		 * end-of-buffer state).  Contrast this with the test
		 * in input().
		 */
                                    if (*yyg).yy_c_buf_p <=
                                           &mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                                      as
                                                                                      isize)).yy_ch_buf.offset((*yyg).yy_n_chars
                                                                                                                   as
                                                                                                                   isize)
                                               as *mut libc::c_char {
                                        /* This was really a NUL. */
                                        yy_next_state = 0;
                                        (*yyg).yy_c_buf_p =
                                            (*yyg).yytext_r.offset(yy_amount_of_matched_text
                                                                       as
                                                                       isize);
                                        yy_current_state =
                                            yy_get_previous_state(yyscanner);
                                        /* Okay, we're now positioned to make the NUL
			 * transition.  We couldn't have
			 * yy_get_previous_state() go ahead and do it
			 * for us because it doesn't know how to deal
			 * with the possibility of jamming (and we don't
			 * want to build jamming into it because then it
			 * will run more slowly).
			 */
                                        yy_next_state =
                                            yy_try_NUL_trans(yy_current_state,
                                                             yyscanner);
                                        yy_bp =
                                            (*yyg).yytext_r.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize);
                                        if yy_next_state != 0 {
                                            current_block =
                                                11384774165255229092;
                                            break ;
                                        } else {
                                            current_block =
                                                3871889238474132049;
                                            break ;
                                        }
                                    } else {
                                        match yy_get_next_buffer(yyscanner) {
                                            1 => {
                                                (*yyg).yy_did_buffer_switch_on_eof
                                                    = 0 as libc::c_int;
                                                /* Note: because we've taken care in
					 * yy_get_next_buffer() to have set up
					 * yytext, we can now set up
					 * yy_c_buf_p so that if some total
					 * hoser (like flex itself) wants to
					 * call the scanner after we return the
					 * YY_NULL, it'll still work - another
					 * YY_NULL will get returned.
					 */
                                                (*yyg).yy_c_buf_p =
                                                    (*yyg).yytext_r.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize);
                                                yy_act =
                                                    51 as libc::c_int +
                                                        ((*yyg).yy_start -
                                                             1 as libc::c_int)
                                                            / 2 as libc::c_int
                                                        + 1 as libc::c_int
                                            }
                                            0 => {
                                                (*yyg).yy_c_buf_p =
                                                    (*yyg).yytext_r.offset(yy_amount_of_matched_text
                                                                               as
                                                                               isize);
                                                yy_current_state =
                                                    yy_get_previous_state(yyscanner);
                                                yy_cp = (*yyg).yy_c_buf_p;
                                                yy_bp =
                                                    (*yyg).yytext_r.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize);
                                                break 'c_11380 ;
                                            }
                                            2 => {
                                                (*yyg).yy_c_buf_p =
                                                    &mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                                               as
                                                                                               isize)).yy_ch_buf.offset((*yyg).yy_n_chars
                                                                                                                            as
                                                                                                                            isize)
                                                        as *mut libc::c_char;
                                                yy_current_state =
                                                    yy_get_previous_state(yyscanner);
                                                yy_cp = (*yyg).yy_c_buf_p;
                                                yy_bp =
                                                    (*yyg).yytext_r.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize);
                                                continue 'c_11380 ;
                                            }
                                            _ => { break 'c_11379 ; }
                                        }
                                    }
                                }
                                _ => {
                                    yy_fatal_error(b"fatal flex scanner internal error--no action found\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   yyscanner);
                                }
                            }
                        }
                        match current_block {
                            3871889238474132049 => {
                                yy_cp = (*yyg).yy_c_buf_p
                            }
                            _ => {
                                /* Consume the NUL. */
                                (*yyg).yy_c_buf_p =
                                    (*yyg).yy_c_buf_p.offset(1);
                                yy_cp = (*yyg).yy_c_buf_p;
                                yy_current_state = yy_next_state;
                                break ;
                            }
                        }
                    }
            }
    };
    /* end of scanning one token */
    /* end of user's declarations */
}
/* end of yylex */
/* yy_get_next_buffer - try to read in a new buffer
 *
 * Returns a code representing an action:
 *	EOB_ACT_LAST_MATCH -
 *	EOB_ACT_CONTINUE_SCAN - continue scanning from current position
 *	EOB_ACT_END_OF_FILE - end of file
 */
unsafe extern "C" fn yy_get_next_buffer(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    let mut dest: *mut libc::c_char =
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_ch_buf;
    let mut source: *mut libc::c_char = (*yyg).yytext_r;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if (*yyg).yy_c_buf_p >
           &mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                      as
                                                      isize)).yy_ch_buf.offset(((*yyg).yy_n_chars
                                                                                    +
                                                                                    1
                                                                                        as
                                                                                        libc::c_int)
                                                                                   as
                                                                                   isize)
               as *mut libc::c_char {
        yy_fatal_error(b"fatal flex scanner internal error--end of buffer missed\x00"
                           as *const u8 as *const libc::c_char, yyscanner);
    }
    if (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                            isize)).yy_fill_buffer ==
           0 as libc::c_int {
        /* Don't try to fill the buffer, so this is an EOF. */
        if (*yyg).yy_c_buf_p.wrapping_offset_from((*yyg).yytext_r) as
               libc::c_long - 0 as libc::c_int as libc::c_long ==
               1 as libc::c_int as libc::c_long {
            /* We matched a single character, the EOB, so
			 * treat this as a final EOF.
			 */
            return 1 as libc::c_int
        } else {
            /* We matched some text prior to the EOB, first
			 * process it.
			 */
            return 2 as libc::c_int
        }
    }
    /* Try to read more data. */
    /* First move last chars to start of buffer. */
    number_to_move =
        ((*yyg).yy_c_buf_p.wrapping_offset_from((*yyg).yytext_r) as
             libc::c_long - 1 as libc::c_int as libc::c_long) as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh2 = source;
        source = source.offset(1);
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 = *fresh2;
        i += 1
    }
    if (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                            isize)).yy_buffer_status ==
           2 as libc::c_int {
        /* don't do the read, it's not guaranteed to return an EOF,
		 * just force an EOF
		 */
        (*yyg).yy_n_chars = 0 as libc::c_int;
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_n_chars =
            (*yyg).yy_n_chars
    } else {
        let mut num_to_read: libc::c_int =
            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                 isize)).yy_buf_size -
                number_to_move - 1 as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            /* Not enough room in the buffer - grow it. */
            /* just a shorter name for the current buffer */
            let mut b: YY_BUFFER_STATE =
                *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                   isize);
            let mut yy_c_buf_p_offset: libc::c_int =
                (*yyg).yy_c_buf_p.wrapping_offset_from((*b).yy_ch_buf) as
                    libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int =
                    (*b).yy_buf_size * 2 as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b).yy_buf_size += (*b).yy_buf_size / 8 as libc::c_int
                } else { (*b).yy_buf_size *= 2 as libc::c_int }
                (*b).yy_ch_buf =
                    jq_yyrealloc((*b).yy_ch_buf as *mut libc::c_void,
                                 ((*b).yy_buf_size + 2 as libc::c_int) as
                                     yy_size_t, yyscanner) as
                        *mut libc::c_char
            } else {
                /* Can't grow it, we don't own it. */
                (*b).yy_ch_buf = 0 as *mut libc::c_char
            }
            if (*b).yy_ch_buf.is_null() {
                yy_fatal_error(b"fatal error - scanner input buffer overflow\x00"
                                   as *const u8 as *const libc::c_char,
                               yyscanner);
            }
            (*yyg).yy_c_buf_p =
                &mut *(*b).yy_ch_buf.offset(yy_c_buf_p_offset as isize) as
                    *mut libc::c_char;
            num_to_read =
                (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                     isize)).yy_buf_size -
                    number_to_move - 1 as libc::c_int
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int
        }
        /* Read in more data. */
        if (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                isize)).yy_is_interactive != 0
           {
            let mut c: libc::c_int = '*' as i32;
            let mut n: libc::c_int = 0;
            n = 0 as libc::c_int;
            while n < num_to_read &&
                      {
                          c = _IO_getc((*yyg).yyin_r);
                          (c) != -(1 as libc::c_int)
                      } && c != '\n' as i32 {
                *(&mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                             as
                                                             isize)).yy_ch_buf.offset(number_to_move
                                                                                          as
                                                                                          isize)
                      as *mut libc::c_char).offset(n as isize) =
                    c as libc::c_char;
                n += 1
            }
            if c == '\n' as i32 {
                let fresh4 = n;
                n = n + 1;
                *(&mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                             as
                                                             isize)).yy_ch_buf.offset(number_to_move
                                                                                          as
                                                                                          isize)
                      as *mut libc::c_char).offset(fresh4 as isize) =
                    c as libc::c_char
            }
            if c == -(1 as libc::c_int) && ferror((*yyg).yyin_r) != 0 {
                yy_fatal_error(b"input in flex scanner failed\x00" as
                                   *const u8 as *const libc::c_char,
                               yyscanner);
            }
            (*yyg).yy_n_chars = n
        } else {
            *__errno_location() = 0 as libc::c_int;
            loop  {
                (*yyg).yy_n_chars =
                    fread(&mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                     as
                                                                     isize)).yy_ch_buf.offset(number_to_move
                                                                                                  as
                                                                                                  isize)
                              as *mut libc::c_char as *mut libc::c_void,
                          1 as libc::c_int as libc::c_ulong,
                          num_to_read as yy_size_t, (*yyg).yyin_r) as
                        libc::c_int;
                if !((*yyg).yy_n_chars == 0 as libc::c_int &&
                         ferror((*yyg).yyin_r) != 0) {
                    break ;
                }
                if *__errno_location() != 4 as libc::c_int {
                    yy_fatal_error(b"input in flex scanner failed\x00" as
                                       *const u8 as *const libc::c_char,
                                   yyscanner);
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    clearerr((*yyg).yyin_r);
                }
            }
        }
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_n_chars =
            (*yyg).yy_n_chars
    }
    if (*yyg).yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            jq_yyrestart((*yyg).yyin_r, yyscanner);
        } else {
            ret_val = 2 as libc::c_int;
            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                 isize)).yy_buffer_status =
                2 as libc::c_int
        }
    } else { ret_val = 0 as libc::c_int }
    if (*yyg).yy_n_chars + number_to_move >
           (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                isize)).yy_buf_size {
        /* Extend the array by 50%, plus the number we really need. */
        let mut new_size_0: libc::c_int =
            (*yyg).yy_n_chars + number_to_move +
                ((*yyg).yy_n_chars >> 1 as libc::c_int);
        let ref mut fresh5 =
            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                 isize)).yy_ch_buf;
        *fresh5 =
            jq_yyrealloc((**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                              as
                                                              isize)).yy_ch_buf
                             as *mut libc::c_void, new_size_0 as yy_size_t,
                         yyscanner) as *mut libc::c_char;
        if (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                isize)).yy_ch_buf.is_null() {
            yy_fatal_error(b"out of dynamic memory in yy_get_next_buffer()\x00"
                               as *const u8 as *const libc::c_char,
                           yyscanner);
        }
        /* "- 2" to take care of EOB's */
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_buf_size =
            new_size_0 - 2 as libc::c_int
    }
    (*yyg).yy_n_chars += number_to_move;
    *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                          isize)).yy_ch_buf.offset((*yyg).yy_n_chars
                                                                       as
                                                                       isize)
        = 0 as libc::c_int as libc::c_char;
    *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                          isize)).yy_ch_buf.offset(((*yyg).yy_n_chars
                                                                        +
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       isize)
        = 0 as libc::c_int as libc::c_char;
    (*yyg).yytext_r =
        &mut *(**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                   isize)).yy_ch_buf.offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
            as *mut libc::c_char;
    return ret_val;
}
/* yy_get_previous_state - get the state just before the EOB char was reached */
unsafe extern "C" fn yy_get_previous_state(mut yyscanner: yyscan_t)
 -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    yy_current_state = (*yyg).yy_start;
    yy_cp = (*yyg).yytext_r.offset(0 as libc::c_int as isize);
    while yy_cp < (*yyg).yy_c_buf_p {
        let mut yy_c: YY_CHAR =
            if *yy_cp as libc::c_int != 0 {
                yy_ec[*yy_cp as YY_CHAR as usize] as libc::c_int
            } else { 1 as libc::c_int } as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            (*yyg).yy_last_accepting_state = yy_current_state;
            (*yyg).yy_last_accepting_cpos = yy_cp
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int +
                          yy_c as libc::c_int) as usize] as libc::c_int !=
                  yy_current_state {
            yy_current_state =
                yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 157 as libc::c_int {
                yy_c = yy_meta[yy_c as usize]
            }
        }
        yy_current_state =
            yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int +
                        yy_c as libc::c_int) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1)
    }
    return yy_current_state;
}
/* yy_try_NUL_trans - try to make a transition on the NUL character
 *
 * synopsis
 *	next_state = yy_try_NUL_trans( current_state );
 */
unsafe extern "C" fn yy_try_NUL_trans(mut yy_current_state: yy_state_type,
                                      mut yyscanner: yyscan_t)
 -> yy_state_type {
    let mut yy_is_jam: libc::c_int =
        0; /* This var may be unused depending upon options. */
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    let mut yy_cp: *mut libc::c_char = (*yyg).yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        (*yyg).yy_last_accepting_state = yy_current_state;
        (*yyg).yy_last_accepting_cpos = yy_cp
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int +
                      yy_c as libc::c_int) as usize] as libc::c_int !=
              yy_current_state {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 157 as libc::c_int {
            yy_c = yy_meta[yy_c as usize]
        }
    }
    yy_current_state =
        yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int +
                    yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 156 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
/* ifndef YY_NO_INPUT */
/* * Immediately switch to a different input stream.
 * @param input_file A readable stream.
 * @param yyscanner The scanner object.
 * @note This function does not reset the start condition to @c INITIAL .
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyrestart(mut input_file: *mut FILE,
                                      mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if if !(*yyg).yy_buffer_stack.is_null() {
           *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        jq_yyensure_buffer_stack(yyscanner);
        let ref mut fresh6 =
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize);
        *fresh6 =
            jq_yy_create_buffer((*yyg).yyin_r, 16384 as libc::c_int,
                                yyscanner)
    }
    jq_yy_init_buffer(if !(*yyg).yy_buffer_stack.is_null() {
                          *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                             as isize)
                      } else { 0 as YY_BUFFER_STATE }, input_file, yyscanner);
    jq_yy_load_buffer_state(yyscanner);
}
/* * Switch to a different input buffer.
 * @param new_buffer The new input buffer.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_switch_to_buffer(mut new_buffer:
                                                    YY_BUFFER_STATE,
                                                mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    /* TODO. We should be able to replace this entire function body
	 * with
	 *		yypop_buffer_state();
	 *		yypush_buffer_state(new_buffer);
     */
    jq_yyensure_buffer_stack(yyscanner);
    if (if !(*yyg).yy_buffer_stack.is_null() {
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize)
        } else { 0 as YY_BUFFER_STATE }) == new_buffer {
        return
    }
    if !if !(*yyg).yy_buffer_stack.is_null() {
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        /* Flush out information for old buffer. */
        *(*yyg).yy_c_buf_p = (*yyg).yy_hold_char;
        let ref mut fresh7 =
            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                 isize)).yy_buf_pos;
        *fresh7 = (*yyg).yy_c_buf_p;
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_n_chars =
            (*yyg).yy_n_chars
    }
    let ref mut fresh8 =
        *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize);
    *fresh8 = new_buffer;
    jq_yy_load_buffer_state(yyscanner);
    /* We don't actually know whether we did this switch during
	 * EOF (yywrap()) processing, but the only time this flag
	 * is looked at is after yywrap() is called, so it's safe
	 * to go ahead and always set it.
	 */
    (*yyg).yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn jq_yy_load_buffer_state(mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yy_n_chars =
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_n_chars;
    (*yyg).yy_c_buf_p =
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_buf_pos;
    (*yyg).yytext_r = (*yyg).yy_c_buf_p;
    (*yyg).yyin_r =
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_input_file;
    (*yyg).yy_hold_char = *(*yyg).yy_c_buf_p;
}
/* * Allocate and initialize an input buffer state.
 * @param file A readable stream.
 * @param size The character buffer size in bytes. When in doubt, use @c YY_BUF_SIZE.
 * @param yyscanner The scanner object.
 * @return the allocated buffer state.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_create_buffer(mut file: *mut FILE,
                                             mut size: libc::c_int,
                                             mut yyscanner: yyscan_t)
 -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b =
        jq_yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong,
                   yyscanner) as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_create_buffer()\x00" as
                           *const u8 as *const libc::c_char, yyscanner);
    }
    (*b).yy_buf_size = size;
    /* yy_ch_buf has to be 2 characters longer than the size given because
	 * we need to put in 2 end-of-buffer characters.
	 */
    (*b).yy_ch_buf =
        jq_yyalloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t,
                   yyscanner) as *mut libc::c_char;
    if (*b).yy_ch_buf.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_create_buffer()\x00" as
                           *const u8 as *const libc::c_char, yyscanner);
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    jq_yy_init_buffer(b, file, yyscanner);
    return b;
}
/* * Destroy the buffer.
 * @param b a buffer created with yy_create_buffer()
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_delete_buffer(mut b: YY_BUFFER_STATE,
                                             mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if b.is_null() { return }
    if b ==
           (if !(*yyg).yy_buffer_stack.is_null() {
                *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                   isize)
            } else { 0 as YY_BUFFER_STATE }) {
        /* Not sure if we should pop here. */
        let ref mut fresh9 =
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize);
        *fresh9 = 0 as YY_BUFFER_STATE
    }
    if (*b).yy_is_our_buffer != 0 {
        jq_yyfree((*b).yy_ch_buf as *mut libc::c_void, yyscanner);
    }
    jq_yyfree(b as *mut libc::c_void, yyscanner);
}
/* Initializes or reinitializes a buffer.
 * This function is sometimes called more than once on the same buffer,
 * such as during a yyrestart() or at EOF.
 */
unsafe extern "C" fn jq_yy_init_buffer(mut b: YY_BUFFER_STATE,
                                       mut file: *mut FILE,
                                       mut yyscanner: yyscan_t) {
    let mut oerrno: libc::c_int = *__errno_location();
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    jq_yy_flush_buffer(b, yyscanner);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    /* If b is the current buffer, then yy_init_buffer was _probably_
     * called from yyrestart() or through yy_get_next_buffer.
     * In that case, we don't want to reset the lineno or column.
     */
    if b !=
           (if !(*yyg).yy_buffer_stack.is_null() {
                *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                   isize)
            } else { 0 as YY_BUFFER_STATE }) {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int
    }
    (*b).yy_is_interactive =
        if !file.is_null() {
            (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
        } else { 0 as libc::c_int };
    *__errno_location() = oerrno;
}
/* * Discard all buffered characters. On the next scan, YY_INPUT will be called.
 * @param b the buffer state to be flushed, usually @c YY_CURRENT_BUFFER.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_flush_buffer(mut b: YY_BUFFER_STATE,
                                            mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if b.is_null() { return }
    (*b).yy_n_chars = 0 as libc::c_int;
    /* We always need two end-of-buffer characters.  The first causes
	 * a transition to the end-of-buffer state.  The second causes
	 * a jam in that state.
	 */
    *(*b).yy_ch_buf.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    *(*b).yy_ch_buf.offset(1 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    (*b).yy_buf_pos =
        &mut *(*b).yy_ch_buf.offset(0 as libc::c_int as isize) as
            *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b ==
           (if !(*yyg).yy_buffer_stack.is_null() {
                *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                   isize)
            } else { 0 as YY_BUFFER_STATE }) {
        jq_yy_load_buffer_state(yyscanner);
    };
}
/* * Pushes the new state onto the stack. The new state becomes
 *  the current state. This function will allocate the stack
 *  if necessary.
 *  @param new_buffer The new state.
 *  @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yypush_buffer_state(mut new_buffer:
                                                    YY_BUFFER_STATE,
                                                mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if new_buffer.is_null() { return }
    jq_yyensure_buffer_stack(yyscanner);
    /* This block is copied from yy_switch_to_buffer. */
    if !if !(*yyg).yy_buffer_stack.is_null() {
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        /* Flush out information for old buffer. */
        *(*yyg).yy_c_buf_p = (*yyg).yy_hold_char;
        let ref mut fresh10 =
            (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                 isize)).yy_buf_pos;
        *fresh10 = (*yyg).yy_c_buf_p;
        (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                             isize)).yy_n_chars =
            (*yyg).yy_n_chars
    }
    /* Only push if top exists. Otherwise, replace top. */
    if !if !(*yyg).yy_buffer_stack.is_null() {
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        (*yyg).yy_buffer_stack_top =
            (*yyg).yy_buffer_stack_top.wrapping_add(1)
    }
    let ref mut fresh11 =
        *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize);
    *fresh11 = new_buffer;
    /* copied from yy_switch_to_buffer. */
    jq_yy_load_buffer_state(yyscanner);
    (*yyg).yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
/* * Removes and deletes the top of the stack, if present.
 *  The next element becomes the new top.
 *  @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yypop_buffer_state(mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if if !(*yyg).yy_buffer_stack.is_null() {
           *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        return
    }
    jq_yy_delete_buffer(if !(*yyg).yy_buffer_stack.is_null() {
                            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                               as isize)
                        } else { 0 as YY_BUFFER_STATE }, yyscanner);
    let ref mut fresh12 =
        *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize);
    *fresh12 = 0 as YY_BUFFER_STATE;
    if (*yyg).yy_buffer_stack_top > 0 as libc::c_int as libc::c_ulong {
        (*yyg).yy_buffer_stack_top =
            (*yyg).yy_buffer_stack_top.wrapping_sub(1)
    }
    if !if !(*yyg).yy_buffer_stack.is_null() {
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        jq_yy_load_buffer_state(yyscanner);
        (*yyg).yy_did_buffer_switch_on_eof = 1 as libc::c_int
    };
}
/* Allocates the stack if it does not exist.
 *  Guarantees space for at least one push.
 */
unsafe extern "C" fn jq_yyensure_buffer_stack(mut yyscanner: yyscan_t) {
    let mut num_to_alloc: yy_size_t = 0;
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if (*yyg).yy_buffer_stack.is_null() {
        /* First allocation is just for 2 elements, since we don't know if this
		 * scanner will even need a stack. We use 2 instead of 1 to avoid an
		 * immediate realloc on the next call.
         */
        num_to_alloc =
            1 as libc::c_int as
                yy_size_t; /* After all that talk, this was set to 1 anyways... */
        (*yyg).yy_buffer_stack =
            jq_yyalloc(num_to_alloc.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                                     as libc::c_ulong),
                       yyscanner) as *mut *mut yy_buffer_state;
        if (*yyg).yy_buffer_stack.is_null() {
            yy_fatal_error(b"out of dynamic memory in yyensure_buffer_stack()\x00"
                               as *const u8 as *const libc::c_char,
                           yyscanner);
        }
        memset((*yyg).yy_buffer_stack as *mut libc::c_void, 0 as libc::c_int,
               num_to_alloc.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                             as libc::c_ulong));
        (*yyg).yy_buffer_stack_max = num_to_alloc;
        (*yyg).yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return
    }
    if (*yyg).yy_buffer_stack_top >=
           (*yyg).yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) {
        /* Increase the buffer to prepare for a possible push. */
        let mut grow_size: yy_size_t = 8 as libc::c_int as yy_size_t;
        num_to_alloc = (*yyg).yy_buffer_stack_max.wrapping_add(grow_size);
        (*yyg).yy_buffer_stack =
            jq_yyrealloc((*yyg).yy_buffer_stack as *mut libc::c_void,
                         num_to_alloc.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                                       as libc::c_ulong),
                         yyscanner) as *mut *mut yy_buffer_state;
        if (*yyg).yy_buffer_stack.is_null() {
            yy_fatal_error(b"out of dynamic memory in yyensure_buffer_stack()\x00"
                               as *const u8 as *const libc::c_char,
                           yyscanner);
        }
        /* arbitrary grow size */
        memset((*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_max as
                                                 isize) as *mut libc::c_void,
               0 as libc::c_int,
               grow_size.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                          as libc::c_ulong));
        (*yyg).yy_buffer_stack_max = num_to_alloc
    };
}
/* zero only the new slots.*/
/* * Setup the input buffer state to scan directly from a user-specified character buffer.
 * @param base the character buffer
 * @param size the size in bytes of the character buffer
 * @param yyscanner The scanner object.
 * @return the newly allocated buffer state object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_scan_buffer(mut base: *mut libc::c_char,
                                           mut size: yy_size_t,
                                           mut yyscanner: yyscan_t)
 -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_ulong ||
           *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            as isize) as libc::c_int != 0 as libc::c_int ||
           *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize) as libc::c_int != 0 as libc::c_int {
        /* They forgot to leave room for the EOB's. */
        return 0 as YY_BUFFER_STATE
    } /* "- 2" to take care of EOB's */
    b =
        jq_yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong,
                   yyscanner) as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_scan_buffer()\x00" as
                           *const u8 as *const libc::c_char, yyscanner);
    }
    (*b).yy_buf_size =
        size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    jq_yy_switch_to_buffer(b, yyscanner);
    return b;
}
/* * Setup the input buffer state to scan a string. The next call to yylex() will
 * scan from a @e copy of @a str.
 * @param yystr a NUL-terminated string to scan
 * @param yyscanner The scanner object.
 * @return the newly allocated buffer state object.
 * @note If you want to scan bytes that may contain NUL values, then use
 *       yy_scan_bytes() instead.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_scan_string(mut yystr: *const libc::c_char,
                                           mut yyscanner: yyscan_t)
 -> YY_BUFFER_STATE {
    return jq_yy_scan_bytes(yystr, strlen(yystr) as libc::c_int, yyscanner);
}
/* * Setup the input buffer state to scan the given bytes. The next call to yylex() will
 * scan from a @e copy of @a bytes.
 * @param yybytes the byte buffer to scan
 * @param _yybytes_len the number of bytes in the buffer pointed to by @a bytes.
 * @param yyscanner The scanner object.
 * @return the newly allocated buffer state object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yy_scan_bytes(mut yybytes: *const libc::c_char,
                                          mut _yybytes_len: libc::c_int,
                                          mut yyscanner: yyscan_t)
 -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    /* Get memory for full buffer, including space for trailing EOB's. */
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = jq_yyalloc(n, yyscanner) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_scan_bytes()\x00" as
                           *const u8 as *const libc::c_char, yyscanner);
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1
    }
    let ref mut fresh13 =
        *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh13 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh13;
    b = jq_yy_scan_buffer(buf, n, yyscanner);
    if b.is_null() {
        yy_fatal_error(b"bad buffer in yy_scan_bytes()\x00" as *const u8 as
                           *const libc::c_char, yyscanner);
    }
    /* It's okay to grow etc. this buffer, and we should throw it
	 * away when we're done.
	 */
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
/* Macros after this point can all be overridden by user definitions in
 * section 1.
 */
unsafe extern "C" fn yy_push_state(mut _new_state: libc::c_int,
                                   mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if (*yyg).yy_start_stack_ptr >= (*yyg).yy_start_stack_depth {
        let mut new_size: yy_size_t = 0;
        (*yyg).yy_start_stack_depth += 25 as libc::c_int;
        new_size =
            ((*yyg).yy_start_stack_depth as
                 yy_size_t).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                             as libc::c_ulong);
        if (*yyg).yy_start_stack.is_null() {
            (*yyg).yy_start_stack =
                jq_yyalloc(new_size, yyscanner) as *mut libc::c_int
        } else {
            (*yyg).yy_start_stack =
                jq_yyrealloc((*yyg).yy_start_stack as *mut libc::c_void,
                             new_size, yyscanner) as *mut libc::c_int
        }
        if (*yyg).yy_start_stack.is_null() {
            yy_fatal_error(b"out of memory expanding start-condition stack\x00"
                               as *const u8 as *const libc::c_char,
                           yyscanner);
        }
    }
    let fresh14 = (*yyg).yy_start_stack_ptr;
    (*yyg).yy_start_stack_ptr = (*yyg).yy_start_stack_ptr + 1;
    *(*yyg).yy_start_stack.offset(fresh14 as isize) =
        ((*yyg).yy_start - 1 as libc::c_int) / 2 as libc::c_int;
    (*yyg).yy_start = 1 as libc::c_int + 2 as libc::c_int * _new_state;
}
unsafe extern "C" fn yy_pop_state(mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yy_start_stack_ptr -= 1;
    if (*yyg).yy_start_stack_ptr < 0 as libc::c_int {
        yy_fatal_error(b"start-condition stack underflow\x00" as *const u8 as
                           *const libc::c_char, yyscanner);
    }
    (*yyg).yy_start =
        1 as libc::c_int +
            2 as libc::c_int *
                *(*yyg).yy_start_stack.offset((*yyg).yy_start_stack_ptr as
                                                  isize);
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char,
                                    mut yyscanner: yyscan_t) -> ! {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
/* Redefine yyless() so it works in section 3 code. */
/* Undo effects of setting up yytext. */
/* Accessor  methods (get/set functions) to struct members. */
/* * Get the user-defined data for this scanner.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_extra(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yyextra_r;
}
/* * Get the current line number.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_lineno(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if if !(*yyg).yy_buffer_stack.is_null() {
           *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        return 0 as libc::c_int
    }
    return (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                isize)).yy_bs_lineno;
}
/* * Get the current column number.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_column(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    if if !(*yyg).yy_buffer_stack.is_null() {
           *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        return 0 as libc::c_int
    }
    return (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                isize)).yy_bs_column;
}
/* * Get the input stream.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_in(mut yyscanner: yyscan_t) -> *mut FILE {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yyin_r;
}
/* * Get the output stream.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_out(mut yyscanner: yyscan_t) -> *mut FILE {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yyout_r;
}
/* * Get the length of the current token.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_leng(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yyleng_r;
}
/* * Get the current token.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_text(mut yyscanner: yyscan_t)
 -> *mut libc::c_char {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yytext_r;
}
/* * Set the user-defined data. This data is never touched by the scanner.
 * @param user_defined The data to be associated with this scanner.
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_extra(mut user_defined: libc::c_int,
                                        mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yyextra_r = user_defined;
}
/* * Set the current line number.
 * @param _line_number line number
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_lineno(mut _line_number: libc::c_int,
                                         mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    /* lineno is only valid if an input buffer exists. */
    if if !(*yyg).yy_buffer_stack.is_null() {
           *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        yy_fatal_error(b"yyset_lineno called with no buffer\x00" as *const u8
                           as *const libc::c_char, yyscanner);
    }
    (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                         isize)).yy_bs_lineno = _line_number;
}
/* * Set the current column.
 * @param _column_no column number
 * @param yyscanner The scanner object.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_column(mut _column_no: libc::c_int,
                                         mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    /* column is only valid if an input buffer exists. */
    if if !(*yyg).yy_buffer_stack.is_null() {
           *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        yy_fatal_error(b"yyset_column called with no buffer\x00" as *const u8
                           as *const libc::c_char, yyscanner);
    }
    (**(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                         isize)).yy_bs_column = _column_no;
}
/* * Set the input stream. This does not discard the current
 * input buffer.
 * @param _in_str A readable stream.
 * @param yyscanner The scanner object.
 * @see yy_switch_to_buffer
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_in(mut _in_str: *mut FILE,
                                     mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yyin_r = _in_str;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_out(mut _out_str: *mut FILE,
                                      mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yyout_r = _out_str;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_debug(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yy_flex_debug_r;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_debug(mut _bdebug: libc::c_int,
                                        mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yy_flex_debug_r = _bdebug;
}
/* Accessor methods for yylval and yylloc */
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_lval(mut yyscanner: yyscan_t)
 -> *mut YYSTYPE {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yylval_r;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_lval(mut yylval_param: *mut YYSTYPE,
                                       mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yylval_r = yylval_param;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyget_lloc(mut yyscanner: yyscan_t)
 -> *mut location {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    return (*yyg).yylloc_r;
}
#[no_mangle]
pub unsafe extern "C" fn jq_yyset_lloc(mut yylloc_param: *mut location,
                                       mut yyscanner: yyscan_t) {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    (*yyg).yylloc_r = yylloc_param;
}
/* User-visible API */
/* yylex_init is special because it creates the scanner itself, so it is
 * the ONLY reentrant function that doesn't take the scanner as the last argument.
 * That's why we explicitly handle the declaration, instead of using our macros.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yylex_init(mut ptr_yy_globals: *mut yyscan_t)
 -> libc::c_int {
    if ptr_yy_globals.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int
    }
    *ptr_yy_globals =
        jq_yyalloc(::std::mem::size_of::<yyguts_t>() as libc::c_ulong,
                   0 as *mut libc::c_void);
    if (*ptr_yy_globals).is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 1 as libc::c_int
    }
    /* By setting to 0xAA, we expose bugs in yy_init_globals. Leave at 0x00 for releases. */
    memset(*ptr_yy_globals, 0 as libc::c_int,
           ::std::mem::size_of::<yyguts_t>() as libc::c_ulong);
    return yy_init_globals(*ptr_yy_globals);
}
/* yylex_init_extra has the same functionality as yylex_init, but follows the
 * convention of taking the scanner as the last argument. Note however, that
 * this is a *pointer* to a scanner, as it will be allocated by this call (and
 * is the reason, too, why this function also must handle its own declaration).
 * The user defined value in the first argument will be available to yyalloc in
 * the yyextra field.
 */
#[no_mangle]
pub unsafe extern "C" fn jq_yylex_init_extra(mut yy_user_defined: libc::c_int,
                                             mut ptr_yy_globals:
                                                 *mut yyscan_t)
 -> libc::c_int {
    let mut dummy_yyguts: yyguts_t =
        yyguts_t{yyextra_r: 0,
                 yyin_r: 0 as *mut FILE,
                 yyout_r: 0 as *mut FILE,
                 yy_buffer_stack_top: 0,
                 yy_buffer_stack_max: 0,
                 yy_buffer_stack: 0 as *mut YY_BUFFER_STATE,
                 yy_hold_char: 0,
                 yy_n_chars: 0,
                 yyleng_r: 0,
                 yy_c_buf_p: 0 as *mut libc::c_char,
                 yy_init: 0,
                 yy_start: 0,
                 yy_did_buffer_switch_on_eof: 0,
                 yy_start_stack_ptr: 0,
                 yy_start_stack_depth: 0,
                 yy_start_stack: 0 as *mut libc::c_int,
                 yy_last_accepting_state: 0,
                 yy_last_accepting_cpos: 0 as *mut libc::c_char,
                 yylineno_r: 0,
                 yy_flex_debug_r: 0,
                 yytext_r: 0 as *mut libc::c_char,
                 yy_more_flag: 0,
                 yy_more_len: 0,
                 yylval_r: 0 as *mut YYSTYPE,
                 yylloc_r: 0 as *mut location,};
    jq_yyset_extra(yy_user_defined,
                   &mut dummy_yyguts as *mut yyguts_t as yyscan_t);
    if ptr_yy_globals.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int
    }
    *ptr_yy_globals =
        jq_yyalloc(::std::mem::size_of::<yyguts_t>() as libc::c_ulong,
                   &mut dummy_yyguts as *mut yyguts_t as yyscan_t);
    if (*ptr_yy_globals).is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 1 as libc::c_int
    }
    /* By setting to 0xAA, we expose bugs in
    yy_init_globals. Leave at 0x00 for releases. */
    memset(*ptr_yy_globals, 0 as libc::c_int,
           ::std::mem::size_of::<yyguts_t>() as libc::c_ulong);
    jq_yyset_extra(yy_user_defined, *ptr_yy_globals);
    return yy_init_globals(*ptr_yy_globals);
}
/* end struct yyguts_t */
unsafe extern "C" fn yy_init_globals(mut yyscanner: yyscan_t) -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    /* Initialization is the same as for the non-reentrant scanner.
     * This function is called from yylex_destroy(), so don't allocate here.
     */
    (*yyg).yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    (*yyg).yy_buffer_stack_top = 0 as libc::c_int as size_t;
    (*yyg).yy_buffer_stack_max = 0 as libc::c_int as size_t;
    (*yyg).yy_c_buf_p = 0 as *mut libc::c_char;
    (*yyg).yy_init = 0 as libc::c_int;
    (*yyg).yy_start = 0 as libc::c_int;
    (*yyg).yy_start_stack_ptr = 0 as libc::c_int;
    (*yyg).yy_start_stack_depth = 0 as libc::c_int;
    (*yyg).yy_start_stack = 0 as *mut libc::c_int;
    /* Defined in main.c */
    (*yyg).yyin_r = 0 as *mut FILE;
    (*yyg).yyout_r = 0 as *mut FILE;
    /* For future reference: Set errno on error, since we are called by
     * yylex_init()
     */
    return 0 as libc::c_int;
}
/* Accessor methods to globals.
   These are made visible to non-reentrant scanners for convenience. */
/* yylex_destroy is for both reentrant and non-reentrant scanners. */
#[no_mangle]
pub unsafe extern "C" fn jq_yylex_destroy(mut yyscanner: yyscan_t)
 -> libc::c_int {
    let mut yyg: *mut yyguts_t = yyscanner as *mut yyguts_t;
    /* Pop the buffer stack, destroying each element. */
    while !if !(*yyg).yy_buffer_stack.is_null() {
               *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                                  isize)
           } else { 0 as YY_BUFFER_STATE }.is_null() {
        jq_yy_delete_buffer(if !(*yyg).yy_buffer_stack.is_null() {
                                *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top
                                                                   as isize)
                            } else { 0 as YY_BUFFER_STATE }, yyscanner);
        let ref mut fresh15 =
            *(*yyg).yy_buffer_stack.offset((*yyg).yy_buffer_stack_top as
                                               isize);
        *fresh15 = 0 as YY_BUFFER_STATE;
        jq_yypop_buffer_state(yyscanner);
    }
    /* Destroy the stack itself. */
    jq_yyfree((*yyg).yy_buffer_stack as *mut libc::c_void, yyscanner);
    (*yyg).yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    /* Destroy the start condition stack. */
    jq_yyfree((*yyg).yy_start_stack as *mut libc::c_void, yyscanner);
    (*yyg).yy_start_stack = 0 as *mut libc::c_int;
    /* Reset the globals. This is important in a non-reentrant scanner so the next time
     * yylex() is called, initialization will occur. */
    yy_init_globals(yyscanner);
    /* Destroy the main struct (reentrant only). */
    jq_yyfree(yyscanner, yyscanner);
    yyscanner = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * Internal utility routines.
 */
