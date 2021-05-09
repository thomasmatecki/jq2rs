#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn jv_mem_free(_: *mut libc::c_void);
    #[no_mangle]
    fn jv_mem_alloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
/* long long available */
/* NO_LONG_LONG */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bigint {
    pub next: *mut Bigint,
    pub k: libc::c_int,
    pub maxwds: libc::c_int,
    pub sign: libc::c_int,
    pub wds: libc::c_int,
    pub x: [ULong; 1],
}
pub type ULong = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtoa_context {
    pub freelist: [*mut Bigint; 8],
    pub p5s: *mut Bigint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union U {
    pub d: libc::c_double,
    pub L: [ULong; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BCinfo {
    pub dp0: libc::c_int,
    pub dp1: libc::c_int,
    pub dplen: libc::c_int,
    pub dsign: libc::c_int,
    pub e0: libc::c_int,
    pub inexact: libc::c_int,
    pub nd: libc::c_int,
    pub nd0: libc::c_int,
    pub rounding: libc::c_int,
    pub scale: libc::c_int,
    pub uflchk: libc::c_int,
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn jvp_dtoa_context_init(mut C: *mut dtoa_context) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[*mut Bigint; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut Bigint>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        (*C).freelist[i as usize] = 0 as *mut Bigint;
        i += 1
    }
    (*C).p5s = 0 as *mut Bigint;
}
unsafe extern "C" fn Balloc(mut C: *mut dtoa_context, mut k: libc::c_int)
 -> *mut Bigint {
    let mut x: libc::c_int = 0;
    let mut rv: *mut Bigint = 0 as *mut Bigint;
    /* The k > Kmax case does not need ACQUIRE_DTOA_LOCK(0), */
	/* but this case seems very unlikely. */
    if k <= 7 as libc::c_int &&
           { rv = (*C).freelist[k as usize]; !rv.is_null() } {
        (*C).freelist[k as usize] = (*rv).next
    } else {
        x = (1 as libc::c_int) << k;
        rv =
            jv_mem_alloc((::std::mem::size_of::<Bigint>() as
                              libc::c_ulong).wrapping_add(((x -
                                                                1 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<ULong>()
                                                                                               as
                                                                                               libc::c_ulong)))
                as *mut Bigint;
        (*rv).k = k;
        (*rv).maxwds = x
    }
    (*rv).wds = 0 as libc::c_int;
    (*rv).sign = (*rv).wds;
    return rv;
}
unsafe extern "C" fn Bfree(mut C: *mut dtoa_context, mut v: *mut Bigint) {
    if !v.is_null() {
        if (*v).k > 7 as libc::c_int {
            jv_mem_free(v as *mut libc::c_void);
        } else {
            (*v).next = (*C).freelist[(*v).k as usize];
            (*C).freelist[(*v).k as usize] = v
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn jvp_dtoa_context_free(mut C: *mut dtoa_context) {
    let mut k: libc::c_int = 0;
    while !(*C).p5s.is_null() {
        let mut p5: *mut Bigint = (*C).p5s;
        (*C).p5s = (*p5).next;
        Bfree(C, p5);
    }
    k = 0 as libc::c_int;
    while k <
              (::std::mem::size_of::<[*mut Bigint; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut Bigint>()
                                                   as libc::c_ulong) as
                  libc::c_int {
        while !(*C).freelist[k as usize].is_null() {
            let mut v: *mut Bigint = (*C).freelist[k as usize];
            (*C).freelist[k as usize] = (*v).next;
            jv_mem_free(v as *mut libc::c_void);
        }
        k += 1
    };
}
unsafe extern "C" fn multadd(mut C: *mut dtoa_context, mut b: *mut Bigint,
                             mut m: libc::c_int, mut a: libc::c_int)
 -> *mut Bigint 
 /* multiply by m and add a */
 {
    let mut i: libc::c_int = 0;
    let mut wds: libc::c_int = 0;
    let mut x: *mut ULong = 0 as *mut ULong;
    let mut carry: libc::c_ulonglong = 0;
    let mut y: libc::c_ulonglong = 0;
    let mut b1: *mut Bigint = 0 as *mut Bigint;
    wds = (*b).wds;
    x = (*b).x.as_mut_ptr();
    i = 0 as libc::c_int;
    carry = a as libc::c_ulonglong;
    loop  {
        y =
            (*x as
                 libc::c_ulonglong).wrapping_mul(m as
                                                     libc::c_ulonglong).wrapping_add(carry);
        carry = y >> 32 as libc::c_int;
        let fresh0 = x;
        x = x.offset(1);
        *fresh0 =
            (y & 0xffffffff as libc::c_ulong as libc::c_ulonglong) as ULong;
        i += 1;
        if !(i < wds) { break ; }
    }
    if carry != 0 {
        if wds >= (*b).maxwds {
            b1 = Balloc(C, (*b).k + 1 as libc::c_int);
            memcpy(&mut (*b1).sign as *mut libc::c_int as *mut libc::c_char as
                       *mut libc::c_void,
                   &mut (*b).sign as *mut libc::c_int as *mut libc::c_char as
                       *const libc::c_void,
                   ((*b).wds as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as
                                                        libc::c_ulong).wrapping_add((2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                                                         as
                                                                                                                         libc::c_ulong)));
            Bfree(C, b);
            b = b1
        }
        let fresh1 = wds;
        wds = wds + 1;
        *(*b).x.as_mut_ptr().offset(fresh1 as isize) = carry as ULong;
        (*b).wds = wds
    }
    return b;
}
unsafe extern "C" fn s2b(mut C: *mut dtoa_context, mut s: *const libc::c_char,
                         mut nd0: libc::c_int, mut nd: libc::c_int,
                         mut y9: ULong, mut dplen: libc::c_int)
 -> *mut Bigint {
    let mut b: *mut Bigint = 0 as *mut Bigint;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    x = (nd + 8 as libc::c_int) / 9 as libc::c_int;
    k = 0 as libc::c_int;
    y = 1 as libc::c_int;
    while x > y { y <<= 1 as libc::c_int; k += 1 }
    b = Balloc(C, k);
    *(*b).x.as_mut_ptr().offset(0 as libc::c_int as isize) = y9;
    (*b).wds = 1 as libc::c_int;
    i = 9 as libc::c_int;
    if (9 as libc::c_int) < nd0 {
        s = s.offset(9 as libc::c_int as isize);
        loop  {
            let fresh2 = s;
            s = s.offset(1);
            b =
                multadd(C, b, 10 as libc::c_int,
                        *fresh2 as libc::c_int - '0' as i32);
            i += 1;
            if !(i < nd0) { break ; }
        }
        s = s.offset(dplen as isize)
    } else { s = s.offset((dplen + 9 as libc::c_int) as isize) }
    while i < nd {
        let fresh3 = s;
        s = s.offset(1);
        b =
            multadd(C, b, 10 as libc::c_int,
                    *fresh3 as libc::c_int - '0' as i32);
        i += 1
    }
    return b;
}
unsafe extern "C" fn hi0bits(mut C: *mut dtoa_context, mut x: ULong)
 -> libc::c_int {
    let mut k: libc::c_int = 0 as libc::c_int;
    if x & 0xffff0000 as libc::c_uint == 0 {
        k = 16 as libc::c_int;
        x <<= 16 as libc::c_int
    }
    if x & 0xff000000 as libc::c_uint == 0 {
        k += 8 as libc::c_int;
        x <<= 8 as libc::c_int
    }
    if x & 0xf0000000 as libc::c_uint == 0 {
        k += 4 as libc::c_int;
        x <<= 4 as libc::c_int
    }
    if x & 0xc0000000 as libc::c_uint == 0 {
        k += 2 as libc::c_int;
        x <<= 2 as libc::c_int
    }
    if x & 0x80000000 as libc::c_uint == 0 {
        k += 1;
        if x & 0x40000000 as libc::c_int as libc::c_uint == 0 {
            return 32 as libc::c_int
        }
    }
    return k;
}
unsafe extern "C" fn lo0bits(mut C: *mut dtoa_context, mut y: *mut ULong)
 -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut x: ULong = *y;
    if x & 7 as libc::c_int as libc::c_uint != 0 {
        if x & 1 as libc::c_int as libc::c_uint != 0 {
            return 0 as libc::c_int
        }
        if x & 2 as libc::c_int as libc::c_uint != 0 {
            *y = x >> 1 as libc::c_int;
            return 1 as libc::c_int
        }
        *y = x >> 2 as libc::c_int;
        return 2 as libc::c_int
    }
    k = 0 as libc::c_int;
    if x & 0xffff as libc::c_int as libc::c_uint == 0 {
        k = 16 as libc::c_int;
        x >>= 16 as libc::c_int
    }
    if x & 0xff as libc::c_int as libc::c_uint == 0 {
        k += 8 as libc::c_int;
        x >>= 8 as libc::c_int
    }
    if x & 0xf as libc::c_int as libc::c_uint == 0 {
        k += 4 as libc::c_int;
        x >>= 4 as libc::c_int
    }
    if x & 0x3 as libc::c_int as libc::c_uint == 0 {
        k += 2 as libc::c_int;
        x >>= 2 as libc::c_int
    }
    if x & 1 as libc::c_int as libc::c_uint == 0 {
        k += 1;
        x >>= 1 as libc::c_int;
        if x == 0 { return 32 as libc::c_int }
    }
    *y = x;
    return k;
}
unsafe extern "C" fn i2b(mut C: *mut dtoa_context, mut i: libc::c_int)
 -> *mut Bigint {
    let mut b: *mut Bigint = 0 as *mut Bigint;
    b = Balloc(C, 1 as libc::c_int);
    *(*b).x.as_mut_ptr().offset(0 as libc::c_int as isize) = i as ULong;
    (*b).wds = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn mult(mut C: *mut dtoa_context, mut a: *mut Bigint,
                          mut b: *mut Bigint) -> *mut Bigint {
    let mut c: *mut Bigint = 0 as *mut Bigint;
    let mut k: libc::c_int = 0;
    let mut wa: libc::c_int = 0;
    let mut wb: libc::c_int = 0;
    let mut wc: libc::c_int = 0;
    let mut x: *mut ULong = 0 as *mut ULong;
    let mut xa: *mut ULong = 0 as *mut ULong;
    let mut xae: *mut ULong = 0 as *mut ULong;
    let mut xb: *mut ULong = 0 as *mut ULong;
    let mut xbe: *mut ULong = 0 as *mut ULong;
    let mut xc: *mut ULong = 0 as *mut ULong;
    let mut xc0: *mut ULong = 0 as *mut ULong;
    let mut y: ULong = 0;
    let mut carry: libc::c_ulonglong = 0;
    let mut z: libc::c_ulonglong = 0;
    if (*a).wds < (*b).wds { c = a; a = b; b = c }
    k = (*a).k;
    wa = (*a).wds;
    wb = (*b).wds;
    wc = wa + wb;
    if wc > (*a).maxwds { k += 1 }
    c = Balloc(C, k);
    x = (*c).x.as_mut_ptr();
    xa = x.offset(wc as isize);
    while x < xa { *x = 0 as libc::c_int as ULong; x = x.offset(1) }
    xa = (*a).x.as_mut_ptr();
    xae = xa.offset(wa as isize);
    xb = (*b).x.as_mut_ptr();
    xbe = xb.offset(wb as isize);
    xc0 = (*c).x.as_mut_ptr();
    while xb < xbe {
        let fresh4 = xb;
        xb = xb.offset(1);
        y = *fresh4;
        if y != 0 {
            x = xa;
            xc = xc0;
            carry = 0 as libc::c_int as libc::c_ulonglong;
            loop  {
                let fresh5 = x;
                x = x.offset(1);
                z =
                    (*fresh5 as
                         libc::c_ulonglong).wrapping_mul(y as
                                                             libc::c_ulonglong).wrapping_add(*xc
                                                                                                 as
                                                                                                 libc::c_ulonglong).wrapping_add(carry);
                carry = z >> 32 as libc::c_int;
                let fresh6 = xc;
                xc = xc.offset(1);
                *fresh6 =
                    (z & 0xffffffff as libc::c_ulong as libc::c_ulonglong) as
                        ULong;
                if !(x < xae) { break ; }
            }
            *xc = carry as ULong
        }
        xc0 = xc0.offset(1)
    }
    xc0 = (*c).x.as_mut_ptr();
    xc = xc0.offset(wc as isize);
    while wc > 0 as libc::c_int && { xc = xc.offset(-1); (*xc) == 0 } {
        wc -= 1
    }
    (*c).wds = wc;
    return c;
}
unsafe extern "C" fn pow5mult(mut C: *mut dtoa_context, mut b: *mut Bigint,
                              mut k: libc::c_int) -> *mut Bigint {
    let mut b1: *mut Bigint = 0 as *mut Bigint;
    let mut p5: *mut Bigint = 0 as *mut Bigint;
    let mut p51: *mut Bigint = 0 as *mut Bigint;
    let mut i: libc::c_int = 0;
    static mut p05: [libc::c_int; 3] =
        [5 as libc::c_int, 25 as libc::c_int, 125 as libc::c_int];
    i = k & 3 as libc::c_int;
    if i != 0 {
        b =
            multadd(C, b, p05[(i - 1 as libc::c_int) as usize],
                    0 as libc::c_int)
    }
    k >>= 2 as libc::c_int;
    if k == 0 { return b }
    p5 = (*C).p5s;
    if p5.is_null() {
        /* first time */
        (*C).p5s =
            i2b(C, 625 as libc::c_int); /* clear sign bit, which we ignore */
        p5 = (*C).p5s;
        (*p5).next = 0 as *mut Bigint
    }
    loop  {
        if k & 1 as libc::c_int != 0 {
            b1 = mult(C, b, p5);
            Bfree(C, b);
            b = b1
        }
        k >>= 1 as libc::c_int;
        if k == 0 { break ; }
        p51 = (*p5).next;
        if p51.is_null() {
            (*p5).next = mult(C, p5, p5);
            p51 = (*p5).next;
            (*p51).next = 0 as *mut Bigint
        }
        p5 = p51
    }
    return b;
}
unsafe extern "C" fn lshift(mut C: *mut dtoa_context, mut b: *mut Bigint,
                            mut k: libc::c_int) -> *mut Bigint {
    let mut i: libc::c_int = 0;
    let mut k1: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut b1: *mut Bigint = 0 as *mut Bigint;
    let mut x: *mut ULong = 0 as *mut ULong;
    let mut x1: *mut ULong = 0 as *mut ULong;
    let mut xe: *mut ULong = 0 as *mut ULong;
    let mut z: ULong = 0;
    n = k >> 5 as libc::c_int;
    k1 = (*b).k;
    n1 = n + (*b).wds + 1 as libc::c_int;
    i = (*b).maxwds;
    while n1 > i { k1 += 1; i <<= 1 as libc::c_int }
    b1 = Balloc(C, k1);
    x1 = (*b1).x.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < n {
        let fresh7 = x1;
        x1 = x1.offset(1);
        *fresh7 = 0 as libc::c_int as ULong;
        i += 1
    }
    x = (*b).x.as_mut_ptr();
    xe = x.offset((*b).wds as isize);
    k &= 0x1f as libc::c_int;
    if k != 0 {
        k1 = 32 as libc::c_int - k;
        z = 0 as libc::c_int as ULong;
        loop  {
            let fresh8 = x1;
            x1 = x1.offset(1);
            *fresh8 = *x << k | z;
            let fresh9 = x;
            x = x.offset(1);
            z = *fresh9 >> k1;
            if !(x < xe) { break ; }
        }
        *x1 = z;
        if *x1 != 0 { n1 += 1 }
    } else {
        loop  {
            let fresh10 = x;
            x = x.offset(1);
            let fresh11 = x1;
            x1 = x1.offset(1);
            *fresh11 = *fresh10;
            if !(x < xe) { break ; }
        }
    }
    (*b1).wds = n1 - 1 as libc::c_int;
    Bfree(C, b);
    return b1;
}
unsafe extern "C" fn cmp(mut C: *mut dtoa_context, mut a: *mut Bigint,
                         mut b: *mut Bigint) -> libc::c_int {
    let mut xa: *mut ULong = 0 as *mut ULong;
    let mut xa0: *mut ULong = 0 as *mut ULong;
    let mut xb: *mut ULong = 0 as *mut ULong;
    let mut xb0: *mut ULong = 0 as *mut ULong;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = (*a).wds;
    j = (*b).wds;
    i -= j;
    if i != 0 { return i }
    xa0 = (*a).x.as_mut_ptr();
    xa = xa0.offset(j as isize);
    xb0 = (*b).x.as_mut_ptr();
    xb = xb0.offset(j as isize);
    loop  {
        xa = xa.offset(-1);
        xb = xb.offset(-1);
        if *xa != *xb {
            return if *xa < *xb {
                       -(1 as libc::c_int)
                   } else { 1 as libc::c_int }
        }
        if xa <= xa0 { break ; }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn diff(mut C: *mut dtoa_context, mut a: *mut Bigint,
                          mut b: *mut Bigint) -> *mut Bigint {
    let mut c: *mut Bigint = 0 as *mut Bigint;
    let mut i: libc::c_int = 0;
    let mut wa: libc::c_int = 0;
    let mut wb: libc::c_int = 0;
    let mut xa: *mut ULong = 0 as *mut ULong;
    let mut xae: *mut ULong = 0 as *mut ULong;
    let mut xb: *mut ULong = 0 as *mut ULong;
    let mut xbe: *mut ULong = 0 as *mut ULong;
    let mut xc: *mut ULong = 0 as *mut ULong;
    let mut borrow: libc::c_ulonglong = 0;
    let mut y: libc::c_ulonglong = 0;
    i = cmp(C, a, b);
    if i == 0 {
        c = Balloc(C, 0 as libc::c_int);
        (*c).wds = 1 as libc::c_int;
        *(*c).x.as_mut_ptr().offset(0 as libc::c_int as isize) =
            0 as libc::c_int as ULong;
        return c
    }
    if i < 0 as libc::c_int {
        c = a;
        a = b;
        b = c;
        i = 1 as libc::c_int
    } else { i = 0 as libc::c_int }
    c = Balloc(C, (*a).k);
    (*c).sign = i;
    wa = (*a).wds;
    xa = (*a).x.as_mut_ptr();
    xae = xa.offset(wa as isize);
    wb = (*b).wds;
    xb = (*b).x.as_mut_ptr();
    xbe = xb.offset(wb as isize);
    xc = (*c).x.as_mut_ptr();
    borrow = 0 as libc::c_int as libc::c_ulonglong;
    loop  {
        let fresh12 = xa;
        xa = xa.offset(1);
        let fresh13 = xb;
        xb = xb.offset(1);
        y =
            (*fresh12 as
                 libc::c_ulonglong).wrapping_sub(*fresh13 as
                                                     libc::c_ulonglong).wrapping_sub(borrow);
        borrow =
            y >> 32 as libc::c_int &
                1 as libc::c_int as ULong as libc::c_ulonglong;
        let fresh14 = xc;
        xc = xc.offset(1);
        *fresh14 =
            (y & 0xffffffff as libc::c_ulong as libc::c_ulonglong) as ULong;
        if !(xb < xbe) { break ; }
    }
    while xa < xae {
        let fresh15 = xa;
        xa = xa.offset(1);
        y = (*fresh15 as libc::c_ulonglong).wrapping_sub(borrow);
        borrow =
            y >> 32 as libc::c_int &
                1 as libc::c_int as ULong as libc::c_ulonglong;
        let fresh16 = xc;
        xc = xc.offset(1);
        *fresh16 =
            (y & 0xffffffff as libc::c_ulong as libc::c_ulonglong) as ULong
    }
    loop  { xc = xc.offset(-1); if !(*xc == 0) { break ; } wa -= 1 }
    (*c).wds = wa;
    return c;
}
unsafe extern "C" fn ulp(mut C: *mut dtoa_context, mut x: *mut U)
 -> libc::c_double {
    let mut L: libc::c_int = 0;
    let mut u: U = U{d: 0.,};
    L =
        ((*x).L[1 as libc::c_int as usize] &
             0x7ff00000 as libc::c_int as
                 libc::c_uint).wrapping_sub(((53 as libc::c_int -
                                                  1 as libc::c_int) *
                                                 0x100000 as libc::c_int) as
                                                libc::c_uint) as libc::c_int;
    u.L[1 as libc::c_int as usize] = L as ULong;
    u.L[0 as libc::c_int as usize] = 0 as libc::c_int as ULong;
    return u.d;
}
unsafe extern "C" fn b2d(mut C: *mut dtoa_context, mut a: *mut Bigint,
                         mut e: *mut libc::c_int) -> libc::c_double {
    let mut xa: *mut ULong = 0 as *mut ULong;
    let mut xa0: *mut ULong = 0 as *mut ULong;
    let mut w: ULong = 0;
    let mut y: ULong = 0;
    let mut z: ULong = 0;
    let mut k: libc::c_int = 0;
    let mut d: U = U{d: 0.,};
    xa0 = (*a).x.as_mut_ptr();
    xa = xa0.offset((*a).wds as isize);
    xa = xa.offset(-1);
    y = *xa;
    k = hi0bits(C, y);
    *e = 32 as libc::c_int - k;
    if k < 11 as libc::c_int {
        d.L[1 as libc::c_int as usize] =
            0x3ff00000 as libc::c_int as libc::c_uint |
                y >> 11 as libc::c_int - k;
        w =
            if xa > xa0 {
                xa = xa.offset(-1);
                *xa
            } else { 0 as libc::c_int as libc::c_uint };
        d.L[0 as libc::c_int as usize] =
            y << 32 as libc::c_int - 11 as libc::c_int + k |
                w >> 11 as libc::c_int - k
    } else {
        z =
            if xa > xa0 {
                xa = xa.offset(-1);
                *xa
            } else { 0 as libc::c_int as libc::c_uint };
        k -= 11 as libc::c_int;
        if k != 0 {
            d.L[1 as libc::c_int as usize] =
                0x3ff00000 as libc::c_int as libc::c_uint | y << k |
                    z >> 32 as libc::c_int - k;
            y =
                if xa > xa0 {
                    xa = xa.offset(-1);
                    *xa
                } else { 0 as libc::c_int as libc::c_uint };
            d.L[0 as libc::c_int as usize] =
                z << k | y >> 32 as libc::c_int - k
        } else {
            d.L[1 as libc::c_int as usize] =
                0x3ff00000 as libc::c_int as libc::c_uint | y;
            d.L[0 as libc::c_int as usize] = z
        }
    }
    return d.d;
}
unsafe extern "C" fn d2b(mut C: *mut dtoa_context, mut d: *mut U,
                         mut e: *mut libc::c_int, mut bits: *mut libc::c_int)
 -> *mut Bigint {
    let mut b: *mut Bigint = 0 as *mut Bigint;
    let mut de: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: *mut ULong = 0 as *mut ULong;
    let mut y: ULong = 0;
    let mut z: ULong = 0;
    let mut i: libc::c_int = 0;
    b = Balloc(C, 1 as libc::c_int);
    x = (*b).x.as_mut_ptr();
    z =
        (*d).L[1 as libc::c_int as usize] &
            0xfffff as libc::c_int as libc::c_uint;
    (*d).L[1 as libc::c_int as usize] &=
        0x7fffffff as libc::c_int as libc::c_uint;
    de =
        ((*d).L[1 as libc::c_int as usize] >> 20 as libc::c_int) as
            libc::c_int;
    if de != 0 { z |= 0x100000 as libc::c_int as libc::c_uint }
    y = (*d).L[0 as libc::c_int as usize];
    if y != 0 {
        k = lo0bits(C, &mut y);
        if k != 0 {
            *x.offset(0 as libc::c_int as isize) =
                y | z << 32 as libc::c_int - k;
            z >>= k
        } else { *x.offset(0 as libc::c_int as isize) = y }
        let ref mut fresh17 = *x.offset(1 as libc::c_int as isize);
        *fresh17 = z;
        (*b).wds =
            if *fresh17 != 0 { 2 as libc::c_int } else { 1 as libc::c_int };
        i = (*b).wds
    } else {
        k = lo0bits(C, &mut z);
        *x.offset(0 as libc::c_int as isize) = z;
        (*b).wds = 1 as libc::c_int;
        i = (*b).wds;
        k += 32 as libc::c_int
    }
    if de != 0 {
        *e =
            de - 1023 as libc::c_int - (53 as libc::c_int - 1 as libc::c_int)
                + k;
        *bits = 53 as libc::c_int - k
    } else {
        *e =
            de - 1023 as libc::c_int - (53 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int + k;
        *bits =
            32 as libc::c_int * i -
                hi0bits(C, *x.offset((i - 1 as libc::c_int) as isize))
    }
    return b;
}
unsafe extern "C" fn ratio(mut C: *mut dtoa_context, mut a: *mut Bigint,
                           mut b: *mut Bigint) -> libc::c_double {
    let mut da: U = U{d: 0.,};
    let mut db: U = U{d: 0.,};
    let mut k: libc::c_int = 0;
    let mut ka: libc::c_int = 0;
    let mut kb: libc::c_int = 0;
    da.d = b2d(C, a, &mut ka);
    db.d = b2d(C, b, &mut kb);
    k = ka - kb + 32 as libc::c_int * ((*a).wds - (*b).wds);
    if k > 0 as libc::c_int {
        da.L[1 as libc::c_int as usize] =
            (da.L[1 as libc::c_int as usize] as
                 libc::c_uint).wrapping_add((k * 0x100000 as libc::c_int) as
                                                libc::c_uint) as ULong as
                ULong
    } else {
        k = -k;
        db.L[1 as libc::c_int as usize] =
            (db.L[1 as libc::c_int as usize] as
                 libc::c_uint).wrapping_add((k * 0x100000 as libc::c_int) as
                                                libc::c_uint) as ULong as
                ULong
    }
    return da.d / db.d;
}
static mut tens: [libc::c_double; 23] =
    [1e0f64, 1e1f64, 1e2f64, 1e3f64, 1e4f64, 1e5f64, 1e6f64, 1e7f64, 1e8f64,
     1e9f64, 1e10f64, 1e11f64, 1e12f64, 1e13f64, 1e14f64, 1e15f64, 1e16f64,
     1e17f64, 1e18f64, 1e19f64, 1e20f64, 1e21f64, 1e22f64];
static mut bigtens: [libc::c_double; 5] =
    [1e16f64, 1e32f64, 1e64f64, 1e128f64, 1e256f64];
static mut tinytens: [libc::c_double; 5] =
    [1e-16f64, 1e-32f64, 1e-64f64, 1e-128f64,
     9007199254740992.0f64 * 9007199254740992.0e-256f64];
unsafe extern "C" fn match_0(mut C: *mut dtoa_context,
                             mut sp: *mut *const libc::c_char,
                             mut t: *const libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut s: *const libc::c_char = *sp;
    loop  {
        let fresh18 = t;
        t = t.offset(1);
        d = *fresh18 as libc::c_int;
        if !(d != 0) { break ; }
        s = s.offset(1);
        c = *s as libc::c_int;
        if c >= 'A' as i32 && c <= 'Z' as i32 { c += 'a' as i32 - 'A' as i32 }
        if c != d { return 0 as libc::c_int }
    }
    *sp = s.offset(1 as libc::c_int as isize);
    return 1 as libc::c_int;
}
/*{*/
/*}*/
/*{*/
/* !NO_HEX_FP}*/
unsafe extern "C" fn dshift(mut C: *mut dtoa_context, mut b: *mut Bigint,
                            mut p2: libc::c_int) -> libc::c_int {
    let mut rv: libc::c_int =
        hi0bits(C,
                *(*b).x.as_mut_ptr().offset(((*b).wds - 1 as libc::c_int) as
                                                isize)) -
            4 as libc::c_int; /* ensure q <= true quotient */
    if p2 > 0 as libc::c_int { rv -= p2 }
    return rv & 31 as libc::c_int;
}
unsafe extern "C" fn quorem(mut C: *mut dtoa_context, mut b: *mut Bigint,
                            mut S: *mut Bigint) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut bx: *mut ULong = 0 as *mut ULong;
    let mut bxe: *mut ULong = 0 as *mut ULong;
    let mut q: ULong = 0;
    let mut sx: *mut ULong = 0 as *mut ULong;
    let mut sxe: *mut ULong = 0 as *mut ULong;
    let mut borrow: libc::c_ulonglong = 0;
    let mut carry: libc::c_ulonglong = 0;
    let mut y: libc::c_ulonglong = 0;
    let mut ys: libc::c_ulonglong = 0;
    n = (*S).wds;
    if (*b).wds < n { return 0 as libc::c_int }
    sx = (*S).x.as_mut_ptr();
    n -= 1;
    sxe = sx.offset(n as isize);
    bx = (*b).x.as_mut_ptr();
    bxe = bx.offset(n as isize);
    q =
        (*bxe).wrapping_div((*sxe).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint));
    if q != 0 {
        borrow = 0 as libc::c_int as libc::c_ulonglong;
        carry = 0 as libc::c_int as libc::c_ulonglong;
        loop  {
            let fresh19 = sx;
            sx = sx.offset(1);
            ys =
                (*fresh19 as
                     libc::c_ulonglong).wrapping_mul(q as
                                                         libc::c_ulonglong).wrapping_add(carry);
            carry = ys >> 32 as libc::c_int;
            y =
                (*bx as
                     libc::c_ulonglong).wrapping_sub(ys &
                                                         0xffffffff as
                                                             libc::c_ulong as
                                                             libc::c_ulonglong).wrapping_sub(borrow);
            borrow =
                y >> 32 as libc::c_int &
                    1 as libc::c_int as ULong as libc::c_ulonglong;
            let fresh20 = bx;
            bx = bx.offset(1);
            *fresh20 =
                (y & 0xffffffff as libc::c_ulong as libc::c_ulonglong) as
                    ULong;
            if !(sx <= sxe) { break ; }
        }
        if *bxe == 0 {
            bx = (*b).x.as_mut_ptr();
            loop  {
                bxe = bxe.offset(-1);
                if !(bxe > bx && *bxe == 0) { break ; }
                n -= 1
            }
            (*b).wds = n
        }
    }
    if cmp(C, b, S) >= 0 as libc::c_int {
        q = q.wrapping_add(1);
        borrow = 0 as libc::c_int as libc::c_ulonglong;
        carry = 0 as libc::c_int as libc::c_ulonglong;
        bx = (*b).x.as_mut_ptr();
        sx = (*S).x.as_mut_ptr();
        loop  {
            let fresh21 = sx;
            sx = sx.offset(1);
            ys = (*fresh21 as libc::c_ulonglong).wrapping_add(carry);
            carry = ys >> 32 as libc::c_int;
            y =
                (*bx as
                     libc::c_ulonglong).wrapping_sub(ys &
                                                         0xffffffff as
                                                             libc::c_ulong as
                                                             libc::c_ulonglong).wrapping_sub(borrow);
            borrow =
                y >> 32 as libc::c_int &
                    1 as libc::c_int as ULong as libc::c_ulonglong;
            let fresh22 = bx;
            bx = bx.offset(1);
            *fresh22 =
                (y & 0xffffffff as libc::c_ulong as libc::c_ulonglong) as
                    ULong;
            if !(sx <= sxe) { break ; }
        }
        bx = (*b).x.as_mut_ptr();
        bxe = bx.offset(n as isize);
        if *bxe == 0 {
            loop  {
                bxe = bxe.offset(-1);
                if !(bxe > bx && *bxe == 0) { break ; }
                n -= 1
            }
            (*b).wds = n
        }
    }
    return q as libc::c_int;
}
/*{*/
unsafe extern "C" fn sulp(mut C: *mut dtoa_context, mut x: *mut U,
                          mut bc: *mut BCinfo) -> libc::c_double {
    let mut u: U = U{d: 0.,}; /* Is there an example where i <= 0 ? */
    let mut rv: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    rv = ulp(C, x);
    if (*bc).scale == 0 ||
           {
               i =
                   ((2 as libc::c_int * 53 as libc::c_int + 1 as libc::c_int)
                        as
                        libc::c_uint).wrapping_sub(((*x).L[1 as libc::c_int as
                                                               usize] &
                                                        0x7ff00000 as
                                                            libc::c_int as
                                                            libc::c_uint) >>
                                                       20 as libc::c_int) as
                       libc::c_int;
               (i) <= 0 as libc::c_int
           } {
        return rv
    }
    u.L[1 as libc::c_int as usize] =
        (0x3ff00000 as libc::c_int + (i << 20 as libc::c_int)) as ULong;
    u.L[0 as libc::c_int as usize] = 0 as libc::c_int as ULong;
    return rv * u.d;
}
/*}*/
unsafe extern "C" fn bigcomp(mut C: *mut dtoa_context, mut rv: *mut U,
                             mut s0: *const libc::c_char,
                             mut bc: *mut BCinfo) {
    let mut current_block: u64;
    let mut b: *mut Bigint = 0 as *mut Bigint;
    let mut d: *mut Bigint = 0 as *mut Bigint;
    let mut b2: libc::c_int = 0;
    let mut bbits: libc::c_int = 0;
    let mut d2: libc::c_int = 0;
    let mut dd: libc::c_int = 0 as libc::c_int;
    let mut dig: libc::c_int = 0;
    let mut dsign: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nd: libc::c_int = 0;
    let mut nd0: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut p5: libc::c_int = 0;
    let mut speccase: libc::c_int = 0;
    dsign = (*bc).dsign;
    nd = (*bc).nd;
    nd0 = (*bc).nd0;
    p5 = nd + (*bc).e0 - 1 as libc::c_int;
    speccase = 0 as libc::c_int;
    if (*rv).d == 0.0f64 {
        /* special case: value near underflow-to-zero */
        /* threshold was rounded to zero */
        b = i2b(C, 1 as libc::c_int);
        p2 = -(1022 as libc::c_int) - 53 as libc::c_int + 1 as libc::c_int;
        bbits = 1 as libc::c_int;
        (*rv).L[1 as libc::c_int as usize] =
            ((53 as libc::c_int + 2 as libc::c_int) << 20 as libc::c_int) as
                ULong;
        i = 0 as libc::c_int;
        speccase = 1 as libc::c_int;
        p2 -= 1;
        dsign = 0 as libc::c_int
    } else {
        b = d2b(C, rv, &mut p2, &mut bbits);
        p2 -= (*bc).scale;
        /* floor(log2(rv)) == bbits - 1 + p2 */
	/* Check for denormal case. */
        i = 53 as libc::c_int - bbits;
        j =
            53 as libc::c_int - -(1022 as libc::c_int) - 1 as libc::c_int +
                p2;
        if i > j { i = j }
        i += 1;
        b = lshift(C, b, i);
        let ref mut fresh23 =
            *(*b).x.as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh23 |= 1 as libc::c_int as libc::c_uint
    }
    p2 -= p5 + i;
    d = i2b(C, 1 as libc::c_int);
    /* Arrange for convenient computation of quotients:
	 * shift left if necessary so divisor has 4 leading 0 bits.
	 */
    if p5 > 0 as libc::c_int {
        d = pow5mult(C, d, p5)
    } else if p5 < 0 as libc::c_int { b = pow5mult(C, b, -p5) }
    if p2 > 0 as libc::c_int {
        b2 = p2;
        d2 = 0 as libc::c_int
    } else { b2 = 0 as libc::c_int; d2 = -p2 }
    i = dshift(C, d, d2);
    b2 += i;
    if b2 > 0 as libc::c_int { b = lshift(C, b, b2) }
    d2 += i;
    if d2 > 0 as libc::c_int { d = lshift(C, d, d2) }
    /* Now b/d = exactly half-way between the two floating-point values */
	/* on either side of the input string.  Compute first digit of b/d. */
    dig = quorem(C, b, d); /* very unlikely */
    if dig == 0 {
        b = multadd(C, b, 10 as libc::c_int, 0 as libc::c_int);
        dig = quorem(C, b, d)
    }
    /* Compare b/d with s0 */
    i = 0 as libc::c_int;
    loop  {
        if !(i < nd0) { current_block = 5141539773904409130; break ; }
        let fresh24 = i;
        i = i + 1;
        dd = *s0.offset(fresh24 as isize) as libc::c_int - '0' as i32 - dig;
        if dd != 0 { current_block = 17231724591110842312; break ; }
        if *(*b).x.as_mut_ptr().offset(0 as libc::c_int as isize) == 0 &&
               (*b).wds == 1 as libc::c_int {
            if i < nd { dd = 1 as libc::c_int }
            current_block = 17231724591110842312;
            break ;
        } else {
            b = multadd(C, b, 10 as libc::c_int, 0 as libc::c_int);
            dig = quorem(C, b, d)
        }
    }
    match current_block {
        5141539773904409130 => {
            j = (*bc).dp1;
            loop  {
                let fresh25 = i;
                i = i + 1;
                if !(fresh25 < nd) {
                    current_block = 11441799814184323368;
                    break ;
                }
                let fresh26 = j;
                j = j + 1;
                dd =
                    *s0.offset(fresh26 as isize) as libc::c_int - '0' as i32 -
                        dig;
                if dd != 0 { current_block = 17231724591110842312; break ; }
                if *(*b).x.as_mut_ptr().offset(0 as libc::c_int as isize) == 0
                       && (*b).wds == 1 as libc::c_int {
                    if i < nd { dd = 1 as libc::c_int }
                    current_block = 17231724591110842312;
                    break ;
                } else {
                    b = multadd(C, b, 10 as libc::c_int, 0 as libc::c_int);
                    dig = quorem(C, b, d)
                }
            }
            match current_block {
                17231724591110842312 => { }
                _ => {
                    if dig > 0 as libc::c_int ||
                           *(*b).x.as_mut_ptr().offset(0 as libc::c_int as
                                                           isize) != 0 ||
                           (*b).wds > 1 as libc::c_int {
                        dd = -(1 as libc::c_int)
                    }
                }
            }
        }
        _ => { }
    }
    Bfree(C, b);
    Bfree(C, d);
    if speccase != 0 {
        if dd <= 0 as libc::c_int { (*rv).d = 0.0f64 }
    } else {
        if dd < 0 as libc::c_int {
            if dsign == 0 {
                current_block = 16193741268801899318;
            } else { current_block = 15237655884915618618; }
        } else {
            if dd > 0 as libc::c_int {
                if dsign != 0 {
                    current_block = 4589532373470817541;
                } else { current_block = 15237655884915618618; }
            } else {
                /* Exact half-way case:  apply round-even rule. */
                j =
                    (((*rv).L[1 as libc::c_int as usize] &
                          0x7ff00000 as libc::c_int as libc::c_uint) >>
                         20 as
                             libc::c_int).wrapping_sub((*bc).scale as
                                                           libc::c_uint) as
                        libc::c_int;
                if j <= 0 as libc::c_int {
                    i = 1 as libc::c_int - j;
                    if i <= 31 as libc::c_int {
                        if (*rv).L[0 as libc::c_int as usize] &
                               ((0x1 as libc::c_int) << i) as libc::c_uint !=
                               0 {
                            current_block = 10386398493584460465;
                        } else { current_block = 15237655884915618618; }
                    } else if (*rv).L[1 as libc::c_int as usize] &
                                  ((0x1 as libc::c_int) <<
                                       i - 32 as libc::c_int) as libc::c_uint
                                  != 0 {
                        current_block = 10386398493584460465;
                    } else { current_block = 15237655884915618618; }
                } else if (*rv).L[0 as libc::c_int as usize] &
                              1 as libc::c_int as libc::c_uint != 0 {
                    current_block = 10386398493584460465;
                } else { current_block = 15237655884915618618; }
                match current_block {
                    15237655884915618618 => { }
                    _ => {
                        if dsign != 0 {
                            current_block = 4589532373470817541;
                        } else { current_block = 16193741268801899318; }
                    }
                }
            }
            match current_block {
                16193741268801899318 => { }
                15237655884915618618 => { }
                _ => {
                    (*rv).d += sulp(C, rv, bc);
                    current_block = 15237655884915618618;
                }
            }
        }
        match current_block {
            15237655884915618618 => { }
            _ =>
            /* does not happen for round-near */
            {
                (*rv).d -= sulp(C, rv, bc)
            }
        }
    };
}
/* NO_STRTOD_BIGCOMP */
#[no_mangle]
pub unsafe extern "C" fn jvp_strtod(mut C: *mut dtoa_context,
                                    mut s00: *const libc::c_char,
                                    mut se: *mut *mut libc::c_char)
 -> libc::c_double {
    let mut current_block: u64;
    let mut bb2: libc::c_int = 0;
    let mut bb5: libc::c_int = 0;
    let mut bbe: libc::c_int = 0;
    let mut bd2: libc::c_int = 0;
    let mut bd5: libc::c_int = 0;
    let mut bbbits: libc::c_int = 0;
    let mut bs2: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut e1: libc::c_int = 0;
    let mut test_scale: libc::c_int = 0;
    let mut esign: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nd: libc::c_int = 0;
    let mut nd0: libc::c_int = 0;
    let mut nf: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    let mut nz0: libc::c_int = 0;
    let mut nz1: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut s0: *const libc::c_char = 0 as *const libc::c_char;
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut aadj: libc::c_double = 0.;
    let mut aadj1: libc::c_double = 0.;
    let mut L: libc::c_int = 0;
    let mut aadj2: U = U{d: 0.,};
    let mut adj: U = U{d: 0.,};
    let mut rv: U = U{d: 0.,};
    let mut rv0: U = U{d: 0.,};
    let mut y: ULong = 0;
    let mut z: ULong = 0;
    let mut bc: BCinfo =
        BCinfo{dp0: 0,
               dp1: 0,
               dplen: 0,
               dsign: 0,
               e0: 0,
               inexact: 0,
               nd: 0,
               nd0: 0,
               rounding: 0,
               scale: 0,
               uflchk: 0,};
    let mut bb: *mut Bigint = 0 as *mut Bigint;
    let mut bb1: *mut Bigint = 0 as *mut Bigint;
    let mut bd: *mut Bigint = 0 as *mut Bigint;
    let mut bd0: *mut Bigint = 0 as *mut Bigint;
    let mut bs: *mut Bigint = 0 as *mut Bigint;
    let mut delta: *mut Bigint = 0 as *mut Bigint;
    let mut Lsb: ULong = 0;
    let mut Lsb1: ULong = 0;
    let mut req_bigcomp: libc::c_int = 0 as libc::c_int;
    /*{*/
    /*}*/
    bc.uflchk = 0 as libc::c_int;
    bc.dplen = bc.uflchk;
    nz = bc.dplen;
    nz1 = nz;
    nz0 = nz1;
    sign = nz0;
    rv.d = 0.0f64;
    s = s00;
    loop  {
        match *s as libc::c_int {
            45 => { sign = 1 as libc::c_int }
            43 => { }
            0 => { current_block = 2153641082331438319; break ; }
            9 | 10 | 11 | 12 | 13 | 32 => { s = s.offset(1); continue ; }
            _ => { current_block = 1661123003442181300; break ; }
        }
        /* no break */
        s = s.offset(1);
        if *s != 0 { current_block = 1661123003442181300; break ; }
        /* no break */
        current_block = 2153641082331438319;
        break ;
    }
    match current_block {
        1661123003442181300 => {
            if *s as libc::c_int == '0' as i32 {
                /*{*/
                /*}*/
                nz0 = 1 as libc::c_int;
                loop  {
                    s = s.offset(1);
                    if !(*s as libc::c_int == '0' as i32) { break ; }
                }
                if *s == 0 {
                    current_block = 4077676600814058918;
                } else { current_block = 14401909646449704462; }
            } else { current_block = 14401909646449704462; }
            match current_block {
                4077676600814058918 => { }
                _ => {
                    s0 = s;
                    z = 0 as libc::c_int as ULong;
                    y = z;
                    nf = 0 as libc::c_int;
                    nd = nf;
                    loop  {
                        c = *s as libc::c_int;
                        if !(c >= '0' as i32 && c <= '9' as i32) { break ; }
                        if nd < 9 as libc::c_int {
                            y =
                                (10 as libc::c_int as
                                     libc::c_uint).wrapping_mul(y).wrapping_add(c
                                                                                    as
                                                                                    libc::c_uint).wrapping_sub('0'
                                                                                                                   as
                                                                                                                   i32
                                                                                                                   as
                                                                                                                   libc::c_uint)
                        } else if nd < 16 as libc::c_int {
                            z =
                                (10 as libc::c_int as
                                     libc::c_uint).wrapping_mul(z).wrapping_add(c
                                                                                    as
                                                                                    libc::c_uint).wrapping_sub('0'
                                                                                                                   as
                                                                                                                   i32
                                                                                                                   as
                                                                                                                   libc::c_uint)
                        }
                        nd += 1;
                        s = s.offset(1)
                    }
                    nd0 = nd;
                    bc.dp1 =
                        s.wrapping_offset_from(s0) as libc::c_long as
                            libc::c_int;
                    bc.dp0 = bc.dp1;
                    s1 = s;
                    while s1 > s0 &&
                              {
                                  s1 = s1.offset(-1);
                                  (*s1 as libc::c_int) == '0' as i32
                              } {
                        nz1 += 1
                    }
                    if c == '.' as i32 {
                        s = s.offset(1);
                        c = *s as libc::c_int;
                        bc.dp1 =
                            s.wrapping_offset_from(s0) as libc::c_long as
                                libc::c_int;
                        bc.dplen = bc.dp1 - bc.dp0;
                        if nd == 0 {
                            while c == '0' as i32 {
                                nz += 1;
                                s = s.offset(1);
                                c = *s as libc::c_int
                            }
                            if c > '0' as i32 && c <= '9' as i32 {
                                bc.dp0 =
                                    s0.wrapping_offset_from(s) as libc::c_long
                                        as libc::c_int;
                                bc.dp1 = bc.dp0 + bc.dplen;
                                s0 = s;
                                nf += nz;
                                nz = 0 as libc::c_int;
                                current_block = 5049394217699438129;
                            } else { current_block = 938300398986307156; }
                        } else { current_block = 6476622998065200121; }
                        match current_block {
                            938300398986307156 => { }
                            _ => {
                                loop  {
                                    match current_block {
                                        6476622998065200121 => {
                                            if c >= '0' as i32 &&
                                                   c <= '9' as i32 {
                                                current_block =
                                                    5049394217699438129;
                                            } else { break ; }
                                        }
                                        _ => {
                                            nz += 1;
                                            c -= '0' as i32;
                                            if c != 0 {
                                                nf += nz;
                                                i = 1 as libc::c_int;
                                                while i < nz {
                                                    let fresh27 = nd;
                                                    nd = nd + 1;
                                                    if fresh27 <
                                                           9 as libc::c_int {
                                                        y =
                                                            (y as
                                                                 libc::c_uint).wrapping_mul(10
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                                as ULong as
                                                                ULong
                                                    } else if nd <=
                                                                  15 as
                                                                      libc::c_int
                                                                      +
                                                                      1 as
                                                                          libc::c_int
                                                     {
                                                        z =
                                                            (z as
                                                                 libc::c_uint).wrapping_mul(10
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                                as ULong as
                                                                ULong
                                                    }
                                                    i += 1
                                                }
                                                let fresh28 = nd;
                                                nd = nd + 1;
                                                if fresh28 < 9 as libc::c_int
                                                   {
                                                    y =
                                                        (10 as libc::c_int as
                                                             libc::c_uint).wrapping_mul(y).wrapping_add(c
                                                                                                            as
                                                                                                            libc::c_uint)
                                                } else if nd <=
                                                              15 as
                                                                  libc::c_int
                                                                  +
                                                                  1 as
                                                                      libc::c_int
                                                 {
                                                    z =
                                                        (10 as libc::c_int as
                                                             libc::c_uint).wrapping_mul(z).wrapping_add(c
                                                                                                            as
                                                                                                            libc::c_uint)
                                                }
                                                nz1 = 0 as libc::c_int;
                                                nz = nz1
                                            }
                                            s = s.offset(1);
                                            c = *s as libc::c_int;
                                            current_block =
                                                6476622998065200121;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    e = 0 as libc::c_int;
                    if c == 'e' as i32 || c == 'E' as i32 {
                        if nd == 0 && nz == 0 && nz0 == 0 {
                            current_block = 2153641082331438319;
                        } else {
                            s00 = s;
                            esign = 0 as libc::c_int;
                            let mut current_block_49: u64;
                            s = s.offset(1);
                            c = *s as libc::c_int;
                            match c {
                                45 => {
                                    esign = 1 as libc::c_int;
                                    current_block_49 = 4005560482714005869;
                                }
                                43 => {
                                    current_block_49 = 4005560482714005869;
                                }
                                _ => {
                                    current_block_49 = 5873035170358615968;
                                }
                            }
                            match current_block_49 {
                                4005560482714005869 => {
                                    s = s.offset(1);
                                    c = *s as libc::c_int
                                }
                                _ => { }
                            }
                            if c >= '0' as i32 && c <= '9' as i32 {
                                while c == '0' as i32 {
                                    s = s.offset(1);
                                    c = *s as libc::c_int
                                }
                                if c > '0' as i32 && c <= '9' as i32 {
                                    L = c - '0' as i32;
                                    s1 = s;
                                    loop  {
                                        s = s.offset(1);
                                        c = *s as libc::c_int;
                                        if !(c >= '0' as i32 &&
                                                 c <= '9' as i32) {
                                            break ;
                                        }
                                        L =
                                            10 as libc::c_int * L + c -
                                                '0' as i32
                                    }
                                    if s.wrapping_offset_from(s1) as
                                           libc::c_long >
                                           8 as libc::c_int as libc::c_long ||
                                           L > 19999 as libc::c_int {
                                        /* Avoid confusion from exponents
					 * so large that e might overflow.
					 */
                                        e = 19999 as libc::c_int
                                    } else {
                                        e = L
                                    } /* safe for 16 bit ints */
                                    if esign != 0 { e = -e }
                                } else { e = 0 as libc::c_int }
                            } else { s = s00 }
                            current_block = 10887629115603254199;
                        }
                    } else { current_block = 10887629115603254199; }
                    match current_block {
                        2153641082331438319 => { }
                        _ => {
                            if nd == 0 {
                                if nz == 0 && nz0 == 0 {
                                    /* Check for Nan and Infinity */
                                    if bc.dplen == 0 {
                                        match c {
                                            105 | 73 => {
                                                current_block =
                                                    11929542598567748919;
                                                match current_block {
                                                    7028913789814249072 => {
                                                        if match_0(C, &mut s,
                                                                   b"an\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
                                                               != 0 {
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0x7ff80000 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            current_block =
                                                                4077676600814058918;
                                                        } else {
                                                            current_block =
                                                                2153641082331438319;
                                                        }
                                                    }
                                                    _ => {
                                                        if match_0(C, &mut s,
                                                                   b"nf\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
                                                               != 0 {
                                                            s = s.offset(-1);
                                                            if match_0(C,
                                                                       &mut s,
                                                                       b"inity\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char)
                                                                   == 0 {
                                                                s =
                                                                    s.offset(1)
                                                            }
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0x7ff00000 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            current_block =
                                                                4077676600814058918;
                                                        } else {
                                                            current_block =
                                                                2153641082331438319;
                                                        }
                                                    }
                                                }
                                            }
                                            110 | 78 => {
                                                current_block =
                                                    7028913789814249072;
                                                match current_block {
                                                    7028913789814249072 => {
                                                        if match_0(C, &mut s,
                                                                   b"an\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
                                                               != 0 {
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0x7ff80000 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            current_block =
                                                                4077676600814058918;
                                                        } else {
                                                            current_block =
                                                                2153641082331438319;
                                                        }
                                                    }
                                                    _ => {
                                                        if match_0(C, &mut s,
                                                                   b"nf\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
                                                               != 0 {
                                                            s = s.offset(-1);
                                                            if match_0(C,
                                                                       &mut s,
                                                                       b"inity\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char)
                                                                   == 0 {
                                                                s =
                                                                    s.offset(1)
                                                            }
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0x7ff00000 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as ULong;
                                                            current_block =
                                                                4077676600814058918;
                                                        } else {
                                                            current_block =
                                                                2153641082331438319;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                current_block =
                                                    2153641082331438319;
                                            }
                                        }
                                    } else {
                                        current_block = 2153641082331438319;
                                    }
                                } else {
                                    current_block = 4077676600814058918;
                                }
                            } else {
                                e -= nf;
                                e1 = e;
                                bc.e0 = e1;
                                /* Now we have nd0 digits, starting at s0, followed by a
	 * decimal point, followed by nd-nd0 digits.  The number we're
	 * after is the integer represented by those digits times
	 * 10**e */
                                if nd0 == 0 { nd0 = nd }
                                k =
                                    if nd <
                                           15 as libc::c_int +
                                               1 as libc::c_int {
                                        nd
                                    } else {
                                        (15 as libc::c_int) + 1 as libc::c_int
                                    };
                                rv.d = y as libc::c_double;
                                if k > 9 as libc::c_int {
                                    rv.d =
                                        tens[(k - 9 as libc::c_int) as usize]
                                            * rv.d + z as libc::c_double
                                }
                                bd0 = 0 as *mut Bigint;
                                if nd <= 15 as libc::c_int &&
                                       1i32 == 1 as libc::c_int {
                                    if e == 0 {
                                        current_block = 4077676600814058918;
                                    } else if e > 0 as libc::c_int {
                                        if e <= 22 as libc::c_int {
                                            /* rv = */
                                            rv.d *= tens[e as usize];
                                            current_block =
                                                4077676600814058918;
                                        } else {
                                            i = 15 as libc::c_int - nd;
                                            if e <= 22 as libc::c_int + i {
                                                /* A fancier test would sometimes let us do
				 * this for larger i values.
				 */
                                                e -= i;
                                                rv.d *= tens[i as usize];
                                                /* rv = */
                                                rv.d *= tens[e as usize];
                                                current_block =
                                                    4077676600814058918;
                                            } else {
                                                current_block =
                                                    12267375779178892496;
                                            }
                                        }
                                    } else if e >= -(22 as libc::c_int) {
                                        /* rv = */
                                        rv.d /= tens[-e as usize];
                                        current_block = 4077676600814058918;
                                    } else {
                                        current_block = 12267375779178892496;
                                    }
                                    /* ROUND_BIASED_without_Round_Up */
                                } else {
                                    current_block = 12267375779178892496;
                                }
                                match current_block {
                                    4077676600814058918 => { }
                                    _ => {
                                        e1 += nd - k;
                                        bc.scale = 0 as libc::c_int;
                                        /*IEEE_Arith*/
                                        /* Get starting approximation = rv * 10**e1 */
                                        if e1 > 0 as libc::c_int {
                                            i = e1 & 15 as libc::c_int;
                                            if i != 0 {
                                                rv.d *= tens[i as usize]
                                            }
                                            e1 &= !(15 as libc::c_int);
                                            if e1 != 0 {
                                                if e1 > 308 as libc::c_int {
                                                    current_block =
                                                        11336053975954288757;
                                                } else {
                                                    e1 >>= 4 as libc::c_int;
                                                    j = 0 as libc::c_int;
                                                    while e1 >
                                                              1 as libc::c_int
                                                          {
                                                        if e1 &
                                                               1 as
                                                                   libc::c_int
                                                               != 0 {
                                                            rv.d *=
                                                                bigtens[j as
                                                                            usize]
                                                        }
                                                        j += 1;
                                                        e1 >>=
                                                            1 as libc::c_int
                                                    }
                                                    /* The last multiplication could overflow. */
                                                    rv.L[1 as libc::c_int as
                                                             usize] =
                                                        (rv.L[1 as libc::c_int
                                                                  as usize] as
                                                             libc::c_uint).wrapping_sub((53
                                                                                             as
                                                                                             libc::c_int
                                                                                             *
                                                                                             0x100000
                                                                                                 as
                                                                                                 libc::c_int)
                                                                                            as
                                                                                            libc::c_uint)
                                                            as ULong as ULong;
                                                    rv.d *=
                                                        bigtens[j as usize];
                                                    z =
                                                        rv.L[1 as libc::c_int
                                                                 as usize] &
                                                            0x7ff00000 as
                                                                libc::c_int as
                                                                libc::c_uint;
                                                    if z >
                                                           (0x100000 as
                                                                libc::c_int *
                                                                (1024 as
                                                                     libc::c_int
                                                                     +
                                                                     1023 as
                                                                         libc::c_int
                                                                     -
                                                                     53 as
                                                                         libc::c_int))
                                                               as libc::c_uint
                                                       {
                                                        current_block =
                                                            11336053975954288757;
                                                    } else {
                                                        if z >
                                                               (0x100000 as
                                                                    libc::c_int
                                                                    *
                                                                    (1024 as
                                                                         libc::c_int
                                                                         +
                                                                         1023
                                                                             as
                                                                             libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int
                                                                         -
                                                                         53 as
                                                                             libc::c_int))
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            /* set to largest number */
				/* (Can't trust DBL_MAX) */
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                (0xfffff as
                                                                     libc::c_int
                                                                     |
                                                                     0x100000
                                                                         as
                                                                         libc::c_int
                                                                         *
                                                                         (1024
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              1023
                                                                                  as
                                                                                  libc::c_int
                                                                              -
                                                                              1
                                                                                  as
                                                                                  libc::c_int))
                                                                    as ULong;
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0xffffffff as
                                                                    libc::c_uint
                                                        } else {
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                (rv.L[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                                     as
                                                                     libc::c_uint).wrapping_add((53
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     *
                                                                                                     0x100000
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                    as ULong
                                                                    as ULong
                                                        }
                                                        current_block =
                                                            17418136423408909163;
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    17418136423408909163;
                                            }
                                        } else if e1 < 0 as libc::c_int {
                                            e1 = -e1;
                                            i = e1 & 15 as libc::c_int;
                                            if i != 0 {
                                                rv.d /= tens[i as usize]
                                            }
                                            e1 >>= 4 as libc::c_int;
                                            if e1 != 0 {
                                                if e1 >=
                                                       (1 as libc::c_int) <<
                                                           5 as libc::c_int {
                                                    current_block =
                                                        1748757789788325135;
                                                } else {
                                                    if e1 &
                                                           0x10 as libc::c_int
                                                           != 0 {
                                                        bc.scale =
                                                            2 as libc::c_int *
                                                                53 as
                                                                    libc::c_int
                                                    }
                                                    j = 0 as libc::c_int;
                                                    while e1 >
                                                              0 as libc::c_int
                                                          {
                                                        if e1 &
                                                               1 as
                                                                   libc::c_int
                                                               != 0 {
                                                            rv.d *=
                                                                tinytens[j as
                                                                             usize]
                                                        }
                                                        j += 1;
                                                        e1 >>=
                                                            1 as libc::c_int
                                                    }
                                                    if bc.scale != 0 &&
                                                           {
                                                               j =
                                                                   ((2 as
                                                                         libc::c_int
                                                                         *
                                                                         53 as
                                                                             libc::c_int
                                                                         +
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        libc::c_uint).wrapping_sub((rv.L[1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                                                                        &
                                                                                                        0x7ff00000
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint)
                                                                                                       >>
                                                                                                       20
                                                                                                           as
                                                                                                           libc::c_int)
                                                                       as
                                                                       libc::c_int;
                                                               (j) >
                                                                   0 as
                                                                       libc::c_int
                                                           } {
                                                        /* scaled rv is denormal; clear j low bits */
                                                        if j >=
                                                               32 as
                                                                   libc::c_int
                                                           {
                                                            if j >
                                                                   54 as
                                                                       libc::c_int
                                                               {
                                                                current_block
                                                                    =
                                                                    1748757789788325135;
                                                            } else {
                                                                rv.L[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    =
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        ULong;
                                                                if j >=
                                                                       53 as
                                                                           libc::c_int
                                                                   {
                                                                    rv.L[1 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                                        =
                                                                        ((53
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              2
                                                                                  as
                                                                                  libc::c_int)
                                                                             *
                                                                             0x100000
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            ULong
                                                                } else {
                                                                    rv.L[1 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                                        &=
                                                                        (0xffffffff
                                                                             as
                                                                             libc::c_uint)
                                                                            <<
                                                                            j
                                                                                -
                                                                                32
                                                                                    as
                                                                                    libc::c_int
                                                                }
                                                                current_block
                                                                    =
                                                                    13796196565926322821;
                                                            }
                                                        } else {
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                &=
                                                                (0xffffffff as
                                                                     libc::c_uint)
                                                                    << j;
                                                            current_block =
                                                                13796196565926322821;
                                                        }
                                                    } else {
                                                        current_block =
                                                            13796196565926322821;
                                                    }
                                                    match current_block {
                                                        1748757789788325135 =>
                                                        {
                                                        }
                                                        _ => {
                                                            if rv.d == 0. {
                                                                current_block
                                                                    =
                                                                    1748757789788325135;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    17418136423408909163;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    17418136423408909163;
                                            }
                                        } else {
                                            current_block =
                                                17418136423408909163;
                                        }
                                        match current_block {
                                            17418136423408909163 => {
                                                /* Now the hard part -- adjusting rv to the correct value.*/
                                                /* Put digits into bd: true value = bd * 10^e */
                                                bc.nd =
                                                    nd -
                                                        nz1; /* Only needed if nd > strtod_diglim, but done here */
                                                bc.nd0 = nd0;
                                                /* to silence an erroneous warning about bc.nd0 */
			/* possibly not being initialized. */
                                                if nd > 40 as libc::c_int {
                                                    /* ASSERT(strtod_diglim >= 18); 18 == one more than the */
		/* minimum number of decimal digits to distinguish double values */
		/* in IEEE arithmetic. */
                                                    j = 18 as libc::c_int;
                                                    i = j;
                                                    if i > nd0 {
                                                        j += bc.dplen
                                                    }
                                                    loop  {
                                                        j -= 1;
                                                        if j < bc.dp1 &&
                                                               j >= bc.dp0 {
                                                            j =
                                                                bc.dp0 -
                                                                    1 as
                                                                        libc::c_int
                                                        }
                                                        if *s0.offset(j as
                                                                          isize)
                                                               as libc::c_int
                                                               != '0' as i32 {
                                                            break ;
                                                        }
                                                        i -= 1
                                                    }
                                                    e += nd - i;
                                                    nd = i;
                                                    if nd0 > nd { nd0 = nd }
                                                    if nd < 9 as libc::c_int {
                                                        /* must recompute y */
                                                        y =
                                                            0 as libc::c_int
                                                                as
                                                                ULong; /* rv = bb * 2^bbe */
                                                        i =
                                                            0 as
                                                                libc::c_int; /* logb(rv) */
                                                        while i < nd0 {
                                                            y =
                                                                (10 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint).wrapping_mul(y).wrapping_add(*s0.offset(i
                                                                                                                               as
                                                                                                                               isize)
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_sub('0'
                                                                                                                                                   as
                                                                                                                                                   i32
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint);
                                                            i += 1
                                                        }
                                                        j = bc.dp1;
                                                        while i < nd {
                                                            let fresh29 = j;
                                                            j = j + 1;
                                                            y =
                                                                (10 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint).wrapping_mul(y).wrapping_add(*s0.offset(fresh29
                                                                                                                               as
                                                                                                                               isize)
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_sub('0'
                                                                                                                                                   as
                                                                                                                                                   i32
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint);
                                                            i += 1
                                                        }
                                                    }
                                                }
                                                bd0 =
                                                    s2b(C, s0, nd0, nd, y,
                                                        bc.dplen);
                                                loop  {
                                                    bd = Balloc(C, (*bd0).k);
                                                    memcpy(&mut (*bd).sign as
                                                               *mut libc::c_int
                                                               as
                                                               *mut libc::c_char
                                                               as
                                                               *mut libc::c_void,
                                                           &mut (*bd0).sign as
                                                               *mut libc::c_int
                                                               as
                                                               *mut libc::c_char
                                                               as
                                                               *const libc::c_void,
                                                           ((*bd0).wds as
                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                                as
                                                                                                libc::c_ulong).wrapping_add((2
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong)));
                                                    bb =
                                                        d2b(C, &mut rv,
                                                            &mut bbe,
                                                            &mut bbbits);
                                                    bs =
                                                        i2b(C,
                                                            1 as libc::c_int);
                                                    if e >= 0 as libc::c_int {
                                                        bb5 =
                                                            0 as libc::c_int;
                                                        bb2 = bb5;
                                                        bd5 = e;
                                                        bd2 = bd5
                                                    } else {
                                                        bb5 = -e;
                                                        bb2 = bb5;
                                                        bd5 =
                                                            0 as libc::c_int;
                                                        bd2 = bd5
                                                    }
                                                    if bbe >= 0 as libc::c_int
                                                       {
                                                        bb2 += bbe
                                                    } else { bd2 -= bbe }
                                                    bs2 = bb2;
                                                    Lsb =
                                                        1 as libc::c_int as
                                                            ULong;
                                                    Lsb1 =
                                                        0 as libc::c_int as
                                                            ULong;
                                                    j = bbe - bc.scale;
                                                    i =
                                                        j + bbbits -
                                                            1 as libc::c_int;
                                                    j =
                                                        53 as libc::c_int +
                                                            1 as libc::c_int -
                                                            bbbits;
                                                    if i <
                                                           -(1022 as
                                                                 libc::c_int)
                                                       {
                                                        /* denormal */
                                                        i =
                                                            -(1022 as
                                                                  libc::c_int)
                                                                - i;
                                                        j -= i;
                                                        if i <
                                                               32 as
                                                                   libc::c_int
                                                           {
                                                            Lsb <<= i
                                                        } else if i <
                                                                      52 as
                                                                          libc::c_int
                                                         {
                                                            Lsb1 =
                                                                Lsb <<
                                                                    i -
                                                                        32 as
                                                                            libc::c_int
                                                        } else {
                                                            Lsb1 =
                                                                0x7ff00000 as
                                                                    libc::c_int
                                                                    as ULong
                                                        }
                                                    }
                                                    /*Avoid_Underflow*/
                                                    /*Avoid_Underflow*/
                                                    bb2 += j;
                                                    bd2 += j;
                                                    bd2 += bc.scale;
                                                    i =
                                                        if bb2 < bd2 {
                                                            bb2
                                                        } else { bd2 };
                                                    if i > bs2 { i = bs2 }
                                                    if i > 0 as libc::c_int {
                                                        bb2 -= i;
                                                        bd2 -= i;
                                                        bs2 -= i
                                                    }
                                                    if bb5 > 0 as libc::c_int
                                                       {
                                                        bs =
                                                            pow5mult(C, bs,
                                                                     bb5);
                                                        bb1 = mult(C, bs, bb);
                                                        Bfree(C, bb);
                                                        bb = bb1
                                                    }
                                                    if bb2 > 0 as libc::c_int
                                                       {
                                                        bb =
                                                            lshift(C, bb, bb2)
                                                    }
                                                    if bd5 > 0 as libc::c_int
                                                       {
                                                        bd =
                                                            pow5mult(C, bd,
                                                                     bd5)
                                                    }
                                                    if bd2 > 0 as libc::c_int
                                                       {
                                                        bd =
                                                            lshift(C, bd, bd2)
                                                    }
                                                    if bs2 > 0 as libc::c_int
                                                       {
                                                        bs =
                                                            lshift(C, bs, bs2)
                                                    }
                                                    delta = diff(C, bb, bd);
                                                    bc.dsign = (*delta).sign;
                                                    (*delta).sign =
                                                        0 as libc::c_int;
                                                    i = cmp(C, delta, bs);
                                                    /*{*/
                                                    if bc.nd > nd &&
                                                           i <=
                                                               0 as
                                                                   libc::c_int
                                                       {
                                                        if bc.dsign != 0 {
                                                            /* Must use bigcomp(C, ). */
                                                            req_bigcomp =
                                                                1 as
                                                                    libc::c_int;
                                                            current_block =
                                                                2717283593422511518;
                                                            break ;
                                                        } else {
                                                            i =
                                                                -(1 as
                                                                      libc::c_int)
                                                        }
                                                        /* Discarded digits make delta smaller. */
                                                    }
                                                    /*}*/
                                                    /*{*/
                                                    /*}Honor_FLT_ROUNDS*/
                                                    if i < 0 as libc::c_int {
                                                        /* Error is less than half an ulp -- check for
			 * special case of mantissa a power of two.
			 */
                                                        if bc.dsign != 0 ||
                                                               rv.L[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                                   != 0 ||
                                                               rv.L[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                                   &
                                                                   0xfffff as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                                   != 0 ||
                                                               rv.L[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                                   &
                                                                   0x7ff00000
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                                   <=
                                                                   ((2 as
                                                                         libc::c_int
                                                                         *
                                                                         53 as
                                                                             libc::c_int
                                                                         +
                                                                         1 as
                                                                             libc::c_int)
                                                                        *
                                                                        0x100000
                                                                            as
                                                                            libc::c_int)
                                                                       as
                                                                       libc::c_uint
                                                           {
                                                            /*}*/
                                                            current_block =
                                                                2717283593422511518;
                                                            break ;
                                                        } else if *(*delta).x.as_mut_ptr().offset(0
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize)
                                                                      == 0 &&
                                                                      (*delta).wds
                                                                          <=
                                                                          1 as
                                                                              libc::c_int
                                                         {
                                                            /* exact result */
                                                            current_block =
                                                                2717283593422511518;
                                                            break ;
                                                        } else {
                                                            delta =
                                                                lshift(C,
                                                                       delta,
                                                                       1 as
                                                                           libc::c_int);
                                                            if !(cmp(C, delta,
                                                                     bs) >
                                                                     0 as
                                                                         libc::c_int)
                                                               {
                                                                current_block
                                                                    =
                                                                    2717283593422511518;
                                                                break ;
                                                            }
                                                        }
                                                        current_block =
                                                            3424850598627274585;
                                                    } else if i ==
                                                                  0 as
                                                                      libc::c_int
                                                     {
                                                        /* exactly half-way between */
                                                        if bc.dsign != 0 {
                                                            if rv.L[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                                   &
                                                                   0xfffff as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                                   ==
                                                                   0xfffff as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                                   &&
                                                                   rv.L[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                                       ==
                                                                       (if bc.scale
                                                                               !=
                                                                               0
                                                                               &&
                                                                               {
                                                                                   y
                                                                                       =
                                                                                       rv.L[1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                                           &
                                                                                           0x7ff00000
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint;
                                                                                   (y)
                                                                                       <=
                                                                                       (2
                                                                                            as
                                                                                            libc::c_int
                                                                                            *
                                                                                            53
                                                                                                as
                                                                                                libc::c_int
                                                                                            *
                                                                                            0x100000
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           libc::c_uint
                                                                               }
                                                                           {
                                                                            (0xffffffff
                                                                                 as
                                                                                 libc::c_uint)
                                                                                &
                                                                                (0xffffffff
                                                                                     as
                                                                                     libc::c_uint)
                                                                                    <<
                                                                                    ((2
                                                                                          as
                                                                                          libc::c_int
                                                                                          *
                                                                                          53
                                                                                              as
                                                                                              libc::c_int
                                                                                          +
                                                                                          1
                                                                                              as
                                                                                              libc::c_int)
                                                                                         as
                                                                                         libc::c_uint).wrapping_sub(y
                                                                                                                        >>
                                                                                                                        20
                                                                                                                            as
                                                                                                                            libc::c_int)
                                                                        } else {
                                                                            0xffffffff
                                                                                as
                                                                                libc::c_uint
                                                                        }) {
                                                                /*boundary case -- increment exponent*/
                                                                if rv.L[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                                       ==
                                                                       (0xfffff
                                                                            as
                                                                            libc::c_int
                                                                            |
                                                                            0x100000
                                                                                as
                                                                                libc::c_int
                                                                                *
                                                                                (1024
                                                                                     as
                                                                                     libc::c_int
                                                                                     +
                                                                                     1023
                                                                                         as
                                                                                         libc::c_int
                                                                                     -
                                                                                     1
                                                                                         as
                                                                                         libc::c_int))
                                                                           as
                                                                           libc::c_uint
                                                                       &&
                                                                       rv.L[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                                           ==
                                                                           0xffffffff
                                                                               as
                                                                               libc::c_uint
                                                                   {
                                                                    current_block
                                                                        =
                                                                        11336053975954288757;
                                                                    break ;
                                                                }
                                                                rv.L[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    =
                                                                    (rv.L[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                                         &
                                                                         0x7ff00000
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint).wrapping_add(0x100000
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint);
                                                                rv.L[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    =
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        ULong;
                                                                bc.dsign =
                                                                    0 as
                                                                        libc::c_int;
                                                                current_block
                                                                    =
                                                                    2717283593422511518;
                                                                break ;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    13976402388691103242;
                                                            }
                                                        } else if rv.L[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                                      &
                                                                      0xfffff
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint
                                                                      == 0 &&
                                                                      rv.L[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                                                          == 0
                                                         {
                                                            current_block =
                                                                3424850598627274585;
                                                        } else {
                                                            current_block =
                                                                13976402388691103242;
                                                        }
                                                        match current_block {
                                                            3424850598627274585
                                                            => {
                                                            }
                                                            _ => {
                                                                if Lsb1 != 0 {
                                                                    if rv.L[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                                           &
                                                                           Lsb1
                                                                           ==
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            2717283593422511518;
                                                                        break
                                                                            ;
                                                                    }
                                                                } else if rv.L[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                              &
                                                                              Lsb
                                                                              ==
                                                                              0
                                                                 {
                                                                    current_block
                                                                        =
                                                                        2717283593422511518;
                                                                    break ;
                                                                }
                                                                if bc.dsign !=
                                                                       0 {
                                                                    rv.d +=
                                                                        sulp(C,
                                                                             &mut rv,
                                                                             &mut bc)
                                                                } else {
                                                                    rv.d -=
                                                                        sulp(C,
                                                                             &mut rv,
                                                                             &mut bc);
                                                                    if rv.d ==
                                                                           0.
                                                                       {
                                                                        if !(bc.nd
                                                                                 >
                                                                                 nd)
                                                                           {
                                                                            current_block
                                                                                =
                                                                                1748757789788325135;
                                                                            break
                                                                                ;
                                                                        }
                                                                        bc.uflchk
                                                                            =
                                                                            1
                                                                                as
                                                                                libc::c_int;
                                                                        current_block
                                                                            =
                                                                            2717283593422511518;
                                                                        break
                                                                            ;
                                                                    }
                                                                }
                                                                bc.dsign =
                                                                    1 as
                                                                        libc::c_int
                                                                        -
                                                                        bc.dsign;
                                                                current_block
                                                                    =
                                                                    2717283593422511518;
                                                                break ;
                                                            }
                                                        }
                                                    } else {
                                                        aadj =
                                                            ratio(C, delta,
                                                                  bs);
                                                        if aadj <= 2.0f64 {
                                                            if bc.dsign != 0 {
                                                                aadj1 =
                                                                    1.0f64;
                                                                aadj = aadj1
                                                            } else if rv.L[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                                                          != 0
                                                                          ||
                                                                          rv.L[1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize]
                                                                              &
                                                                              0xfffff
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint
                                                                              !=
                                                                              0
                                                             {
                                                                if rv.L[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                                       ==
                                                                       1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                                       &&
                                                                       rv.L[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                                           ==
                                                                           0 {
                                                                    if !(bc.nd
                                                                             >
                                                                             nd)
                                                                       {
                                                                        current_block
                                                                            =
                                                                            1748757789788325135;
                                                                        break
                                                                            ;
                                                                    }
                                                                    bc.uflchk
                                                                        =
                                                                        1 as
                                                                            libc::c_int;
                                                                    current_block
                                                                        =
                                                                        2717283593422511518;
                                                                    break ;
                                                                } else {
                                                                    aadj =
                                                                        1.0f64;
                                                                    aadj1 =
                                                                        -1.0f64
                                                                }
                                                            } else {
                                                                /* special case -- power of FLT_RADIX to be */
				/* rounded down... */
                                                                if aadj <
                                                                       2.0f64
                                                                           /
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_double
                                                                   {
                                                                    aadj =
                                                                        1.0f64
                                                                            /
                                                                            2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_double
                                                                } else {
                                                                    aadj *=
                                                                        0.5f64
                                                                }
                                                                aadj1 = -aadj
                                                            }
                                                        } else {
                                                            aadj *= 0.5f64;
                                                            aadj1 =
                                                                if bc.dsign !=
                                                                       0 {
                                                                    aadj
                                                                } else {
                                                                    -aadj
                                                                };
                                                            if 1i32 ==
                                                                   0 as
                                                                       libc::c_int
                                                               {
                                                                aadj1 +=
                                                                    0.5f64
                                                            }
                                                            /*Check_FLT_ROUNDS*/
                                                        }
                                                        y =
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                &
                                                                0x7ff00000 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint;
                                                        /* Check for overflow */
                                                        if y ==
                                                               (0x100000 as
                                                                    libc::c_int
                                                                    *
                                                                    (1024 as
                                                                         libc::c_int
                                                                         +
                                                                         1023
                                                                             as
                                                                             libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int))
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            rv0.d = rv.d;
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                (rv.L[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                                     as
                                                                     libc::c_uint).wrapping_sub((53
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     *
                                                                                                     0x100000
                                                                                                         as
                                                                                                         libc::c_int)
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                    as ULong
                                                                    as ULong;
                                                            adj.d =
                                                                aadj1 *
                                                                    ulp(C,
                                                                        &mut rv);
                                                            rv.d += adj.d;
                                                            if rv.L[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                                                   &
                                                                   0x7ff00000
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                                   >=
                                                                   (0x100000
                                                                        as
                                                                        libc::c_int
                                                                        *
                                                                        (1024
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             1023
                                                                                 as
                                                                                 libc::c_int
                                                                             -
                                                                             53
                                                                                 as
                                                                                 libc::c_int))
                                                                       as
                                                                       libc::c_uint
                                                               {
                                                                if rv0.L[1 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                                       ==
                                                                       (0xfffff
                                                                            as
                                                                            libc::c_int
                                                                            |
                                                                            0x100000
                                                                                as
                                                                                libc::c_int
                                                                                *
                                                                                (1024
                                                                                     as
                                                                                     libc::c_int
                                                                                     +
                                                                                     1023
                                                                                         as
                                                                                         libc::c_int
                                                                                     -
                                                                                     1
                                                                                         as
                                                                                         libc::c_int))
                                                                           as
                                                                           libc::c_uint
                                                                       &&
                                                                       rv0.L[0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                           ==
                                                                           0xffffffff
                                                                               as
                                                                               libc::c_uint
                                                                   {
                                                                    current_block
                                                                        =
                                                                        11336053975954288757;
                                                                    break ;
                                                                }
                                                                rv.L[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    =
                                                                    (0xfffff
                                                                         as
                                                                         libc::c_int
                                                                         |
                                                                         0x100000
                                                                             as
                                                                             libc::c_int
                                                                             *
                                                                             (1024
                                                                                  as
                                                                                  libc::c_int
                                                                                  +
                                                                                  1023
                                                                                      as
                                                                                      libc::c_int
                                                                                  -
                                                                                  1
                                                                                      as
                                                                                      libc::c_int))
                                                                        as
                                                                        ULong;
                                                                rv.L[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    =
                                                                    0xffffffff
                                                                        as
                                                                        libc::c_uint;
                                                                current_block
                                                                    =
                                                                    13145881433301908673;
                                                            } else {
                                                                rv.L[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    =
                                                                    (rv.L[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                                         as
                                                                         libc::c_uint).wrapping_add((53
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         *
                                                                                                         0x100000
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                        as
                                                                        ULong
                                                                        as
                                                                        ULong;
                                                                current_block
                                                                    =
                                                                    13931475854052133560;
                                                            }
                                                        } else {
                                                            if bc.scale != 0
                                                                   &&
                                                                   y <=
                                                                       (2 as
                                                                            libc::c_int
                                                                            *
                                                                            53
                                                                                as
                                                                                libc::c_int
                                                                            *
                                                                            0x100000
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           libc::c_uint
                                                               {
                                                                if aadj <=
                                                                       0x7fffffff
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_double
                                                                   {
                                                                    z =
                                                                        aadj
                                                                            as
                                                                            ULong;
                                                                    if z <=
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                       {
                                                                        z =
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                ULong
                                                                    }
                                                                    aadj =
                                                                        z as
                                                                            libc::c_double;
                                                                    aadj1 =
                                                                        if bc.dsign
                                                                               !=
                                                                               0
                                                                           {
                                                                            aadj
                                                                        } else {
                                                                            -aadj
                                                                        }
                                                                }
                                                                aadj2.d =
                                                                    aadj1;
                                                                aadj2.L[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                                    =
                                                                    (aadj2.L[1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                         as
                                                                         libc::c_uint).wrapping_add((((2
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           *
                                                                                                           53
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                           +
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int)
                                                                                                          *
                                                                                                          0x100000
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_sub(y))
                                                                        as
                                                                        ULong
                                                                        as
                                                                        ULong;
                                                                aadj1 =
                                                                    aadj2.d;
                                                                adj.d =
                                                                    aadj1 *
                                                                        ulp(C,
                                                                            &mut rv);
                                                                rv.d += adj.d;
                                                                if rv.d ==
                                                                       0.0f64
                                                                   {
                                                                    if bc.nd >
                                                                           nd
                                                                       {
                                                                        bc.dsign
                                                                            =
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                    }
                                                                    current_block
                                                                        =
                                                                        2717283593422511518;
                                                                    break ;
                                                                }
                                                            } else {
                                                                adj.d =
                                                                    aadj1 *
                                                                        ulp(C,
                                                                            &mut rv);
                                                                rv.d += adj.d
                                                            }
                                                            current_block =
                                                                13931475854052133560;
                                                            /*Avoid_Underflow*/
                                                        }
                                                        match current_block {
                                                            13145881433301908673
                                                            => {
                                                            }
                                                            _ => {
                                                                z =
                                                                    rv.L[1 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                                        &
                                                                        0x7ff00000
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint;
                                                                if bc.nd == nd
                                                                   {
                                                                    if bc.scale
                                                                           ==
                                                                           0 {
                                                                        if y
                                                                               ==
                                                                               z
                                                                           {
                                                                            /* Can we stop now? */
                                                                            L
                                                                                =
                                                                                aadj
                                                                                    as
                                                                                    libc::c_int;
                                                                            aadj
                                                                                -=
                                                                                L
                                                                                    as
                                                                                    libc::c_double;
                                                                            /* The tolerances below are conservative. */
                                                                            if bc.dsign
                                                                                   !=
                                                                                   0
                                                                                   ||
                                                                                   rv.L[0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            usize]
                                                                                       !=
                                                                                       0
                                                                                   ||
                                                                                   rv.L[1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            usize]
                                                                                       &
                                                                                       0xfffff
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint
                                                                                       !=
                                                                                       0
                                                                               {
                                                                                if aadj
                                                                                       <
                                                                                       0.4999999f64
                                                                                       ||
                                                                                       aadj
                                                                                           >
                                                                                           0.5000001f64
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        2717283593422511518;
                                                                                    break
                                                                                        ;
                                                                                }
                                                                            } else if aadj
                                                                                          <
                                                                                          0.4999999f64
                                                                                              /
                                                                                              2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_double
                                                                             {
                                                                                current_block
                                                                                    =
                                                                                    2717283593422511518;
                                                                                break
                                                                                    ;
                                                                            }
                                                                            current_block
                                                                                =
                                                                                13145881433301908673;
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                13145881433301908673;
                                                                        }
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            13145881433301908673;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        13145881433301908673;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    match current_block {
                                                        3424850598627274585 =>

                                                        /* boundary case -- decrement exponent */
                                                        /*{{*/
                                                        /*Sudden_Underflow}{*/
                                                        {
                                                            if bc.scale != 0 {
                                                                L =
                                                                    (rv.L[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                                         &
                                                                         0x7ff00000
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                                        as
                                                                        libc::c_int;
                                                                if L <=
                                                                       (2 as
                                                                            libc::c_int
                                                                            *
                                                                            53
                                                                                as
                                                                                libc::c_int
                                                                            +
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                           *
                                                                           0x100000
                                                                               as
                                                                               libc::c_int
                                                                   {
                                                                    if L >
                                                                           (53
                                                                                as
                                                                                libc::c_int
                                                                                +
                                                                                2
                                                                                    as
                                                                                    libc::c_int)
                                                                               *
                                                                               0x100000
                                                                                   as
                                                                                   libc::c_int
                                                                       {
                                                                        current_block
                                                                            =
                                                                            2717283593422511518;
                                                                        break
                                                                            ;
                                                                    }
                                                                    /* rv = smallest denormal */
                                                                    if !(bc.nd
                                                                             >
                                                                             nd)
                                                                       {
                                                                        current_block
                                                                            =
                                                                            1748757789788325135;
                                                                        break
                                                                            ;
                                                                    }
                                                                    bc.uflchk
                                                                        =
                                                                        1 as
                                                                            libc::c_int;
                                                                    current_block
                                                                        =
                                                                        2717283593422511518;
                                                                    break ;
                                                                }
                                                            }
                                                            /*Avoid_Underflow*/
                                                            L =
                                                                (rv.L[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                                     &
                                                                     0x7ff00000
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint).wrapping_sub(0x100000
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                    as
                                                                    libc::c_int;
                                                            /*Sudden_Underflow}}*/
                                                            rv.L[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                (L |
                                                                     0xfffff
                                                                         as
                                                                         libc::c_int)
                                                                    as ULong;
                                                            rv.L[0 as
                                                                     libc::c_int
                                                                     as usize]
                                                                =
                                                                0xffffffff as
                                                                    libc::c_uint;
                                                            if !(bc.nd > nd) {
                                                                current_block
                                                                    =
                                                                    2717283593422511518;
                                                                break ;
                                                            }
                                                        }
                                                        _ => { }
                                                    }
                                                    Bfree(C, bb);
                                                    Bfree(C, bd);
                                                    Bfree(C, bs);
                                                    Bfree(C, delta);
                                                }
                                                match current_block {
                                                    1748757789788325135 => { }
                                                    11336053975954288757 => {
                                                    }
                                                    _ =>
                                                    /* round even ==> */
							/* accept rv */
                                                    {
                                                        Bfree(C, bb);
                                                        Bfree(C, bd);
                                                        Bfree(C, bs);
                                                        Bfree(C, bd0);
                                                        Bfree(C, delta);
                                                        if req_bigcomp != 0 {
                                                            bd0 =
                                                                0 as
                                                                    *mut Bigint;
                                                            bc.e0 += nz1;
                                                            bigcomp(C,
                                                                    &mut rv,
                                                                    s0,
                                                                    &mut bc);
                                                            y =
                                                                rv.L[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                                                    &
                                                                    0x7ff00000
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint;
                                                            if y ==
                                                                   0x7ff00000
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                               {
                                                                current_block
                                                                    =
                                                                    11336053975954288757;
                                                            } else if y ==
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint
                                                                          &&
                                                                          rv.d
                                                                              ==
                                                                              0.0f64
                                                             {
                                                                current_block
                                                                    =
                                                                    1748757789788325135;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    2021172840241467368;
                                                            }
                                                        } else {
                                                            current_block =
                                                                2021172840241467368;
                                                        }
                                                        match current_block {
                                                            1748757789788325135
                                                            => {
                                                            }
                                                            11336053975954288757
                                                            => {
                                                            }
                                                            _ => {
                                                                if bc.scale !=
                                                                       0 {
                                                                    rv0.L[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                                        =
                                                                        (0x3ff00000
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             2
                                                                                 as
                                                                                 libc::c_int
                                                                                 *
                                                                                 53
                                                                                     as
                                                                                     libc::c_int
                                                                                 *
                                                                                 0x100000
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            ULong;
                                                                    rv0.L[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                                        =
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            ULong;
                                                                    rv.d *=
                                                                        rv0.d
                                                                }
                                                                current_block
                                                                    =
                                                                    4077676600814058918;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => { }
                                        }
                                        match current_block {
                                            4077676600814058918 => { }
                                            _ => {
                                                match current_block {
                                                    1748757789788325135 => {
                                                        rv.d = 0.0f64
                                                    }
                                                    _ => {
                                                        /* Can't trust HUGE_VAL */
                                                        /*Honor_FLT_ROUNDS*/
                                                        rv.L[1 as libc::c_int
                                                                 as usize] =
                                                            0x7ff00000 as
                                                                libc::c_int as
                                                                ULong;
                                                        rv.L[0 as libc::c_int
                                                                 as usize] =
                                                            0 as libc::c_int
                                                                as ULong
                                                    }
                                                }
                                                /*Honor_FLT_ROUNDS*/
                                                /*IEEE_Arith*/
                                                /*IEEE_Arith*/
                                                if !bd0.is_null() {
                                                    Bfree(C, bb);
                                                    Bfree(C, bd);
                                                    Bfree(C, bs);
                                                    Bfree(C, bd0);
                                                    Bfree(C, delta);
                                                }
                                                current_block =
                                                    4077676600814058918;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        2153641082331438319 =>
        /* INFNAN_CHECK */
        {
            s = s00;
            sign = 0 as libc::c_int
        }
        _ => { }
    }
    /* Avoid_Underflow */
    if !se.is_null() { *se = s as *mut libc::c_char }
    return if sign != 0 { -rv.d } else { rv.d };
}
unsafe extern "C" fn rv_alloc(mut C: *mut dtoa_context, mut i: libc::c_int)
 -> *mut libc::c_char {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut r: *mut libc::c_int = 0 as *mut libc::c_int;
    j = ::std::mem::size_of::<ULong>() as libc::c_ulong as libc::c_int;
    k = 0 as libc::c_int;
    while (::std::mem::size_of::<Bigint>() as
               libc::c_ulong).wrapping_sub(::std::mem::size_of::<ULong>() as
                                               libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_int>()
                                                                               as
                                                                               libc::c_ulong)
              as libc::c_int + j <= i {
        k += 1;
        j <<= 1 as libc::c_int
    }
    r = Balloc(C, k) as *mut libc::c_int;
    *r = k;
    return r.offset(1 as libc::c_int as isize) as *mut libc::c_char;
}
unsafe extern "C" fn nrv_alloc(mut C: *mut dtoa_context,
                               mut s: *const libc::c_char,
                               mut rve: *mut *mut libc::c_char,
                               mut n: libc::c_int) -> *mut libc::c_char {
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    rv = rv_alloc(C, n);
    t = rv;
    loop  {
        let fresh30 = s;
        s = s.offset(1);
        *t = *fresh30;
        if !(*t != 0) { break ; }
        t = t.offset(1)
    }
    if !rve.is_null() { *rve = t }
    return rv;
}
/* freedtoa(s) must be used to free values s returned by dtoa
 * when MULTIPLE_THREADS is #defined.  It should be used in all cases,
 * but for consistency with earlier versions of dtoa, it is optional
 * when MULTIPLE_THREADS is not defined.
 */
#[no_mangle]
pub unsafe extern "C" fn jvp_freedtoa(mut C: *mut dtoa_context,
                                      mut s: *mut libc::c_char) {
    let mut b: *mut Bigint =
        (s as *mut libc::c_int).offset(-(1 as libc::c_int as isize)) as
            *mut Bigint;
    (*b).k = *(b as *mut libc::c_int);
    (*b).maxwds = (1 as libc::c_int) << (*b).k;
    Bfree(C, b);
}
/* dtoa for IEEE arithmetic (dmg): convert double to ASCII string.
 *
 * Inspired by "How to Print Floating-Point Numbers Accurately" by
 * Guy L. Steele, Jr. and Jon L. White [Proc. ACM SIGPLAN '90, pp. 112-126].
 *
 * Modifications:
 *	1. Rather than iterating, we use a simple numeric overestimate
 *	   to determine k = floor(log10(d)).  We scale relevant
 *	   quantities using O(log2(k)) rather than O(k) multiplications.
 *	2. For some modes > 2 (corresponding to ecvt and fcvt), we don't
 *	   try to generate digits strictly left to right.  Instead, we
 *	   compute with fewer bits and propagate the carry if necessary
 *	   when rounding the final digit up.  This is often faster.
 *	3. Under the assumption that input will be rounded nearest,
 *	   mode 0 renders 1e23 as 1e23 rather than 9.999999999999999e22.
 *	   That is, we allow equality in stopping tests when the
 *	   round-nearest rule will give the same floating-point value
 *	   as would satisfaction of the stopping test with strict
 *	   inequality.
 *	4. We remove common factors of powers of 2 from relevant
 *	   quantities.
 *	5. When converting floating-point integers less than 1e16,
 *	   we use floating-point arithmetic rather than resorting
 *	   to multiple-precision integers.
 *	6. When asked to produce fewer than 15 digits, we first try
 *	   to get by with floating-point arithmetic; we resort to
 *	   multiple-precision integer arithmetic only if we cannot
 *	   guarantee that the floating-point calculation has given
 *	   the correctly rounded result.  For k requested digits and
 *	   "uniformly" distributed input, the probability is
 *	   something like 10^(k-15) that we must resort to the Long
 *	   calculation.
 */
#[no_mangle]
pub unsafe extern "C" fn jvp_dtoa(mut C: *mut dtoa_context,
                                  mut dd: libc::c_double,
                                  mut mode: libc::c_int,
                                  mut ndigits: libc::c_int,
                                  mut decpt: *mut libc::c_int,
                                  mut sign: *mut libc::c_int,
                                  mut rve: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut current_block: u64;
    /*	Arguments ndigits, decpt, sign are similar to those
	of ecvt and fcvt; trailing zeros are suppressed from
	the returned string.  If not null, *rve is set to point
	to the end of the return value.  If d is +-Infinity or NaN,
	then *decpt is set to 9999.

	mode:
		0 ==> shortest string that yields d when read in
			and rounded to nearest.
		1 ==> like 0, but with Steele & White stopping rule;
			e.g. with IEEE P754 arithmetic , mode 0 gives
			1e23 whereas mode 1 gives 9.999999999999999e22.
		2 ==> max(1,ndigits) significant digits.  This gives a
			return value similar to that of ecvt, except
			that trailing zeros are suppressed.
		3 ==> through ndigits past the decimal point.  This
			gives a return value similar to that from fcvt,
			except that trailing zeros are suppressed, and
			ndigits can be negative.
		4,5 ==> similar to 2 and 3, respectively, but (in
			round-nearest mode) with the tests of mode 0 to
			possibly return a shorter string that rounds to d.
			With IEEE arithmetic and compilation with
			-DHonor_FLT_ROUNDS, modes 4 and 5 behave the same
			as modes 2 and 3 when FLT_ROUNDS != 1.
		6-9 ==> Debugging modes similar to mode - 4:  don't try
			fast floating-point estimate (if applicable).

		Values of mode other than 0-9 are treated as mode 0.

		Sufficient space is allocated to the return value
		to hold the suppressed trailing zeros.
	*/
    let mut bbits: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut b5: libc::c_int = 0;
    let mut be: libc::c_int = 0;
    let mut dig: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ieps: libc::c_int = 0;
    let mut ilim: libc::c_int = 0;
    let mut ilim0: libc::c_int = 0;
    let mut ilim1: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j1: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut k0: libc::c_int = 0;
    let mut k_check: libc::c_int = 0;
    let mut leftright: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut m5: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut s5: libc::c_int = 0;
    let mut spec_case: libc::c_int = 0;
    let mut try_quick: libc::c_int = 0;
    let mut L: libc::c_int = 0;
    let mut denorm: libc::c_int = 0;
    let mut x: ULong = 0;
    let mut b: *mut Bigint = 0 as *mut Bigint;
    let mut b1: *mut Bigint = 0 as *mut Bigint;
    let mut delta: *mut Bigint = 0 as *mut Bigint;
    let mut mlo: *mut Bigint = 0 as *mut Bigint;
    let mut mhi: *mut Bigint = 0 as *mut Bigint;
    let mut S: *mut Bigint = 0 as *mut Bigint;
    let mut d2: U = U{d: 0.,};
    let mut eps: U = U{d: 0.,};
    let mut u: U = U{d: 0.,};
    let mut ds: libc::c_double = 0.;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eps1: U = U{d: 0.,};
    /*{*/
    /*}*/
    u.d = dd;
    if u.L[1 as libc::c_int as usize] & 0x80000000 as libc::c_uint != 0 {
        /* set sign for everything, including 0's and NaNs */
        *sign = 1 as libc::c_int;
        u.L[1 as libc::c_int as usize] &= !(0x80000000 as libc::c_uint)
        /* clear sign bit */
    } else { *sign = 0 as libc::c_int }
    if u.L[1 as libc::c_int as usize] &
           0x7ff00000 as libc::c_int as libc::c_uint ==
           0x7ff00000 as libc::c_int as libc::c_uint {
        /* Infinity or NaN */
        *decpt = 9999 as libc::c_int;
        if u.L[0 as libc::c_int as usize] == 0 &&
               u.L[1 as libc::c_int as usize] &
                   0xfffff as libc::c_int as libc::c_uint == 0 {
            return nrv_alloc(C,
                             b"Infinity\x00" as *const u8 as
                                 *const libc::c_char, rve, 8 as libc::c_int)
        }
        return nrv_alloc(C, b"NaN\x00" as *const u8 as *const libc::c_char,
                         rve, 3 as libc::c_int)
    }
    if u.d == 0. {
        *decpt = 1 as libc::c_int;
        return nrv_alloc(C, b"0\x00" as *const u8 as *const libc::c_char, rve,
                         1 as libc::c_int)
    }
    b = d2b(C, &mut u, &mut be, &mut bbits);
    i =
        (u.L[1 as libc::c_int as usize] >> 20 as libc::c_int &
             (0x7ff00000 as libc::c_int >> 20 as libc::c_int) as libc::c_uint)
            as libc::c_int;
    if i != 0 {
        d2.d = u.d;
        d2.L[1 as libc::c_int as usize] &=
            0xfffff as libc::c_int as libc::c_uint;
        d2.L[1 as libc::c_int as usize] |=
            0x3ff00000 as libc::c_int as libc::c_uint;
        /* log(x)	~=~ log(1.5) + (x-1.5)/1.5
		 * log10(x)	 =  log(x) / log(10)
		 *		~=~ log(1.5)/log(10) + (x-1.5)/(1.5*log(10))
		 * log10(d) = (i-Bias)*log(2)/log(10) + log10(d2)
		 *
		 * This suggests computing an approximation k to log10(d) by
		 *
		 * k = (i - Bias)*0.301029995663981
		 *	+ ( (d2-1.5)*0.289529654602168 + 0.176091259055681 );
		 *
		 * We want k to be too large rather than too small.
		 * The error in the first-order Taylor series approximation
		 * is in our favor, so we just round up the constant enough
		 * to compensate for any error in the multiplication of
		 * (i - Bias) by 0.301029995663981; since |i - Bias| <= 1077,
		 * and 1077 * 0.30103 * 2^-52 ~=~ 7.2e-14,
		 * adding 1e-13 to the constant term more than suffices.
		 * Hence we adjust the constant term to 0.1760912590558.
		 * (We could get a more accurate k by invoking log10,
		 *  but this is probably not worthwhile.)
		 */
        i -= 1023 as libc::c_int;
        denorm = 0 as libc::c_int
    } else {
        /* d is denormalized */
        i =
            bbits + be +
                (1023 as libc::c_int + (53 as libc::c_int - 1 as libc::c_int)
                     - 1 as libc::c_int); /* adjust exponent */
        x =
            if i > 32 as libc::c_int {
                (u.L[1 as libc::c_int as usize] << 64 as libc::c_int - i) |
                    u.L[0 as libc::c_int as usize] >> i - 32 as libc::c_int
            } else {
                (u.L[0 as libc::c_int as usize]) << 32 as libc::c_int - i
            }; /* want k = floor(ds) */
        d2.d = x as libc::c_double;
        d2.L[1 as libc::c_int as usize] =
            (d2.L[1 as libc::c_int as usize] as
                 libc::c_uint).wrapping_sub((31 as libc::c_int *
                                                 0x100000 as libc::c_int) as
                                                libc::c_uint) as ULong as
                ULong;
        i -=
            1023 as libc::c_int + (53 as libc::c_int - 1 as libc::c_int) -
                1 as libc::c_int + 1 as libc::c_int;
        denorm = 1 as libc::c_int
    }
    ds =
        (d2.d - 1.5f64) * 0.289529654602168f64 + 0.1760912590558f64 +
            i as libc::c_double * 0.301029995663981f64;
    k = ds as libc::c_int;
    if ds < 0.0f64 && ds != k as libc::c_double { k -= 1 }
    k_check = 1 as libc::c_int;
    if k >= 0 as libc::c_int && k <= 22 as libc::c_int {
        if u.d < tens[k as usize] { k -= 1 }
        k_check = 0 as libc::c_int
    }
    j = bbits - i - 1 as libc::c_int;
    if j >= 0 as libc::c_int {
        b2 = 0 as libc::c_int;
        s2 = j
    } else { b2 = -j; s2 = 0 as libc::c_int }
    if k >= 0 as libc::c_int {
        b5 = 0 as libc::c_int;
        s5 = k;
        s2 += k
    } else { b2 -= k; b5 = -k; s5 = 0 as libc::c_int }
    if mode < 0 as libc::c_int || mode > 9 as libc::c_int {
        mode = 0 as libc::c_int
    }
    try_quick = 1 as libc::c_int;
    /*SET_INEXACT*/
    if mode > 5 as libc::c_int {
        mode -= 4 as libc::c_int; /* Values for cases 0 and 1; done here to */
        try_quick = 0 as libc::c_int
    }
    leftright = 1 as libc::c_int;
    ilim1 = -(1 as libc::c_int);
    ilim = ilim1;
    let mut current_block_78: u64;
    /* silence erroneous "gcc -Wall" warning. */
    match mode {
        0 | 1 => {
            i = 18 as libc::c_int;
            ndigits = 0 as libc::c_int;
            current_block_78 = 11052029508375673978;
        }
        2 => {
            leftright = 0 as libc::c_int;
            current_block_78 = 2916120083628436309;
        }
        4 => { current_block_78 = 2916120083628436309; }
        3 => {
            leftright = 0 as libc::c_int;
            current_block_78 = 8048257192757430128;
        }
        5 => { current_block_78 = 8048257192757430128; }
        _ => { current_block_78 = 11052029508375673978; }
    }
    match current_block_78 {
        8048257192757430128 =>
        /* no break */
        {
            i = ndigits + k + 1 as libc::c_int;
            ilim = i;
            ilim1 = i - 1 as libc::c_int;
            if i <= 0 as libc::c_int { i = 1 as libc::c_int }
        }
        2916120083628436309 =>
        /* no break */
        {
            if ndigits <= 0 as libc::c_int { ndigits = 1 as libc::c_int }
            i = ndigits;
            ilim1 = i;
            ilim = ilim1
        }
        _ => { }
    }
    s0 = rv_alloc(C, i);
    s = s0;
    if ilim >= 0 as libc::c_int && ilim <= 14 as libc::c_int && try_quick != 0
       {
        /* Try to get by with floating-point arithmetic. */
        i = 0 as libc::c_int; /* conservative */
        d2.d = u.d;
        k0 = k;
        ilim0 = ilim;
        ieps = 2 as libc::c_int;
        if k > 0 as libc::c_int {
            ds = tens[(k & 0xf as libc::c_int) as usize];
            j = k >> 4 as libc::c_int;
            if j & 0x10 as libc::c_int != 0 {
                /* prevent overflows */
                j &= 0x10 as libc::c_int - 1 as libc::c_int;
                u.d /=
                    bigtens[(5 as libc::c_int - 1 as libc::c_int) as usize];
                ieps += 1
            }
            while j != 0 {
                if j & 1 as libc::c_int != 0 {
                    ieps += 1;
                    ds *= bigtens[i as usize]
                }
                j >>= 1 as libc::c_int;
                i += 1
            }
            u.d /= ds
        } else {
            j1 = -k;
            if j1 != 0 {
                u.d *= tens[(j1 & 0xf as libc::c_int) as usize];
                j = j1 >> 4 as libc::c_int;
                while j != 0 {
                    if j & 1 as libc::c_int != 0 {
                        ieps += 1;
                        u.d *= bigtens[i as usize]
                    }
                    j >>= 1 as libc::c_int;
                    i += 1
                }
            }
        }
        if k_check != 0 && u.d < 1.0f64 && ilim > 0 as libc::c_int {
            if ilim1 <= 0 as libc::c_int {
                current_block = 15538154479287176543;
            } else {
                ilim = ilim1;
                k -= 1;
                u.d *= 10.0f64;
                ieps += 1;
                current_block = 8656139126282042408;
            }
        } else { current_block = 8656139126282042408; }
        match current_block {
            8656139126282042408 => {
                eps.d = ieps as libc::c_double * u.d + 7.0f64;
                eps.L[1 as libc::c_int as usize] =
                    (eps.L[1 as libc::c_int as usize] as
                         libc::c_uint).wrapping_sub(((53 as libc::c_int -
                                                          1 as libc::c_int) *
                                                         0x100000 as
                                                             libc::c_int) as
                                                        libc::c_uint) as ULong
                        as ULong;
                if ilim == 0 as libc::c_int {
                    mhi = 0 as *mut Bigint;
                    S = mhi;
                    u.d -= 5.0f64;
                    if u.d > eps.d {
                        current_block = 10550230639224458007;
                    } else if u.d < -eps.d {
                        current_block = 5233031407977168135;
                    } else { current_block = 15538154479287176543; }
                } else if leftright != 0 {
                    /* Use Steele & White method of only
			 * generating digits needed.
			 */
                    eps.d =
                        0.5f64 / tens[(ilim - 1 as libc::c_int) as usize] -
                            eps.d; /* 1.01 allows roundoff in the next few lines */
                    if k0 < 0 as libc::c_int && j1 >= 307 as libc::c_int {
                        eps1.d = 1.01e256f64;
                        eps1.L[1 as libc::c_int as usize] =
                            (eps1.L[1 as libc::c_int as usize] as
                                 libc::c_uint).wrapping_sub((0x100000 as
                                                                 libc::c_int *
                                                                 (1023 as
                                                                      libc::c_int
                                                                      +
                                                                      53 as
                                                                          libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int))
                                                                as
                                                                libc::c_uint)
                                as ULong as ULong;
                        eps1.d *= tens[(j1 & 0xf as libc::c_int) as usize];
                        i = 0 as libc::c_int;
                        j = j1 - 256 as libc::c_int >> 4 as libc::c_int;
                        while j != 0 {
                            if j & 1 as libc::c_int != 0 {
                                eps1.d *= bigtens[i as usize]
                            }
                            j >>= 1 as libc::c_int;
                            i += 1
                        }
                        if eps.d < eps1.d { eps.d = eps1.d }
                    }
                    i = 0 as libc::c_int;
                    loop  {
                        L = u.d as libc::c_int;
                        u.d -= L as libc::c_double;
                        let fresh31 = s;
                        s = s.offset(1);
                        *fresh31 = ('0' as i32 + L) as libc::c_char;
                        if 1.0f64 - u.d < eps.d {
                            current_block = 1726299285531216109;
                            break ;
                        }
                        if u.d < eps.d {
                            current_block = 1239993098926645252;
                            break ;
                        }
                        i += 1;
                        if i >= ilim {
                            current_block = 15538154479287176543;
                            break ;
                        }
                        eps.d *= 10.0f64;
                        u.d *= 10.0f64
                    }
                } else {
                    /* Generate ilim digits, then fix them up. */
                    eps.d *= tens[(ilim - 1 as libc::c_int) as usize];
                    i = 1 as libc::c_int;
                    loop  {
                        L = u.d as libc::c_int;
                        u.d -= L as libc::c_double;
                        if u.d == 0. { ilim = i }
                        let fresh32 = s;
                        s = s.offset(1);
                        *fresh32 = ('0' as i32 + L) as libc::c_char;
                        if i == ilim {
                            if u.d > 0.5f64 + eps.d {
                                current_block = 1726299285531216109;
                                break ;
                            }
                            if !(u.d < 0.5f64 - eps.d) {
                                current_block = 15538154479287176543;
                                break ;
                            }
                            loop  {
                                s = s.offset(-1);
                                if !(*s as libc::c_int == '0' as i32) {
                                    break ;
                                }
                            }
                            s = s.offset(1);
                            current_block = 1239993098926645252;
                            break ;
                        } else { i += 1; u.d *= 10.0f64 }
                    }
                }
            }
            _ => { }
        }
        match current_block {
            1239993098926645252 => { }
            10550230639224458007 => { }
            5233031407977168135 => { }
            1726299285531216109 => { }
            _ => {
                s = s0;
                u.d = d2.d;
                k = k0;
                ilim = ilim0;
                current_block = 18316056106135622027;
            }
        }
    } else { current_block = 18316056106135622027; }
    match current_block {
        18316056106135622027 =>
        /* Do we have a "small" integer? */
        {
            if be >= 0 as libc::c_int && k <= 14 as libc::c_int {
                /* Yes. */
                ds = tens[k as usize];
                if ndigits < 0 as libc::c_int && ilim <= 0 as libc::c_int {
                    mhi = 0 as *mut Bigint;
                    S = mhi;
                    if ilim < 0 as libc::c_int ||
                           u.d <= 5 as libc::c_int as libc::c_double * ds {
                        current_block = 5233031407977168135;
                    } else { current_block = 10550230639224458007; }
                } else {
                    i = 1 as libc::c_int;
                    loop  {
                        L = (u.d / ds) as libc::c_int;
                        u.d -= L as libc::c_double * ds;
                        let fresh33 = s;
                        s = s.offset(1);
                        *fresh33 = ('0' as i32 + L) as libc::c_char;
                        if u.d == 0. {
                            current_block = 1239993098926645252;
                            break ;
                        }
                        if i == ilim {
                            u.d += u.d;
                            if u.d > ds ||
                                   u.d == ds && L & 1 as libc::c_int != 0 {
                                current_block = 1726299285531216109;
                                break ;
                            } else {
                                current_block = 1239993098926645252;
                                break ;
                            }
                        } else { i += 1; u.d *= 10.0f64 }
                    }
                }
            } else {
                m2 = b2;
                m5 = b5;
                mlo = 0 as *mut Bigint;
                mhi = mlo;
                if leftright != 0 {
                    i =
                        if denorm != 0 {
                            (be) +
                                (1023 as libc::c_int +
                                     (53 as libc::c_int - 1 as libc::c_int) -
                                     1 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (1 as libc::c_int + 53 as libc::c_int) - bbits
                        };
                    b2 += i;
                    s2 += i;
                    mhi = i2b(C, 1 as libc::c_int)
                }
                if m2 > 0 as libc::c_int && s2 > 0 as libc::c_int {
                    i = if m2 < s2 { m2 } else { s2 };
                    b2 -= i;
                    m2 -= i;
                    s2 -= i
                }
                if b5 > 0 as libc::c_int {
                    if leftright != 0 {
                        if m5 > 0 as libc::c_int {
                            mhi = pow5mult(C, mhi, m5);
                            b1 = mult(C, mhi, b);
                            Bfree(C, b);
                            b = b1
                        }
                        j = b5 - m5;
                        if j != 0 { b = pow5mult(C, b, j) }
                    } else { b = pow5mult(C, b, b5) }
                }
                S = i2b(C, 1 as libc::c_int);
                if s5 > 0 as libc::c_int { S = pow5mult(C, S, s5) }
                /* Check for special case that d is a normalized power of 2. */
                spec_case = 0 as libc::c_int;
                if mode < 2 as libc::c_int || leftright != 0 {
                    if u.L[0 as libc::c_int as usize] == 0 &&
                           u.L[1 as libc::c_int as usize] &
                               0xfffff as libc::c_int as libc::c_uint == 0 &&
                           u.L[1 as libc::c_int as usize] &
                               (0x7ff00000 as libc::c_int &
                                    !(0x100000 as libc::c_int)) as
                                   libc::c_uint != 0 {
                        /* The special case */
                        b2 += 1 as libc::c_int;
                        s2 += 1 as libc::c_int;
                        spec_case = 1 as libc::c_int
                    }
                }
                /* Arrange for convenient computation of quotients:
	 * shift left if necessary so divisor has 4 leading 0 bits.
	 *
	 * Perhaps we should just compute leading 28 bits of S once
	 * and for all and pass them and a shift to quorem, so it
	 * can do shifts and ors to compute the numerator for q.
	 */
                i = dshift(C, S, s2); /* we botched the k estimate */
                b2 += i;
                m2 += i;
                s2 += i;
                if b2 > 0 as libc::c_int { b = lshift(C, b, b2) }
                if s2 > 0 as libc::c_int { S = lshift(C, S, s2) }
                if k_check != 0 {
                    if cmp(C, b, S) < 0 as libc::c_int {
                        k -= 1;
                        b =
                            multadd(C, b, 10 as libc::c_int,
                                    0 as libc::c_int);
                        if leftright != 0 {
                            mhi =
                                multadd(C, mhi, 10 as libc::c_int,
                                        0 as libc::c_int)
                        }
                        ilim = ilim1
                    }
                }
                if ilim <= 0 as libc::c_int &&
                       (mode == 3 as libc::c_int || mode == 5 as libc::c_int)
                   {
                    if ilim < 0 as libc::c_int ||
                           {
                               S =
                                   multadd(C, S, 5 as libc::c_int,
                                           0 as libc::c_int);
                               (cmp(C, b, S)) <= 0 as libc::c_int
                           } {
                        current_block = 5233031407977168135;
                    } else { current_block = 10550230639224458007; }
                } else {
                    if leftright != 0 {
                        if m2 > 0 as libc::c_int { mhi = lshift(C, mhi, m2) }
                        /* Compute mlo -- check for special case
		 * that d is a normalized power of 2.
		 */
                        mlo = mhi;
                        if spec_case != 0 {
                            mhi = Balloc(C, (*mhi).k);
                            memcpy(&mut (*mhi).sign as *mut libc::c_int as
                                       *mut libc::c_char as *mut libc::c_void,
                                   &mut (*mlo).sign as *mut libc::c_int as
                                       *mut libc::c_char as
                                       *const libc::c_void,
                                   ((*mlo).wds as
                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                        as
                                                                        libc::c_ulong).wrapping_add((2
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)));
                            mhi = lshift(C, mhi, 1 as libc::c_int)
                        }
                        i = 1 as libc::c_int;
                        loop  {
                            dig = quorem(C, b, S) + '0' as i32;
                            /* Do we yet have the shortest decimal string
			 * that will round to d?
			 */
                            j = cmp(C, b, mlo);
                            delta = diff(C, S, mhi);
                            j1 =
                                if (*delta).sign != 0 {
                                    1 as libc::c_int
                                } else { cmp(C, b, delta) };
                            Bfree(C, delta);
                            if j1 == 0 as libc::c_int &&
                                   mode != 1 as libc::c_int &&
                                   u.L[0 as libc::c_int as usize] &
                                       1 as libc::c_int as libc::c_uint == 0 {
                                if !(dig == '9' as i32) {
                                    if j > 0 as libc::c_int { dig += 1 }
                                    let fresh36 = s;
                                    s = s.offset(1);
                                    *fresh36 = dig as libc::c_char;
                                    current_block = 16967581672222079673;
                                    break ;
                                }
                            } else if j < 0 as libc::c_int ||
                                          j == 0 as libc::c_int &&
                                              mode != 1 as libc::c_int &&
                                              u.L[0 as libc::c_int as usize] &
                                                  1 as libc::c_int as
                                                      libc::c_uint == 0 {
                                if *(*b).x.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                                       == 0 && (*b).wds <= 1 as libc::c_int {
                                    current_block = 8432270317622632645;
                                } else if j1 > 0 as libc::c_int {
                                    b = lshift(C, b, 1 as libc::c_int);
                                    j1 = cmp(C, b, S);
                                    if (j1 > 0 as libc::c_int ||
                                            j1 == 0 as libc::c_int &&
                                                dig & 1 as libc::c_int != 0)
                                           &&
                                           {
                                               let fresh37 = dig;
                                               dig = dig + 1;
                                               (fresh37) == '9' as i32
                                           } {
                                        current_block = 11522940221586662047;
                                    } else {
                                        current_block = 8432270317622632645;
                                    }
                                } else {
                                    current_block = 8432270317622632645;
                                }
                                match current_block {
                                    11522940221586662047 => { }
                                    _ => {
                                        let fresh38 = s;
                                        s = s.offset(1);
                                        *fresh38 = dig as libc::c_char;
                                        current_block = 16967581672222079673;
                                        break ;
                                    }
                                }
                            } else if j1 > 0 as libc::c_int {
                                if !(dig == '9' as i32) {
                                    let fresh40 = s;
                                    s = s.offset(1);
                                    *fresh40 =
                                        (dig + 1 as libc::c_int) as
                                            libc::c_char;
                                    current_block = 16967581672222079673;
                                    break ;
                                }
                            } else {
                                let fresh41 = s;
                                s = s.offset(1);
                                *fresh41 = dig as libc::c_char;
                                if i == ilim {
                                    current_block = 3316925224002568308;
                                    break ;
                                }
                                b =
                                    multadd(C, b, 10 as libc::c_int,
                                            0 as libc::c_int);
                                if mlo == mhi {
                                    mhi =
                                        multadd(C, mhi, 10 as libc::c_int,
                                                0 as libc::c_int);
                                    mlo = mhi
                                } else {
                                    mlo =
                                        multadd(C, mlo, 10 as libc::c_int,
                                                0 as libc::c_int);
                                    mhi =
                                        multadd(C, mhi, 10 as libc::c_int,
                                                0 as libc::c_int)
                                }
                                i += 1;
                                continue ;
                            }
                            /*Honor_FLT_ROUNDS*/
                            /* possible if i == 1 */
                            let fresh39 = s;
                            s = s.offset(1);
                            *fresh39 = '9' as i32 as libc::c_char;
                            current_block = 9530373920617481355;
                            break ;
                        }
                    } else {
                        i = 1 as libc::c_int;
                        loop  {
                            dig = quorem(C, b, S) + '0' as i32;
                            let fresh42 = s;
                            s = s.offset(1);
                            *fresh42 = dig as libc::c_char;
                            if *(*b).x.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) == 0 &&
                                   (*b).wds <= 1 as libc::c_int {
                                current_block = 16967581672222079673;
                                break ;
                            }
                            if i >= ilim {
                                current_block = 3316925224002568308;
                                break ;
                            }
                            b =
                                multadd(C, b, 10 as libc::c_int,
                                        0 as libc::c_int);
                            i += 1
                        }
                    }
                    match current_block {
                        16967581672222079673 => { }
                        _ => {
                            match current_block {
                                3316925224002568308 => {
                                    /* Round off last digit */
                                    b = lshift(C, b, 1 as libc::c_int);
                                    j = cmp(C, b, S);
                                    if j > 0 as libc::c_int ||
                                           j == 0 as libc::c_int &&
                                               dig & 1 as libc::c_int != 0 {
                                        current_block = 9530373920617481355;
                                    } else {
                                        loop  {
                                            s = s.offset(-1);
                                            if !(*s as libc::c_int ==
                                                     '0' as i32) {
                                                break ;
                                            }
                                        }
                                        s = s.offset(1);
                                        current_block = 16967581672222079673;
                                    }
                                }
                                _ => { }
                            }
                            match current_block {
                                16967581672222079673 => { }
                                _ => {
                                    loop  {
                                        s = s.offset(-1);
                                        if !(*s as libc::c_int == '9' as i32)
                                           {
                                            current_block =
                                                7121273192085788486;
                                            break ;
                                        }
                                        if !(s == s0) { continue ; }
                                        k += 1;
                                        let fresh43 = s;
                                        s = s.offset(1);
                                        *fresh43 = '1' as i32 as libc::c_char;
                                        current_block = 16967581672222079673;
                                        break ;
                                    }
                                    match current_block {
                                        16967581672222079673 => { }
                                        _ => {
                                            let fresh44 = s;
                                            s = s.offset(1);
                                            *fresh44 += 1;
                                            current_block =
                                                16967581672222079673;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        10550230639224458007 => {
            let fresh35 = s;
            s = s.offset(1);
            *fresh35 = '1' as i32 as libc::c_char;
            k += 1;
            current_block = 16967581672222079673;
        }
        1726299285531216109 => {
            loop  {
                s = s.offset(-1);
                if !(*s as libc::c_int == '9' as i32) { break ; }
                if !(s == s0) { continue ; }
                k += 1;
                *s = '0' as i32 as libc::c_char;
                break ;
            }
            let fresh34 = s;
            s = s.offset(1);
            *fresh34 += 1;
            current_block = 1239993098926645252;
        }
        5233031407977168135 =>
        /* no digits, fcvt style */
        {
            k = -(1 as libc::c_int) - ndigits;
            current_block = 16967581672222079673;
        }
        _ => { }
    }
    match current_block {
        16967581672222079673 => {
            Bfree(C, S);
            if !mhi.is_null() {
                if !mlo.is_null() && mlo != mhi { Bfree(C, mlo); }
                Bfree(C, mhi);
            }
        }
        _ => { }
    }
    Bfree(C, b);
    *s = 0 as libc::c_int as libc::c_char;
    *decpt = k + 1 as libc::c_int;
    if !rve.is_null() { *rve = s }
    return s0;
}
/* ***************************************************************
 *
 * The author of this software is David M. Gay.
 *
 * Copyright (c) 1991, 1996 by Lucent Technologies.
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose without fee is hereby granted, provided that this entire notice
 * is included in all copies of any software which is or includes a copy
 * or modification of this software and in all copies of the supporting
 * documentation for such software.
 *
 * THIS SOFTWARE IS BEING PROVIDED "AS IS", WITHOUT ANY EXPRESS OR IMPLIED
 * WARRANTY.  IN PARTICULAR, NEITHER THE AUTHOR NOR LUCENT MAKES ANY
 * REPRESENTATION OR WARRANTY OF ANY KIND CONCERNING THE MERCHANTABILITY
 * OF THIS SOFTWARE OR ITS FITNESS FOR ANY PARTICULAR PURPOSE.
 *
 ***************************************************************/
/* g_fmt(buf,x) stores the closest decimal approximation to x in buf;
 * it suffices to declare buf
 *	char buf[32];
 */
#[no_mangle]
pub unsafe extern "C" fn jvp_dtoa_fmt(mut C: *mut dtoa_context,
                                      mut b: *mut libc::c_char,
                                      mut x: libc::c_double)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut decpt: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut b0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut se: *mut libc::c_char = 0 as *mut libc::c_char;
    b0 = b;
    s0 =
        jvp_dtoa(C, x, 0 as libc::c_int, 0 as libc::c_int, &mut decpt,
                 &mut sign, &mut se);
    s = s0;
    if sign != 0 {
        let fresh45 = b;
        b = b.offset(1);
        *fresh45 = '-' as i32 as libc::c_char
    }
    if decpt == 9999 as libc::c_int {
        /* Infinity or Nan */
        loop  {
            let fresh46 = s;
            s = s.offset(1);
            let fresh47 = b;
            b = b.offset(1);
            *fresh47 = *fresh46;
            if !(*fresh47 != 0) { break ; }
        }
    } else if decpt <= -(4 as libc::c_int) ||
                  decpt as libc::c_long >
                      se.wrapping_offset_from(s) as libc::c_long +
                          15 as libc::c_int as libc::c_long {
        let fresh48 = s;
        s = s.offset(1);
        let fresh49 = b;
        b = b.offset(1);
        *fresh49 = *fresh48;
        if *s != 0 {
            let fresh50 = b;
            b = b.offset(1);
            *fresh50 = '.' as i32 as libc::c_char;
            loop  {
                let fresh51 = s;
                s = s.offset(1);
                *b = *fresh51;
                if !(*b != 0) { break ; }
                b = b.offset(1)
            }
        }
        let fresh52 = b;
        b = b.offset(1);
        *fresh52 = 'e' as i32 as libc::c_char;
        /* sprintf(b, "%+.2d", decpt - 1); */
        decpt -= 1;
        if decpt < 0 as libc::c_int {
            let fresh53 = b;
            b = b.offset(1);
            *fresh53 = '-' as i32 as libc::c_char;
            decpt = -decpt
        } else {
            let fresh54 = b;
            b = b.offset(1);
            *fresh54 = '+' as i32 as libc::c_char
        }
        j = 2 as libc::c_int;
        k = 10 as libc::c_int;
        while 10 as libc::c_int * k <= decpt {
            j += 1;
            k *= 10 as libc::c_int
        }
        loop  {
            i = decpt / k;
            let fresh55 = b;
            b = b.offset(1);
            *fresh55 = (i + '0' as i32) as libc::c_char;
            j -= 1;
            if j <= 0 as libc::c_int { break ; }
            decpt -= i * k;
            decpt *= 10 as libc::c_int
        }
        *b = 0 as libc::c_int as libc::c_char
    } else if decpt <= 0 as libc::c_int {
        let fresh56 = b;
        b = b.offset(1);
        *fresh56 = '0' as i32 as libc::c_char;
        let fresh57 = b;
        b = b.offset(1);
        *fresh57 = '.' as i32 as libc::c_char;
        while decpt < 0 as libc::c_int {
            let fresh58 = b;
            b = b.offset(1);
            *fresh58 = '0' as i32 as libc::c_char;
            decpt += 1
        }
        loop  {
            let fresh59 = s;
            s = s.offset(1);
            let fresh60 = b;
            b = b.offset(1);
            *fresh60 = *fresh59;
            if !(*fresh60 != 0) { break ; }
        }
    } else {
        loop  {
            let fresh61 = s;
            s = s.offset(1);
            *b = *fresh61;
            if !(*b != 0) { break ; }
            b = b.offset(1);
            decpt -= 1;
            if decpt == 0 as libc::c_int && *s as libc::c_int != 0 {
                let fresh62 = b;
                b = b.offset(1);
                *fresh62 = '.' as i32 as libc::c_char
            }
        }
        while decpt > 0 as libc::c_int {
            let fresh63 = b;
            b = b.offset(1);
            *fresh63 = '0' as i32 as libc::c_char;
            decpt -= 1
        }
        *b = 0 as libc::c_int as libc::c_char
    }
    jvp_freedtoa(C, s0);
    return b0;
}
