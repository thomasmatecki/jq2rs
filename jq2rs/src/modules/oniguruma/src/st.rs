#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
/* This is a public domain general purpose hash table package written by Peter Moore @ UCB. */
/* @(#) st.h 5.1 89/12/14 */
pub type st_data_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_table {
    pub type_0: *mut st_hash_type,
    pub num_bins: libc::c_int,
    pub num_entries: libc::c_int,
    pub bins: *mut *mut st_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_table_entry {
    pub hash: libc::c_uint,
    pub key: st_data_t,
    pub record: st_data_t,
    pub next: *mut st_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_hash_type {
    pub compare: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub hash: Option<unsafe extern "C" fn() -> libc::c_int>,
}
pub type st_retval = libc::c_uint;
pub const ST_CHECK: st_retval = 3;
pub const ST_DELETE: st_retval = 2;
pub const ST_STOP: st_retval = 1;
pub const ST_CONTINUE: st_retval = 0;
static mut type_numhash: st_hash_type =
    unsafe {
        {
            let mut init =
                st_hash_type{compare:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         libc::c_long,
                                                                                     _:
                                                                                         libc::c_long)
                                                                    ->
                                                                        libc::c_int>,
                                                         Option<unsafe extern "C" fn()
                                                                    ->
                                                                        libc::c_int>>(Some(numcmp
                                                                                               as
                                                                                               unsafe extern "C" fn(_:
                                                                                                                        libc::c_long,
                                                                                                                    _:
                                                                                                                        libc::c_long)
                                                                                                   ->
                                                                                                       libc::c_int)),
                             hash:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         libc::c_long)
                                                                    ->
                                                                        libc::c_int>,
                                                         Option<unsafe extern "C" fn()
                                                                    ->
                                                                        libc::c_int>>(Some(numhash
                                                                                               as
                                                                                               unsafe extern "C" fn(_:
                                                                                                                        libc::c_long)
                                                                                                   ->
                                                                                                       libc::c_int)),};
            init
        }
    };
static mut type_strhash: st_hash_type =
    unsafe {
        {
            let mut init =
                st_hash_type{compare:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *const libc::c_char,
                                                                                     _:
                                                                                         *const libc::c_char)
                                                                    ->
                                                                        libc::c_int>,
                                                         Option<unsafe extern "C" fn()
                                                                    ->
                                                                        libc::c_int>>(Some(strcmp
                                                                                               as
                                                                                               unsafe extern "C" fn(_:
                                                                                                                        *const libc::c_char,
                                                                                                                    _:
                                                                                                                        *const libc::c_char)
                                                                                                   ->
                                                                                                       libc::c_int)),
                             hash:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *const libc::c_char)
                                                                    ->
                                                                        libc::c_int>,
                                                         Option<unsafe extern "C" fn()
                                                                    ->
                                                                        libc::c_int>>(Some(strhash
                                                                                               as
                                                                                               unsafe extern "C" fn(_:
                                                                                                                        *const libc::c_char)
                                                                                                   ->
                                                                                                       libc::c_int)),};
            init
        }
    };
