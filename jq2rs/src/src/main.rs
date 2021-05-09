#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jv_parser;
    pub type jq_state;
    pub type jq_util_input_state;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __strdup(__string: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
    fn jv_invalid_get_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_has_msg(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_array() -> jv;
    /*
 * We use char * instead of jf for filenames here because filenames
 * should be in the process' locale's codeset, which may not be UTF-8,
 * whereas jv string values must be in UTF-8.  This way the caller
 * doesn't have to perform any codeset conversions.
 */
    #[no_mangle]
    fn jq_util_input_errors(_: *mut jq_util_input_state) -> libc::c_int;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_has(object: jv, key: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_dumpf(_: jv, f: *mut FILE, flags: libc::c_int);
    #[no_mangle]
    fn jv_dump(_: jv, flags: libc::c_int);
    #[no_mangle]
    fn jv_dump_string(_: jv, flags: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_parse(string: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_load_file(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_parser_new(_: libc::c_int) -> *mut jv_parser;
    #[no_mangle]
    fn jq_init() -> *mut jq_state;
    #[no_mangle]
    fn jq_compile_args(_: *mut jq_state, _: *const libc::c_char, _: jv)
     -> libc::c_int;
    #[no_mangle]
    fn jq_dump_disassembly(_: *mut jq_state, _: libc::c_int);
    #[no_mangle]
    fn jq_start(_: *mut jq_state, value: jv, _: libc::c_int);
    #[no_mangle]
    fn jq_next(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_teardown(_: *mut *mut jq_state);
    #[no_mangle]
    fn jq_halted(_: *mut jq_state) -> libc::c_int;
    #[no_mangle]
    fn jq_get_exit_code(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_get_error_message(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_set_input_cb(_: *mut jq_state, _: jq_input_cb,
                       _: *mut libc::c_void);
    #[no_mangle]
    fn jq_set_debug_cb(_: *mut jq_state, _: jq_msg_cb, _: *mut libc::c_void);
    #[no_mangle]
    fn jq_set_attr(_: *mut jq_state, _: jv, _: jv);
    #[no_mangle]
    fn jq_util_input_init(_: jq_util_msg_cb, _: *mut libc::c_void)
     -> *mut jq_util_input_state;
    #[no_mangle]
    fn jq_util_input_set_parser(_: *mut jq_util_input_state,
                                _: *mut jv_parser, _: libc::c_int);
    #[no_mangle]
    fn jq_util_input_free(_: *mut *mut jq_util_input_state);
    #[no_mangle]
    fn jq_util_input_add_input(_: *mut jq_util_input_state,
                               _: *const libc::c_char);
    #[no_mangle]
    fn jq_util_input_next_input(_: *mut jq_util_input_state) -> jv;
    #[no_mangle]
    fn jq_util_input_next_input_cb(_: *mut jq_state, _: *mut libc::c_void)
     -> jv;
    #[no_mangle]
    fn jq_util_input_get_position(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_set_colors(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn jq_realpath(_: jv) -> jv;
    #[no_mangle]
    fn jq_testsuite(lib_dirs: jv, verbose: libc::c_int, argc: libc::c_int,
                    argv: *mut *mut libc::c_char) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
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
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const JV_PARSE_STREAM_ERRORS: C2RustUnnamed_1 = 4;
pub const JV_PARSE_STREAMING: C2RustUnnamed_1 = 2;
pub const JV_PARSE_SEQ: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const JQ_DEBUG_TRACE_ALL: C2RustUnnamed_2 = 3;
pub const JQ_DEBUG_TRACE_DETAIL: C2RustUnnamed_2 = 2;
pub const JQ_DEBUG_TRACE: C2RustUnnamed_2 = 1;
pub type jq_msg_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: jv) -> ()>;
pub type jq_input_cb
    =
    Option<unsafe extern "C" fn(_: *mut jq_state, _: *mut libc::c_void)
               -> jv>;
pub type jq_util_msg_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char)
               -> ()>;
pub type C2RustUnnamed_3 = libc::c_uint;
/* debugging only */
pub const DUMP_DISASM: C2RustUnnamed_3 = 65536;
pub const RUN_TESTS: C2RustUnnamed_3 = 32768;
pub const SEQ: C2RustUnnamed_3 = 16384;
pub const EXIT_STATUS_EXACT: C2RustUnnamed_3 = 8192;
pub const EXIT_STATUS: C2RustUnnamed_3 = 4096;
pub const UNBUFFERED_OUTPUT: C2RustUnnamed_3 = 2048;
pub const RAW_NO_LF: C2RustUnnamed_3 = 1024;
pub const FROM_FILE: C2RustUnnamed_3 = 512;
pub const SORTED_OUTPUT: C2RustUnnamed_3 = 256;
pub const NO_COLOR_OUTPUT: C2RustUnnamed_3 = 128;
pub const COLOR_OUTPUT: C2RustUnnamed_3 = 64;
pub const ASCII_OUTPUT: C2RustUnnamed_3 = 32;
pub const RAW_NUL: C2RustUnnamed_3 = 16;
pub const RAW_OUTPUT: C2RustUnnamed_3 = 8;
pub const PROVIDE_NULL: C2RustUnnamed_3 = 4;
pub const RAW_INPUT: C2RustUnnamed_3 = 2;
pub const SLURP: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_int;
/* exit 0 if --exit-status is not set*/
pub const JQ_ERROR_UNKNOWN: C2RustUnnamed_4 = 5;
pub const JQ_OK_NO_OUTPUT: C2RustUnnamed_4 = -4;
pub const JQ_ERROR_COMPILE: C2RustUnnamed_4 = 3;
/* exit 0 if --exit-status is not set*/
pub const JQ_ERROR_SYSTEM: C2RustUnnamed_4 = 2;
pub const JQ_OK_NULL_KIND: C2RustUnnamed_4 = -1;
pub const JQ_OK: C2RustUnnamed_4 = 0;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/*
 * The Windows CRT and console are something else.  In order for the
 * console to get UTF-8 written to it correctly we have to bypass stdio
 * completely.  No amount of fflush()ing helps.  If the first byte of a
 * buffer being written with fwrite() is non-ASCII UTF-8 then the
 * console misinterprets the byte sequence.  But one must not
 * WriteFile() if stdout is a file!1!!
 *
 * We carry knowledge of whether the FILE * is a tty everywhere we
 * output to it just so we can write with WriteFile() if stdout is a
 * console on WIN32.
 */
unsafe extern "C" fn priv_fwrite(mut s: *const libc::c_char, mut len: size_t,
                                 mut fout: *mut FILE,
                                 mut is_tty: libc::c_int) {
    fwrite(s as *const libc::c_void, 1 as libc::c_int as libc::c_ulong, len,
           fout);
}
static mut progname: *const libc::c_char = 0 as *const libc::c_char;
/*
 * For a longer help message we could use a better option parsing
 * strategy, one that lets stack options.
 */
unsafe extern "C" fn usage(mut code: libc::c_int,
                           mut keep_it_short: libc::c_int) {
    let mut f: *mut FILE = stderr; // No valid results && -e -> exit(4)
    if code == 0 as libc::c_int { f = stdout }
    let mut ret: libc::c_int =
        fprintf(f,
                b"jq - commandline JSON processor [version %s]\n\nUsage:\t%s [options] <jq filter> [file...]\n\t%s [options] --args <jq filter> [strings...]\n\t%s [options] --jsonargs <jq filter> [JSON_TEXTS...]\n\njq is a tool for processing JSON inputs, applying the given filter to\nits JSON text inputs and producing the filter\'s results as JSON on\nstandard output.\n\nThe simplest filter is ., which copies jq\'s input to its output\nunmodified (except for formatting, but note that IEEE754 is used\nfor number representation internally, with all that that implies).\n\nFor more advanced filters see the jq(1) manpage (\"man jq\")\nand/or https://stedolan.github.io/jq\n\nExample:\n\n\t$ echo \'{\"foo\": 0}\' | jq .\n\t{\n\t\t\"foo\": 0\n\t}\n\n\x00"
                    as *const u8 as *const libc::c_char,
                b"1.6-137-gd18b2d0-dirty\x00" as *const u8 as
                    *const libc::c_char, progname, progname, progname);
    if keep_it_short != 0 {
        fprintf(f,
                b"For a listing of options, use %s --help.\n\x00" as *const u8
                    as *const libc::c_char, progname);
    } else {
        fprintf(f,
                b"Some of the options include:\n  -c               compact instead of pretty-printed output;\n  -n               use `null` as the single input value;\n  -e               set the exit status code based on the output;\n  -s               read (slurp) all inputs into an array; apply filter to it;\n  -r               output raw strings, not JSON texts;\n  -R               read raw strings, not JSON texts;\n  -C               colorize JSON;\n  -M               monochrome (don\'t colorize JSON);\n  -S               sort keys of objects on output;\n  --tab            use tabs for indentation;\n  --arg a v        set variable $a to value <v>;\n  --argjson a v    set variable $a to JSON value <v>;\n  --slurpfile a f  set variable $a to an array of JSON texts read from <f>;\n  --rawfile a f    set variable $a to a string consisting of the contents of <f>;\n  --args           remaining arguments are string arguments, not files;\n  --jsonargs       remaining arguments are JSON arguments, not files;\n  --               terminates argument processing;\n\nNamed arguments are also available as $ARGS.named[], while\npositional arguments are available as $ARGS.positional[].\n\nSee the manpage for more options.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    exit(if ret < 0 as libc::c_int && code == 0 as libc::c_int {
             2 as libc::c_int
         } else { code });
}
unsafe extern "C" fn die() {
    fprintf(stderr,
            b"Use %s --help for help with command-line options,\n\x00" as
                *const u8 as *const libc::c_char, progname);
    fprintf(stderr,
            b"or see the jq manpage, or online docs  at https://stedolan.github.io/jq\n\x00"
                as *const u8 as *const libc::c_char);
    exit(2 as libc::c_int);
}
unsafe extern "C" fn isoptish(mut text: *const libc::c_char) -> libc::c_int {
    return (*text.offset(0 as libc::c_int as isize) as libc::c_int ==
                '-' as i32 &&
                (*text.offset(1 as libc::c_int as isize) as libc::c_int ==
                     '-' as i32 ||
                     *(*__ctype_b_loc()).offset(*text.offset(1 as libc::c_int
                                                                 as isize) as
                                                    libc::c_int as isize) as
                         libc::c_int &
                         _ISalpha as libc::c_int as libc::c_ushort as
                             libc::c_int != 0)) as libc::c_int;
}
static mut options: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn process(mut jq: *mut jq_state, mut value: jv,
                             mut flags: libc::c_int,
                             mut dumpopts: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = JQ_OK_NO_OUTPUT as libc::c_int;
    jq_start(jq, value, flags);
    let mut result: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed_0{ptr: 0 as *mut jv_refcnt,},};
    loop  {
        result = jq_next(jq);
        if !(jv_is_valid(result) != 0) { break ; }
        if options & RAW_OUTPUT as libc::c_int != 0 &&
               jv_get_kind(result) as libc::c_uint ==
                   JV_KIND_STRING as libc::c_int as libc::c_uint {
            if options & ASCII_OUTPUT as libc::c_int != 0 {
                jv_dumpf(jv_copy(result), stdout,
                         JV_PRINT_ASCII as libc::c_int);
            } else {
                fwrite(jv_string_value(result) as *const libc::c_void,
                       1 as libc::c_int as libc::c_ulong,
                       jv_string_length_bytes(jv_copy(result)) as
                           libc::c_ulong, stdout);
            }
            ret = JQ_OK as libc::c_int;
            jv_free(result);
        } else {
            if jv_get_kind(result) as libc::c_uint ==
                   JV_KIND_FALSE as libc::c_int as libc::c_uint ||
                   jv_get_kind(result) as libc::c_uint ==
                       JV_KIND_NULL as libc::c_int as libc::c_uint {
                ret = JQ_OK_NULL_KIND as libc::c_int
            } else { ret = JQ_OK as libc::c_int }
            if options & SEQ as libc::c_int != 0 {
                priv_fwrite(b"\x1e\x00" as *const u8 as *const libc::c_char,
                            1 as libc::c_int as size_t, stdout,
                            dumpopts & JV_PRINT_ISATTY as libc::c_int);
            }
            jv_dump(result, dumpopts);
        }
        if options & RAW_NO_LF as libc::c_int == 0 {
            priv_fwrite(b"\n\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as size_t, stdout,
                        dumpopts & JV_PRINT_ISATTY as libc::c_int);
        }
        if options & RAW_NUL as libc::c_int != 0 {
            priv_fwrite(b"\x00\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as size_t, stdout,
                        dumpopts & JV_PRINT_ISATTY as libc::c_int);
        }
        if options & UNBUFFERED_OUTPUT as libc::c_int != 0 { fflush(stdout); }
    }
    if jq_halted(jq) != 0 {
        // jq program invoked `halt` or `halt_error`
        options |=
            EXIT_STATUS_EXACT as
                libc::c_int; // else no message on stderr; use --debug-trace to see a message
        let mut exit_code: jv = jq_get_exit_code(jq);
        if jv_is_valid(exit_code) == 0 {
            ret = JQ_OK as libc::c_int
        } else if jv_get_kind(exit_code) as libc::c_uint ==
                      JV_KIND_NUMBER as libc::c_int as libc::c_uint {
            ret = jv_number_value(exit_code) as libc::c_int
        } else { ret = JQ_ERROR_UNKNOWN as libc::c_int }
        jv_free(exit_code);
        let mut error_message: jv = jq_get_error_message(jq);
        if jv_get_kind(error_message) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"jq: error: %s\x00" as *const u8 as *const libc::c_char,
                    jv_string_value(error_message));
        } else if !(jv_get_kind(error_message) as libc::c_uint ==
                        JV_KIND_NULL as libc::c_int as libc::c_uint) {
            if jv_is_valid(error_message) != 0 {
                error_message =
                    jv_dump_string(jv_copy(error_message), 0 as libc::c_int);
                fprintf(stderr,
                        b"jq: error: %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        jv_string_value(error_message));
            }
        }
        fflush(stderr);
        jv_free(error_message);
    } else if jv_invalid_has_msg(jv_copy(result)) != 0 {
        // Uncaught jq exception
        let mut msg: jv = jv_invalid_get_msg(jv_copy(result));
        let mut input_pos: jv = jq_util_input_get_position(jq);
        if jv_get_kind(msg) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"jq: error (at %s): %s\n\x00" as *const u8 as
                        *const libc::c_char, jv_string_value(input_pos),
                    jv_string_value(msg));
        } else {
            msg = jv_dump_string(msg, 0 as libc::c_int);
            fprintf(stderr,
                    b"jq: error (at %s) (not a string): %s\n\x00" as *const u8
                        as *const libc::c_char, jv_string_value(input_pos),
                    jv_string_value(msg));
        }
        ret = JQ_ERROR_UNKNOWN as libc::c_int;
        jv_free(input_pos);
        jv_free(msg);
    }
    jv_free(result);
    return ret;
}
unsafe extern "C" fn debug_cb(mut data: *mut libc::c_void, mut input: jv) {
    let mut dumpopts: libc::c_int = *(data as *mut libc::c_int);
    jv_dumpf(jv_array_append(jv_array_append(jv_array(),
                                             jv_string(b"DEBUG:\x00" as
                                                           *const u8 as
                                                           *const libc::c_char)),
                             input), stderr,
             dumpopts & !(JV_PRINT_PRETTY as libc::c_int));
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
