#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
static mut utf8_coding_length: [libc::c_uchar; 256] =
    [0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar];
static mut utf8_coding_bits: [libc::c_uchar; 256] =
    [0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar, 0x7 as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar, 0x7 as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar];
static mut utf8_first_codepoint: [libc::c_int; 5] =
    [0 as libc::c_int, 0 as libc::c_int, 0x80 as libc::c_int,
     0x800 as libc::c_int, 0x10000 as libc::c_int];
// jvp_utf8_backtrack returns the beginning of the last codepoint in the
// string, assuming that start is the last byte in the string.
// If the last codepoint is incomplete, returns the number of missing bytes via
// *missing_bytes.  If there are no leading bytes or an invalid byte is
// encountered, NULL is returned and *missing_bytes is not altered.
#[no_mangle]
pub unsafe extern "C" fn jvp_utf8_backtrack(mut start: *const libc::c_char,
                                            mut min: *const libc::c_char,
                                            mut missing_bytes:
                                                *mut libc::c_int)
 -> *const libc::c_char {
    if min <= start {
    } else {
        __assert_fail(b"min <= start\x00" as *const u8 as *const libc::c_char,
                      b"src/jv_unicode.c\x00" as *const u8 as
                          *const libc::c_char,
                      12 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"const char *jvp_utf8_backtrack(const char *, const char *, int *)\x00")).as_ptr());
    };
    if min == start { return min }
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut seen: libc::c_int = 1 as libc::c_int;
    while start >= min &&
              {
                  length =
                      utf8_coding_length[*start as libc::c_uchar as usize] as
                          libc::c_int;
                  (length) ==
                      255 as libc::c_int as libc::c_uchar as libc::c_int
              } {
        start = start.offset(-1);
        seen += 1
    }
    if length == 0 as libc::c_int ||
           length == 255 as libc::c_int as libc::c_uchar as libc::c_int ||
           length - seen < 0 as libc::c_int {
        return 0 as *const libc::c_char
    }
    if !missing_bytes.is_null() { *missing_bytes = length - seen }
    return start;
}
#[no_mangle]
pub unsafe extern "C" fn jvp_utf8_next(mut in_0: *const libc::c_char,
                                       mut end: *const libc::c_char,
                                       mut codepoint_ret: *mut libc::c_int)
 -> *const libc::c_char {
    if in_0 <= end {
    } else {
        __assert_fail(b"in <= end\x00" as *const u8 as *const libc::c_char,
                      b"src/jv_unicode.c\x00" as *const u8 as
                          *const libc::c_char,
                      30 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"const char *jvp_utf8_next(const char *, const char *, int *)\x00")).as_ptr());
    };
    if in_0 == end { return 0 as *const libc::c_char }
    let mut codepoint: libc::c_int = -(1 as libc::c_int);
    let mut first: libc::c_uchar =
        *in_0.offset(0 as libc::c_int as isize) as libc::c_uchar;
    let mut length: libc::c_int =
        utf8_coding_length[first as usize] as libc::c_int;
    if first as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
        /* Fast-path for ASCII */
        codepoint = first as libc::c_int;
        length = 1 as libc::c_int
    } else if length == 0 as libc::c_int ||
                  length == 255 as libc::c_int as libc::c_uchar as libc::c_int
     {
        /* Bad single byte - either an invalid byte or an out-of-place continuation byte */
        length = 1 as libc::c_int
    } else if in_0.offset(length as isize) > end {
        /* String ends before UTF8 sequence ends */
        length = end.wrapping_offset_from(in_0) as libc::c_long as libc::c_int
    } else {
        codepoint =
            (*in_0.offset(0 as libc::c_int as isize) as libc::c_uint &
                 utf8_coding_bits[first as usize] as libc::c_uint) as
                libc::c_int;
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < length {
            let mut ch: libc::c_uint =
                *in_0.offset(i as isize) as libc::c_uchar as libc::c_uint;
            if utf8_coding_length[ch as usize] as libc::c_int !=
                   255 as libc::c_int as libc::c_uchar as libc::c_int {
                /* Invalid UTF8 sequence - not followed by the right number of continuation bytes */
                codepoint = -(1 as libc::c_int);
                length = i;
                break ;
            } else {
                codepoint =
                    ((codepoint << 6 as libc::c_int) as libc::c_uint |
                         ch & 0x3f as libc::c_int as libc::c_uint) as
                        libc::c_int;
                i += 1
            }
        }
        if codepoint < utf8_first_codepoint[length as usize] {
            /* Overlong UTF8 sequence */
            codepoint = -(1 as libc::c_int)
        }
        if 0xd800 as libc::c_int <= codepoint &&
               codepoint <= 0xdfff as libc::c_int {
            /* Surrogate codepoints can't be encoded in UTF8 */
            codepoint = -(1 as libc::c_int)
        }
        if codepoint > 0x10ffff as libc::c_int {
            /* Outside Unicode range */
            codepoint = -(1 as libc::c_int)
        }
    }
    if length > 0 as libc::c_int {
    } else {
        __assert_fail(b"length > 0\x00" as *const u8 as *const libc::c_char,
                      b"src/jv_unicode.c\x00" as *const u8 as
                          *const libc::c_char,
                      72 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"const char *jvp_utf8_next(const char *, const char *, int *)\x00")).as_ptr());
    };
    *codepoint_ret = codepoint;
    return in_0.offset(length as isize);
}
#[no_mangle]
pub unsafe extern "C" fn jvp_utf8_is_valid(mut in_0: *const libc::c_char,
                                           mut end: *const libc::c_char)
 -> libc::c_int {
    let mut codepoint: libc::c_int = 0;
    loop  {
        in_0 = jvp_utf8_next(in_0, end, &mut codepoint);
        if in_0.is_null() { break ; }
        if codepoint == -(1 as libc::c_int) { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
/* Assumes startchar is the first byte of a valid character sequence */
#[no_mangle]
pub unsafe extern "C" fn jvp_utf8_decode_length(mut startchar: libc::c_char)
 -> libc::c_int {
    if startchar as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int
    } else if startchar as libc::c_int & 0xe0 as libc::c_int ==
                  0xc0 as libc::c_int { // 0___ ____
        return 2 as libc::c_int
    } else if startchar as libc::c_int & 0xf0 as libc::c_int ==
                  0xe0 as libc::c_int { // 110_ ____
        return 3 as libc::c_int
    } else { return 4 as libc::c_int }; // 1110 ____
    // 1111 ____
}
#[no_mangle]
pub unsafe extern "C" fn jvp_utf8_encode_length(mut codepoint: libc::c_int)
 -> libc::c_int {
    if codepoint <= 0x7f as libc::c_int {
        return 1 as libc::c_int
    } else if codepoint <= 0x7ff as libc::c_int {
        return 2 as libc::c_int
    } else if codepoint <= 0xffff as libc::c_int {
        return 3 as libc::c_int
    } else { return 4 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn jvp_utf8_encode(mut codepoint: libc::c_int,
                                         mut out: *mut libc::c_char)
 -> libc::c_int {
    if codepoint >= 0 as libc::c_int && codepoint <= 0x10ffff as libc::c_int {
    } else {
        __assert_fail(b"codepoint >= 0 && codepoint <= 0x10FFFF\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv_unicode.c\x00" as *const u8 as
                          *const libc::c_char,
                      101 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int jvp_utf8_encode(int, char *)\x00")).as_ptr());
    };
    let mut start: *mut libc::c_char = out;
    if codepoint <= 0x7f as libc::c_int {
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = codepoint as libc::c_char
    } else if codepoint <= 0x7ff as libc::c_int {
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 =
            (0xc0 as libc::c_int +
                 ((codepoint & 0x7c0 as libc::c_int) >> 6 as libc::c_int)) as
                libc::c_char;
        let fresh2 = out;
        out = out.offset(1);
        *fresh2 =
            (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int)) as
                libc::c_char
    } else if codepoint <= 0xffff as libc::c_int {
        let fresh3 = out;
        out = out.offset(1);
        *fresh3 =
            (0xe0 as libc::c_int +
                 ((codepoint & 0xf000 as libc::c_int) >> 12 as libc::c_int))
                as libc::c_char;
        let fresh4 = out;
        out = out.offset(1);
        *fresh4 =
            (0x80 as libc::c_int +
                 ((codepoint & 0xfc0 as libc::c_int) >> 6 as libc::c_int)) as
                libc::c_char;
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 =
            (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int)) as
                libc::c_char
    } else {
        let fresh6 = out;
        out = out.offset(1);
        *fresh6 =
            (0xf0 as libc::c_int +
                 ((codepoint & 0x1c0000 as libc::c_int) >> 18 as libc::c_int))
                as libc::c_char;
        let fresh7 = out;
        out = out.offset(1);
        *fresh7 =
            (0x80 as libc::c_int +
                 ((codepoint & 0x3f000 as libc::c_int) >> 12 as libc::c_int))
                as libc::c_char;
        let fresh8 = out;
        out = out.offset(1);
        *fresh8 =
            (0x80 as libc::c_int +
                 ((codepoint & 0xfc0 as libc::c_int) >> 6 as libc::c_int)) as
                libc::c_char;
        let fresh9 = out;
        out = out.offset(1);
        *fresh9 =
            (0x80 as libc::c_int + (codepoint & 0x3f as libc::c_int)) as
                libc::c_char
    }
    if out.wrapping_offset_from(start) as libc::c_long ==
           jvp_utf8_encode_length(codepoint) as libc::c_long {
    } else {
        __assert_fail(b"out - start == jvp_utf8_encode_length(codepoint)\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv_unicode.c\x00" as *const u8 as
                          *const libc::c_char,
                      118 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int jvp_utf8_encode(int, char *)\x00")).as_ptr());
    };
    return out.wrapping_offset_from(start) as libc::c_long as libc::c_int;
}