/*
Table of prime numbers 2^n+a, 2<=n<=30.
*/
static mut primes: [libc::c_long; 29] =
    [(8 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (16 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (32 as libc::c_int + 5 as libc::c_int) as libc::c_long,
     (64 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (128 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (256 as libc::c_int + 27 as libc::c_int) as libc::c_long,
     (512 as libc::c_int + 9 as libc::c_int) as libc::c_long,
     (1024 as libc::c_int + 9 as libc::c_int) as libc::c_long,
     (2048 as libc::c_int + 5 as libc::c_int) as libc::c_long,
     (4096 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (8192 as libc::c_int + 27 as libc::c_int) as libc::c_long,
     (16384 as libc::c_int + 43 as libc::c_int) as libc::c_long,
     (32768 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (65536 as libc::c_int + 45 as libc::c_int) as libc::c_long,
     (131072 as libc::c_int + 29 as libc::c_int) as libc::c_long,
     (262144 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (524288 as libc::c_int + 21 as libc::c_int) as libc::c_long,
     (1048576 as libc::c_int + 7 as libc::c_int) as libc::c_long,
     (2097152 as libc::c_int + 17 as libc::c_int) as libc::c_long,
     (4194304 as libc::c_int + 15 as libc::c_int) as libc::c_long,
     (8388608 as libc::c_int + 9 as libc::c_int) as libc::c_long,
     (16777216 as libc::c_int + 43 as libc::c_int) as libc::c_long,
     (33554432 as libc::c_int + 35 as libc::c_int) as libc::c_long,
     (67108864 as libc::c_int + 15 as libc::c_int) as libc::c_long,
     (134217728 as libc::c_int + 29 as libc::c_int) as libc::c_long,
     (268435456 as libc::c_int + 3 as libc::c_int) as libc::c_long,
     (536870912 as libc::c_int + 11 as libc::c_int) as libc::c_long,
     (1073741824 as libc::c_int + 85 as libc::c_int) as libc::c_long,
     0 as libc::c_int as libc::c_long];
unsafe extern "C" fn new_size(mut size: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut newsize: libc::c_int = 0;
    i = 0 as libc::c_int;
    newsize = 8 as libc::c_int;
    while i <
              (::std::mem::size_of::<[libc::c_long; 29]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_long>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        if newsize > size { return primes[i as usize] as libc::c_int }
        i += 1;
        newsize <<= 1 as libc::c_int
    }
    /* Ran out of polynomials */
    return -(1 as libc::c_int);
    /* should raise exception */
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_table_with_size(mut type_0:
                                                          *mut st_hash_type,
                                                      mut size: libc::c_int)
 -> *mut st_table {
    let mut tbl: *mut st_table =
        0 as *mut st_table; /* round up to prime number */
    size = new_size(size);
    tbl =
        malloc(::std::mem::size_of::<st_table>() as libc::c_ulong as
                   libc::c_uint as libc::c_ulong) as *mut st_table;
    if tbl.is_null() { return 0 as *mut st_table }
    (*tbl).type_0 = type_0;
    (*tbl).num_entries = 0 as libc::c_int;
    (*tbl).num_bins = size;
    (*tbl).bins =
        calloc(size as libc::c_ulong,
               ::std::mem::size_of::<*mut st_table_entry>() as libc::c_ulong)
            as *mut libc::c_char as *mut *mut st_table_entry;
    if (*tbl).bins.is_null() {
        free(tbl as *mut libc::c_void);
        return 0 as *mut st_table
    }
    return tbl;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_table(mut type_0: *mut st_hash_type)
 -> *mut st_table {
    return onig_st_init_table_with_size(type_0, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_numtable() -> *mut st_table {
    return onig_st_init_table(&mut type_numhash);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_numtable_with_size(mut size:
                                                             libc::c_int)
 -> *mut st_table {
    return onig_st_init_table_with_size(&mut type_numhash, size);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_strtable() -> *mut st_table {
    return onig_st_init_table(&mut type_strhash);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_init_strtable_with_size(mut size:
                                                             libc::c_int)
 -> *mut st_table {
    return onig_st_init_table_with_size(&mut type_strhash, size);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_free_table(mut table: *mut st_table) {
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut next: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*table).num_bins {
        ptr = *(*table).bins.offset(i as isize);
        while !ptr.is_null() {
            next = (*ptr).next;
            free(ptr as *mut libc::c_void);
            ptr = next
        }
        i += 1
    }
    free((*table).bins as *mut libc::c_void);
    free(table as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_lookup(mut table: *mut st_table,
                                        mut key: st_data_t,
                                        mut value: *mut st_data_t)
 -> libc::c_int {
    let mut hash_val: libc::c_uint = 0;
    let mut bin_pos: libc::c_uint = 0;
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    hash_val =
        ::std::mem::transmute::<_,
                                fn(_: _)
                                    ->
                                        libc::c_int>(Some((*(*table).type_0).hash.expect("non-null function pointer")).expect("non-null function pointer"))(key)
            as libc::c_uint;
    bin_pos = hash_val.wrapping_rem((*table).num_bins as libc::c_uint);
    ptr = *(*table).bins.offset(bin_pos as isize);
    if !ptr.is_null() &&
           ((*ptr).hash != hash_val ||
                !(key == (*ptr).key ||
                      ::std::mem::transmute::<_,
                                              fn(_: _, _: _)
                                                  ->
                                                      libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))(key,
                                                                                                                                                                             (*ptr).key)
                          == 0 as libc::c_int)) {
        while !(*ptr).next.is_null() &&
                  ((*(*ptr).next).hash != hash_val ||
                       !(key == (*(*ptr).next).key ||
                             ::std::mem::transmute::<_,
                                                     fn(_: _, _: _)
                                                         ->
                                                             libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))(key,
                                                                                                                                                                                    (*(*ptr).next).key)
                                 == 0 as libc::c_int)) {
            ptr = (*ptr).next
        }
        ptr = (*ptr).next
    }
    if ptr.is_null() {
        return 0 as libc::c_int
    } else {
        if !value.is_null() { *value = (*ptr).record }
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_insert(mut table: *mut st_table,
                                        mut key: st_data_t,
                                        mut value: st_data_t) -> libc::c_int {
    let mut hash_val: libc::c_uint = 0;
    let mut bin_pos: libc::c_uint = 0;
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    hash_val =
        ::std::mem::transmute::<_,
                                fn(_: _)
                                    ->
                                        libc::c_int>(Some((*(*table).type_0).hash.expect("non-null function pointer")).expect("non-null function pointer"))(key)
            as libc::c_uint;
    bin_pos = hash_val.wrapping_rem((*table).num_bins as libc::c_uint);
    ptr = *(*table).bins.offset(bin_pos as isize);
    if !ptr.is_null() &&
           ((*ptr).hash != hash_val ||
                !(key == (*ptr).key ||
                      ::std::mem::transmute::<_,
                                              fn(_: _, _: _)
                                                  ->
                                                      libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))(key,
                                                                                                                                                                             (*ptr).key)
                          == 0 as libc::c_int)) {
        while !(*ptr).next.is_null() &&
                  ((*(*ptr).next).hash != hash_val ||
                       !(key == (*(*ptr).next).key ||
                             ::std::mem::transmute::<_,
                                                     fn(_: _, _: _)
                                                         ->
                                                             libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))(key,
                                                                                                                                                                                    (*(*ptr).next).key)
                                 == 0 as libc::c_int)) {
            ptr = (*ptr).next
        }
        ptr = (*ptr).next
    }
    if ptr.is_null() {
        let mut entry: *mut st_table_entry = 0 as *mut st_table_entry;
        if (*table).num_entries / (*table).num_bins > 5 as libc::c_int {
            rehash(table);
            bin_pos = hash_val.wrapping_rem((*table).num_bins as libc::c_uint)
        }
        entry =
            malloc(::std::mem::size_of::<st_table_entry>() as libc::c_ulong as
                       libc::c_uint as libc::c_ulong) as *mut st_table_entry;
        (*entry).hash = hash_val;
        (*entry).key = key;
        (*entry).record = value;
        (*entry).next = *(*table).bins.offset(bin_pos as isize);
        let ref mut fresh0 = *(*table).bins.offset(bin_pos as isize);
        *fresh0 = entry;
        (*table).num_entries += 1;
        return 0 as libc::c_int
    } else { (*ptr).record = value; return 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_add_direct(mut table: *mut st_table,
                                            mut key: st_data_t,
                                            mut value: st_data_t) {
    let mut hash_val: libc::c_uint = 0;
    let mut bin_pos: libc::c_uint = 0;
    hash_val =
        ::std::mem::transmute::<_,
                                fn(_: _)
                                    ->
                                        libc::c_int>(Some((*(*table).type_0).hash.expect("non-null function pointer")).expect("non-null function pointer"))(key)
            as libc::c_uint;
    bin_pos = hash_val.wrapping_rem((*table).num_bins as libc::c_uint);
    let mut entry: *mut st_table_entry = 0 as *mut st_table_entry;
    if (*table).num_entries / (*table).num_bins > 5 as libc::c_int {
        rehash(table);
        bin_pos = hash_val.wrapping_rem((*table).num_bins as libc::c_uint)
    }
    entry =
        malloc(::std::mem::size_of::<st_table_entry>() as libc::c_ulong as
                   libc::c_uint as libc::c_ulong) as *mut st_table_entry;
    (*entry).hash = hash_val;
    (*entry).key = key;
    (*entry).record = value;
    (*entry).next = *(*table).bins.offset(bin_pos as isize);
    let ref mut fresh1 = *(*table).bins.offset(bin_pos as isize);
    *fresh1 = entry;
    (*table).num_entries += 1;
}
unsafe extern "C" fn rehash(mut table: *mut st_table) {
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut next: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut new_bins: *mut *mut st_table_entry =
        0 as *mut *mut st_table_entry;
    let mut i: libc::c_int = 0;
    let mut old_num_bins: libc::c_int = (*table).num_bins;
    let mut new_num_bins: libc::c_int = 0;
    let mut hash_val: libc::c_uint = 0;
    new_num_bins = new_size(old_num_bins + 1 as libc::c_int);
    new_bins =
        calloc(new_num_bins as libc::c_ulong,
               ::std::mem::size_of::<*mut st_table_entry>() as libc::c_ulong)
            as *mut libc::c_char as *mut *mut st_table_entry;
    if new_bins.is_null() { return }
    i = 0 as libc::c_int;
    while i < old_num_bins {
        ptr = *(*table).bins.offset(i as isize);
        while !ptr.is_null() {
            next = (*ptr).next;
            hash_val = (*ptr).hash.wrapping_rem(new_num_bins as libc::c_uint);
            (*ptr).next = *new_bins.offset(hash_val as isize);
            let ref mut fresh2 = *new_bins.offset(hash_val as isize);
            *fresh2 = ptr;
            ptr = next
        }
        i += 1
    }
    free((*table).bins as *mut libc::c_void);
    (*table).num_bins = new_num_bins;
    (*table).bins = new_bins;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_copy(mut old_table: *mut st_table)
 -> *mut st_table {
    let mut new_table: *mut st_table = 0 as *mut st_table;
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut entry: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut i: libc::c_int = 0;
    let mut num_bins: libc::c_int = (*old_table).num_bins;
    new_table =
        malloc(::std::mem::size_of::<st_table>() as libc::c_ulong as
                   libc::c_uint as libc::c_ulong) as *mut st_table;
    if new_table.is_null() { return 0 as *mut st_table }
    *new_table = *old_table;
    (*new_table).bins =
        calloc(num_bins as libc::c_uint as libc::c_ulong,
               ::std::mem::size_of::<*mut st_table_entry>() as libc::c_ulong)
            as *mut libc::c_char as *mut *mut st_table_entry;
    if (*new_table).bins.is_null() {
        free(new_table as *mut libc::c_void);
        return 0 as *mut st_table
    }
    i = 0 as libc::c_int;
    while i < num_bins {
        let ref mut fresh3 = *(*new_table).bins.offset(i as isize);
        *fresh3 = 0 as *mut st_table_entry;
        ptr = *(*old_table).bins.offset(i as isize);
        while !ptr.is_null() {
            entry =
                malloc(::std::mem::size_of::<st_table_entry>() as
                           libc::c_ulong as libc::c_uint as libc::c_ulong) as
                    *mut st_table_entry;
            if entry.is_null() {
                free((*new_table).bins as *mut libc::c_void);
                free(new_table as *mut libc::c_void);
                return 0 as *mut st_table
            }
            *entry = *ptr;
            (*entry).next = *(*new_table).bins.offset(i as isize);
            let ref mut fresh4 = *(*new_table).bins.offset(i as isize);
            *fresh4 = entry;
            ptr = (*ptr).next
        }
        i += 1
    }
    return new_table;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_delete(mut table: *mut st_table,
                                        mut key: *mut st_data_t,
                                        mut value: *mut st_data_t)
 -> libc::c_int {
    let mut hash_val: libc::c_uint = 0;
    let mut tmp: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    hash_val =
        (::std::mem::transmute::<_,
                                 fn(_: _)
                                     ->
                                         libc::c_int>(Some((*(*table).type_0).hash.expect("non-null function pointer")).expect("non-null function pointer"))(*key)
             as libc::c_uint).wrapping_rem((*table).num_bins as libc::c_uint);
    ptr = *(*table).bins.offset(hash_val as isize);
    if ptr.is_null() {
        if !value.is_null() { *value = 0 as libc::c_int as st_data_t }
        return 0 as libc::c_int
    }
    if *key == (*ptr).key ||
           ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))(*key,
                                                                                                                                                                  (*ptr).key)
               == 0 as libc::c_int {
        let ref mut fresh5 = *(*table).bins.offset(hash_val as isize);
        *fresh5 = (*ptr).next;
        (*table).num_entries -= 1;
        if !value.is_null() { *value = (*ptr).record }
        *key = (*ptr).key;
        free(ptr as *mut libc::c_void);
        return 1 as libc::c_int
    }
    while !(*ptr).next.is_null() {
        if (*(*ptr).next).key == *key ||
               ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))((*(*ptr).next).key,
                                                                                                                                                                      *key)
                   == 0 as libc::c_int {
            tmp = (*ptr).next;
            (*ptr).next = (*(*ptr).next).next;
            (*table).num_entries -= 1;
            if !value.is_null() { *value = (*tmp).record }
            *key = (*tmp).key;
            free(tmp as *mut libc::c_void);
            return 1 as libc::c_int
        }
        ptr = (*ptr).next
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_delete_safe(mut table: *mut st_table,
                                             mut key: *mut st_data_t,
                                             mut value: *mut st_data_t,
                                             mut never: st_data_t)
 -> libc::c_int {
    let mut hash_val: libc::c_uint = 0;
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    hash_val =
        (::std::mem::transmute::<_,
                                 fn(_: _)
                                     ->
                                         libc::c_int>(Some((*(*table).type_0).hash.expect("non-null function pointer")).expect("non-null function pointer"))(*key)
             as libc::c_uint).wrapping_rem((*table).num_bins as libc::c_uint);
    ptr = *(*table).bins.offset(hash_val as isize);
    if ptr.is_null() {
        if !value.is_null() { *value = 0 as libc::c_int as st_data_t }
        return 0 as libc::c_int
    }
    while !ptr.is_null() {
        if (*ptr).key != never &&
               ((*ptr).key == *key ||
                    ::std::mem::transmute::<_,
                                            fn(_: _, _: _)
                                                ->
                                                    libc::c_int>(Some((*(*table).type_0).compare.expect("non-null function pointer")).expect("non-null function pointer"))((*ptr).key,
                                                                                                                                                                           *key)
                        == 0 as libc::c_int) {
            (*table).num_entries -= 1;
            *key = (*ptr).key;
            if !value.is_null() { *value = (*ptr).record }
            (*ptr).record = never;
            (*ptr).key = (*ptr).record;
            return 1 as libc::c_int
        }
        ptr = (*ptr).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn delete_never(mut key: st_data_t, mut value: st_data_t,
                                  mut never: st_data_t) -> libc::c_int {
    if value == never { return ST_DELETE as libc::c_int }
    return ST_CONTINUE as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_cleanup_safe(mut table: *mut st_table,
                                              mut never: st_data_t) {
    let mut num_entries: libc::c_int = (*table).num_entries;
    onig_st_foreach(table,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            st_data_t,
                                                                        _:
                                                                            st_data_t,
                                                                        _:
                                                                            st_data_t)
                                                       -> libc::c_int>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           libc::c_int>>(Some(delete_never
                                                                                  as
                                                                                  unsafe extern "C" fn(_:
                                                                                                           st_data_t,
                                                                                                       _:
                                                                                                           st_data_t,
                                                                                                       _:
                                                                                                           st_data_t)
                                                                                      ->
                                                                                          libc::c_int)),
                    never);
    (*table).num_entries = num_entries;
}
#[no_mangle]
pub unsafe extern "C" fn onig_st_foreach(mut table: *mut st_table,
                                         mut func:
                                             Option<unsafe extern "C" fn()
                                                        -> libc::c_int>,
                                         mut arg: st_data_t) -> libc::c_int {
    let mut ptr: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut last: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut tmp: *mut st_table_entry = 0 as *mut st_table_entry;
    let mut retval: st_retval = ST_CONTINUE;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*table).num_bins {
        last = 0 as *mut st_table_entry;
        ptr = *(*table).bins.offset(i as isize);
        while !ptr.is_null() {
            retval =
                ::std::mem::transmute::<_,
                                        fn(_: _, _: _, _: _)
                                            ->
                                                libc::c_int>(Some(func.expect("non-null function pointer")).expect("non-null function pointer"))((*ptr).key,
                                                                                                                                                 (*ptr).record,
                                                                                                                                                 arg)
                    as st_retval;
            let mut current_block_23: u64;
            match retval as libc::c_uint {
                3 => {
                    /* check if hash is modified during iteration */
                    tmp = 0 as *mut st_table_entry;
                    if i < (*table).num_bins {
                        tmp = *(*table).bins.offset(i as isize);
                        while !tmp.is_null() {
                            if tmp == ptr { break ; }
                            tmp = (*tmp).next
                        }
                    }
                    if tmp.is_null() {
                        /* call func with error notice */
                        return 1 as libc::c_int
                    }
                    current_block_23 = 2241088539712269953;
                }
                0 => { current_block_23 = 2241088539712269953; }
                1 => { return 0 as libc::c_int }
                2 => {
                    tmp = ptr;
                    if last.is_null() {
                        let ref mut fresh6 =
                            *(*table).bins.offset(i as isize);
                        *fresh6 = (*ptr).next
                    } else { (*last).next = (*ptr).next }
                    ptr = (*ptr).next;
                    free(tmp as *mut libc::c_void);
                    (*table).num_entries -= 1;
                    current_block_23 = 5689316957504528238;
                }
                _ => { current_block_23 = 5689316957504528238; }
            }
            match current_block_23 {
                2241088539712269953 =>
                /* fall through */
                {
                    last = ptr;
                    ptr = (*ptr).next
                }
                _ => { }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* extern int strcmp(const char *, const char *); */
unsafe extern "C" fn strhash(mut string: *const libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut val: libc::c_int = 0 as libc::c_int;
    loop  {
        let fresh7 = string;
        string = string.offset(1);
        c = *fresh7 as libc::c_int;
        if !(c != '\u{0}' as i32) { break ; }
        val = val * 997 as libc::c_int + c
    }
    return val + (val >> 5 as libc::c_int);
}
/*
     * DEFAULT_MAX_DENSITY is the default for the largest we allow the
     * average number of items per bin before increasing the number of
     * bins
     *
     * DEFAULT_INIT_TABLE_SIZE is the default for the number of bins
     * allocated initially
     *
     */
unsafe extern "C" fn numcmp(mut x: libc::c_long, mut y: libc::c_long)
 -> libc::c_int {
    return (x != y) as libc::c_int;
}
unsafe extern "C" fn numhash(mut n: libc::c_long) -> libc::c_int {
    return n as libc::c_int;
}
