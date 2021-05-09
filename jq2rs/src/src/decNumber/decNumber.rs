#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn decContextDefault(_: *mut decContext, _: int32_t) -> *mut decContext;
    #[no_mangle]
    fn decContextSetStatus(_: *mut decContext, _: uint32_t)
     -> *mut decContext;
    /* re-round digits if sticky  */
    #[no_mangle]
    static DECPOWERS: [uint32_t; 10];
}
pub type int32_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulong;
pub type rounding = libc::c_uint;
pub const DEC_ROUND_MAX: rounding = 8;
pub const DEC_ROUND_05UP: rounding = 7;
pub const DEC_ROUND_FLOOR: rounding = 6;
pub const DEC_ROUND_DOWN: rounding = 5;
pub const DEC_ROUND_HALF_DOWN: rounding = 4;
pub const DEC_ROUND_HALF_EVEN: rounding = 3;
pub const DEC_ROUND_HALF_UP: rounding = 2;
pub const DEC_ROUND_UP: rounding = 1;
pub const DEC_ROUND_CEILING: rounding = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decContext {
    pub digits: int32_t,
    pub emax: int32_t,
    pub emin: int32_t,
    pub round: rounding,
    pub traps: uint32_t,
    pub status: uint32_t,
    pub clamp: uint8_t,
}
/* max emax, etc., for math funcs. */
/* Classifications for decimal numbers, aligned with 754 (note that */
  /* 'normal' and 'subnormal' are meaningful only with a decContext   */
  /* or a fixed size format).                                         */
pub type decClass = libc::c_uint;
pub const DEC_CLASS_POS_INF: decClass = 9;
pub const DEC_CLASS_POS_NORMAL: decClass = 8;
pub const DEC_CLASS_POS_SUBNORMAL: decClass = 7;
pub const DEC_CLASS_POS_ZERO: decClass = 6;
pub const DEC_CLASS_NEG_ZERO: decClass = 5;
pub const DEC_CLASS_NEG_SUBNORMAL: decClass = 4;
pub const DEC_CLASS_NEG_NORMAL: decClass = 3;
pub const DEC_CLASS_NEG_INF: decClass = 2;
pub const DEC_CLASS_QNAN: decClass = 1;
pub const DEC_CLASS_SNAN: decClass = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decNumber {
    pub digits: int32_t,
    pub exponent: int32_t,
    pub bits: uint8_t,
    pub lsu: [uint16_t; 1],
}
/* ------------------------------------------------------------------ */
/* Decimal Number arithmetic module                                   */
/* ------------------------------------------------------------------ */
/* Copyright (c) IBM Corporation, 2000, 2009.  All rights reserved.   */
/*                                                                    */
/* This software is made available under the terms of the             */
/* ICU License -- ICU 1.8.1 and later.                                */
/*                                                                    */
/* The description and User's Guide ("The decNumber C Library") for   */
/* this software is called decNumber.pdf.  This document is           */
/* available, together with arithmetic and format specifications,     */
/* testcases, and Web links, on the General Decimal Arithmetic page.  */
/*                                                                    */
/* Please send comments, suggestions, and corrections to the author:  */
/*   mfc@uk.ibm.com                                                   */
/*   Mike Cowlishaw, IBM Fellow                                       */
/*   IBM UK, PO Box 31, Birmingham Road, Warwick CV34 5JL, UK         */
/* ------------------------------------------------------------------ */
/* This module comprises the routines for arbitrary-precision General */
/* Decimal Arithmetic as defined in the specification which may be    */
/* found on the General Decimal Arithmetic pages.  It implements both */
/* the full ('extended') arithmetic and the simpler ('subset')        */
/* arithmetic.                                                        */
/*                                                                    */
/* Usage notes:                                                       */
/*                                                                    */
/* 1. This code is ANSI C89 except:                                   */
/*                                                                    */
/*    a) C99 line comments (double forward slash) are used.  (Most C  */
/*       compilers accept these.  If yours does not, a simple script  */
/*       can be used to convert them to ANSI C comments.)             */
/*                                                                    */
/*    b) Types from C99 stdint.h are used.  If you do not have this   */
/*       header file, see the User's Guide section of the decNumber   */
/*       documentation; this lists the necessary definitions.         */
/*                                                                    */
/*    c) If DECDPUN>4 or DECUSE64=1, the C99 64-bit int64_t and       */
/*       uint64_t types may be used.  To avoid these, set DECUSE64=0  */
/*       and DECDPUN<=4 (see documentation).                          */
/*                                                                    */
/*    The code also conforms to C99 restrictions; in particular,      */
/*    strict aliasing rules are observed.                             */
/*                                                                    */
/* 2. The decNumber format which this library uses is optimized for   */
/*    efficient processing of relatively short numbers; in particular */
/*    it allows the use of fixed sized structures and minimizes copy  */
/*    and move operations.  It does, however, support arbitrary       */
/*    precision (up to 999,999,999 digits) and arbitrary exponent     */
/*    range (Emax in the range 0 through 999,999,999 and Emin in the  */
/*    range -999,999,999 through 0).  Mathematical functions (for     */
/*    example decNumberExp) as identified below are restricted more   */
/*    tightly: digits, emax, and -emin in the context must be <=      */
/*    DEC_MAX_MATH (999999), and their operand(s) must be within      */
/*    these bounds.                                                   */
/*                                                                    */
/* 3. Logical functions are further restricted; their operands must   */
/*    be finite, positive, have an exponent of zero, and all digits   */
/*    must be either 0 or 1.  The result will only contain digits     */
/*    which are 0 or 1 (and will have exponent=0 and a sign of 0).    */
/*                                                                    */
/* 4. Operands to operator functions are never modified unless they   */
/*    are also specified to be the result number (which is always     */
/*    permitted).  Other than that case, operands must not overlap.   */
/*                                                                    */
/* 5. Error handling: the type of the error is ORed into the status   */
/*    flags in the current context (decContext structure).  The       */
/*    SIGFPE signal is then raised if the corresponding trap-enabler  */
/*    flag in the decContext is set (is 1).                           */
/*                                                                    */
/*    It is the responsibility of the caller to clear the status      */
/*    flags as required.                                              */
/*                                                                    */
/*    The result of any routine which returns a number will always    */
/*    be a valid number (which may be a special value, such as an     */
/*    Infinity or NaN).                                               */
/*                                                                    */
/* 6. The decNumber format is not an exchangeable concrete            */
/*    representation as it comprises fields which may be machine-     */
/*    dependent (packed or unpacked, or special length, for example). */
/*    Canonical conversions to and from strings are provided; other   */
/*    conversions are available in separate modules.                  */
/*                                                                    */
/* 7. Normally, input operands are assumed to be valid.  Set DECCHECK */
/*    to 1 for extended operand checking (including NULL operands).   */
/*    Results are undefined if a badly-formed structure (or a NULL    */
/*    pointer to a structure) is provided, though with DECCHECK       */
/*    enabled the operator routines are protected against exceptions. */
/*    (Except if the result pointer is NULL, which is unrecoverable.) */
/*                                                                    */
/*    However, the routines will never cause exceptions if they are   */
/*    given well-formed operands, even if the value of the operands   */
/*    is inappropriate for the operation and DECCHECK is not set.     */
/*    (Except for SIGFPE, as and where documented.)                   */
/*                                                                    */
/* 8. Subset arithmetic is available only if DECSUBSET is set to 1.   */
/* ------------------------------------------------------------------ */
/* Implementation notes for maintenance of this module:               */
/*                                                                    */
/* 1. Storage leak protection:  Routines which use malloc are not     */
/*    permitted to use return for fastpath or error exits (i.e.,      */
/*    they follow strict structured programming conventions).         */
/*    Instead they have a do{}while(0); construct surrounding the     */
/*    code which is protected -- break may be used to exit this.      */
/*    Other routines can safely use the return statement inline.      */
/*                                                                    */
/*    Storage leak accounting can be enabled using DECALLOC.          */
/*                                                                    */
/* 2. All loops use the for(;;) construct.  Any do construct does     */
/*    not loop; it is for allocation protection as just described.    */
/*                                                                    */
/* 3. Setting status in the context must always be the very last      */
/*    action in a routine, as non-0 status may raise a trap and hence */
/*    the call to set status may not return (if the handler uses long */
/*    jump).  Therefore all cleanup must be done first.  In general,  */
/*    to achieve this status is accumulated and is only applied just  */
/*    before return by calling decContextSetStatus (via decStatus).   */
/*                                                                    */
/*    Routines which allocate storage cannot, in general, use the     */
/*    'top level' routines which could cause a non-returning          */
/*    transfer of control.  The decXxxxOp routines are safe (do not   */
/*    call decStatus even if traps are set in the context) and should */
/*    be used instead (they are also a little faster).                */
/*                                                                    */
/* 4. Exponent checking is minimized by allowing the exponent to      */
/*    grow outside its limits during calculations, provided that      */
/*    the decFinalize function is called later.  Multiplication and   */
/*    division, and intermediate calculations in exponentiation,      */
/*    require more careful checks because of the risk of 31-bit       */
/*    overflow (the most negative valid exponent is -1999999997, for  */
/*    a 999999999-digit number with adjusted exponent of -999999999). */
/*                                                                    */
/* 5. Rounding is deferred until finalization of results, with any    */
/*    'off to the right' data being represented as a single digit     */
/*    residue (in the range -1 through 9).  This avoids any double-   */
/*    rounding when more than one shortening takes place (for         */
/*    example, when a result is subnormal).                           */
/*                                                                    */
/* 6. The digits count is allowed to rise to a multiple of DECDPUN    */
/*    during many operations, so whole Units are handled and exact    */
/*    accounting of digits is not needed.  The correct digits value   */
/*    is found by decGetDigits, which accounts for leading zeros.     */
/*    This must be called before any rounding if the number of digits */
/*    is not known exactly.                                           */
/*                                                                    */
/* 7. The multiply-by-reciprocal 'trick' is used for partitioning     */
/*    numbers up to four digits, using appropriate constants.  This   */
/*    is not useful for longer numbers because overflow of 32 bits    */
/*    would lead to 4 multiplies, which is almost as expensive as     */
/*    a divide (unless a floating-point or 64-bit multiply is         */
/*    assumed to be available).                                       */
/*                                                                    */
/* 8. Unusual abbreviations that may be used in the commentary:       */
/*      lhs -- left hand side (operand, of an operation)              */
/*      lsd -- least significant digit (of coefficient)               */
/*      lsu -- least significant Unit (of coefficient)                */
/*      msd -- most significant digit (of coefficient)                */
/*      msi -- most significant item (in an array)                    */
/*      msu -- most significant Unit (of coefficient)                 */
/*      rhs -- right hand side (operand, of an operation)             */
/*      +ve -- positive                                               */
/*      -ve -- negative                                               */
/*      **  -- raise to the power                                     */
/* ------------------------------------------------------------------ */
// for malloc, free, etc.
// for printf [if needed]
// for strcpy
// for lower
/* Constants */
// Public lookup table used by the D2U macro
#[no_mangle]
pub static mut d2utable: [uint8_t; 50] =
    [0 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     3 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     4 as libc::c_int as uint8_t, 4 as libc::c_int as uint8_t,
     4 as libc::c_int as uint8_t, 5 as libc::c_int as uint8_t,
     5 as libc::c_int as uint8_t, 5 as libc::c_int as uint8_t,
     6 as libc::c_int as uint8_t, 6 as libc::c_int as uint8_t,
     6 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     8 as libc::c_int as uint8_t, 8 as libc::c_int as uint8_t,
     8 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     10 as libc::c_int as uint8_t, 10 as libc::c_int as uint8_t,
     10 as libc::c_int as uint8_t, 11 as libc::c_int as uint8_t,
     11 as libc::c_int as uint8_t, 11 as libc::c_int as uint8_t,
     12 as libc::c_int as uint8_t, 12 as libc::c_int as uint8_t,
     12 as libc::c_int as uint8_t, 13 as libc::c_int as uint8_t,
     13 as libc::c_int as uint8_t, 13 as libc::c_int as uint8_t,
     14 as libc::c_int as uint8_t, 14 as libc::c_int as uint8_t,
     14 as libc::c_int as uint8_t, 15 as libc::c_int as uint8_t,
     15 as libc::c_int as uint8_t, 15 as libc::c_int as uint8_t,
     16 as libc::c_int as uint8_t, 16 as libc::c_int as uint8_t,
     16 as libc::c_int as uint8_t, 17 as libc::c_int as uint8_t];
static mut uarrone: [uint16_t; 1] = [1 as libc::c_int as uint16_t];
// Unit array of 1, used for incrementing
/* Granularity-dependent code */
// extended integer
// unsigned extended integer
// Constant multipliers for divide-by-power-of five using reciprocal
  // multiply, after removing powers of 2 by shifting, and final shift
  // of 17 [we only need up to **4]
static mut multies: [uint32_t; 5] =
    [131073 as libc::c_int as uint32_t, 26215 as libc::c_int as uint32_t,
     5243 as libc::c_int as uint32_t, 1049 as libc::c_int as uint32_t,
     210 as libc::c_int as uint32_t];
/* Diagnostic macros, etc. */
/* ================================================================== */
/* Conversions                                                        */
/* ================================================================== */
/* ------------------------------------------------------------------ */
/* from-int32 -- conversion from Int or uInt                          */
/*                                                                    */
/*  dn is the decNumber to receive the integer                        */
/*  in or uin is the integer to be converted                          */
/*  returns dn                                                        */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberFromInt32(mut dn: *mut decNumber,
                                            mut in_0: int32_t)
 -> *mut decNumber {
    let mut unsig: uint32_t = 0;
    if in_0 >= 0 as libc::c_int {
        unsig = in_0 as uint32_t
    } else if in_0 == 0x80000000 as libc::c_uint as int32_t {
        // negative (possibly BADINT)
        unsig =
            (1073741824 as libc::c_int as
                 uint32_t).wrapping_mul(2 as libc::c_int as libc::c_uint)
    } else { unsig = -in_0 as uint32_t } // special case
    // invert
    // in is now positive
    decNumberFromUInt32(dn, unsig); // sign needed
    if in_0 < 0 as libc::c_int { (*dn).bits = 0x80 as libc::c_int as uint8_t }
    return dn;
}
// decNumberFromInt32
#[no_mangle]
pub unsafe extern "C" fn decNumberFromUInt32(mut dn: *mut decNumber,
                                             mut uin: uint32_t)
 -> *mut decNumber {
    let mut up: *mut uint16_t = 0 as *mut uint16_t; // work pointer
    decNumberZero(dn); // clean
    if uin == 0 as libc::c_int as libc::c_uint {
        return dn
    } // [or decGetDigits bad call]
    up = (*dn).lsu.as_mut_ptr();
    while uin > 0 as libc::c_int as libc::c_uint {
        *up =
            uin.wrapping_rem((999 as libc::c_int + 1 as libc::c_int) as
                                 libc::c_uint) as uint16_t;
        uin =
            uin.wrapping_div((999 as libc::c_int + 1 as libc::c_int) as
                                 libc::c_uint);
        up = up.offset(1)
    }
    (*dn).digits =
        decGetDigits((*dn).lsu.as_mut_ptr(),
                     up.wrapping_offset_from((*dn).lsu.as_mut_ptr()) as
                         libc::c_long as int32_t);
    return dn;
}
// decNumberFromUInt32
/* ------------------------------------------------------------------ */
/* to-int32 -- conversion to Int or uInt                              */
/*                                                                    */
/*  dn is the decNumber to convert                                    */
/*  set is the context for reporting errors                           */
/*  returns the converted decNumber, or 0 if Invalid is set           */
/*                                                                    */
/* Invalid is set if the decNumber does not have exponent==0 or if    */
/* it is a NaN, Infinite, or out-of-range.                            */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberToInt32(mut dn: *const decNumber,
                                          mut set: *mut decContext)
 -> int32_t {
    // special or too many digits, or bad exponent
    if !((*dn).bits as libc::c_int &
             (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
             != 0 || (*dn).digits > 10 as libc::c_int ||
             (*dn).exponent != 0 as libc::c_int) { // integer
        // is a finite integer with 10 or fewer digits
        let mut d: int32_t = 0; // work
        let mut up: *const uint16_t = 0 as *const uint16_t; // ..
        let mut hi: uint32_t = 0 as libc::c_int as uint32_t; // ..
        let mut lo: uint32_t = 0; // -> lsu
        up = (*dn).lsu.as_ptr(); // get 1 to 9 digits
        lo = *up as uint32_t;
        // split to higher
        hi = lo.wrapping_div(10 as libc::c_int as libc::c_uint);
        lo = lo.wrapping_rem(10 as libc::c_int as libc::c_uint);
        up = up.offset(1);
        // collect remaining Units, if any, into hi
        d = 3 as libc::c_int;
        while d < (*dn).digits {
            hi =
                (hi as
                     libc::c_uint).wrapping_add((*up as
                                                     libc::c_uint).wrapping_mul(DECPOWERS[(d
                                                                                               -
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              usize]))
                    as uint32_t as uint32_t;
            up = up.offset(1);
            d += 3 as libc::c_int
        }
        // now low has the lsd, hi the remainder
        if hi > 214748364 as libc::c_int as libc::c_uint ||
               hi == 214748364 as libc::c_int as libc::c_uint &&
                   lo > 7 as libc::c_int as libc::c_uint {
            // out of range?
            // most-negative is a reprieve
            if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 &&
                   hi == 214748364 as libc::c_int as libc::c_uint &&
                   lo == 8 as libc::c_int as libc::c_uint {
                return 0x80000000 as libc::c_uint as int32_t
            }
            // bad -- drop through
        } else {
            // in-range always
            let mut i: int32_t =
                (hi <<
                     1 as
                         libc::c_int).wrapping_add(hi <<
                                                       3 as
                                                           libc::c_int).wrapping_add(lo)
                    as int32_t; // [may not return]
            if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 {
                return -i
            }
            return i
        }
    }
    decContextSetStatus(set, 0x80 as libc::c_int as uint32_t);
    return 0 as libc::c_int;
}
// decNumberToInt32
#[no_mangle]
pub unsafe extern "C" fn decNumberToUInt32(mut dn: *const decNumber,
                                           mut set: *mut decContext)
 -> uint32_t {
    // special or too many digits, or bad exponent, or negative (<0)
    if !((*dn).bits as libc::c_int &
             (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
             != 0 || (*dn).digits > 10 as libc::c_int ||
             (*dn).exponent != 0 as libc::c_int ||
             (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 &&
                 !(*(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                       (*dn).digits == 1 as libc::c_int &&
                       (*dn).bits as libc::c_int &
                           (0x40 as libc::c_int | 0x20 as libc::c_int |
                                0x10 as libc::c_int) == 0 as libc::c_int))
       { // integer
        // is a finite integer with 10 or fewer digits
        let mut d: int32_t = 0; // work
        let mut up: *const uint16_t = 0 as *const uint16_t; // ..
        let mut hi: uint32_t = 0 as libc::c_int as uint32_t; // ..
        let mut lo: uint32_t = 0; // -> lsu
        up = (*dn).lsu.as_ptr(); // get 1 to 9 digits
        lo = *up as uint32_t;
        // split to higher
        hi = lo.wrapping_div(10 as libc::c_int as libc::c_uint);
        lo = lo.wrapping_rem(10 as libc::c_int as libc::c_uint);
        up = up.offset(1);
        // collect remaining Units, if any, into hi
        d = 3 as libc::c_int;
        while d < (*dn).digits {
            hi =
                (hi as
                     libc::c_uint).wrapping_add((*up as
                                                     libc::c_uint).wrapping_mul(DECPOWERS[(d
                                                                                               -
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              usize]))
                    as uint32_t as uint32_t;
            up = up.offset(1);
            d += 3 as libc::c_int
        }
        // now low has the lsd, hi the remainder
        if hi > 429496729 as libc::c_int as libc::c_uint ||
               hi == 429496729 as libc::c_int as libc::c_uint &&
                   lo > 5 as libc::c_int as libc::c_uint {
        } else {
            return (hi <<
                        1 as
                            libc::c_int).wrapping_add(hi <<
                                                          3 as
                                                              libc::c_int).wrapping_add(lo)
        }
    } // [may not return]
    decContextSetStatus(set, 0x80 as libc::c_int as uint32_t);
    return 0 as libc::c_int as uint32_t;
}
// decNumberToUInt32
/* ------------------------------------------------------------------ */
/* to-scientific-string -- conversion to numeric string               */
/* to-engineering-string -- conversion to numeric string              */
/*                                                                    */
/*   decNumberToString(dn, string);                                   */
/*   decNumberToEngString(dn, string);                                */
/*                                                                    */
/*  dn is the decNumber to convert                                    */
/*  string is the string where the result will be laid out            */
/*                                                                    */
/*  string must be at least dn->digits+14 characters long             */
/*                                                                    */
/*  No error is possible, and no status can be set.                   */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberToString(mut dn: *const decNumber,
                                           mut string: *mut libc::c_char)
 -> *mut libc::c_char {
    decToString(dn, string, 0 as libc::c_int as uint8_t);
    return string;
}
// DecNumberToString
#[no_mangle]
pub unsafe extern "C" fn decNumberToEngString(mut dn: *const decNumber,
                                              mut string: *mut libc::c_char)
 -> *mut libc::c_char {
    decToString(dn, string, 1 as libc::c_int as uint8_t);
    return string;
}
// DecNumberToEngString
/* ------------------------------------------------------------------ */
/* to-number -- conversion from numeric string                        */
/*                                                                    */
/* decNumberFromString -- convert string to decNumber                 */
/*   dn        -- the number structure to fill                        */
/*   chars[]   -- the string to convert ('\0' terminated)             */
/*   set       -- the context used for processing any error,          */
/*                determining the maximum precision available         */
/*                (set.digits), determining the maximum and minimum   */
/*                exponent (set.emax and set.emin), determining if    */
/*                extended values are allowed, and checking the       */
/*                rounding mode if overflow occurs or rounding is     */
/*                needed.                                             */
/*                                                                    */
/* The length of the coefficient and the size of the exponent are     */
/* checked by this routine, so the correct error (Underflow or        */
/* Overflow) can be reported or rounding applied, as necessary.       */
/*                                                                    */
/* If bad syntax is detected, the result will be a quiet NaN.         */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberFromString(mut dn: *mut decNumber,
                                             mut chars: *const libc::c_char,
                                             mut set: *mut decContext)
 -> *mut decNumber {
    let mut exponent: int32_t =
        0 as libc::c_int; // working exponent [assume 0]
    let mut bits: uint8_t =
        0 as libc::c_int as uint8_t; // working flags [assume +ve]
    let mut res: *mut uint16_t =
        0 as *mut uint16_t; // where result will be built
    let mut resbuff: [uint16_t; 15] =
        [0;
            15]; // local buffer in case need temporary
                                   // [+9 allows for ln() constants]
    let mut allocres: *mut uint16_t =
        0 as *mut uint16_t; // -> allocated result, iff allocated
    let mut d: int32_t =
        0 as libc::c_int; // count of digits found in decimal part
    let mut dotchar: *const libc::c_char =
        0 as *const libc::c_char; // where dot was found
    let mut cfirst: *const libc::c_char =
        chars; // -> first character of decimal part
    let mut last: *const libc::c_char =
        0 as *const libc::c_char; // -> last digit of decimal part
    let mut c: *const libc::c_char = 0 as *const libc::c_char; // work
    let mut up: *mut uint16_t = 0 as *mut uint16_t; // ..
    let mut cut: int32_t = 0; // ..
    let mut out: int32_t = 0; // rounding residue
    let mut residue: int32_t = 0; // error code
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // [for break]
    let mut current_block_88: u64;
    // status & malloc protection
    c = chars; // c
    loop 
         // -> input character
         {
        if *c as libc::c_int >= '0' as i32 && *c as libc::c_int <= '9' as i32
           {
            // test for Arabic digit
            last = c;
            // still in decimal part
            d += 1
        } else if *c as libc::c_int == '.' as i32 && dotchar.is_null()
         { // count of real digits
            // first '.'
            dotchar = c; // record offset into decimal part
            if c == cfirst { cfirst = cfirst.offset(1) }
        } else {
            if !(c == chars) {
                break ; // first digit must follow
            }
            // first in string...
            if *c as libc::c_int == '-' as i32 {
                // valid - sign
                cfirst = cfirst.offset(1);
                bits = 0x80 as libc::c_int as uint8_t
            } else {
                if !(*c as libc::c_int == '+' as i32) { break ; }
                // valid + sign
                cfirst = cfirst.offset(1)
            }
        } // stuff after digits
        c = c.offset(1)
    }
    if last.is_null() {
        // no digits yet
        status = 0x1 as libc::c_int as uint32_t; // assume the worst
        // for copy-back
        if *c as libc::c_int == '\u{0}' as i32 {
            current_block_88 = 13326139174796812312; // and no more to come...
        } else if !dotchar.is_null() {
            current_block_88 = 13326139174796812312;
        } else {
            // Infinities and NaNs are possible, here
            // .. unless had a dot
            decNumberZero(dn); // be optimistic
            if decBiStr(c,
                        b"infinity\x00" as *const u8 as *const libc::c_char,
                        b"INFINITY\x00" as *const u8 as *const libc::c_char)
                   as libc::c_int != 0 ||
                   decBiStr(c, b"inf\x00" as *const u8 as *const libc::c_char,
                            b"INF\x00" as *const u8 as *const libc::c_char) as
                       libc::c_int != 0 {
                (*dn).bits =
                    (bits as libc::c_int | 0x40 as libc::c_int) as uint8_t;
                // all done
                status = 0 as libc::c_int as uint32_t; // is OK
                current_block_88 = 13326139174796812312;
            } else {
                // a NaN expected
      // 2003.09.10 NaNs are now permitted to have a sign
                (*dn).bits =
                    (bits as libc::c_int | 0x20 as libc::c_int) as
                        uint8_t; // assume simple NaN
                if *c as libc::c_int == 's' as i32 ||
                       *c as libc::c_int == 'S' as i32 {
                    // looks like an sNaN
                    c = c.offset(1); // check caseless "NaN"
                    (*dn).bits =
                        (bits as libc::c_int | 0x10 as libc::c_int) as uint8_t
                } // ..
                if *c as libc::c_int != 'n' as i32 &&
                       *c as libc::c_int != 'N' as i32 {
                    current_block_88 = 13326139174796812312; // ..
                } else {
                    c = c.offset(1);
                    if *c as libc::c_int != 'a' as i32 &&
                           *c as libc::c_int != 'A' as i32 {
                        current_block_88 = 13326139174796812312;
                    } else {
                        c = c.offset(1);
                        if *c as libc::c_int != 'n' as i32 &&
                               *c as libc::c_int != 'N' as i32 {
                            current_block_88 = 13326139174796812312;
                        } else {
                            c = c.offset(1);
                            // now either nothing, or nnnn payload, expected
      // -> start of integer and skip leading 0s [including plain 0]
                            cfirst = c;
                            while *cfirst as libc::c_int == '0' as i32 {
                                cfirst = cfirst.offset(1)
                            }
                            if *cfirst as libc::c_int == '\u{0}' as i32 {
                                // "NaN" or "sNaN", maybe with all 0s
                                status = 0 as libc::c_int as uint32_t;
                                current_block_88 =
                                    13326139174796812312; // it's good
                                // ..
                            } else {
                                // something other than 0s; setup last and d as usual [no dots]
                                c = cfirst; // test for Arabic digit
                                while !((*c as libc::c_int) < '0' as i32 ||
                                            *c as libc::c_int > '9' as i32) {
                                    last = c; // not all digits
                                    c = c.offset(1); // too many digits?
                                    d += 1
                                }
                                if *c as libc::c_int != '\u{0}' as i32 {
                                    current_block_88 = 13326139174796812312;
                                } else {
                                    if d > (*set).digits - 1 as libc::c_int {
                                        // [NB: payload in a decNumber can be full length unless
        // clamped, in which case can only be digits-1]
                                        if (*set).clamp != 0 {
                                            current_block_88 =
                                                13326139174796812312;
                                        } else if d > (*set).digits {
                                            current_block_88 =
                                                13326139174796812312;
                                        } else {
                                            current_block_88 =
                                                17075014677070940716;
                                        }
                                    } else {
                                        current_block_88 =
                                            17075014677070940716;
                                    }
                                    match current_block_88 {
                                        13326139174796812312 => { }
                                        _ => {
                                            // good; drop through to convert the integer to coefficient
                                            status =
                                                0 as libc::c_int as
                                                    uint32_t; // syntax is OK
                                            bits = (*dn).bits;
                                            current_block_88 =
                                                17769492591016358583;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if *c as libc::c_int != '\u{0}' as i32 {
        // more to process...
        // had some digits; exponent is only valid sequence now
        let mut nege: uint8_t = 0; // 1=negative exponent
        let mut firstexp: *const libc::c_char =
            0 as *const libc::c_char; // -> first significant exponent digit
        status = 0x1 as libc::c_int as uint32_t; // assume the worst
        if *c as libc::c_int != 'e' as i32 && *c as libc::c_int != 'E' as i32
           {
            current_block_88 = 13326139174796812312;
        } else {
            /* Found 'e' or 'E' -- now process explicit exponent */
      // 1998.07.11: sign no longer required
            nege = 0 as libc::c_int as uint8_t; // to (possible) sign
            c = c.offset(1); // strip insignificant zeros
            if *c as libc::c_int == '-' as i32 {
                nege =
                    1 as libc::c_int as uint8_t; // save exponent digit place
                c = c.offset(1)
            } else if *c as libc::c_int == '+' as i32 { c = c.offset(1) } // c
            if *c as libc::c_int == '\u{0}' as i32 {
                current_block_88 = 13326139174796812312; // not a digit
            } else {
                while *c as libc::c_int == '0' as i32 &&
                          *c.offset(1 as libc::c_int as isize) as libc::c_int
                              != '\u{0}' as i32 {
                    c = c.offset(1)
                }
                firstexp = c;
                while !((*c as libc::c_int) < '0' as i32 ||
                            *c as libc::c_int > '9' as i32) {
                    exponent =
                        (exponent << 1 as libc::c_int) +
                            (exponent << 3 as libc::c_int) + *c as int32_t -
                            '0' as i32;
                    c = c.offset(1)
                }
                if *c as libc::c_int != '\u{0}' as i32 {
                    current_block_88 = 13326139174796812312;
                } else {
                    // (this next test must be after the syntax checks)
      // if it was too long the exponent may have wrapped, so check
      // carefully and set it to a certain overflow if wrap possible
                    if c >=
                           firstexp.offset(9 as libc::c_int as
                                               isize).offset(1 as libc::c_int
                                                                 as isize) {
                        if c >
                               firstexp.offset(9 as libc::c_int as
                                                   isize).offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                               || *firstexp as libc::c_int > '1' as i32 {
                            exponent =
                                999999999 as libc::c_int * 2 as libc::c_int
                        }
                        // [up to 1999999999 is OK, for example 1E-1000000998]
                    } // was negative
                    if nege != 0 { exponent = -exponent }
                    status = 0 as libc::c_int as uint32_t;
                    current_block_88 = 17769492591016358583;
                }
            }
        }
        // if not now on a '\0', *c must not be a digit
        // is OK
    } else { current_block_88 = 17769492591016358583; }
    match current_block_88 {
        17769492591016358583 => {
            // Here when whole string has been inspected; syntax is good
    // cfirst->first digit (never dot), last->last digit (ditto)
            // strip leading zeros/dot [leave final 0 if all 0's]
            if *cfirst as libc::c_int == '0' as i32
               { // at least one leading 0
                // [cfirst has stepped over .]
                c = cfirst; // ignore dots
                while c < last {
                    if !(*c as libc::c_int == '.' as i32) {
                        if *c as libc::c_int != '0' as i32 {
                            break ; // non-zero found
                        }
                        d -= 1
                    }
                    c = c.offset(1);
                    cfirst = cfirst.offset(1)
                    // 0 stripped
                }
                // c
            }
            // Handle decimal point...
            if !dotchar.is_null() && dotchar < last { // adjust exponent
                // non-trailing '.' found?
                exponent =
                    (exponent as libc::c_long -
                         last.wrapping_offset_from(dotchar) as libc::c_long)
                        as int32_t
            }
            // [we can now ignore the .]
            // OK, the digits string is good.  Assemble in the decNumber, or in
    // a temporary units array if rounding is needed
            if d <= (*set).digits {
                res = (*dn).lsu.as_mut_ptr(); // fits into supplied decNumber
                current_block_88 = 7019009297990327870;
            } else {
                // rounding needed
                let mut needbytes: int32_t =
                    ((if d <= 49 as libc::c_int {
                          d2utable[d as usize] as libc::c_int
                      } else {
                          (d + 3 as libc::c_int - 1 as libc::c_int) /
                              3 as libc::c_int
                      }) as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                         as libc::c_ulong) as
                        int32_t; // bytes needed
                res = resbuff.as_mut_ptr(); // assume use local buffer
                if needbytes >
                       ::std::mem::size_of::<[uint16_t; 15]>() as
                           libc::c_ulong as int32_t {
                    // too big for local
                    allocres =
                        malloc(needbytes as libc::c_ulong) as *mut uint16_t;
                    if allocres.is_null() {
                        status |= 0x10 as libc::c_int as libc::c_uint;
                        current_block_88 = 13326139174796812312;
                    } else {
                        res = allocres;
                        current_block_88 = 7019009297990327870;
                    }
                } else { current_block_88 = 7019009297990327870; }
            }
            match current_block_88 {
                13326139174796812312 => { }
                _ => {
                    // res now -> number lsu, buffer, or allocated storage for Unit array
                    // Place the coefficient into the selected Unit array
    // [this is often 70% of the cost of this function when DECDPUN>1]
                    out = 0 as libc::c_int; // accumulator
                    up =
                        res.offset((if d <= 49 as libc::c_int {
                                        d2utable[d as usize] as libc::c_int
                                    } else {
                                        (d + 3 as libc::c_int -
                                             1 as libc::c_int) /
                                            3 as libc::c_int
                                    }) as
                                       isize).offset(-(1 as libc::c_int as
                                                           isize)); // -> msu
                    cut =
                        (d as libc::c_long -
                             up.wrapping_offset_from(res) as libc::c_long *
                                 3 as libc::c_int as libc::c_long) as
                            int32_t; // digits in top unit
                    c = cfirst; // c
                    loop  {
                        // along the digits
                        if !(*c as libc::c_int == '.' as i32) {
                            out =
                                (out << 1 as libc::c_int) +
                                    (out << 3 as libc::c_int) + *c as int32_t
                                    -
                                    '0' as
                                        i32; // ignore '.' [don't decrement cut]
                            if c == last {
                                break ; // done [never get to trailing '.']
                            } // more for this unit
                            cut -= 1; // write unit
                            if !(cut > 0 as libc::c_int) {
                                *up =
                                    out as
                                        uint16_t; // prepare for unit below..
                                up = up.offset(-1); // ..
                                cut = 3 as libc::c_int;
                                out = 0 as libc::c_int
                            }
                        }
                        c = c.offset(1)
                        // ..
                    } // write lsu
                    *up = out as uint16_t;
                    (*dn).bits = bits;
                    (*dn).exponent = exponent;
                    (*dn).digits = d;
                    // if not in number (too long) shorten into the number
                    if d > (*set).digits {
                        residue = 0 as libc::c_int;
                        decSetCoeff(dn, set, res, d, &mut residue,
                                    &mut status);
                        // always check for overflow or subnormal and round as needed
                        decFinalize(dn, set, &mut residue, &mut status);
                    } else if ((*dn).exponent - 1 as libc::c_int) <
                                  (*set).emin - (*dn).digits ||
                                  (*dn).exponent - 1 as libc::c_int >
                                      (*set).emax - (*set).digits {
                        residue = 0 as libc::c_int;
                        decFinalize(dn, set, &mut residue, &mut status);
                    }
                }
            }
        }
        _ => { }
    }
    // no rounding, but may still have overflow or subnormal
    // [these tests are just for performance; finalize repeats them]
    if !allocres.is_null() {
        free(allocres as *mut libc::c_void); // drop any storage used
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(dn, status, set);
    }
    return dn;
}
/* decNumberFromString */
/* ================================================================== */
/* Operators                                                          */
/* ================================================================== */
/* ------------------------------------------------------------------ */
/* decNumberAbs -- absolute value operator                            */
/*                                                                    */
/*   This computes C = abs(A)                                         */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context                                               */
/*                                                                    */
/* See also decNumberCopyAbs for a quiet bitwise version of this.     */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
/* This has the same effect as decNumberPlus unless A is negative,    */
/* in which case it has the same effect as decNumberMinus.            */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberAbs(mut res: *mut decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut dzero: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; // for 0
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decNumberZero(&mut dzero); // set 0
    dzero.exponent = (*rhs).exponent; // [no coefficient expansion]
    decAddOp(res, &mut dzero, rhs, set,
             ((*rhs).bits as libc::c_int & 0x80 as libc::c_int) as uint8_t,
             &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberAbs
/* ------------------------------------------------------------------ */
/* decNumberAdd -- add two Numbers                                    */
/*                                                                    */
/*   This computes C = A + B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X+X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
/* This just calls the routine shared with Subtract                   */
#[no_mangle]
pub unsafe extern "C" fn decNumberAdd(mut res: *mut decNumber,
                                      mut lhs: *const decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decAddOp(res, lhs, rhs, set, 0 as libc::c_int as uint8_t, &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberAdd
/* ------------------------------------------------------------------ */
/* decNumberAnd -- AND two Numbers, digitwise                         */
/*                                                                    */
/*   This computes C = A & B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X&X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context (used for result length and error report)     */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Logical function restrictions apply (see above); a NaN is          */
/* returned with Invalid_operation if a restriction is violated.      */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberAnd(mut res: *mut decNumber,
                                      mut lhs: *const decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut ua: *const uint16_t = 0 as *const uint16_t; // -> operands
    let mut ub: *const uint16_t = 0 as *const uint16_t; // -> operand msus
    let mut msua: *const uint16_t =
        0 as *const uint16_t; // -> result and its msu
    let mut msub: *const uint16_t = 0 as *const uint16_t; // digits in res msu
    let mut uc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msuc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msudigs: int32_t = 0;
    if (*lhs).exponent != 0 as libc::c_int ||
           (*lhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int || (*rhs).exponent != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
        decStatus(res, 0x80 as libc::c_int as uint32_t, set);
        return res
    }
    // operands are valid
    ua = (*lhs).lsu.as_ptr(); // bottom-up
    ub = (*rhs).lsu.as_ptr(); // ..
    uc = (*res).lsu.as_mut_ptr(); // ..
    msua =
        ua.offset((if (*lhs).digits <= 49 as libc::c_int {
                       d2utable[(*lhs).digits as usize] as libc::c_int
                   } else {
                       ((*lhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of lhs
    msub =
        ub.offset((if (*rhs).digits <= 49 as libc::c_int {
                       d2utable[(*rhs).digits as usize] as libc::c_int
                   } else {
                       ((*rhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of rhs
    msuc =
        uc.offset((if (*set).digits <= 49 as libc::c_int {
                       d2utable[(*set).digits as usize] as libc::c_int
                   } else {
                       ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of result
    msudigs =
        (*set).digits -
            ((if (*set).digits <= 49 as libc::c_int {
                  d2utable[(*set).digits as usize] as libc::c_int
              } else {
                  ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                      3 as libc::c_int
              }) - 1 as libc::c_int) *
                3 as libc::c_int; // [faster than remainder]
    while uc <= msuc { // each unit
        // Unit loop
        let mut a: uint16_t = 0; // extract units
        let mut b: uint16_t = 0; // can now write back
        if ua > msua { a = 0 as libc::c_int as uint16_t } else { a = *ua }
        if ub > msub { b = 0 as libc::c_int as uint16_t } else { b = *ub }
        *uc = 0 as libc::c_int as uint16_t;
        if a as libc::c_int | b as libc::c_int != 0 {
            // maybe 1 bits to examine
            let mut i: int32_t = 0; // can now write back
            let mut j: int32_t = 0;
            *uc = 0 as libc::c_int as uint16_t;
            // This loop could be unrolled and/or use BIN2BCD tables
            i = 0 as libc::c_int; // effect AND
            while i < 3 as libc::c_int {
                if a as libc::c_int & b as libc::c_int & 1 as libc::c_int != 0
                   {
                    *uc =
                        (*uc as libc::c_int +
                             DECPOWERS[i as usize] as uint16_t as libc::c_int)
                            as uint16_t
                }
                j = a as libc::c_int % 10 as libc::c_int;
                a = (a as libc::c_int / 10 as libc::c_int) as uint16_t;
                j |= b as libc::c_int % 10 as libc::c_int;
                b = (b as libc::c_int / 10 as libc::c_int) as uint16_t;
                if j > 1 as libc::c_int {
                    decStatus(res, 0x80 as libc::c_int as uint32_t, set);
                    return res
                }
                if uc == msuc && i == msudigs - 1 as libc::c_int { break ; }
                i += 1
                // just did final digit
            }
        }
        ua = ua.offset(1);
        ub = ub.offset(1);
        uc = uc.offset(1)
    }
    // [here uc-1 is the msu of the result]
    (*res).digits =
        decGetDigits((*res).lsu.as_mut_ptr(),
                     uc.wrapping_offset_from((*res).lsu.as_mut_ptr()) as
                         libc::c_long as int32_t); // integer
    (*res).exponent = 0 as libc::c_int; // sign=0
    (*res).bits = 0 as libc::c_int as uint8_t;
    return res;
    // [no status to set]
}
// decNumberAnd
/* ------------------------------------------------------------------ */
/* decNumberCompare -- compare two Numbers                            */
/*                                                                    */
/*   This computes C = A ? B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for one digit (or NaN).                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCompare(mut res: *mut decNumber,
                                          mut lhs: *const decNumber,
                                          mut rhs: *const decNumber,
                                          mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x1 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberCompare
/* ------------------------------------------------------------------ */
/* decNumberCompareSignal -- compare, signalling on all NaNs          */
/*                                                                    */
/*   This computes C = A ? B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for one digit (or NaN).                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCompareSignal(mut res: *mut decNumber,
                                                mut lhs: *const decNumber,
                                                mut rhs: *const decNumber,
                                                mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x6 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberCompareSignal
/* ------------------------------------------------------------------ */
/* decNumberCompareTotal -- compare two Numbers, using total ordering */
/*                                                                    */
/*   This computes C = A ? B, under total ordering                    */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for one digit; the result will always be one of  */
/* -1, 0, or 1.                                                       */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCompareTotal(mut res: *mut decNumber,
                                               mut lhs: *const decNumber,
                                               mut rhs: *const decNumber,
                                               mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x4 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberCompareTotal
/* ------------------------------------------------------------------ */
/* decNumberCompareTotalMag -- compare, total ordering of magnitudes  */
/*                                                                    */
/*   This computes C = |A| ? |B|, under total ordering                */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for one digit; the result will always be one of  */
/* -1, 0, or 1.                                                       */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCompareTotalMag(mut res: *mut decNumber,
                                                  mut lhs: *const decNumber,
                                                  mut rhs: *const decNumber,
                                                  mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    let mut needbytes: uint32_t = 0; // for space calculations
    let mut bufa: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            3]; // +1 in case DECBUFFER=0
    let mut allocbufa: *mut decNumber =
        0 as *mut decNumber; // -> allocated bufa, iff allocated
    let mut bufb: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            3]; // -> allocated bufb, iff allocated
    let mut allocbufb: *mut decNumber =
        0 as *mut decNumber; // temporary pointers
    let mut a: *mut decNumber = 0 as *mut decNumber;
    let mut b: *mut decNumber = 0 as *mut decNumber;
    let mut current_block_16: u64;
    if (*lhs).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int {
        // lhs<0
        a = bufa.as_mut_ptr();
        needbytes =
            (::std::mem::size_of::<decNumber>() as
                 libc::c_ulong).wrapping_add((((if (*lhs).digits <=
                                                       49 as libc::c_int {
                                                    d2utable[(*lhs).digits as
                                                                 usize] as
                                                        libc::c_int
                                                } else {
                                                    ((*lhs).digits +
                                                         3 as libc::c_int -
                                                         1 as libc::c_int) /
                                                        3 as libc::c_int
                                                }) - 1 as libc::c_int) as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                  as
                                                                                  libc::c_ulong))
                as uint32_t;
        if needbytes as libc::c_ulong >
               ::std::mem::size_of::<[decNumber; 3]>() as libc::c_ulong {
            // use copy from here on
            // need malloc space
            allocbufa = malloc(needbytes as libc::c_ulong) as *mut decNumber;
            if allocbufa.is_null() {
                // use the allocated space
                // hopeless -- abandon
                status |= 0x10 as libc::c_int as libc::c_uint; // copy content
                current_block_16 =
                    4775909272756257391; // .. and clear the sign
            } else {
                a = allocbufa; // end protected
                current_block_16 = 13586036798005543211;
            }
        } else { current_block_16 = 13586036798005543211; }
        match current_block_16 {
            4775909272756257391 => { }
            _ => {
                decNumberCopy(a, lhs);
                (*a).bits =
                    ((*a).bits as libc::c_int & !(0x80 as libc::c_int)) as
                        uint8_t;
                lhs = a;
                current_block_16 = 12039483399334584727;
            }
        }
    } else { current_block_16 = 12039483399334584727; }
    match current_block_16 {
        12039483399334584727 => {
            if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
                // protect allocated storage
                // if either is negative, take a copy and absolute
                // rhs<0
                b = bufb.as_mut_ptr();
                needbytes =
                    (::std::mem::size_of::<decNumber>() as
                         libc::c_ulong).wrapping_add((((if (*rhs).digits <=
                                                               49 as
                                                                   libc::c_int
                                                           {
                                                            d2utable[(*rhs).digits
                                                                         as
                                                                         usize]
                                                                as libc::c_int
                                                        } else {
                                                            ((*rhs).digits +
                                                                 3 as
                                                                     libc::c_int
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                /
                                                                3 as
                                                                    libc::c_int
                                                        }) - 1 as libc::c_int)
                                                          as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                          as
                                                                                          libc::c_ulong))
                        as uint32_t;
                if needbytes as libc::c_ulong >
                       ::std::mem::size_of::<[decNumber; 3]>() as
                           libc::c_ulong {
                    // use copy from here on
                    // need malloc space
                    allocbufb =
                        malloc(needbytes as libc::c_ulong) as *mut decNumber;
                    if allocbufb.is_null() {
                        // use the allocated space
                        // hopeless -- abandon
                        status |=
                            0x10 as libc::c_int as
                                libc::c_uint; // copy content
                        current_block_16 =
                            4775909272756257391; // .. and clear the sign
                    } else {
                        b = allocbufb; // drop any storage used
                        current_block_16 = 17478428563724192186; // ..
                    }
                } else { current_block_16 = 17478428563724192186; }
                match current_block_16 {
                    4775909272756257391 => { }
                    _ => {
                        decNumberCopy(b, rhs);
                        (*b).bits =
                            ((*b).bits as libc::c_int &
                                 !(0x80 as libc::c_int)) as uint8_t;
                        rhs = b;
                        current_block_16 = 13550086250199790493;
                    }
                }
            } else { current_block_16 = 13550086250199790493; }
            match current_block_16 {
                4775909272756257391 => { }
                _ => {
                    decCompareOp(res, lhs, rhs, set,
                                 0x4 as libc::c_int as uint8_t, &mut status);
                }
            }
        }
        _ => { }
    }
    if !allocbufa.is_null() { free(allocbufa as *mut libc::c_void); }
    if !allocbufb.is_null() { free(allocbufb as *mut libc::c_void); }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberCompareTotalMag
/* ------------------------------------------------------------------ */
/* decNumberDivide -- divide one number by another                    */
/*                                                                    */
/*   This computes C = A / B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X/X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberDivide(mut res: *mut decNumber,
                                         mut lhs: *const decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decDivideOp(res, lhs, rhs, set, 0x80 as libc::c_int as uint8_t,
                &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberDivide
/* ------------------------------------------------------------------ */
/* decNumberDivideInteger -- divide and return integer quotient       */
/*                                                                    */
/*   This computes C = A # B, where # is the integer divide operator  */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X#X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberDivideInteger(mut res: *mut decNumber,
                                                mut lhs: *const decNumber,
                                                mut rhs: *const decNumber,
                                                mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decDivideOp(res, lhs, rhs, set, 0x20 as libc::c_int as uint8_t,
                &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberDivideInteger
/* ------------------------------------------------------------------ */
/* decNumberExp -- exponentiation                                     */
/*                                                                    */
/*   This computes C = exp(A)                                         */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context; note that rounding mode has no effect        */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Mathematical function restrictions apply (see above); a NaN is     */
/* returned with Invalid_operation if a restriction is violated.      */
/*                                                                    */
/* Finite results will always be full precision and Inexact, except   */
/* when A is a zero or -Infinity (giving 1 or 0 respectively).        */
/*                                                                    */
/* An Inexact result is rounded using DEC_ROUND_HALF_EVEN; it will    */
/* almost always be correctly rounded, but may be up to 1 ulp in      */
/* error in rare cases.                                               */
/* ------------------------------------------------------------------ */
/* This is a wrapper for decExpOp which can handle the slightly wider */
/* (double) range needed by Ln (which has to be able to calculate     */
/* exp(-a) where a can be the tiniest number (Ntiny).                 */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberExp(mut res: *mut decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    // Check restrictions; these restrictions ensure that if h=8 (see
  // decExpOp) then the result will either overflow or underflow to 0.
  // Other math functions restrict the input range, too, for inverses.
  // If not violated then carry out the operation.
    if decCheckMath(rhs, set, &mut status) == 0 { // end protected
        // protect allocation
        decExpOp(res, rhs, set, &mut status);
    }
    // apply significant status
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberExp
/* ------------------------------------------------------------------ */
/* decNumberFMA -- fused multiply add                                 */
/*                                                                    */
/*   This computes D = (A * B) + C with only one rounding             */
/*                                                                    */
/*   res is D, the result.  D may be A or B or C (e.g., X=FMA(X,X,X)) */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   fhs is C [far hand side]                                         */
/*   set is the context                                               */
/*                                                                    */
/* Mathematical function restrictions apply (see above); a NaN is     */
/* returned with Invalid_operation if a restriction is violated.      */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberFMA(mut res: *mut decNumber,
                                      mut lhs: *const decNumber,
                                      mut rhs: *const decNumber,
                                      mut fhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    let mut dcmul: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // context for the multiplication
    let mut needbytes: uint32_t = 0; // for space calculations
    let mut bufa: [decNumber; 5] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            5]; // -> allocated bufa, iff allocated
    let mut allocbufa: *mut decNumber =
        0 as *mut decNumber; // accumulator pointer
    let mut acc: *mut decNumber = 0 as *mut decNumber; // work
    let mut dzero: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
    let mut current_block_14: u64;
    if !(!((*lhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int) &&
             decCheckMath(lhs, set, &mut status) != 0 ||
             !((*rhs).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) != 0 as libc::c_int) &&
                 decCheckMath(rhs, set, &mut status) != 0 ||
             !((*fhs).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) != 0 as libc::c_int) &&
                 decCheckMath(fhs, set, &mut status) != 0) {
        // set up context for multiply
        dcmul = *set; // just enough
        dcmul.digits = (*lhs).digits + (*rhs).digits;
        // [The above may be an over-estimate for subset arithmetic, but that's OK]
        dcmul.emax = 999999999 as libc::c_int; // effectively unbounded ..
        dcmul.emin =
            -(999999999 as libc::c_int); // [thanks to Math restrictions]
        // set up decNumber space to receive the result of the multiply
        acc = bufa.as_mut_ptr(); // may fit
        needbytes =
            (::std::mem::size_of::<decNumber>() as
                 libc::c_ulong).wrapping_add((((if dcmul.digits <=
                                                       49 as libc::c_int {
                                                    d2utable[dcmul.digits as
                                                                 usize] as
                                                        libc::c_int
                                                } else {
                                                    (dcmul.digits +
                                                         3 as libc::c_int -
                                                         1 as libc::c_int) /
                                                        3 as libc::c_int
                                                }) - 1 as libc::c_int) as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                  as
                                                                                  libc::c_ulong))
                as uint32_t;
        if needbytes as libc::c_ulong >
               ::std::mem::size_of::<[decNumber; 5]>() as libc::c_ulong {
            // need malloc space
            allocbufa = malloc(needbytes as libc::c_ulong) as *mut decNumber;
            if allocbufa.is_null() {
                // use the allocated space
                // hopeless -- abandon
                status |= 0x10 as libc::c_int as libc::c_uint;
                current_block_14 = 7172762164747879670;
            } else {
                acc = allocbufa;
                current_block_14 = 5143058163439228106;
            }
        } else { current_block_14 = 5143058163439228106; }
        match current_block_14 {
            7172762164747879670 => { }
            _ => {
                // multiply with extended range and necessary precision
    //printf("emin=%ld\n", dcmul.emin);
                decMultiplyOp(acc, lhs, rhs, &mut dcmul, &mut status);
                // Only Invalid operation (from sNaN or Inf * 0) is possible in
    // status; if either is seen than ignore fhs (in case it is
    // another sNaN) and set acc to NaN unless we had an sNaN
    // [decMultiplyOp leaves that to caller]
    // Note sNaN has to go through addOp to shorten payload if
    // necessary
                if status & 0x80 as libc::c_int as libc::c_uint !=
                       0 as libc::c_int as libc::c_uint {
                    if status & 0x40000000 as libc::c_int as libc::c_uint == 0
                       {
                        // but be true invalid
                        decNumberZero(res); // acc not yet set
                        (*res).bits =
                            0x20 as libc::c_int as
                                uint8_t; // make 0 (any non-NaN would do)
                        current_block_14 = 7172762164747879670;
                    } else {
                        decNumberZero(&mut dzero);
                        fhs = &mut dzero;
                        current_block_14 = 10043043949733653460;
                    }
                    // use that
                } else { current_block_14 = 10043043949733653460; }
                match current_block_14 {
                    7172762164747879670 => { }
                    _ => {
                        // add the third operand and result -> res, and all is done
                        decAddOp(res, acc, fhs, set,
                                 0 as libc::c_int as uint8_t,
                                 &mut status); // end protected
                    }
                }
            }
        }
    }
    // protect allocated storage
    // Check math restrictions [these ensure no overflow or underflow]
    if !allocbufa.is_null() {
        free(allocbufa as *mut libc::c_void); // drop any storage used
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberFMA
/* ------------------------------------------------------------------ */
/* decNumberInvert -- invert a Number, digitwise                      */
/*                                                                    */
/*   This computes C = ~A                                             */
/*                                                                    */
/*   res is C, the result.  C may be A (e.g., X=~X)                   */
/*   rhs is A                                                         */
/*   set is the context (used for result length and error report)     */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Logical function restrictions apply (see above); a NaN is          */
/* returned with Invalid_operation if a restriction is violated.      */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberInvert(mut res: *mut decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut ua: *const uint16_t =
        0 as *const uint16_t; // -> operand and its msu
    let mut msua: *const uint16_t =
        0 as *const uint16_t; // -> result and its msu
    let mut uc: *mut uint16_t = 0 as *mut uint16_t; // digits in res msu
    let mut msuc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msudigs: int32_t = 0;
    if (*rhs).exponent != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
        decStatus(res, 0x80 as libc::c_int as uint32_t, set);
        return res
    }
    // operand is valid
    ua = (*rhs).lsu.as_ptr(); // bottom-up
    uc = (*res).lsu.as_mut_ptr(); // ..
    msua =
        ua.offset((if (*rhs).digits <= 49 as libc::c_int {
                       d2utable[(*rhs).digits as usize] as libc::c_int
                   } else {
                       ((*rhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of rhs
    msuc =
        uc.offset((if (*set).digits <= 49 as libc::c_int {
                       d2utable[(*set).digits as usize] as libc::c_int
                   } else {
                       ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of result
    msudigs =
        (*set).digits -
            ((if (*set).digits <= 49 as libc::c_int {
                  d2utable[(*set).digits as usize] as libc::c_int
              } else {
                  ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                      3 as libc::c_int
              }) - 1 as libc::c_int) *
                3 as libc::c_int; // [faster than remainder]
    while uc <= msuc { // each unit
        // Unit loop
        let mut a: uint16_t = 0; // extract unit
        let mut i: int32_t = 0; // work
        let mut j: int32_t = 0; // can now write back
        if ua > msua { a = 0 as libc::c_int as uint16_t } else { a = *ua }
        *uc = 0 as libc::c_int as uint16_t;
        // always need to examine all bits in rhs
    // This loop could be unrolled and/or use BIN2BCD tables
        i = 0 as libc::c_int; // effect INVERT
        while i < 3 as libc::c_int {
            if !(a as libc::c_int) & 1 as libc::c_int != 0 {
                *uc =
                    (*uc as libc::c_int +
                         DECPOWERS[i as usize] as uint16_t as libc::c_int) as
                        uint16_t
            }
            j = a as libc::c_int % 10 as libc::c_int;
            a = (a as libc::c_int / 10 as libc::c_int) as uint16_t;
            if j > 1 as libc::c_int {
                decStatus(res, 0x80 as libc::c_int as uint32_t, set);
                return res
            }
            if uc == msuc && i == msudigs - 1 as libc::c_int { break ; }
            i += 1
            // just did final digit
        }
        ua = ua.offset(1);
        uc = uc.offset(1)
    }
    // [here uc-1 is the msu of the result]
    (*res).digits =
        decGetDigits((*res).lsu.as_mut_ptr(),
                     uc.wrapping_offset_from((*res).lsu.as_mut_ptr()) as
                         libc::c_long as int32_t); // integer
    (*res).exponent = 0 as libc::c_int; // sign=0
    (*res).bits = 0 as libc::c_int as uint8_t;
    return res;
    // [no status to set]
}
// decNumberInvert
/* ------------------------------------------------------------------ */
/* decNumberLn -- natural logarithm                                   */
/*                                                                    */
/*   This computes C = ln(A)                                          */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context; note that rounding mode has no effect        */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Notable cases:                                                     */
/*   A<0 -> Invalid                                                   */
/*   A=0 -> -Infinity (Exact)                                         */
/*   A=+Infinity -> +Infinity (Exact)                                 */
/*   A=1 exactly -> 0 (Exact)                                         */
/*                                                                    */
/* Mathematical function restrictions apply (see above); a NaN is     */
/* returned with Invalid_operation if a restriction is violated.      */
/*                                                                    */
/* An Inexact result is rounded using DEC_ROUND_HALF_EVEN; it will    */
/* almost always be correctly rounded, but may be up to 1 ulp in      */
/* error in rare cases.                                               */
/* ------------------------------------------------------------------ */
/* This is a wrapper for decLnOp which can handle the slightly wider  */
/* (+11) range needed by Ln, Log10, etc. (which may have to be able   */
/* to calculate at p+e+2).                                            */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberLn(mut res: *mut decNumber,
                                     mut rhs: *const decNumber,
                                     mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    // Check restrictions; this is a math function; if not violated
  // then carry out the operation.
    if decCheckMath(rhs, set, &mut status) == 0 { // end protected
        // protect allocation
        decLnOp(res, rhs, set, &mut status);
    }
    // apply significant status
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberLn
/* ------------------------------------------------------------------ */
/* decNumberLogB - get adjusted exponent, by 754 rules                */
/*                                                                    */
/*   This computes C = adjustedexponent(A)                            */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context, used only for digits and status              */
/*                                                                    */
/* For an unrounded result, digits may need to be 10 (A might have    */
/* 10**9 digits and an exponent of +999999999, or one digit and an    */
/* exponent of -1999999999).                                          */
/*                                                                    */
/* This returns the adjusted exponent of A after (in theory) padding  */
/* with zeros on the right to set->digits digits while keeping the    */
/* same value.  The exponent is not limited by emin/emax.             */
/*                                                                    */
/* Notable cases:                                                     */
/*   A<0 -> Use |A|                                                   */
/*   A=0 -> -Infinity (Division by zero)                              */
/*   A=Infinite -> +Infinity (Exact)                                  */
/*   A=1 exactly -> 0 (Exact)                                         */
/*   NaNs are propagated as usual                                     */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberLogB(mut res: *mut decNumber,
                                       mut rhs: *const decNumber,
                                       mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    // NaNs as usual; Infinities return +Infinity; 0->oops
    if (*rhs).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int {
        decNaNs(res, rhs, 0 as *const decNumber, set,
                &mut status); // prepare for Infinity
    } else if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                  0 as libc::c_int {
        decNumberCopyAbs(res, rhs);
    } else if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*rhs).digits == 1 as libc::c_int &&
                  (*rhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        decNumberZero(res);
        // as per 754
        (*res).bits =
            (0x80 as libc::c_int | 0x40 as libc::c_int) as
                uint8_t; // -Infinity
        status |= 0x2 as libc::c_int as libc::c_uint
    } else {
        // finite non-zero
        let mut ae: int32_t =
            (*rhs).exponent + (*rhs).digits -
                1 as libc::c_int; // adjusted exponent
        if (*set).digits >= 10 as libc::c_int {
            decNumberFromInt32(res, ae); // lay it out
        } else {
            let mut buft: [decNumber; 2] =
                [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
                    2]; // temporary number
            let mut t: *mut decNumber = buft.as_mut_ptr(); // ..
            decNumberFromInt32(t, ae); // lay it out
            decNumberPlus(res, t, set);
        }
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberLogB
/* ------------------------------------------------------------------ */
/* decNumberLog10 -- logarithm in base 10                             */
/*                                                                    */
/*   This computes C = log10(A)                                       */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context; note that rounding mode has no effect        */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Notable cases:                                                     */
/*   A<0 -> Invalid                                                   */
/*   A=0 -> -Infinity (Exact)                                         */
/*   A=+Infinity -> +Infinity (Exact)                                 */
/*   A=10**n (if n is an integer) -> n (Exact)                        */
/*                                                                    */
/* Mathematical function restrictions apply (see above); a NaN is     */
/* returned with Invalid_operation if a restriction is violated.      */
/*                                                                    */
/* An Inexact result is rounded using DEC_ROUND_HALF_EVEN; it will    */
/* almost always be correctly rounded, but may be up to 1 ulp in      */
/* error in rare cases.                                               */
/* ------------------------------------------------------------------ */
/* This calculates ln(A)/ln(10) using appropriate precision.  For     */
/* ln(A) this is the max(p, rhs->digits + t) + 3, where p is the      */
/* requested digits and t is the number of digits in the exponent     */
/* (maximum 6).  For ln(10) it is p + 3; this is often handled by the */
/* fastpath in decLnOp.  The final division is done to the requested  */
/* precision.                                                         */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberLog10(mut res: *mut decNumber,
                                        mut rhs: *const decNumber,
                                        mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t =
        0 as libc::c_int as uint32_t; // status accumulators
    let mut ignore: uint32_t =
        0 as libc::c_int as uint32_t; // for space calculations
    let mut needbytes: uint32_t = 0; // working precision
    let mut p: int32_t = 0; // digits in exponent of A
    let mut t: int32_t = 0;
    // buffers for a and b working decimals
  // (adjustment calculator, same size)
    let mut bufa: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            3]; // -> allocated bufa, iff allocated
    let mut allocbufa: *mut decNumber = 0 as *mut decNumber; // temporary a
    let mut a: *mut decNumber =
        bufa.as_mut_ptr(); // -> allocated bufb, iff allocated
    let mut bufb: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            3]; // temporary b
    let mut allocbufb: *mut decNumber =
        0 as *mut decNumber; // working 2-10 digit number
    let mut b: *mut decNumber = bufb.as_mut_ptr(); // ..
    let mut bufw: [decNumber; 2] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            2]; // working context
    let mut w: *mut decNumber = bufw.as_mut_ptr();
    let mut aset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,};
    // Check restrictions; this is a math function; if not violated
  // then carry out the operation.
    if decCheckMath(rhs, set, &mut status) == 0 {
        let mut current_block_30: u64; // [for break]
        // protect malloc
        decContextDefault(&mut aset, 64 as libc::c_int); // clean context
        // into result
        if (*rhs).bits as libc::c_int &
               (0x80 as libc::c_int |
                    (0x40 as libc::c_int | 0x20 as libc::c_int |
                         0x10 as libc::c_int)) == 0 &&
               !(*(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                     (*rhs).digits == 1 as libc::c_int &&
                     (*rhs).bits as libc::c_int &
                         (0x40 as libc::c_int | 0x20 as libc::c_int |
                              0x10 as libc::c_int) == 0 as libc::c_int) {
            // handle exact powers of 10; only check if +ve finite
            // not a candidate for exact
            let mut residue: int32_t = 0 as libc::c_int; // (no residue)
            let mut copystat: uint32_t =
                0 as libc::c_int as uint32_t; // clean status
            // round to a single digit...
            aset.digits = 1 as libc::c_int; // copy & shorten
            decCopyFit(w, rhs, &mut aset, &mut residue, &mut copystat);
            // if exact and the digit is 1, rhs is a power of 10
            if copystat & 0x20 as libc::c_int as libc::c_uint == 0 &&
                   *(*w).lsu.as_mut_ptr().offset(0 as libc::c_int as isize) as
                       libc::c_int == 1 as libc::c_int {
                // the exponent, conveniently, is the power of 10; making
        // this the result needs a little care as it might not fit,
        // so first convert it into the working number, and then move
        // to res
                decNumberFromInt32(w, (*w).exponent); // copy & round
                residue = 0 as libc::c_int; // cleanup/set flags
                decCopyFit(res, w, set, &mut residue, &mut status);
                decFinalize(res, set, &mut residue, &mut status);
                current_block_30 = 6476622998065200121;
            } else { current_block_30 = 13242334135786603907; }
            // not a power of 10
        } else { current_block_30 = 13242334135786603907; }
        match current_block_30 {
            13242334135786603907 => {
                // simplify the information-content calculation to use 'total
    // number of digits in a, including exponent' as compared to the
    // requested digits, as increasing this will only rarely cost an
    // iteration in ln(a) anyway
                t = 6 as libc::c_int; // it can never be >6
                // allocate space when needed...
                p =
                    (if (*rhs).digits + t > (*set).digits {
                         ((*rhs).digits) + t
                     } else { (*set).digits }) + 3 as libc::c_int;
                needbytes =
                    (::std::mem::size_of::<decNumber>() as
                         libc::c_ulong).wrapping_add((((if p <=
                                                               49 as
                                                                   libc::c_int
                                                           {
                                                            d2utable[p as
                                                                         usize]
                                                                as libc::c_int
                                                        } else {
                                                            (p +
                                                                 3 as
                                                                     libc::c_int
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                /
                                                                3 as
                                                                    libc::c_int
                                                        }) - 1 as libc::c_int)
                                                          as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                          as
                                                                                          libc::c_ulong))
                        as uint32_t;
                if needbytes as libc::c_ulong >
                       ::std::mem::size_of::<[decNumber; 3]>() as
                           libc::c_ulong {
                    // need malloc space
                    allocbufa =
                        malloc(needbytes as libc::c_ulong) as *mut decNumber;
                    if allocbufa.is_null() {
                        // use the allocated space
                        // hopeless -- abandon
                        status |=
                            0x10 as libc::c_int as
                                libc::c_uint; // as calculated
                        current_block_30 =
                            6476622998065200121; // usual bounds
                    } else {
                        a = allocbufa; // ..
                        current_block_30 =
                            14763689060501151050; // and no concrete format
                    }
                } else {
                    current_block_30 = 14763689060501151050; // a=ln(rhs)
                }
                match current_block_30 {
                    6476622998065200121 => { }
                    _ => {
                        aset.digits = p;
                        aset.emax = 999999 as libc::c_int;
                        aset.emin = -(999999 as libc::c_int);
                        aset.clamp = 0 as libc::c_int as uint8_t;
                        decLnOp(a, rhs, &mut aset, &mut status);
                        // skip the division if the result so far is infinite, NaN, or
    // zero, or there was an error; note NaN from sNaN needs copy
                        if !(status &
                                 (0x1 as libc::c_int | 0x4 as libc::c_int |
                                      0x8 as libc::c_int | 0x10 as libc::c_int
                                      | 0x40 as libc::c_int |
                                      0x80 as libc::c_int) as libc::c_uint !=
                                 0 &&
                                 status &
                                     0x40000000 as libc::c_int as libc::c_uint
                                     == 0) {
                            if (*a).bits as libc::c_int &
                                   (0x40 as libc::c_int | 0x20 as libc::c_int
                                        | 0x10 as libc::c_int) != 0 ||
                                   *(*a).lsu.as_mut_ptr() as libc::c_int ==
                                       0 as libc::c_int &&
                                       (*a).digits == 1 as libc::c_int &&
                                       (*a).bits as libc::c_int &
                                           (0x40 as libc::c_int |
                                                0x20 as libc::c_int |
                                                0x10 as libc::c_int) ==
                                           0 as libc::c_int {
                                decNumberCopy(res, a); // [will fit]
                            } else {
                                // for ln(10) an extra 3 digits of precision are needed
                                p = (*set).digits + 3 as libc::c_int;
                                needbytes =
                                    (::std::mem::size_of::<decNumber>() as
                                         libc::c_ulong).wrapping_add((((if p
                                                                               <=
                                                                               49
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            d2utable[p
                                                                                         as
                                                                                         usize]
                                                                                as
                                                                                libc::c_int
                                                                        } else {
                                                                            (p
                                                                                 +
                                                                                 3
                                                                                     as
                                                                                     libc::c_int
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                /
                                                                                3
                                                                                    as
                                                                                    libc::c_int
                                                                        }) -
                                                                           1
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                                          as
                                                                                                          libc::c_ulong))
                                        as uint32_t;
                                if needbytes as libc::c_ulong >
                                       ::std::mem::size_of::<[decNumber; 3]>()
                                           as libc::c_ulong {
                                    // need malloc space
                                    allocbufb =
                                        malloc(needbytes as libc::c_ulong) as
                                            *mut decNumber;
                                    if allocbufb.is_null() {
                                        // use the allocated space
                                        // hopeless -- abandon
                                        status |=
                                            0x10 as libc::c_int as
                                                libc::c_uint; // set up 10...
                                        current_block_30 =
                                            6476622998065200121; // ..
                                    } else {
                                        b = allocbufb; // ..
                                        current_block_30 =
                                            15512526488502093901; // b=ln(10)
                                    }
                                } else {
                                    current_block_30 =
                                        15512526488502093901; // for final divide
                                } // drop any storage used
                                match current_block_30 {
                                    6476622998065200121 => { }
                                    _ => {
                                        decNumberZero(w); // ..
                                        *(*w).lsu.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                            = 10 as libc::c_int as uint16_t;
                                        (*w).digits = 2 as libc::c_int;
                                        aset.digits = p;
                                        decLnOp(b, w, &mut aset, &mut ignore);
                                        aset.digits = (*set).digits;
                                        decDivideOp(res, a, b, &mut aset,
                                                    0x80 as libc::c_int as
                                                        uint8_t, &mut status);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
    }
    if !allocbufa.is_null() { free(allocbufa as *mut libc::c_void); }
    if !allocbufb.is_null() { free(allocbufb as *mut libc::c_void); }
    // apply significant status
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberLog10
/* ------------------------------------------------------------------ */
/* decNumberMax -- compare two Numbers and return the maximum         */
/*                                                                    */
/*   This computes C = A ? B, returning the maximum by 754 rules      */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberMax(mut res: *mut decNumber,
                                      mut lhs: *const decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x2 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberMax
/* ------------------------------------------------------------------ */
/* decNumberMaxMag -- compare and return the maximum by magnitude     */
/*                                                                    */
/*   This computes C = A ? B, returning the maximum by 754 rules      */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberMaxMag(mut res: *mut decNumber,
                                         mut lhs: *const decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x7 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberMaxMag
/* ------------------------------------------------------------------ */
/* decNumberMin -- compare two Numbers and return the minimum         */
/*                                                                    */
/*   This computes C = A ? B, returning the minimum by 754 rules      */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberMin(mut res: *mut decNumber,
                                      mut lhs: *const decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x3 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberMin
/* ------------------------------------------------------------------ */
/* decNumberMinMag -- compare and return the minimum by magnitude     */
/*                                                                    */
/*   This computes C = A ? B, returning the minimum by 754 rules      */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberMinMag(mut res: *mut decNumber,
                                         mut lhs: *const decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decCompareOp(res, lhs, rhs, set, 0x8 as libc::c_int as uint8_t,
                 &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberMinMag
/* ------------------------------------------------------------------ */
/* decNumberMinus -- prefix minus operator                            */
/*                                                                    */
/*   This computes C = 0 - A                                          */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context                                               */
/*                                                                    */
/* See also decNumberCopyNegate for a quiet bitwise version of this.  */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
/* Simply use AddOp for the subtract, which will do the necessary.    */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberMinus(mut res: *mut decNumber,
                                        mut rhs: *const decNumber,
                                        mut set: *mut decContext)
 -> *mut decNumber {
    let mut dzero: decNumber =
        decNumber{digits: 0,
                  exponent: 0,
                  bits: 0,
                  lsu: [0; 1],}; // accumulator
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // make 0
    decNumberZero(&mut dzero); // [no coefficient expansion]
    dzero.exponent = (*rhs).exponent;
    decAddOp(res, &mut dzero, rhs, set, 0x80 as libc::c_int as uint8_t,
             &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberMinus
/* ------------------------------------------------------------------ */
/* decNumberNextMinus -- next towards -Infinity                       */
/*                                                                    */
/*   This computes C = A - infinitesimal, rounded towards -Infinity   */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context                                               */
/*                                                                    */
/* This is a generalization of 754 NextDown.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberNextMinus(mut res: *mut decNumber,
                                            mut rhs: *const decNumber,
                                            mut set: *mut decContext)
 -> *mut decNumber {
    let mut dtiny: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; // constant
    let mut workset: decContext = *set; // work
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    // +Infinity is the special case
    if (*rhs).bits as libc::c_int &
           (0x40 as libc::c_int | 0x80 as libc::c_int) == 0x40 as libc::c_int
       {
        decSetMaxValue(res, set); // is +ve
        // there is no status to set
        return res
    } // start with 0
    decNumberZero(&mut dtiny); // make number that is ..
    *dtiny.lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
        1 as libc::c_int as uint16_t; // .. smaller than tiniest
    dtiny.exponent =
        -(999999999 as libc::c_int) -
            1 as libc::c_int; // only sNaN Invalid please
    workset.round = DEC_ROUND_FLOOR;
    decAddOp(res, rhs, &mut dtiny, &mut workset,
             0x80 as libc::c_int as uint8_t, &mut status);
    status &=
        (0x80 as libc::c_int | 0x40000000 as libc::c_int) as libc::c_uint;
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
/* ------------------------------------------------------------------ */
/* Decimal Number arithmetic module header                            */
/* ------------------------------------------------------------------ */
/* Copyright (c) IBM Corporation, 2000, 2010.  All rights reserved.   */
/*                                                                    */
/* This software is made available under the terms of the             */
/* ICU License -- ICU 1.8.1 and later.                                */
/*                                                                    */
/* The description and User's Guide ("The decNumber C Library") for   */
/* this software is called decNumber.pdf.  This document is           */
/* available, together with arithmetic and format specifications,     */
/* testcases, and Web links, on the General Decimal Arithmetic page.  */
/*                                                                    */
/* Please send comments, suggestions, and corrections to the author:  */
/*   mfc@uk.ibm.com                                                   */
/*   Mike Cowlishaw, IBM Fellow                                       */
/*   IBM UK, PO Box 31, Birmingham Road, Warwick CV34 5JL, UK         */
/* ------------------------------------------------------------------ */
/* Short name */
/* Verbose name */
/* Who to blame */
/* Bit settings for decNumber.bits                                  */
/* Sign; 1=negative, 0=positive or zero */
/* 1=Infinity                           */
/* 1=NaN                                */
/* 1=sNaN                               */
/* The remaining bits are reserved; they must be 0                  */
/* any special value     */
/* Define the decNumber data structure.  The size and shape of the  */
  /* units array in the structure is determined by the following      */
  /* constant.  This must not be changed without recompiling the      */
  /* decNumber library modules. */
/* DECimal Digits Per UNit [must be >0  */
                              /* and <10; 3 or powers of 2 are best]. */
/* DECNUMDIGITS is the default number of digits that can be held in */
  /* the structure.  If undefined, 1 is assumed and it is assumed     */
  /* that the structure will be immediately followed by extra space,  */
  /* as required.  DECNUMDIGITS is always >0.                         */
/* The size (integer data type) of each unit is determined by the   */
  /* number of digits it will hold.                                   */
/* The number of units needed is ceil(DECNUMDIGITS/DECDPUN)         */
/* The data structure... */
/* Count of digits in the coefficient; >0    */
/* Unadjusted exponent, unbiased, in         */
                         /* range: -1999999997 through 999999999      */
/* Indicator bits (see above)                */
                         /* Coefficient, from least significant unit  */
/* Notes:                                                           */
  /* 1. If digits is > DECDPUN then there will one or more            */
  /*    decNumberUnits immediately following the first element of lsu.*/
  /*    These contain the remaining (more significant) digits of the  */
  /*    number, and may be in the lsu array, or may be guaranteed by  */
  /*    some other mechanism (such as being contained in another      */
  /*    structure, or being overlaid on dynamically allocated         */
  /*    storage).                                                     */
  /*                                                                  */
  /*    Each integer of the coefficient (except potentially the last) */
  /*    contains DECDPUN digits (e.g., a value in the range 0 through */
  /*    99999999 if DECDPUN is 8, or 0 through 999 if DECDPUN is 3).  */
  /*                                                                  */
  /* 2. A decNumber converted to a string may need up to digits+14    */
  /*    characters.  The worst cases (non-exponential and exponential */
  /*    formats) are -0.00000{9...}# and -9.{9...}E+999999999#        */
  /*    (where # is '\0')                                             */
/* ---------------------------------------------------------------- */
  /* decNumber public functions and macros                            */
  /* ---------------------------------------------------------------- */
  /* Conversions                                                      */
/* Operators and elementary functions                               */
/* Utilities                                                        */
// decNumberNextMinus
/* ------------------------------------------------------------------ */
/* decNumberNextPlus -- next towards +Infinity                        */
/*                                                                    */
/*   This computes C = A + infinitesimal, rounded towards +Infinity   */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context                                               */
/*                                                                    */
/* This is a generalization of 754 NextUp.                            */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberNextPlus(mut res: *mut decNumber,
                                           mut rhs: *const decNumber,
                                           mut set: *mut decContext)
 -> *mut decNumber {
    let mut dtiny: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; // constant
    let mut workset: decContext = *set; // work
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    // -Infinity is the special case
    if (*rhs).bits as libc::c_int &
           (0x40 as libc::c_int | 0x80 as libc::c_int) ==
           0x40 as libc::c_int | 0x80 as libc::c_int {
        decSetMaxValue(res, set); // negative
        (*res).bits = 0x80 as libc::c_int as uint8_t;
        // there is no status to set
        return res
    } // start with 0
    decNumberZero(&mut dtiny); // make number that is ..
    *dtiny.lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
        1 as libc::c_int as uint16_t; // .. smaller than tiniest
    dtiny.exponent =
        -(999999999 as libc::c_int) -
            1 as libc::c_int; // only sNaN Invalid please
    workset.round = DEC_ROUND_CEILING;
    decAddOp(res, rhs, &mut dtiny, &mut workset, 0 as libc::c_int as uint8_t,
             &mut status);
    status &=
        (0x80 as libc::c_int | 0x40000000 as libc::c_int) as libc::c_uint;
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberNextPlus
/* ------------------------------------------------------------------ */
/* decNumberNextToward -- next towards rhs                            */
/*                                                                    */
/*   This computes C = A +/- infinitesimal, rounded towards           */
/*   +/-Infinity in the direction of B, as per 754-1985 nextafter     */
/*   modified during revision but dropped from 754-2008.              */
/*                                                                    */
/*   res is C, the result.  C may be A or B.                          */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* This is a generalization of 754-1985 NextAfter.                    */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberNextToward(mut res: *mut decNumber,
                                             mut lhs: *const decNumber,
                                             mut rhs: *const decNumber,
                                             mut set: *mut decContext)
 -> *mut decNumber {
    let mut dtiny: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; // constant
    let mut workset: decContext = *set; // work
    let mut result: int32_t = 0; // ..
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    if (*lhs).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
       {
        decNaNs(res, lhs, rhs, set, &mut status); // numeric
    } else {
        // Is numeric, so no chance of sNaN Invalid, etc.
        result = decCompare(lhs, rhs, 0 as libc::c_int as uint8_t);
        if result == 0x80000000 as libc::c_uint as int32_t { // sign matters
            // compare OK
            status |= 0x10 as libc::c_int as libc::c_uint
        } else if result == 0 as libc::c_int { // rare
            // valid compare
            decNumberCopySign(res, lhs, rhs); // easy
        } else {
            // differ: need NextPlus or NextMinus
            let mut sub: uint8_t = 0; // add or subtract
            if result < 0 as libc::c_int { // minus
                // lhs<rhs, do nextplus
                // -Infinity is the special case
                if (*lhs).bits as libc::c_int &
                       (0x40 as libc::c_int | 0x80 as libc::c_int) ==
                       0x40 as libc::c_int | 0x80 as libc::c_int {
                    decSetMaxValue(res, set);
                    // there is no status to set
                    (*res).bits = 0x80 as libc::c_int as uint8_t; // negative
                    return res
                }
                workset.round = DEC_ROUND_CEILING;
                sub = 0 as libc::c_int as uint8_t
                // add, please
            } else {
                // lhs>rhs, do nextminus
                // +Infinity is the special case
                if (*lhs).bits as libc::c_int &
                       (0x40 as libc::c_int | 0x80 as libc::c_int) ==
                       0x40 as libc::c_int {
                    decSetMaxValue(res, set);
                    return res
                    // there is no status to set
                }
                workset.round = DEC_ROUND_FLOOR;
                sub = 0x80 as libc::c_int as uint8_t
                // subtract, please
            } // start with 0
            decNumberZero(&mut dtiny); // make number that is ..
            *dtiny.lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
                1 as libc::c_int as uint16_t; // .. smaller than tiniest
            dtiny.exponent =
                -(999999999 as libc::c_int) - 1 as libc::c_int; // + or -
            decAddOp(res, lhs, &mut dtiny, &mut workset, sub, &mut status);
            // turn off exceptions if the result is a normal number
        // (including Nmin), otherwise let all status through
            if decNumberIsNormal(res, set) != 0 {
                status = 0 as libc::c_int as uint32_t
            }
        }
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// unequal
// decNumberNextToward
/* ------------------------------------------------------------------ */
/* decNumberOr -- OR two Numbers, digitwise                           */
/*                                                                    */
/*   This computes C = A | B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X|X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context (used for result length and error report)     */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Logical function restrictions apply (see above); a NaN is          */
/* returned with Invalid_operation if a restriction is violated.      */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberOr(mut res: *mut decNumber,
                                     mut lhs: *const decNumber,
                                     mut rhs: *const decNumber,
                                     mut set: *mut decContext)
 -> *mut decNumber {
    let mut ua: *const uint16_t = 0 as *const uint16_t; // -> operands
    let mut ub: *const uint16_t = 0 as *const uint16_t; // -> operand msus
    let mut msua: *const uint16_t =
        0 as *const uint16_t; // -> result and its msu
    let mut msub: *const uint16_t = 0 as *const uint16_t; // digits in res msu
    let mut uc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msuc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msudigs: int32_t = 0;
    if (*lhs).exponent != 0 as libc::c_int ||
           (*lhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int || (*rhs).exponent != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
        decStatus(res, 0x80 as libc::c_int as uint32_t, set);
        return res
    }
    // operands are valid
    ua = (*lhs).lsu.as_ptr(); // bottom-up
    ub = (*rhs).lsu.as_ptr(); // ..
    uc = (*res).lsu.as_mut_ptr(); // ..
    msua =
        ua.offset((if (*lhs).digits <= 49 as libc::c_int {
                       d2utable[(*lhs).digits as usize] as libc::c_int
                   } else {
                       ((*lhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of lhs
    msub =
        ub.offset((if (*rhs).digits <= 49 as libc::c_int {
                       d2utable[(*rhs).digits as usize] as libc::c_int
                   } else {
                       ((*rhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of rhs
    msuc =
        uc.offset((if (*set).digits <= 49 as libc::c_int {
                       d2utable[(*set).digits as usize] as libc::c_int
                   } else {
                       ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of result
    msudigs =
        (*set).digits -
            ((if (*set).digits <= 49 as libc::c_int {
                  d2utable[(*set).digits as usize] as libc::c_int
              } else {
                  ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                      3 as libc::c_int
              }) - 1 as libc::c_int) *
                3 as libc::c_int; // [faster than remainder]
    while uc <= msuc { // each unit
        // Unit loop
        let mut a: uint16_t = 0; // extract units
        let mut b: uint16_t = 0; // can now write back
        if ua > msua { a = 0 as libc::c_int as uint16_t } else { a = *ua }
        if ub > msub { b = 0 as libc::c_int as uint16_t } else { b = *ub }
        *uc = 0 as libc::c_int as uint16_t;
        if a as libc::c_int | b as libc::c_int != 0 {
            // maybe 1 bits to examine
            let mut i: int32_t = 0;
            let mut j: int32_t = 0;
            // This loop could be unrolled and/or use BIN2BCD tables
            i = 0 as libc::c_int; // effect OR
            while i < 3 as libc::c_int {
                if (a as libc::c_int | b as libc::c_int) & 1 as libc::c_int !=
                       0 {
                    *uc =
                        (*uc as libc::c_int +
                             DECPOWERS[i as usize] as uint16_t as libc::c_int)
                            as uint16_t
                }
                j = a as libc::c_int % 10 as libc::c_int;
                a = (a as libc::c_int / 10 as libc::c_int) as uint16_t;
                j |= b as libc::c_int % 10 as libc::c_int;
                b = (b as libc::c_int / 10 as libc::c_int) as uint16_t;
                if j > 1 as libc::c_int {
                    decStatus(res, 0x80 as libc::c_int as uint32_t, set);
                    return res
                }
                if uc == msuc && i == msudigs - 1 as libc::c_int { break ; }
                i += 1
                // just did final digit
            }
        }
        ua = ua.offset(1);
        ub = ub.offset(1);
        uc = uc.offset(1)
    }
    // [here uc-1 is the msu of the result]
    (*res).digits =
        decGetDigits((*res).lsu.as_mut_ptr(),
                     uc.wrapping_offset_from((*res).lsu.as_mut_ptr()) as
                         libc::c_long as int32_t); // integer
    (*res).exponent = 0 as libc::c_int; // sign=0
    (*res).bits = 0 as libc::c_int as uint8_t;
    return res;
    // [no status to set]
}
// decNumberOr
/* ------------------------------------------------------------------ */
/* decNumberPlus -- prefix plus operator                              */
/*                                                                    */
/*   This computes C = 0 + A                                          */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context                                               */
/*                                                                    */
/* See also decNumberCopy for a quiet bitwise version of this.        */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
/* This simply uses AddOp; Add will take fast path after preparing A. */
/* Performance is a concern here, as this routine is often used to    */
/* check operands and apply rounding and overflow/underflow testing.  */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberPlus(mut res: *mut decNumber,
                                       mut rhs: *const decNumber,
                                       mut set: *mut decContext)
 -> *mut decNumber {
    let mut dzero: decNumber =
        decNumber{digits: 0,
                  exponent: 0,
                  bits: 0,
                  lsu: [0; 1],}; // accumulator
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // make 0
    decNumberZero(&mut dzero); // [no coefficient expansion]
    dzero.exponent = (*rhs).exponent;
    decAddOp(res, &mut dzero, rhs, set, 0 as libc::c_int as uint8_t,
             &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberPlus
/* ------------------------------------------------------------------ */
/* decNumberMultiply -- multiply two Numbers                          */
/*                                                                    */
/*   This computes C = A x B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X+X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberMultiply(mut res: *mut decNumber,
                                           mut lhs: *const decNumber,
                                           mut rhs: *const decNumber,
                                           mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decMultiplyOp(res, lhs, rhs, set, &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberMultiply
/* ------------------------------------------------------------------ */
/* decNumberPower -- raise a number to a power                        */
/*                                                                    */
/*   This computes C = A ** B                                         */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X**X)        */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Mathematical function restrictions apply (see above); a NaN is     */
/* returned with Invalid_operation if a restriction is violated.      */
/*                                                                    */
/* However, if 1999999997<=B<=999999999 and B is an integer then the  */
/* restrictions on A and the context are relaxed to the usual bounds, */
/* for compatibility with the earlier (integer power only) version    */
/* of this function.                                                  */
/*                                                                    */
/* When B is an integer, the result may be exact, even if rounded.    */
/*                                                                    */
/* The final result is rounded according to the context; it will      */
/* almost always be correctly rounded, but may be up to 1 ulp in      */
/* error in rare cases.                                               */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberPower(mut res: *mut decNumber,
                                        mut lhs: *const decNumber,
                                        mut rhs: *const decNumber,
                                        mut set: *mut decContext)
 -> *mut decNumber {
    let mut allocdac: *mut decNumber =
        0 as *mut decNumber; // -> allocated acc buffer, iff used
    let mut allocinv: *mut decNumber =
        0 as *mut decNumber; // -> allocated 1/x buffer, iff used
    let mut reqdigits: int32_t = (*set).digits; // requested DIGITS
    let mut n: int32_t = 0; // rhs in binary
    let mut rhsint: uint8_t =
        0 as libc::c_int as uint8_t; // 1 if rhs is an integer
    let mut useint: uint8_t =
        0 as libc::c_int as uint8_t; // 1 if can use integer calculation
    let mut isoddint: uint8_t =
        0 as libc::c_int as uint8_t; // 1 if rhs is an integer and odd
    let mut i: int32_t = 0; // work
    let mut needbytes: uint32_t = 0; // buffer size needed
    let mut seenbit: uint8_t = 0; // seen a bit while powering
    let mut residue: int32_t = 0 as libc::c_int; // rounding residue
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulators
    let mut bits: uint8_t =
        0 as libc::c_int as uint8_t; // result sign if errors
    let mut aset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // working context
    let mut dnOne: decNumber =
        decNumber{digits: 0,
                  exponent: 0,
                  bits: 0,
                  lsu: [0; 1],}; // work value 1...
    // local accumulator buffer [a decNumber, with digits+elength+1 digits]
    let mut dacbuff: [decNumber; 4] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            4]; // -> result accumulator
    let mut dac: *mut decNumber = dacbuff.as_mut_ptr();
    // same again for possible 1/lhs calculation
    let mut invbuff: [decNumber; 4] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; 4];
    let mut current_block_99: u64;
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        if (*lhs).bits as libc::c_int &
               (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
               ||
               (*rhs).bits as libc::c_int &
                   (0x20 as libc::c_int | 0x10 as libc::c_int) !=
                   0 as libc::c_int {
            // NaNs
            decNaNs(res, lhs, rhs, set, &mut status);
            current_block_99 = 13174377073168946860;
        } else if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                      0 as libc::c_int {
            // rhs Infinity
            let mut rhsneg: uint8_t =
                ((*rhs).bits as libc::c_int & 0x80 as libc::c_int) as
                    uint8_t; // save rhs sign
            if (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int &&
                   !(*(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int
                         && (*lhs).digits == 1 as libc::c_int &&
                         (*lhs).bits as libc::c_int &
                             (0x40 as libc::c_int | 0x20 as libc::c_int |
                                  0x10 as libc::c_int) == 0 as libc::c_int)
               { // lhs>=0
                // ..
                status |= 0x80 as libc::c_int as libc::c_uint
            } else {
                // lhs >=0
                decNumberZero(&mut dnOne); // set up 1
                *dnOne.lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
                    1 as libc::c_int as uint16_t; // lhs ? 1
                decNumberCompare(dac, lhs, &mut dnOne,
                                 set); // prepare for 0/1/Infinity
                decNumberZero(res);
                if (*dac).bits as libc::c_int & 0x80 as libc::c_int !=
                       0 as libc::c_int {
                    // lhs<1
                    if rhsneg != 0 {
                        (*res).bits =
                            ((*res).bits as libc::c_int | 0x40 as libc::c_int)
                                as uint8_t
                    }
                    // +Infinity [else is +0]
                } else if *(*dac).lsu.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                              libc::c_int == 0 as libc::c_int {
                    // lhs=1
                    // 1**Infinity is inexact, so return fully-padded 1.0000
                    let mut shift: int32_t =
                        (*set).digits - 1 as libc::c_int; // was 0, make int 1
                    *(*res).lsu.as_mut_ptr() =
                        1 as libc::c_int as uint16_t; // make 1.0000...
                    (*res).digits =
                        decShiftToMost((*res).lsu.as_mut_ptr(),
                                       1 as libc::c_int, shift);
                    (*res).exponent = -shift;
                    status |=
                        (0x20 as libc::c_int | 0x800 as libc::c_int) as
                            libc::c_uint
                } else if rhsneg == 0 {
                    (*res).bits =
                        ((*res).bits as libc::c_int | 0x40 as libc::c_int) as
                            uint8_t
                }
            }
            current_block_99 = 13174377073168946860;
        } else { current_block_99 = 6450636197030046351; }
        // lhs>1
        // +Infinity [else is +0]
        // [lhs infinity drops through]
    } else { current_block_99 = 6450636197030046351; }
    match current_block_99 {
        6450636197030046351 => {
            // final cleanup
            // protect allocated storage
            // [following code does not require input rounding]
            // handle NaNs and rhs Infinity (lhs infinity is harder)
            // specials
            // Original rhs may be an integer that fits and is in range
            n = decGetInt(rhs);
            if n != 0x80000000 as libc::c_uint as int32_t {
                // it is an integer
                rhsint =
                    1 as libc::c_int as uint8_t; // record the fact for 1**n
                // looks good
                isoddint =
                    (n as uint8_t as libc::c_int & 1 as libc::c_int) as
                        uint8_t; // [works even if big]
                if n != 0x80000002 as libc::c_uint as int32_t &&
                       n != 0x80000003 as libc::c_uint as int32_t {
                    // can use integer path?
                    useint = 1 as libc::c_int as uint8_t
                }
            } // .. to an odd power
            if (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int && isoddint as libc::c_int != 0 {
                bits = 0x80 as libc::c_int as uint8_t
            }
            // handle LHS infinity
            if (*lhs).bits as libc::c_int & 0x40 as libc::c_int !=
                   0 as libc::c_int {
                // [NaNs already handled]
                let mut rbits: uint8_t = (*rhs).bits; // save
                decNumberZero(res); // prepare
                if n == 0 as libc::c_int {
                    *(*res).lsu.as_mut_ptr() = 1 as libc::c_int as uint16_t
                } else if rhsint == 0 &&
                              (*lhs).bits as libc::c_int & 0x80 as libc::c_int
                                  != 0 as libc::c_int { // [-]Inf**0 => 1
                    // -Inf**nonint -> error
                    status |= 0x80 as libc::c_int as libc::c_uint
                } else { // -Inf**nonint is error
                    if rbits as libc::c_int & 0x80 as libc::c_int == 0 {
                        bits =
                            (bits as libc::c_int | 0x40 as libc::c_int) as
                                uint8_t
                    } // was not a **-n
                    // [otherwise will be 0 or -0]
                    (*res).bits = bits
                }
            } else if *(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int
                          && (*lhs).digits == 1 as libc::c_int &&
                          (*lhs).bits as libc::c_int &
                              (0x40 as libc::c_int | 0x20 as libc::c_int |
                                   0x10 as libc::c_int) == 0 as libc::c_int {
                if n == 0 as libc::c_int {
                    // similarly handle LHS zero
                    // 0**0 => Error
                    status |= 0x80 as libc::c_int as libc::c_uint
                } else {
                    // 0**x
                    let mut rbits_0: uint8_t = (*rhs).bits; // save
                    if rbits_0 as libc::c_int & 0x80 as libc::c_int != 0 {
                        // was a 0**(-n)
                        bits =
                            (bits as libc::c_int | 0x40 as libc::c_int) as
                                uint8_t
                    } // prepare
                    decNumberZero(res);
                    // [otherwise will be 0 or -0]
                    (*res).bits = bits
                }
            } else {
                // here both lhs and rhs are finite; rhs==0 is handled in the
    // integer path.  Next handle the non-integer cases
                if useint == 0 {
                    if (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                           0 as libc::c_int {
                        status |=
                            0x80 as libc::c_int as
                                libc::c_uint; // integer path
                        current_block_99 =
                            13174377073168946860; // variable status
                    } else if decCheckMath(lhs, set, &mut status) != 0 ||
                                  decCheckMath(rhs, set, &mut status) != 0 {
                        current_block_99 =
                            13174377073168946860; // clean context
                    } else {
                        decContextDefault(&mut aset,
                                          64 as libc::c_int); // usual bounds
                        aset.emax = 999999 as libc::c_int; // ..
                        aset.emin =
                            -(999999 as
                                  libc::c_int); // and no concrete format
                        aset.clamp = 0 as libc::c_int as uint8_t;
                        // calculate the result using exp(ln(lhs)*rhs), which can
      // all be done into the accumulator, dac.  The precision needed
      // is enough to contain the full information in the lhs (which
      // is the total digits, including exponent), or the requested
      // precision, if larger, + 4; 6 is used for the exponent
      // maximum length, and this is also used when it is shorter
      // than the requested digits as it greatly reduces the >0.5 ulp
      // cases at little cost (because Ln doubles digits each
      // iteration so a few extra digits rarely causes an extra
      // iteration)
                        aset.digits =
                            (if (*lhs).digits < (*set).digits {
                                 (*set).digits
                             } else { (*lhs).digits }) + 6 as libc::c_int +
                                4 as libc::c_int; // non-integer rhs
                        current_block_99 = 10213293998891106930;
                    }
                } else if n == 0 as libc::c_int {
                    // non-integral rhs
                    // any -ve lhs is bad, as is either operand or context out of
      // bounds
                    // rhs is in-range integer
                    // x**0 = 1
                    // (0**0 was handled above)
                    decNumberZero(res); // result=1
                    *(*res).lsu.as_mut_ptr() =
                        1 as libc::c_int as uint16_t; // ..
                    current_block_99 = 13174377073168946860;
                } else {
                    // rhs is a non-zero integer
                    if n < 0 as libc::c_int { n = -n } // use abs(n)
                    aset = *set; // clone the context
                    aset.round =
                        DEC_ROUND_HALF_EVEN; // internally use balanced
                    // calculate the working DIGITS
                    aset.digits =
                        reqdigits + ((*rhs).digits + (*rhs).exponent) +
                            2 as libc::c_int;
                    // it's an error if this is more than can be handled
                    if aset.digits > 999999999 as libc::c_int {
                        status |= 0x80 as libc::c_int as libc::c_uint;
                        current_block_99 = 13174377073168946860;
                    } else { current_block_99 = 10213293998891106930; }
                }
                match current_block_99 {
                    13174377073168946860 => { }
                    _ => {
                        // aset.digits is the count of digits for the accumulator needed
    // if accumulator is too long for local storage, then allocate
                        needbytes =
                            (::std::mem::size_of::<decNumber>() as
                                 libc::c_ulong).wrapping_add((((if aset.digits
                                                                       <=
                                                                       49 as
                                                                           libc::c_int
                                                                   {
                                                                    d2utable[aset.digits
                                                                                 as
                                                                                 usize]
                                                                        as
                                                                        libc::c_int
                                                                } else {
                                                                    (aset.digits
                                                                         +
                                                                         3 as
                                                                             libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        /
                                                                        3 as
                                                                            libc::c_int
                                                                }) -
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as uint32_t;
                        // [needbytes also used below if 1/lhs needed]
                        if needbytes as libc::c_ulong >
                               ::std::mem::size_of::<[decNumber; 4]>() as
                                   libc::c_ulong {
                            allocdac =
                                malloc(needbytes as libc::c_ulong) as
                                    *mut decNumber;
                            if allocdac.is_null() {
                                // use the allocated space
                                // hopeless -- abandon
                                status |= 0x10 as libc::c_int as libc::c_uint;
                                current_block_99 = 13174377073168946860;
                            } else {
                                dac = allocdac;
                                current_block_99 = 11071260907632769126;
                            }
                        } else { current_block_99 = 11071260907632769126; }
                        match current_block_99 {
                            13174377073168946860 => { }
                            _ =>
                            // here, aset is set up and accumulator is ready for use
                            {
                                if useint == 0 { // rhs integer path
                                    // non-integral rhs
                                    // x ** y; special-case x=1 here as it will otherwise always
      // reduce to integer 1; decLnOp has a fastpath which detects
      // the case of x=1
                                    decLnOp(dac, lhs, &mut aset,
                                            &mut status); // dac=ln(lhs)
                                    // and drop through for final rounding
                                    if *(*dac).lsu.as_mut_ptr() as libc::c_int
                                           == 0 as libc::c_int &&
                                           (*dac).digits == 1 as libc::c_int
                                           &&
                                           (*dac).bits as libc::c_int &
                                               (0x40 as libc::c_int |
                                                    0x20 as libc::c_int |
                                                    0x10 as libc::c_int) ==
                                               0 as libc::c_int {
                                        // [no error possible, as lhs 0 already handled]
                                        // x==1, 1.0, etc.
                                        // need to return fully-padded 1.0000 etc., but rhsint->1
                                        *(*dac).lsu.as_mut_ptr() =
                                            1 as libc::c_int as
                                                uint16_t; // was 0, make int 1
                                        if rhsint == 0 {
                                            // add padding
                                            let mut shift_0: int32_t =
                                                (*set).digits -
                                                    1 as
                                                        libc::c_int; // make 1.0000...
                                            (*dac).digits =
                                                decShiftToMost((*dac).lsu.as_mut_ptr(),
                                                               1 as
                                                                   libc::c_int,
                                                               shift_0); // dac=dac*rhs
                                            (*dac).exponent = -shift_0;
                                            status |=
                                                (0x20 as libc::c_int |
                                                     0x800 as libc::c_int) as
                                                    libc::c_uint
                                        }
                                    } else {
                                        decMultiplyOp(dac, dac, rhs,
                                                      &mut aset, &mut status);
                                        decExpOp(dac, dac, &mut aset,
                                                 &mut status);
                                        // dac=exp(dac)
                                    }
                                    current_block_99 = 15650704408606443395;
                                } else {
                                    // carry on with integer
                                    decNumberZero(dac); // acc=1
                                    *(*dac).lsu.as_mut_ptr() =
                                        1 as libc::c_int as uint16_t; // ..
                                    // if a negative power the constant 1 is needed, and if not subset
      // invert the lhs now rather than inverting the result later
                                    if (*rhs).bits as libc::c_int &
                                           0x80 as libc::c_int !=
                                           0 as libc::c_int {
                                        // was a **-n [hence digits>0]
                                        let mut inv: *mut decNumber =
                                            invbuff.as_mut_ptr(); // asssume use fixed buffer
                                        decNumberCopy(&mut dnOne,
                                                      dac); // dnOne=1;  [needed now or later]
                                        // divide lhs into 1, putting result in dac [dac=1/dac]
                                        decDivideOp(dac, &mut dnOne, lhs,
                                                    &mut aset,
                                                    0x80 as libc::c_int as
                                                        uint8_t, &mut status);
                                        // now locate or allocate space for the inverted lhs
                                        if needbytes as libc::c_ulong >
                                               ::std::mem::size_of::<[decNumber; 4]>()
                                                   as libc::c_ulong {
                                            allocinv =
                                                malloc(needbytes as
                                                           libc::c_ulong) as
                                                    *mut decNumber;
                                            if allocinv.is_null() {
                                                // use the allocated space
                                                // hopeless -- abandon
                                                status |=
                                                    0x10 as libc::c_int as
                                                        libc::c_uint;
                                                current_block_99 =
                                                    13174377073168946860;
                                            } else {
                                                inv = allocinv;
                                                current_block_99 =
                                                    1739363794695357236;
                                            }
                                        } else {
                                            current_block_99 =
                                                1739363794695357236;
                                        }
                                        match current_block_99 {
                                            13174377073168946860 => { }
                                            _ => {
                                                // [inv now points to big-enough buffer or allocated storage]
                                                decNumberCopy(inv,
                                                              dac); // copy the 1/lhs
                                                decNumberCopy(dac,
                                                              &mut dnOne); // restore acc=1
                                                lhs = inv;
                                                current_block_99 =
                                                    14648249180243006330;
                                            }
                                        }
                                        // .. and go forward with new lhs
                                    } else {
                                        current_block_99 =
                                            14648249180243006330;
                                    }
                                    match current_block_99 {
                                        13174377073168946860 => { }
                                        _ => {
                                            // Raise-to-the-power loop...
                                            seenbit =
                                                0 as libc::c_int as
                                                    uint8_t; // set once a 1-bit is encountered
                                            i =
                                                1 as
                                                    libc::c_int; /*i*/ // 32 bits
                                            loop  {
                                                // for each bit [top bit ignored]
                                                // abandon if had overflow or terminal underflow
                                                if status &
                                                       (0x200 as libc::c_int |
                                                            0x2000 as
                                                                libc::c_int)
                                                           as libc::c_uint !=
                                                       0 {
                                                    // interesting?
                                                    if status &
                                                           0x200 as
                                                               libc::c_int as
                                                               libc::c_uint !=
                                                           0 ||
                                                           *(*dac).lsu.as_mut_ptr()
                                                               as libc::c_int
                                                               ==
                                                               0 as
                                                                   libc::c_int
                                                               &&
                                                               (*dac).digits
                                                                   ==
                                                                   1 as
                                                                       libc::c_int
                                                               &&
                                                               (*dac).bits as
                                                                   libc::c_int
                                                                   &
                                                                   (0x40 as
                                                                        libc::c_int
                                                                        |
                                                                        0x20
                                                                            as
                                                                            libc::c_int
                                                                        |
                                                                        0x10
                                                                            as
                                                                            libc::c_int)
                                                                   ==
                                                                   0 as
                                                                       libc::c_int
                                                       {
                                                        break ;
                                                    }
                                                }
                                                // [the following two lines revealed an optimizer bug in a C++
        // compiler, with symptom: 5**3 -> 25, when n=n+n was used]
                                                n =
                                                    n <<
                                                        1 as
                                                            libc::c_int; // move next bit to testable position
                                                if n < 0 as libc::c_int {
                                                    // top bit is set
                                                    seenbit =
                                                        1 as libc::c_int as
                                                            uint8_t;
                                                    decMultiplyOp(dac, dac,
                                                                  lhs,
                                                                  &mut aset,
                                                                  &mut status); // OK, significant bit seen
                                                    // dac=dac*x
                                                } // that was the last bit
                                                if i == 31 as libc::c_int {
                                                    break
                                                        ; // no need to square 1
                                                }
                                                if !(seenbit == 0) {
                                                    decMultiplyOp(dac, dac,
                                                                  dac,
                                                                  &mut aset,
                                                                  &mut status);
                                                }
                                                i += 1
                                                // dac=dac*dac [square]
                                            }
                                            // complete internal overflow or underflow processing
                                            if status &
                                                   (0x200 as libc::c_int |
                                                        0x2000 as libc::c_int)
                                                       as libc::c_uint != 0 {
                                                (*dac).bits =
                                                    ((*dac).bits as
                                                         libc::c_int &
                                                         !(0x80 as
                                                               libc::c_int) |
                                                         bits as libc::c_int)
                                                        as
                                                        uint8_t; // force correct sign
                                                // round subnormals [to set.digits rather than aset.digits]
        // or set overflow result similarly as required
                                                decFinalize(dac, set,
                                                            &mut residue,
                                                            &mut status); // copy to result (is now OK length)
                                                decNumberCopy(res, dac);
                                                current_block_99 =
                                                    13174377073168946860;
                                            } else {
                                                current_block_99 =
                                                    15650704408606443395;
                                            }
                                        }
                                    }
                                }
                                match current_block_99 {
                                    13174377073168946860 => { }
                                    _ => {
                                        // reduce result to the requested length and copy to result
                                        decCopyFit(res, dac, set,
                                                   &mut residue,
                                                   &mut status); // drop any storage used
                                        decFinalize(res, set, &mut residue,
                                                    &mut status); // ..
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
    if !allocdac.is_null() { free(allocdac as *mut libc::c_void); }
    if !allocinv.is_null() { free(allocinv as *mut libc::c_void); }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberPower
/* ------------------------------------------------------------------ */
/* decNumberQuantize -- force exponent to requested value             */
/*                                                                    */
/*   This computes C = op(A, B), where op adjusts the coefficient     */
/*   of C (by rounding or shifting) such that the exponent (-scale)   */
/*   of C has exponent of B.  The numerical value of C will equal A,  */
/*   except for the effects of any rounding that occurred.            */
/*                                                                    */
/*   res is C, the result.  C may be A or B                           */
/*   lhs is A, the number to adjust                                   */
/*   rhs is B, the number with exponent to match                      */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Unless there is an error or the result is infinite, the exponent   */
/* after the operation is guaranteed to be equal to that of B.        */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberQuantize(mut res: *mut decNumber,
                                           mut lhs: *const decNumber,
                                           mut rhs: *const decNumber,
                                           mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decQuantizeOp(res, lhs, rhs, set, 1 as libc::c_int as uint8_t,
                  &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberQuantize
/* ------------------------------------------------------------------ */
/* decNumberReduce -- remove trailing zeros                           */
/*                                                                    */
/*   This computes C = 0 + A, and normalizes the result               */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
// Previously known as Normalize
#[no_mangle]
pub unsafe extern "C" fn decNumberNormalize(mut res: *mut decNumber,
                                            mut rhs: *const decNumber,
                                            mut set: *mut decContext)
 -> *mut decNumber {
    return decNumberReduce(res, rhs, set);
}
// decNumberNormalize
#[no_mangle]
pub unsafe extern "C" fn decNumberReduce(mut res: *mut decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // as usual
    let mut residue: int32_t = 0 as libc::c_int; // as usual
    let mut dropped: int32_t = 0; // work
    if (*rhs).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int {
        decNaNs(res, rhs, 0 as *const decNumber, set, &mut status);
    } else {
        // reduce result to the requested length and copy to result
        decCopyFit(res, rhs, set, &mut residue, &mut status); // copy & round
        decFinalize(res, set, &mut residue, &mut status); // cleanup/set flags
        decTrim(res, set, 1 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t, &mut dropped);
    }
    // normalize in place
                                                  // [may clamp]
    // protect allocated storage
    // [following code does not require input rounding]
    // Infinities copy through; NaNs need usual treatment
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set); // then report status
    }
    return res;
}
// decNumberReduce
/* ------------------------------------------------------------------ */
/* decNumberRescale -- force exponent to requested value              */
/*                                                                    */
/*   This computes C = op(A, B), where op adjusts the coefficient     */
/*   of C (by rounding or shifting) such that the exponent (-scale)   */
/*   of C has the value B.  The numerical value of C will equal A,    */
/*   except for the effects of any rounding that occurred.            */
/*                                                                    */
/*   res is C, the result.  C may be A or B                           */
/*   lhs is A, the number to adjust                                   */
/*   rhs is B, the requested exponent                                 */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Unless there is an error or the result is infinite, the exponent   */
/* after the operation is guaranteed to be equal to B.                */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberRescale(mut res: *mut decNumber,
                                          mut lhs: *const decNumber,
                                          mut rhs: *const decNumber,
                                          mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decQuantizeOp(res, lhs, rhs, set, 0 as libc::c_int as uint8_t,
                  &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberRescale
/* ------------------------------------------------------------------ */
/* decNumberRemainder -- divide and return remainder                  */
/*                                                                    */
/*   This computes C = A % B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X%X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberRemainder(mut res: *mut decNumber,
                                            mut lhs: *const decNumber,
                                            mut rhs: *const decNumber,
                                            mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decDivideOp(res, lhs, rhs, set, 0x40 as libc::c_int as uint8_t,
                &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberRemainder
/* ------------------------------------------------------------------ */
/* decNumberRemainderNear -- divide and return remainder from nearest */
/*                                                                    */
/*   This computes C = A % B, where % is the IEEE remainder operator  */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X%X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberRemainderNear(mut res: *mut decNumber,
                                                mut lhs: *const decNumber,
                                                mut rhs: *const decNumber,
                                                mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decDivideOp(res, lhs, rhs, set, 0x10 as libc::c_int as uint8_t,
                &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberRemainderNear
/* ------------------------------------------------------------------ */
/* decNumberRotate -- rotate the coefficient of a Number left/right   */
/*                                                                    */
/*   This computes C = A rot B  (in base ten and rotating set->digits */
/*   digits).                                                         */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=XrotX)       */
/*   lhs is A                                                         */
/*   rhs is B, the number of digits to rotate (-ve to right)          */
/*   set is the context                                               */
/*                                                                    */
/* The digits of the coefficient of A are rotated to the left (if B   */
/* is positive) or to the right (if B is negative) without adjusting  */
/* the exponent or the sign of A.  If lhs->digits is less than        */
/* set->digits the coefficient is padded with zeros on the left       */
/* before the rotate.  Any leading zeros in the result are removed    */
/* as usual.                                                          */
/*                                                                    */
/* B must be an integer (q=0) and in the range -set->digits through   */
/* +set->digits.                                                      */
/* C must have space for set->digits digits.                          */
/* NaNs are propagated as usual.  Infinities are unaffected (but      */
/* B must be valid).  No status is set unless B is invalid or an      */
/* operand is an sNaN.                                                */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberRotate(mut res: *mut decNumber,
                                         mut lhs: *const decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    let mut rotate: int32_t = 0; // rhs as an Int
    // NaNs propagate as normal
    if (*lhs).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
       {
        decNaNs(res, lhs, rhs, set, &mut status); // numerics
    } else if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                  0 as libc::c_int || (*rhs).exponent != 0 as libc::c_int {
        status = 0x80 as libc::c_int as uint32_t
    } else {
        // rhs must be an integer
        // both numeric, rhs is an integer
        rotate = decGetInt(rhs);
        if rotate == 0x80000000 as libc::c_uint as int32_t ||
               rotate == 0x80000003 as libc::c_uint as int32_t ||
               rotate == 0x80000002 as libc::c_uint as int32_t ||
               abs(rotate) > (*set).digits { // [cannot fail]
            // rhs OK
            // .. or out of range
            status = 0x80 as libc::c_int as uint32_t
        } else {
            // rhs is OK
            decNumberCopy(res, lhs);
            // rotate needed
            if rotate < 0 as libc::c_int { rotate = (*set).digits + rotate }
            if rotate != 0 as libc::c_int && rotate != (*set).digits &&
                   !((*res).bits as libc::c_int & 0x40 as libc::c_int !=
                         0 as libc::c_int) {
                // convert -ve rotate to equivalent positive rotation
                // lhs was infinite
                // left-rotate to do; 0 < rotate < set->digits
                let mut units: uint32_t = 0; // work
                let mut shift: uint32_t = 0; // digits in result msu
                let mut msudigits: uint32_t = 0; // current msu
                let mut msu: *mut uint16_t =
                    (*res).lsu.as_mut_ptr().offset((if (*res).digits <=
                                                           49 as libc::c_int {
                                                        d2utable[(*res).digits
                                                                     as usize]
                                                            as libc::c_int
                                                    } else {
                                                        ((*res).digits +
                                                             3 as libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                            / 3 as libc::c_int
                                                    }) as
                                                       isize).offset(-(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)); // rotation msu
                let mut msumax: *mut uint16_t =
                    (*res).lsu.as_mut_ptr().offset((if (*set).digits <=
                                                           49 as libc::c_int {
                                                        d2utable[(*set).digits
                                                                     as usize]
                                                            as libc::c_int
                                                    } else {
                                                        ((*set).digits +
                                                             3 as libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                            / 3 as libc::c_int
                                                    }) as
                                                       isize).offset(-(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)); // ensure high units=0
                msu = msu.offset(1); // now full-length
                while msu <= msumax {
                    *msu =
                        0 as libc::c_int as uint16_t; // actual digits in msu
                    msu = msu.offset(1)
                }
                (*res).digits = (*set).digits;
                msudigits =
                    ((*res).digits -
                         ((if (*res).digits <= 49 as libc::c_int {
                               d2utable[(*res).digits as usize] as libc::c_int
                           } else {
                               ((*res).digits + 3 as libc::c_int -
                                    1 as libc::c_int) / 3 as libc::c_int
                           }) - 1 as libc::c_int) * 3 as libc::c_int) as
                        uint32_t;
                // rotation here is done in-place, in three steps
        // 1. shift all to least up to one unit to unit-align final
        //    lsd [any digits shifted out are rotated to the left,
        //    abutted to the original msd (which may require split)]
        //
        //    [if there are no whole units left to rotate, the
        //    rotation is now complete]
        //
        // 2. shift to least, from below the split point only, so that
        //    the final msd is in the right place in its Unit [any
        //    digits shifted out will fit exactly in the current msu,
        //    left aligned, no split required]
        //
        // 3. rotate all the units by reversing left part, right
        //    part, and then whole
        //
        // example: rotate right 8 digits (2 units + 2), DECDPUN=3.
        //
        //   start: 00a bcd efg hij klm npq
        //
        //      1a  000 0ab cde fgh|ijk lmn [pq saved]
        //      1b  00p qab cde fgh|ijk lmn
        //
        //      2a  00p qab cde fgh|00i jkl [mn saved]
        //      2b  mnp qab cde fgh|00i jkl
        //
        //      3a  fgh cde qab mnp|00i jkl
        //      3b  fgh cde qab mnp|jkl 00i
        //      3c  00i jkl mnp qab cde fgh
                // Step 1: amount to shift is the partial right-rotate count
                rotate = (*set).digits - rotate; // make it right-rotate
                units =
                    (rotate / 3 as libc::c_int) as
                        uint32_t; // whole units to rotate
                shift =
                    (rotate % 3 as libc::c_int) as
                        uint32_t; // left-over digits count
                if shift > 0 as libc::c_int as libc::c_uint
                   { // digits shift needed
                    // not an exact number of units
                    let mut save: uint32_t =
                        (*(*res).lsu.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize) as
                             libc::c_uint).wrapping_rem(DECPOWERS[shift as
                                                                      usize]); // save low digit(s)
                    decShiftToLeast((*res).lsu.as_mut_ptr(),
                                    if (*res).digits <= 49 as libc::c_int {
                                        d2utable[(*res).digits as usize] as
                                            libc::c_int
                                    } else {
                                        ((*res).digits + 3 as libc::c_int -
                                             1 as libc::c_int) /
                                            3 as libc::c_int
                                    }, shift as int32_t);
                    if shift > msudigits {
                        // msumax-1 needs >0 digits
                        let mut rem: uint32_t =
                            save.wrapping_rem(DECPOWERS[shift.wrapping_sub(msudigits)
                                                            as
                                                            usize]); // split save
                        *msumax =
                            save.wrapping_div(DECPOWERS[shift.wrapping_sub(msudigits)
                                                            as usize]) as
                                uint16_t; // and insert
                        *msumax.offset(-(1 as libc::c_int as isize)) =
                            (*msumax.offset(-(1 as libc::c_int as isize)) as
                                 libc::c_int +
                                 rem.wrapping_mul(DECPOWERS[(3 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_sub(shift.wrapping_sub(msudigits))
                                                                as usize]) as
                                     uint16_t as libc::c_int) as uint16_t
                    } else {
                        // all fits in msumax
                        *msumax =
                            (*msumax as libc::c_int +
                                 save.wrapping_mul(DECPOWERS[msudigits.wrapping_sub(shift)
                                                                 as usize]) as
                                     uint16_t as libc::c_int) as uint16_t
                        // [maybe *1]
                    }
                }
                // If whole units to rotate...
                if units > 0 as libc::c_int as libc::c_uint
                   { // whole units to rotate
                    // some to do
                    // Step 2: the units to touch are the whole ones in rotate,
          //   if any, and the shift is DECDPUN-msudigits (which may be
          //   0, again)
                    shift =
                        (3 as libc::c_int as
                             libc::c_uint).wrapping_sub(msudigits);
                    // whole
                    if shift > 0 as libc::c_int as libc::c_uint
                       { // partial shift needed
                        // not an exact number of units
                        let mut save_0: uint32_t =
                            (*(*res).lsu.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize) as
                                 libc::c_uint).wrapping_rem(DECPOWERS[shift as
                                                                          usize]); // save low digit(s)
                        decShiftToLeast((*res).lsu.as_mut_ptr(),
                                        units as int32_t, shift as int32_t);
                        *msumax =
                            (*msumax as libc::c_int +
                                 save_0.wrapping_mul(DECPOWERS[msudigits as
                                                                   usize]) as
                                     uint16_t as libc::c_int) as uint16_t
                    }
                    decReverse((*res).lsu.as_mut_ptr().offset(units as isize),
                               msumax);
                    decReverse((*res).lsu.as_mut_ptr(),
                               (*res).lsu.as_mut_ptr().offset(units as
                                                                  isize).offset(-(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)));
                    decReverse((*res).lsu.as_mut_ptr(), msumax);
                }
                // Step 3: rotate the units array using triple reverse
          // (reversing is easy and fast)
                // left part
                // right part
                // the rotation may have left an undetermined number of zeros
        // on the left, so true length needs to be calculated
                (*res).digits =
                    decGetDigits((*res).lsu.as_mut_ptr(),
                                 (msumax.wrapping_offset_from((*res).lsu.as_mut_ptr())
                                      as libc::c_long +
                                      1 as libc::c_int as libc::c_long) as
                                     int32_t)
            }
        }
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberRotate
/* ------------------------------------------------------------------ */
/* decNumberSameQuantum -- test for equal exponents                   */
/*                                                                    */
/*   res is the result number, which will contain either 0 or 1       */
/*   lhs is a number to test                                          */
/*   rhs is the second (usually a pattern)                            */
/*                                                                    */
/* No errors are possible and no context is needed.                   */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberSameQuantum(mut res: *mut decNumber,
                                              mut lhs: *const decNumber,
                                              mut rhs: *const decNumber)
 -> *mut decNumber {
    let mut ret: uint16_t = 0 as libc::c_int as uint16_t; // return value
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        if (*lhs).bits as libc::c_int &
               (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
               &&
               (*rhs).bits as libc::c_int &
                   (0x20 as libc::c_int | 0x10 as libc::c_int) !=
                   0 as libc::c_int {
            ret = 1 as libc::c_int as uint16_t
        } else if (*lhs).bits as libc::c_int & 0x40 as libc::c_int !=
                      0 as libc::c_int &&
                      (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                          0 as libc::c_int {
            ret = 1 as libc::c_int as uint16_t
        }
        // [anything else with a special gives 0]
    } else if (*lhs).exponent == (*rhs).exponent {
        ret = 1 as libc::c_int as uint16_t
    } // OK to overwrite an operand now
    decNumberZero(res);
    *(*res).lsu.as_mut_ptr() = ret;
    return res;
}
// decNumberSameQuantum
/* ------------------------------------------------------------------ */
/* decNumberScaleB -- multiply by a power of 10                       */
/*                                                                    */
/* This computes C = A x 10**B where B is an integer (q=0) with       */
/* maximum magnitude 2*(emax+digits)                                  */
/*                                                                    */
/*   res is C, the result.  C may be A or B                           */
/*   lhs is A, the number to adjust                                   */
/*   rhs is B, the requested power of ten to use                      */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* The result may underflow or overflow.                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberScaleB(mut res: *mut decNumber,
                                         mut lhs: *const decNumber,
                                         mut rhs: *const decNumber,
                                         mut set: *mut decContext)
 -> *mut decNumber {
    let mut reqexp: int32_t = 0; // requested exponent change [B]
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    let mut residue: int32_t = 0; // work
    // Handle special values except lhs infinite
    if (*lhs).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
       {
        decNaNs(res, lhs, rhs, set, &mut status); // rhs finite
    } else if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                  0 as libc::c_int || (*rhs).exponent != 0 as libc::c_int {
        status = 0x80 as libc::c_int as uint32_t
    } else {
        // rhs must be an integer
        // lhs is a number; rhs is a finite with q==0
        reqexp = decGetInt(rhs); // [cannot fail]
        // rhs OK
        if reqexp == 0x80000000 as libc::c_uint as int32_t ||
               reqexp == 0x80000003 as libc::c_uint as int32_t ||
               reqexp == 0x80000002 as libc::c_uint as int32_t ||
               (abs(reqexp) + 1 as libc::c_int) / 2 as libc::c_int >
                   (*set).digits + (*set).emax {
            // maximum range is larger than getInt can handle, so this is
    // more restrictive than the specification
            // .. or out of range
            status = 0x80 as libc::c_int as uint32_t
        } else {
            // rhs is OK
            decNumberCopy(res, lhs);
            if !((*res).bits as libc::c_int & 0x40 as libc::c_int !=
                     0 as libc::c_int) { // all done if infinite lhs
                // finite LHS
                // prepare to scale
                let mut exp: int32_t =
                    (*res).exponent; // save for overflow test
                (*res).exponent += reqexp; // adjust the exponent
                if exp ^ reqexp >= 0 as libc::c_int &&
                       exp ^ (*res).exponent < 0 as libc::c_int {
                    // .. but result had different
                    // the calculation overflowed, so force right treatment
                    if exp < 0 as libc::c_int {
                        (*res).exponent =
                            -(999999999 as libc::c_int) -
                                999999999 as libc::c_int
                    } else {
                        (*res).exponent =
                            999999999 as libc::c_int + 1 as libc::c_int
                    }
                }
                residue = 0 as libc::c_int;
                decFinalize(res, set, &mut residue, &mut status);
            }
        }
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberScaleB
/* ------------------------------------------------------------------ */
/* decNumberShift -- shift the coefficient of a Number left or right  */
/*                                                                    */
/*   This computes C = A << B or C = A >> -B  (in base ten).          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X<<X)        */
/*   lhs is A                                                         */
/*   rhs is B, the number of digits to shift (-ve to right)           */
/*   set is the context                                               */
/*                                                                    */
/* The digits of the coefficient of A are shifted to the left (if B   */
/* is positive) or to the right (if B is negative) without adjusting  */
/* the exponent or the sign of A.                                     */
/*                                                                    */
/* B must be an integer (q=0) and in the range -set->digits through   */
/* +set->digits.                                                      */
/* C must have space for set->digits digits.                          */
/* NaNs are propagated as usual.  Infinities are unaffected (but      */
/* B must be valid).  No status is set unless B is invalid or an      */
/* operand is an sNaN.                                                */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberShift(mut res: *mut decNumber,
                                        mut lhs: *const decNumber,
                                        mut rhs: *const decNumber,
                                        mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    let mut shift: int32_t = 0; // rhs as an Int
    // NaNs propagate as normal
    if (*lhs).bits as libc::c_int &
           (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x20 as libc::c_int | 0x10 as libc::c_int) != 0 as libc::c_int
       {
        decNaNs(res, lhs, rhs, set, &mut status); // numerics
    } else if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                  0 as libc::c_int || (*rhs).exponent != 0 as libc::c_int {
        status = 0x80 as libc::c_int as uint32_t
    } else {
        // rhs must be an integer
        // both numeric, rhs is an integer
        shift = decGetInt(rhs);
        if shift == 0x80000000 as libc::c_uint as int32_t ||
               shift == 0x80000003 as libc::c_uint as int32_t ||
               shift == 0x80000002 as libc::c_uint as int32_t ||
               abs(shift) > (*set).digits { // [cannot fail]
            // rhs OK
            // .. or out of range
            status = 0x80 as libc::c_int as uint32_t
        } else {
            // rhs is OK
            decNumberCopy(res, lhs);
            if shift != 0 as libc::c_int &&
                   !((*res).bits as libc::c_int & 0x40 as libc::c_int !=
                         0 as libc::c_int) {
                // non-0 non-Inf shift
                // something to do
                if shift > 0 as libc::c_int {
                    // to left
                    if shift == (*set).digits {
                        // removing all
                        *(*res).lsu.as_mut_ptr() =
                            0 as libc::c_int as uint16_t;
                        (*res).digits = 1 as libc::c_int // so place 0
                        // ..
                    } else {
                        //
                        // first remove leading digits if necessary
                        if (*res).digits + shift > (*set).digits {
                            decDecap(res,
                                     (*res).digits + shift - (*set).digits);
                            // that updated res->digits; may have gone to 1 (for a
              // single digit or for zero
                        }
                        if (*res).digits > 1 as libc::c_int ||
                               *(*res).lsu.as_mut_ptr() as libc::c_int != 0 {
                            // if non-zero..
                            (*res).digits =
                                decShiftToMost((*res).lsu.as_mut_ptr(),
                                               (*res).digits, shift)
                        }
                    }
                    // partial left
                } else if -shift >= (*res).digits {
                    // to right
                    // discarding all
                    *(*res).lsu.as_mut_ptr() = 0 as libc::c_int as uint16_t;
                    (*res).digits = 1 as libc::c_int // so place 0
                    // ..
                } else {
                    decShiftToLeast((*res).lsu.as_mut_ptr(),
                                    if (*res).digits <= 49 as libc::c_int {
                                        d2utable[(*res).digits as usize] as
                                            libc::c_int
                                    } else {
                                        ((*res).digits + 3 as libc::c_int -
                                             1 as libc::c_int) /
                                            3 as libc::c_int
                                    }, -shift);
                    (*res).digits -= -shift
                }
                // to right
            }
        }
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberShift
/* ------------------------------------------------------------------ */
/* decNumberSquareRoot -- square root operator                        */
/*                                                                    */
/*   This computes C = squareroot(A)                                  */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context; note that rounding mode has no effect        */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
/* This uses the following varying-precision algorithm in:            */
/*                                                                    */
/*   Properly Rounded Variable Precision Square Root, T. E. Hull and  */
/*   A. Abrham, ACM Transactions on Mathematical Software, Vol 11 #3, */
/*   pp229-237, ACM, September 1985.                                  */
/*                                                                    */
/* The square-root is calculated using Newton's method, after which   */
/* a check is made to ensure the result is correctly rounded.         */
/*                                                                    */
/* % [Reformatted original Numerical Turing source code follows.]     */
/* function sqrt(x : real) : real                                     */
/* % sqrt(x) returns the properly rounded approximation to the square */
/* % root of x, in the precision of the calling environment, or it    */
/* % fails if x < 0.                                                  */
/* % t e hull and a abrham, august, 1984                              */
/* if x <= 0 then                                                     */
/*   if x < 0 then                                                    */
/*     assert false                                                   */
/*   else                                                             */
/*     result 0                                                       */
/*   end if                                                           */
/* end if                                                             */
/* var f := setexp(x, 0)  % fraction part of x   [0.1 <= x < 1]       */
/* var e := getexp(x)     % exponent part of x                        */
/* var approx : real                                                  */
/* if e mod 2 = 0  then                                               */
/*   approx := .259 + .819 * f   % approx to root of f                */
/* else                                                               */
/*   f := f/l0                   % adjustments                        */
/*   e := e + 1                  %   for odd                          */
/*   approx := .0819 + 2.59 * f  %   exponent                         */
/* end if                                                             */
/*                                                                    */
/* var p:= 3                                                          */
/* const maxp := currentprecision + 2                                 */
/* loop                                                               */
/*   p := min(2*p - 2, maxp)     % p = 4,6,10, . . . , maxp           */
/*   precision p                                                      */
/*   approx := .5 * (approx + f/approx)                               */
/*   exit when p = maxp                                               */
/* end loop                                                           */
/*                                                                    */
/* % approx is now within 1 ulp of the properly rounded square root   */
/* % of f; to ensure proper rounding, compare squares of (approx -    */
/* % l/2 ulp) and (approx + l/2 ulp) with f.                          */
/* p := currentprecision                                              */
/* begin                                                              */
/*   precision p + 2                                                  */
/*   const approxsubhalf := approx - setexp(.5, -p)                   */
/*   if mulru(approxsubhalf, approxsubhalf) > f then                  */
/*     approx := approx - setexp(.l, -p + 1)                          */
/*   else                                                             */
/*     const approxaddhalf := approx + setexp(.5, -p)                 */
/*     if mulrd(approxaddhalf, approxaddhalf) < f then                */
/*       approx := approx + setexp(.l, -p + 1)                        */
/*     end if                                                         */
/*   end if                                                           */
/* end                                                                */
/* result setexp(approx, e div 2)  % fix exponent                     */
/* end sqrt                                                           */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberSquareRoot(mut res: *mut decNumber,
                                             mut rhs: *const decNumber,
                                             mut set: *mut decContext)
 -> *mut decNumber {
    let mut workset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // work contexts
    let mut approxset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // used for constant zero
    let mut dzero: decNumber =
        decNumber{digits: 0,
                  exponent: 0,
                  bits: 0,
                  lsu: [0; 1],}; // largest working precision
    let mut maxp: int32_t = 0; // working precision
    let mut workp: int32_t = 0; // rounding residue
    let mut residue: int32_t = 0 as libc::c_int; // status accumulators
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // ..
    let mut ignore: uint32_t =
        0 as libc::c_int as uint32_t; // working exponent
    let mut rstatus: uint32_t = 0; // ideal (preferred) exponent
    let mut exp: int32_t = 0; // work
    let mut ideal: int32_t = 0; // ..
    let mut needbytes: int32_t = 0;
    let mut dropped: int32_t = 0;
    // buffer for f [needs +1 in case DECBUFFER 0]
    let mut buff: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; 3];
    // buffer for a [needs +2 to match likely maxp]
    let mut bufa: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],}; 3];
    // buffer for temporary, b [must be same size as a]
    let mut bufb: [decNumber; 3] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            3]; // -> allocated buff, iff allocated
    let mut allocbuff: *mut decNumber =
        0 as *mut decNumber; // -> allocated bufa, iff allocated
    let mut allocbufa: *mut decNumber =
        0 as *mut decNumber; // -> allocated bufb, iff allocated
    let mut allocbufb: *mut decNumber =
        0 as *mut decNumber; // reduced fraction
    let mut f: *mut decNumber = buff.as_mut_ptr(); // approximation to result
    let mut a: *mut decNumber = bufa.as_mut_ptr(); // intermediate result
    let mut b: *mut decNumber = bufb.as_mut_ptr();
    // buffer for temporary variable, up to 3 digits
    let mut buft: [decNumber; 1] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            1]; // up-to-3-digit constant or work
    let mut t: *mut decNumber = buft.as_mut_ptr(); // a NaN
    let mut current_block_137: u64;
    if (*rhs).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
               0 as libc::c_int {
            // an infinity
            if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
                status |= 0x80 as libc::c_int as libc::c_uint
            } else { decNumberCopy(res, rhs); }
            // +Infinity
        } else { decNaNs(res, rhs, 0 as *const decNumber, set, &mut status); }
    } else {
        // calculate the ideal (preferred) exponent [floor(exp/2)]
    // [It would be nicer to write: ideal=rhs->exponent>>1, but this
    // generates a compiler warning.  Generated code is the same.]
        ideal =
            ((*rhs).exponent & !(1 as libc::c_int)) /
                2 as libc::c_int; // target
        // handle zeros
        if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
               (*rhs).digits == 1 as libc::c_int &&
               (*rhs).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) == 0 as libc::c_int {
            decNumberCopy(res, rhs); // could be 0 or -0
            (*res).exponent = ideal; // use the ideal [safe]
            // use decFinish to clamp any out-of-range exponent, etc.
            decFinalize(res, set, &mut residue, &mut status);
        } else if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                      0 as libc::c_int {
            status |= 0x80 as libc::c_int as libc::c_uint
        } else {
            // any other -x is an oops
            // space is needed for three working variables
    //   f -- the same precision as the RHS, reduced to 0.01->0.99...
    //   a -- Hull's approximation -- precision, when assigned, is
    //        currentprecision+1 or the input argument precision,
    //        whichever is larger (+2 for use as temporary)
    //   b -- intermediate temporary result (same size as a)
    // if any is too long for local storage, then allocate
            workp =
                if ((*set).digits + 1 as libc::c_int) < (*rhs).digits {
                    (*rhs).digits
                } else {
                    ((*set).digits) + 1 as libc::c_int
                }; // actual rounding precision
            workp =
                if workp < 7 as libc::c_int {
                    7 as libc::c_int
                } else { workp }; // at least 7 for low cases
            maxp = workp + 2 as libc::c_int; // largest working precision
            needbytes =
                (::std::mem::size_of::<decNumber>() as
                     libc::c_ulong).wrapping_add((((if (*rhs).digits <=
                                                           49 as libc::c_int {
                                                        d2utable[(*rhs).digits
                                                                     as usize]
                                                            as libc::c_int
                                                    } else {
                                                        ((*rhs).digits +
                                                             3 as libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                            / 3 as libc::c_int
                                                    }) - 1 as libc::c_int) as
                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                      as
                                                                                      libc::c_ulong))
                    as int32_t;
            if needbytes >
                   ::std::mem::size_of::<[decNumber; 3]>() as libc::c_ulong as
                       int32_t {
                allocbuff =
                    malloc(needbytes as libc::c_ulong) as *mut decNumber;
                if allocbuff.is_null() {
                    // use the allocated space
                    // hopeless -- abandon
                    status |= 0x10 as libc::c_int as libc::c_uint;
                    current_block_137 = 17727836384662615028;
                } else {
                    f = allocbuff;
                    current_block_137 = 14832935472441733737;
                }
            } else { current_block_137 = 14832935472441733737; }
            match current_block_137 {
                17727836384662615028 => { }
                _ => {
                    // a and b both need to be able to hold a maxp-length number
                    needbytes =
                        (::std::mem::size_of::<decNumber>() as
                             libc::c_ulong).wrapping_add((((if maxp <=
                                                                   49 as
                                                                       libc::c_int
                                                               {
                                                                d2utable[maxp
                                                                             as
                                                                             usize]
                                                                    as
                                                                    libc::c_int
                                                            } else {
                                                                (maxp +
                                                                     3 as
                                                                         libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    /
                                                                    3 as
                                                                        libc::c_int
                                                            }) -
                                                               1 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                              as
                                                                                              libc::c_ulong))
                            as int32_t;
                    if needbytes >
                           ::std::mem::size_of::<[decNumber; 3]>() as
                               libc::c_ulong as int32_t {
                        // [same applies to b]
                        allocbufa =
                            malloc(needbytes as libc::c_ulong) as
                                *mut decNumber;
                        allocbufb =
                            malloc(needbytes as libc::c_ulong) as
                                *mut decNumber;
                        if allocbufa.is_null() || allocbufb.is_null() {
                            // ..
                            // hopeless
                            status |=
                                0x10 as libc::c_int as
                                    libc::c_uint; // use the allocated spaces
                            current_block_137 = 17727836384662615028;
                        } else {
                            a = allocbufa;
                            b = allocbufb;
                            current_block_137 = 12381812505308290051;
                        }
                    } else { current_block_137 = 12381812505308290051; }
                    match current_block_137 {
                        17727836384662615028 => { }
                        _ => {
                            // copy rhs -> f, save exponent, and reduce so 0.1 <= f < 1
                            decNumberCopy(f, rhs); // adjusted to Hull rules
                            exp = (*f).exponent + (*f).digits; // to range
                            (*f).exponent = -(*f).digits;
                            // set up working context
                            decContextDefault(&mut workset,
                                              64 as libc::c_int);
                            workset.emax = 999999999 as libc::c_int;
                            workset.emin = -(999999999 as libc::c_int);
                            // [Until further notice, no error is possible and status bits
    // (Rounded, etc.) should be ignored, not accumulated.]
                            // Calculate initial approximation, and allow for odd exponent
                            workset.digits =
                                workp; // p for initial calculation
                            (*t).bits = 0 as libc::c_int as uint8_t;
                            (*t).digits = 3 as libc::c_int;
                            (*a).bits = 0 as libc::c_int as uint8_t;
                            (*a).digits = 3 as libc::c_int;
                            if exp & 1 as libc::c_int == 0 as libc::c_int {
                                // even exponent
                                // Set t=0.259, a=0.819
                                (*t).exponent = -(3 as libc::c_int);
                                (*a).exponent = -(3 as libc::c_int);
                                *(*t).lsu.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) =
                                    259 as libc::c_int as uint16_t;
                                *(*a).lsu.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) =
                                    819 as libc::c_int as uint16_t
                            } else {
                                // odd exponent
                                // Set t=0.0819, a=2.59
                                (*f).exponent -= 1; // f=f/10
                                exp += 1; // e=e+1
                                (*t).exponent = -(4 as libc::c_int); // a=a*f
                                (*a).exponent = -(2 as libc::c_int); // ..+t
                                *(*t).lsu.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) =
                                    819 as libc::c_int as uint16_t;
                                *(*a).lsu.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) =
                                    259 as libc::c_int as uint16_t
                            }
                            decMultiplyOp(a, a, f, &mut workset, &mut ignore);
                            decAddOp(a, a, t, &mut workset,
                                     0 as libc::c_int as uint8_t,
                                     &mut ignore);
                            // [a is now the initial approximation for sqrt(f), calculated with
    // currentprecision, which is also a's precision.]
                            // the main calculation loop
                            decNumberZero(&mut dzero); // make 0
                            decNumberZero(t); // set t = 0.5
                            *(*t).lsu.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) =
                                5 as libc::c_int as uint16_t; // ..
                            (*t).exponent = -(1 as libc::c_int); // ..
                            workset.digits = 3 as libc::c_int; // initial p
                            // a=b*0.5
                            while workset.digits < maxp {
                                workset.digits =
                                    if workset.digits * 2 as libc::c_int -
                                           2 as libc::c_int > maxp {
                                        maxp
                                    } else {
                                        (workset.digits * 2 as libc::c_int) -
                                            2 as libc::c_int
                                    };
                                decDivideOp(b, f, a, &mut workset,
                                            0x80 as libc::c_int as uint8_t,
                                            &mut ignore);
                                decAddOp(b, b, a, &mut workset,
                                         0 as libc::c_int as uint8_t,
                                         &mut ignore);
                                decMultiplyOp(a, b, t, &mut workset,
                                              &mut ignore); // loop
                            }
                            // set p to min(2*p - 2, maxp)  [hence 3; or: 4, 6, 10, ... , maxp]
                            // a = 0.5 * (a + f/a)
      // [calculated at p then rounded to currentprecision]
                            // b=f/a
                            // b=b+a
                            // Here, 0.1 <= a < 1 [Hull], and a has maxp digits
    // now reduce to length, etc.; this needs to be done with a
    // having the correct exponent so as to handle subnormals
    // correctly
                            approxset = *set; // get emin, emax, etc.
                            approxset.round =
                                DEC_ROUND_HALF_EVEN; // set correct exponent
                            (*a).exponent +=
                                exp / 2 as libc::c_int; // clear status
                            rstatus =
                                0 as libc::c_int as
                                    uint32_t; // .. and accumulator
                            residue = 0 as libc::c_int; // reduce (if needed)
                            decCopyFit(a, a, &mut approxset, &mut residue,
                                       &mut rstatus); // clean and finalize
                            decFinalize(a, &mut approxset, &mut residue,
                                        &mut rstatus); // use the status as-is
                            if rstatus & 0x200 as libc::c_int as libc::c_uint
                                   != 0 {
                                status = rstatus; // copy to result
                                decNumberCopy(res, a);
                            } else {
                                // Preserve status except Inexact/Rounded
                                status |=
                                    rstatus &
                                        !(0x800 as libc::c_int |
                                              0x20 as libc::c_int) as
                                            libc::c_uint;
                                // Carry out the Hull correction
                                (*a).exponent -=
                                    exp / 2 as libc::c_int; // back to 0.1->1
                                // a is now at final precision and within 1 ulp of the properly
    // rounded square root of f; to ensure proper rounding, compare
    // squares of (a - l/2 ulp) and (a + l/2 ulp) with f.
    // Here workset.digits=maxp and t=0.5, and a->digits determines
    // the ulp
                                workset.digits -= 1; // maxp-1 is OK now
                                (*t).exponent =
                                    -(*a).digits -
                                        1 as libc::c_int; // make 0.5 ulp
                                decAddOp(b, a, t, &mut workset,
                                         0x80 as libc::c_int as uint8_t,
                                         &mut ignore); // b = a - 0.5 ulp
                                workset.round =
                                    DEC_ROUND_UP; // b = mulru(b, b)
                                decMultiplyOp(b, b, b, &mut workset,
                                              &mut ignore); // b ? f, reversed
                                decCompareOp(b, f, b, &mut workset,
                                             0x1 as libc::c_int as uint8_t,
                                             &mut ignore);
                                if (*b).bits as libc::c_int &
                                       0x80 as libc::c_int != 0 as libc::c_int
                                   {
                                    // f < b [i.e., b > f]
                                    // this is the more common adjustment, though both are rare
                                    (*t).exponent += 1; // make 1.0 ulp
                                    *(*t).lsu.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize) =
                                        1 as libc::c_int as uint16_t; // ..
                                    decAddOp(a, a, t, &mut workset,
                                             0x80 as libc::c_int as uint8_t,
                                             &mut ignore); // a = a - 1 ulp
                                    // assign to approx [round to length]
                                    approxset.emin -=
                                        exp /
                                            2 as
                                                libc::c_int; // adjust to match a
                                    approxset.emax -=
                                        exp /
                                            2 as
                                                libc::c_int; // b = a + 0.5 ulp
                                    decAddOp(a, &mut dzero, a, &mut approxset,
                                             0 as libc::c_int as uint8_t,
                                             &mut ignore); // b = mulrd(b, b)
                                } else {
                                    decAddOp(b, a, t, &mut workset,
                                             0 as libc::c_int as uint8_t,
                                             &mut ignore); // b ? f
                                    workset.round = DEC_ROUND_DOWN;
                                    decMultiplyOp(b, b, b, &mut workset,
                                                  &mut ignore);
                                    decCompareOp(b, b, f, &mut workset,
                                                 0x1 as libc::c_int as
                                                     uint8_t, &mut ignore);
                                    if (*b).bits as libc::c_int &
                                           0x80 as libc::c_int !=
                                           0 as libc::c_int {
                                        // b < f
                                        (*t).exponent += 1; // make 1.0 ulp
                                        *(*t).lsu.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                            =
                                            1 as libc::c_int as
                                                uint16_t; // ..
                                        decAddOp(a, a, t, &mut workset,
                                                 0 as libc::c_int as uint8_t,
                                                 &mut ignore); // a = a + 1 ulp
                                        // assign to approx [round to length]
                                        approxset.emin -=
                                            exp /
                                                2 as
                                                    libc::c_int; // adjust to match a
                                        approxset.emax -=
                                            exp / 2 as libc::c_int;
                                        decAddOp(a, &mut dzero, a,
                                                 &mut approxset,
                                                 0 as libc::c_int as uint8_t,
                                                 &mut ignore);
                                    }
                                }
                                // [no errors are possible in the above, and rounding/inexact during
    // estimation are irrelevant, so status was not accumulated]
                                // Here, 0.1 <= a < 1  (still), so adjust back
                                (*a).exponent +=
                                    exp /
                                        2 as
                                            libc::c_int; // set correct exponent
                                // count droppable zeros [after any subnormal rounding] by
    // trimming a copy
                                decNumberCopy(b, a); // [drops trailing zeros]
                                decTrim(b, set, 1 as libc::c_int as uint8_t,
                                        1 as libc::c_int as uint8_t,
                                        &mut dropped);
                                // Set Inexact and Rounded.  The answer can only be exact if
    // it is short enough so that squaring it could fit in workp
    // digits, so this is the only (relatively rare) condition that
    // a careful check is needed
                                if (*b).digits * 2 as libc::c_int -
                                       1 as libc::c_int > workp {
                                    // cannot fit
                                    status |=
                                        (0x20 as libc::c_int |
                                             0x800 as libc::c_int) as
                                            libc::c_uint
                                } else {
                                    // could be exact/unrounded
                                    let mut mstatus: uint32_t =
                                        0 as libc::c_int as
                                            uint32_t; // local status
                                    decMultiplyOp(b, b, b, &mut workset,
                                                  &mut mstatus); // try the multiply
                                    if mstatus &
                                           0x200 as libc::c_int as
                                               libc::c_uint != 0 {
                                        // result just won't fit
                                        status |=
                                            (0x20 as libc::c_int |
                                                 0x800 as libc::c_int) as
                                                libc::c_uint
                                    } else {
                                        // plausible
                                        decCompareOp(t, b, rhs, &mut workset,
                                                     0x1 as libc::c_int as
                                                         uint8_t,
                                                     &mut mstatus); // b ? rhs
                                        if !(*(*t).lsu.as_mut_ptr() as
                                                 libc::c_int ==
                                                 0 as libc::c_int &&
                                                 (*t).digits ==
                                                     1 as libc::c_int &&
                                                 (*t).bits as libc::c_int &
                                                     (0x40 as libc::c_int |
                                                          0x20 as libc::c_int
                                                          |
                                                          0x10 as libc::c_int)
                                                     == 0 as libc::c_int) {
                                            status |=
                                                (0x20 as libc::c_int |
                                                     0x800 as libc::c_int) as
                                                    libc::c_uint
                                        } else { // not equal
                                            // is Exact
                                            // here, dropped is the count of trailing zeros in 'a'
          // use closest exponent to ideal...
                                            let mut todrop: int32_t =
                                                ideal -
                                                    (*a).exponent; // most that can be dropped
                                            if todrop < 0 as libc::c_int {
                                                status |=
                                                    0x800 as libc::c_int as
                                                        libc::c_uint
                                            } else { // ideally would add 0s
                                                // unrounded
                                                // there are some to drop, but emax may not allow all
                                                let mut maxexp: int32_t =
                                                    (*set).emax -
                                                        (*set).digits +
                                                        1 as libc::c_int;
                                                let mut maxdrop: int32_t =
                                                    maxexp - (*a).exponent;
                                                if todrop > maxdrop &&
                                                       (*set).clamp as
                                                           libc::c_int != 0 {
                                                    // apply clamping
                                                    todrop = maxdrop;
                                                    status |=
                                                        0x400 as libc::c_int
                                                            as libc::c_uint
                                                }
                                                if dropped < todrop {
                                                    // clamp to those available
                                                    todrop = dropped;
                                                    status |=
                                                        0x400 as libc::c_int
                                                            as libc::c_uint
                                                }
                                                if todrop > 0 as libc::c_int {
                                                    // have some to drop
                                                    decShiftToLeast((*a).lsu.as_mut_ptr(),
                                                                    if (*a).digits
                                                                           <=
                                                                           49
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        d2utable[(*a).digits
                                                                                     as
                                                                                     usize]
                                                                            as
                                                                            libc::c_int
                                                                    } else {
                                                                        ((*a).digits
                                                                             +
                                                                             3
                                                                                 as
                                                                                 libc::c_int
                                                                             -
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                                                                            /
                                                                            3
                                                                                as
                                                                                libc::c_int
                                                                    },
                                                                    todrop);
                                                    // new length
                                                    (*a).exponent +=
                                                        todrop; // maintain numerical value
                                                    (*a).digits -= todrop
                                                }
                                            }
                                        }
                                    }
                                }
                                // double-check Underflow, as perhaps the result could not have
    // been subnormal (initial argument too big), or it is now Exact
                                if status &
                                       0x2000 as libc::c_int as libc::c_uint
                                       != 0 {
                                    let mut ae: int32_t =
                                        (*rhs).exponent + (*rhs).digits -
                                            1 as
                                                libc::c_int; // adjusted exponent
                                    // check if truly subnormal
                                    // DEC_Subnormal too
                                    if ae >= (*set).emin * 2 as libc::c_int {
                                        status &=
                                            !(0x1000 as libc::c_int |
                                                  0x2000 as libc::c_int) as
                                                libc::c_uint
                                    }
                                    // check if truly inexact
                                    if status &
                                           0x20 as libc::c_int as libc::c_uint
                                           == 0 {
                                        status &=
                                            !(0x2000 as libc::c_int) as
                                                libc::c_uint
                                    }
                                }
                                decNumberCopy(res, a);
                            }
                        }
                    }
                }
            }
        }
    }
    // Overflow was possible if the input exponent was out-of-range,
    // in which case quit
    // a is now the result
    // protect allocated storage
    // [following code does not require input rounding]
    // handle infinities and NaNs
    if !allocbuff.is_null() {
        free(allocbuff as *mut libc::c_void); // drop any storage used
    } // ..
    if !allocbufa.is_null() {
        free(allocbufa as *mut libc::c_void); // ..
    } // then report status
    if !allocbufb.is_null() { free(allocbufb as *mut libc::c_void); }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberSquareRoot
/* ------------------------------------------------------------------ */
/* decNumberSubtract -- subtract two Numbers                          */
/*                                                                    */
/*   This computes C = A - B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X-X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberSubtract(mut res: *mut decNumber,
                                           mut lhs: *const decNumber,
                                           mut rhs: *const decNumber,
                                           mut set: *mut decContext)
 -> *mut decNumber {
    let mut status: uint32_t = 0 as libc::c_int as uint32_t; // accumulator
    decAddOp(res, lhs, rhs, set, 0x80 as libc::c_int as uint8_t, &mut status);
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberSubtract
/* ------------------------------------------------------------------ */
/* decNumberToIntegralExact -- round-to-integral-value with InExact   */
/* decNumberToIntegralValue -- round-to-integral-value                */
/*                                                                    */
/*   res is the result                                                */
/*   rhs is input number                                              */
/*   set is the context                                               */
/*                                                                    */
/* res must have space for any value of rhs.                          */
/*                                                                    */
/* This implements the IEEE special operators and therefore treats    */
/* special values as valid.  For finite numbers it returns            */
/* rescale(rhs, 0) if rhs->exponent is <0.                            */
/* Otherwise the result is rhs (so no error is possible, except for   */
/* sNaN).                                                             */
/*                                                                    */
/* The context is used for rounding mode and status after sNaN, but   */
/* the digits setting is ignored.  The Exact version will signal      */
/* Inexact if the result differs numerically from rhs; the other      */
/* never signals Inexact.                                             */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberToIntegralExact(mut res: *mut decNumber,
                                                  mut rhs: *const decNumber,
                                                  mut set: *mut decContext)
 -> *mut decNumber {
    let mut dn: decNumber =
        decNumber{digits: 0,
                  exponent: 0,
                  bits: 0,
                  lsu: [0; 1],}; // working context
    let mut workset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // accumulator
    let mut status: uint32_t = 0 as libc::c_int as uint32_t;
    // handle infinities and NaNs
    if (*rhs).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
               0 as libc::c_int {
            decNumberCopy(res, rhs); // an Infinity
        } else { decNaNs(res, rhs, 0 as *const decNumber, set, &mut status); }
        // a NaN
    } else {
        // finite
        // have a finite number; no error possible (res must be big enough)
        if (*rhs).exponent >= 0 as libc::c_int {
            return decNumberCopy(res, rhs)
        }
        // that was easy, but if negative exponent there is work to do...
        workset = *set; // clone rounding, etc.
        workset.digits = (*rhs).digits; // no length rounding
        workset.traps = 0 as libc::c_int as uint32_t; // no traps
        decNumberZero(&mut dn); // make a number with exponent 0
        decNumberQuantize(res, rhs, &mut dn, &mut workset);
        status |= workset.status
    }
    if status != 0 as libc::c_int as libc::c_uint {
        decStatus(res, status, set);
    }
    return res;
}
// decNumberToIntegralExact
#[no_mangle]
pub unsafe extern "C" fn decNumberToIntegralValue(mut res: *mut decNumber,
                                                  mut rhs: *const decNumber,
                                                  mut set: *mut decContext)
 -> *mut decNumber {
    let mut workset: decContext = *set; // working context
    workset.traps = 0 as libc::c_int as uint32_t; // no traps
    decNumberToIntegralExact(res, rhs, &mut workset);
    // this never affects set, except for sNaNs; NaN will have been set
  // or propagated already, so no need to call decStatus
    (*set).status |= workset.status & 0x80 as libc::c_int as libc::c_uint;
    return res;
}
// decNumberToIntegralValue
/* ------------------------------------------------------------------ */
/* decNumberXor -- XOR two Numbers, digitwise                         */
/*                                                                    */
/*   This computes C = A ^ B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X^X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context (used for result length and error report)     */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Logical function restrictions apply (see above); a NaN is          */
/* returned with Invalid_operation if a restriction is violated.      */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberXor(mut res: *mut decNumber,
                                      mut lhs: *const decNumber,
                                      mut rhs: *const decNumber,
                                      mut set: *mut decContext)
 -> *mut decNumber {
    let mut ua: *const uint16_t = 0 as *const uint16_t; // -> operands
    let mut ub: *const uint16_t = 0 as *const uint16_t; // -> operand msus
    let mut msua: *const uint16_t =
        0 as *const uint16_t; // -> result and its msu
    let mut msub: *const uint16_t = 0 as *const uint16_t; // digits in res msu
    let mut uc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msuc: *mut uint16_t = 0 as *mut uint16_t;
    let mut msudigs: int32_t = 0;
    if (*lhs).exponent != 0 as libc::c_int ||
           (*lhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int || (*rhs).exponent != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) != 0 as libc::c_int ||
           (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
        decStatus(res, 0x80 as libc::c_int as uint32_t, set);
        return res
    }
    // operands are valid
    ua = (*lhs).lsu.as_ptr(); // bottom-up
    ub = (*rhs).lsu.as_ptr(); // ..
    uc = (*res).lsu.as_mut_ptr(); // ..
    msua =
        ua.offset((if (*lhs).digits <= 49 as libc::c_int {
                       d2utable[(*lhs).digits as usize] as libc::c_int
                   } else {
                       ((*lhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of lhs
    msub =
        ub.offset((if (*rhs).digits <= 49 as libc::c_int {
                       d2utable[(*rhs).digits as usize] as libc::c_int
                   } else {
                       ((*rhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of rhs
    msuc =
        uc.offset((if (*set).digits <= 49 as libc::c_int {
                       d2utable[(*set).digits as usize] as libc::c_int
                   } else {
                       ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) as
                      isize).offset(-(1 as libc::c_int as
                                          isize)); // -> msu of result
    msudigs =
        (*set).digits -
            ((if (*set).digits <= 49 as libc::c_int {
                  d2utable[(*set).digits as usize] as libc::c_int
              } else {
                  ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                      3 as libc::c_int
              }) - 1 as libc::c_int) *
                3 as libc::c_int; // [faster than remainder]
    while uc <= msuc { // each unit
        // Unit loop
        let mut a: uint16_t = 0; // extract units
        let mut b: uint16_t = 0; // can now write back
        if ua > msua { a = 0 as libc::c_int as uint16_t } else { a = *ua }
        if ub > msub { b = 0 as libc::c_int as uint16_t } else { b = *ub }
        *uc = 0 as libc::c_int as uint16_t;
        if a as libc::c_int | b as libc::c_int != 0 {
            // maybe 1 bits to examine
            let mut i: int32_t = 0;
            let mut j: int32_t = 0;
            // This loop could be unrolled and/or use BIN2BCD tables
            i = 0 as libc::c_int; // effect XOR
            while i < 3 as libc::c_int {
                if (a as libc::c_int ^ b as libc::c_int) & 1 as libc::c_int !=
                       0 {
                    *uc =
                        (*uc as libc::c_int +
                             DECPOWERS[i as usize] as uint16_t as libc::c_int)
                            as uint16_t
                }
                j = a as libc::c_int % 10 as libc::c_int;
                a = (a as libc::c_int / 10 as libc::c_int) as uint16_t;
                j |= b as libc::c_int % 10 as libc::c_int;
                b = (b as libc::c_int / 10 as libc::c_int) as uint16_t;
                if j > 1 as libc::c_int {
                    decStatus(res, 0x80 as libc::c_int as uint32_t, set);
                    return res
                }
                if uc == msuc && i == msudigs - 1 as libc::c_int { break ; }
                i += 1
                // just did final digit
            }
        }
        ua = ua.offset(1);
        ub = ub.offset(1);
        uc = uc.offset(1)
    }
    // [here uc-1 is the msu of the result]
    (*res).digits =
        decGetDigits((*res).lsu.as_mut_ptr(),
                     uc.wrapping_offset_from((*res).lsu.as_mut_ptr()) as
                         libc::c_long as int32_t); // integer
    (*res).exponent = 0 as libc::c_int; // sign=0
    (*res).bits = 0 as libc::c_int as uint8_t;
    return res;
    // [no status to set]
}
// decNumberXor
/* ================================================================== */
/* Utility routines                                                   */
/* ================================================================== */
/* ------------------------------------------------------------------ */
/* decNumberClass -- return the decClass of a decNumber               */
/*   dn -- the decNumber to test                                      */
/*   set -- the context to use for Emin                               */
/*   returns the decClass enum                                        */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberClass(mut dn: *const decNumber,
                                        mut set: *mut decContext)
 -> decClass {
    if (*dn).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 as libc::c_int {
        if (*dn).bits as libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int
           {
            return DEC_CLASS_QNAN
        }
        if (*dn).bits as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int
           {
            return DEC_CLASS_SNAN
        }
        // must be an infinity
        if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int
           {
            return DEC_CLASS_NEG_INF
        }
        return DEC_CLASS_POS_INF
    }
    // is finite
    if decNumberIsNormal(dn, set) != 0 {
        // most common
        if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int
           {
            return DEC_CLASS_NEG_NORMAL
        }
        return DEC_CLASS_POS_NORMAL
    }
    // is subnormal or zero
    if *(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        // most common
        if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int
           {
            return DEC_CLASS_NEG_ZERO
        }
        return DEC_CLASS_POS_ZERO
    }
    if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int {
        return DEC_CLASS_NEG_SUBNORMAL
    }
    return DEC_CLASS_POS_SUBNORMAL;
}
// decNumberClass
/* ------------------------------------------------------------------ */
/* decNumberClassToString -- convert decClass to a string             */
/*                                                                    */
/*  eclass is a valid decClass                                        */
/*  returns a constant string describing the class (max 13+1 chars)   */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberClassToString(mut eclass: decClass)
 -> *const libc::c_char {
    if eclass as libc::c_uint ==
           DEC_CLASS_POS_NORMAL as libc::c_int as libc::c_uint {
        return b"+Normal\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_NEG_NORMAL as libc::c_int as libc::c_uint {
        return b"-Normal\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_POS_ZERO as libc::c_int as libc::c_uint {
        return b"+Zero\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_NEG_ZERO as libc::c_int as libc::c_uint {
        return b"-Zero\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_POS_SUBNORMAL as libc::c_int as libc::c_uint {
        return b"+Subnormal\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_NEG_SUBNORMAL as libc::c_int as libc::c_uint {
        return b"-Subnormal\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_POS_INF as libc::c_int as libc::c_uint {
        return b"+Infinity\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint ==
           DEC_CLASS_NEG_INF as libc::c_int as libc::c_uint {
        return b"-Infinity\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint == DEC_CLASS_QNAN as libc::c_int as libc::c_uint
       {
        return b"NaN\x00" as *const u8 as *const libc::c_char
    }
    if eclass as libc::c_uint == DEC_CLASS_SNAN as libc::c_int as libc::c_uint
       {
        return b"sNaN\x00" as *const u8 as *const libc::c_char
    }
    return b"Invalid\x00" as *const u8 as *const libc::c_char;
    // Unknown
}
// decNumberClassToString
/* ------------------------------------------------------------------ */
/* decNumberCopy -- copy a number                                     */
/*                                                                    */
/*   dest is the target decNumber                                     */
/*   src  is the source decNumber                                     */
/*   returns dest                                                     */
/*                                                                    */
/* (dest==src is allowed and is a no-op)                              */
/* All fields are updated as required.  This is a utility operation,  */
/* so special values are unchanged and no error is possible.          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCopy(mut dest: *mut decNumber,
                                       mut src: *const decNumber)
 -> *mut decNumber {
    if dest == src as *mut decNumber { return dest } // no copy required
    // Use explicit assignments here as structure assignment could copy
  // more than just the lsu (for small DECDPUN).  This would not affect
  // the value of the results, but could disturb test harness spill
  // checking.
    (*dest).bits = (*src).bits;
    (*dest).exponent = (*src).exponent;
    (*dest).digits = (*src).digits;
    *(*dest).lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
        *(*src).lsu.as_ptr().offset(0 as libc::c_int as isize);
    if (*src).digits > 3 as libc::c_int {
        // more Units to come
        let mut smsup: *const uint16_t = 0 as *const uint16_t; // work
        let mut s: *const uint16_t = 0 as *const uint16_t; // ..
        let mut d: *mut uint16_t = 0 as *mut uint16_t;
        // memcpy for the remaining Units would be safe as they cannot
    // overlap.  However, this explicit loop is faster in short cases.
        d =
            (*dest).lsu.as_mut_ptr().offset(1 as libc::c_int as
                                                isize); // -> first destination
        smsup =
            (*src).lsu.as_ptr().offset((if (*src).digits <= 49 as libc::c_int
                                           {
                                            d2utable[(*src).digits as usize]
                                                as libc::c_int
                                        } else {
                                            ((*src).digits + 3 as libc::c_int
                                                 - 1 as libc::c_int) /
                                                3 as libc::c_int
                                        }) as isize); // -> source msu+1
        s = (*src).lsu.as_ptr().offset(1 as libc::c_int as isize);
        while s < smsup { *d = *s; s = s.offset(1); d = d.offset(1) }
    }
    return dest;
}
// decNumberCopy
/* ------------------------------------------------------------------ */
/* decNumberCopyAbs -- quiet absolute value operator                  */
/*                                                                    */
/*   This sets C = abs(A)                                             */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* No exception or error can occur; this is a quiet bitwise operation.*/
/* See also decNumberAbs for a checking version of this.              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCopyAbs(mut res: *mut decNumber,
                                          mut rhs: *const decNumber)
 -> *mut decNumber {
    decNumberCopy(res, rhs); // turn off sign
    (*res).bits =
        ((*res).bits as libc::c_int & !(0x80 as libc::c_int)) as uint8_t;
    return res;
}
// decNumberCopyAbs
/* ------------------------------------------------------------------ */
/* decNumberCopyNegate -- quiet negate value operator                 */
/*                                                                    */
/*   This sets C = negate(A)                                          */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* No exception or error can occur; this is a quiet bitwise operation.*/
/* See also decNumberMinus for a checking version of this.            */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCopyNegate(mut res: *mut decNumber,
                                             mut rhs: *const decNumber)
 -> *mut decNumber {
    decNumberCopy(res, rhs); // invert the sign
    (*res).bits =
        ((*res).bits as libc::c_int ^ 0x80 as libc::c_int) as uint8_t;
    return res;
}
// decNumberCopyNegate
/* ------------------------------------------------------------------ */
/* decNumberCopySign -- quiet copy and set sign operator              */
/*                                                                    */
/*   This sets C = A with the sign of B                               */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* No exception or error can occur; this is a quiet bitwise operation.*/
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberCopySign(mut res: *mut decNumber,
                                           mut lhs: *const decNumber,
                                           mut rhs: *const decNumber)
 -> *mut decNumber {
    let mut sign: uint8_t = 0; // rhs sign
    sign =
        ((*rhs).bits as libc::c_int & 0x80 as libc::c_int) as
            uint8_t; // save sign bit
    decNumberCopy(res, lhs); // clear the sign
    (*res).bits =
        ((*res).bits as libc::c_int & !(0x80 as libc::c_int)) as
            uint8_t; // set from rhs
    (*res).bits =
        ((*res).bits as libc::c_int | sign as libc::c_int) as uint8_t;
    return res;
}
// decNumberCopySign
/* ------------------------------------------------------------------ */
/* decNumberGetBCD -- get the coefficient in BCD8                     */
/*   dn is the source decNumber                                       */
/*   bcd is the uInt array that will receive dn->digits BCD bytes,    */
/*     most-significant at offset 0                                   */
/*   returns bcd                                                      */
/*                                                                    */
/* bcd must have at least dn->digits bytes.  No error is possible; if */
/* dn is a NaN or Infinite, digits must be 1 and the coefficient 0.   */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberGetBCD(mut dn: *const decNumber,
                                         mut bcd: *mut uint8_t)
 -> *mut uint8_t {
    let mut ub: *mut uint8_t =
        bcd.offset((*dn).digits as
                       isize).offset(-(1 as libc::c_int as isize)); // -> lsd
    let mut up: *const uint16_t = (*dn).lsu.as_ptr(); // Unit pointer, -> lsu
    // trivial simple copy
    // chopping needed
    let mut u: uint32_t = *up as uint32_t; // work
    let mut cut: uint32_t =
        3 as libc::c_int as uint32_t; // downcounter through unit
    while ub >= bcd {
        *ub =
            u.wrapping_rem(10 as libc::c_int as libc::c_uint) as
                uint8_t; // [*6554 trick inhibits, here]
        u =
            u.wrapping_div(10 as libc::c_int as
                               libc::c_uint); // more in this unit
        cut = cut.wrapping_sub(1);
        if !(cut > 0 as libc::c_int as libc::c_uint) {
            up = up.offset(1);
            u = *up as uint32_t;
            cut = 3 as libc::c_int as uint32_t
        }
        ub = ub.offset(-1)
    }
    return bcd;
}
// decNumberGetBCD
/* ------------------------------------------------------------------ */
/* decNumberSetBCD -- set (replace) the coefficient from BCD8         */
/*   dn is the target decNumber                                       */
/*   bcd is the uInt array that will source n BCD bytes, most-        */
/*     significant at offset 0                                        */
/*   n is the number of digits in the source BCD array (bcd)          */
/*   returns dn                                                       */
/*                                                                    */
/* dn must have space for at least n digits.  No error is possible;   */
/* if dn is a NaN, or Infinite, or is to become a zero, n must be 1   */
/* and bcd[0] zero.                                                   */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberSetBCD(mut dn: *mut decNumber,
                                         mut bcd: *const uint8_t,
                                         mut n: uint32_t) -> *mut decNumber {
    let mut up: *mut uint16_t =
        (*dn).lsu.as_mut_ptr().offset((if (*dn).digits <= 49 as libc::c_int {
                                           d2utable[(*dn).digits as usize] as
                                               libc::c_int
                                       } else {
                                           ((*dn).digits + 3 as libc::c_int -
                                                1 as libc::c_int) /
                                               3 as libc::c_int
                                       }) as
                                          isize).offset(-(1 as libc::c_int as
                                                              isize)); // -> msu [target pointer]
    let mut ub: *const uint8_t = bcd; // -> source msd
    // trivial simple copy
    // some assembly needed
    // calculate how many digits in msu, and hence first cut
    let mut cut: int32_t =
        n.wrapping_sub((if n <= 49 as libc::c_int as libc::c_uint {
                            d2utable[n as usize] as libc::c_uint
                        } else {
                            n.wrapping_add(3 as libc::c_int as
                                               libc::c_uint).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint).wrapping_div(3
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                        }).wrapping_sub(1 as libc::c_int as
                                            libc::c_uint).wrapping_mul(3 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint))
            as int32_t; // [faster than remainder]
    while up >= (*dn).lsu.as_mut_ptr() {
        // each Unit from msu
        *up = 0 as libc::c_int as uint16_t;
        while cut > 0 as libc::c_int {
            *up =
                (((*up as libc::c_int) << 1 as libc::c_int) +
                     ((*up as libc::c_int) << 3 as libc::c_int) +
                     *ub as libc::c_int) as
                    uint16_t; // will take <=DECDPUN digits
            ub = ub.offset(1);
            cut -= 1
        }
        cut = 3 as libc::c_int;
        up = up.offset(-1)
        // next Unit has all digits
    } // set digit count
    (*dn).digits = n as int32_t;
    return dn;
}
/* Functions for testing decNumbers (normality depends on context)  */
// decNumberSetBCD
/* ------------------------------------------------------------------ */
/* decNumberIsNormal -- test normality of a decNumber                 */
/*   dn is the decNumber to test                                      */
/*   set is the context to use for Emin                               */
/*   returns 1 if |dn| is finite and >=Nmin, 0 otherwise              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberIsNormal(mut dn: *const decNumber,
                                           mut set: *mut decContext)
 -> int32_t {
    let mut ae: int32_t = 0; // adjusted exponent
    if (*dn).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 as libc::c_int {
        return 0 as libc::c_int
    } // not finite
    if *(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        return 0 as libc::c_int
    } // not non-zero
    ae =
        (*dn).exponent + (*dn).digits - 1 as libc::c_int; // adjusted exponent
    if ae < (*set).emin { return 0 as libc::c_int } // is subnormal
    return 1 as libc::c_int;
}
// decNumberIsNormal
/* ------------------------------------------------------------------ */
/* decNumberIsSubnormal -- test subnormality of a decNumber           */
/*   dn is the decNumber to test                                      */
/*   set is the context to use for Emin                               */
/*   returns 1 if |dn| is finite, non-zero, and <Nmin, 0 otherwise    */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberIsSubnormal(mut dn: *const decNumber,
                                              mut set: *mut decContext)
 -> int32_t {
    let mut ae: int32_t = 0; // adjusted exponent
    if (*dn).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 as libc::c_int {
        return 0 as libc::c_int
    } // not finite
    if *(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        return 0 as libc::c_int
    } // not non-zero
    ae =
        (*dn).exponent + (*dn).digits - 1 as libc::c_int; // adjusted exponent
    if ae < (*set).emin { return 1 as libc::c_int } // is subnormal
    return 0 as libc::c_int;
}
// decNumberIsSubnormal
/* ------------------------------------------------------------------ */
/* decNumberTrim -- remove insignificant zeros                        */
/*                                                                    */
/*   dn is the number to trim                                         */
/*   returns dn                                                       */
/*                                                                    */
/* All fields are updated as required.  This is a utility operation,  */
/* so special values are unchanged and no error is possible.  The     */
/* zeros are removed unconditionally.                                 */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberTrim(mut dn: *mut decNumber)
 -> *mut decNumber {
    let mut dropped: int32_t = 0; // work
    let mut set: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // ..
    decContextDefault(&mut set, 0 as libc::c_int); // clamp=0
    return decTrim(dn, &mut set, 0 as libc::c_int as uint8_t,
                   1 as libc::c_int as uint8_t, &mut dropped);
}
// decNumberTrim
/* ------------------------------------------------------------------ */
/* decNumberVersion -- return the name and version of this module     */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decNumberVersion() -> *const libc::c_char {
    return b"decNumber 3.68\x00" as *const u8 as *const libc::c_char;
}
// decNumberVersion
/* ------------------------------------------------------------------ */
/* decNumberZero -- set a number to 0                                 */
/*                                                                    */
/*   dn is the number to set, with space for one digit                */
/*   returns dn                                                       */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
// Memset is not used as it is much slower in some environments.
#[no_mangle]
pub unsafe extern "C" fn decNumberZero(mut dn: *mut decNumber)
 -> *mut decNumber {
    (*dn).bits = 0 as libc::c_int as uint8_t;
    (*dn).exponent = 0 as libc::c_int;
    (*dn).digits = 1 as libc::c_int;
    *(*dn).lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
        0 as libc::c_int as uint16_t;
    return dn;
}
// decNumberZero
/* ================================================================== */
/* Local routines                                                     */
/* ================================================================== */
/* ------------------------------------------------------------------ */
/* decToString -- lay out a number into a string                      */
/*                                                                    */
/*   dn     is the number to lay out                                  */
/*   string is where to lay out the number                            */
/*   eng    is 1 if Engineering, 0 if Scientific                      */
/*                                                                    */
/* string must be at least dn->digits+14 characters long              */
/* No error is possible.                                              */
/*                                                                    */
/* Note that this routine can generate a -0 or 0.000.  These are      */
/* never generated in subset to-number or arithmetic, but can occur   */
/* in non-subset arithmetic (e.g., -1*0 or 1.234-1.234).              */
/* ------------------------------------------------------------------ */
// If DECCHECK is enabled the string "?" is returned if a number is
// invalid.
unsafe extern "C" fn decToString(mut dn: *const decNumber,
                                 mut string: *mut libc::c_char,
                                 mut eng: uint8_t) {
    let mut exp: int32_t = (*dn).exponent; // local copy
    let mut e: int32_t = 0; // E-part value
    let mut pre: int32_t = 0; // digits before the '.'
    let mut cut: int32_t = 0; // for counting digits in a Unit
    let mut c: *mut libc::c_char = string; // work [output pointer]
    let mut up: *const uint16_t =
        (*dn).lsu.as_ptr().offset((if (*dn).digits <= 49 as libc::c_int {
                                       d2utable[(*dn).digits as usize] as
                                           libc::c_int
                                   } else {
                                       ((*dn).digits + 3 as libc::c_int -
                                            1 as libc::c_int) /
                                           3 as libc::c_int
                                   }) as
                                      isize).offset(-(1 as libc::c_int as
                                                          isize)); // -> msu [input pointer]
    let mut u: uint32_t = 0; // work
    let mut pow: uint32_t = 0;
    if (*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int {
        // Negatives get a minus
        *c = '-' as i32 as libc::c_char;
        c = c.offset(1)
    }
    if (*dn).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // Is a special value
        if (*dn).bits as libc::c_int & 0x40 as libc::c_int != 0 as libc::c_int
           {
            strcpy(c, b"Inf\x00" as *const u8 as *const libc::c_char);
            strcpy(c.offset(3 as libc::c_int as isize),
                   b"inity\x00" as *const u8 as *const libc::c_char);
            return
        }
        // [drop through to add integer]
        if (*dn).bits as libc::c_int & 0x10 as libc::c_int != 0 {
            // a NaN
            // signalling NaN
            *c = 's' as i32 as libc::c_char; // step past
            c = c.offset(1)
        }
        strcpy(c, b"NaN\x00" as *const u8 as *const libc::c_char);
        c = c.offset(3 as libc::c_int as isize);
        if exp != 0 as libc::c_int ||
               *(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                   (*dn).digits == 1 as libc::c_int {
            return
        }
    }
    // if not a clean non-zero coefficient, that's all there is in a
    // NaN string
    // calculate how many digits in msu, and hence first cut
    cut =
        (*dn).digits -
            ((if (*dn).digits <= 49 as libc::c_int {
                  d2utable[(*dn).digits as usize] as libc::c_int
              } else {
                  ((*dn).digits + 3 as libc::c_int - 1 as libc::c_int) /
                      3 as libc::c_int
              }) - 1 as libc::c_int) *
                3 as libc::c_int; // [faster than remainder]
    cut -= 1; // power of ten for digit
    if exp == 0 as libc::c_int {
        // simple integer [common fastpath]
        while up >= (*dn).lsu.as_ptr() {
            // each Unit from msu
            u = *up as uint32_t;
            while cut >= 0 as libc::c_int {
                *c =
                    '0' as i32 as
                        libc::c_char; // contains DECDPUN digits to lay out
                pow =
                    DECPOWERS[cut as
                                  usize].wrapping_mul(2 as libc::c_int as
                                                          libc::c_uint);
                if u > pow {
                    pow =
                        (pow as
                             libc::c_uint).wrapping_mul(4 as libc::c_int as
                                                            libc::c_uint) as
                            uint32_t as uint32_t;
                    if u >= pow {
                        u =
                            (u as libc::c_uint).wrapping_sub(pow) as uint32_t
                                as uint32_t;
                        *c =
                            (*c as libc::c_int + 8 as libc::c_int) as
                                libc::c_char
                    }
                    pow =
                        (pow as
                             libc::c_uint).wrapping_div(2 as libc::c_int as
                                                            libc::c_uint) as
                            uint32_t as uint32_t;
                    if u >= pow {
                        u =
                            (u as libc::c_uint).wrapping_sub(pow) as uint32_t
                                as uint32_t;
                        *c =
                            (*c as libc::c_int + 4 as libc::c_int) as
                                libc::c_char
                    }
                    pow =
                        (pow as
                             libc::c_uint).wrapping_div(2 as libc::c_int as
                                                            libc::c_uint) as
                            uint32_t as uint32_t
                }
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 2 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 1 as libc::c_int) as libc::c_char
                }
                c = c.offset(1);
                cut -= 1
            }
            cut = 3 as libc::c_int - 1 as libc::c_int;
            up = up.offset(-1)
            // next Unit has all digits
        } // terminate the string
        *c = '\u{0}' as i32 as libc::c_char;
        return
    }
    /* non-0 exponent -- assume plain form */
    pre = (*dn).digits + exp; // digits before '.'
    e = 0 as libc::c_int; // no E
    if exp > 0 as libc::c_int || pre < -(5 as libc::c_int) { // need exponent
        // need exponential form
        e = exp + (*dn).digits - 1 as libc::c_int; // calculate E value
        // eng
        pre = 1 as libc::c_int; // assume one digit before '.'
        if eng as libc::c_int != 0 && e != 0 as libc::c_int {
            // engineering: may need to adjust
            let mut adj: int32_t = 0; // adjustment
            // The C remainder operator is undefined for negative numbers, so
      // a positive remainder calculation must be used here
            if e < 0 as libc::c_int {
                adj = -e % 3 as libc::c_int;
                if adj != 0 as libc::c_int { adj = 3 as libc::c_int - adj }
            } else {
                // e>0
                adj = e % 3 as libc::c_int
            }
            e = e - adj;
            // if dealing with zero still produce an exponent which is a
      // multiple of three, as expected, but there will only be the
      // one zero before the E, still.  Otherwise note the padding.
            if !(*(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                     (*dn).digits == 1 as libc::c_int &&
                     (*dn).bits as libc::c_int &
                         (0x40 as libc::c_int | 0x20 as libc::c_int |
                              0x10 as libc::c_int) == 0 as libc::c_int) {
                pre += adj
            } else if adj != 0 as libc::c_int {
                // is zero
                // 0.00Esnn needed
                e = e + 3 as libc::c_int;
                pre = -(2 as libc::c_int - adj)
            }
        }
    }
    /* lay out the digits of the coefficient, adding 0s and . as needed */
    u = *up as uint32_t;
    if pre > 0 as libc::c_int {
        // xxx.xxx or xx00 (engineering) form
        let mut n: int32_t = pre;
        while pre > 0 as libc::c_int {
            if cut < 0 as libc::c_int {
                // need new Unit
                if up == (*dn).lsu.as_ptr() {
                    break ; // out of input digits (pre>digits)
                }
                up = up.offset(-1);
                cut = 3 as libc::c_int - 1 as libc::c_int;
                u = *up as uint32_t
            }
            *c = '0' as i32 as libc::c_char;
            pow =
                DECPOWERS[cut as
                              usize].wrapping_mul(2 as libc::c_int as
                                                      libc::c_uint);
            if u > pow {
                pow =
                    (pow as
                         libc::c_uint).wrapping_mul(4 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 8 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 4 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t
            }
            if u >= pow {
                u =
                    (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                        uint32_t;
                *c = (*c as libc::c_int + 2 as libc::c_int) as libc::c_char
            }
            pow =
                (pow as
                     libc::c_uint).wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            if u >= pow {
                u =
                    (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                        uint32_t;
                *c = (*c as libc::c_int + 1 as libc::c_int) as libc::c_char
            }
            pre -= 1;
            c = c.offset(1);
            cut -= 1
        }
        if n < (*dn).digits {
            // more to come, after '.'
            *c = '.' as i32 as libc::c_char;
            c = c.offset(1);
            loop  {
                if cut < 0 as libc::c_int {
                    // need new Unit
                    if up == (*dn).lsu.as_ptr() {
                        break ; // out of input digits
                    }
                    up = up.offset(-1);
                    cut = 3 as libc::c_int - 1 as libc::c_int;
                    u = *up as uint32_t
                }
                *c = '0' as i32 as libc::c_char;
                pow =
                    DECPOWERS[cut as
                                  usize].wrapping_mul(2 as libc::c_int as
                                                          libc::c_uint);
                if u > pow {
                    pow =
                        (pow as
                             libc::c_uint).wrapping_mul(4 as libc::c_int as
                                                            libc::c_uint) as
                            uint32_t as uint32_t;
                    if u >= pow {
                        u =
                            (u as libc::c_uint).wrapping_sub(pow) as uint32_t
                                as uint32_t;
                        *c =
                            (*c as libc::c_int + 8 as libc::c_int) as
                                libc::c_char
                    }
                    pow =
                        (pow as
                             libc::c_uint).wrapping_div(2 as libc::c_int as
                                                            libc::c_uint) as
                            uint32_t as uint32_t;
                    if u >= pow {
                        u =
                            (u as libc::c_uint).wrapping_sub(pow) as uint32_t
                                as uint32_t;
                        *c =
                            (*c as libc::c_int + 4 as libc::c_int) as
                                libc::c_char
                    }
                    pow =
                        (pow as
                             libc::c_uint).wrapping_div(2 as libc::c_int as
                                                            libc::c_uint) as
                            uint32_t as uint32_t
                }
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 2 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 1 as libc::c_int) as libc::c_char
                }
                c = c.offset(1);
                cut -= 1
            }
        } else {
            while pre > 0 as libc::c_int {
                *c = '0' as i32 as libc::c_char;
                pre -= 1;
                c = c.offset(1)
            }
        }
    } else {
        // 0.xxx or 0.000xxx form
        *c = '0' as i32 as libc::c_char; // add any 0's after '.'
        c = c.offset(1);
        *c = '.' as i32 as libc::c_char;
        c = c.offset(1);
        while pre < 0 as libc::c_int {
            *c = '0' as i32 as libc::c_char;
            pre += 1;
            c = c.offset(1)
        }
        loop  {
            if cut < 0 as libc::c_int {
                // need new Unit
                if up == (*dn).lsu.as_ptr() {
                    break ; // out of input digits
                }
                up = up.offset(-1);
                cut = 3 as libc::c_int - 1 as libc::c_int;
                u = *up as uint32_t
            }
            *c = '0' as i32 as libc::c_char;
            pow =
                DECPOWERS[cut as
                              usize].wrapping_mul(2 as libc::c_int as
                                                      libc::c_uint);
            if u > pow {
                pow =
                    (pow as
                         libc::c_uint).wrapping_mul(4 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 8 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 4 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t
            }
            if u >= pow {
                u =
                    (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                        uint32_t;
                *c = (*c as libc::c_int + 2 as libc::c_int) as libc::c_char
            }
            pow =
                (pow as
                     libc::c_uint).wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            if u >= pow {
                u =
                    (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                        uint32_t;
                *c = (*c as libc::c_int + 1 as libc::c_int) as libc::c_char
            }
            c = c.offset(1);
            cut -= 1
        }
    }
    /* Finally add the E-part, if needed.  It will never be 0, has a
     base maximum and minimum of +999999999 through -999999999, but
     could range down to -1999999998 for anormal numbers */
    if e != 0 as libc::c_int {
        let mut had: uint8_t = 0 as libc::c_int as uint8_t; // 1=had non-zero
        *c = 'E' as i32 as libc::c_char; // assume positive
        c = c.offset(1); // ..
        *c = '+' as i32 as libc::c_char; // oops, need -
        c = c.offset(1);
        u = e as uint32_t;
        if e < 0 as libc::c_int {
            *c.offset(-(1 as libc::c_int as isize)) =
                '-' as i32 as libc::c_char;
            u = -e as uint32_t
            // uInt, please
        }
        // lay out the exponent [_itoa or equivalent is not ANSI C]
        cut = 9 as libc::c_int;
        while cut >= 0 as libc::c_int {
            *c = '0' as i32 as libc::c_char;
            pow =
                DECPOWERS[cut as
                              usize].wrapping_mul(2 as libc::c_int as
                                                      libc::c_uint);
            if u > pow {
                pow =
                    (pow as
                         libc::c_uint).wrapping_mul(4 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 8 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                if u >= pow {
                    u =
                        (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                            uint32_t;
                    *c =
                        (*c as libc::c_int + 4 as libc::c_int) as libc::c_char
                }
                pow =
                    (pow as
                         libc::c_uint).wrapping_div(2 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t
            }
            if u >= pow {
                u =
                    (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                        uint32_t;
                *c = (*c as libc::c_int + 2 as libc::c_int) as libc::c_char
            }
            pow =
                (pow as
                     libc::c_uint).wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            if u >= pow {
                u =
                    (u as libc::c_uint).wrapping_sub(pow) as uint32_t as
                        uint32_t;
                *c = (*c as libc::c_int + 1 as libc::c_int) as libc::c_char
            }
            // step for next
            if !(*c as libc::c_int == '0' as i32 && had == 0)
               { // skip leading zeros
                had = 1 as libc::c_int as uint8_t; // had non-0
                c = c.offset(1)
            } // terminate the string (all paths)
            cut -= 1
        }
    }
    *c = '\u{0}' as i32 as libc::c_char;
}
/* Local routines */
// decToString
/* ------------------------------------------------------------------ */
/* decAddOp -- add/subtract operation                                 */
/*                                                                    */
/*   This computes C = A + B                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X+X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*   negate is DECNEG if rhs should be negated, or 0 otherwise        */
/*   status accumulates status for the caller                         */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/* Inexact in status must be 0 for correct Exact zero sign in result  */
/* ------------------------------------------------------------------ */
/* If possible, the coefficient is calculated directly into C.        */
/* However, if:                                                       */
/*   -- a digits+1 calculation is needed because the numbers are      */
/*      unaligned and span more than set->digits digits               */
/*   -- a carry to digits+1 digits looks possible                     */
/*   -- C is the same as A or B, and the result would destructively   */
/*      overlap the A or B coefficient                                */
/* then the result must be calculated into a temporary buffer.  In    */
/* this case a local (stack) buffer is used if possible, and only if  */
/* too long for that does malloc become the final resort.             */
/*                                                                    */
/* Misalignment is handled as follows:                                */
/*   Apad: (AExp>BExp) Swap operands and proceed as for BExp>AExp.    */
/*   BPad: Apply the padding by a combination of shifting (whole      */
/*         units) and multiplication (part units).                    */
/*                                                                    */
/* Addition, especially x=x+1, is speed-critical.                     */
/* The static buffer is larger than might be expected to allow for    */
/* calls from higher-level funtions (notable exp).                    */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decAddOp(mut res: *mut decNumber,
                              mut lhs: *const decNumber,
                              mut rhs: *const decNumber,
                              mut set: *mut decContext, mut negate: uint8_t,
                              mut status: *mut uint32_t) -> *mut decNumber {
    let mut rhsshift: int32_t = 0; // working shift (in Units)
    let mut maxdigits: int32_t = 0; // longest logical length
    let mut mult: int32_t = 0; // multiplier
    let mut residue: int32_t = 0; // rounding accumulator
    let mut bits: uint8_t = 0; // result bits
    let mut diffsign: uint8_t = 0; // non-0 if arguments have different sign
    let mut acc: *mut uint16_t = 0 as *mut uint16_t; // accumulator for result
    let mut accbuff: [uint16_t; 31] =
        [0; 31]; // local buffer [*2+20 reduces many
    // allocations when called from
                                   // other operations, notable exp]
    let mut allocacc: *mut uint16_t =
        0 as *mut uint16_t; // -> allocated acc buffer, iff allocated
    let mut reqdigits: int32_t =
        (*set).digits; // local copy; requested DIGITS
    let mut padding: int32_t = 0; // work
    let mut current_block_108: u64; // end protected
    // protect allocated storage
    // [following code does not require input rounding]
    // note whether signs differ [used all paths]
    diffsign =
        (((*lhs).bits as libc::c_int ^ (*rhs).bits as libc::c_int ^
              negate as libc::c_int) & 0x80 as libc::c_int) as uint8_t;
    // handle infinities and NaNs
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // a special bit set
        if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) &
               (0x10 as libc::c_int | 0x20 as libc::c_int) != 0
           { // an infinity
            // a NaN
            decNaNs(res, lhs, rhs, set, status);
        } else {
            // one or two infinities
            if (*lhs).bits as libc::c_int & 0x40 as libc::c_int !=
                   0 as libc::c_int { // RHS must be Infinity
                // LHS is infinity
                // two infinities with different signs is invalid
                if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                       0 as libc::c_int && diffsign as libc::c_int != 0 {
                    *status |= 0x80 as libc::c_int as libc::c_uint;
                    current_block_108 = 15460309861373144675;
                } else {
                    bits =
                        ((*lhs).bits as libc::c_int & 0x80 as libc::c_int) as
                            uint8_t;
                    current_block_108 = 12039483399334584727;
                }
                // get sign from LHS
            } else {
                bits =
                    (((*rhs).bits as libc::c_int ^ negate as libc::c_int) &
                         0x80 as libc::c_int) as uint8_t;
                current_block_108 = 12039483399334584727;
            }
            match current_block_108 {
                15460309861373144675 => { }
                _ => {
                    bits =
                        (bits as libc::c_int | 0x40 as libc::c_int) as
                            uint8_t;
                    decNumberZero(res);
                    (*res).bits = bits
                }
            }
            // set +/- infinity
        }
    } else if *(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*lhs).digits == 1 as libc::c_int &&
                  (*lhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        // Quick exit for add 0s; return the non-0, modified as need be
        let mut adjust: int32_t = 0; // work
        let mut lexp: int32_t = (*lhs).exponent; // save in case LHS==RES
        bits = (*lhs).bits; // ..
        residue = 0 as libc::c_int; // clear accumulator
        decCopyFit(res, rhs, set, &mut residue, status); // copy (as needed)
        (*res).bits =
            ((*res).bits as libc::c_int ^ negate as libc::c_int) as
                uint8_t; // flip if rhs was negated
        // exponent will be the lower of the two
        adjust = lexp - (*res).exponent; // adjustment needed [if -ve]
        if *(*res).lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
               (*res).digits == 1 as libc::c_int &&
               (*res).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) == 0 as libc::c_int
           { // non-0 res
            // both 0: special IEEE 754 rules
            if adjust < 0 as libc::c_int {
                (*res).exponent = lexp
            } // set exponent
            // 0-0 gives +0 unless rounding to -infinity, and -0-0 gives -0
            if diffsign != 0 {
                if (*set).round as libc::c_uint !=
                       DEC_ROUND_FLOOR as libc::c_int as libc::c_uint {
                    (*res).bits = 0 as libc::c_int as uint8_t
                } else { (*res).bits = 0x80 as libc::c_int as uint8_t }
                // preserve 0 sign
            }
        } else if adjust < 0 as libc::c_int {
            // non-0 res
            // 0-padding needed
            if (*res).digits - adjust > (*set).digits {
                adjust = (*res).digits - (*set).digits;
                *status |=
                    0x800 as libc::c_int as libc::c_uint // to fit exactly
                // [but exact]
            }
            (*res).digits =
                decShiftToMost((*res).lsu.as_mut_ptr(), (*res).digits,
                               -adjust);
            (*res).exponent += adjust
            // set the exponent.
        } // clean and finalize
        decFinalize(res, set, &mut residue, status);
    } else if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*rhs).digits == 1 as libc::c_int &&
                  (*rhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        // [lhs is non-zero]
        let mut adjust_0: int32_t = 0; // work
        let mut rexp: int32_t = (*rhs).exponent; // save in case RHS==RES
        bits = (*rhs).bits; // be clean
        residue = 0 as libc::c_int; // clear accumulator
        decCopyFit(res, lhs, set, &mut residue, status); // copy (as needed)
        // exponent will be the lower of the two
        // [0-0 case handled above]
        adjust_0 = rexp - (*res).exponent; // adjustment needed [if -ve]
        if adjust_0 < 0 as libc::c_int {
            // 0-padding needed
            if (*res).digits - adjust_0 > (*set).digits {
                adjust_0 = (*res).digits - (*set).digits;
                *status |=
                    0x800 as libc::c_int as libc::c_uint // to fit exactly
                // [but exact]
            }
            (*res).digits =
                decShiftToMost((*res).lsu.as_mut_ptr(), (*res).digits,
                               -adjust_0);
            (*res).exponent += adjust_0
            // set the exponent.
        } // clean and finalize
        decFinalize(res, set, &mut residue, status);
    } else {
        // [NB: both fastpath and mainpath code below assume these cases
    // (notably 0-0) have already been handled]
        // calculate the padding needed to align the operands
        padding = (*rhs).exponent - (*lhs).exponent;
        // Fastpath cases where the numbers are aligned and normal, the RHS
    // is all in one unit, no operand rounding is needed, and no carry,
    // lengthening, or borrow is needed
        if padding == 0 as libc::c_int && (*rhs).digits <= 3 as libc::c_int &&
               (*rhs).exponent >= (*set).emin &&
               (*rhs).exponent <=
                   (*set).emax - (*set).digits + 1 as libc::c_int &&
               (*rhs).digits <= reqdigits && (*lhs).digits <= reqdigits {
            let mut partial: int32_t = *(*lhs).lsu.as_ptr() as int32_t;
            if diffsign == 0 {
                // adding
                partial += *(*rhs).lsu.as_ptr() as libc::c_int;
                if partial <= 999 as libc::c_int &&
                       ((*lhs).digits >= 3 as libc::c_int ||
                            partial <
                                DECPOWERS[(*lhs).digits as usize] as int32_t)
                   {
                    // else drop out for careful add
                    // ..
                    if res != lhs as *mut decNumber {
                        decNumberCopy(res, lhs); // not in place
                    } // [copy could have overwritten RHS]
                    *(*res).lsu.as_mut_ptr() = partial as uint16_t;
                    current_block_108 = 15460309861373144675;
                } else { current_block_108 = 919954187481050311; }
            } else {
                // signs differ
                partial -= *(*rhs).lsu.as_ptr() as libc::c_int;
                if partial > 0 as libc::c_int {
                    // else drop out for careful subtract
                    // no borrow needed, and non-0 result
                    if res != lhs as *mut decNumber {
                        decNumberCopy(res, lhs); // not in place
                    }
                    *(*res).lsu.as_mut_ptr() = partial as uint16_t;
                    // this could have reduced digits [but result>0]
                    (*res).digits =
                        decGetDigits((*res).lsu.as_mut_ptr(),
                                     if (*res).digits <= 49 as libc::c_int {
                                         d2utable[(*res).digits as usize] as
                                             libc::c_int
                                     } else {
                                         ((*res).digits + 3 as libc::c_int -
                                              1 as libc::c_int) /
                                             3 as libc::c_int
                                     });
                    current_block_108 = 15460309861373144675;
                } else { current_block_108 = 919954187481050311; }
            }
        } else { current_block_108 = 919954187481050311; }
        match current_block_108 {
            15460309861373144675 => { }
            _ => {
                // Now align (pad) the lhs or rhs so they can be added or
    // subtracted, as necessary.  If one number is much larger than
    // the other (that is, if in plain form there is a least one
    // digit between the lowest digit of one and the highest of the
    // other) padding with up to DIGITS-1 trailing zeros may be
    // needed; then apply rounding (as exotic rounding modes may be
    // affected by the residue).
                rhsshift =
                    0 as libc::c_int; // rhs shift to left (padding) in Units
                bits = (*lhs).bits; // assume sign is that of LHS
                mult = 1 as libc::c_int; // likely multiplier
                // [if padding==0 the operands are aligned; no padding is needed]
                if padding != 0 as libc::c_int { // padding needed
                    // some padding needed; always pad the RHS, as any required
      // padding can then be effected by a simple combination of
      // shifts and a multiply
                    let mut swapped: uint8_t = 0 as libc::c_int as uint8_t;
                    if padding < 0 as libc::c_int {
                        // LHS needs the padding
                        let mut t: *const decNumber =
                            0 as *const decNumber; // will be +ve
                        padding = -padding; // assumed sign is now that of RHS
                        bits =
                            ((*rhs).bits as libc::c_int ^
                                 negate as libc::c_int) as uint8_t;
                        t = lhs;
                        lhs = rhs;
                        rhs = t;
                        swapped = 1 as libc::c_int as uint8_t
                    }
                    // If, after pad, rhs would be longer than lhs by digits+1 or
      // more then lhs cannot affect the answer, except as a residue,
      // so only need to pad up to a length of DIGITS+1.
                    if (*rhs).digits + padding >
                           (*lhs).digits + reqdigits + 1 as libc::c_int {
                        // The RHS is sufficient
        // for residue use the relative sign indication...
                        let mut shift: int32_t =
                            reqdigits - (*rhs).digits; // left shift needed
                        residue = 1 as libc::c_int; // residue for rounding
                        if diffsign != 0 {
                            residue = -residue
                        } // signs differ
                        // copy, shortening if necessary
                        decCopyFit(res, rhs, set, &mut residue, status);
                        // if it was already shorter, then need to pad with zeros
                        if shift > 0 as libc::c_int {
                            (*res).digits =
                                decShiftToMost((*res).lsu.as_mut_ptr(),
                                               (*res).digits, shift);
                            (*res).exponent -= shift
                            // adjust the exponent.
                        }
                        // flip the result sign if unswapped and rhs was negated
                        if swapped == 0 {
                            (*res).bits =
                                ((*res).bits as libc::c_int ^
                                     negate as libc::c_int) as uint8_t
                        } // done
                        decFinalize(res, set, &mut residue, status);
                        current_block_108 = 15460309861373144675;
                    } else {
                        // LHS digits may affect result
                        rhsshift =
                            (if padding + 1 as libc::c_int <=
                                    49 as libc::c_int {
                                 d2utable[(padding + 1 as libc::c_int) as
                                              usize] as libc::c_int
                             } else {
                                 (padding + 1 as libc::c_int +
                                      3 as libc::c_int - 1 as libc::c_int) /
                                     3 as libc::c_int
                             }) -
                                1 as
                                    libc::c_int; // this much by Unit shift ..
                        mult =
                            DECPOWERS[(padding - rhsshift * 3 as libc::c_int)
                                          as usize] as int32_t;
                        current_block_108 = 5700653730392116747;
                    }
                    // .. this by multiplication
                } else {
                    current_block_108 = 5700653730392116747; // signs differ
                }
                match current_block_108 {
                    15460309861373144675 => { }
                    _ => {
                        if diffsign != 0 { mult = -mult }
                        // determine the longer operand
                        maxdigits =
                            (*rhs).digits + padding; // virtual length of RHS
                        if (*lhs).digits > maxdigits {
                            maxdigits = (*lhs).digits
                        }
                        // Decide on the result buffer to use; if possible place directly
    // into result.
                        acc =
                            (*res).lsu.as_mut_ptr(); // assume add direct to result
                        // If destructive overlap, or the number is too long, or a carry or
    // borrow to DIGITS+1 might be possible, a buffer must be used.
    // [Might be worth more sophisticated tests when maxdigits==reqdigits]
                        if maxdigits >= reqdigits ||
                               res == rhs as *mut decNumber &&
                                   rhsshift > 0 as libc::c_int {
                            // destructive overlap
                            // buffer needed, choose it; units for maxdigits digits will be
      // needed, +1 Unit for carry or borrow
                            let mut need: int32_t =
                                (if maxdigits <= 49 as libc::c_int {
                                     d2utable[maxdigits as usize] as
                                         libc::c_int
                                 } else {
                                     (maxdigits + 3 as libc::c_int -
                                          1 as libc::c_int) / 3 as libc::c_int
                                 }) +
                                    1 as
                                        libc::c_int; // assume use local buffer
                            acc = accbuff.as_mut_ptr();
                            if (need as
                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                    as
                                                                    libc::c_ulong)
                                   >
                                   ::std::mem::size_of::<[uint16_t; 31]>() as
                                       libc::c_ulong {
                                // printf("malloc add %ld %ld\n", need, sizeof(accbuff));
                                allocacc =
                                    malloc((need as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                as
                                                                                libc::c_ulong))
                                        as *mut uint16_t;
                                if allocacc.is_null() {
                                    // hopeless -- abandon
                                    *status |=
                                        0x10 as libc::c_int as
                                            libc::c_uint; // it's now safe to overwrite..
                                    current_block_108 =
                                        15460309861373144675; // .. operands (even if aliased)
                                } else {
                                    acc = allocacc;
                                    current_block_108 = 8602574157404971894;
                                }
                            } else {
                                current_block_108 = 8602574157404971894;
                            }
                        } else { current_block_108 = 8602574157404971894; }
                        match current_block_108 {
                            15460309861373144675 => { }
                            _ => {
                                (*res).bits =
                                    (bits as libc::c_int &
                                         0x80 as libc::c_int) as uint8_t;
                                (*res).exponent = (*lhs).exponent;
                                // add [A+B*m] or subtract [A+B*(-m)]
                                (*res).digits =
                                    decUnitAddSub((*lhs).lsu.as_ptr(),
                                                  (if (*lhs).digits <=
                                                          49 as libc::c_int {
                                                       d2utable[(*lhs).digits
                                                                    as usize]
                                                           as libc::c_int
                                                   } else {
                                                       ((*lhs).digits +
                                                            3 as libc::c_int -
                                                            1 as libc::c_int)
                                                           / 3 as libc::c_int
                                                   }), (*rhs).lsu.as_ptr(),
                                                  (if (*rhs).digits <=
                                                          49 as libc::c_int {
                                                       d2utable[(*rhs).digits
                                                                    as usize]
                                                           as libc::c_int
                                                   } else {
                                                       ((*rhs).digits +
                                                            3 as libc::c_int -
                                                            1 as libc::c_int)
                                                           / 3 as libc::c_int
                                                   }), rhsshift, acc, mult) *
                                        3 as libc::c_int; // [units -> digits]
                                if (*res).digits < 0 as libc::c_int {
                                    // borrowed...
                                    (*res).digits = -(*res).digits;
                                    (*res).bits =
                                        ((*res).bits as libc::c_int ^
                                             0x80 as libc::c_int) as uint8_t
                                    // flip the sign
                                }
                                // If a buffer was used the result must be copied back, possibly
    // shortening.  (If no buffer was used then the result must have
    // fit, so can't need rounding and residue must be 0.)
                                residue =
                                    0 as libc::c_int; // clear accumulator
                                if acc != (*res).lsu.as_mut_ptr()
                                   { // used buffer
                                    // remove leading zeros that were added due to rounding up to
        // integral Units -- before the test for rounding.
                                    if (*res).digits > reqdigits {
                                        (*res).digits =
                                            decGetDigits(acc,
                                                         if (*res).digits <=
                                                                49 as
                                                                    libc::c_int
                                                            {
                                                             d2utable[(*res).digits
                                                                          as
                                                                          usize]
                                                                 as
                                                                 libc::c_int
                                                         } else {
                                                             ((*res).digits +
                                                                  3 as
                                                                      libc::c_int
                                                                  -
                                                                  1 as
                                                                      libc::c_int)
                                                                 /
                                                                 3 as
                                                                     libc::c_int
                                                         })
                                    }
                                    decSetCoeff(res, set, acc, (*res).digits,
                                                &mut residue, status);
                                }
                                // strip leading zeros [these were left on in case of subset subtract]
                                (*res).digits =
                                    decGetDigits((*res).lsu.as_mut_ptr(),
                                                 if (*res).digits <=
                                                        49 as libc::c_int {
                                                     d2utable[(*res).digits as
                                                                  usize] as
                                                         libc::c_int
                                                 } else {
                                                     ((*res).digits +
                                                          3 as libc::c_int -
                                                          1 as libc::c_int) /
                                                         3 as libc::c_int
                                                 });
                                // apply checks and rounding
                                decFinalize(res, set, &mut residue, status);
                                // "When the sum of two operands with opposite signs is exactly
    // zero, the sign of that sum shall be '+' in all rounding modes
    // except round toward -Infinity, in which mode that sign shall be
    // '-'."  [Subset zeros also never have '-', set by decFinish.]
                                if *(*res).lsu.as_mut_ptr() as libc::c_int ==
                                       0 as libc::c_int &&
                                       (*res).digits == 1 as libc::c_int &&
                                       (*res).bits as libc::c_int &
                                           (0x40 as libc::c_int |
                                                0x20 as libc::c_int |
                                                0x10 as libc::c_int) ==
                                           0 as libc::c_int &&
                                       diffsign as libc::c_int != 0 &&
                                       *status &
                                           0x20 as libc::c_int as libc::c_uint
                                           == 0 as libc::c_int as libc::c_uint
                                   {
                                    if (*set).round as libc::c_uint ==
                                           DEC_ROUND_FLOOR as libc::c_int as
                                               libc::c_uint {
                                        (*res).bits =
                                            ((*res).bits as libc::c_int |
                                                 0x80 as libc::c_int) as
                                                uint8_t
                                    } else {
                                        (*res).bits =
                                            ((*res).bits as libc::c_int &
                                                 !(0x80 as libc::c_int)) as
                                                uint8_t
                                    } // sign -
                                    // sign +
                                }
                            }
                        }
                    }
                }
            }
        }
    } // drop any storage used
    if !allocacc.is_null() { free(allocacc as *mut libc::c_void); }
    return res;
}
// decAddOp
/* ------------------------------------------------------------------ */
/* decDivideOp -- division operation                                  */
/*                                                                    */
/*  This routine performs the calculations for all four division      */
/*  operators (divide, divideInteger, remainder, remainderNear).      */
/*                                                                    */
/*  C=A op B                                                          */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X/X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*   op  is DIVIDE, DIVIDEINT, REMAINDER, or REMNEAR respectively.    */
/*   status is the usual accumulator                                  */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* ------------------------------------------------------------------ */
/*   The underlying algorithm of this routine is the same as in the   */
/*   1981 S/370 implementation, that is, non-restoring long division  */
/*   with bi-unit (rather than bi-digit) estimation for each unit     */
/*   multiplier.  In this pseudocode overview, complications for the  */
/*   Remainder operators and division residues for exact rounding are */
/*   omitted for clarity.                                             */
/*                                                                    */
/*     Prepare operands and handle special values                     */
/*     Test for x/0 and then 0/x                                      */
/*     Exp =Exp1 - Exp2                                               */
/*     Exp =Exp +len(var1) -len(var2)                                 */
/*     Sign=Sign1 * Sign2                                             */
/*     Pad accumulator (Var1) to double-length with 0's (pad1)        */
/*     Pad Var2 to same length as Var1                                */
/*     msu2pair/plus=1st 2 or 1 units of var2, +1 to allow for round  */
/*     have=0                                                         */
/*     Do until (have=digits+1 OR residue=0)                          */
/*       if exp<0 then if integer divide/residue then leave           */
/*       this_unit=0                                                  */
/*       Do forever                                                   */
/*          compare numbers                                           */
/*          if <0 then leave inner_loop                               */
/*          if =0 then (* quick exit without subtract *) do           */
/*             this_unit=this_unit+1; output this_unit                */
/*             leave outer_loop; end                                  */
/*          Compare lengths of numbers (mantissae):                   */
/*          If same then tops2=msu2pair -- {units 1&2 of var2}        */
/*                  else tops2=msu2plus -- {0, unit 1 of var2}        */
/*          tops1=first_unit_of_Var1*10**DECDPUN +second_unit_of_var1 */
/*          mult=tops1/tops2  -- Good and safe guess at divisor       */
/*          if mult=0 then mult=1                                     */
/*          this_unit=this_unit+mult                                  */
/*          subtract                                                  */
/*          end inner_loop                                            */
/*        if have\=0 | this_unit\=0 then do                           */
/*          output this_unit                                          */
/*          have=have+1; end                                          */
/*        var2=var2/10                                                */
/*        exp=exp-1                                                   */
/*        end outer_loop                                              */
/*     exp=exp+1   -- set the proper exponent                         */
/*     if have=0 then generate answer=0                               */
/*     Return (Result is defined by Var1)                             */
/*                                                                    */
/* ------------------------------------------------------------------ */
/* Two working buffers are needed during the division; one (digits+   */
/* 1) to accumulate the result, and the other (up to 2*digits+1) for  */
/* long subtractions.  These are acc and var1 respectively.           */
/* var1 is a copy of the lhs coefficient, var2 is the rhs coefficient.*/
/* The static buffers may be larger than might be expected to allow   */
/* for calls from higher-level funtions (notable exp).                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decDivideOp(mut res: *mut decNumber,
                                 mut lhs: *const decNumber,
                                 mut rhs: *const decNumber,
                                 mut set: *mut decContext, mut op: uint8_t,
                                 mut status: *mut uint32_t)
 -> *mut decNumber {
    let mut accbuff: [uint16_t; 17] = [0; 17]; // local buffer
    let mut acc: *mut uint16_t =
        accbuff.as_mut_ptr(); // -> accumulator array for result
    let mut allocacc: *mut uint16_t =
        0 as *mut uint16_t; // -> allocated buffer, iff allocated
    let mut accnext: *mut uint16_t =
        0 as *mut uint16_t; // -> where next digit will go
    let mut acclength: int32_t = 0; // length of acc needed [Units]
    let mut accunits: int32_t = 0; // count of units accumulated
    let mut accdigits: int32_t = 0; // count of digits accumulated
    let mut varbuff: [uint16_t; 25] = [0; 25]; // buffer for var1
    let mut var1: *mut uint16_t =
        varbuff.as_mut_ptr(); // -> var1 array for long subtraction
    let mut varalloc: *mut uint16_t =
        0 as *mut uint16_t; // -> allocated buffer, iff used
    let mut msu1: *mut uint16_t = 0 as *mut uint16_t; // -> msu of var1
    let mut var2: *const uint16_t = 0 as *const uint16_t; // -> var2 array
    let mut msu2: *const uint16_t = 0 as *const uint16_t; // -> msu of var2
    let mut msu2plus: int32_t = 0; // msu2 plus one [does not vary]
    let mut msu2pair: int32_t = 0; // msu2 pair plus one [does not vary]
    let mut var1units: int32_t = 0; // actual lengths
    let mut var2units: int32_t = 0; // logical length (units)
    let mut var2ulen: int32_t = 0; // var1 initial padding (digits)
    let mut var1initpad: int32_t =
        0 as libc::c_int; // longest LHS or required acc length
    let mut maxdigits: int32_t = 0; // multiplier for subtraction
    let mut mult: int32_t = 0; // current unit being accumulated
    let mut thisunit: uint16_t = 0; // for rounding
    let mut residue: int32_t = 0; // requested DIGITS
    let mut reqdigits: int32_t = (*set).digits; // working exponent
    let mut exponent: int32_t = 0; // DIVIDE maximum exponent if unrounded
    let mut maxexponent: int32_t = 0 as libc::c_int; // working sign
    let mut bits: uint8_t = 0; // work
    let mut target: *mut uint16_t = 0 as *mut uint16_t; // ..
    let mut source: *const uint16_t = 0 as *const uint16_t; // ..
    let mut pow: *const uint32_t = 0 as *const uint32_t; // ..
    let mut shift: int32_t = 0; // end protected
    let mut cut: int32_t = 0;
    let mut current_block_213: u64;
    // protect allocated storage
    // [following code does not require input rounding]
    bits =
        (((*lhs).bits as libc::c_int ^ (*rhs).bits as libc::c_int) &
             0x80 as libc::c_int) as uint8_t; // assumed sign for divisions
    // handle infinities and NaNs
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // a special bit set
        if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) &
               (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
            // one or two NaNs
            decNaNs(res, lhs, rhs, set, status);
        } else if (*lhs).bits as libc::c_int & 0x40 as libc::c_int !=
                      0 as libc::c_int {
            // one or two infinities
            // LHS (dividend) is infinite
            if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
                   0 as libc::c_int ||
                   op as libc::c_int &
                       (0x40 as libc::c_int | 0x10 as libc::c_int) != 0 {
                // as is remainder of infinity
                *status |= 0x80 as libc::c_int as libc::c_uint
            } else {
                // [Note that infinity/0 raises no exceptions]
                decNumberZero(res); // set +/- infinity
                (*res).bits =
                    (bits as libc::c_int | 0x40 as libc::c_int) as uint8_t
            }
        } else {
            // RHS (divisor) is infinite
            residue = 0 as libc::c_int;
            if op as libc::c_int & (0x40 as libc::c_int | 0x10 as libc::c_int)
                   != 0 {
                // result is [finished clone of] lhs
                decCopyFit(res, lhs, set, &mut residue, status);
            } else {
                // a division
                decNumberZero(res); // set +/- zero
                (*res).bits = bits;
                // for DIVIDEINT the exponent is always 0.  For DIVIDE, result
          // is a 0 with infinitely negative exponent, clamped to minimum
                if op as libc::c_int & 0x80 as libc::c_int != 0 {
                    (*res).exponent =
                        (*set).emin - (*set).digits + 1 as libc::c_int;
                    *status |= 0x400 as libc::c_int as libc::c_uint
                }
            }
            decFinalize(res, set, &mut residue, status);
        }
    } else if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*rhs).digits == 1 as libc::c_int &&
                  (*rhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        // handle 0 rhs (x/0)
        // x/0 is always exceptional
        if *(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
               (*lhs).digits == 1 as libc::c_int &&
               (*lhs).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) == 0 as libc::c_int {
            decNumberZero(res);
            *status |= 0x8 as libc::c_int as libc::c_uint // [after lhs test]
            // 0/0 will become NaN
        } else {
            decNumberZero(res); // x/0
            if op as libc::c_int & (0x40 as libc::c_int | 0x10 as libc::c_int)
                   != 0 {
                *status |= 0x80 as libc::c_int as libc::c_uint
            } else {
                *status |= 0x2 as libc::c_int as libc::c_uint;
                (*res).bits =
                    (bits as libc::c_int | 0x40 as libc::c_int) as uint8_t
                // .. is +/- Infinity
            }
        }
    } else if *(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*lhs).digits == 1 as libc::c_int &&
                  (*lhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        // handle 0 lhs (0/x)
        // 0/x [x!=0]
        if op as libc::c_int & 0x80 as libc::c_int != 0 {
            residue = 0 as libc::c_int;
            // check exponent
            exponent = (*lhs).exponent - (*rhs).exponent; // ideal exponent
            decNumberCopy(res, lhs); // [zeros always fit]
            (*res).bits = bits; // sign as computed
            (*res).exponent = exponent; // exponent, too
            decFinalize(res, set, &mut residue, status); // integer 0
        } else if op as libc::c_int & 0x20 as libc::c_int != 0 {
            decNumberZero(res);
            (*res).bits = bits
            // sign as computed
        } else {
            // a remainder
            exponent = (*rhs).exponent; // [save in case overwrite]
            // use lower
            decNumberCopy(res, lhs); // [zeros always fit]
            if exponent < (*res).exponent { (*res).exponent = exponent }
        }
    } else {
        // Precalculate exponent.  This starts off adjusted (and hence fits
    // in 31 bits) and becomes the usual unadjusted exponent as the
    // division proceeds.  The order of evaluation is important, here,
    // to avoid wrap.
        exponent =
            (*lhs).exponent + (*lhs).digits -
                ((*rhs).exponent + (*rhs).digits);
        // If the working exponent is -ve, then some quick exits are
    // possible because the quotient is known to be <1
    // [for REMNEAR, it needs to be < -1, as -0.5 could need work]
        if exponent < 0 as libc::c_int &&
               !(op as libc::c_int == 0x80 as libc::c_int) {
            if op as libc::c_int & 0x20 as libc::c_int != 0 { // fastpaths
                decNumberZero(res); // integer part is 0
                (*res).bits = bits; // set +/- zero
                current_block_213 = 5213335239096154175;
            } else if (*lhs).exponent <= (*rhs).exponent {
                if op as libc::c_int & 0x40 as libc::c_int != 0 ||
                       exponent < -(1 as libc::c_int) {
                    // fastpath remainders so long as the lhs has the smaller
      // (or equal) exponent
                    // It is REMAINDER or safe REMNEAR; result is [finished
          // clone of] lhs  (r = x - 0*y)
                    residue = 0 as libc::c_int;
                    decCopyFit(res, lhs, set, &mut residue, status);
                    decFinalize(res, set, &mut residue, status);
                    current_block_213 = 5213335239096154175;
                } else { current_block_213 = 17019156190352891614; }
                // [unsafe REMNEAR drops through]
            } else { current_block_213 = 17019156190352891614; }
        } else { current_block_213 = 17019156190352891614; }
        match current_block_213 {
            5213335239096154175 => { }
            _ => {
                /* Long (slow) division is needed; roll up the sleeves... */
                // The accumulator will hold the quotient of the division.
    // If it needs to be too long for stack storage, then allocate.
                acclength =
                    if reqdigits + 3 as libc::c_int <= 49 as libc::c_int {
                        d2utable[(reqdigits + 3 as libc::c_int) as usize] as
                            libc::c_int
                    } else {
                        (reqdigits + 3 as libc::c_int + 3 as libc::c_int -
                             1 as libc::c_int) / 3 as libc::c_int
                    }; // in Units
                if (acclength as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                        as libc::c_ulong) >
                       ::std::mem::size_of::<[uint16_t; 17]>() as
                           libc::c_ulong {
                    // printf("malloc dvacc %ld units\n", acclength);
                    allocacc =
                        malloc((acclength as
                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                    as
                                                                    libc::c_ulong))
                            as *mut uint16_t;
                    if allocacc.is_null() {
                        // use the allocated space
                        // hopeless -- abandon
                        *status |= 0x10 as libc::c_int as libc::c_uint;
                        current_block_213 = 5213335239096154175;
                    } else {
                        acc = allocacc;
                        current_block_213 = 8102658916883067714;
                    }
                } else { current_block_213 = 8102658916883067714; }
                match current_block_213 {
                    5213335239096154175 => { }
                    _ => {
                        // var1 is the padded LHS ready for subtractions.
    // If it needs to be too long for stack storage, then allocate.
    // The maximum units needed for var1 (long subtraction) is:
    // Enough for
    //     (rhs->digits+reqdigits-1) -- to allow full slide to right
    // or  (lhs->digits)             -- to allow for long lhs
    // whichever is larger
    //   +1                -- for rounding of slide to right
    //   +1                -- for leading 0s
    //   +1                -- for pre-adjust if a remainder or DIVIDEINT
    // [Note: unused units do not participate in decUnitAddSub data]
                        maxdigits =
                            (*rhs).digits + reqdigits - 1 as libc::c_int;
                        if (*lhs).digits > maxdigits {
                            maxdigits = (*lhs).digits
                        }
                        var1units =
                            (if maxdigits <= 49 as libc::c_int {
                                 d2utable[maxdigits as usize] as libc::c_int
                             } else {
                                 (maxdigits + 3 as libc::c_int -
                                      1 as libc::c_int) / 3 as libc::c_int
                             }) + 2 as libc::c_int;
                        // allocate a guard unit above msu1 for REMAINDERNEAR
                        if op as libc::c_int & 0x80 as libc::c_int == 0 {
                            var1units += 1
                        }
                        if ((var1units + 1 as libc::c_int) as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                as
                                                                libc::c_ulong)
                               >
                               ::std::mem::size_of::<[uint16_t; 25]>() as
                                   libc::c_ulong {
                            // printf("malloc dvvar %ld units\n", var1units+1);
                            varalloc =
                                malloc(((var1units + 1 as libc::c_int) as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                            as
                                                                            libc::c_ulong))
                                    as *mut uint16_t;
                            if varalloc.is_null() {
                                // use the allocated space
                                // hopeless -- abandon
                                *status |=
                                    0x10 as libc::c_int as libc::c_uint;
                                current_block_213 = 5213335239096154175;
                            } else {
                                var1 = varalloc;
                                current_block_213 = 4488496028633655612;
                            }
                        } else { current_block_213 = 4488496028633655612; }
                        match current_block_213 {
                            5213335239096154175 => { }
                            _ => {
                                // Extend the lhs and rhs to full long subtraction length.  The lhs
    // is truly extended into the var1 buffer, with 0 padding, so a
    // subtract in place is always possible.  The rhs (var2) has
    // virtual padding (implemented by decUnitAddSub).
    // One guard unit was allocated above msu1 for rem=rem+rem in
    // REMAINDERNEAR.
                                msu1 =
                                    var1.offset(var1units as
                                                    isize).offset(-(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)); // msu of var1
                                source =
                                    (*lhs).lsu.as_ptr().offset((if (*lhs).digits
                                                                       <=
                                                                       49 as
                                                                           libc::c_int
                                                                   {
                                                                    d2utable[(*lhs).digits
                                                                                 as
                                                                                 usize]
                                                                        as
                                                                        libc::c_int
                                                                } else {
                                                                    ((*lhs).digits
                                                                         +
                                                                         3 as
                                                                             libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        /
                                                                        3 as
                                                                            libc::c_int
                                                                }) as
                                                                   isize).offset(-(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize)); // msu of input array
                                target = msu1;
                                while source >= (*lhs).lsu.as_ptr() {
                                    *target = *source;
                                    source = source.offset(-1);
                                    target = target.offset(-1)
                                }
                                while target >= var1 {
                                    *target = 0 as libc::c_int as uint16_t;
                                    target = target.offset(-1)
                                }
                                // rhs (var2) is left-aligned with var1 at the start
                                var2ulen =
                                    var1units; // rhs logical length (units)
                                var2units =
                                    if (*rhs).digits <= 49 as libc::c_int {
                                        d2utable[(*rhs).digits as usize] as
                                            libc::c_int
                                    } else {
                                        ((*rhs).digits + 3 as libc::c_int -
                                             1 as libc::c_int) /
                                            3 as libc::c_int
                                    }; // rhs actual length (units)
                                var2 = (*rhs).lsu.as_ptr(); // -> rhs array
                                msu2 =
                                    var2.offset(var2units as
                                                    isize).offset(-(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)); // -> msu of var2 [never changes]
                                // now set up the variables which will be used for estimating the
    // multiplication factor.  If these variables are not exact, add
    // 1 to make sure that the multiplier is never overestimated.
                                msu2plus = *msu2 as int32_t; // it's value ..
                                if var2units > 1 as libc::c_int {
                                    msu2plus += 1
                                } // .. +1 if any more
                                msu2pair =
                                    *msu2 as int32_t *
                                        (999 as libc::c_int +
                                             1 as
                                                 libc::c_int); // top two pair ..
                                if var2units > 1 as libc::c_int {
                                    // .. [else treat 2nd as 0]
                                    msu2pair +=
                                        *msu2.offset(-(1 as libc::c_int as
                                                           isize)) as
                                            libc::c_int;
                                    if var2units > 2 as libc::c_int {
                                        msu2pair += 1
                                    } // ..
                                    // .. +1 if any more
                                }
                                // The calculation is working in units, which may have leading zeros,
    // but the exponent was calculated on the assumption that they are
    // both left-aligned.  Adjust the exponent to compensate: add the
    // number of leading zeros in var1 msu and subtract those in var2 msu.
    // [This is actually done by counting the digits and negating, as
    // lead1=DECDPUN-digits1, and similarly for lead2.]
                                pow =
                                    &*DECPOWERS.as_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                        as *const uint32_t;
                                while *msu1 as libc::c_uint >= *pow {
                                    exponent -= 1;
                                    pow = pow.offset(1)
                                }
                                pow =
                                    &*DECPOWERS.as_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                                        as *const uint32_t;
                                while *msu2 as libc::c_uint >= *pow {
                                    exponent += 1;
                                    pow = pow.offset(1)
                                }
                                // Now, if doing an integer divide or remainder, ensure that
    // the result will be Unit-aligned.  To do this, shift the var1
    // accumulator towards least if need be.  (It's much easier to
    // do this now than to reassemble the residue afterwards, if
    // doing a remainder.)  Also ensure the exponent is not negative.
                                if op as libc::c_int & 0x80 as libc::c_int ==
                                       0 {
                                    let mut u: *mut uint16_t =
                                        0 as *mut uint16_t; // align
                                    var1initpad =
                                        (var1units -
                                             (if (*lhs).digits <=
                                                     49 as libc::c_int {
                                                  d2utable[(*lhs).digits as
                                                               usize] as
                                                      libc::c_int
                                              } else {
                                                  ((*lhs).digits +
                                                       3 as libc::c_int -
                                                       1 as libc::c_int) /
                                                      3 as libc::c_int
                                              })) * 3 as libc::c_int;
                                    if exponent < 0 as libc::c_int {
                                        cut = -exponent
                                    } else {
                                        cut =
                                            3 as libc::c_int -
                                                exponent % 3 as libc::c_int
                                    }
                                    decShiftToLeast(var1, var1units,
                                                    cut); // work
                                    exponent += cut;
                                    var1initpad -= cut;
                                    u = msu1;
                                    while cut >= 3 as libc::c_int {
                                        *u = 0 as libc::c_int as uint16_t;
                                        cut -= 3 as libc::c_int;
                                        u = u.offset(-1)
                                    }
                                } else {
                                    // save the initial 'false' padding of var1, in digits
                                    // Determine the shift to do.
                                    // maintain numerical value
                                    // .. and reduce padding
                                    // clean any most-significant units which were just emptied
                                    // is DIVIDE
                                    maxexponent =
                                        (*lhs).exponent -
                                            (*rhs).exponent; // save
                                    // optimization: if the first iteration will just produce 0,
      // preadjust to skip it [valid for DIVIDE only]
                                    if (*msu1 as libc::c_int) <
                                           *msu2 as libc::c_int {
                                        var2ulen -= 1;
                                        exponent -=
                                            3 as libc::c_int // shift down
                                        // update the exponent
                                    }
                                }
                                // ---- start the long-division loops ------------------------------
                                accunits =
                                    0 as
                                        libc::c_int; // no units accumulated yet
                                accdigits = 0 as libc::c_int; // .. or digits
                                accnext =
                                    acc.offset(acclength as
                                                   isize).offset(-(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)); // -> msu of acc [NB: allows digits+1]
                                loop  {
                                    thisunit =
                                        0 as libc::c_int as
                                            uint16_t; // outer loop
                                    loop 
                                         // outer forever loop
                                         // current unit assumed 0
                                         // find the next unit
                                         {
                                        while *msu1 as libc::c_int ==
                                                  0 as libc::c_int &&
                                                  msu1 > var1 {
                                            var1units -= 1; // inner loop
                                            msu1 = msu1.offset(-1)
                                        }
                                        // inner forever loop
                                        // strip leading zero units [from either pre-adjust or from
        // subtract last time around].  Leave at least one unit.
                                        if var1units < var2ulen {
                                            break
                                                ; // var1 too low for subtract
                                        } // lengths the same
                                        if var1units == var2ulen {
                                            let mut pv1: *const uint16_t =
                                                0 as *const uint16_t;
                                            let mut pv2: *const uint16_t =
                                                0 as *const uint16_t;
                                            // unit-by-unit compare needed
                                            // compare the two numbers, from msu
                                            let mut v2: uint16_t =
                                                0; // units to compare
                                            pv2 = msu2; // -> msu
                                            pv1 = msu1;
                                            loop  {
                                                // v1=*pv1 -- always OK
                                                v2 =
                                                    0 as libc::c_int as
                                                        uint16_t; // assume in padding
                                                // done; leave pv1 as is
                                                if pv2 >= var2 {
                                                    v2 = *pv2
                                                } // in range
                                                if *pv1 as libc::c_int !=
                                                       v2 as libc::c_int {
                                                    break
                                                        ; // no longer the same
                                                }
                                                if pv1 == var1 { break ; }
                                                pv1 = pv1.offset(-1);
                                                pv2 = pv2.offset(-1)
                                            }
                                            // here when all inspected or a difference seen
                                            if (*pv1 as libc::c_int) <
                                                   v2 as libc::c_int {
                                                break
                                                    ; // var1 too low to subtract
                                            } // var1 == var2
                                            if *pv1 as libc::c_int ==
                                                   v2 as libc::c_int {
                                                // var1 == var2
                                                // reach here if var1 and var2 are identical; subtraction
            // would increase digit by one, and the residue will be 0 so
            // the calculation is done; leave the loop with residue=0.
                                                thisunit =
                                                    thisunit.wrapping_add(1); // as though subtracted
                                                // from inner
                                                *var1 =
                                                    0 as libc::c_int as
                                                        uint16_t; // set var1 to 0
                                                var1units =
                                                    1 as libc::c_int; // ..
                                                break ;
                                            } else {
                                                // *pv1>v2.  Prepare for real subtraction; the lengths are equal
          // Estimate the multiplier (there's always a msu1-1)...
          // Bring in two units of var2 to provide a good estimate.
                                                mult =
                                                    (*msu1 as int32_t *
                                                         (999 as libc::c_int +
                                                              1 as
                                                                  libc::c_int)
                                                         +
                                                         *msu1.offset(-(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize))
                                                             as libc::c_int) /
                                                        msu2pair
                                            }
                                        } else {
                                            // var1units > var2ulen, so subtraction is safe
                                            // The var2 msu is one unit towards the lsu of the var1 msu,
          // so only one unit for var2 can be used.
                                            mult =
                                                (*msu1 as int32_t *
                                                     (999 as libc::c_int +
                                                          1 as libc::c_int) +
                                                     *msu1.offset(-(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize))
                                                         as libc::c_int) /
                                                    msu2plus
                                        } // must always be at least 1
                                        if mult == 0 as libc::c_int {
                                            mult = 1 as libc::c_int
                                        }
                                        // subtraction needed; var1 is > var2
                                        thisunit =
                                            (thisunit as libc::c_int + mult)
                                                as uint16_t; // accumulate
                                        // subtract var1-var2, into var1; only the overlap needs
        // processing, as this is an in-place calculation
                                        shift = var2ulen - var2units;
                                        decUnitAddSub(&mut *var1.offset(shift
                                                                            as
                                                                            isize),
                                                      var1units - shift, var2,
                                                      var2units,
                                                      0 as libc::c_int,
                                                      &mut *var1.offset(shift
                                                                            as
                                                                            isize),
                                                      -mult);
                                    }
                                    // The next unit has been calculated in full; unless it's a
      // leading zero, add to acc
                                    if accunits != 0 as libc::c_int ||
                                           thisunit as libc::c_int !=
                                               0 as libc::c_int {
                                        // is first or non-zero
                                        *accnext =
                                            thisunit; // store in accumulator
                                        // have enough digits
                                        // account exactly for the new digits
                                        if accunits == 0 as libc::c_int {
                                            accdigits += 1; // at least one
                                            pow =
                                                &*DECPOWERS.as_ptr().offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                    as
                                                    *const uint32_t; // update count
                                            while thisunit as libc::c_uint >=
                                                      *pow {
                                                accdigits +=
                                                    1; // ready for next
                                                pow = pow.offset(1)
                                            }
                                        } else {
                                            accdigits += 3 as libc::c_int
                                        }
                                        accunits += 1;
                                        accnext = accnext.offset(-1);
                                        if accdigits > reqdigits { break ; }
                                    }
                                    // if the residue is zero, the operation is done (unless divide
      // or divideInteger and still not enough digits yet)
                                    if *var1 as libc::c_int ==
                                           0 as libc::c_int &&
                                           var1units == 1 as libc::c_int {
                                        // residue is 0
                                        if op as libc::c_int &
                                               (0x40 as libc::c_int |
                                                    0x10 as libc::c_int) != 0
                                           {
                                            break ;
                                        }
                                        if op as libc::c_int &
                                               0x80 as libc::c_int != 0 &&
                                               exponent <= maxexponent {
                                            break ;
                                        }
                                        // [drop through if divideInteger]
                                    }
                                    // also done enough if calculating remainder or integer
      // divide and just did the last ('units') unit
                                    if exponent == 0 as libc::c_int &&
                                           op as libc::c_int &
                                               0x80 as libc::c_int == 0 {
                                        break ;
                                    }
                                    // to get here, var1 is less than var2, so divide var2 by the per-
      // Unit power of ten and go for the next digit
                                    var2ulen -= 1; // shift down
                                    exponent -= 3 as libc::c_int
                                }
                                // ---- division is complete ---------------------------------------
    // here: acc      has at least reqdigits+1 of good results (or fewer
    //                if early stop), starting at accnext+1 (its lsu)
    //       var1     has any residue at the stopping point
    //       accunits is the number of digits collected in acc
                                if accunits == 0 as libc::c_int
                                   { // back to last placed
                                    // acc is 0
                                    accunits =
                                        1 as
                                            libc::c_int; // show have a unit ..
                                    // .. whose value is 0
                                    accdigits = 1 as libc::c_int; // ..
                                    *accnext = 0 as libc::c_int as uint16_t
                                } else { accnext = accnext.offset(1) }
                                // accnext now -> lowest unit of result
                                residue =
                                    0 as libc::c_int; // assume no residue
                                if op as libc::c_int & 0x80 as libc::c_int !=
                                       0 { // not DIVIDE
                                    // record the presence of any residue, for rounding
                                    if *var1 as libc::c_int !=
                                           0 as libc::c_int ||
                                           var1units > 1 as libc::c_int {
                                        residue = 1 as libc::c_int
                                    } else {
                                        // no residue
                                        // Had an exact division; clean up spurious trailing 0s.
        // There will be at most DECDPUN-1, from the final multiply,
        // and then only if the result is non-0 (and even) and the
        // exponent is 'loose'.
                                        let mut lsu: uint16_t = *accnext;
                                        if lsu as libc::c_int &
                                               0x1 as libc::c_int == 0 &&
                                               lsu as libc::c_int !=
                                                   0 as libc::c_int {
                                            // count the trailing zeros
                                            let mut drop_0: int32_t =
                                                0 as libc::c_int;
                                            // [will terminate because lsu!=0]
                                            while !(exponent >= maxexponent)
                                                  { // don't chop real 0s
                                                if (lsu as
                                                        libc::c_uint).wrapping_sub(((lsu
                                                                                         as
                                                                                         uint32_t
                                                                                         >>
                                                                                         drop_0
                                                                                             +
                                                                                             1
                                                                                                 as
                                                                                                 libc::c_int).wrapping_mul(multies[(drop_0
                                                                                                                                        +
                                                                                                                                        1
                                                                                                                                            as
                                                                                                                                            libc::c_int)
                                                                                                                                       as
                                                                                                                                       usize])
                                                                                        >>
                                                                                        17
                                                                                            as
                                                                                            libc::c_int).wrapping_mul(DECPOWERS[(drop_0
                                                                                                                                     +
                                                                                                                                     1
                                                                                                                                         as
                                                                                                                                         libc::c_int)
                                                                                                                                    as
                                                                                                                                    usize]))
                                                       !=
                                                       0 as libc::c_int as
                                                           libc::c_uint {
                                                    break
                                                        ; // found non-0 digit
                                                }
                                                exponent += 1;
                                                drop_0 += 1
                                            }
                                            if drop_0 > 0 as libc::c_int {
                                                accunits =
                                                    decShiftToLeast(accnext,
                                                                    accunits,
                                                                    drop_0);
                                                accdigits =
                                                    decGetDigits(accnext,
                                                                 accunits);
                                                accunits =
                                                    if accdigits <=
                                                           49 as libc::c_int {
                                                        d2utable[accdigits as
                                                                     usize] as
                                                            libc::c_int
                                                    } else {
                                                        (accdigits +
                                                             3 as libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                            / 3 as libc::c_int
                                                    }
                                                // [exponent was adjusted in the loop]
                                            }
                                        }
                                    }
                                    current_block_213 = 11911614146017124710;
                                    // exact divide
                                } else if accdigits + exponent > reqdigits {
                                    *status |=
                                        0x4 as libc::c_int as libc::c_uint;
                                    current_block_213 = 5213335239096154175;
                                } else if op as libc::c_int &
                                              (0x40 as libc::c_int |
                                                   0x10 as libc::c_int) != 0 {
                                    /* op!=DIVIDE */
                                    // check for coefficient overflow
                                    // [Here, the exponent will be 0, because var1 was adjusted
        // appropriately.]
                                    let mut postshift: int32_t = 0; // work
                                    let mut wasodd: uint8_t =
                                        0 as libc::c_int as
                                            uint8_t; // integer was odd
                                    let mut quotlsu: *mut uint16_t =
                                        0 as *mut uint16_t; // for save
                                    let mut quotdigits: int32_t = 0; // ..
                                    bits =
                                        (*lhs).bits; // remainder sign is always as lhs
                                    // Fastpath when residue is truly 0 is worthwhile [and
        // simplifies the code below]
                                    if *var1 as libc::c_int ==
                                           0 as libc::c_int &&
                                           var1units == 1 as libc::c_int {
                                        // residue is 0
                                        let mut exp: int32_t =
                                            (*lhs).exponent; // save min(exponents)
                                        if (*rhs).exponent < exp {
                                            exp = (*rhs).exponent
                                        } // 0 coefficient
                                        decNumberZero(res); // .. with proper exponent
                                        (*res).exponent = exp; // [cleaned]
                                        (*res).bits =
                                            (bits as libc::c_int &
                                                 0x80 as libc::c_int) as
                                                uint8_t; // might clamp
                                        decFinalize(res, set, &mut residue,
                                                    status);
                                        current_block_213 =
                                            5213335239096154175;
                                    } else {
                                        // note if the quotient was odd
                                        if *accnext as libc::c_int &
                                               0x1 as libc::c_int != 0 {
                                            wasodd =
                                                1 as libc::c_int as uint8_t
                                        } // acc is odd
                                        quotlsu =
                                            accnext; // save in case need to reinspect
                                        quotdigits = accdigits; // ..
                                        // treat the residue, in var1, as the value to return, via acc
        // calculate the unused zero digits.  This is the smaller of:
        //   var1 initial padding (saved above)
        //   var2 residual padding, which happens to be given by:
                                        postshift =
                                            var1initpad + exponent -
                                                (*lhs).exponent +
                                                (*rhs).exponent;
                                        // [the 'exponent' term accounts for the shifts during divide]
                                        if var1initpad < postshift {
                                            postshift = var1initpad
                                        }
                                        // shift var1 the requested amount, and adjust its digits
                                        var1units =
                                            decShiftToLeast(var1, var1units,
                                                            postshift); // exponent is smaller of lhs & rhs
                                        accnext = var1;
                                        accdigits =
                                            decGetDigits(var1, var1units);
                                        accunits =
                                            if accdigits <= 49 as libc::c_int
                                               {
                                                d2utable[accdigits as usize]
                                                    as libc::c_int
                                            } else {
                                                (accdigits + 3 as libc::c_int
                                                     - 1 as libc::c_int) /
                                                    3 as libc::c_int
                                            };
                                        exponent = (*lhs).exponent;
                                        if (*rhs).exponent < exponent {
                                            exponent = (*rhs).exponent
                                        }
                                        // Now correct the result if doing remainderNear; if it
        // (looking just at coefficients) is > rhs/2, or == rhs/2 and
        // the integer was odd then the result should be rem-rhs.
                                        if op as libc::c_int &
                                               0x10 as libc::c_int != 0 {
                                            let mut compare: int32_t =
                                                0; // work
                                            let mut tarunits: int32_t =
                                                0; // ..
                                            let mut up: *mut uint16_t =
                                                0 as *mut uint16_t;
                                            // calculate remainder*2 into the var1 buffer (which has
          // 'headroom' of an extra unit and hence enough space)
          // [a dedicated 'double' loop would be faster, here]
                                            tarunits =
                                                decUnitAddSub(accnext,
                                                              accunits,
                                                              accnext,
                                                              accunits,
                                                              0 as
                                                                  libc::c_int,
                                                              accnext,
                                                              1 as
                                                                  libc::c_int);
                                            // decDumpAr('r', accnext, tarunits);
                                            // Here, accnext (var1) holds tarunits Units with twice the
          // remainder's coefficient, which must now be compared to the
          // RHS.  The remainder's exponent may be smaller than the RHS's.
                                            compare =
                                                decUnitCompare(accnext,
                                                               tarunits,
                                                               (*rhs).lsu.as_ptr(),
                                                               if (*rhs).digits
                                                                      <=
                                                                      49 as
                                                                          libc::c_int
                                                                  {
                                                                   d2utable[(*rhs).digits
                                                                                as
                                                                                usize]
                                                                       as
                                                                       libc::c_int
                                                               } else {
                                                                   ((*rhs).digits
                                                                        +
                                                                        3 as
                                                                            libc::c_int
                                                                        -
                                                                        1 as
                                                                            libc::c_int)
                                                                       /
                                                                       3 as
                                                                           libc::c_int
                                                               },
                                                               (*rhs).exponent
                                                                   -
                                                                   exponent);
                                            if compare ==
                                                   0x80000000 as libc::c_uint
                                                       as int32_t {
                                                // deep trouble
                                                *status |=
                                                    0x10 as libc::c_int as
                                                        libc::c_uint;
                                                current_block_213 =
                                                    5213335239096154175;
                                            } else {
                                                // now restore the remainder by dividing by two; the lsu
          // is known to be even.
                                                up =
                                                    accnext; // half to add to lower unit
                                                while up <
                                                          accnext.offset(tarunits
                                                                             as
                                                                             isize)
                                                      {
                                                    let mut half: int32_t =
                                                        0; // [shift]
                                                    half =
                                                        *up as libc::c_int &
                                                            0x1 as
                                                                libc::c_int;
                                                    *up =
                                                        (*up as libc::c_int /
                                                             2 as libc::c_int)
                                                            as uint16_t;
                                                    if !(half == 0) {
                                                        let ref mut fresh0 =
                                                            *up.offset(-(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize));
                                                        *fresh0 =
                                                            (*fresh0 as
                                                                 libc::c_int +
                                                                 (999 as
                                                                      libc::c_int
                                                                      +
                                                                      1 as
                                                                          libc::c_int)
                                                                     /
                                                                     2 as
                                                                         libc::c_int)
                                                                as uint16_t
                                                    }
                                                    up = up.offset(1)
                                                }
                                                // [accunits still describes the original remainder length]
                                                if compare > 0 as libc::c_int
                                                       ||
                                                       compare ==
                                                           0 as libc::c_int &&
                                                           wasodd as
                                                               libc::c_int !=
                                                               0 {
                                                    // adjustment needed
                                                    let mut exp_0: int32_t =
                                                        0; // work
                                                    let mut expunits:
                                                            int32_t = 0;
                                                    let mut exprem: int32_t =
                                                        0;
                                                    // This is effectively causing round-up of the quotient,
            // so if it was the rare case where it was full and all
            // nines, it would overflow and hence division-impossible
            // should be raised
                                                    let mut allnines:
                                                            uint8_t =
                                                        0 as libc::c_int as
                                                            uint8_t; // 1 if quotient all nines
                                                    if quotdigits == reqdigits
                                                       { // borderline check
                                                        // could be borderline
                                                        up = quotlsu;
                                                        loop  {
                                                            if quotdigits >
                                                                   3 as
                                                                       libc::c_int
                                                               {
                                                                if *up as
                                                                       libc::c_int
                                                                       !=
                                                                       999 as
                                                                           libc::c_int
                                                                   {
                                                                    break ;
                                                                    // non-nines
                                                                }
                                                                quotdigits -=
                                                                    3 as
                                                                        libc::c_int;
                                                                up =
                                                                    up.offset(1)
                                                            } else {
                                                                // this is the last Unit
                                                                if *up as
                                                                       libc::c_uint
                                                                       ==
                                                                       DECPOWERS[quotdigits
                                                                                     as
                                                                                     usize].wrapping_sub(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                   {
                                                                    allnines =
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            uint8_t
                                                                }
                                                                break ;
                                                            }
                                                            // checked those digits
                                                        }
                                                        // up
                                                    }
                                                    if allnines != 0 {
                                                        *status |=
                                                            0x4 as libc::c_int
                                                                as
                                                                libc::c_uint;
                                                        current_block_213 =
                                                            5213335239096154175;
                                                    } else {
                                                        // rem-rhs is needed; the sign will invert.  Again, var1
            // can safely be used for the working Units array.
                                                        exp_0 =
                                                            (*rhs).exponent -
                                                                exponent; // RHS padding needed
                                                        // Calculate units and remainder from exponent.
                                                        expunits =
                                                            exp_0 /
                                                                3 as
                                                                    libc::c_int;
                                                        exprem =
                                                            exp_0 %
                                                                3 as
                                                                    libc::c_int;
                                                        // subtract [A+B*(-m)]; the result will always be negative
                                                        accunits =
                                                            -decUnitAddSub(accnext,
                                                                           accunits,
                                                                           (*rhs).lsu.as_ptr(),
                                                                           if (*rhs).digits
                                                                                  <=
                                                                                  49
                                                                                      as
                                                                                      libc::c_int
                                                                              {
                                                                               d2utable[(*rhs).digits
                                                                                            as
                                                                                            usize]
                                                                                   as
                                                                                   libc::c_int
                                                                           } else {
                                                                               ((*rhs).digits
                                                                                    +
                                                                                    3
                                                                                        as
                                                                                        libc::c_int
                                                                                    -
                                                                                    1
                                                                                        as
                                                                                        libc::c_int)
                                                                                   /
                                                                                   3
                                                                                       as
                                                                                       libc::c_int
                                                                           },
                                                                           expunits,
                                                                           accnext,
                                                                           -(DECPOWERS[exprem
                                                                                           as
                                                                                           usize]
                                                                                 as
                                                                                 int32_t)); // count digits exactly
                                                        accdigits =
                                                            decGetDigits(accnext,
                                                                         accunits); // and recalculate the units for copy
                                                        accunits =
                                                            if accdigits <=
                                                                   49 as
                                                                       libc::c_int
                                                               {
                                                                d2utable[accdigits
                                                                             as
                                                                             usize]
                                                                    as
                                                                    libc::c_int
                                                            } else {
                                                                (accdigits +
                                                                     3 as
                                                                         libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    /
                                                                    3 as
                                                                        libc::c_int
                                                            };
                                                        // [exponent is as for original remainder]
                                                        bits =
                                                            (bits as
                                                                 libc::c_int ^
                                                                 0x80 as
                                                                     libc::c_int)
                                                                as uint8_t;
                                                        current_block_213 =
                                                            11911614146017124710;
                                                    }
                                                    // flip the sign
                                                } else {
                                                    current_block_213 =
                                                        11911614146017124710;
                                                }
                                            }
                                        } else {
                                            current_block_213 =
                                                11911614146017124710;
                                        }
                                    }
                                    // REMNEAR
                                } else {
                                    current_block_213 = 11911614146017124710;
                                }
                                match current_block_213 {
                                    5213335239096154175 => { }
                                    _ => {
                                        // Set exponent and bits
                                        (*res).exponent =
                                            exponent; // [cleaned]
                                        (*res).bits =
                                            (bits as libc::c_int &
                                                 0x80 as libc::c_int) as
                                                uint8_t;
                                        // Now the coefficient.
                                        decSetCoeff(res, set, accnext,
                                                    accdigits, &mut residue,
                                                    status); // drop any storage used
                                        decFinalize(res, set, &mut residue,
                                                    status); // ..
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !varalloc.is_null() { free(varalloc as *mut libc::c_void); }
    if !allocacc.is_null() { free(allocacc as *mut libc::c_void); }
    return res;
}
// decDivideOp
/* ------------------------------------------------------------------ */
/* decMultiplyOp -- multiplication operation                          */
/*                                                                    */
/*  This routine performs the multiplication C=A x B.                 */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X*X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*   status is the usual accumulator                                  */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* ------------------------------------------------------------------ */
/* 'Classic' multiplication is used rather than Karatsuba, as the     */
/* latter would give only a minor improvement for the short numbers   */
/* expected to be handled most (and uses much more memory).           */
/*                                                                    */
/* There are two major paths here: the general-purpose ('old code')   */
/* path which handles all DECDPUN values, and a fastpath version      */
/* which is used if 64-bit ints are available, DECDPUN<=4, and more   */
/* than two calls to decUnitAddSub would be made.                     */
/*                                                                    */
/* The fastpath version lumps units together into 8-digit or 9-digit  */
/* chunks, and also uses a lazy carry strategy to minimise expensive  */
/* 64-bit divisions.  The chunks are then broken apart again into     */
/* units for continuing processing.  Despite this overhead, the       */
/* fastpath can speed up some 16-digit operations by 10x (and much    */
/* more for higher-precision calculations).                           */
/*                                                                    */
/* A buffer always has to be used for the accumulator; in the         */
/* fastpath, buffers are also always needed for the chunked copies of */
/* of the operand coefficients.                                       */
/* Static buffers are larger than needed just for multiply, to allow  */
/* for calls from other operations (notably exp).                     */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decMultiplyOp(mut res: *mut decNumber,
                                   mut lhs: *const decNumber,
                                   mut rhs: *const decNumber,
                                   mut set: *mut decContext,
                                   mut status: *mut uint32_t)
 -> *mut decNumber {
    let mut accunits: int32_t = 0; // Units of accumulator in use
    let mut exponent: int32_t = 0; // work
    let mut residue: int32_t = 0 as libc::c_int; // rounding residue
    let mut bits: uint8_t = 0; // result sign
    let mut acc: *mut uint16_t =
        0 as *mut uint16_t; // -> accumulator Unit array
    let mut needbytes: int32_t = 0; // size calculator
    let mut allocacc: *mut libc::c_void =
        0 as *mut libc::c_void; // -> allocated accumulator, iff allocated
    let mut accbuff: [uint16_t; 49] = [0; 49]; // buffer (+1 for DECBUFFER==0,
    // *4 for calls from other operations)
    let mut mer: *const uint16_t = 0 as *const uint16_t; // work
    let mut mermsup: *const uint16_t =
        0 as *const uint16_t; // Units in multiplicand
    let mut madlength: int32_t = 0; // Units to shift multiplicand by
    let mut shift: int32_t = 0;
    // if DECDPUN is 1 or 3 work in base 10**9, otherwise
    // (DECDPUN is 2 or 4) then work in base 10**8
    // odd
    // base
    // digits in base
    // carry resolution point [1->18]
    // three buffers are used, two for chunked copies of the operands
    // (base 10**8 or base 10**9) and one base 2**64 accumulator with
    // lazy carry evaluation
    let mut zlhibuff: [uint32_t; 10] =
        [0; 10]; // buffer (+1 for DECBUFFER==0)
    let mut zlhi: *mut uint32_t = zlhibuff.as_mut_ptr(); // -> lhs array
    let mut alloclhi: *mut uint32_t =
        0 as *mut uint32_t; // -> allocated buffer, iff allocated
    let mut zrhibuff: [uint32_t; 10] =
        [0; 10]; // buffer (+1 for DECBUFFER==0)
    let mut zrhi: *mut uint32_t = zrhibuff.as_mut_ptr(); // -> rhs array
    let mut allocrhi: *mut uint32_t =
        0 as *mut uint32_t; // -> allocated buffer, iff allocated
    let mut zaccbuff: [uint64_t; 20] =
        [0; 20]; // buffer (+1 for DECBUFFER==0)
    // [allocacc is shared for both paths, as only one will run]
    let mut zacc: *mut uint64_t =
        zaccbuff.as_mut_ptr(); // -> accumulator array for exact result
    let mut lip: *mut uint32_t = 0 as *mut uint32_t; // item pointers
    let mut rip: *mut uint32_t = 0 as *mut uint32_t; // most significant items
    let mut lmsi: *mut uint32_t =
        0 as *mut uint32_t; // item counts in the arrays
    let mut rmsi: *mut uint32_t = 0 as *mut uint32_t; // lazy carry counter
    let mut ilhs: int32_t = 0; // uLong carry
    let mut irhs: int32_t = 0; // carry (NB not uLong)
    let mut iacc: int32_t = 0; // work
    let mut lazy: int32_t = 0; // ..
    let mut lcarry: uint64_t = 0; // ..
    let mut carry: uint32_t = 0; // ..
    let mut count: int32_t = 0; // ..
    let mut cup: *const uint16_t = 0 as *const uint16_t;
    let mut up: *mut uint16_t = 0 as *mut uint16_t;
    let mut lp: *mut uint64_t = 0 as *mut uint64_t;
    let mut p: int32_t = 0;
    // precalculate result sign
    bits =
        (((*lhs).bits as libc::c_int ^ (*rhs).bits as libc::c_int) &
             0x80 as libc::c_int) as uint8_t;
    // handle infinities and NaNs
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // a special bit set
        if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) &
               (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
            // one or two NaNs
            decNaNs(res, lhs, rhs, set, status);
            return res
        }
        // one or two infinities; Infinity * 0 is invalid
        if (*lhs).bits as libc::c_int & 0x40 as libc::c_int ==
               0 as libc::c_int &&
               (*(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                    (*lhs).digits == 1 as libc::c_int &&
                    (*lhs).bits as libc::c_int &
                        (0x40 as libc::c_int | 0x20 as libc::c_int |
                             0x10 as libc::c_int) == 0 as libc::c_int) ||
               (*rhs).bits as libc::c_int & 0x40 as libc::c_int ==
                   0 as libc::c_int &&
                   (*(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                        (*rhs).digits == 1 as libc::c_int &&
                        (*rhs).bits as libc::c_int &
                            (0x40 as libc::c_int | 0x20 as libc::c_int |
                                 0x10 as libc::c_int) == 0 as libc::c_int) {
            *status |= 0x80 as libc::c_int as libc::c_uint; // infinity
            return res
        }
        decNumberZero(res);
        (*res).bits = (bits as libc::c_int | 0x40 as libc::c_int) as uint8_t;
        return res
    }
    // For best speed, as in DMSRCN [the original Rexx numerics
  // module], use the shorter number as the multiplier (rhs) and
  // the longer as the multiplicand (lhs) to minimise the number of
  // adds (partial products)
    if (*lhs).digits < (*rhs).digits {
        // swap...
        let mut hold: *const decNumber = lhs;
        lhs = rhs;
        rhs = hold
    }
    let mut current_block_111: u64;
    if (*rhs).digits > 3 as libc::c_int * 2 as libc::c_int {
        // use fastpath...
        // calculate the number of elements in each array
        ilhs =
            ((*lhs).digits + 9 as libc::c_int - 1 as libc::c_int) /
                9 as libc::c_int; // [ceiling]
        // count of units
        irhs =
            ((*rhs).digits + 9 as libc::c_int - 1 as libc::c_int) /
                9 as libc::c_int; // ..
        iacc = ilhs + irhs;
        // allocate buffers if required, as usual
        needbytes =
            (ilhs as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                 as libc::c_ulong) as int32_t;
        if needbytes >
               ::std::mem::size_of::<[uint32_t; 10]>() as libc::c_ulong as
                   int32_t {
            alloclhi = malloc(needbytes as libc::c_ulong) as *mut uint32_t;
            zlhi = alloclhi
        }
        needbytes =
            (irhs as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                 as libc::c_ulong) as int32_t;
        if needbytes >
               ::std::mem::size_of::<[uint32_t; 10]>() as libc::c_ulong as
                   int32_t {
            allocrhi = malloc(needbytes as libc::c_ulong) as *mut uint32_t;
            zrhi = allocrhi
        }
        // Allocating the accumulator space needs a special case when
      // DECDPUN=1 because when converting the accumulator to Units
      // after the multiplication each 8-byte item becomes 9 1-byte
      // units.  Therefore iacc extra bytes are needed at the front
      // (rounded up to a multiple of 8 bytes), and the uLong
      // accumulator starts offset the appropriate number of units
      // to the right to avoid overwrite during the unchunking.
        needbytes =
            (iacc as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint64_t>()
                                                 as libc::c_ulong) as
                int32_t; // -> target Unit array
        if needbytes >
               ::std::mem::size_of::<[uint64_t; 20]>() as libc::c_ulong as
                   int32_t {
            allocacc =
                malloc(needbytes as libc::c_ulong) as *mut uint64_t as
                    *mut libc::c_void;
            zacc = allocacc as *mut uint64_t
        }
        if zlhi.is_null() || zrhi.is_null() || zacc.is_null() {
            *status |= 0x10 as libc::c_int as libc::c_uint;
            current_block_111 = 12267375779178892496;
        } else {
            acc = zacc as *mut uint16_t;
            // assemble the chunked copies of the left and right sides
            count = (*lhs).digits; // save -> msi
            cup = (*lhs).lsu.as_ptr(); // save -> msi
            lip = zlhi;
            while count > 0 as libc::c_int {
                p = 0 as libc::c_int;
                *lip = 0 as libc::c_int as uint32_t;
                while p < 9 as libc::c_int && count > 0 as libc::c_int {
                    *lip =
                        (*lip as
                             libc::c_uint).wrapping_add((*cup as
                                                             libc::c_uint).wrapping_mul(DECPOWERS[p
                                                                                                      as
                                                                                                      usize]))
                            as uint32_t as uint32_t;
                    p += 3 as libc::c_int;
                    cup = cup.offset(1);
                    count -= 3 as libc::c_int
                }
                lip = lip.offset(1)
            }
            lmsi = lip.offset(-(1 as libc::c_int as isize));
            count = (*rhs).digits;
            cup = (*rhs).lsu.as_ptr();
            rip = zrhi;
            while count > 0 as libc::c_int {
                p = 0 as libc::c_int;
                *rip = 0 as libc::c_int as uint32_t;
                while p < 9 as libc::c_int && count > 0 as libc::c_int {
                    *rip =
                        (*rip as
                             libc::c_uint).wrapping_add((*cup as
                                                             libc::c_uint).wrapping_mul(DECPOWERS[p
                                                                                                      as
                                                                                                      usize]))
                            as uint32_t as uint32_t;
                    p += 3 as libc::c_int;
                    cup = cup.offset(1);
                    count -= 3 as libc::c_int
                }
                rip = rip.offset(1)
            }
            rmsi = rip.offset(-(1 as libc::c_int as isize));
            // zero the accumulator
            lp = zacc;
            while lp < zacc.offset(iacc as isize) {
                *lp = 0 as libc::c_int as uint64_t;
                lp = lp.offset(1)
            }
            /* Start the multiplication */
      // Resolving carries can dominate the cost of accumulating the
      // partial products, so this is only done when necessary.
      // Each uLong item in the accumulator can hold values up to
      // 2**64-1, and each partial product can be as large as
      // (10**FASTDIGS-1)**2.  When FASTDIGS=9, this can be added to
      // itself 18.4 times in a uLong without overflowing, so during
      // the main calculation resolution is carried out every 18th
      // add -- every 162 digits.  Similarly, when FASTDIGS=8, the
      // partial products can be added to themselves 1844.6 times in
      // a uLong without overflowing, so intermediate carry
      // resolution occurs only every 14752 digits.  Hence for common
      // short numbers usually only the one final carry resolution
      // occurs.
      // (The count is set via FASTLAZY to simplify experiments to
      // measure the value of this approach: a 35% improvement on a
      // [34x34] multiply.)
            lazy = 18 as libc::c_int; // carry delay count
            rip = zrhi; // rip loop
            while rip <= rmsi {
                // over each item in rhs
                lp =
                    zacc.offset(rip.wrapping_offset_from(zrhi) as libc::c_long
                                    as isize); // where to add the lhs
                // carry resolution
                lip = zlhi; // lip loop
                while lip <= lmsi {
                    // over each item in lhs
                    *lp =
                        (*lp as
                             libc::c_ulong).wrapping_add((*lip as
                                                              uint64_t).wrapping_mul(*rip
                                                                                         as
                                                                                         libc::c_ulong))
                            as uint64_t as uint64_t;
                    lip = lip.offset(1);
                    lp = lp.offset(1)
                    // [this should in-line]
                } // reset delay count
                lazy -= 1;
                if !(lazy > 0 as libc::c_int && rip != rmsi) {
                    lazy = 18 as libc::c_int;
                    // spin up the accumulator resolving overflows
                    lp = zacc; // it fits
                    while lp < zacc.offset(iacc as isize) {
                        if !(*lp < 1000000000 as libc::c_int as libc::c_ulong)
                           {
                            lcarry =
                                (*lp).wrapping_div(1000000000 as libc::c_int
                                                       as
                                                       libc::c_ulong); // top part [slow divide]
                            // lcarry can exceed 2**32-1, so check again; this check
          // and occasional extra divide (slow) is well worth it, as
          // it allows FASTLAZY to be increased to 18 rather than 4
          // in the FASTDIGS=9 case
                            if lcarry <
                                   1000000000 as libc::c_int as libc::c_ulong
                               {
                                carry = lcarry as uint32_t
                            } else { // [usual]
                                // two-place carry [fairly rare]
                                let mut carry2: uint32_t =
                                    lcarry.wrapping_div(1000000000 as
                                                            libc::c_int as
                                                            libc::c_ulong) as
                                        uint32_t; // top top part
                                let ref mut fresh1 =
                                    *lp.offset(2 as libc::c_int as
                                                   isize); // add to item+2
                                *fresh1 =
                                    (*fresh1 as
                                         libc::c_ulong).wrapping_add(carry2 as
                                                                         libc::c_ulong)
                                        as uint64_t as uint64_t; // [slow]
                                *lp =
                                    (*lp as
                                         libc::c_ulong).wrapping_sub((1000000000
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          uint64_t).wrapping_mul(1000000000
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_mul(carry2
                                                                                                                                     as
                                                                                                                                     libc::c_ulong))
                                        as uint64_t as
                                        uint64_t; // add to item above [inline]
                                carry =
                                    lcarry.wrapping_sub((1000000000 as
                                                             libc::c_int as
                                                             uint64_t).wrapping_mul(carry2
                                                                                        as
                                                                                        libc::c_ulong))
                                        as uint32_t
                            }
                            let ref mut fresh2 =
                                *lp.offset(1 as libc::c_int as isize);
                            *fresh2 =
                                (*fresh2 as
                                     libc::c_ulong).wrapping_add(carry as
                                                                     libc::c_ulong)
                                    as uint64_t as uint64_t;
                            *lp =
                                (*lp as
                                     libc::c_ulong).wrapping_sub((1000000000
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      uint64_t).wrapping_mul(carry
                                                                                                 as
                                                                                                 libc::c_ulong))
                                    as uint64_t as uint64_t
                        }
                        lp = lp.offset(1)
                        // [inline]
                    }
                }
                rip = rip.offset(1)
            }
            // The multiplication is complete; time to convert back into
      // units.  This can be done in-place in the accumulator and in
      // 32-bit operations, because carries were resolved after the
      // final add.  This needs N-1 divides and multiplies for
      // each item in the accumulator (which will become up to N
      // units, where 2<=N<=9).
            lp = zacc; // lp
            up = acc; // decapitate to uInt
            while lp < zacc.offset(iacc as isize) {
                let mut item: uint32_t = *lp as uint32_t; // p
                p = 0 as libc::c_int;
                while p < 9 as libc::c_int - 3 as libc::c_int {
                    let mut part: uint32_t =
                        item.wrapping_div((999 as libc::c_int +
                                               1 as libc::c_int) as
                                              libc::c_uint);
                    *up =
                        item.wrapping_sub(part.wrapping_mul((999 as
                                                                 libc::c_int +
                                                                 1 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint))
                            as uint16_t;
                    item = part;
                    p += 3 as libc::c_int;
                    up = up.offset(1)
                }
                *up = item as uint16_t;
                up = up.offset(1);
                lp = lp.offset(1)
            }
            accunits =
                up.wrapping_offset_from(acc) as libc::c_long as int32_t;
            current_block_111 = 9430418855388998878;
        }
    } else {
        // here to use units directly, without chunking ['old code']
        // if accumulator will be too long for local storage, then allocate
        acc = accbuff.as_mut_ptr();
        needbytes =
            (((if (*lhs).digits <= 49 as libc::c_int {
                   d2utable[(*lhs).digits as usize] as libc::c_int
               } else {
                   ((*lhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                       3 as libc::c_int
               }) +
                  (if (*rhs).digits <= 49 as libc::c_int {
                       d2utable[(*rhs).digits as usize] as libc::c_int
                   } else {
                       ((*rhs).digits + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   })) as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                 as libc::c_ulong) as
                int32_t; // -> assume buffer for accumulator
        if needbytes >
               ::std::mem::size_of::<[uint16_t; 49]>() as libc::c_ulong as
                   int32_t {
            allocacc =
                malloc(needbytes as libc::c_ulong) as *mut uint16_t as
                    *mut libc::c_void;
            if allocacc.is_null() {
                *status |= 0x10 as libc::c_int as libc::c_uint;
                current_block_111 = 12267375779178892496;
            } else {
                acc = allocacc as *mut uint16_t;
                current_block_111 = 900943123863005455;
            }
            // n
            // use the allocated space
        } else { current_block_111 = 900943123863005455; }
        match current_block_111 {
            12267375779178892496 => { }
            _ => {
                /* Now the main long multiplication loop */
      // Unlike the equivalent in the IBM Java implementation, there
      // is no advantage in calculating from msu to lsu.  So, do it
      // by the book, as it were.
      // Each iteration calculates ACC=ACC+MULTAND*MULT
                accunits = 1 as libc::c_int; // accumulator starts at '0'
                *acc = 0 as libc::c_int as uint16_t; // .. (lsu=0)
                shift = 0 as libc::c_int; // no multiplicand shift at first
                madlength =
                    if (*lhs).digits <= 49 as libc::c_int {
                        d2utable[(*lhs).digits as usize] as libc::c_int
                    } else {
                        ((*lhs).digits + 3 as libc::c_int - 1 as libc::c_int)
                            / 3 as libc::c_int
                    }; // this won't change
                mermsup =
                    (*rhs).lsu.as_ptr().offset((if (*rhs).digits <=
                                                       49 as libc::c_int {
                                                    d2utable[(*rhs).digits as
                                                                 usize] as
                                                        libc::c_int
                                                } else {
                                                    ((*rhs).digits +
                                                         3 as libc::c_int -
                                                         1 as libc::c_int) /
                                                        3 as libc::c_int
                                                }) as
                                                   isize); // -> msu+1 of multiplier
                mer = (*rhs).lsu.as_ptr();
                while mer < mermsup {
                    // Here, *mer is the next Unit in the multiplier to use
        // If non-zero [optimization] add it...
                    if *mer as libc::c_int != 0 as libc::c_int {
                        accunits =
                            decUnitAddSub(&mut *acc.offset(shift as isize),
                                          accunits - shift,
                                          (*lhs).lsu.as_ptr(), madlength,
                                          0 as libc::c_int,
                                          &mut *acc.offset(shift as isize),
                                          *mer as int32_t) + shift
                    } else {
                        // extend acc with a 0; it will be used shortly
                        *acc.offset(accunits as isize) =
                            0 as libc::c_int as
                                uint16_t; // [this avoids length of <=0 later]
                        accunits += 1
                    }
                    // add this for 'logical length'
                    shift += 1;
                    mer = mer.offset(1)
                }
                current_block_111 = 9430418855388998878;
            }
        }
    }
    match current_block_111 {
        9430418855388998878 => {
            // multiply multiplicand by 10**DECDPUN for next Unit to left
            // final cleanup
            // protect allocated storage
            // [following code does not require input rounding]
            // fastpath can be used
            // use the fast path if there are enough digits in the shorter
    // operand to make the setup and takedown worthwhile
            // within two decUnitAddSub calls
            // unchunked units
            // common end-path
            // acc now contains the exact result of the multiplication,
    // possibly with a leading zero unit; build the decNumber from
    // it, noting if any residue
            (*res).bits = bits; // set sign
            (*res).digits =
                decGetDigits(acc, accunits); // count digits exactly
            // There can be a 31-bit wrap in calculating the exponent.
    // This can only happen if both input exponents are negative and
    // both their magnitudes are large.  If there was a wrap, set a
    // safe very negative exponent, from which decFinalize() will
    // raise a hard underflow shortly.
            exponent =
                (*lhs).exponent + (*rhs).exponent; // calculate exponent
            if (*lhs).exponent < 0 as libc::c_int &&
                   (*rhs).exponent < 0 as libc::c_int &&
                   exponent > 0 as libc::c_int {
                exponent = -(2 as libc::c_int) * 999999999 as libc::c_int
            } // force underflow
            (*res).exponent = exponent; // OK to overwrite now
            // Set the coefficient.  If any rounding, residue records
            decSetCoeff(res, set, acc, (*res).digits, &mut residue,
                        status); // drop any storage used
            decFinalize(res, set, &mut residue, status); // ..
        }
        _ => { }
    } // ..
    if !allocacc.is_null() { free(allocacc); }
    if !allocrhi.is_null() { free(allocrhi as *mut libc::c_void); }
    if !alloclhi.is_null() { free(alloclhi as *mut libc::c_void); }
    return res;
}
// decMultiplyOp
/* ------------------------------------------------------------------ */
/* decExpOp -- effect exponentiation                                  */
/*                                                                    */
/*   This computes C = exp(A)                                         */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context; note that rounding mode has no effect        */
/*                                                                    */
/* C must have space for set->digits digits. status is updated but    */
/* not set.                                                           */
/*                                                                    */
/* Restrictions:                                                      */
/*                                                                    */
/*   digits, emax, and -emin in the context must be less than         */
/*   2*DEC_MAX_MATH (1999998), and the rhs must be within these       */
/*   bounds or a zero.  This is an internal routine, so these         */
/*   restrictions are contractual and not enforced.                   */
/*                                                                    */
/* A finite result is rounded using DEC_ROUND_HALF_EVEN; it will      */
/* almost always be correctly rounded, but may be up to 1 ulp in      */
/* error in rare cases.                                               */
/*                                                                    */
/* Finite results will always be full precision and Inexact, except   */
/* when A is a zero or -Infinity (giving 1 or 0 respectively).        */
/* ------------------------------------------------------------------ */
/* This approach used here is similar to the algorithm described in   */
/*                                                                    */
/*   Variable Precision Exponential Function, T. E. Hull and          */
/*   A. Abrham, ACM Transactions on Mathematical Software, Vol 12 #2, */
/*   pp79-91, ACM, June 1986.                                         */
/*                                                                    */
/* with the main difference being that the iterations in the series   */
/* evaluation are terminated dynamically (which does not require the  */
/* extra variable-precision variables which are expensive in this     */
/* context).                                                          */
/*                                                                    */
/* The error analysis in Hull & Abrham's paper applies except for the */
/* round-off error accumulation during the series evaluation.  This   */
/* code does not precalculate the number of iterations and so cannot  */
/* use Horner's scheme.  Instead, the accumulation is done at double- */
/* precision, which ensures that the additions of the terms are exact */
/* and do not accumulate round-off (and any round-off errors in the   */
/* terms themselves move 'to the right' faster than they can          */
/* accumulate).  This code also extends the calculation by allowing,  */
/* in the spirit of other decNumber operators, the input to be more   */
/* precise than the result (the precision used is based on the more   */
/* precise of the input or requested result).                         */
/*                                                                    */
/* Implementation notes:                                              */
/*                                                                    */
/* 1. This is separated out as decExpOp so it can be called from      */
/*    other Mathematical functions (notably Ln) with a wider range    */
/*    than normal.  In particular, it can handle the slightly wider   */
/*    (double) range needed by Ln (which has to be able to calculate  */
/*    exp(-x) where x can be the tiniest number (Ntiny).              */
/*                                                                    */
/* 2. Normalizing x to be <=0.1 (instead of <=1) reduces loop         */
/*    iterations by appoximately a third with additional (although    */
/*    diminishing) returns as the range is reduced to even smaller    */
/*    fractions.  However, h (the power of 10 used to correct the     */
/*    result at the end, see below) must be kept <=8 as otherwise     */
/*    the final result cannot be computed.  Hence the leverage is a   */
/*    sliding value (8-h), where potentially the range is reduced     */
/*    more for smaller values.                                        */
/*                                                                    */
/*    The leverage that can be applied in this way is severely        */
/*    limited by the cost of the raise-to-the power at the end,       */
/*    which dominates when the number of iterations is small (less    */
/*    than ten) or when rhs is short.  As an example, the adjustment  */
/*    x**10,000,000 needs 31 multiplications, all but one full-width. */
/*                                                                    */
/* 3. The restrictions (especially precision) could be raised with    */
/*    care, but the full decNumber range seems very hard within the   */
/*    32-bit limits.                                                  */
/*                                                                    */
/* 4. The working precisions for the static buffers are twice the     */
/*    obvious size to allow for calls from decNumberPower.            */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decExpOp(mut res: *mut decNumber,
                              mut rhs: *const decNumber,
                              mut set: *mut decContext,
                              mut status: *mut uint32_t) -> *mut decNumber {
    let mut ignore: uint32_t = 0 as libc::c_int as uint32_t; // working status
    let mut h: int32_t = 0; // adjusted exponent for 0.xxxx
    let mut p: int32_t = 0; // working precision
    let mut residue: int32_t = 0; // rounding residue
    let mut needbytes: uint32_t = 0; // for space calculations
    let mut x: *const decNumber = rhs; // (may point to safe copy later)
    let mut aset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // working contexts
    let mut tset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // work
    let mut dset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,};
    let mut comp: int32_t = 0;
    // the argument is often copied to normalize it, so (unusually) it
  // is treated like other buffers, using DECBUFFER, +1 in case
  // DECBUFFER is 0
    let mut bufr: [decNumber; 5] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            5]; // non-NULL if rhs buffer allocated
    let mut allocrhs: *mut decNumber = 0 as *mut decNumber;
    // the working precision will be no more than set->digits+8+1
  // so for on-stack buffers DECBUFFER+9 is used, +1 in case DECBUFFER
  // is 0 (and twice that for the accumulator)
    // buffer for t, term (working precision plus)
    let mut buft: [decNumber; 6] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            6]; // -> allocated buft, iff allocated
    let mut allocbuft: *mut decNumber = 0 as *mut decNumber; // term
    let mut t: *mut decNumber = buft.as_mut_ptr();
    // buffer for a, accumulator (working precision * 2), at least 9
    let mut bufa: [decNumber; 10] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            10]; // -> allocated bufa, iff allocated
    let mut allocbufa: *mut decNumber = 0 as *mut decNumber; // accumulator
    let mut a: *mut decNumber = bufa.as_mut_ptr();
    // decNumber for the divisor term; this needs at most 9 digits
  // and so can be fixed size [16 so can use standard context]
    let mut bufd: [decNumber; 2] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            2]; // divisor
    let mut d: *mut decNumber = bufd.as_mut_ptr(); // constant 1
    let mut numone: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
    let mut current_block_89: u64;
    if (*rhs).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // handle infinities and NaNs
        if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
               0 as libc::c_int { // a NaN
            // an infinity
            if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
                // -Infinity -> +0
                decNumberZero(res);
            } else { decNumberCopy(res, rhs); }
            // +Infinity -> self
        } else {
            decNaNs(res, rhs, 0 as *const decNumber, set,
                    status); // [no status to set]
        }
    } else if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*rhs).digits == 1 as libc::c_int &&
                  (*rhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        // zeros -> exact 1
        decNumberZero(res); // make clean 1
        *(*res).lsu.as_mut_ptr() = 1 as libc::c_int as uint16_t
    } else { // ..
        // e**x when 0 < x < 0.66 is < 1+3x/2, hence can fast-path
    // positive and negative tiny cases which will result in inexact
    // 1.  This also allows the later add-accumulate to always be
    // exact (because its length will never be more than twice the
    // working precision).
    // The comparator (tiny) needs just one digit, so use the
    // decNumber d for it (reused as the divisor, etc., below); its
    // exponent is such that if x is positive it will have
    // set->digits-1 zeros between the decimal point and the digit,
    // which is 4, and if x is negative one more zero there as the
    // more precise result will be of the form 0.9999999 rather than
    // 1.0000001.  Hence, tiny will be 0.0000004  if digits=7 and x>0
    // or 0.00000004 if digits=7 and x<0.  If RHS not larger than
    // this then the result will be 1.000000
        decNumberZero(d); // clean
        *(*d).lsu.as_mut_ptr() = 4 as libc::c_int as uint16_t; // set 4 ..
        (*d).exponent = -(*set).digits; // * 10**(-d)
        if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
               0 as libc::c_int {
            (*d).exponent -= 1
        } // negative case
        comp =
            decCompare(d, rhs,
                       1 as libc::c_int as uint8_t); // signless compare
        if comp == 0x80000000 as libc::c_uint as int32_t {
            *status |= 0x10 as libc::c_int as libc::c_uint
        } else if comp >= 0 as libc::c_int { // tiny
            // rhs < d
            let mut shift: int32_t =
                (*set).digits - 1 as libc::c_int; // set 1
            decNumberZero(res); // ..
            *(*res).lsu.as_mut_ptr() =
                1 as libc::c_int as uint16_t; // make 1.0000...
            (*res).digits =
                decShiftToMost((*res).lsu.as_mut_ptr(), 1 as libc::c_int,
                               shift); // .. inexactly
            (*res).exponent = -shift;
            *status |=
                (0x20 as libc::c_int | 0x800 as libc::c_int) as libc::c_uint
        } else {
            // set up the context to be used for calculating a, as this is
    // used on both paths below
            decContextDefault(&mut aset, 64 as libc::c_int);
            // accumulator bounds are as requested (could underflow)
            aset.emax = (*set).emax; // usual bounds
            aset.emin = (*set).emin; // ..
            aset.clamp =
                0 as libc::c_int as uint8_t; // and no concrete format
            // calculate the adjusted (Hull & Abrham) exponent (where the
    // decimal point is just to the left of the coefficient msd)
            h = (*rhs).exponent + (*rhs).digits;
            // if h>8 then 10**h cannot be calculated safely; however, when
    // h=8 then exp(|rhs|) will be at least exp(1E+7) which is at
    // least 6.59E+4342944, so (due to the restriction on Emax/Emin)
    // overflow (or underflow to 0) is guaranteed -- so this case can
    // be handled by simply forcing the appropriate excess
            if h > 8 as libc::c_int { // h<=8
                // overflow/underflow
                // set up here so Power call below will over or underflow to
      // zero; set accumulator to either 2 or 0.02
      // [stack buffer for a is always big enough for this]
                decNumberZero(a);
                // set a working precision
                *(*a).lsu.as_mut_ptr() =
                    2 as libc::c_int as uint16_t; // not 1 but < exp(1)
                if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                       0 as libc::c_int {
                    (*a).exponent = -(2 as libc::c_int)
                } // make 0.02
                h = 8 as libc::c_int; // clamp so 10**h computable
                p = 9 as libc::c_int;
                current_block_89 = 7416055328783156979;
            } else {
                // h<=8
                let mut maxlever: int32_t =
                    if (*rhs).digits > 8 as libc::c_int {
                        1 as libc::c_int
                    } else { 0 as libc::c_int };
                // [could/should increase this for precisions >40 or so, too]
                // if h is 8, cannot normalize to a lower upper limit because
      // the final result will not be computable (see notes above),
      // but leverage can be applied whenever h is less than 8.
      // Apply as much as possible, up to a MAXLEVER digits, which
      // sets the tradeoff against the cost of the later a**(10**h).
      // As h is increased, the working precision below also
      // increases to compensate for the "constant digits at the
      // front" effect.
                let mut lever: int32_t =
                    if 8 as libc::c_int - h > maxlever {
                        maxlever
                    } else { (8 as libc::c_int) - h }; // leverage attainable
                let mut use_0: int32_t =
                    -(*rhs).digits - lever; // exponent to use for RHS
                h += lever; // apply leverage selected
                if h < 0 as libc::c_int {
                    // clamp
                    use_0 += h; // [may end up subnormal]
                    h = 0 as libc::c_int
                }
                // Take a copy of RHS if it needs normalization (true whenever x>=1)
                if (*rhs).exponent != use_0 {
                    let mut newrhs: *mut decNumber =
                        bufr.as_mut_ptr(); // assume will fit on stack
                    needbytes =
                        (::std::mem::size_of::<decNumber>() as
                             libc::c_ulong).wrapping_add((((if (*rhs).digits
                                                                   <=
                                                                   49 as
                                                                       libc::c_int
                                                               {
                                                                d2utable[(*rhs).digits
                                                                             as
                                                                             usize]
                                                                    as
                                                                    libc::c_int
                                                            } else {
                                                                ((*rhs).digits
                                                                     +
                                                                     3 as
                                                                         libc::c_int
                                                                     -
                                                                     1 as
                                                                         libc::c_int)
                                                                    /
                                                                    3 as
                                                                        libc::c_int
                                                            }) -
                                                               1 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                              as
                                                                                              libc::c_ulong))
                            as uint32_t;
                    if needbytes as libc::c_ulong >
                           ::std::mem::size_of::<[decNumber; 5]>() as
                               libc::c_ulong {
                        // need malloc space
                        allocrhs =
                            malloc(needbytes as libc::c_ulong) as
                                *mut decNumber;
                        if allocrhs.is_null() {
                            // use the allocated space
                            // hopeless -- abandon
                            *status |=
                                0x10 as libc::c_int as
                                    libc::c_uint; // copy to safe space
                            current_block_89 =
                                5636883459695696059; // normalize; now <1
                        } else {
                            newrhs = allocrhs;
                            current_block_89 = 9925100494328262799;
                        }
                    } else { current_block_89 = 9925100494328262799; }
                    match current_block_89 {
                        5636883459695696059 => { }
                        _ => {
                            decNumberCopy(newrhs, rhs);
                            (*newrhs).exponent = use_0;
                            x = newrhs;
                            current_block_89 = 13707613154239713890;
                        }
                    }
                    // ready for use
                    // decNumberShow(x);
                } else { current_block_89 = 13707613154239713890; }
                match current_block_89 {
                    5636883459695696059 => { }
                    _ => {
                        // Now use the usual power series to evaluate exp(x).  The
      // series starts as 1 + x + x^2/2 ... so prime ready for the
      // third term by setting the term variable t=x, the accumulator
      // a=1, and the divisor d=2.
                        // First determine the working precision.  From Hull & Abrham
      // this is set->digits+h+2.  However, if x is 'over-precise' we
      // need to allow for all its digits to potentially participate
      // (consider an x where all the excess digits are 9s) so in
      // this case use x->digits+h+2
                        p =
                            (if (*x).digits < (*set).digits {
                                 (*set).digits
                             } else { (*x).digits }) + h +
                                2 as libc::c_int; // [h<=8]
                        // a and t are variable precision, and depend on p, so space
      // must be allocated for them if necessary
                        // the accumulator needs to be able to hold 2p digits so that
      // the additions on the second and subsequent iterations are
      // sufficiently exact.
                        needbytes =
                            (::std::mem::size_of::<decNumber>() as
                                 libc::c_ulong).wrapping_add((((if p *
                                                                       2 as
                                                                           libc::c_int
                                                                       <=
                                                                       49 as
                                                                           libc::c_int
                                                                   {
                                                                    d2utable[(p
                                                                                  *
                                                                                  2
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 usize]
                                                                        as
                                                                        libc::c_int
                                                                } else {
                                                                    (p *
                                                                         2 as
                                                                             libc::c_int
                                                                         +
                                                                         3 as
                                                                             libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        /
                                                                        3 as
                                                                            libc::c_int
                                                                }) -
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as uint32_t;
                        if needbytes as libc::c_ulong >
                               ::std::mem::size_of::<[decNumber; 10]>() as
                                   libc::c_ulong {
                            // need malloc space
                            allocbufa =
                                malloc(needbytes as libc::c_ulong) as
                                    *mut decNumber;
                            if allocbufa.is_null() {
                                // use the allocated space
                                // hopeless -- abandon
                                *status |=
                                    0x10 as libc::c_int as libc::c_uint;
                                current_block_89 = 5636883459695696059;
                            } else {
                                a = allocbufa;
                                current_block_89 = 12961834331865314435;
                            }
                        } else { current_block_89 = 12961834331865314435; }
                        match current_block_89 {
                            5636883459695696059 => { }
                            _ => {
                                // the term needs to be able to hold p digits (which is
      // guaranteed to be larger than x->digits, so the initial copy
      // is safe); it may also be used for the raise-to-power
      // calculation below, which needs an extra two digits
                                needbytes =
                                    (::std::mem::size_of::<decNumber>() as
                                         libc::c_ulong).wrapping_add((((if p +
                                                                               2
                                                                                   as
                                                                                   libc::c_int
                                                                               <=
                                                                               49
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            d2utable[(p
                                                                                          +
                                                                                          2
                                                                                              as
                                                                                              libc::c_int)
                                                                                         as
                                                                                         usize]
                                                                                as
                                                                                libc::c_int
                                                                        } else {
                                                                            (p
                                                                                 +
                                                                                 2
                                                                                     as
                                                                                     libc::c_int
                                                                                 +
                                                                                 3
                                                                                     as
                                                                                     libc::c_int
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                /
                                                                                3
                                                                                    as
                                                                                    libc::c_int
                                                                        }) -
                                                                           1
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                                          as
                                                                                                          libc::c_ulong))
                                        as uint32_t;
                                if needbytes as libc::c_ulong >
                                       ::std::mem::size_of::<[decNumber; 6]>()
                                           as libc::c_ulong {
                                    // need malloc space
                                    allocbuft =
                                        malloc(needbytes as libc::c_ulong) as
                                            *mut decNumber;
                                    if allocbuft.is_null() {
                                        // use the allocated space
                                        // hopeless -- abandon
                                        *status |=
                                            0x10 as libc::c_int as
                                                libc::c_uint; // term=x
                                        current_block_89 =
                                            5636883459695696059; // accumulator=1
                                    } else {
                                        t = allocbuft; // divisor=2
                                        current_block_89 =
                                            6033931424626438518; // constant 1 for increment
                                    }
                                } else {
                                    current_block_89 = 6033931424626438518;
                                }
                                match current_block_89 {
                                    5636883459695696059 => { }
                                    _ => {
                                        decNumberCopy(t, x);
                                        decNumberZero(a);
                                        *(*a).lsu.as_mut_ptr() =
                                            1 as libc::c_int as uint16_t;
                                        decNumberZero(d);
                                        *(*d).lsu.as_mut_ptr() =
                                            2 as libc::c_int as uint16_t;
                                        decNumberZero(&mut numone);
                                        *numone.lsu.as_mut_ptr() =
                                            1 as libc::c_int as uint16_t;
                                        // set up the contexts for calculating a, t, and d
                                        decContextDefault(&mut tset,
                                                          64 as libc::c_int);
                                        dset = tset;
                                        // accumulator bounds are set above, set precision now
                                        aset.digits =
                                            p * 2 as libc::c_int; // double
                                        // term bounds avoid any underflow or overflow
                                        tset.digits = p; // [emax is plenty]
                                        tset.emin =
                                            -(999999999 as libc::c_int);
                                        loop 
                                             // [dset.digits=16, etc., are sufficient]
                                             // finally ready to roll
                                             // only the status from the accumulation is interesting
        // [but it should remain unchanged after first add]
                                             {
                                            decAddOp(a, a, t, &mut aset,
                                                     0 as libc::c_int as
                                                         uint8_t,
                                                     status); // a=a+t
                                            decMultiplyOp(t, t, x, &mut tset,
                                                          &mut ignore); // t=t*x
                                            decDivideOp(t, t, d, &mut tset,
                                                        0x80 as libc::c_int as
                                                            uint8_t,
                                                        &mut ignore); // t=t/d
                                            // the iteration ends when the term cannot affect the result,
        // if rounded to p digits, which is when its value is smaller
        // than the accumulator by p+1 digits.  There must also be
        // full precision in a.
                                            if (*a).digits + (*a).exponent >=
                                                   (*t).digits + (*t).exponent
                                                       + p + 1 as libc::c_int
                                                   && (*a).digits >= p {
                                                break ;
                                            }
                                            decAddOp(d, d, &mut numone,
                                                     &mut dset,
                                                     0 as libc::c_int as
                                                         uint8_t,
                                                     &mut ignore);
                                        }
                                        current_block_89 =
                                            7416055328783156979;
                                    }
                                }
                            }
                        }
                    }
                }
                // iterate
            }
            match current_block_89 {
                5636883459695696059 => { }
                _ => {
                    // apply postconditioning: a=a**(10**h) -- this is calculated
    // at a slightly higher precision than Hull & Abrham suggest
                    if h > 0 as libc::c_int {
                        let mut seenbit: int32_t =
                            0 as libc::c_int; // set once a 1-bit is seen
                        let mut i: int32_t = 0; // counter
                        let mut n: int32_t =
                            DECPOWERS[h as usize] as
                                int32_t; // always positive
                        aset.digits =
                            p + 2 as libc::c_int; // sufficient precision
                        // avoid the overhead and many extra digits of decNumberPower
      // as all that is needed is the short 'multipliers' loop; here
      // accumulate the answer into t
                        decNumberZero(t); // acc=1
                        *(*t).lsu.as_mut_ptr() =
                            1 as libc::c_int as
                                uint16_t; /*i*/ // 32 bits
      // decNumberShow(t);
                        i = 1 as libc::c_int;
                        loop  {
                            // for each bit [top bit ignored]
                            // abandon if have had overflow or terminal underflow
                            if *status &
                                   (0x200 as libc::c_int |
                                        0x2000 as libc::c_int) as libc::c_uint
                                   != 0 {
                                // interesting?
                                if *status &
                                       0x200 as libc::c_int as libc::c_uint !=
                                       0 ||
                                       *(*t).lsu.as_mut_ptr() as libc::c_int
                                           == 0 as libc::c_int &&
                                           (*t).digits == 1 as libc::c_int &&
                                           (*t).bits as libc::c_int &
                                               (0x40 as libc::c_int |
                                                    0x20 as libc::c_int |
                                                    0x10 as libc::c_int) ==
                                               0 as libc::c_int {
                                    break
                                        ; // move next bit to testable position
                                }
                            }
                            n = n << 1 as libc::c_int;
                            if n < 0 as libc::c_int {
                                // top bit is set
                                seenbit = 1 as libc::c_int;
                                decMultiplyOp(t, t, a, &mut aset,
                                              status); // OK, have a significant bit
                                // acc=acc*x
                            } // that was the last bit
                            if i == 31 as libc::c_int {
                                break ; // no need to square 1
                            }
                            if !(seenbit == 0) {
                                decMultiplyOp(t, t, t, &mut aset, status);
                            }
                            i += 1
                            // acc=acc*acc [square]
                        }
                        a = t
                    }
                    // Copy and round the result to res
                    residue = 1 as libc::c_int; // indicate dirt to right ..
                    if *(*a).lsu.as_mut_ptr() as libc::c_int ==
                           0 as libc::c_int && (*a).digits == 1 as libc::c_int
                           &&
                           (*a).bits as libc::c_int &
                               (0x40 as libc::c_int | 0x20 as libc::c_int |
                                    0x10 as libc::c_int) == 0 as libc::c_int {
                        residue = 0 as libc::c_int
                    } // .. unless underflowed to 0
                    aset.digits = (*set).digits; // [use default rounding]
                    decCopyFit(res, a, &mut aset, &mut residue,
                               status); // copy & shorten
                    decFinalize(res, set, &mut residue, status);
                }
            }
        }
    }
    // cleanup/set flags
    // protect allocated storage
    if !allocrhs.is_null() {
        free(allocrhs as *mut libc::c_void); // drop any storage used
    } // ..
    if !allocbufa.is_null() {
        free(allocbufa as *mut libc::c_void); // ..
    }
    if !allocbuft.is_null() { free(allocbuft as *mut libc::c_void); }
    return res;
}
// [status is handled by caller]
// decExpOp
/* ------------------------------------------------------------------ */
/* Initial-estimate natural logarithm table                           */
/*                                                                    */
/*   LNnn -- 90-entry 16-bit table for values from .10 through .99.   */
/*           The result is a 4-digit encode of the coefficient (c=the */
/*           top 14 bits encoding 0-9999) and a 2-digit encode of the */
/*           exponent (e=the bottom 2 bits encoding 0-3)              */
/*                                                                    */
/*           The resulting value is given by:                         */
/*                                                                    */
/*             v = -c * 10**(-e-3)                                    */
/*                                                                    */
/*           where e and c are extracted from entry k = LNnn[x-10]    */
/*           where x is truncated (NB) into the range 10 through 99,  */
/*           and then c = k>>2 and e = k&3.                           */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub static mut LNnn: [uint16_t; 90] =
    [9016 as libc::c_int as uint16_t, 8652 as libc::c_int as uint16_t,
     8316 as libc::c_int as uint16_t, 8008 as libc::c_int as uint16_t,
     7724 as libc::c_int as uint16_t, 7456 as libc::c_int as uint16_t,
     7208 as libc::c_int as uint16_t, 6972 as libc::c_int as uint16_t,
     6748 as libc::c_int as uint16_t, 6540 as libc::c_int as uint16_t,
     6340 as libc::c_int as uint16_t, 6148 as libc::c_int as uint16_t,
     5968 as libc::c_int as uint16_t, 5792 as libc::c_int as uint16_t,
     5628 as libc::c_int as uint16_t, 5464 as libc::c_int as uint16_t,
     5312 as libc::c_int as uint16_t, 5164 as libc::c_int as uint16_t,
     5020 as libc::c_int as uint16_t, 4884 as libc::c_int as uint16_t,
     4748 as libc::c_int as uint16_t, 4620 as libc::c_int as uint16_t,
     4496 as libc::c_int as uint16_t, 4376 as libc::c_int as uint16_t,
     4256 as libc::c_int as uint16_t, 4144 as libc::c_int as uint16_t,
     4032 as libc::c_int as uint16_t, 39233 as libc::c_int as uint16_t,
     38181 as libc::c_int as uint16_t, 37157 as libc::c_int as uint16_t,
     36157 as libc::c_int as uint16_t, 35181 as libc::c_int as uint16_t,
     34229 as libc::c_int as uint16_t, 33297 as libc::c_int as uint16_t,
     32389 as libc::c_int as uint16_t, 31501 as libc::c_int as uint16_t,
     30629 as libc::c_int as uint16_t, 29777 as libc::c_int as uint16_t,
     28945 as libc::c_int as uint16_t, 28129 as libc::c_int as uint16_t,
     27329 as libc::c_int as uint16_t, 26545 as libc::c_int as uint16_t,
     25777 as libc::c_int as uint16_t, 25021 as libc::c_int as uint16_t,
     24281 as libc::c_int as uint16_t, 23553 as libc::c_int as uint16_t,
     22837 as libc::c_int as uint16_t, 22137 as libc::c_int as uint16_t,
     21445 as libc::c_int as uint16_t, 20769 as libc::c_int as uint16_t,
     20101 as libc::c_int as uint16_t, 19445 as libc::c_int as uint16_t,
     18801 as libc::c_int as uint16_t, 18165 as libc::c_int as uint16_t,
     17541 as libc::c_int as uint16_t, 16925 as libc::c_int as uint16_t,
     16321 as libc::c_int as uint16_t, 15721 as libc::c_int as uint16_t,
     15133 as libc::c_int as uint16_t, 14553 as libc::c_int as uint16_t,
     13985 as libc::c_int as uint16_t, 13421 as libc::c_int as uint16_t,
     12865 as libc::c_int as uint16_t, 12317 as libc::c_int as uint16_t,
     11777 as libc::c_int as uint16_t, 11241 as libc::c_int as uint16_t,
     10717 as libc::c_int as uint16_t, 10197 as libc::c_int as uint16_t,
     9685 as libc::c_int as uint16_t, 9177 as libc::c_int as uint16_t,
     8677 as libc::c_int as uint16_t, 8185 as libc::c_int as uint16_t,
     7697 as libc::c_int as uint16_t, 7213 as libc::c_int as uint16_t,
     6737 as libc::c_int as uint16_t, 6269 as libc::c_int as uint16_t,
     5801 as libc::c_int as uint16_t, 5341 as libc::c_int as uint16_t,
     4889 as libc::c_int as uint16_t, 4437 as libc::c_int as uint16_t,
     39930 as libc::c_int as uint16_t, 35534 as libc::c_int as uint16_t,
     31186 as libc::c_int as uint16_t, 26886 as libc::c_int as uint16_t,
     22630 as libc::c_int as uint16_t, 18418 as libc::c_int as uint16_t,
     14254 as libc::c_int as uint16_t, 10130 as libc::c_int as uint16_t,
     6046 as libc::c_int as uint16_t, 20055 as libc::c_int as uint16_t];
/* ------------------------------------------------------------------ */
/* decLnOp -- effect natural logarithm                                */
/*                                                                    */
/*   This computes C = ln(A)                                          */
/*                                                                    */
/*   res is C, the result.  C may be A                                */
/*   rhs is A                                                         */
/*   set is the context; note that rounding mode has no effect        */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Notable cases:                                                     */
/*   A<0 -> Invalid                                                   */
/*   A=0 -> -Infinity (Exact)                                         */
/*   A=+Infinity -> +Infinity (Exact)                                 */
/*   A=1 exactly -> 0 (Exact)                                         */
/*                                                                    */
/* Restrictions (as for Exp):                                         */
/*                                                                    */
/*   digits, emax, and -emin in the context must be less than         */
/*   DEC_MAX_MATH+11 (1000010), and the rhs must be within these      */
/*   bounds or a zero.  This is an internal routine, so these         */
/*   restrictions are contractual and not enforced.                   */
/*                                                                    */
/* A finite result is rounded using DEC_ROUND_HALF_EVEN; it will      */
/* almost always be correctly rounded, but may be up to 1 ulp in      */
/* error in rare cases.                                               */
/* ------------------------------------------------------------------ */
/* The result is calculated using Newton's method, with each          */
/* iteration calculating a' = a + x * exp(-a) - 1.  See, for example, */
/* Epperson 1989.                                                     */
/*                                                                    */
/* The iteration ends when the adjustment x*exp(-a)-1 is tiny enough. */
/* This has to be calculated at the sum of the precision of x and the */
/* working precision.                                                 */
/*                                                                    */
/* Implementation notes:                                              */
/*                                                                    */
/* 1. This is separated out as decLnOp so it can be called from       */
/*    other Mathematical functions (e.g., Log 10) with a wider range  */
/*    than normal.  In particular, it can handle the slightly wider   */
/*    (+9+2) range needed by a power function.                        */
/*                                                                    */
/* 2. The speed of this function is about 10x slower than exp, as     */
/*    it typically needs 4-6 iterations for short numbers, and the    */
/*    extra precision needed adds a squaring effect, twice.           */
/*                                                                    */
/* 3. Fastpaths are included for ln(10) and ln(2), up to length 40,   */
/*    as these are common requests.  ln(10) is used by log10(x).      */
/*                                                                    */
/* 4. An iteration might be saved by widening the LNnn table, and     */
/*    would certainly save at least one if it were made ten times     */
/*    bigger, too (for truncated fractions 0.100 through 0.999).      */
/*    However, for most practical evaluations, at least four or five  */
/*    iterations will be neede -- so this would only speed up by      */
/*    20-25% and that probably does not justify increasing the table  */
/*    size.                                                           */
/*                                                                    */
/* 5. The static buffers are larger than might be expected to allow   */
/*    for calls from decNumberPower.                                  */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decLnOp(mut res: *mut decNumber,
                             mut rhs: *const decNumber,
                             mut set: *mut decContext,
                             mut status: *mut uint32_t) -> *mut decNumber {
    let mut ignore: uint32_t =
        0 as libc::c_int as uint32_t; // working status accumulator
    let mut needbytes: uint32_t = 0; // for space calculations
    let mut residue: int32_t = 0; // rounding residue
    let mut r: int32_t = 0; // rhs=f*10**r [see below]
    let mut p: int32_t = 0; // working precision
    let mut pp: int32_t = 0; // precision for iteration
    let mut t: int32_t = 0; // work
    // buffers for a (accumulator, typically precision+2) and b
  // (adjustment calculator, same size)
    let mut bufa: [decNumber; 4] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            4]; // -> allocated bufa, iff allocated
    let mut allocbufa: *mut decNumber =
        0 as *mut decNumber; // accumulator/work
    let mut a: *mut decNumber =
        bufa.as_mut_ptr(); // -> allocated bufa, iff allocated
    let mut bufb: [decNumber; 5] =
        [decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
            5]; // adjustment/work
    let mut allocbufb: *mut decNumber = 0 as *mut decNumber; // constant 1
    let mut b: *mut decNumber = bufb.as_mut_ptr(); // work
    let mut numone: decNumber =
        decNumber{digits: 0,
                  exponent: 0,
                  bits: 0,
                  lsu: [0; 1],}; // working contexts
    let mut cmp: decNumber =
        decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
    let mut aset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,};
    let mut bset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,};
    let mut current_block_82: u64;
    if (*rhs).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // handle infinities and NaNs
        if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
               0 as libc::c_int { // a NaN
            // an infinity
            if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
                // -Infinity -> error
                *status |= 0x80 as libc::c_int as libc::c_uint
            } else { decNumberCopy(res, rhs); }
            // +Infinity -> self
        } else {
            decNaNs(res, rhs, 0 as *const decNumber, set,
                    status); // [no status to set]
        }
    } else if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                  (*rhs).digits == 1 as libc::c_int &&
                  (*rhs).bits as libc::c_int &
                      (0x40 as libc::c_int | 0x20 as libc::c_int |
                           0x10 as libc::c_int) == 0 as libc::c_int {
        // +/- zeros -> -Infinity
        decNumberZero(res); // make clean
        (*res).bits = (0x40 as libc::c_int | 0x80 as libc::c_int) as uint8_t
    } else if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                  0 as libc::c_int { // set - infinity
        // Non-zero negatives are bad...
        // -x -> error
        *status |= 0x80 as libc::c_int as libc::c_uint
    } else {
        // Here, rhs is positive, finite, and in range
        // lookaside fastpath code for ln(2) and ln(10) at common lengths
        if (*rhs).exponent == 0 as libc::c_int &&
               (*set).digits <= 40 as libc::c_int {
            if *(*rhs).lsu.as_ptr().offset(0 as libc::c_int as isize) as
                   libc::c_int == 10 as libc::c_int &&
                   (*rhs).digits == 2 as libc::c_int { // integer and short
                // ln(10)
                aset = *set; // is inexact
                aset.round = DEC_ROUND_HALF_EVEN;
                decNumberFromString(res,
                                    b"2.302585092994045684017991454684364207601\x00"
                                        as *const u8 as *const libc::c_char,
                                    &mut aset);
                *status |=
                    (0x20 as libc::c_int | 0x800 as libc::c_int) as
                        libc::c_uint;
                current_block_82 = 17372050596571538954;
            } else if *(*rhs).lsu.as_ptr().offset(0 as libc::c_int as isize)
                          as libc::c_int == 2 as libc::c_int &&
                          (*rhs).digits == 1 as libc::c_int {
                // ln(2)
                aset = *set;
                aset.round = DEC_ROUND_HALF_EVEN;
                decNumberFromString(res,
                                    b"0.6931471805599453094172321214581765680755\x00"
                                        as *const u8 as *const libc::c_char,
                                    &mut aset);
                *status |=
                    (0x20 as libc::c_int | 0x800 as libc::c_int) as
                        libc::c_uint;
                current_block_82 = 17372050596571538954;
            } else { current_block_82 = 2873832966593178012; }
        } else { current_block_82 = 2873832966593178012; }
        match current_block_82 {
            17372050596571538954 => { }
            _ => {
                // Determine the working precision.  This is normally the
    // requested precision + 2, with a minimum of 9.  However, if
    // the rhs is 'over-precise' then allow for all its digits to
    // potentially participate (consider an rhs where all the excess
    // digits are 9s) so in this case use rhs->digits+2.
                p =
                    (if (*rhs).digits <
                            (if (*set).digits < 7 as libc::c_int {
                                 7 as libc::c_int
                             } else { (*set).digits }) {
                         (if (*set).digits < 7 as libc::c_int {
                              7 as libc::c_int
                          } else { (*set).digits })
                     } else { (*rhs).digits }) + 2 as libc::c_int;
                // Allocate space for the accumulator and the high-precision
    // adjustment calculator, if necessary.  The accumulator must
    // be able to hold p digits, and the adjustment up to
    // rhs->digits+p digits.  They are also made big enough for 16
    // digits so that they can be used for calculating the initial
    // estimate.
                needbytes =
                    (::std::mem::size_of::<decNumber>() as
                         libc::c_ulong).wrapping_add((((if (if p <
                                                                   16 as
                                                                       libc::c_int
                                                               {
                                                                16 as
                                                                    libc::c_int
                                                            } else { p }) <=
                                                               49 as
                                                                   libc::c_int
                                                           {
                                                            d2utable[(if p <
                                                                             16
                                                                                 as
                                                                                 libc::c_int
                                                                         {
                                                                          16
                                                                              as
                                                                              libc::c_int
                                                                      } else {
                                                                          p
                                                                      }) as
                                                                         usize]
                                                                as libc::c_int
                                                        } else {
                                                            ((if p <
                                                                     16 as
                                                                         libc::c_int
                                                                 {
                                                                  16 as
                                                                      libc::c_int
                                                              } else { p }) +
                                                                 3 as
                                                                     libc::c_int
                                                                 -
                                                                 1 as
                                                                     libc::c_int)
                                                                /
                                                                3 as
                                                                    libc::c_int
                                                        }) - 1 as libc::c_int)
                                                          as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                          as
                                                                                          libc::c_ulong))
                        as uint32_t;
                if needbytes as libc::c_ulong >
                       ::std::mem::size_of::<[decNumber; 4]>() as
                           libc::c_ulong {
                    // need malloc space
                    allocbufa =
                        malloc(needbytes as libc::c_ulong) as *mut decNumber;
                    if allocbufa.is_null() {
                        // use the allocated space
                        // hopeless -- abandon
                        *status |= 0x10 as libc::c_int as libc::c_uint;
                        current_block_82 = 17372050596571538954;
                    } else {
                        a = allocbufa;
                        current_block_82 = 15512526488502093901;
                    }
                } else { current_block_82 = 15512526488502093901; }
                match current_block_82 {
                    17372050596571538954 => { }
                    _ => {
                        pp = p + (*rhs).digits;
                        needbytes =
                            (::std::mem::size_of::<decNumber>() as
                                 libc::c_ulong).wrapping_add((((if (if pp <
                                                                           16
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        16 as
                                                                            libc::c_int
                                                                    } else {
                                                                        pp
                                                                    }) <=
                                                                       49 as
                                                                           libc::c_int
                                                                   {
                                                                    d2utable[(if pp
                                                                                     <
                                                                                     16
                                                                                         as
                                                                                         libc::c_int
                                                                                 {
                                                                                  16
                                                                                      as
                                                                                      libc::c_int
                                                                              } else {
                                                                                  pp
                                                                              })
                                                                                 as
                                                                                 usize]
                                                                        as
                                                                        libc::c_int
                                                                } else {
                                                                    ((if pp <
                                                                             16
                                                                                 as
                                                                                 libc::c_int
                                                                         {
                                                                          16
                                                                              as
                                                                              libc::c_int
                                                                      } else {
                                                                          pp
                                                                      }) +
                                                                         3 as
                                                                             libc::c_int
                                                                         -
                                                                         1 as
                                                                             libc::c_int)
                                                                        /
                                                                        3 as
                                                                            libc::c_int
                                                                }) -
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                                                                  as
                                                                                                  libc::c_ulong))
                                as uint32_t;
                        if needbytes as libc::c_ulong >
                               ::std::mem::size_of::<[decNumber; 5]>() as
                                   libc::c_ulong {
                            // need malloc space
                            allocbufb =
                                malloc(needbytes as libc::c_ulong) as
                                    *mut decNumber;
                            if allocbufb.is_null() {
                                // use the allocated space
                                // hopeless -- abandon
                                *status |=
                                    0x10 as libc::c_int as libc::c_uint;
                                current_block_82 = 17372050596571538954;
                            } else {
                                b = allocbufb;
                                current_block_82 = 13678349939556791712;
                            }
                        } else { current_block_82 = 13678349939556791712; }
                        match current_block_82 {
                            17372050596571538954 => { }
                            _ => {
                                // Prepare an initial estimate in acc. Calculate this by
    // considering the coefficient of x to be a normalized fraction,
    // f, with the decimal point at far left and multiplied by
    // 10**r.  Then, rhs=f*10**r and 0.1<=f<1, and
    //   ln(x) = ln(f) + ln(10)*r
    // Get the initial estimate for ln(f) from a small lookup
    // table (see above) indexed by the first two digits of f,
    // truncated.
                                decContextDefault(&mut aset,
                                                  64 as
                                                      libc::c_int); // 16-digit extended
                                r =
                                    (*rhs).exponent +
                                        (*rhs).digits; // 'normalised' exponent
                                decNumberFromInt32(a, r); // a=r
                                decNumberFromInt32(b,
                                                   2302585 as
                                                       libc::c_int); // b=ln(10) (2.302585)
                                (*b).exponent = -(6 as libc::c_int); //  ..
                                decMultiplyOp(a, a, b, &mut aset,
                                              &mut ignore); // a=a*b
                                // now get top two digits of rhs into b by simple truncate and
    // force to integer
                                residue = 0 as libc::c_int; // (no residue)
                                aset.digits =
                                    2 as libc::c_int; // copy & shorten
                                aset.round = DEC_ROUND_DOWN; // make integer
                                decCopyFit(b, rhs, &mut aset, &mut residue,
                                           &mut ignore); // [cannot fail]
                                (*b).exponent =
                                    0 as libc::c_int; // adjust single-digit b
                                t = decGetInt(b); // look up ln(b)
                                if t < 10 as libc::c_int {
                                    t =
                                        (t << 1 as libc::c_int) +
                                            (t << 3 as libc::c_int)
                                } // b=ln(b) coefficient
                                t =
                                    LNnn[(t - 10 as libc::c_int) as usize] as
                                        int32_t; // set exponent
                                decNumberFromInt32(b,
                                                   t >>
                                                       2 as
                                                           libc::c_int); // ln(0.10)->ln(0.99) always -ve
                                (*b).exponent =
                                    -(t & 3 as libc::c_int) -
                                        3 as libc::c_int; // restore
                                (*b).bits =
                                    0x80 as libc::c_int as uint8_t; // acc=a+b
                                aset.digits = 16 as libc::c_int;
                                aset.round = DEC_ROUND_HALF_EVEN;
                                decAddOp(a, a, b, &mut aset,
                                         0 as libc::c_int as uint8_t,
                                         &mut ignore);
                                // the initial estimate is now in a, with up to 4 digits correct.
    // When rhs is at or near Nmax the estimate will be low, so we
    // will approach it from below, avoiding overflow when calling exp.
                                decNumberZero(&mut numone); // constant 1 for adjustment
                                *numone.lsu.as_mut_ptr() =
                                    1 as libc::c_int as uint16_t;
                                // accumulator bounds are as requested (could underflow, but
    // cannot overflow)
                                aset.emax = (*set).emax; // no concrete format
                                aset.emin = (*set).emin;
                                aset.clamp = 0 as libc::c_int as uint8_t;
                                // set up a context to be used for the multiply and subtract
                                bset = aset; // use double bounds for the
                                bset.emax =
                                    999999 as libc::c_int *
                                        2 as
                                            libc::c_int; // adjustment calculation
                                        // [see decExpOp call below]
                                bset.emin =
                                    -(999999 as libc::c_int) *
                                        2 as libc::c_int;
                                // for each iteration double the number of digits to calculate,
    // up to a maximum of p
                                pp = 9 as libc::c_int; // initial precision
                                // [initially 9 as then the sequence starts 7+2, 16+2, and
    // 34+2, which is ideal for standard-sized numbers]
                                aset.digits = pp; // working context
                                bset.digits =
                                    pp + (*rhs).digits; // wider context
                                loop  {
                                    (*a).bits =
                                        ((*a).bits as libc::c_int ^
                                             0x80 as libc::c_int) as
                                            uint8_t; // Newton's iteration
                                    // iterate
                                    // calculate the adjustment (exp(-a)*x-1) into b.  This is a
      // catastrophic subtraction but it really is the difference
      // from 1 that is of interest.
      // Use the internal entry point to Exp as it allows the double
      // range for calculating exp(-a) when a is the tiniest subnormal.
                                    // make -a
                                    decExpOp(b, a, &mut bset,
                                             &mut ignore); // b=exp(-a)
                                    (*a).bits =
                                        ((*a).bits as libc::c_int ^
                                             0x80 as libc::c_int) as
                                            uint8_t; // restore sign of a
                                    // now multiply by rhs and subtract 1, at the wider precision
                                    decMultiplyOp(b, b, rhs, &mut bset,
                                                  &mut ignore); // b=b*rhs
                                    decAddOp(b, b, &mut numone, &mut bset,
                                             0x80 as libc::c_int as uint8_t,
                                             &mut ignore); // b=b-1
                                    // the iteration ends when the adjustment cannot affect the
      // result by >=0.5 ulp (at the requested digits), which
      // is when its value is smaller than the accumulator by
      // set->digits+1 digits (or it is zero) -- this is a looser
      // requirement than for Exp because all that happens to the
      // accumulator after this is the final rounding (but note that
      // there must also be full precision in a, or a=0).
                                    if *(*b).lsu.as_mut_ptr() as libc::c_int
                                           == 0 as libc::c_int &&
                                           (*b).digits == 1 as libc::c_int &&
                                           (*b).bits as libc::c_int &
                                               (0x40 as libc::c_int |
                                                    0x20 as libc::c_int |
                                                    0x10 as libc::c_int) ==
                                               0 as libc::c_int ||
                                           (*a).digits + (*a).exponent >=
                                               (*b).digits + (*b).exponent +
                                                   (*set).digits +
                                                   1 as libc::c_int {
                                        if (*a).digits == p {
                                            break ; // rhs=1 ?
                                        } // no, inexact
                                        if *(*a).lsu.as_mut_ptr() as
                                               libc::c_int == 0 as libc::c_int
                                               &&
                                               (*a).digits == 1 as libc::c_int
                                               &&
                                               (*a).bits as libc::c_int &
                                                   (0x40 as libc::c_int |
                                                        0x20 as libc::c_int |
                                                        0x10 as libc::c_int)
                                                   == 0 as libc::c_int {
                                            decCompareOp(&mut cmp, rhs,
                                                         &mut numone,
                                                         &mut aset,
                                                         0x1 as libc::c_int as
                                                             uint8_t,
                                                         &mut ignore); // yes, exact 0
                                            if *cmp.lsu.as_mut_ptr().offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                   as libc::c_int ==
                                                   0 as libc::c_int {
                                                (*a).exponent =
                                                    0 as libc::c_int
                                            } else {
                                                *status |=
                                                    (0x20 as libc::c_int |
                                                         0x800 as libc::c_int)
                                                        as libc::c_uint
                                            }
                                            break ;
                                        } else if *(*b).lsu.as_mut_ptr() as
                                                      libc::c_int ==
                                                      0 as libc::c_int &&
                                                      (*b).digits ==
                                                          1 as libc::c_int &&
                                                      (*b).bits as libc::c_int
                                                          &
                                                          (0x40 as libc::c_int
                                                               |
                                                               0x20 as
                                                                   libc::c_int
                                                               |
                                                               0x10 as
                                                                   libc::c_int)
                                                          == 0 as libc::c_int
                                         {
                                            (*b).exponent = (*a).exponent - p
                                        }
                                    }
                                    // force padding if adjustment has gone to 0 before full length
                                    // not done yet ...
                                    decAddOp(a, a, b, &mut aset,
                                             0 as libc::c_int as uint8_t,
                                             &mut ignore); // a=a+b for next estimate
                                    if pp == p {
                                        continue ; // precision is at maximum
                                    }
                                    // lengthen the next calculation
                                    pp =
                                        pp *
                                            2 as
                                                libc::c_int; // double precision
                                    if pp > p { pp = p } // clamp to maximum
                                    aset.digits = pp; // working context
                                    bset.digits = pp + (*rhs).digits
                                }
                                // Copy and round the result to res
                                residue =
                                    1 as
                                        libc::c_int; // indicate dirt to right
                                if *(*a).lsu.as_mut_ptr() as libc::c_int ==
                                       0 as libc::c_int &&
                                       (*a).digits == 1 as libc::c_int &&
                                       (*a).bits as libc::c_int &
                                           (0x40 as libc::c_int |
                                                0x20 as libc::c_int |
                                                0x10 as libc::c_int) ==
                                           0 as libc::c_int {
                                    residue = 0 as libc::c_int
                                } // .. unless underflowed to 0
                                aset.digits =
                                    (*set).digits; // [use default rounding]
                                decCopyFit(res, a, &mut aset, &mut residue,
                                           status); // copy & shorten
                                decFinalize(res, set, &mut residue, status);
                            }
                        }
                    }
                }
            }
        }
    }
    // cleanup/set flags
    // protect allocated storage
    if !allocbufa.is_null() {
        free(allocbufa as *mut libc::c_void); // drop any storage used
    } // ..
    if !allocbufb.is_null() { free(allocbufb as *mut libc::c_void); }
    return res;
}
// [status is handled by caller]
// decLnOp
/* ------------------------------------------------------------------ */
/* decQuantizeOp  -- force exponent to requested value                */
/*                                                                    */
/*   This computes C = op(A, B), where op adjusts the coefficient     */
/*   of C (by rounding or shifting) such that the exponent (-scale)   */
/*   of C has the value B or matches the exponent of B.               */
/*   The numerical value of C will equal A, except for the effects of */
/*   any rounding that occurred.                                      */
/*                                                                    */
/*   res is C, the result.  C may be A or B                           */
/*   lhs is A, the number to adjust                                   */
/*   rhs is B, the requested exponent                                 */
/*   set is the context                                               */
/*   quant is 1 for quantize or 0 for rescale                         */
/*   status is the status accumulator (this can be called without     */
/*          risk of control loss)                                     */
/*                                                                    */
/* C must have space for set->digits digits.                          */
/*                                                                    */
/* Unless there is an error or the result is infinite, the exponent   */
/* after the operation is guaranteed to be that requested.            */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decQuantizeOp(mut res: *mut decNumber,
                                   mut lhs: *const decNumber,
                                   mut rhs: *const decNumber,
                                   mut set: *mut decContext,
                                   mut quant: uint8_t,
                                   mut status: *mut uint32_t)
 -> *mut decNumber {
    let mut inrhs: *const decNumber = rhs; // save original rhs
    let mut reqdigits: int32_t = (*set).digits; // requested DIGITS
    let mut reqexp: int32_t = 0; // requested exponent [-scale]
    let mut residue: int32_t = 0 as libc::c_int; // rounding residue
    let mut etiny: int32_t = (*set).emin - (reqdigits - 1 as libc::c_int);
    let mut current_block_32: u64;
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 {
        // NaNs get usual processing
        if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) &
               (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
            decNaNs(res, lhs, rhs, set, status); // [nop if in place]
        } else if ((*lhs).bits as libc::c_int ^ (*rhs).bits as libc::c_int) &
                      0x40 as libc::c_int != 0 {
            *status |= 0x80 as libc::c_int as libc::c_uint
        } else {
            // one infinity but not both is bad
            // both infinity: return lhs
            decNumberCopy(res, lhs);
        }
    } else {
        // set requested exponent
        if quant != 0 {
            reqexp = (*inrhs).exponent
        } else { // quantize -- match exponents
            // rescale -- use value of rhs
            // Original rhs must be an integer that fits and is in range,
      // which could be from -1999999997 to +999999999, thanks to
      // subnormals
            reqexp = decGetInt(inrhs)
            // [cannot fail]
        }
        if reqexp == 0x80000000 as libc::c_uint as int32_t ||
               reqexp == 0x80000003 as libc::c_uint as int32_t ||
               reqexp == 0x80000002 as libc::c_uint as int32_t ||
               reqexp < etiny || reqexp > (*set).emax {
            // > emax
            *status |= 0x80 as libc::c_int as libc::c_uint
        } else {
            // the RHS has been processed, so it can be overwritten now if necessary
            if *(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                   (*lhs).digits == 1 as libc::c_int &&
                   (*lhs).bits as libc::c_int &
                       (0x40 as libc::c_int | 0x20 as libc::c_int |
                            0x10 as libc::c_int) == 0 as libc::c_int
               { // non-zero
                // zero coefficient unchanged
                decNumberCopy(res, lhs);
                (*res).exponent = reqexp;
                current_block_32 = 12199444798915819164; // [nop if in place]
                // .. just set exponent
            } else {
                // non-zero lhs
                let mut adjust: int32_t =
                    reqexp - (*lhs).exponent; // digit adjustment needed
                // if adjusted coefficient will definitely not fit, give up now
                if (*lhs).digits - adjust > reqdigits {
                    *status |=
                        0x80 as libc::c_int as libc::c_uint; // increase
                    current_block_32 = 10150597327160359210;
                } else if adjust > 0 as libc::c_int {
                    let mut workset: decContext =
                        decContext{digits: 0,
                                   emax: 0,
                                   emin: 0,
                                   round: DEC_ROUND_CEILING,
                                   traps: 0,
                                   status: 0,
                                   clamp: 0,};
                    // increasing exponent
                    // this will decrease the length of the coefficient by adjust
        // digits, and must round as it does so
                    // work
                    workset = *set; // clone rounding, etc.
                    workset.digits =
                        (*lhs).digits - adjust; // set requested length
                    // [note that the latter can be <1, here]
                    decCopyFit(res, lhs, &mut workset, &mut residue,
                               status); // fit to result
                    decApplyRound(res, &mut workset, residue,
                                  status); // .. and round
                    residue = 0 as libc::c_int; // [used]
                    // If just rounded a 999s case, exponent will be off by one;
        // adjust back (after checking space), if so.
                    if (*res).exponent > reqexp {
                        // re-check needed, e.g., for quantize(0.9999, 0.001) under
          // set->digits==3
                        if (*res).digits == reqdigits {
                            // cannot shift by 1
                            *status &=
                                !(0x20 as libc::c_int | 0x800 as libc::c_int)
                                    as libc::c_uint; // [clean these]
                            *status |=
                                0x80 as libc::c_int as libc::c_uint; // shift
                            current_block_32 = 10150597327160359210;
                        } else {
                            (*res).digits =
                                decShiftToMost((*res).lsu.as_mut_ptr(),
                                               (*res).digits,
                                               1 as libc::c_int);
                            (*res).exponent -= 1;
                            current_block_32 = 12199444798915819164;
                        }
                        // (re)adjust the exponent.
                    } else { current_block_32 = 12199444798915819164; }
                } else {
                    /* adjust<=0 */
                    // decreasing or = exponent
                    // this will increase the length of the coefficient by -adjust
        // digits, by adding zero or more trailing zeros; this is
        // already checked for fit, above
                    decNumberCopy(res, lhs); // [it will fit]
                    if adjust < 0 as libc::c_int {
                        (*res).digits =
                            decShiftToMost((*res).lsu.as_mut_ptr(),
                                           (*res).digits, -adjust);
                        (*res).exponent += adjust
                        // adjust the exponent
                    }
                    current_block_32 = 12199444798915819164;
                }
                // if padding needed (adjust<0), add it now...
                // decrease
            }
            match current_block_32 {
                10150597327160359210 => { }
                _ =>
                // Check for overflow [do not use Finalize in this case, as an
    // overflow here is a "don't fit" situation]
                {
                    if (*res).exponent >
                           (*set).emax - (*res).digits + 1 as libc::c_int {
                        // too big
                        *status |= 0x80 as libc::c_int as libc::c_uint
                    } else {
                        decFinalize(res, set, &mut residue, status);
                        *status &=
                            !(0x2000 as libc::c_int) as
                                libc::c_uint // set subnormal flags
                        // suppress Underflow [as per 754]
                    }
                }
            }
        }
    } // end protected
    return res;
}
// protect allocated storage
// [following code does not require input rounding]
// Handle special values
// decQuantizeOp
/* ------------------------------------------------------------------ */
/* decCompareOp -- compare, min, or max two Numbers                   */
/*                                                                    */
/*   This computes C = A ? B and carries out one of four operations:  */
/*     COMPARE    -- returns the signum (as a number) giving the      */
/*                   result of a comparison unless one or both        */
/*                   operands is a NaN (in which case a NaN results)  */
/*     COMPSIG    -- as COMPARE except that a quiet NaN raises        */
/*                   Invalid operation.                               */
/*     COMPMAX    -- returns the larger of the operands, using the    */
/*                   754 maxnum operation                             */
/*     COMPMAXMAG -- ditto, comparing absolute values                 */
/*     COMPMIN    -- the 754 minnum operation                         */
/*     COMPMINMAG -- ditto, comparing absolute values                 */
/*     COMTOTAL   -- returns the signum (as a number) giving the      */
/*                   result of a comparison using 754 total ordering  */
/*                                                                    */
/*   res is C, the result.  C may be A and/or B (e.g., X=X?X)         */
/*   lhs is A                                                         */
/*   rhs is B                                                         */
/*   set is the context                                               */
/*   op  is the operation flag                                        */
/*   status is the usual accumulator                                  */
/*                                                                    */
/* C must have space for one digit for COMPARE or set->digits for     */
/* COMPMAX, COMPMIN, COMPMAXMAG, or COMPMINMAG.                       */
/* ------------------------------------------------------------------ */
/* The emphasis here is on speed for common cases, and avoiding       */
/* coefficient comparison if possible.                                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decCompareOp(mut res: *mut decNumber,
                                  mut lhs: *const decNumber,
                                  mut rhs: *const decNumber,
                                  mut set: *mut decContext, mut op: uint8_t,
                                  mut status: *mut uint32_t)
 -> *mut decNumber {
    let mut result: int32_t = 0 as libc::c_int; // default result value
    let mut merged: uint8_t = 0; // work
    let mut current_block_26: u64;
    if op as libc::c_int == 0x4 as libc::c_int {
        // total ordering
        if ((*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                0 as libc::c_int) as libc::c_int &
               !((*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                     0 as libc::c_int) as libc::c_int != 0 {
            result = -(1 as libc::c_int);
            current_block_26 = 9007357115414505193;
        } else if !((*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                        0 as libc::c_int) as libc::c_int &
                      ((*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                           0 as libc::c_int) as libc::c_int != 0 {
            result = 1 as libc::c_int;
            current_block_26 = 9007357115414505193;
        } else { current_block_26 = 13513818773234778473; }
    } else { current_block_26 = 13513818773234778473; }
    match current_block_26 {
        13513818773234778473 => {
            // sign matters
            // protect allocated storage
            // [following code does not require input rounding]
            // If total ordering then handle differing signs 'up front'
            // handle NaNs specially; let infinities drop through
    // This assumes sNaN (even just one) leads to NaN.
            merged =
                (((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
                     (0x10 as libc::c_int | 0x20 as libc::c_int)) as uint8_t;
            if merged != 0 {
                // a NaN bit set
                if op as libc::c_int == 0x1 as libc::c_int { // max or min
                    // result will be NaN
                    current_block_26 = 1118134448028020070;
                } else if op as libc::c_int == 0x6 as libc::c_int {
                    // treat qNaN as sNaN
                    *status |=
                        (0x80 as libc::c_int | 0x40000000 as libc::c_int) as
                            libc::c_uint; // total order
                    current_block_26 = 1118134448028020070;
                } else if op as libc::c_int == 0x4 as libc::c_int {
                    // total ordering, always finite
                    // signs are known to be the same; compute the ordering here
        // as if the signs are both positive, then invert for negatives
                    if !((*lhs).bits as libc::c_int &
                             (0x20 as libc::c_int | 0x10 as libc::c_int) !=
                             0 as libc::c_int) {
                        result = -(1 as libc::c_int)
                    } else if !((*rhs).bits as libc::c_int &
                                    (0x20 as libc::c_int |
                                         0x10 as libc::c_int) !=
                                    0 as libc::c_int) {
                        result = 1 as libc::c_int
                    } else if (*lhs).bits as libc::c_int & 0x10 as libc::c_int
                                  != 0 as libc::c_int &&
                                  (*rhs).bits as libc::c_int &
                                      0x20 as libc::c_int != 0 as libc::c_int
                     {
                        result = -(1 as libc::c_int)
                    } else if (*lhs).bits as libc::c_int & 0x20 as libc::c_int
                                  != 0 as libc::c_int &&
                                  (*rhs).bits as libc::c_int &
                                      0x10 as libc::c_int != 0 as libc::c_int
                     {
                        result = 1 as libc::c_int
                    } else { // both same NaNs
                        // here if both NaNs
                        // both NaN or both sNaN
                        // now it just depends on the payload
                        result =
                            decUnitCompare((*lhs).lsu.as_ptr(),
                                           if (*lhs).digits <=
                                                  49 as libc::c_int {
                                               d2utable[(*lhs).digits as
                                                            usize] as
                                                   libc::c_int
                                           } else {
                                               ((*lhs).digits +
                                                    3 as libc::c_int -
                                                    1 as libc::c_int) /
                                                   3 as libc::c_int
                                           }, (*rhs).lsu.as_ptr(),
                                           if (*rhs).digits <=
                                                  49 as libc::c_int {
                                               d2utable[(*rhs).digits as
                                                            usize] as
                                                   libc::c_int
                                           } else {
                                               ((*rhs).digits +
                                                    3 as libc::c_int -
                                                    1 as libc::c_int) /
                                                   3 as libc::c_int
                                           }, 0 as libc::c_int)
                        // [Error not possible, as these are 'aligned']
                    }
                    if (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                           0 as libc::c_int {
                        result = -result
                    }
                    current_block_26 = 9007357115414505193;
                } else if merged as libc::c_int & 0x10 as libc::c_int != 0 {
                    current_block_26 = 1118134448028020070;
                } else if !((*lhs).bits as libc::c_int &
                                (0x20 as libc::c_int | 0x10 as libc::c_int) !=
                                0 as libc::c_int) ||
                              !((*rhs).bits as libc::c_int &
                                    (0x20 as libc::c_int |
                                         0x10 as libc::c_int) !=
                                    0 as libc::c_int) {
                    // here if MIN or MAX and one or two quiet NaNs
                    // min or max -- 754 rules ignore single NaN
                    // just one NaN; force choice to be the non-NaN operand
                    op = 0x2 as libc::c_int as uint8_t; // pick lhs
                    if (*lhs).bits as libc::c_int & 0x20 as libc::c_int != 0 {
                        result = -(1 as libc::c_int)
                    } else { result = 1 as libc::c_int } // pick rhs
                    current_block_26 = 9007357115414505193;
                } else { current_block_26 = 1118134448028020070; }
                match current_block_26 {
                    9007357115414505193 => { }
                    _ =>
                    // sNaN -> qNaN
                    {
                        op =
                            0x5 as libc::c_int as uint8_t; // use special path
                        decNaNs(res, lhs, rhs, set, status); // propagate NaN
                    }
                }
            } else if op as libc::c_int == 0x7 as libc::c_int ||
                          op as libc::c_int == 0x8 as libc::c_int {
                result = decCompare(lhs, rhs, 1 as libc::c_int as uint8_t)
            } else {
                result = decCompare(lhs, rhs, 0 as libc::c_int as uint8_t)
            }
        }
        _ => { }
    }
    if result == 0x80000000 as libc::c_uint as int32_t {
        // have numbers
        *status |= 0x10 as libc::c_int as libc::c_uint
    } else if op as libc::c_int == 0x1 as libc::c_int ||
                  op as libc::c_int == 0x6 as libc::c_int ||
                  op as libc::c_int == 0x4 as libc::c_int { // rare
        // returning signum
        if op as libc::c_int == 0x4 as libc::c_int &&
               result == 0 as libc::c_int { // total-order by exponent
            // operands are numerically equal or same NaN (and same sign,
        // tested first); if identical, leave result 0
            if (*lhs).exponent != (*rhs).exponent {
                if (*lhs).exponent < (*rhs).exponent {
                    result = -(1 as libc::c_int)
                } else { result = 1 as libc::c_int }
                if (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                       0 as libc::c_int {
                    result = -result
                }
            }
            // lexp!=rexp
        } // [always a valid result]
        decNumberZero(res);
        if result != 0 as libc::c_int {
            // must be -1 or +1
            *(*res).lsu.as_mut_ptr() = 1 as libc::c_int as uint16_t;
            if result < 0 as libc::c_int {
                (*res).bits = 0x80 as libc::c_int as uint8_t
            }
        }
    } else if !(op as libc::c_int == 0x5 as libc::c_int) {
        // MAX or MIN, non-NaN result
        let mut residue: int32_t = 0 as libc::c_int; // rounding accumulator
        // choose the operand for the result
        let mut choice: *const decNumber =
            0 as *const decNumber; // numerically equal
        if result == 0 as libc::c_int {
            // operands are numerically equal
            // choose according to sign then exponent (see 754)
            let mut slhs: uint8_t =
                ((*lhs).bits as libc::c_int & 0x80 as libc::c_int) as uint8_t;
            let mut srhs: uint8_t =
                ((*rhs).bits as libc::c_int & 0x80 as libc::c_int) as uint8_t;
            if slhs as libc::c_int != srhs as libc::c_int {
                // signs differ
                if slhs != 0 {
                    result = -(1 as libc::c_int)
                } else { result = 1 as libc::c_int } // rhs is max
                // lhs is max
            } else if slhs as libc::c_int != 0 && srhs as libc::c_int != 0 {
                // both negative
                if (*lhs).exponent < (*rhs).exponent {
                    result = 1 as libc::c_int
                } else { result = -(1 as libc::c_int) }
                // [if equal, use lhs, technically identical]
            } else if (*lhs).exponent > (*rhs).exponent {
                result = 1 as libc::c_int
            } else { result = -(1 as libc::c_int) }
        }
        // both positive
        // [ditto]
        // here result will be non-0; reverse if looking for MIN
        if op as libc::c_int == 0x3 as libc::c_int ||
               op as libc::c_int == 0x8 as libc::c_int {
            result = -result
        } // choose
        choice = if result > 0 as libc::c_int { lhs } else { rhs };
        // copy chosen to result, rounding if need be
        decCopyFit(res, choice, set, &mut residue, status);
        decFinalize(res, set, &mut residue, status);
    }
    return res;
}
// decCompareOp
/* ------------------------------------------------------------------ */
/* decCompare -- compare two decNumbers by numerical value            */
/*                                                                    */
/*  This routine compares A ? B without altering them.                */
/*                                                                    */
/*  Arg1 is A, a decNumber which is not a NaN                         */
/*  Arg2 is B, a decNumber which is not a NaN                         */
/*  Arg3 is 1 for a sign-independent compare, 0 otherwise             */
/*                                                                    */
/*  returns -1, 0, or 1 for A<B, A==B, or A>B, or BADINT if failure   */
/*  (the only possible failure is an allocation error)                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decCompare(mut lhs: *const decNumber,
                                mut rhs: *const decNumber, mut abs_0: uint8_t)
 -> int32_t {
    let mut result: int32_t = 0; // result value
    let mut sigr: int32_t = 0; // rhs signum
    let mut compare: int32_t = 0; // work
    result = 1 as libc::c_int; // assume signum(lhs)
    if *(*lhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
           (*lhs).digits == 1 as libc::c_int &&
           (*lhs).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        result = 0 as libc::c_int
    } // LHS wins or both 0
    if abs_0 != 0 {
        if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
               (*rhs).digits == 1 as libc::c_int &&
               (*rhs).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) == 0 as libc::c_int {
            return result
        }
        // LHS is 0; RHS wins
        // [here, both non-zero, result=1]
        if result == 0 as libc::c_int { return -(1 as libc::c_int) }
    } else {
        // RHS is non-zero
        // signs matter
        if result != 0 &&
               (*lhs).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
            result = -(1 as libc::c_int)
        }
        // both 0
        sigr = 1 as libc::c_int; // compute signum(rhs)
        if *(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
               (*rhs).digits == 1 as libc::c_int &&
               (*rhs).bits as libc::c_int &
                   (0x40 as libc::c_int | 0x20 as libc::c_int |
                        0x10 as libc::c_int) == 0 as libc::c_int {
            sigr = 0 as libc::c_int
        } else if (*rhs).bits as libc::c_int & 0x80 as libc::c_int !=
                      0 as libc::c_int {
            sigr = -(1 as libc::c_int)
        } // L > R, return 1
        if result > sigr { return 1 as libc::c_int } // L < R, return -1
        if result < sigr { return -(1 as libc::c_int) }
        if result == 0 as libc::c_int { return 0 as libc::c_int }
    }
    // signums are the same; both are non-zero
    if ((*lhs).bits as libc::c_int | (*rhs).bits as libc::c_int) &
           0x40 as libc::c_int != 0 {
        // one or more infinities
        if (*rhs).bits as libc::c_int & 0x40 as libc::c_int !=
               0 as libc::c_int {
            if (*lhs).bits as libc::c_int & 0x40 as libc::c_int !=
                   0 as libc::c_int {
                result = 0 as libc::c_int
            } else { result = -result } // both infinite
            // only rhs infinite
        }
        return result
    }
    // must compare the coefficients, allowing for exponents
    if (*lhs).exponent > (*rhs).exponent {
        // LHS exponent larger
        // swap sides, and sign
        let mut temp: *const decNumber = lhs; // comparison succeeded
        lhs = rhs;
        rhs = temp;
        result = -result
    }
    compare =
        decUnitCompare((*lhs).lsu.as_ptr(),
                       if (*lhs).digits <= 49 as libc::c_int {
                           d2utable[(*lhs).digits as usize] as libc::c_int
                       } else {
                           ((*lhs).digits + 3 as libc::c_int -
                                1 as libc::c_int) / 3 as libc::c_int
                       }, (*rhs).lsu.as_ptr(),
                       if (*rhs).digits <= 49 as libc::c_int {
                           d2utable[(*rhs).digits as usize] as libc::c_int
                       } else {
                           ((*rhs).digits + 3 as libc::c_int -
                                1 as libc::c_int) / 3 as libc::c_int
                       }, (*rhs).exponent - (*lhs).exponent);
    if compare != 0x80000000 as libc::c_uint as int32_t { compare *= result }
    return compare;
}
// decCompare
/* ------------------------------------------------------------------ */
/* decUnitCompare -- compare two >=0 integers in Unit arrays          */
/*                                                                    */
/*  This routine compares A ? B*10**E where A and B are unit arrays   */
/*  A is a plain integer                                              */
/*  B has an exponent of E (which must be non-negative)               */
/*                                                                    */
/*  Arg1 is A first Unit (lsu)                                        */
/*  Arg2 is A length in Units                                         */
/*  Arg3 is B first Unit (lsu)                                        */
/*  Arg4 is B length in Units                                         */
/*  Arg5 is E (0 if the units are aligned)                            */
/*                                                                    */
/*  returns -1, 0, or 1 for A<B, A==B, or A>B, or BADINT if failure   */
/*  (the only possible failure is an allocation error, which can      */
/*  only occur if E!=0)                                               */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decUnitCompare(mut a: *const uint16_t,
                                    mut alength: int32_t,
                                    mut b: *const uint16_t,
                                    mut blength: int32_t, mut exp: int32_t)
 -> int32_t {
    let mut acc: *mut uint16_t = 0 as *mut uint16_t; // accumulator for result
    let mut accbuff: [uint16_t; 25] = [0; 25]; // local buffer
    let mut allocacc: *mut uint16_t =
        0 as *mut uint16_t; // -> allocated acc buffer, iff allocated
    let mut accunits: int32_t = 0; // units in use or needed for acc
    let mut need: int32_t = 0; // work
    let mut l: *const uint16_t = 0 as *const uint16_t; // ..
    let mut r: *const uint16_t = 0 as *const uint16_t; // aligned
    let mut u: *const uint16_t = 0 as *const uint16_t;
    let mut expunits: int32_t = 0;
    let mut exprem: int32_t = 0;
    let mut result: int32_t = 0;
    if exp == 0 as libc::c_int {
        // aligned; fastpath
        if alength > blength { return 1 as libc::c_int }
        if alength < blength { return -(1 as libc::c_int) }
        // all units match
        l = a.offset(alength as isize).offset(-(1 as libc::c_int as isize));
        r = b.offset(alength as isize).offset(-(1 as libc::c_int as isize));
        while l >= a {
            if *l as libc::c_int > *r as libc::c_int {
                return 1 as libc::c_int
            }
            if (*l as libc::c_int) < *r as libc::c_int {
                return -(1 as libc::c_int)
            }
            l = l.offset(-1);
            r = r.offset(-1)
        }
        return 0 as libc::c_int
    }
    // same number of units in both -- need unit-by-unit compare
    // Unaligned.  If one is >1 unit longer than the other, padded
  // approximately, then can return easily
    if alength >
           blength +
               (if exp <= 49 as libc::c_int {
                    d2utable[exp as usize] as libc::c_int
                } else {
                    (exp + 3 as libc::c_int - 1 as libc::c_int) /
                        3 as libc::c_int
                }) {
        return 1 as libc::c_int
    }
    if (alength + 1 as libc::c_int) <
           blength +
               (if exp <= 49 as libc::c_int {
                    d2utable[exp as usize] as libc::c_int
                } else {
                    (exp + 3 as libc::c_int - 1 as libc::c_int) /
                        3 as libc::c_int
                }) {
        return -(1 as libc::c_int)
    }
    // Need to do a real subtract.  For this, a result buffer is needed
  // even though only the sign is of interest.  Its length needs
  // to be the larger of alength and padded blength, +2
    need =
        blength +
            (if exp <= 49 as libc::c_int {
                 d2utable[exp as usize] as libc::c_int
             } else {
                 (exp + 3 as libc::c_int - 1 as libc::c_int) /
                     3 as libc::c_int
             }); // maximum real length of B
    if need < alength { need = alength } // assume use local buffer
    need += 2 as libc::c_int; // hopeless -- abandon
    acc = accbuff.as_mut_ptr();
    if (need as
            libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>() as
                                            libc::c_ulong) >
           ::std::mem::size_of::<[uint16_t; 25]>() as libc::c_ulong {
        allocacc =
            malloc((need as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                        as libc::c_ulong)) as
                *mut uint16_t;
        if allocacc.is_null() { return 0x80000000 as libc::c_uint as int32_t }
        acc = allocacc
    }
    // Calculate units and remainder from exponent.
    expunits = exp / 3 as libc::c_int;
    exprem = exp % 3 as libc::c_int;
    // subtract [A+B*(-m)]
    accunits =
        decUnitAddSub(a, alength, b, blength, expunits, acc,
                      -(DECPOWERS[exprem as usize] as int32_t));
    // [UnitAddSub result may have leading zeros, even on zero]
    if accunits < 0 as libc::c_int {
        result = -(1 as libc::c_int)
    } else { // negative result
        // non-negative result
        // check units of the result before freeing any storage
        u = acc;
        while u <
                  acc.offset(accunits as
                                 isize).offset(-(1 as libc::c_int as isize))
                  && *u as libc::c_int == 0 as libc::c_int {
            u = u.offset(1)
        }
        result =
            if *u as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else { 1 as libc::c_int }
    }
    // clean up and return the result
    if !allocacc.is_null() {
        free(allocacc as *mut libc::c_void); // drop any storage used
    }
    return result;
}
// decUnitCompare
/* ------------------------------------------------------------------ */
/* decUnitAddSub -- add or subtract two >=0 integers in Unit arrays   */
/*                                                                    */
/*  This routine performs the calculation:                            */
/*                                                                    */
/*  C=A+(B*M)                                                         */
/*                                                                    */
/*  Where M is in the range -DECDPUNMAX through +DECDPUNMAX.          */
/*                                                                    */
/*  A may be shorter or longer than B.                                */
/*                                                                    */
/*  Leading zeros are not removed after a calculation.  The result is */
/*  either the same length as the longer of A and B (adding any       */
/*  shift), or one Unit longer than that (if a Unit carry occurred).  */
/*                                                                    */
/*  A and B content are not altered unless C is also A or B.          */
/*  C may be the same array as A or B, but only if no zero padding is */
/*  requested (that is, C may be B only if bshift==0).                */
/*  C is filled from the lsu; only those units necessary to complete  */
/*  the calculation are referenced.                                   */
/*                                                                    */
/*  Arg1 is A first Unit (lsu)                                        */
/*  Arg2 is A length in Units                                         */
/*  Arg3 is B first Unit (lsu)                                        */
/*  Arg4 is B length in Units                                         */
/*  Arg5 is B shift in Units  (>=0; pads with 0 units if positive)    */
/*  Arg6 is C first Unit (lsu)                                        */
/*  Arg7 is M, the multiplier                                         */
/*                                                                    */
/*  returns the count of Units written to C, which will be non-zero   */
/*  and negated if the result is negative.  That is, the sign of the  */
/*  returned Int is the sign of the result (positive for zero) and    */
/*  the absolute value of the Int is the count of Units.              */
/*                                                                    */
/*  It is the caller's responsibility to make sure that C size is     */
/*  safe, allowing space if necessary for a one-Unit carry.           */
/*                                                                    */
/*  This routine is severely performance-critical; *any* change here  */
/*  must be measured (timed) to assure no performance degradation.    */
/*  In particular, trickery here tends to be counter-productive, as   */
/*  increased complexity of code hurts register optimizations on      */
/*  register-poor architectures.  Avoiding divisions is nearly        */
/*  always a Good Idea, however.                                      */
/*                                                                    */
/* Special thanks to Rick McGuire (IBM Cambridge, MA) and Dave Clark  */
/* (IBM Warwick, UK) for some of the ideas used in this routine.      */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decUnitAddSub(mut a: *const uint16_t,
                                   mut alength: int32_t,
                                   mut b: *const uint16_t,
                                   mut blength: int32_t, mut bshift: int32_t,
                                   mut c: *mut uint16_t, mut m: int32_t)
 -> int32_t {
    let mut alsu: *const uint16_t = a; // A lsu [need to remember it]
    let mut clsu: *mut uint16_t = c; // C ditto
    let mut minC: *mut uint16_t = 0 as *mut uint16_t; // low water mark for C
    let mut maxC: *mut uint16_t = 0 as *mut uint16_t; // high water mark for C
    let mut carry: int32_t =
        0 as libc::c_int; // carry integer (could be Long)
    let mut add: int32_t = 0; // work
    // myriadal, millenary, etc.
    let mut est: int32_t = 0; // estimated quotient
    maxC = c.offset(alength as isize); // A is usually the longer
    minC = c.offset(blength as isize); // .. and B the shorter
    if bshift != 0 as libc::c_int {
        // B is shifted; low As copy across
        minC = minC.offset(bshift as isize);
        // if in place [common], skip copy unless there's a gap [rare]
        if a == c && bshift <= alength {
            c = c.offset(bshift as isize);
            a = a.offset(bshift as isize)
        } else {
            while c < clsu.offset(bshift as isize) {
                // copy needed
                if a < alsu.offset(alength as isize) {
                    *c = *a
                } else { *c = 0 as libc::c_int as uint16_t }
                a = a.offset(1);
                c = c.offset(1)
            }
        }
    }
    if minC > maxC {
        // swap
        let mut hold: *mut uint16_t = minC;
        minC = maxC;
        maxC = hold
    }
    // For speed, do the addition as two loops; the first where both A
  // and B contribute, and the second (if necessary) where only one or
  // other of the numbers contribute.
  // Carry handling is the same (i.e., duplicated) in each case.
    while c < minC {
        carry += *a as libc::c_int; // c
        a = a.offset(1); // [special-casing m=1/-1
        carry += *b as int32_t * m; // here is not a win]
        b = b.offset(1);
        // here carry is new Unit of digits; it could be +ve or -ve
        if carry as uint32_t <= 999 as libc::c_int as libc::c_uint {
            // fastpath 0-DECDPUNMAX
            *c = carry as uint16_t;
            carry = 0 as libc::c_int
        } else if carry >= 0 as libc::c_int {
            est =
                ((carry as uint32_t >>
                      3 as
                          libc::c_int).wrapping_mul(16777 as libc::c_int as
                                                        libc::c_uint) >>
                     21 as libc::c_int) as int32_t;
            // use divide-by-multiply
            *c =
                (carry - est * (999 as libc::c_int + 1 as libc::c_int)) as
                    uint16_t; // remainder
            carry = est; // likely quotient [99%]
            if !((*c as libc::c_int) < 999 as libc::c_int + 1 as libc::c_int)
               {
                carry += 1; // estimate was correct
                *c =
                    (*c as libc::c_int -
                         (999 as libc::c_int + 1 as libc::c_int)) as uint16_t
            }
        } else {
            // negative case
            carry =
                carry +
                    (999 as libc::c_int + 1 as libc::c_int) *
                        (999 as libc::c_int +
                             1 as libc::c_int); // make positive
            est =
                ((carry as uint32_t >>
                      3 as
                          libc::c_int).wrapping_mul(16777 as libc::c_int as
                                                        libc::c_uint) >>
                     21 as libc::c_int) as int32_t; // correctly negative
            *c =
                (carry - est * (999 as libc::c_int + 1 as libc::c_int)) as
                    uint16_t; // was OK
            carry = est - (999 as libc::c_int + 1 as libc::c_int);
            if !((*c as libc::c_int) < 999 as libc::c_int + 1 as libc::c_int)
               {
                carry += 1;
                *c =
                    (*c as libc::c_int -
                         (999 as libc::c_int + 1 as libc::c_int)) as uint16_t
            }
        }
        c = c.offset(1)
    }
    // now may have one or other to complete
  // [pretest to avoid loop setup/shutdown]
    if c < maxC {
        while c < maxC {
            if a < alsu.offset(alength as isize) { // c
                // still in A
                carry += *a as libc::c_int;
                a = a.offset(1)
            } else {
                // inside B
                carry += *b as int32_t * m;
                b = b.offset(1)
            }
            // here carry is new Unit of digits; it could be +ve or -ve and
    // magnitude up to DECDPUNMAX squared
            if carry as uint32_t <= 999 as libc::c_int as libc::c_uint {
                // fastpath 0-DECDPUNMAX
                *c = carry as uint16_t;
                carry = 0 as libc::c_int
            } else if carry >= 0 as libc::c_int {
                est =
                    ((carry as uint32_t >>
                          3 as
                              libc::c_int).wrapping_mul(16777 as libc::c_int
                                                            as libc::c_uint)
                         >> 21 as libc::c_int) as int32_t;
                // result for this unit is negative or >DECDPUNMAX
                // use divide-by-multiply
                *c =
                    (carry - est * (999 as libc::c_int + 1 as libc::c_int)) as
                        uint16_t; // remainder
                carry = est; // likely quotient [99%]
                if !((*c as libc::c_int) <
                         999 as libc::c_int + 1 as libc::c_int) {
                    carry += 1; // estimate was correct
                    *c =
                        (*c as libc::c_int -
                             (999 as libc::c_int + 1 as libc::c_int)) as
                            uint16_t
                }
            } else {
                // negative case
                carry =
                    carry +
                        (999 as libc::c_int + 1 as libc::c_int) *
                            (999 as libc::c_int +
                                 1 as libc::c_int); // make positive
                est =
                    ((carry as uint32_t >>
                          3 as
                              libc::c_int).wrapping_mul(16777 as libc::c_int
                                                            as libc::c_uint)
                         >> 21 as libc::c_int) as
                        int32_t; // correctly negative
                *c =
                    (carry - est * (999 as libc::c_int + 1 as libc::c_int)) as
                        uint16_t; // was OK
                carry = est - (999 as libc::c_int + 1 as libc::c_int);
                if !((*c as libc::c_int) <
                         999 as libc::c_int + 1 as libc::c_int) {
                    carry += 1;
                    *c =
                        (*c as libc::c_int -
                             (999 as libc::c_int + 1 as libc::c_int)) as
                            uint16_t
                }
            }
            c = c.offset(1)
        }
    }
    // OK, all A and B processed; might still have carry or borrow
  // return number of Units in the result, negated if a borrow
    if carry == 0 as libc::c_int {
        return c.wrapping_offset_from(clsu) as libc::c_long as int32_t
    } // no carry, so no more to do
    if carry > 0 as libc::c_int {
        // positive carry
        *c = carry as uint16_t; // place as new unit
        c = c.offset(1); // ..
        return c.wrapping_offset_from(clsu) as libc::c_long as int32_t
    }
    // -ve carry: it's a borrow; complement needed
    add = 1 as libc::c_int; // temporary carry...
    c = clsu;
    while c < maxC {
        add = 999 as libc::c_int + add - *c as libc::c_int;
        if add <= 999 as libc::c_int {
            *c = add as uint16_t;
            add = 0 as libc::c_int
        } else { *c = 0 as libc::c_int as uint16_t; add = 1 as libc::c_int }
        c = c.offset(1)
    }
    // add an extra unit iff it would be non-zero
    if add - carry - 1 as libc::c_int != 0 as libc::c_int {
        *c = (add - carry - 1 as libc::c_int) as uint16_t;
        c = c.offset(1)
        // interesting, include it
    }
    return clsu.wrapping_offset_from(c) as libc::c_long as int32_t;
    // -ve result indicates borrowed
}
// decUnitAddSub
/* ------------------------------------------------------------------ */
/* decTrim -- trim trailing zeros or normalize                        */
/*                                                                    */
/*   dn is the number to trim or normalize                            */
/*   set is the context to use to check for clamp                     */
/*   all is 1 to remove all trailing zeros, 0 for just fraction ones  */
/*   noclamp is 1 to unconditional (unclamped) trim                   */
/*   dropped returns the number of discarded trailing zeros           */
/*   returns dn                                                       */
/*                                                                    */
/* If clamp is set in the context then the number of zeros trimmed    */
/* may be limited if the exponent is high.                            */
/* All fields are updated as required.  This is a utility operation,  */
/* so special values are unchanged and no error is possible.          */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decTrim(mut dn: *mut decNumber, mut set: *mut decContext,
                             mut all: uint8_t, mut noclamp: uint8_t,
                             mut dropped: *mut int32_t) -> *mut decNumber {
    let mut d: int32_t = 0; // work
    let mut exp: int32_t = 0; // ..
    let mut cut: uint32_t = 0; // -> current Unit
    let mut up: *mut uint16_t = 0 as *mut uint16_t; // assume no zeros dropped
    *dropped = 0 as libc::c_int; // .. or odd
    if (*dn).bits as libc::c_int &
           (0x40 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int)
           != 0 ||
           *(*dn).lsu.as_mut_ptr() as libc::c_int & 0x1 as libc::c_int != 0 {
        return dn
    }
    if *(*dn).lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        // .. or 0
        (*dn).exponent = 0 as libc::c_int; // (sign is preserved)
        return dn
    }
    // have a finite number which is even
    exp = (*dn).exponent; // digit (1-DECDPUN) in Unit
    cut = 1 as libc::c_int as uint32_t; // -> current Unit
    up = (*dn).lsu.as_mut_ptr(); // d
    d = 0 as libc::c_int;
    while d < (*dn).digits - 1 as libc::c_int {
        // [don't strip the final digit]
        // slice by powers
        let mut quot: uint32_t =
            (*up as uint32_t >> cut).wrapping_mul(multies[cut as usize]) >>
                17 as libc::c_int; // found non-0 digit
        if (*up as
                libc::c_uint).wrapping_sub(quot.wrapping_mul(DECPOWERS[cut as
                                                                           usize]))
               != 0 as libc::c_int as libc::c_uint {
            break ;
        }
        // have a trailing 0
        if all == 0 {
            // trimming
            // [if exp>0 then all trailing 0s are significant for trim]
            if exp <= 0 as libc::c_int {
                // if digit might be significant
                if exp == 0 as libc::c_int {
                    break ; // then quit
                }
                exp += 1
                // next digit might be significant
            }
        } // next power
        cut = cut.wrapping_add(1);
        if cut > 3 as libc::c_int as libc::c_uint {
            // need new Unit
            up = up.offset(1); // none to drop
            cut = 1 as libc::c_int as uint32_t
        }
        d += 1
    }
    if d == 0 as libc::c_int { return dn }
    // may need to limit drop if clamping
    if (*set).clamp as libc::c_int != 0 && noclamp == 0 {
        let mut maxd: int32_t =
            (*set).emax - (*set).digits + 1 as libc::c_int -
                (*dn).exponent; // nothing possible
        if maxd <= 0 as libc::c_int { return dn }
        if d > maxd { d = maxd }
    }
    // effect the drop
    decShiftToLeast((*dn).lsu.as_mut_ptr(),
                    if (*dn).digits <= 49 as libc::c_int {
                        d2utable[(*dn).digits as usize] as libc::c_int
                    } else {
                        ((*dn).digits + 3 as libc::c_int - 1 as libc::c_int) /
                            3 as libc::c_int
                    }, d); // maintain numerical value
    (*dn).exponent += d; // new length
    (*dn).digits -= d; // report the count
    *dropped = d;
    return dn;
}
// decTrim
/* ------------------------------------------------------------------ */
/* decReverse -- reverse a Unit array in place                        */
/*                                                                    */
/*   ulo    is the start of the array                                 */
/*   uhi    is the end of the array (highest Unit to include)         */
/*                                                                    */
/* The units ulo through uhi are reversed in place (if the number     */
/* of units is odd, the middle one is untouched).  Note that the      */
/* digit(s) in each unit are unaffected.                              */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decReverse(mut ulo: *mut uint16_t,
                                mut uhi: *mut uint16_t) {
    let mut temp: uint16_t = 0;
    while ulo < uhi {
        temp = *ulo;
        *ulo = *uhi;
        *uhi = temp;
        ulo = ulo.offset(1);
        uhi = uhi.offset(-1)
    };
}
// decReverse
/* ------------------------------------------------------------------ */
/* decShiftToMost -- shift digits in array towards most significant   */
/*                                                                    */
/*   uar    is the array                                              */
/*   digits is the count of digits in use in the array                */
/*   shift  is the number of zeros to pad with (least significant);   */
/*     it must be zero or positive                                    */
/*                                                                    */
/*   returns the new length of the integer in the array, in digits    */
/*                                                                    */
/* No overflow is permitted (that is, the uar array must be known to  */
/* be large enough to hold the result, after shifting).               */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decShiftToMost(mut uar: *mut uint16_t,
                                    mut digits: int32_t, mut shift: int32_t)
 -> int32_t {
    let mut target: *mut uint16_t = 0 as *mut uint16_t; // work
    let mut source: *mut uint16_t = 0 as *mut uint16_t; // odd 0's to add
    let mut first: *mut uint16_t = 0 as *mut uint16_t; // work
    let mut cut: int32_t = 0; // [fastpath] nothing to do
    let mut next: uint32_t = 0;
    if shift == 0 as libc::c_int { return digits }
    if digits + shift <= 3 as libc::c_int {
        // [fastpath] single-unit case
        *uar =
            (*uar as libc::c_uint).wrapping_mul(DECPOWERS[shift as usize]) as
                uint16_t; // all paths
        return digits + shift
    } // where msu comes from
    next = 0 as libc::c_int as uint32_t; // where upper part of first cut goes
    source =
        uar.offset((if digits <= 49 as libc::c_int {
                        d2utable[digits as usize] as libc::c_int
                    } else {
                        (digits + 3 as libc::c_int - 1 as libc::c_int) /
                            3 as libc::c_int
                    }) as
                       isize).offset(-(1 as libc::c_int as
                                           isize)); // where to slice
    target =
        source.offset((if shift <= 49 as libc::c_int {
                           d2utable[shift as usize] as libc::c_int
                       } else {
                           (shift + 3 as libc::c_int - 1 as libc::c_int) /
                               3 as libc::c_int
                       }) as isize); // shift-move
    cut =
        3 as libc::c_int -
            (shift -
                 ((if shift <= 49 as libc::c_int {
                       d2utable[shift as usize] as libc::c_int
                   } else {
                       (shift + 3 as libc::c_int - 1 as libc::c_int) /
                           3 as libc::c_int
                   }) - 1 as libc::c_int) * 3 as libc::c_int);
    if cut == 0 as libc::c_int {
        // unit-boundary case
        while source >= uar {
            *target = *source; // where msu of source will end up
            source = source.offset(-1);
            target = target.offset(-1)
        }
    } else {
        first =
            uar.offset((if digits + shift <= 49 as libc::c_int {
                            d2utable[(digits + shift) as usize] as libc::c_int
                        } else {
                            (digits + shift + 3 as libc::c_int -
                                 1 as libc::c_int) / 3 as libc::c_int
                        }) as isize).offset(-(1 as libc::c_int as isize));
        while source >= uar {
            // split the source Unit and accumulate remainder for next
            let mut quot: uint32_t =
                (*source as uint32_t >>
                     cut).wrapping_mul(multies[cut as usize]) >>
                    17 as libc::c_int; // write to target iff valid
            let mut rem: uint32_t =
                (*source as
                     libc::c_uint).wrapping_sub(quot.wrapping_mul(DECPOWERS[cut
                                                                                as
                                                                                usize]));
            next =
                (next as libc::c_uint).wrapping_add(quot) as uint32_t as
                    uint32_t;
            if target <= first { *target = next as uint16_t }
            next =
                rem.wrapping_mul(DECPOWERS[(3 as libc::c_int - cut) as
                                               usize]);
            source = source.offset(-1);
            target = target.offset(-1)
        }
    }
    // propagate any partial unit to one below and clear the rest
    while target >= uar {
        *target = next as uint16_t;
        next = 0 as libc::c_int as uint32_t;
        target = target.offset(-1)
    }
    return digits + shift;
}
// decShiftToMost
/* ------------------------------------------------------------------ */
/* decShiftToLeast -- shift digits in array towards least significant */
/*                                                                    */
/*   uar   is the array                                               */
/*   units is length of the array, in units                           */
/*   shift is the number of digits to remove from the lsu end; it     */
/*     must be zero or positive and <= than units*DECDPUN.            */
/*                                                                    */
/*   returns the new length of the integer in the array, in units     */
/*                                                                    */
/* Removed digits are discarded (lost).  Units not required to hold   */
/* the final result are unchanged.                                    */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decShiftToLeast(mut uar: *mut uint16_t,
                                     mut units: int32_t, mut shift: int32_t)
 -> int32_t {
    let mut target: *mut uint16_t = 0 as *mut uint16_t; // work
    let mut up: *mut uint16_t = 0 as *mut uint16_t; // work
    let mut cut: int32_t = 0; // for division
    let mut count: int32_t = 0; // [fastpath] nothing to do
    let mut quot: int32_t = 0;
    let mut rem: int32_t = 0;
    if shift == 0 as libc::c_int { return units }
    if shift == units * 3 as libc::c_int {
        // [fastpath] little to do
        *uar = 0 as libc::c_int as uint16_t;
        return 1 as libc::c_int // all digits cleared gives zero
        // leaves just the one
    } // both paths
    target = uar;
    cut =
        shift -
            ((if shift <= 49 as libc::c_int {
                  d2utable[shift as usize] as libc::c_int
              } else {
                  (shift + 3 as libc::c_int - 1 as libc::c_int) /
                      3 as libc::c_int
              }) - 1 as libc::c_int) * 3 as libc::c_int;
    if cut == 3 as libc::c_int {
        // unit-boundary case; easy
        up =
            uar.offset((if shift <= 49 as libc::c_int {
                            d2utable[shift as usize] as libc::c_int
                        } else {
                            (shift + 3 as libc::c_int - 1 as libc::c_int) /
                                3 as libc::c_int
                        }) as isize);
        while up < uar.offset(units as isize) {
            *target = *up;
            target = target.offset(1);
            up = up.offset(1)
        }
        return target.wrapping_offset_from(uar) as libc::c_long as int32_t
    }
    // messier
    up =
        uar.offset((if shift - cut <= 49 as libc::c_int {
                        d2utable[(shift - cut) as usize] as libc::c_int
                    } else {
                        (shift - cut + 3 as libc::c_int - 1 as libc::c_int) /
                            3 as libc::c_int
                    }) as isize); // source; correct to whole Units
    count = units * 3 as libc::c_int - shift; // the maximum new length
    quot =
        ((*up as uint32_t >> cut).wrapping_mul(multies[cut as usize]) >>
             17 as libc::c_int) as int32_t;
    loop  {
        *target = quot as uint16_t;
        count -= 3 as libc::c_int - cut;
        if count <= 0 as libc::c_int { break ; }
        up = up.offset(1);
        quot = *up as int32_t;
        quot =
            ((quot as uint32_t >> cut).wrapping_mul(multies[cut as usize]) >>
                 17 as libc::c_int) as int32_t;
        rem =
            (*up as
                 libc::c_uint).wrapping_sub((quot as
                                                 libc::c_uint).wrapping_mul(DECPOWERS[cut
                                                                                          as
                                                                                          usize]))
                as int32_t;
        *target =
            (*target as
                 libc::c_uint).wrapping_add((rem as
                                                 libc::c_uint).wrapping_mul(DECPOWERS[(3
                                                                                           as
                                                                                           libc::c_int
                                                                                           -
                                                                                           cut)
                                                                                          as
                                                                                          usize]))
                as uint16_t;
        count -= cut;
        if count <= 0 as libc::c_int { break ; }
        target = target.offset(1)
    }
    return (target.wrapping_offset_from(uar) as libc::c_long +
                1 as libc::c_int as libc::c_long) as int32_t;
}
// decShiftToLeast
/* ------------------------------------------------------------------ */
/* decCopyFit -- copy a number, truncating the coefficient if needed  */
/*                                                                    */
/*   dest is the target decNumber                                     */
/*   src  is the source decNumber                                     */
/*   set is the context [used for length (digits) and rounding mode]  */
/*   residue is the residue accumulator                               */
/*   status contains the current status to be updated                 */
/*                                                                    */
/* (dest==src is allowed and will be a no-op if fits)                 */
/* All fields are updated as required.                                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decCopyFit(mut dest: *mut decNumber,
                                mut src: *const decNumber,
                                mut set: *mut decContext,
                                mut residue: *mut int32_t,
                                mut status: *mut uint32_t) {
    (*dest).bits = (*src).bits;
    (*dest).exponent = (*src).exponent;
    decSetCoeff(dest, set, (*src).lsu.as_ptr(), (*src).digits, residue,
                status);
}
// decCopyFit
/* ------------------------------------------------------------------ */
/* decSetCoeff -- set the coefficient of a number                     */
/*                                                                    */
/*   dn    is the number whose coefficient array is to be set.        */
/*         It must have space for set->digits digits                  */
/*   set   is the context [for size]                                  */
/*   lsu   -> lsu of the source coefficient [may be dn->lsu]          */
/*   len   is digits in the source coefficient [may be dn->digits]    */
/*   residue is the residue accumulator.  This has values as in       */
/*         decApplyRound, and will be unchanged unless the            */
/*         target size is less than len.  In this case, the           */
/*         coefficient is truncated and the residue is updated to     */
/*         reflect the previous residue and the dropped digits.       */
/*   status is the status accumulator, as usual                       */
/*                                                                    */
/* The coefficient may already be in the number, or it can be an      */
/* external intermediate array.  If it is in the number, lsu must ==  */
/* dn->lsu and len must == dn->digits.                                */
/*                                                                    */
/* Note that the coefficient length (len) may be < set->digits, and   */
/* in this case this merely copies the coefficient (or is a no-op     */
/* if dn->lsu==lsu).                                                  */
/*                                                                    */
/* Note also that (only internally, from decQuantizeOp and            */
/* decSetSubnormal) the value of set->digits may be less than one,    */
/* indicating a round to left.  This routine handles that case        */
/* correctly; caller ensures space.                                   */
/*                                                                    */
/* dn->digits, dn->lsu (and as required), and dn->exponent are        */
/* updated as necessary.   dn->bits (sign) is unchanged.              */
/*                                                                    */
/* DEC_Rounded status is set if any digits are discarded.             */
/* DEC_Inexact status is set if any non-zero digits are discarded, or */
/*                       incoming residue was non-0 (implies rounded) */
/* ------------------------------------------------------------------ */
// mapping array: maps 0-9 to canonical residues, so that a residue
// can be adjusted in the range [-1, +1] and achieve correct rounding
//                             0  1  2  3  4  5  6  7  8  9
static mut resmap: [uint8_t; 10] =
    [0 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     3 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     3 as libc::c_int as uint8_t, 5 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t];
unsafe extern "C" fn decSetCoeff(mut dn: *mut decNumber,
                                 mut set: *mut decContext,
                                 mut lsu: *const uint16_t, mut len: int32_t,
                                 mut residue: *mut int32_t,
                                 mut status: *mut uint32_t) {
    let mut discard: int32_t = 0; // number of digits to discard
    let mut cut: uint32_t = 0; // cut point in Unit
    let mut up: *const uint16_t = 0 as *const uint16_t; // work
    let mut target: *mut uint16_t = 0 as *mut uint16_t; // ..
    let mut count: int32_t = 0; // ..
    let mut temp: uint32_t = 0; // ..
    discard = len - (*set).digits; // digits to discard
    if discard <= 0 as libc::c_int {
        // no digits are being discarded
        if (*dn).lsu.as_mut_ptr() != lsu as *mut uint16_t {
            // copy needed
            // copy the coefficient array to the result number; no shift needed
            count = len;
            up = lsu;
            target = (*dn).lsu.as_mut_ptr();
            while count > 0 as libc::c_int {
                *target = *up; // avoids D2U
                target = target.offset(1);
                up = up.offset(1);
                count -= 3 as libc::c_int
            }
            (*dn).digits = len
            // set the new length
        }
        // dn->exponent and residue are unchanged, record any inexactitude
        if *residue != 0 as libc::c_int {
            *status |=
                (0x20 as libc::c_int | 0x800 as libc::c_int) as libc::c_uint
        }
        return
    }
    // some digits must be discarded ...
    (*dn).exponent += discard; // maintain numerical value
    *status |=
        0x800 as libc::c_int as libc::c_uint; // accumulate Rounded status
    if *residue > 1 as libc::c_int {
        *residue = 1 as libc::c_int
    } // previous residue now to right, so reduce
    if discard > len { // total discard
        // everything, +1, is being discarded
        // guard digit is 0
    // residue is all the number [NB could be all 0s]
        if *residue <= 0 as libc::c_int {
            // not already positive
            count = len; // avoids D2U
            up = lsu;
            while count > 0 as libc::c_int {
                if *up as libc::c_int != 0 as libc::c_int {
                    // found non-0
                    *residue = 1 as libc::c_int;
                    break ;
                    // no need to check any others
                } else {
                    up = up.offset(1); // record inexactitude
                    count -= 3 as libc::c_int
                }
            }
        } // coefficient will now be 0
        if *residue != 0 as libc::c_int {
            *status |= 0x20 as libc::c_int as libc::c_uint
        } // ..
        *(*dn).lsu.as_mut_ptr() = 0 as libc::c_int as uint16_t;
        (*dn).digits = 1 as libc::c_int;
        return
    }
    // partial discard [most common case]
  // here, at least the first (most significant) discarded digit exists
    // spin up the number, noting residue during the spin, until get to
  // the Unit with the first discarded digit.  When reach it, extract
  // it and remember its position
    count = 0 as libc::c_int; // up
    up = lsu; // full ones all checked
    loop  {
        count += 3 as libc::c_int;
        if count >= discard { break ; }
        if *up as libc::c_int != 0 as libc::c_int {
            *residue = 1 as libc::c_int
        }
        up = up.offset(1)
    }
    // here up -> Unit with first discarded digit
    cut =
        (discard - (count - 3 as libc::c_int) - 1 as libc::c_int) as
            uint32_t; // not unit boundary
    if cut == (3 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        let mut half: uint16_t =
            (DECPOWERS[3 as libc::c_int as usize] as uint16_t as libc::c_int
                 >> 1 as libc::c_int) as uint16_t; // unit-boundary case
        // unit-boundary case (fast)
        if *up as libc::c_int >= half as libc::c_int {
            if *up as libc::c_int > half as libc::c_int {
                *residue = 7 as libc::c_int
            } else { *residue += 5 as libc::c_int }
            // set residue directly
            // add sticky bit
        } else if *up as libc::c_int != 0 as libc::c_int {
            *residue = 3 as libc::c_int
        }
        if (*set).digits <= 0 as libc::c_int {
            // <half
            // [else is 0, leave as sticky bit]
            // special for Quantize/Subnormal :-(
            *(*dn).lsu.as_mut_ptr() = 0 as libc::c_int as uint16_t;
            (*dn).digits = 1 as libc::c_int // .. result is 0
            // ..
        } else {
            // shift to least
            count = (*set).digits; // now digits to end up with
            (*dn).digits = count; // set the new length
            up = up.offset(1); // move to next
            // on unit boundary, so shift-down copy loop is simple
            target = (*dn).lsu.as_mut_ptr();
            while count > 0 as libc::c_int {
                *target = *up;
                target = target.offset(1);
                up = up.offset(1);
                count -= 3 as libc::c_int
            }
        }
    } else {
        // discard digit is in low digit(s), and not top digit
        let mut discard1: uint32_t = 0; // first discarded digit
        let mut quot: uint32_t = 0; // for divisions
        let mut rem: uint32_t = 0; // is at bottom of unit
        if cut == 0 as libc::c_int as libc::c_uint {
            quot = *up as uint32_t
        } else {
            /* cut>0 */
            // it's not at bottom of unit
            quot =
                (*up as uint32_t >> cut).wrapping_mul(multies[cut as usize])
                    >> 17 as libc::c_int;
            rem =
                (*up as
                     libc::c_uint).wrapping_sub(quot.wrapping_mul(DECPOWERS[cut
                                                                                as
                                                                                usize]));
            if rem != 0 as libc::c_int as libc::c_uint {
                *residue = 1 as libc::c_int
            }
        }
        // discard digit is now at bottom of quot
        temp =
            quot.wrapping_mul(6554 as libc::c_int as libc::c_uint) >>
                16 as libc::c_int; // fast /10
        // Vowels algorithm here not a win (9 instructions)
        discard1 =
            quot.wrapping_sub((temp <<
                                   1 as
                                       libc::c_int).wrapping_add(temp <<
                                                                     3 as
                                                                         libc::c_int));
        quot = temp;
        // here, discard1 is the guard digit, and residue is everything
    // else [use mapping array to accumulate residue safely]
        *residue += resmap[discard1 as usize] as libc::c_int; // update cut
        cut = cut.wrapping_add(1);
        // here: up -> Unit of the array with bottom digit
    //       cut is the division point for each Unit
    //       quot holds the uncut high-order digits for the current unit
        if (*set).digits <= 0 as libc::c_int {
            // special for Quantize/Subnormal :-(
            *(*dn).lsu.as_mut_ptr() = 0 as libc::c_int as uint16_t;
            (*dn).digits = 1 as libc::c_int // .. result is 0
            // ..
        } else {
            // shift to least needed
            count = (*set).digits; // now digits to end up with
            // shift-copy loop
            (*dn).digits = count; // set the new length
            target = (*dn).lsu.as_mut_ptr();
            loop  {
                *target = quot as uint16_t;
                count =
                    (count as
                         libc::c_uint).wrapping_sub((3 as libc::c_int as
                                                         libc::c_uint).wrapping_sub(cut))
                        as int32_t as int32_t;
                if count <= 0 as libc::c_int { break ; }
                up = up.offset(1);
                quot = *up as uint32_t;
                quot =
                    (quot >> cut).wrapping_mul(multies[cut as usize]) >>
                        17 as libc::c_int;
                rem =
                    (*up as
                         libc::c_uint).wrapping_sub(quot.wrapping_mul(DECPOWERS[cut
                                                                                    as
                                                                                    usize]));
                *target =
                    (*target as
                         libc::c_uint).wrapping_add(rem.wrapping_mul(DECPOWERS[(3
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_sub(cut)
                                                                                   as
                                                                                   usize]))
                        as uint16_t;
                count =
                    (count as libc::c_uint).wrapping_sub(cut) as int32_t as
                        int32_t;
                if count <= 0 as libc::c_int { break ; }
                target = target.offset(1)
            }
        }
    }
    // shift-copy the coefficient array to the result number
    if *residue != 0 as libc::c_int {
        *status |= 0x20 as libc::c_int as libc::c_uint
    }; // record inexactitude
}
// decSetCoeff
/* ------------------------------------------------------------------ */
/* decApplyRound -- apply pending rounding to a number                */
/*                                                                    */
/*   dn    is the number, with space for set->digits digits           */
/*   set   is the context [for size and rounding mode]                */
/*   residue indicates pending rounding, being any accumulated        */
/*         guard and sticky information.  It may be:                  */
/*         6-9: rounding digit is >5                                  */
/*         5:   rounding digit is exactly half-way                    */
/*         1-4: rounding digit is <5 and >0                           */
/*         0:   the coefficient is exact                              */
/*        -1:   as 1, but the hidden digits are subtractive, that     */
/*              is, of the opposite sign to dn.  In this case the     */
/*              coefficient must be non-0.  This case occurs when     */
/*              subtracting a small number (which can be reduced to   */
/*              a sticky bit); see decAddOp.                          */
/*   status is the status accumulator, as usual                       */
/*                                                                    */
/* This routine applies rounding while keeping the length of the      */
/* coefficient constant.  The exponent and status are unchanged       */
/* except if:                                                         */
/*                                                                    */
/*   -- the coefficient was increased and is all nines (in which      */
/*      case Overflow could occur, and is handled directly here so    */
/*      the caller does not need to re-test for overflow)             */
/*                                                                    */
/*   -- the coefficient was decreased and becomes all nines (in which */
/*      case Underflow could occur, and is also handled directly).    */
/*                                                                    */
/* All fields in dn are updated as required.                          */
/*                                                                    */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decApplyRound(mut dn: *mut decNumber,
                                   mut set: *mut decContext,
                                   mut residue: int32_t,
                                   mut status: *mut uint32_t) {
    let mut bump: int32_t =
        0; // 1 if coefficient needs to be incremented
                              // -1 if coefficient needs to be decremented
    if residue == 0 as libc::c_int { return } // nothing to apply
    bump = 0 as libc::c_int; // assume a smooth ride
    // now decide whether, and how, to round, depending on mode
    match (*set).round as libc::c_uint {
        7 => { // switch
            // round zero or five up (for reround)
            // This is the same as DEC_ROUND_DOWN unless there is a
      // positive residue and the lsd of dn is 0 or 5, in which case
      // it is bumped; when residue is <0, the number is therefore
      // bumped down unless the final digit was 1 or 6 (in which
      // case it is bumped down and then up -- a no-op)
            let mut lsd5: int32_t =
                *(*dn).lsu.as_mut_ptr() as libc::c_int %
                    5 as libc::c_int; // get lsd and quintate
            if residue < 0 as libc::c_int && lsd5 != 1 as libc::c_int {
                bump = -(1 as libc::c_int)
            } else if residue > 0 as libc::c_int && lsd5 == 0 as libc::c_int {
                bump = 1 as libc::c_int
            }
        }
        5 => { if residue < 0 as libc::c_int { bump = -(1 as libc::c_int) } }
        4 => { // r-d
            // no change, except if negative residue
            if residue > 5 as libc::c_int { bump = 1 as libc::c_int }
        }
        3 => { // r-h-d
            if residue > 5 as libc::c_int { // r-h-e
                bump = 1 as libc::c_int
            } else if residue == 5 as libc::c_int { // >0.5 goes up
                // exactly 0.5000...
                // 0.5 goes up iff [new] lsd is odd
                if *(*dn).lsu.as_mut_ptr() as libc::c_int & 0x1 as libc::c_int
                       != 0 {
                    bump = 1 as libc::c_int
                }
            }
        }
        2 => { if residue >= 5 as libc::c_int { bump = 1 as libc::c_int } }
        1 => { // r-h-u
            if residue > 0 as libc::c_int { bump = 1 as libc::c_int }
        }
        0 => { // r-u
            if (*dn).bits as libc::c_int & 0x80 as libc::c_int !=
                   0 as libc::c_int {
                if residue < 0 as libc::c_int { bump = -(1 as libc::c_int) }
            } else if residue > 0 as libc::c_int { bump = 1 as libc::c_int }
        }
        6 => { // r-c
            // same as _UP for positive numbers, and as _DOWN for negatives
      // [negative residue cannot occur on 0]
            if !((*dn).bits as libc::c_int & 0x80 as libc::c_int !=
                     0 as libc::c_int) {
                if residue < 0 as libc::c_int { bump = -(1 as libc::c_int) }
            } else if residue > 0 as libc::c_int { bump = 1 as libc::c_int }
        }
        _ => { // r-f
            // same as _UP for negative numbers, and as _DOWN for positive
      // [negative residue cannot occur on 0]
            // e.g., DEC_ROUND_MAX
            *status |= 0x40 as libc::c_int as libc::c_uint
        }
    }
    // now bump the number, up or down, if need be
    if bump == 0 as libc::c_int { return } // no action required
    // Simply use decUnitAddSub unless bumping up and the number is
  // all nines.  In this special case set to 100... explicitly
  // and adjust the exponent by one (as otherwise could overflow
  // the array)
  // Similarly handle all-nines result if bumping down.
    if bump > 0 as libc::c_int { // bump<0
        let mut up: *mut uint16_t = 0 as *mut uint16_t; // work
        let mut count: uint32_t =
            (*dn).digits as uint32_t; // digits to be checked
        up = (*dn).lsu.as_mut_ptr();
        loop  {
            if count <= 3 as libc::c_int as libc::c_uint {
                // this is the last Unit (the msu)
                if *up as libc::c_uint !=
                       DECPOWERS[count as
                                     usize].wrapping_sub(1 as libc::c_int as
                                                             libc::c_uint) {
                    break ; // not still 9s
                }
                // here if it, too, is all nines
                *up =
                    DECPOWERS[count.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) as usize]
                        as uint16_t; // here 999 -> 100 etc.
                up =
                    up.offset(-(1 as libc::c_int as
                                    isize)); // others all to 0
                while up >= (*dn).lsu.as_mut_ptr() {
                    *up = 0 as libc::c_int as uint16_t; // and bump exponent
                    up = up.offset(-1)
                }
                (*dn).exponent += 1;
                // [which, very rarely, could cause Overflow...]
                if (*dn).exponent + (*dn).digits >
                       (*set).emax + 1 as libc::c_int {
                    decSetOverflow(dn, set, status);
                }
                return
                // done
            } else {
                // a full unit to check, with more to come
                if *up as libc::c_int != 999 as libc::c_int {
                    break ; // not still 9s
                }
                count =
                    (count as
                         libc::c_uint).wrapping_sub(3 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                up = up.offset(1)
            }
        }
    } else {
        // -1
        // here checking for a pre-bump of 1000... (leading 1, all
    // other digits zero)
        let mut up_0: *mut uint16_t = 0 as *mut uint16_t; // work
        let mut sup: *mut uint16_t =
            0 as *mut uint16_t; // digits to be checked
        let mut count_0: uint32_t = (*dn).digits as uint32_t;
        up_0 = (*dn).lsu.as_mut_ptr();
        loop  {
            if count_0 <= 3 as libc::c_int as libc::c_uint {
                // this is the last Unit (the msu)
                if *up_0 as libc::c_uint !=
                       DECPOWERS[count_0.wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) as
                                     usize] {
                    break ; // not 100..
                }
                // here if have the 1000... case
                sup = up_0; // save msu pointer
                *up_0 =
                    (DECPOWERS[count_0 as usize] as uint16_t as libc::c_int -
                         1 as libc::c_int) as
                        uint16_t; // here 100 in msu -> 999
                // others all to all-nines, too
                up_0 =
                    up_0.offset(-(1 as libc::c_int as
                                      isize)); // and bump exponent
                while up_0 >= (*dn).lsu.as_mut_ptr() {
                    *up_0 =
                        (DECPOWERS[3 as libc::c_int as usize] as uint16_t as
                             libc::c_int - 1 as libc::c_int) as uint16_t;
                    up_0 = up_0.offset(-1)
                }
                (*dn).exponent -= 1;
                // iff the number was at the subnormal boundary (exponent=etiny)
        // then the exponent is now out of range, so it will in fact get
        // clamped to etiny and the final 9 dropped.
        // printf(">> emin=%d exp=%d sdig=%d\n", set->emin,
        //        dn->exponent, set->digits);
                if (*dn).exponent + 1 as libc::c_int ==
                       (*set).emin - (*set).digits + 1 as libc::c_int {
                    if count_0 == 1 as libc::c_int as libc::c_uint &&
                           (*dn).digits == 1 as libc::c_int {
                        *sup = 0 as libc::c_int as uint16_t
                    } else { // here 9 -> 0[.9]
                        *sup =
                            (DECPOWERS[count_0.wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                           as usize] as uint16_t as
                                 libc::c_int - 1 as libc::c_int) as
                                uint16_t; // here 999.. in msu -> 99..
                        (*dn).digits -= 1
                    }
                    (*dn).exponent += 1;
                    *status |=
                        (0x2000 as libc::c_int | 0x1000 as libc::c_int |
                             0x20 as libc::c_int | 0x800 as libc::c_int) as
                            libc::c_uint
                }
                return
                // done
            } else {
                // a full unit to check, with more to come
                if *up_0 as libc::c_int != 0 as libc::c_int {
                    break ; // not still 0s
                }
                count_0 =
                    (count_0 as
                         libc::c_uint).wrapping_sub(3 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t;
                up_0 = up_0.offset(1)
            }
        }
    }
    // Actual bump needed.  Do it.
    decUnitAddSub((*dn).lsu.as_mut_ptr(),
                  if (*dn).digits <= 49 as libc::c_int {
                      d2utable[(*dn).digits as usize] as libc::c_int
                  } else {
                      ((*dn).digits + 3 as libc::c_int - 1 as libc::c_int) /
                          3 as libc::c_int
                  }, uarrone.as_mut_ptr(), 1 as libc::c_int, 0 as libc::c_int,
                  (*dn).lsu.as_mut_ptr(), bump);
}
// decApplyRound
/* ------------------------------------------------------------------ */
/* decFinalize -- final check, clamp, and round of a number           */
/*                                                                    */
/*   dn is the number                                                 */
/*   set is the context                                               */
/*   residue is the rounding accumulator (as in decApplyRound)        */
/*   status is the status accumulator                                 */
/*                                                                    */
/* This finishes off the current number by checking for subnormal     */
/* results, applying any pending rounding, checking for overflow,     */
/* and applying any clamping.                                         */
/* Underflow and overflow conditions are raised as appropriate.       */
/* All fields are updated as required.                                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decFinalize(mut dn: *mut decNumber,
                                 mut set: *mut decContext,
                                 mut residue: *mut int32_t,
                                 mut status: *mut uint32_t) {
    let mut shift: int32_t = 0; // shift needed if clamping
    let mut tinyexp: int32_t =
        (*set).emin - (*dn).digits +
            1 as libc::c_int; // precalculate subnormal boundary
    // Must be careful, here, when checking the exponent as the
  // adjusted exponent could overflow 31 bits [because it may already
  // be up to twice the expected].
    // First test for subnormal.  This must be done before any final
  // round as the result could be rounded to Nmin or 0.
    if (*dn).exponent <= tinyexp {
        // prefilter
        let mut comp: int32_t = 0;
        let mut nmin: decNumber =
            decNumber{digits: 0, exponent: 0, bits: 0, lsu: [0; 1],};
        // A very nasty case here is dn == Nmin and residue<0
        if (*dn).exponent < tinyexp {
            // Go handle subnormals; this will apply round if needed.
            decSetSubnormal(dn, set, residue, status);
            return
        }
        // Equals case: only subnormal if dn=Nmin and negative residue
        decNumberZero(&mut nmin); // (signless compare)
        *nmin.lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
            1 as libc::c_int as uint16_t;
        nmin.exponent = (*set).emin;
        comp = decCompare(dn, &mut nmin, 1 as libc::c_int as uint8_t);
        if comp == 0x80000000 as libc::c_uint as int32_t {
            // oops
            *status |= 0x10 as libc::c_int as libc::c_uint; // abandon...
            return
        }
        if *residue < 0 as libc::c_int && comp == 0 as libc::c_int {
            // neg residue and dn==Nmin
            decApplyRound(dn, set, *residue, status); // might force down
            decSetSubnormal(dn, set, residue, status);
            return
        }
    }
    // now apply any pending round (this could raise overflow).
    if *residue != 0 as libc::c_int {
        decApplyRound(dn, set, *residue, status);
    }
    // Check for overflow [redundant in the 'rare' case] or clamp
    if (*dn).exponent <= (*set).emax - (*set).digits + 1 as libc::c_int {
        return
    } // neither needed
    // here when might have an overflow or clamp to do
    if (*dn).exponent > (*set).emax - (*dn).digits + 1 as libc::c_int {
        // too big
        decSetOverflow(dn, set, status);
        return
    }
    // here when the result is normal but in clamp range
    if (*set).clamp == 0 { return }
    // here when need to apply the IEEE exponent clamp (fold-down)
    shift = (*dn).exponent - ((*set).emax - (*set).digits + 1 as libc::c_int);
    // shift coefficient (if non-zero)
    if !(*(*dn).lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
             (*dn).digits == 1 as libc::c_int &&
             (*dn).bits as libc::c_int &
                 (0x40 as libc::c_int | 0x20 as libc::c_int |
                      0x10 as libc::c_int) == 0 as libc::c_int) {
        (*dn).digits =
            decShiftToMost((*dn).lsu.as_mut_ptr(), (*dn).digits, shift)
    } // adjust the exponent to match
    (*dn).exponent -= shift; // and record the dirty deed
    *status |= 0x400 as libc::c_int as libc::c_uint;
}
// decFinalize
/* ------------------------------------------------------------------ */
/* decSetOverflow -- set number to proper overflow value              */
/*                                                                    */
/*   dn is the number (used for sign [only] and result)               */
/*   set is the context [used for the rounding mode, etc.]            */
/*   status contains the current status to be updated                 */
/*                                                                    */
/* This sets the sign of a number and sets its value to either        */
/* Infinity or the maximum finite value, depending on the sign of     */
/* dn and the rounding mode, following IEEE 754 rules.                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decSetOverflow(mut dn: *mut decNumber,
                                    mut set: *mut decContext,
                                    mut status: *mut uint32_t) {
    let mut needmax: uint8_t =
        0 as libc::c_int as uint8_t; // result is maximum finite value
    let mut sign: uint8_t =
        ((*dn).bits as libc::c_int & 0x80 as libc::c_int) as
            uint8_t; // clean and save sign bit
    if *(*dn).lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        // zero does not overflow magnitude
        let mut emax: int32_t = (*set).emax; // limit value
        if (*set).clamp != 0 {
            emax -= (*set).digits - 1 as libc::c_int
        } // lower if clamping
        if (*dn).exponent > emax {
            // clamp required
            (*dn).exponent = emax; // never Infinity
            *status |= 0x400 as libc::c_int as libc::c_uint
        }
        return
    }
    decNumberZero(dn);
    match (*set).round as libc::c_uint {
        5 => {
            needmax = 1 as libc::c_int as uint8_t
            // Infinity in all other cases
        }
        7 => { needmax = 1 as libc::c_int as uint8_t }
        0 => { // r-05
            // never Infinity
            if sign != 0 { needmax = 1 as libc::c_int as uint8_t }
        }
        6 => { // r-c
            // Infinity if non-negative
            if sign == 0 { needmax = 1 as libc::c_int as uint8_t }
        }
        _ => { }
    } // r-f
    // Infinity if negative
    if needmax != 0 {
        decSetMaxValue(dn, set);
        (*dn).bits = sign // Value is +/-Infinity
        // set sign
    } else {
        (*dn).bits = (sign as libc::c_int | 0x40 as libc::c_int) as uint8_t
    }
    *status |=
        (0x200 as libc::c_int | 0x20 as libc::c_int | 0x800 as libc::c_int) as
            libc::c_uint;
}
// decSetOverflow
/* ------------------------------------------------------------------ */
/* decSetMaxValue -- set number to +Nmax (maximum normal value)       */
/*                                                                    */
/*   dn is the number to set                                          */
/*   set is the context [used for digits and emax]                    */
/*                                                                    */
/* This sets the number to the maximum positive value.                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decSetMaxValue(mut dn: *mut decNumber,
                                    mut set: *mut decContext) {
    let mut up: *mut uint16_t = 0 as *mut uint16_t; // work
    let mut count: int32_t = (*set).digits; // nines to add
    (*dn).digits = count;
    // fill in all nines to set maximum value
    up = (*dn).lsu.as_mut_ptr(); // up
    loop  {
        if count > 3 as libc::c_int {
            *up = 999 as libc::c_int as uint16_t; // unit full o'nines
            count -= 3 as libc::c_int;
            up = up.offset(1)
        } else {
            // this is the msu
            *up =
                DECPOWERS[count as
                              usize].wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
                    uint16_t;
            break ;
        }
        // filled those digits
    } // + sign
    (*dn).bits = 0 as libc::c_int as uint8_t;
    (*dn).exponent = (*set).emax - (*set).digits + 1 as libc::c_int;
}
// decSetMaxValue
/* ------------------------------------------------------------------ */
/* decSetSubnormal -- process value whose exponent is <Emin           */
/*                                                                    */
/*   dn is the number (used as input as well as output; it may have   */
/*         an allowed subnormal value, which may need to be rounded)  */
/*   set is the context [used for the rounding mode]                  */
/*   residue is any pending residue                                   */
/*   status contains the current status to be updated                 */
/*                                                                    */
/* If subset mode, set result to zero and set Underflow flags.        */
/*                                                                    */
/* Value may be zero with a low exponent; this does not set Subnormal */
/* but the exponent will be clamped to Etiny.                         */
/*                                                                    */
/* Otherwise ensure exponent is not out of range, and round as        */
/* necessary.  Underflow is set if the result is Inexact.             */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decSetSubnormal(mut dn: *mut decNumber,
                                     mut set: *mut decContext,
                                     mut residue: *mut int32_t,
                                     mut status: *mut uint32_t) {
    let mut workset: decContext =
        decContext{digits: 0,
                   emax: 0,
                   emin: 0,
                   round: DEC_ROUND_CEILING,
                   traps: 0,
                   status: 0,
                   clamp: 0,}; // work
    let mut etiny: int32_t = 0; // ..
    let mut adjust: int32_t = 0;
    // Full arithmetic -- allow subnormals, rounded to minimum exponent
  // (Etiny) if needed
    etiny =
        (*set).emin -
            ((*set).digits - 1 as libc::c_int); // smallest allowed exponent
    if *(*dn).lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        // value is zero
        // residue can never be non-zero here
        if (*dn).exponent < etiny {
            // clamp required
            (*dn).exponent = etiny; // have a non-zero subnormal
            *status |= 0x400 as libc::c_int as libc::c_uint
        } // calculate digits to remove
        return
    }
    *status |= 0x1000 as libc::c_int as libc::c_uint;
    adjust = etiny - (*dn).exponent;
    if adjust <= 0 as libc::c_int {
        // not out of range; unrounded
        // residue can never be non-zero here, except in the Nmin-residue
    // case (which is a subnormal result), so can take fast-path here
    // it may already be inexact (from setting the coefficient)
        if *status & 0x20 as libc::c_int as libc::c_uint != 0 {
            *status |= 0x2000 as libc::c_int as libc::c_uint
        }
        return
    }
    // adjust>0, so need to rescale the result so exponent becomes Etiny
  // [this code is similar to that in rescale]
    workset = *set; // clone rounding, etc.
    workset.digits = (*dn).digits - adjust; // set requested length
    workset.emin -= adjust; // and adjust emin to match
    // [note that the latter can be <1, here, similar to Rescale case]
    decSetCoeff(dn, &mut workset, (*dn).lsu.as_mut_ptr(), (*dn).digits,
                residue, status);
    decApplyRound(dn, &mut workset, *residue, status);
    // Use 754 default rule: Underflow is set iff Inexact
  // [independent of whether trapped]
    if *status & 0x20 as libc::c_int as libc::c_uint != 0 {
        *status |= 0x2000 as libc::c_int as libc::c_uint
    }
    // if rounded up a 999s case, exponent will be off by one; adjust
  // back if so [it will fit, because it was shortened earlier]
    if (*dn).exponent > etiny {
        (*dn).digits =
            decShiftToMost((*dn).lsu.as_mut_ptr(), (*dn).digits,
                           1 as libc::c_int);
        (*dn).exponent -= 1
        // (re)adjust the exponent.
    }
    // if rounded to zero, it is by definition clamped...
    if *(*dn).lsu.as_mut_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        *status |= 0x400 as libc::c_int as libc::c_uint
    };
}
// decSetSubnormal
/* ------------------------------------------------------------------ */
/* decCheckMath - check entry conditions for a math function          */
/*                                                                    */
/*   This checks the context and the operand                          */
/*                                                                    */
/*   rhs is the operand to check                                      */
/*   set is the context to check                                      */
/*   status is unchanged if both are good                             */
/*                                                                    */
/* returns non-zero if status is changed, 0 otherwise                 */
/*                                                                    */
/* Restrictions enforced:                                             */
/*                                                                    */
/*   digits, emax, and -emin in the context must be less than         */
/*   DEC_MAX_MATH (999999), and A must be within these bounds if      */
/*   non-zero.  Invalid_operation is set in the status if a           */
/*   restriction is violated.                                         */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decCheckMath(mut rhs: *const decNumber,
                                  mut set: *mut decContext,
                                  mut status: *mut uint32_t) -> uint32_t {
    let mut save: uint32_t = *status; // record
    if (*set).digits > 999999 as libc::c_int ||
           (*set).emax > 999999 as libc::c_int ||
           -(*set).emin > 999999 as libc::c_int {
        *status |= 0x40 as libc::c_int as libc::c_uint
    } else if ((*rhs).digits > 999999 as libc::c_int ||
                   (*rhs).exponent + (*rhs).digits >
                       999999 as libc::c_int + 1 as libc::c_int ||
                   (*rhs).exponent + (*rhs).digits <
                       2 as libc::c_int *
                           (1 as libc::c_int - 999999 as libc::c_int)) &&
                  !(*(*rhs).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
                        (*rhs).digits == 1 as libc::c_int &&
                        (*rhs).bits as libc::c_int &
                            (0x40 as libc::c_int | 0x20 as libc::c_int |
                                 0x10 as libc::c_int) == 0 as libc::c_int) {
        *status |= 0x80 as libc::c_int as libc::c_uint
    }
    return (*status != save) as libc::c_int as uint32_t;
}
// decCheckMath
/* ------------------------------------------------------------------ */
/* decGetInt -- get integer from a number                             */
/*                                                                    */
/*   dn is the number [which will not be altered]                     */
/*                                                                    */
/*   returns one of:                                                  */
/*     BADINT if there is a non-zero fraction                         */
/*     the converted integer                                          */
/*     BIGEVEN if the integer is even and magnitude > 2*10**9         */
/*     BIGODD  if the integer is odd  and magnitude > 2*10**9         */
/*                                                                    */
/* This checks and gets a whole number from the input decNumber.      */
/* The sign can be determined from dn by the caller when BIGEVEN or   */
/* BIGODD is returned.                                                */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decGetInt(mut dn: *const decNumber) -> int32_t {
    let mut theInt: int32_t = 0; // result accumulator
    let mut up: *const uint16_t = 0 as *const uint16_t; // work
    let mut got: int32_t = 0; // digits (real or not) processed
    let mut ilength: int32_t =
        (*dn).digits + (*dn).exponent; // integral length
    let mut neg: uint8_t =
        ((*dn).bits as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int)
            as libc::c_int as uint8_t; // 1 if -ve
    // The number must be an integer that fits in 10 digits
  // Assert, here, that 10 is enough for any rescale Etiny
    if *(*dn).lsu.as_ptr() as libc::c_int == 0 as libc::c_int &&
           (*dn).digits == 1 as libc::c_int &&
           (*dn).bits as libc::c_int &
               (0x40 as libc::c_int | 0x20 as libc::c_int |
                    0x10 as libc::c_int) == 0 as libc::c_int {
        return 0 as libc::c_int
    } // zeros are OK, with any exponent
    up = (*dn).lsu.as_ptr(); // ready for lsu
    theInt = 0 as libc::c_int; // ready to accumulate
    if (*dn).exponent >= 0 as libc::c_int {
        // relatively easy
        // no fractional part [usual]; allow for positive exponent
        got = (*dn).exponent
    } else {
        // -ve exponent; some fractional part to check and discard
        let mut count: int32_t = -(*dn).exponent; // digits to discard
        // spin up whole units until reach the Unit with the unit digit
        while count >= 3 as libc::c_int {
            if *up as libc::c_int != 0 as libc::c_int {
                return 0x80000000 as libc::c_uint as int32_t
            } // non-zero Unit to discard
            count -= 3 as libc::c_int; // [a multiple of DECDPUN]
            up = up.offset(1)
        }
        if count == 0 as libc::c_int {
            got = 0 as libc::c_int
        } else {
            // [not multiple of DECDPUN]
            let mut rem: int32_t = 0; // work
            // slice off fraction digits and check for non-zero
            theInt =
                ((*up as uint32_t >>
                      count).wrapping_mul(multies[count as usize]) >>
                     17 as libc::c_int) as int32_t; // non-zero fraction
            rem =
                (*up as
                     libc::c_uint).wrapping_sub((theInt as
                                                     libc::c_uint).wrapping_mul(DECPOWERS[count
                                                                                              as
                                                                                              usize]))
                    as int32_t;
            if rem != 0 as libc::c_int {
                return 0x80000000 as libc::c_uint as int32_t
            }
            // it looks good
            got = 3 as libc::c_int - count; // number of digits so far
            up = up.offset(1)
        }
    }
    // now it's known there's no fractional part
    // tricky code now, to accumulate up to 9.3 digits
    if got == 0 as libc::c_int {
        theInt = *up as int32_t; // ensure lsu is there
        got += 3 as libc::c_int;
        up = up.offset(1)
    }
    if ilength < 11 as libc::c_int {
        let mut save: int32_t = theInt;
        // collect any remaining unit(s)
        while got < ilength {
            theInt =
                (theInt as
                     libc::c_uint).wrapping_add((*up as
                                                     libc::c_uint).wrapping_mul(DECPOWERS[got
                                                                                              as
                                                                                              usize]))
                    as int32_t as int32_t;
            got += 3 as libc::c_int;
            up = up.offset(1)
        }
        if ilength == 10 as libc::c_int {
            // need to check for wrap
            if theInt /
                   DECPOWERS[(got - 3 as libc::c_int) as usize] as int32_t !=
                   *up.offset(-(1 as libc::c_int as isize)) as int32_t {
                ilength = 11 as libc::c_int
            } else if neg as libc::c_int != 0 &&
                          theInt > 1999999997 as libc::c_int {
                ilength = 11 as libc::c_int
            } else if neg == 0 && theInt > 999999999 as libc::c_int {
                ilength = 11 as libc::c_int
            }
            if ilength == 11 as libc::c_int { theInt = save }
            // [that test also disallows the BADINT result case]
            // restore correct low bit
        }
    }
    if ilength > 10 as libc::c_int {
        // too big
        if theInt & 1 as libc::c_int != 0 {
            return 0x80000003 as libc::c_uint as int32_t
        }
        return 0x80000002 as libc::c_uint as int32_t // bottom bit 1
        // bottom bit 0
    } // apply sign
    if neg != 0 { theInt = -theInt }
    return theInt;
}
// decGetInt
/* ------------------------------------------------------------------ */
/* decDecap -- decapitate the coefficient of a number                 */
/*                                                                    */
/*   dn   is the number to be decapitated                             */
/*   drop is the number of digits to be removed from the left of dn;  */
/*     this must be <= dn->digits (if equal, the coefficient is       */
/*     set to 0)                                                      */
/*                                                                    */
/* Returns dn; dn->digits will be <= the initial digits less drop     */
/* (after removing drop digits there may be leading zero digits       */
/* which will also be removed).  Only dn->lsu and dn->digits change.  */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decDecap(mut dn: *mut decNumber, mut drop_0: int32_t)
 -> *mut decNumber {
    let mut msu: *mut uint16_t = 0 as *mut uint16_t; // -> target cut point
    let mut cut: int32_t = 0; // work
    if drop_0 >= (*dn).digits {
        // losing the whole thing
        *(*dn).lsu.as_mut_ptr().offset(0 as libc::c_int as isize) =
            0 as libc::c_int as uint16_t; // -> likely msu
        (*dn).digits = 1 as libc::c_int; // digits to be in use in msu
        return dn
    } // clear left digits
    msu =
        (*dn).lsu.as_mut_ptr().offset((if (*dn).digits - drop_0 <=
                                              49 as libc::c_int {
                                           d2utable[((*dn).digits - drop_0) as
                                                        usize] as libc::c_int
                                       } else {
                                           ((*dn).digits - drop_0 +
                                                3 as libc::c_int -
                                                1 as libc::c_int) /
                                               3 as libc::c_int
                                       }) as
                                          isize).offset(-(1 as libc::c_int as
                                                              isize));
    cut =
        (*dn).digits - drop_0 -
            ((if (*dn).digits - drop_0 <= 49 as libc::c_int {
                  d2utable[((*dn).digits - drop_0) as usize] as libc::c_int
              } else {
                  ((*dn).digits - drop_0 + 3 as libc::c_int -
                       1 as libc::c_int) / 3 as libc::c_int
              }) - 1 as libc::c_int) * 3 as libc::c_int;
    if cut != 3 as libc::c_int {
        *msu =
            (*msu as libc::c_uint).wrapping_rem(DECPOWERS[cut as usize]) as
                uint16_t as uint16_t
    }
    // that may have left leading zero digits, so do a proper count...
    (*dn).digits =
        decGetDigits((*dn).lsu.as_mut_ptr(),
                     (msu.wrapping_offset_from((*dn).lsu.as_mut_ptr()) as
                          libc::c_long + 1 as libc::c_int as libc::c_long) as
                         int32_t);
    return dn;
}
// decDecap
/* ------------------------------------------------------------------ */
/* decBiStr -- compare string with pairwise options                   */
/*                                                                    */
/*   targ is the string to compare                                    */
/*   str1 is one of the strings to compare against (length may be 0)  */
/*   str2 is the other; it must be the same length as str1            */
/*                                                                    */
/*   returns 1 if strings compare equal, (that is, it is the same     */
/*   length as str1 and str2, and each character of targ is in either */
/*   str1 or str2 in the corresponding position), or 0 otherwise      */
/*                                                                    */
/* This is used for generic caseless compare, including the awkward   */
/* case of the Turkish dotted and dotless Is.  Use as (for example):  */
/*   if (decBiStr(test, "mike", "MIKE")) ...                          */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decBiStr(mut targ: *const libc::c_char,
                              mut str1: *const libc::c_char,
                              mut str2: *const libc::c_char) -> uint8_t {
    loop  {
        if *targ as libc::c_int != *str1 as libc::c_int &&
               *targ as libc::c_int != *str2 as libc::c_int {
            return 0 as libc::c_int as uint8_t
        } // forever
        // *targ has a match in one (or both, if terminator)
        if *targ as libc::c_int == '\u{0}' as i32 { break ; }
        targ = targ.offset(1);
        str1 = str1.offset(1);
        str2 = str2.offset(1)
    }
    return 1 as libc::c_int as uint8_t;
}
// decBiStr
/* ------------------------------------------------------------------ */
/* decNaNs -- handle NaN operand or operands                          */
/*                                                                    */
/*   res     is the result number                                     */
/*   lhs     is the first operand                                     */
/*   rhs     is the second operand, or NULL if none                   */
/*   context is used to limit payload length                          */
/*   status  contains the current status                              */
/*   returns res in case convenient                                   */
/*                                                                    */
/* Called when one or both operands is a NaN, and propagates the      */
/* appropriate result to res.  When an sNaN is found, it is changed   */
/* to a qNaN and Invalid operation is set.                            */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decNaNs(mut res: *mut decNumber,
                             mut lhs: *const decNumber,
                             mut rhs: *const decNumber,
                             mut set: *mut decContext,
                             mut status: *mut uint32_t) -> *mut decNumber {
    // This decision tree ends up with LHS being the source pointer,
  // and status updated if need be
    if (*lhs).bits as libc::c_int & 0x10 as libc::c_int != 0 {
        *status |=
            (0x80 as libc::c_int | 0x40000000 as libc::c_int) as libc::c_uint
    } else if !rhs.is_null() {
        if (*rhs).bits as libc::c_int & 0x10 as libc::c_int != 0 {
            lhs = rhs;
            *status |=
                (0x80 as libc::c_int | 0x40000000 as libc::c_int) as
                    libc::c_uint
        } else if !((*lhs).bits as libc::c_int & 0x20 as libc::c_int != 0) {
            lhs = rhs
        }
    }
    // propagate the payload
    if (*lhs).digits <= (*set).digits {
        decNumberCopy(res, lhs); // easy
    } else {
        // too long
        let mut ul: *const uint16_t = 0 as *const uint16_t;
        let mut ur: *mut uint16_t = 0 as *mut uint16_t;
        let mut uresp1: *mut uint16_t = 0 as *mut uint16_t;
        // copy safe number of units, then decapitate
        (*res).bits = (*lhs).bits; // need sign etc.
        uresp1 =
            (*res).lsu.as_mut_ptr().offset((if (*set).digits <=
                                                   49 as libc::c_int {
                                                d2utable[(*set).digits as
                                                             usize] as
                                                    libc::c_int
                                            } else {
                                                ((*set).digits +
                                                     3 as libc::c_int -
                                                     1 as libc::c_int) /
                                                    3 as libc::c_int
                                            }) as isize);
        ur = (*res).lsu.as_mut_ptr();
        ul = (*lhs).lsu.as_ptr();
        while ur < uresp1 { *ur = *ul; ur = ur.offset(1); ul = ul.offset(1) }
        (*res).digits =
            (if (*set).digits <= 49 as libc::c_int {
                 d2utable[(*set).digits as usize] as libc::c_int
             } else {
                 ((*set).digits + 3 as libc::c_int - 1 as libc::c_int) /
                     3 as libc::c_int
             }) * 3 as libc::c_int;
        // maybe still too long
        if (*res).digits > (*set).digits {
            decDecap(res,
                     (*res).digits -
                         (*set).digits); // convert any sNaN to NaN, while
        }
    } // .. preserving sign
    (*res).bits =
        ((*res).bits as libc::c_int & !(0x10 as libc::c_int)) as
            uint8_t; // clean exponent
                              // [coefficient was copied/decapitated]
    (*res).bits =
        ((*res).bits as libc::c_int | 0x20 as libc::c_int) as uint8_t;
    (*res).exponent = 0 as libc::c_int;
    return res;
}
// decNaNs
/* ------------------------------------------------------------------ */
/* decStatus -- apply non-zero status                                 */
/*                                                                    */
/*   dn     is the number to set if error                             */
/*   status contains the current status (not yet in context)          */
/*   set    is the context                                            */
/*                                                                    */
/* If the status is an error status, the number is set to a NaN,      */
/* unless the error was an overflow, divide-by-zero, or underflow,    */
/* in which case the number will have already been set.               */
/*                                                                    */
/* The context status is then updated with the new status.  Note that */
/* this may raise a signal, so control may never return from this     */
/* routine (hence resources must be recovered before it is called).   */
/* ------------------------------------------------------------------ */
unsafe extern "C" fn decStatus(mut dn: *mut decNumber, mut status: uint32_t,
                               mut set: *mut decContext) {
    if status &
           (0x1 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int |
                0x10 as libc::c_int | 0x40 as libc::c_int |
                0x80 as libc::c_int) as libc::c_uint != 0 {
        // error status -> NaN
        // if cause was an sNaN, clear and propagate [NaN is already set up]
        if status & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            status &= !(0x40000000 as libc::c_int) as libc::c_uint
        } else {
            decNumberZero(dn);
            (*dn).bits =
                0x20 as libc::c_int as
                    uint8_t // other error: clean throughout
            // and make a quiet NaN
        }
    } // [may not return]
    decContextSetStatus(set, status);
}
// decStatus
/* ------------------------------------------------------------------ */
/* decGetDigits -- count digits in a Units array                      */
/*                                                                    */
/*   uar is the Unit array holding the number (this is often an       */
/*          accumulator of some sort)                                 */
/*   len is the length of the array in units [>=1]                    */
/*                                                                    */
/*   returns the number of (significant) digits in the array          */
/*                                                                    */
/* All leading zeros are excluded, except the last if the array has   */
/* only zero Units.                                                   */
/* ------------------------------------------------------------------ */
// This may be called twice during some operations.
unsafe extern "C" fn decGetDigits(mut uar: *mut uint16_t, mut len: int32_t)
 -> int32_t {
    let mut up: *mut uint16_t =
        uar.offset((len - 1 as libc::c_int) as isize); // -> msu
    let mut digits: int32_t =
        (len - 1 as libc::c_int) * 3 as libc::c_int +
            1 as libc::c_int; // possible digits excluding msu
    // (at least 1 in final msu)
    while up >= uar {
        if *up as libc::c_int == 0 as libc::c_int { // up
            // unit is all 0s
            if digits == 1 as libc::c_int {
                break ; // a zero has one digit
            } // adjust for 0 unit
            digits -= 3 as libc::c_int;
            up = up.offset(-1)
        } else {
            // found the first (most significant) non-zero Unit
            // not done yet
            if (*up as libc::c_int) < 10 as libc::c_int {
                break ; // is 1-9
            }
            digits += 1;
            // not done yet
            if (*up as libc::c_int) < 100 as libc::c_int {
                break ; // is 10-99
            }
            digits += 1;
            break ;
        }
    }
    return digits;
}
// decGetDigits
