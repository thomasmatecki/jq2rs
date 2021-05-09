#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jq_state;
    pub type inst;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __strdup(__string: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __rawmemchr(__s: *const libc::c_void, __c: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
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
    fn jv_equal(_: jv, _: jv) -> libc::c_int;
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
    fn jv_true() -> jv;
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
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_string_split(j: jv, sep: jv) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_load_file(_: *const libc::c_char, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jq_report_error(_: *mut jq_state, _: jv);
    #[no_mangle]
    fn jq_get_jq_origin(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_get_prog_origin(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn jq_get_lib_dirs(_: *mut jq_state) -> jv;
    #[no_mangle]
    fn locfile_init(_: *mut jq_state, _: *const libc::c_char,
                    _: *const libc::c_char, _: libc::c_int) -> *mut locfile;
    #[no_mangle]
    fn locfile_free(_: *mut locfile);
    #[no_mangle]
    fn gen_noop() -> block;
    #[no_mangle]
    fn gen_const(constant: jv) -> block;
    #[no_mangle]
    fn gen_const_global(constant: jv, name: *const libc::c_char) -> block;
    #[no_mangle]
    fn block_is_const(b: block) -> libc::c_int;
    #[no_mangle]
    fn block_module_meta(b: block) -> jv;
    #[no_mangle]
    fn gen_import(name: *const libc::c_char, as_0: *const libc::c_char,
                  is_data: libc::c_int) -> block;
    #[no_mangle]
    fn gen_import_meta(import: block, metadata: block) -> block;
    #[no_mangle]
    fn jq_parse_library(locations: *mut locfile, answer: *mut block)
     -> libc::c_int;
    #[no_mangle]
    fn jq_parse(source: *mut locfile, answer: *mut block) -> libc::c_int;
    #[no_mangle]
    fn block_join(a: block, b: block) -> block;
    #[no_mangle]
    fn block_bind_library(binder: block, body: block, bindflags: libc::c_int,
                          libname: *const libc::c_char) -> block;
    #[no_mangle]
    fn block_bind_self(binder: block, bindflags: libc::c_int) -> block;
    #[no_mangle]
    fn block_drop_unreferenced(body: block) -> block;
    #[no_mangle]
    fn block_take_imports(body: *mut block) -> jv;
    #[no_mangle]
    fn block_free(_: block);
    #[no_mangle]
    fn expand_path(_: jv) -> jv;
    #[no_mangle]
    fn jq_realpath(_: jv) -> jv;
    #[no_mangle]
    fn jv_mem_realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub struct block {
    pub first: *mut inst,
    pub last: *mut inst,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lib_loading_state {
    pub names: *mut *mut libc::c_char,
    pub defs: *mut block,
    pub ct: uint64_t,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
unsafe extern "C" fn path_is_relative(mut p: jv) -> libc::c_int {
    let mut s: *const libc::c_char = jv_string_value(p);
    let mut res: libc::c_int =
        (*s as libc::c_int != '/' as i32) as libc::c_int;
    jv_free(p);
    return res;
}
// Assumes name has been validated
unsafe extern "C" fn jv_basename(mut name: jv) -> jv {
    let mut s: *const libc::c_char = jv_string_value(name);
    let mut p: *const libc::c_char = strrchr(s, '/' as i32);
    if p.is_null() { return name }
    let mut res: jv =
        jv_string_fmt(b"%s\x00" as *const u8 as *const libc::c_char, p);
    jv_free(name);
    return res;
}
unsafe extern "C" fn default_search(mut jq: *mut jq_state, mut value: jv)
 -> jv {
    if jv_is_valid(value) == 0 {
        // dependent didn't say; prepend . to system search path listj
        jv_free(value);
        return jv_array_concat(jv_array_append(jv_array(),
                                               jv_string(b".\x00" as *const u8
                                                             as
                                                             *const libc::c_char)),
                               jq_get_lib_dirs(jq))
    }
    if jv_get_kind(value) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        return jv_array_append(jv_array(), value)
    }
    return value;
}
// Loads the library at lib_path into lib_state, putting the library's defs
// into *out_block
unsafe extern "C" fn load_library(mut jq: *mut jq_state, mut lib_path: jv,
                                  mut is_data: libc::c_int,
                                  mut raw: libc::c_int,
                                  mut optional: libc::c_int,
                                  mut as_0: *const libc::c_char,
                                  mut out_block: *mut block,
                                  mut lib_state: *mut lib_loading_state)
 -> libc::c_int {
    let mut nerrors: libc::c_int = 0 as libc::c_int;
    let mut src: *mut locfile = 0 as *mut locfile;
    let mut program: block =
        block{first: 0 as *mut inst, last: 0 as *mut inst,};
    let mut data: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if is_data != 0 && raw == 0 {
        data = jv_load_file(jv_string_value(lib_path), 0 as libc::c_int)
    } else {
        data = jv_load_file(jv_string_value(lib_path), 1 as libc::c_int)
    }
    let mut state_idx: libc::c_int = 0;
    if jv_is_valid(data) == 0 {
        program = gen_noop();
        if optional == 0 {
            if jv_invalid_has_msg(jv_copy(data)) != 0 {
                data = jv_invalid_get_msg(data)
            } else {
                data =
                    jv_string(b"unknown error\x00" as *const u8 as
                                  *const libc::c_char)
            }
            jq_report_error(jq,
                            jv_string_fmt(b"jq: error loading data file %s: %s\n\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          jv_string_value(lib_path),
                                          jv_string_value(data)));
            nerrors += 1
        }
    } else {
        if is_data != 0 {
            // import "foo" as $bar;
            program = gen_const_global(jv_copy(data), as_0)
        } else {
            // import "foo" as bar;
            src =
                locfile_init(jq, jv_string_value(lib_path),
                             jv_string_value(data),
                             jv_string_length_bytes(jv_copy(data)));
            nerrors += jq_parse_library(src, &mut program);
            locfile_free(src);
            if nerrors == 0 as libc::c_int {
                let mut lib_origin: *mut libc::c_char =
                    if 0 != 0 &&
                           (jv_string_value(lib_path).offset(1 as libc::c_int
                                                                 as isize) as
                                *const libc::c_void as
                                size_t).wrapping_sub(jv_string_value(lib_path)
                                                         as
                                                         *const libc::c_void
                                                         as size_t) ==
                               1 as libc::c_int as libc::c_ulong {
                        if *jv_string_value(lib_path).offset(0 as libc::c_int
                                                                 as isize) as
                               libc::c_int == '\u{0}' as i32 {
                            calloc(1 as libc::c_int as size_t,
                                   1 as libc::c_int as size_t) as
                                *mut libc::c_char
                        } else {
                            ({
                                 let mut __len: size_t =
                                     strlen(jv_string_value(lib_path)).wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong);
                                 let mut __retval: *mut libc::c_char =
                                     malloc(__len) as *mut libc::c_char;
                                 if !__retval.is_null() {
                                     __retval =
                                         memcpy(__retval as *mut libc::c_void,
                                                jv_string_value(lib_path) as
                                                    *const libc::c_void,
                                                __len) as *mut libc::c_char
                                 }
                                 __retval
                             })
                        }
                    } else { __strdup(jv_string_value(lib_path)) };
                nerrors +=
                    process_dependencies(jq, jq_get_jq_origin(jq),
                                         jv_string(dirname(lib_origin)),
                                         &mut program, lib_state);
                free(lib_origin as *mut libc::c_void);
                program =
                    block_bind_self(program, OP_IS_CALL_PSEUDO as libc::c_int)
            }
        }
        let fresh0 = (*lib_state).ct;
        (*lib_state).ct = (*lib_state).ct.wrapping_add(1);
        state_idx = fresh0 as libc::c_int;
        (*lib_state).names =
            jv_mem_realloc((*lib_state).names as *mut libc::c_void,
                           (*lib_state).ct.wrapping_mul(::std::mem::size_of::<*const libc::c_char>()
                                                            as libc::c_ulong))
                as *mut *mut libc::c_char;
        (*lib_state).defs =
            jv_mem_realloc((*lib_state).defs as *mut libc::c_void,
                           (*lib_state).ct.wrapping_mul(::std::mem::size_of::<block>()
                                                            as libc::c_ulong))
                as *mut block;
        let ref mut fresh1 = *(*lib_state).names.offset(state_idx as isize);
        *fresh1 =
            if 0 != 0 &&
                   (jv_string_value(lib_path).offset(1 as libc::c_int as
                                                         isize) as
                        *const libc::c_void as
                        size_t).wrapping_sub(jv_string_value(lib_path) as
                                                 *const libc::c_void as
                                                 size_t) ==
                       1 as libc::c_int as libc::c_ulong {
                if *jv_string_value(lib_path).offset(0 as libc::c_int as
                                                         isize) as libc::c_int
                       == '\u{0}' as i32 {
                    calloc(1 as libc::c_int as size_t,
                           1 as libc::c_int as size_t) as *mut libc::c_char
                } else {
                    ({
                         let mut __len: size_t =
                             strlen(jv_string_value(lib_path)).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong);
                         let mut __retval: *mut libc::c_char =
                             malloc(__len) as *mut libc::c_char;
                         if !__retval.is_null() {
                             __retval =
                                 memcpy(__retval as *mut libc::c_void,
                                        jv_string_value(lib_path) as
                                            *const libc::c_void, __len) as
                                     *mut libc::c_char
                         }
                         __retval
                     })
                }
            } else { __strdup(jv_string_value(lib_path)) };
        *(*lib_state).defs.offset(state_idx as isize) = program
    }
    *out_block = program;
    jv_free(lib_path);
    jv_free(data);
    return nerrors;
}
// FIXME It'd be nice to have an option to search the same search path
// as we do in process_dependencies.
#[no_mangle]
pub unsafe extern "C" fn load_module_meta(mut jq: *mut jq_state,
                                          mut mod_relpath: jv) -> jv {
    // We can't know the caller's origin; we could though, if it was passed in
    let mut lib_path: jv =
        find_lib(jq, validate_relpath(mod_relpath), jq_get_lib_dirs(jq),
                 b".jq\x00" as *const u8 as *const libc::c_char,
                 jq_get_jq_origin(jq), jv_null());
    if jv_is_valid(lib_path) == 0 { return lib_path }
    let mut meta: jv = jv_null();
    let mut data: jv =
        jv_load_file(jv_string_value(lib_path), 1 as libc::c_int);
    if jv_is_valid(data) != 0 {
        let mut program: block =
            block{first: 0 as *mut inst, last: 0 as *mut inst,};
        let mut src: *mut locfile =
            locfile_init(jq, jv_string_value(lib_path), jv_string_value(data),
                         jv_string_length_bytes(jv_copy(data)));
        let mut nerrors: libc::c_int = jq_parse_library(src, &mut program);
        if nerrors == 0 as libc::c_int {
            meta = block_module_meta(program);
            if jv_get_kind(meta) as libc::c_uint ==
                   JV_KIND_NULL as libc::c_int as libc::c_uint {
                meta = jv_object()
            }
            meta =
                jv_object_set(meta,
                              jv_string(b"deps\x00" as *const u8 as
                                            *const libc::c_char),
                              block_take_imports(&mut program))
        }
        locfile_free(src);
        block_free(program);
    }
    jv_free(lib_path);
    jv_free(data);
    return meta;
}
#[no_mangle]
pub unsafe extern "C" fn load_program(mut jq: *mut jq_state,
                                      mut src: *mut locfile,
                                      mut out_block: *mut block)
 -> libc::c_int {
    let mut nerrors: libc::c_int = 0 as libc::c_int;
    let mut program: block =
        block{first: 0 as *mut inst, last: 0 as *mut inst,};
    let mut lib_state: lib_loading_state =
        {
            let mut init =
                lib_loading_state{names: 0 as *mut *mut libc::c_char,
                                  defs: 0 as *mut block,
                                  ct: 0 as libc::c_int as uint64_t,};
            init
        };
    nerrors = jq_parse(src, &mut program);
    if nerrors != 0 { return nerrors }
    let mut home: *mut libc::c_char =
        getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
    if !home.is_null() {
        // silently ignore no $HOME
        /* Import ~/.jq as a library named "" found in $HOME */
        let mut import: block =
            gen_import_meta(gen_import(b"\x00" as *const u8 as
                                           *const libc::c_char,
                                       0 as *const libc::c_char,
                                       0 as libc::c_int),
                            gen_const(jv_object_set(jv_object_set(jv_object(),
                                                                  jv_string(b"optional\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char),
                                                                  jv_true()),
                                                    jv_string(b"search\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char),
                                                    jv_string(home))));
        program = block_join(import, program)
    }
    nerrors =
        process_dependencies(jq, jq_get_jq_origin(jq), jq_get_prog_origin(jq),
                             &mut program, &mut lib_state);
    let mut libs: block = gen_noop();
    let mut i: uint64_t = 0 as libc::c_int as uint64_t;
    while i < lib_state.ct {
        free(*lib_state.names.offset(i as isize) as *mut libc::c_void);
        if nerrors == 0 as libc::c_int &&
               block_is_const(*lib_state.defs.offset(i as isize)) == 0 {
            libs = block_join(libs, *lib_state.defs.offset(i as isize))
        } else { block_free(*lib_state.defs.offset(i as isize)); }
        i = i.wrapping_add(1)
    }
    free(lib_state.names as *mut libc::c_void);
    free(lib_state.defs as *mut libc::c_void);
    if nerrors != 0 {
        block_free(program);
    } else { *out_block = block_drop_unreferenced(block_join(libs, program)) }
    return nerrors;
}
