#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __strdup(__string: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn pthread_once(__once_control: *mut pthread_once_t,
                    __init_routine: Option<unsafe extern "C" fn() -> ()>)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    #[no_mangle]
    fn pthread_setspecific(__key: pthread_key_t,
                           __pointer: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_key_create(__key: *mut pthread_key_t,
                          __destr_function:
                              Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void)
                                         -> ()>) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type pthread_key_t = libc::c_uint;
pub type pthread_once_t = libc::c_int;
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
pub type jv_nomem_handler_f
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nomem_handler {
    pub handler: jv_nomem_handler_f,
    pub data: *mut libc::c_void,
}
/* !HAVE_PTHREAD_KEY_CREATE */
/* USE_TLS */
#[no_mangle]
pub static mut nomem_handler_key: pthread_key_t = 0;
#[no_mangle]
pub static mut mem_once: pthread_once_t = 0 as libc::c_int;
unsafe extern "C" fn tsd_fini() {
    let mut nomem_handler: *mut nomem_handler =
        0 as *mut nomem_handler; // cannot fail
    nomem_handler =
        pthread_getspecific(nomem_handler_key) as
            *mut nomem_handler; // Maybe handler() will longjmp() to safety
    if !nomem_handler.is_null() {
        pthread_setspecific(nomem_handler_key, 0 as *const libc::c_void);
        free(nomem_handler as *mut libc::c_void);
    };
}
unsafe extern "C" fn tsd_init() {
    if pthread_key_create(&mut nomem_handler_key, None) != 0 as libc::c_int {
        fprintf(stderr,
                b"jq: error: cannot create thread specific key\x00" as
                    *const u8 as *const libc::c_char);
        abort();
    }
    if atexit(Some(tsd_fini as unsafe extern "C" fn() -> ())) !=
           0 as libc::c_int {
        fprintf(stderr,
                b"jq: error: cannot set an exit handler\x00" as *const u8 as
                    *const libc::c_char);
        abort();
    }
    let mut nomem_handler: *mut nomem_handler =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<nomem_handler>() as libc::c_ulong) as
            *mut nomem_handler;
    if pthread_setspecific(nomem_handler_key,
                           nomem_handler as *const libc::c_void) !=
           0 as libc::c_int {
        fprintf(stderr,
                b"jq: error: cannot set thread specific data\x00" as *const u8
                    as *const libc::c_char);
        abort();
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_nomem_handler(mut handler: jv_nomem_handler_f,
                                          mut data: *mut libc::c_void) {
    pthread_once(&mut mem_once,
                 Some(tsd_init as unsafe extern "C" fn() -> ()));
    let mut nomem_handler: *mut nomem_handler = 0 as *mut nomem_handler;
    nomem_handler =
        pthread_getspecific(nomem_handler_key) as *mut nomem_handler;
    if nomem_handler.is_null() {
        handler.expect("non-null function pointer")(data);
        fprintf(stderr,
                b"jq: error: cannot allocate memory\n\x00" as *const u8 as
                    *const libc::c_char);
        abort();
    }
    (*nomem_handler).handler = handler;
    (*nomem_handler).data = data;
}
unsafe extern "C" fn memory_exhausted() {
    let mut nomem_handler: *mut nomem_handler = 0 as *mut nomem_handler;
    pthread_once(&mut mem_once,
                 Some(tsd_init as unsafe extern "C" fn() -> ()));
    nomem_handler =
        pthread_getspecific(nomem_handler_key) as *mut nomem_handler;
    if !nomem_handler.is_null() {
        (*nomem_handler).handler.expect("non-null function pointer")((*nomem_handler).data);
    }
    // Or not
    fprintf(stderr,
            b"jq: error: cannot allocate memory\n\x00" as *const u8 as
                *const libc::c_char);
    abort();
}
/* HAVE_PTHREAD_KEY_CREATE */
/* USE_TLS */
#[no_mangle]
pub unsafe extern "C" fn jv_mem_alloc(mut sz: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(sz);
    if p.is_null() { memory_exhausted(); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn jv_mem_alloc_unguarded(mut sz: size_t)
 -> *mut libc::c_void {
    return malloc(sz);
}
#[no_mangle]
pub unsafe extern "C" fn jv_mem_calloc(mut nemb: size_t, mut sz: size_t)
 -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nemb, sz);
    if p.is_null() { memory_exhausted(); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn jv_mem_calloc_unguarded(mut nemb: size_t,
                                                 mut sz: size_t)
 -> *mut libc::c_void {
    return calloc(nemb, sz);
}
#[no_mangle]
pub unsafe extern "C" fn jv_mem_strdup(mut s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut p: *mut libc::c_char =
        if 0 != 0 &&
               (s.offset(1 as libc::c_int as isize) as *const libc::c_void as
                    size_t).wrapping_sub(s as *const libc::c_void as size_t)
                   == 1 as libc::c_int as libc::c_ulong {
            if *s.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '\u{0}' as i32 {
                calloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
                    as *mut libc::c_char
            } else {
                ({
                     let mut __len: size_t =
                         strlen(s).wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong);
                     let mut __retval: *mut libc::c_char =
                         malloc(__len) as *mut libc::c_char;
                     if !__retval.is_null() {
                         __retval =
                             memcpy(__retval as *mut libc::c_void,
                                    s as *const libc::c_void, __len) as
                                 *mut libc::c_char
                     }
                     __retval
                 })
            }
        } else { __strdup(s) };
    if p.is_null() { memory_exhausted(); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn jv_mem_strdup_unguarded(mut s: *const libc::c_char)
 -> *mut libc::c_char {
    return if 0 != 0 &&
                  (s.offset(1 as libc::c_int as isize) as *const libc::c_void
                       as
                       size_t).wrapping_sub(s as *const libc::c_void as
                                                size_t) ==
                      1 as libc::c_int as libc::c_ulong {
               if *s.offset(0 as libc::c_int as isize) as libc::c_int ==
                      '\u{0}' as i32 {
                   calloc(1 as libc::c_int as size_t,
                          1 as libc::c_int as size_t) as *mut libc::c_char
               } else {
                   ({
                        let mut __len: size_t =
                            strlen(s).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong);
                        let mut __retval: *mut libc::c_char =
                            malloc(__len) as *mut libc::c_char;
                        if !__retval.is_null() {
                            __retval =
                                memcpy(__retval as *mut libc::c_void,
                                       s as *const libc::c_void, __len) as
                                    *mut libc::c_char
                        }
                        __retval
                    })
               }
           } else { __strdup(s) };
}
#[no_mangle]
pub unsafe extern "C" fn jv_mem_free(mut p: *mut libc::c_void) { free(p); }
#[no_mangle]
pub unsafe extern "C" fn jv_mem_realloc(mut p: *mut libc::c_void,
                                        mut sz: size_t) -> *mut libc::c_void {
    p = realloc(p, sz);
    if p.is_null() { memory_exhausted(); }
    return p;
}
#[no_mangle]
pub static mut jv_mem_uninitialised: libc::c_char = 0;
#[no_mangle]
pub unsafe extern "C" fn jv_mem_uninit_setup() {
    // ignore warning that this reads uninitialized memory - that's the point!
    let mut p: *mut libc::c_char =
        malloc(1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    ::std::ptr::write_volatile(&mut jv_mem_uninitialised as *mut libc::c_char,
                               *p);
    free(p as *mut libc::c_void);
}
