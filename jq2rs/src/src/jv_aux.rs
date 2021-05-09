#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type jv_refcnt;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* All of the fields of this struct are private.
   Really. Do not play with them. */
    /* array offsets */
    /*
 * All jv_* functions consume (decref) input and produce (incref) output
 * Except jv_copy
 */
    #[no_mangle]
    fn jv_false() -> jv;
    #[no_mangle]
    fn jv_get_kind(_: jv) -> jv_kind;
    #[no_mangle]
    fn jv_kind_name(_: jv_kind) -> *const libc::c_char;
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
    fn jv_null() -> jv;
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn jv_array_append(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_array_slice(_: jv, _: libc::c_int, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_indexes(_: jv, _: jv) -> jv;
    #[no_mangle]
    fn jv_string(_: *const libc::c_char) -> jv;
    #[no_mangle]
    fn jv_string_length_bytes(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_length_codepoints(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_string_value(_: jv) -> *const libc::c_char;
    #[no_mangle]
    fn jv_string_slice(j: jv, start: libc::c_int, end: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_string_fmt(_: *const libc::c_char, _: ...) -> jv;
    #[no_mangle]
    fn jv_object() -> jv;
    #[no_mangle]
    fn jv_object_get(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_set(object: jv, key: jv, value: jv) -> jv;
    #[no_mangle]
    fn jv_object_delete(object: jv, key: jv) -> jv;
    #[no_mangle]
    fn jv_object_length(object: jv) -> libc::c_int;
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
    fn jv_array_set(_: jv, _: libc::c_int, _: jv) -> jv;
    #[no_mangle]
    fn jv_bool(_: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_number(_: libc::c_double) -> jv;
    #[no_mangle]
    fn jv_number_value(_: jv) -> libc::c_double;
    #[no_mangle]
    fn jv_is_integer(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jv_array() -> jv;
    #[no_mangle]
    fn jv_array_sized(_: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_get(_: jv, _: libc::c_int) -> jv;
    #[no_mangle]
    fn jv_array_length(_: jv) -> libc::c_int;
    #[no_mangle]
    fn jvp_number_cmp(_: jv, _: jv) -> libc::c_int;
    #[no_mangle]
    fn jvp_number_is_nan(_: jv) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
pub struct sort_entry {
    pub object: jv,
    pub key: jv,
    pub index: libc::c_int,
}
unsafe extern "C" fn jv_is_valid(mut x: jv) -> libc::c_int {
    return (jv_get_kind(x) as libc::c_uint !=
                JV_KIND_INVALID as libc::c_int as libc::c_uint) as
               libc::c_int;
}
// making this static verbose function here
// until we introduce a less confusing naming scheme
// of jv_* API with regards to the memory management
unsafe extern "C" fn jv_number_get_value_and_consume(mut number: jv)
 -> libc::c_double {
    let mut value: libc::c_double = jv_number_value(number);
    jv_free(number);
    return value;
}
unsafe extern "C" fn parse_slice(mut j: jv, mut slice: jv,
                                 mut pstart: *mut libc::c_int,
                                 mut pend: *mut libc::c_int) -> libc::c_int {
    // Array slices
    let mut start_jv: jv =
        jv_object_get(jv_copy(slice),
                      jv_string(b"start\x00" as *const u8 as
                                    *const libc::c_char));
    let mut end_jv: jv =
        jv_object_get(slice,
                      jv_string(b"end\x00" as *const u8 as
                                    *const libc::c_char));
    if jv_get_kind(start_jv) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint {
        jv_free(start_jv);
        start_jv = jv_number(0 as libc::c_int as libc::c_double)
    }
    let mut len: libc::c_int = 0;
    if jv_get_kind(j) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        len = jv_array_length(j)
    } else if jv_get_kind(j) as libc::c_uint ==
                  JV_KIND_STRING as libc::c_int as libc::c_uint {
        len = jv_string_length_codepoints(j)
    } else { jv_free(j); return 0 as libc::c_int }
    if jv_get_kind(end_jv) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint {
        jv_free(end_jv);
        end_jv = jv_number(len as libc::c_double)
    }
    if jv_get_kind(start_jv) as libc::c_uint !=
           JV_KIND_NUMBER as libc::c_int as libc::c_uint ||
           jv_get_kind(end_jv) as libc::c_uint !=
               JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        jv_free(start_jv);
        jv_free(end_jv);
        return 0 as libc::c_int
    } else {
        let mut dstart: libc::c_double = jv_number_value(start_jv);
        let mut dend: libc::c_double = jv_number_value(end_jv);
        jv_free(start_jv);
        jv_free(end_jv);
        if dstart < 0 as libc::c_int as libc::c_double {
            dstart += len as libc::c_double
        }
        if dend < 0 as libc::c_int as libc::c_double {
            dend += len as libc::c_double
        }
        if dstart < 0 as libc::c_int as libc::c_double {
            dstart = 0 as libc::c_int as libc::c_double
        }
        if dstart > len as libc::c_double { dstart = len as libc::c_double }
        let mut start: libc::c_int = dstart as libc::c_int;
        let mut end: libc::c_int =
            if dend > len as libc::c_double {
                len
            } else { dend as libc::c_int };
        // Ends are exclusive but e.g. 1 < 1.5 so :1.5 should be :2 not :1
        if (end as libc::c_double) < dend { end += 1 as libc::c_int }
        if end > len { end = len }
        if end < start { end = start }
        if 0 as libc::c_int <= start && start <= end && end <= len {
        } else {
            __assert_fail(b"0 <= start && start <= end && end <= len\x00" as
                              *const u8 as *const libc::c_char,
                          b"src/jv_aux.c\x00" as *const u8 as
                              *const libc::c_char,
                          59 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"int parse_slice(jv, jv, int *, int *)\x00")).as_ptr());
        };
        *pstart = start;
        *pend = end;
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_get(mut t: jv, mut k: jv) -> jv {
    let mut v: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if jv_get_kind(t) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint &&
           jv_get_kind(k) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint {
        v = jv_object_get(t, k);
        if jv_is_valid(v) == 0 { jv_free(v); v = jv_null() }
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
                  jv_get_kind(k) as libc::c_uint ==
                      JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        if jv_is_integer(k) != 0 {
            let mut idx: libc::c_int = jv_number_value(k) as libc::c_int;
            if idx < 0 as libc::c_int { idx += jv_array_length(jv_copy(t)) }
            v = jv_array_get(t, idx);
            if jv_is_valid(v) == 0 { jv_free(v); v = jv_null() }
            jv_free(k);
        } else { jv_free(t); jv_free(k); v = jv_null() }
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
                  jv_get_kind(k) as libc::c_uint ==
                      JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        let mut start: libc::c_int = 0;
        let mut end: libc::c_int = 0;
        if parse_slice(jv_copy(t), k, &mut start, &mut end) != 0 {
            v = jv_array_slice(t, start, end)
        } else {
            jv_free(t);
            v =
                jv_invalid_with_msg(jv_string_fmt(b"Start and end indices of an array slice must be numbers\x00"
                                                      as *const u8 as
                                                      *const libc::c_char))
        }
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_STRING as libc::c_int as libc::c_uint &&
                  jv_get_kind(k) as libc::c_uint ==
                      JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        let mut start_0: libc::c_int = 0;
        let mut end_0: libc::c_int = 0;
        if parse_slice(jv_copy(t), k, &mut start_0, &mut end_0) != 0 {
            v = jv_string_slice(t, start_0, end_0)
        } else {
            v =
                jv_invalid_with_msg(jv_string_fmt(b"Start and end indices of an string slice must be numbers\x00"
                                                      as *const u8 as
                                                      *const libc::c_char));
            jv_free(t);
        }
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
                  jv_get_kind(k) as libc::c_uint ==
                      JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        v = jv_array_indexes(t, k)
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_NULL as libc::c_int as libc::c_uint &&
                  (jv_get_kind(k) as libc::c_uint ==
                       JV_KIND_STRING as libc::c_int as libc::c_uint ||
                       jv_get_kind(k) as libc::c_uint ==
                           JV_KIND_NUMBER as libc::c_int as libc::c_uint ||
                       jv_get_kind(k) as libc::c_uint ==
                           JV_KIND_OBJECT as libc::c_int as libc::c_uint) {
        jv_free(t);
        jv_free(k);
        v = jv_null()
    } else {
        /*
     * If k is a short string it's probably from a jq .foo expression or
     * similar, in which case putting it in the invalid msg may help the
     * user.  The length 30 is arbitrary.
     */
        if jv_get_kind(k) as libc::c_uint ==
               JV_KIND_STRING as libc::c_int as libc::c_uint &&
               jv_string_length_bytes(jv_copy(k)) < 30 as libc::c_int {
            v =
                jv_invalid_with_msg(jv_string_fmt(b"Cannot index %s with string \"%s\"\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  jv_kind_name(jv_get_kind(t)),
                                                  jv_string_value(k)))
        } else {
            v =
                jv_invalid_with_msg(jv_string_fmt(b"Cannot index %s with %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  jv_kind_name(jv_get_kind(t)),
                                                  jv_kind_name(jv_get_kind(k))))
        }
        jv_free(t);
        jv_free(k);
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn jv_set(mut t: jv, mut k: jv, mut v: jv) -> jv {
    if jv_is_valid(v) == 0 { jv_free(t); jv_free(k); return v }
    let mut isnull: libc::c_int =
        (jv_get_kind(t) as libc::c_uint ==
             JV_KIND_NULL as libc::c_int as libc::c_uint) as libc::c_int;
    if jv_get_kind(k) as libc::c_uint ==
           JV_KIND_STRING as libc::c_int as libc::c_uint &&
           (jv_get_kind(t) as libc::c_uint ==
                JV_KIND_OBJECT as libc::c_int as libc::c_uint || isnull != 0)
       {
        if isnull != 0 { t = jv_object() }
        t = jv_object_set(t, k, v)
    } else if jv_get_kind(k) as libc::c_uint ==
                  JV_KIND_NUMBER as libc::c_int as libc::c_uint &&
                  (jv_get_kind(t) as libc::c_uint ==
                       JV_KIND_ARRAY as libc::c_int as libc::c_uint ||
                       isnull != 0) {
        if isnull != 0 { t = jv_array() }
        t = jv_array_set(t, jv_number_value(k) as libc::c_int, v);
        jv_free(k);
    } else if jv_get_kind(k) as libc::c_uint ==
                  JV_KIND_OBJECT as libc::c_int as libc::c_uint &&
                  (jv_get_kind(t) as libc::c_uint ==
                       JV_KIND_ARRAY as libc::c_int as libc::c_uint ||
                       isnull != 0) {
        if isnull != 0 { t = jv_array() }
        let mut start: libc::c_int = 0;
        let mut end: libc::c_int = 0;
        if parse_slice(jv_copy(t), k, &mut start, &mut end) != 0 {
            if jv_get_kind(v) as libc::c_uint ==
                   JV_KIND_ARRAY as libc::c_int as libc::c_uint {
                let mut array_len: libc::c_int = jv_array_length(jv_copy(t));
                if 0 as libc::c_int <= start && start <= end &&
                       end <= array_len {
                } else {
                    __assert_fail(b"0 <= start && start <= end && end <= array_len\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"src/jv_aux.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  159 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 22],
                                                            &[libc::c_char; 22]>(b"jv jv_set(jv, jv, jv)\x00")).as_ptr());
                };
                let mut slice_len: libc::c_int = end - start;
                let mut insert_len: libc::c_int = jv_array_length(jv_copy(v));
                if slice_len < insert_len {
                    // array is growing
                    let mut shift: libc::c_int = insert_len - slice_len;
                    let mut i: libc::c_int = array_len - 1 as libc::c_int;
                    while i >= end {
                        t =
                            jv_array_set(t, i + shift,
                                         jv_array_get(jv_copy(t), i));
                        i -= 1
                    }
                } else if slice_len > insert_len {
                    // array is shrinking
                    let mut shift_0: libc::c_int = slice_len - insert_len;
                    let mut i_0: libc::c_int = end;
                    while i_0 < array_len {
                        t =
                            jv_array_set(t, i_0 - shift_0,
                                         jv_array_get(jv_copy(t), i_0));
                        i_0 += 1
                    }
                    t =
                        jv_array_slice(t, 0 as libc::c_int,
                                       array_len - shift_0)
                }
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while i_1 < insert_len {
                    t =
                        jv_array_set(t, start + i_1,
                                     jv_array_get(jv_copy(v), i_1));
                    i_1 += 1
                }
                jv_free(v);
            } else {
                jv_free(t);
                jv_free(v);
                t =
                    jv_invalid_with_msg(jv_string_fmt(b"A slice of an array can only be assigned another array\x00"
                                                          as *const u8 as
                                                          *const libc::c_char))
            }
        } else {
            jv_free(t);
            jv_free(v);
            t =
                jv_invalid_with_msg(jv_string_fmt(b"Start and end indices of an array slice must be numbers\x00"
                                                      as *const u8 as
                                                      *const libc::c_char))
        }
    } else {
        let mut err: jv =
            jv_invalid_with_msg(jv_string_fmt(b"Cannot update field at %s index of %s\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              jv_kind_name(jv_get_kind(k)),
                                              jv_kind_name(jv_get_kind(t))));
        jv_free(t);
        jv_free(k);
        jv_free(v);
        t = err
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn jv_has(mut t: jv, mut k: jv) -> jv {
    if jv_is_valid(t) != 0 {
    } else {
        __assert_fail(b"jv_is_valid(t)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      203 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"jv jv_has(jv, jv)\x00")).as_ptr());
    };
    if jv_is_valid(k) != 0 {
    } else {
        __assert_fail(b"jv_is_valid(k)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      204 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"jv jv_has(jv, jv)\x00")).as_ptr());
    };
    let mut ret: jv =
        jv{kind_flags: 0,
           pad_: 0,
           offset: 0,
           size: 0,
           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
    if jv_get_kind(t) as libc::c_uint ==
           JV_KIND_NULL as libc::c_int as libc::c_uint {
        jv_free(t);
        jv_free(k);
        ret = jv_false()
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_OBJECT as libc::c_int as libc::c_uint &&
                  jv_get_kind(k) as libc::c_uint ==
                      JV_KIND_STRING as libc::c_int as libc::c_uint {
        let mut elem: jv = jv_object_get(t, k);
        ret = jv_bool(jv_is_valid(elem));
        jv_free(elem);
    } else if jv_get_kind(t) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint &&
                  jv_get_kind(k) as libc::c_uint ==
                      JV_KIND_NUMBER as libc::c_int as libc::c_uint {
        let mut elem_0: jv =
            jv_array_get(t, jv_number_value(k) as libc::c_int);
        ret = jv_bool(jv_is_valid(elem_0));
        jv_free(k);
        jv_free(elem_0);
    } else {
        ret =
            jv_invalid_with_msg(jv_string_fmt(b"Cannot check whether %s has a %s key\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              jv_kind_name(jv_get_kind(t)),
                                              jv_kind_name(jv_get_kind(k))));
        jv_free(t);
        jv_free(k);
    }
    return ret;
}
// assumes keys is a sorted array
unsafe extern "C" fn jv_dels(mut t: jv, mut keys: jv) -> jv {
    let mut neg_idx: libc::c_int = 0;
    let mut nonneg_idx: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut current_block: u64;
    if jv_get_kind(keys) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(keys) == JV_KIND_ARRAY\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      233 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"jv jv_dels(jv, jv)\x00")).as_ptr());
    };
    if jv_is_valid(t) != 0 {
    } else {
        __assert_fail(b"jv_is_valid(t)\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      234 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"jv jv_dels(jv, jv)\x00")).as_ptr());
    };
    if !(jv_get_kind(t) as libc::c_uint ==
             JV_KIND_NULL as libc::c_int as libc::c_uint ||
             jv_array_length(jv_copy(keys)) == 0 as libc::c_int) {
        if jv_get_kind(t) as libc::c_uint ==
               JV_KIND_ARRAY as libc::c_int as libc::c_uint {
            // extract slices, they must be handled differently
            let mut neg_keys: jv = jv_array();
            let mut nonneg_keys: jv = jv_array();
            let mut new_array: jv = jv_array();
            let mut starts: jv = jv_array();
            let mut ends: jv = jv_array();
            let mut jv_len__: libc::c_int = jv_array_length(jv_copy(keys));
            let mut i: libc::c_int = 0 as libc::c_int;
            let mut jv_j__: libc::c_int = 1 as libc::c_int;
            's_35:
                loop  {
                    if !(jv_j__ != 0) {
                        current_block = 4761528863920922185;
                        break ;
                    }
                    let mut key: jv =
                        jv{kind_flags: 0,
                           pad_: 0,
                           offset: 0,
                           size: 0,
                           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                    while if i < jv_len__ {
                              key = jv_array_get(jv_copy(keys), i);
                              1 as libc::c_int
                          } else { 0 as libc::c_int } != 0 {
                        if jv_get_kind(key) as libc::c_uint ==
                               JV_KIND_NUMBER as libc::c_int as libc::c_uint {
                            if jv_number_value(key) <
                                   0 as libc::c_int as libc::c_double {
                                neg_keys = jv_array_append(neg_keys, key)
                            } else {
                                nonneg_keys =
                                    jv_array_append(nonneg_keys, key)
                            }
                        } else if jv_get_kind(key) as libc::c_uint ==
                                      JV_KIND_OBJECT as libc::c_int as
                                          libc::c_uint {
                            let mut start: libc::c_int = 0;
                            let mut end: libc::c_int = 0;
                            if parse_slice(jv_copy(t), key, &mut start,
                                           &mut end) != 0 {
                                starts =
                                    jv_array_append(starts,
                                                    jv_number(start as
                                                                  libc::c_double));
                                ends =
                                    jv_array_append(ends,
                                                    jv_number(end as
                                                                  libc::c_double))
                            } else {
                                jv_free(new_array);
                                jv_free(key);
                                new_array =
                                    jv_invalid_with_msg(jv_string_fmt(b"Start and end indices of an array slice must be numbers\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char));
                                current_block = 4413034795906836112;
                                break 's_35 ;
                            }
                        } else {
                            jv_free(new_array);
                            new_array =
                                jv_invalid_with_msg(jv_string_fmt(b"Cannot delete %s element of array\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  jv_kind_name(jv_get_kind(key))));
                            jv_free(key);
                            current_block = 4413034795906836112;
                            break 's_35 ;
                        }
                        i += 1
                    }
                    jv_j__ = 0 as libc::c_int
                }
            match current_block {
                4761528863920922185 => {
                    neg_idx = 0 as libc::c_int;
                    nonneg_idx = 0 as libc::c_int;
                    len = jv_array_length(jv_copy(t));
                    let mut jv_len___0: libc::c_int =
                        jv_array_length(jv_copy(t));
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    let mut jv_j___0: libc::c_int = 1 as libc::c_int;
                    while jv_j___0 != 0 {
                        let mut elem: jv =
                            jv{kind_flags: 0,
                               pad_: 0,
                               offset: 0,
                               size: 0,
                               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                        while if i_0 < jv_len___0 {
                                  elem = jv_array_get(jv_copy(t), i_0);
                                  1 as libc::c_int
                              } else { 0 as libc::c_int } != 0 {
                            let mut del: libc::c_int = 0 as libc::c_int;
                            while neg_idx < jv_array_length(jv_copy(neg_keys))
                                  {
                                let mut delidx: libc::c_int =
                                    len +
                                        jv_number_get_value_and_consume(jv_array_get(jv_copy(neg_keys),
                                                                                     neg_idx))
                                            as libc::c_int;
                                if i_0 == delidx { del = 1 as libc::c_int }
                                if i_0 < delidx { break ; }
                                neg_idx += 1
                            }
                            while nonneg_idx <
                                      jv_array_length(jv_copy(nonneg_keys)) {
                                let mut delidx_0: libc::c_int =
                                    jv_number_get_value_and_consume(jv_array_get(jv_copy(nonneg_keys),
                                                                                 nonneg_idx))
                                        as libc::c_int;
                                if i_0 == delidx_0 { del = 1 as libc::c_int }
                                if i_0 < delidx_0 { break ; }
                                nonneg_idx += 1
                            }
                            let mut sidx: libc::c_int = 0 as libc::c_int;
                            while del == 0 &&
                                      sidx < jv_array_length(jv_copy(starts))
                                  {
                                if jv_number_get_value_and_consume(jv_array_get(jv_copy(starts),
                                                                                sidx))
                                       as libc::c_int <= i_0 &&
                                       i_0 <
                                           jv_number_get_value_and_consume(jv_array_get(jv_copy(ends),
                                                                                        sidx))
                                               as libc::c_int {
                                    del = 1 as libc::c_int
                                }
                                sidx += 1
                            }
                            if del == 0 {
                                new_array = jv_array_append(new_array, elem)
                            } else { jv_free(elem); }
                            i_0 += 1
                        }
                        jv_j___0 = 0 as libc::c_int
                    }
                }
                _ => { }
            }
            jv_free(neg_keys);
            jv_free(nonneg_keys);
            jv_free(starts);
            jv_free(ends);
            jv_free(t);
            t = new_array
        } else if jv_get_kind(t) as libc::c_uint ==
                      JV_KIND_OBJECT as libc::c_int as libc::c_uint {
            let mut jv_len___1: libc::c_int = jv_array_length(jv_copy(keys));
            let mut i_1: libc::c_int = 0 as libc::c_int;
            let mut jv_j___1: libc::c_int = 1 as libc::c_int;
            while jv_j___1 != 0 {
                let mut k: jv =
                    jv{kind_flags: 0,
                       pad_: 0,
                       offset: 0,
                       size: 0,
                       u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                while if i_1 < jv_len___1 {
                          k = jv_array_get(jv_copy(keys), i_1);
                          1 as libc::c_int
                      } else { 0 as libc::c_int } != 0 {
                    if jv_get_kind(k) as libc::c_uint !=
                           JV_KIND_STRING as libc::c_int as libc::c_uint {
                        jv_free(t);
                        t =
                            jv_invalid_with_msg(jv_string_fmt(b"Cannot delete %s field of object\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              jv_kind_name(jv_get_kind(k))));
                        jv_free(k);
                        break ;
                    } else { t = jv_object_delete(t, k); i_1 += 1 }
                }
                jv_j___1 = 0 as libc::c_int
            }
        } else {
            let mut err: jv =
                jv_invalid_with_msg(jv_string_fmt(b"Cannot delete fields from %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  jv_kind_name(jv_get_kind(t))));
            jv_free(t);
            t = err
        }
    }
    // no change
    jv_free(keys);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn jv_setpath(mut root: jv, mut path: jv, mut value: jv)
 -> jv {
    if jv_get_kind(path) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        jv_free(value);
        jv_free(root);
        jv_free(path);
        return jv_invalid_with_msg(jv_string(b"Path must be specified as an array\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    if jv_is_valid(root) == 0 { jv_free(value); jv_free(path); return root }
    if jv_array_length(jv_copy(path)) == 0 as libc::c_int {
        jv_free(path);
        jv_free(root);
        return value
    }
    let mut pathcurr: jv = jv_array_get(jv_copy(path), 0 as libc::c_int);
    let mut pathrest: jv =
        jv_array_slice(path, 1 as libc::c_int,
                       jv_array_length(jv_copy(path)));
    return jv_set(root, pathcurr,
                  jv_setpath(jv_get(jv_copy(root), jv_copy(pathcurr)),
                             pathrest, value));
}
#[no_mangle]
pub unsafe extern "C" fn jv_getpath(mut root: jv, mut path: jv) -> jv {
    if jv_get_kind(path) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        jv_free(root);
        jv_free(path);
        return jv_invalid_with_msg(jv_string(b"Path must be specified as an array\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    if jv_is_valid(root) == 0 { jv_free(path); return root }
    if jv_array_length(jv_copy(path)) == 0 as libc::c_int {
        jv_free(path);
        return root
    }
    let mut pathcurr: jv = jv_array_get(jv_copy(path), 0 as libc::c_int);
    let mut pathrest: jv =
        jv_array_slice(path, 1 as libc::c_int,
                       jv_array_length(jv_copy(path)));
    return jv_getpath(jv_get(root, pathcurr), pathrest);
}
// assumes paths is a sorted array of arrays
unsafe extern "C" fn delpaths_sorted(mut object: jv, mut paths: jv,
                                     mut start: libc::c_int) -> jv {
    let mut delkeys: jv = jv_array();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < jv_array_length(jv_copy(paths)) {
        let mut j: libc::c_int = i;
        if jv_array_length(jv_array_get(jv_copy(paths), i)) > start {
        } else {
            __assert_fail(b"jv_array_length(jv_array_get(jv_copy(paths), i)) > start\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/jv_aux.c\x00" as *const u8 as
                              *const libc::c_char,
                          382 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 32],
                                                    &[libc::c_char; 32]>(b"jv delpaths_sorted(jv, jv, int)\x00")).as_ptr());
        };
        let mut delkey: libc::c_int =
            (jv_array_length(jv_array_get(jv_copy(paths), i)) ==
                 start + 1 as libc::c_int) as libc::c_int;
        let mut key: jv =
            jv_array_get(jv_array_get(jv_copy(paths), i), start);
        while j < jv_array_length(jv_copy(paths)) &&
                  jv_equal(jv_copy(key),
                           jv_array_get(jv_array_get(jv_copy(paths), j),
                                        start)) != 0 {
            j += 1
        }
        // if i <= entry < j, then entry starts with key
        if delkey != 0 {
            // deleting this entire key, we don't care about any more specific deletions
            delkeys = jv_array_append(delkeys, key)
        } else {
            // deleting certain sub-parts of this key
            let mut subobject: jv = jv_get(jv_copy(object), jv_copy(key));
            if jv_is_valid(subobject) == 0 {
                jv_free(key);
                jv_free(object);
                object = subobject;
                break ;
            } else {
                if jv_get_kind(subobject) as libc::c_uint ==
                       JV_KIND_NULL as libc::c_int as libc::c_uint {
                    jv_free(key);
                    jv_free(subobject);
                } else {
                    let mut newsubobject: jv =
                        delpaths_sorted(subobject,
                                        jv_array_slice(jv_copy(paths), i, j),
                                        start + 1 as libc::c_int);
                    if jv_is_valid(newsubobject) == 0 {
                        jv_free(key);
                        jv_free(object);
                        object = newsubobject;
                        break ;
                    } else { object = jv_set(object, key, newsubobject) }
                }
                if jv_is_valid(object) == 0 { break ; }
            }
        }
        i = j
    }
    jv_free(paths);
    if jv_is_valid(object) != 0 {
        object = jv_dels(object, delkeys)
    } else { jv_free(delkeys); }
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn jv_delpaths(mut object: jv, mut paths: jv) -> jv {
    if jv_get_kind(paths) as libc::c_uint !=
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        jv_free(object);
        jv_free(paths);
        return jv_invalid_with_msg(jv_string(b"Paths must be specified as an array\x00"
                                                 as *const u8 as
                                                 *const libc::c_char))
    }
    paths = jv_sort(paths, jv_copy(paths));
    let mut jv_len__: libc::c_int = jv_array_length(jv_copy(paths));
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
        let mut elem: jv =
            jv{kind_flags: 0,
               pad_: 0,
               offset: 0,
               size: 0,
               u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
        while if i < jv_len__ {
                  elem = jv_array_get(jv_copy(paths), i);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            if jv_get_kind(elem) as libc::c_uint !=
                   JV_KIND_ARRAY as libc::c_int as libc::c_uint {
                jv_free(object);
                jv_free(paths);
                let mut err: jv =
                    jv_invalid_with_msg(jv_string_fmt(b"Path must be specified as array, not %s\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      jv_kind_name(jv_get_kind(elem))));
                jv_free(elem);
                return err
            }
            jv_free(elem);
            i += 1
        }
        jv_j__ = 0 as libc::c_int
    }
    if jv_array_length(jv_copy(paths)) == 0 as libc::c_int {
        // nothing is being deleted
        jv_free(paths);
        return object
    }
    if jv_array_length(jv_array_get(jv_copy(paths), 0 as libc::c_int)) ==
           0 as libc::c_int {
        // everything is being deleted
        jv_free(paths);
        jv_free(object);
        return jv_null()
    }
    return delpaths_sorted(object, paths, 0 as libc::c_int);
}
unsafe extern "C" fn string_cmp(mut pa: *const libc::c_void,
                                mut pb: *const libc::c_void) -> libc::c_int {
    let mut a: *const jv = pa as *const jv;
    let mut b: *const jv = pb as *const jv;
    let mut lena: libc::c_int = jv_string_length_bytes(jv_copy(*a));
    let mut lenb: libc::c_int = jv_string_length_bytes(jv_copy(*b));
    let mut minlen: libc::c_int = if lena < lenb { lena } else { lenb };
    let mut r: libc::c_int =
        memcmp(jv_string_value(*a) as *const libc::c_void,
               jv_string_value(*b) as *const libc::c_void,
               minlen as libc::c_ulong);
    if r == 0 as libc::c_int { r = lena - lenb }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn jv_keys_unsorted(mut x: jv) -> jv {
    if jv_get_kind(x) as libc::c_uint !=
           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        return jv_keys(x)
    }
    let mut answer: jv = jv_array_sized(jv_object_length(jv_copy(x)));
    let mut jv_i__: libc::c_int = jv_object_iter(x);
    let mut jv_j__: libc::c_int = 1 as libc::c_int;
    while jv_j__ != 0 {
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
        while if jv_object_iter_valid(x, jv_i__) != 0 {
                  key = jv_object_iter_key(x, jv_i__);
                  value = jv_object_iter_value(x, jv_i__);
                  1 as libc::c_int
              } else { 0 as libc::c_int } != 0 {
            answer = jv_array_append(answer, key);
            jv_free(value);
            jv_i__ = jv_object_iter_next(x, jv_i__)
        }
        jv_j__ = 0 as libc::c_int
    }
    jv_free(x);
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn jv_keys(mut x: jv) -> jv {
    if jv_get_kind(x) as libc::c_uint ==
           JV_KIND_OBJECT as libc::c_int as libc::c_uint {
        let mut nkeys: libc::c_int = jv_object_length(jv_copy(x));
        let mut keys: *mut jv =
            jv_mem_calloc(::std::mem::size_of::<jv>() as libc::c_ulong,
                          nkeys as size_t) as *mut jv;
        let mut kidx: libc::c_int = 0 as libc::c_int;
        let mut jv_i__: libc::c_int = jv_object_iter(x);
        let mut jv_j__: libc::c_int = 1 as libc::c_int;
        while jv_j__ != 0 {
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
            while if jv_object_iter_valid(x, jv_i__) != 0 {
                      key = jv_object_iter_key(x, jv_i__);
                      value = jv_object_iter_value(x, jv_i__);
                      1 as libc::c_int
                  } else { 0 as libc::c_int } != 0 {
                let fresh0 = kidx;
                kidx = kidx + 1;
                *keys.offset(fresh0 as isize) = key;
                jv_free(value);
                jv_i__ = jv_object_iter_next(x, jv_i__)
            }
            jv_j__ = 0 as libc::c_int
        }
        qsort(keys as *mut libc::c_void, nkeys as size_t,
              ::std::mem::size_of::<jv>() as libc::c_ulong,
              Some(string_cmp as
                       unsafe extern "C" fn(_: *const libc::c_void,
                                            _: *const libc::c_void)
                           -> libc::c_int));
        let mut answer: jv = jv_array_sized(nkeys);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < nkeys {
            answer = jv_array_append(answer, *keys.offset(i as isize));
            i += 1
        }
        jv_mem_free(keys as *mut libc::c_void);
        jv_free(x);
        return answer
    } else if jv_get_kind(x) as libc::c_uint ==
                  JV_KIND_ARRAY as libc::c_int as libc::c_uint {
        let mut n: libc::c_int = jv_array_length(x);
        let mut answer_0: jv = jv_array();
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < n {
            answer_0 =
                jv_array_set(answer_0, i_0, jv_number(i_0 as libc::c_double));
            i_0 += 1
        }
        return answer_0
    } else {
        if 0 as libc::c_int != 0 &&
               !(b"jv_keys passed something neither object nor array\x00" as
                     *const u8 as *const libc::c_char).is_null() {
        } else {
            __assert_fail(b"0 && \"jv_keys passed something neither object nor array\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/jv_aux.c\x00" as *const u8 as
                              *const libc::c_char,
                          506 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"jv jv_keys(jv)\x00")).as_ptr());
        };
        return jv_invalid()
    };
}
#[no_mangle]
pub unsafe extern "C" fn jv_cmp(mut a: jv, mut b: jv) -> libc::c_int {
    if jv_get_kind(a) as libc::c_uint != jv_get_kind(b) as libc::c_uint {
        let mut r: libc::c_int =
            jv_get_kind(a) as libc::c_int - jv_get_kind(b) as libc::c_int;
        jv_free(a);
        jv_free(b);
        return r
    }
    let mut r_0: libc::c_int = 0 as libc::c_int;
    let mut current_block_26: u64;
    match jv_get_kind(a) as libc::c_uint {
        1 | 2 | 3 => { current_block_26 = 15611060060028977054; }
        4 => {
            if jvp_number_is_nan(a) != 0 {
                r_0 = jv_cmp(jv_null(), jv_copy(b))
            } else if jvp_number_is_nan(b) != 0 {
                r_0 = jv_cmp(jv_copy(a), jv_null())
            } else { r_0 = jvp_number_cmp(a, b) }
            current_block_26 = 15004371738079956865;
        }
        5 => {
            r_0 =
                string_cmp(&mut a as *mut jv as *const libc::c_void,
                           &mut b as *mut jv as *const libc::c_void);
            current_block_26 = 15004371738079956865;
        }
        6 => {
            // Lexical ordering of arrays
            let mut i: libc::c_int = 0 as libc::c_int; //suddenly, logic
            while r_0 == 0 as libc::c_int {
                let mut a_done: libc::c_int =
                    (i >= jv_array_length(jv_copy(a))) as libc::c_int;
                let mut b_done: libc::c_int =
                    (i >= jv_array_length(jv_copy(b))) as libc::c_int;
                if a_done != 0 || b_done != 0 {
                    r_0 = b_done - a_done;
                    break ;
                } else {
                    let mut xa: jv = jv_array_get(jv_copy(a), i);
                    let mut xb: jv = jv_array_get(jv_copy(b), i);
                    r_0 = jv_cmp(xa, xb);
                    i += 1
                }
            }
            current_block_26 = 15004371738079956865;
        }
        7 => {
            let mut keys_a: jv = jv_keys(jv_copy(a));
            let mut keys_b: jv = jv_keys(jv_copy(b));
            r_0 = jv_cmp(jv_copy(keys_a), keys_b);
            if r_0 == 0 as libc::c_int {
                let mut jv_len__: libc::c_int =
                    jv_array_length(jv_copy(keys_a));
                let mut i_0: libc::c_int = 0 as libc::c_int;
                let mut jv_j__: libc::c_int = 1 as libc::c_int;
                while jv_j__ != 0 {
                    let mut key: jv =
                        jv{kind_flags: 0,
                           pad_: 0,
                           offset: 0,
                           size: 0,
                           u: C2RustUnnamed{ptr: 0 as *mut jv_refcnt,},};
                    while if i_0 < jv_len__ {
                              key = jv_array_get(jv_copy(keys_a), i_0);
                              1 as libc::c_int
                          } else { 0 as libc::c_int } != 0 {
                        let mut xa_0: jv =
                            jv_object_get(jv_copy(a), jv_copy(key));
                        let mut xb_0: jv = jv_object_get(jv_copy(b), key);
                        r_0 = jv_cmp(xa_0, xb_0);
                        if r_0 != 0 { break ; }
                        i_0 += 1
                    }
                    jv_j__ = 0 as libc::c_int
                }
            }
            jv_free(keys_a);
            current_block_26 = 15004371738079956865;
        }
        _ => {
            if 0 as libc::c_int != 0 &&
                   !(b"invalid kind passed to jv_cmp\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"0 && \"invalid kind passed to jv_cmp\"\x00" as
                                  *const u8 as *const libc::c_char,
                              b"src/jv_aux.c\x00" as *const u8 as
                                  *const libc::c_char,
                              521 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"int jv_cmp(jv, jv)\x00")).as_ptr());
            };
            current_block_26 = 15611060060028977054;
        }
    }
    match current_block_26 {
        15611060060028977054 => {
            // there's only one of each of these values
            r_0 = 0 as libc::c_int
        }
        _ => { }
    }
    jv_free(a);
    jv_free(b);
    return r_0;
}
unsafe extern "C" fn sort_cmp(mut pa: *const libc::c_void,
                              mut pb: *const libc::c_void) -> libc::c_int {
    let mut a: *const sort_entry = pa as *const sort_entry;
    let mut b: *const sort_entry = pb as *const sort_entry;
    let mut r: libc::c_int = jv_cmp(jv_copy((*a).key), jv_copy((*b).key));
    // comparing by index if r == 0 makes the sort stable
    return if r != 0 { r } else { ((*a).index) - (*b).index };
}
unsafe extern "C" fn sort_items(mut objects: jv, mut keys: jv)
 -> *mut sort_entry {
    if jv_get_kind(objects) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(objects) == JV_KIND_ARRAY\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      601 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"struct sort_entry *sort_items(jv, jv)\x00")).as_ptr());
    };
    if jv_get_kind(keys) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(keys) == JV_KIND_ARRAY\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      602 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"struct sort_entry *sort_items(jv, jv)\x00")).as_ptr());
    };
    if jv_array_length(jv_copy(objects)) == jv_array_length(jv_copy(keys)) {
    } else {
        __assert_fail(b"jv_array_length(jv_copy(objects)) == jv_array_length(jv_copy(keys))\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      603 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"struct sort_entry *sort_items(jv, jv)\x00")).as_ptr());
    };
    let mut n: libc::c_int = jv_array_length(jv_copy(objects));
    let mut entries: *mut sort_entry =
        jv_mem_calloc(::std::mem::size_of::<sort_entry>() as libc::c_ulong,
                      n as size_t) as *mut sort_entry;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        (*entries.offset(i as isize)).object =
            jv_array_get(jv_copy(objects), i);
        (*entries.offset(i as isize)).key = jv_array_get(jv_copy(keys), i);
        (*entries.offset(i as isize)).index = i;
        i += 1
    }
    jv_free(objects);
    jv_free(keys);
    qsort(entries as *mut libc::c_void, n as size_t,
          ::std::mem::size_of::<sort_entry>() as libc::c_ulong,
          Some(sort_cmp as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    return entries;
}
#[no_mangle]
pub unsafe extern "C" fn jv_sort(mut objects: jv, mut keys: jv) -> jv {
    if jv_get_kind(objects) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(objects) == JV_KIND_ARRAY\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      618 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"jv jv_sort(jv, jv)\x00")).as_ptr());
    };
    if jv_get_kind(keys) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(keys) == JV_KIND_ARRAY\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      619 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"jv jv_sort(jv, jv)\x00")).as_ptr());
    };
    if jv_array_length(jv_copy(objects)) == jv_array_length(jv_copy(keys)) {
    } else {
        __assert_fail(b"jv_array_length(jv_copy(objects)) == jv_array_length(jv_copy(keys))\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      620 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"jv jv_sort(jv, jv)\x00")).as_ptr());
    };
    let mut n: libc::c_int = jv_array_length(jv_copy(objects));
    let mut entries: *mut sort_entry = sort_items(objects, keys);
    let mut ret: jv = jv_array();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        jv_free((*entries.offset(i as isize)).key);
        ret = jv_array_set(ret, i, (*entries.offset(i as isize)).object);
        i += 1
    }
    jv_mem_free(entries as *mut libc::c_void);
    return ret;
}
/*object or array*/
/*object or array*/
#[no_mangle]
pub unsafe extern "C" fn jv_group(mut objects: jv, mut keys: jv) -> jv {
    if jv_get_kind(objects) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(objects) == JV_KIND_ARRAY\x00" as
                          *const u8 as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      633 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"jv jv_group(jv, jv)\x00")).as_ptr());
    };
    if jv_get_kind(keys) as libc::c_uint ==
           JV_KIND_ARRAY as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"jv_get_kind(keys) == JV_KIND_ARRAY\x00" as *const u8
                          as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      634 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"jv jv_group(jv, jv)\x00")).as_ptr());
    };
    if jv_array_length(jv_copy(objects)) == jv_array_length(jv_copy(keys)) {
    } else {
        __assert_fail(b"jv_array_length(jv_copy(objects)) == jv_array_length(jv_copy(keys))\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/jv_aux.c\x00" as *const u8 as *const libc::c_char,
                      635 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"jv jv_group(jv, jv)\x00")).as_ptr());
    };
    let mut n: libc::c_int = jv_array_length(jv_copy(objects));
    let mut entries: *mut sort_entry = sort_items(objects, keys);
    let mut ret: jv = jv_array();
    if n > 0 as libc::c_int {
        let mut curr_key: jv =
            (*entries.offset(0 as libc::c_int as isize)).key;
        let mut group: jv =
            jv_array_append(jv_array(),
                            (*entries.offset(0 as libc::c_int as
                                                 isize)).object);
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < n {
            if jv_equal(jv_copy(curr_key),
                        jv_copy((*entries.offset(i as isize)).key)) != 0 {
                jv_free((*entries.offset(i as isize)).key);
            } else {
                jv_free(curr_key);
                curr_key = (*entries.offset(i as isize)).key;
                ret = jv_array_append(ret, group);
                group = jv_array()
            }
            group =
                jv_array_append(group, (*entries.offset(i as isize)).object);
            i += 1
        }
        jv_free(curr_key);
        ret = jv_array_append(ret, group)
    }
    jv_mem_free(entries as *mut libc::c_void);
    return ret;
}
