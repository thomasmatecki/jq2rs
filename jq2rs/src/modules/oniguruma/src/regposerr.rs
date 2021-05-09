#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_t {
    pub onig: *mut libc::c_void,
    pub re_nsub: size_t,
    pub comp_options: libc::c_int,
}
static mut ESTRING: [*mut libc::c_char; 17] =
    [0 as *const libc::c_char as *mut libc::c_char,
     b"failed to match\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"Invalid regular expression\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"invalid collating element referenced\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"invalid character class type referenced\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"bad backslash-escape sequence\x00" as *const u8 as *const libc::c_char
         as *mut libc::c_char,
     b"invalid back reference number\x00" as *const u8 as *const libc::c_char
         as *mut libc::c_char,
     b"imbalanced [ and ]\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"imbalanced ( and )\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"imbalanced { and }\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"invalid repeat range {n,m}\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"invalid range\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"Out of memory\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"? * + not preceded by valid regular expression\x00" as *const u8 as
         *const libc::c_char as *mut libc::c_char,
     b"internal error\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"invalid wide char value\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"invalid argument\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn regerror(mut posix_ecode: libc::c_int,
                                  mut reg: *const regex_t,
                                  mut buf: *mut libc::c_char,
                                  mut size: size_t) -> size_t {
    let mut s: *mut libc::c_char =
        0 as
            *mut libc::c_char; /* use strlen() because s is ascii encoding. */
    let mut tbuf: [libc::c_char; 35] = [0; 35];
    let mut len: size_t = 0;
    if posix_ecode > 0 as libc::c_int &&
           posix_ecode <
               (::std::mem::size_of::<[*mut libc::c_char; 17]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong) as
                   libc::c_int {
        s = ESTRING[posix_ecode as usize]
    } else if posix_ecode == 0 as libc::c_int {
        s = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        snprintf(tbuf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong,
                 b"undefined error code (%d)\x00" as *const u8 as
                     *const libc::c_char, posix_ecode);
        s = tbuf.as_mut_ptr()
    }
    len = strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !buf.is_null() && size > 0 as libc::c_int as libc::c_ulong {
        libc::strncpy(buf, s,
                      size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                          libc::size_t);
        *buf.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                        isize) = '\u{0}' as i32 as libc::c_char
    }
    return len;
}
