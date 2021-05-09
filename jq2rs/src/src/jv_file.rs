#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    pub type jv_parser;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_copy(_: jv) -> jv;
    #[no_mangle]
    fn jv_free(_: jv);
    #[no_mangle]
    fn jv_invalid_with_msg(_: jv) -> jv;
    #[no_mangle]
    fn jv_invalid_has_msg(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_string_append_buf(a: jv, buf: *const libc::c_char, len: libc::c_int)
     -> jv;
    #[no_mangle]
    fn jv_parser_set_buf(_: *mut jv_parser, _: *const libc::c_char,
                         _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn jv_parser_next(_: *mut jv_parser) -> jv;
    #[no_mangle]
    fn jv_parser_free(_: *mut jv_parser);
    #[no_mangle]
    fn jv_parser_new(_: libc::c_int) -> *mut jv_parser;
    #[no_mangle]
    fn jvp_utf8_backtrack(start: *const libc::c_char,
                          min: *const libc::c_char,
                          missing_bytes: *mut libc::c_int)
     -> *const libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *mut jv_refcnt,
    pub number: libc::c_double,
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat)
 -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jv_load_file(mut filename: *const libc::c_char,
                                      mut raw: libc::c_int) -> jv {
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut fd: libc::c_int = open(filename, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return jv_invalid_with_msg(jv_string_fmt(b"Could not open %s: %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 filename,
                                                 strerror(*__errno_location())))
    }
    if fstat(fd, &mut sb) == -(1 as libc::c_int) ||
           sb.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
               0o40000 as libc::c_int as libc::c_uint {
        close(fd);
        return jv_invalid_with_msg(jv_string_fmt(b"Could not open %s: %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 filename,
                                                 b"It\'s a directory\x00" as
                                                     *const u8 as
                                                     *const libc::c_char))
    }
    let mut file: *mut FILE =
        fdopen(fd, b"r\x00" as *const u8 as *const libc::c_char);
    let mut parser: *mut jv_parser = 0 as *mut jv_parser;
    let mut data: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if file.is_null() {
        close(fd);
        return jv_invalid_with_msg(jv_string_fmt(b"Could not open %s: %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 filename,
                                                 strerror(*__errno_location())))
    }
    if raw != 0 {
        data = jv_string(b"\x00" as *const u8 as *const libc::c_char)
    } else { data = jv_array(); parser = jv_parser_new(0 as libc::c_int) }
    // To avoid mangling UTF-8 multi-byte sequences that cross the end of our read
  // buffer, we need to be able to read the remainder of a sequence and add that
  // before appending.
    let max_utf8_len: libc::c_int = 4 as libc::c_int;
    let mut buf: [libc::c_char; 4100] = [0; 4100];
    while feof(file) == 0 && ferror(file) == 0 {
        let mut n: size_t =
            fread(buf.as_mut_ptr() as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  (::std::mem::size_of::<[libc::c_char; 4100]>() as
                       libc::c_ulong).wrapping_sub(max_utf8_len as
                                                       libc::c_ulong), file);
        let mut len: libc::c_int = 0 as libc::c_int;
        if n == 0 as libc::c_int as libc::c_ulong { continue ; }
        if !jvp_utf8_backtrack(buf.as_mut_ptr().offset(n.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                                           as isize),
                               buf.as_mut_ptr(), &mut len).is_null() &&
               len > 0 as libc::c_int && feof(file) == 0 && ferror(file) == 0
           {
            n =
                (n as
                     libc::c_ulong).wrapping_add(fread(buf.as_mut_ptr().offset(n
                                                                                   as
                                                                                   isize)
                                                           as
                                                           *mut libc::c_void,
                                                       1 as libc::c_int as
                                                           libc::c_ulong,
                                                       len as libc::c_ulong,
                                                       file)) as size_t as
                    size_t
        }
        if raw != 0 {
            data =
                jv_string_append_buf(data, buf.as_mut_ptr(), n as libc::c_int)
        } else {
            jv_parser_set_buf(parser, buf.as_mut_ptr(), n as libc::c_int,
                              (feof(file) == 0) as libc::c_int);
            let mut value: jv =
                jv{kind_flags: 0,
                   pad_: 0,
                   offset: 0,
                   size: 0,
                   u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
            loop  {
                value = jv_parser_next(parser);
                if !(jv_is_valid(value) != 0) { break ; }
                data = jv_array_append(data, value)
            }
            if !(jv_invalid_has_msg(jv_copy(value)) != 0) { continue ; }
            jv_free(data);
            data = value;
            break ;
        }
    }
    if raw == 0 { jv_parser_free(parser); }
    let mut badread: libc::c_int = ferror(file);
    if fclose(file) != 0 as libc::c_int || badread != 0 {
        jv_free(data);
        return jv_invalid_with_msg(jv_string_fmt(b"Error reading from %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 filename))
    }
    return data;
}
