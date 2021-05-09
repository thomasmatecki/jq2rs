#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn raise(__sig: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type int32_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
/* ------------------------------------------------------------------ */
/* Decimal Context module header                                      */
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
/*                                                                    */
/* Context variables must always have valid values:                   */
/*                                                                    */
/*  status   -- [any bits may be cleared, but not set, by user]       */
/*  round    -- must be one of the enumerated rounding modes          */
/*                                                                    */
/* The following variables are implied for fixed size formats (i.e.,  */
/* they are ignored) but should still be set correctly in case used   */
/* with decNumber functions:                                          */
/*                                                                    */
/*  clamp    -- must be either 0 or 1                                 */
/*  digits   -- must be in the range 1 through 999999999              */
/*  emax     -- must be in the range 0 through 999999999              */
/*  emin     -- must be in the range 0 through -999999999             */
/*  extended -- must be either 0 or 1 [present only if DECSUBSET]     */
/*  traps    -- only defined bits may be set                          */
/*                                                                    */
/* ------------------------------------------------------------------ */
/* Short name */
/* Verbose name */
/* Who to blame */
/* C99 standard integers           */
/* for printf, etc.                */
/* for traps                       */
/* Extended flags setting -- set this to 0 to use only IEEE flags   */
/* 1=enable extended flags         */
/* Conditional code flag -- set this to 0 for best performance      */
/* 1=enable subset arithmetic      */
/* Context for operations, with associated constants                */
pub type rounding = libc::c_uint;
/* enum must be less than this     */
/* round for reround               */
pub const DEC_ROUND_MAX: rounding = 8;
/* round towards -infinity         */
pub const DEC_ROUND_05UP: rounding = 7;
/* round towards 0 (truncate)      */
pub const DEC_ROUND_FLOOR: rounding = 6;
/* 0.5 rounds down                 */
pub const DEC_ROUND_DOWN: rounding = 5;
/* 0.5 rounds to nearest even      */
pub const DEC_ROUND_HALF_DOWN: rounding = 4;
/* 0.5 rounds up                   */
pub const DEC_ROUND_HALF_EVEN: rounding = 3;
/* round away from 0               */
pub const DEC_ROUND_HALF_UP: rounding = 2;
/* round towards +infinity         */
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
/* ------------------------------------------------------------------ */
/* Decimal Context module                                             */
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
/* This module comprises the routines for handling arithmetic         */
/* context structures.                                                */
/* ------------------------------------------------------------------ */
// for strcmp
// for printf if DECCHECK
/* compile-time endian tester [assumes sizeof(Int)>1] */
static mut mfcone: int32_t = 1 as libc::c_int;
// constant 1
static mut mfctop: *const uint8_t =
    unsafe { &mfcone as *const int32_t as *const uint8_t };
// named flag; 1=little-endian
/* ------------------------------------------------------------------ */
/* round-for-reround digits                                           */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub static mut DECSTICKYTAB: [uint8_t; 10] =
    [1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     4 as libc::c_int as uint8_t, 6 as libc::c_int as uint8_t,
     6 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     8 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t];
/* used if sticky */
/* ------------------------------------------------------------------ */
/* Powers of ten (powers[n]==10**n, 0<=n<=9)                          */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub static mut DECPOWERS: [uint32_t; 10] =
    [1 as libc::c_int as uint32_t, 10 as libc::c_int as uint32_t,
     100 as libc::c_int as uint32_t, 1000 as libc::c_int as uint32_t,
     10000 as libc::c_int as uint32_t, 100000 as libc::c_int as uint32_t,
     1000000 as libc::c_int as uint32_t, 10000000 as libc::c_int as uint32_t,
     100000000 as libc::c_int as uint32_t,
     1000000000 as libc::c_int as uint32_t];
/* ------------------------------------------------------------------ */
/* decContextClearStatus -- clear bits in current status              */
/*                                                                    */
/*  context is the context structure to be queried                    */
/*  mask indicates the bits to be cleared (the status bit that        */
/*    corresponds to each 1 bit in the mask is cleared)               */
/*  returns context                                                   */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextClearStatus(mut context: *mut decContext,
                                               mut mask: uint32_t)
 -> *mut decContext {
    (*context).status &= !mask;
    return context;
}
// decContextClearStatus
/* ------------------------------------------------------------------ */
/* decContextDefault -- initialize a context structure                */
/*                                                                    */
/*  context is the structure to be initialized                        */
/*  kind selects the required set of default values, one of:          */
/*      DEC_INIT_BASE       -- select ANSI X3-274 defaults            */
/*      DEC_INIT_DECIMAL32  -- select IEEE 754 defaults, 32-bit       */
/*      DEC_INIT_DECIMAL64  -- select IEEE 754 defaults, 64-bit       */
/*      DEC_INIT_DECIMAL128 -- select IEEE 754 defaults, 128-bit      */
/*      For any other value a valid context is returned, but with     */
/*      Invalid_operation set in the status field.                    */
/*  returns a context structure with the appropriate initial values.  */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextDefault(mut context: *mut decContext,
                                           mut kind: int32_t)
 -> *mut decContext {
    // set defaults...
    (*context).digits = 9 as libc::c_int; // 9 digits
    (*context).emax = 999999999 as libc::c_int; // 9-digit exponents
    (*context).emin = -(999999999 as libc::c_int); // .. balanced
    (*context).round = DEC_ROUND_HALF_UP; // 0.5 rises
    (*context).traps =
        (0x2 as libc::c_int |
             (0x1 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int |
                  0x10 as libc::c_int | 0x40 as libc::c_int |
                  0x80 as libc::c_int) | 0x200 as libc::c_int |
             0x2000 as libc::c_int) as uint32_t; // all but informational
    (*context).status = 0 as libc::c_int as uint32_t; // cleared
    (*context).clamp = 0 as libc::c_int as uint8_t; // no clamping
    match kind {
        0 => { }
        32 => {
            (*context).digits = 7 as libc::c_int; // digits
            (*context).emax = 96 as libc::c_int; // Emax
            (*context).emin = -(95 as libc::c_int); // Emin
            (*context).round = DEC_ROUND_HALF_EVEN; // 0.5 to nearest even
            (*context).traps = 0 as libc::c_int as uint32_t; // no traps set
            (*context).clamp = 1 as libc::c_int as uint8_t
        }
        64 => { // clamp exponents
            (*context).digits = 16 as libc::c_int; // digits
            (*context).emax = 384 as libc::c_int; // Emax
            (*context).emin = -(383 as libc::c_int); // Emin
            (*context).round = DEC_ROUND_HALF_EVEN; // 0.5 to nearest even
            (*context).traps = 0 as libc::c_int as uint32_t; // no traps set
            (*context).clamp = 1 as libc::c_int as uint8_t
        }
        128 => { // clamp exponents
            (*context).digits = 34 as libc::c_int; // digits
            (*context).emax = 6144 as libc::c_int; // Emax
            (*context).emin = -(6143 as libc::c_int); // Emin
            (*context).round = DEC_ROUND_HALF_EVEN; // 0.5 to nearest even
            (*context).traps = 0 as libc::c_int as uint32_t; // no traps set
            (*context).clamp = 1 as libc::c_int as uint8_t
        }
        _ => { // clamp exponents
            // invalid Kind
            // use defaults, and ..
            decContextSetStatus(context, 0x80 as libc::c_int as uint32_t);
        }
    }
    return context;
}
// decContextDefault
/* ------------------------------------------------------------------ */
/* decContextGetRounding -- return current rounding mode              */
/*                                                                    */
/*  context is the context structure to be queried                    */
/*  returns the rounding mode                                         */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextGetRounding(mut context: *mut decContext)
 -> rounding {
    return (*context).round;
}
// decContextGetRounding
/* ------------------------------------------------------------------ */
/* decContextGetStatus -- return current status                       */
/*                                                                    */
/*  context is the context structure to be queried                    */
/*  returns status                                                    */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextGetStatus(mut context: *mut decContext)
 -> uint32_t {
    return (*context).status;
}
// decContextGetStatus
/* ------------------------------------------------------------------ */
/* decContextRestoreStatus -- restore bits in current status          */
/*                                                                    */
/*  context is the context structure to be updated                    */
/*  newstatus is the source for the bits to be restored               */
/*  mask indicates the bits to be restored (the status bit that       */
/*    corresponds to each 1 bit in the mask is set to the value of    */
/*    the correspnding bit in newstatus)                              */
/*  returns context                                                   */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextRestoreStatus(mut context: *mut decContext,
                                                 mut newstatus: uint32_t,
                                                 mut mask: uint32_t)
 -> *mut decContext {
    (*context).status &= !mask; // clear the selected bits
    (*context).status |= mask & newstatus; // or in the new bits
    return context;
}
// decContextRestoreStatus
/* ------------------------------------------------------------------ */
/* decContextSaveStatus -- save bits in current status                */
/*                                                                    */
/*  context is the context structure to be queried                    */
/*  mask indicates the bits to be saved (the status bits that         */
/*    correspond to each 1 bit in the mask are saved)                 */
/*  returns the AND of the mask and the current status                */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextSaveStatus(mut context: *mut decContext,
                                              mut mask: uint32_t)
 -> uint32_t {
    return (*context).status & mask;
}
// decContextSaveStatus
/* ------------------------------------------------------------------ */
/* decContextSetRounding -- set current rounding mode                 */
/*                                                                    */
/*  context is the context structure to be updated                    */
/*  newround is the value which will replace the current mode         */
/*  returns context                                                   */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextSetRounding(mut context: *mut decContext,
                                               mut newround: rounding)
 -> *mut decContext {
    (*context).round = newround;
    return context;
}
// decContextSetRounding
/* ------------------------------------------------------------------ */
/* decContextSetStatus -- set status and raise trap if appropriate    */
/*                                                                    */
/*  context is the context structure to be updated                    */
/*  status  is the DEC_ exception code                                */
/*  returns the context structure                                     */
/*                                                                    */
/* Control may never return from this routine, if there is a signal   */
/* handler and it takes a long jump.                                  */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextSetStatus(mut context: *mut decContext,
                                             mut status: uint32_t)
 -> *mut decContext {
    (*context).status |= status;
    if status & (*context).traps != 0 { raise(8 as libc::c_int); }
    return context;
}
// decContextSetStatusFromStringQuiet
/* ------------------------------------------------------------------ */
/* decContextSetStatusQuiet -- set status without trap                */
/*                                                                    */
/*  context is the context structure to be updated                    */
/*  status  is the DEC_ exception code                                */
/*  returns the context structure                                     */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextSetStatusQuiet(mut context:
                                                      *mut decContext,
                                                  mut status: uint32_t)
 -> *mut decContext {
    (*context).status |= status;
    return context;
}
// decContextSetStatusQuiet
/* ------------------------------------------------------------------ */
/* decContextStatusToString -- convert status flags to a string       */
/*                                                                    */
/*  context is a context with valid status field                      */
/*                                                                    */
/*  returns a constant string describing the condition.  If multiple  */
/*    (or no) flags are set, a generic constant message is returned.  */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextStatusToString(mut context:
                                                      *const decContext)
 -> *const libc::c_char {
    let mut status: int32_t = (*context).status as int32_t;
    // test the five IEEE first, as some of the others are ambiguous when
  // DECEXTFLAG=0
    if status == 0x80 as libc::c_int {
        return b"Invalid operation\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x2 as libc::c_int {
        return b"Division by zero\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x200 as libc::c_int {
        return b"Overflow\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x2000 as libc::c_int {
        return b"Underflow\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x20 as libc::c_int {
        return b"Inexact\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x4 as libc::c_int {
        return b"Division impossible\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x8 as libc::c_int {
        return b"Division undefined\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x800 as libc::c_int {
        return b"Rounded\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x400 as libc::c_int {
        return b"Clamped\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x1000 as libc::c_int {
        return b"Subnormal\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x1 as libc::c_int {
        return b"Conversion syntax\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x10 as libc::c_int {
        return b"Insufficient storage\x00" as *const u8 as *const libc::c_char
    }
    if status == 0x40 as libc::c_int {
        return b"Invalid context\x00" as *const u8 as *const libc::c_char
    }
    if status == 0 as libc::c_int {
        return b"No status\x00" as *const u8 as *const libc::c_char
    }
    return b"Multiple status\x00" as *const u8 as *const libc::c_char;
    // Multiple errors
}
// decContextStatusToString
/* ------------------------------------------------------------------ */
/* decContextTestEndian -- test whether DECLITEND is set correctly    */
/*                                                                    */
/*  quiet is 1 to suppress message; 0 otherwise                       */
/*  returns 0 if DECLITEND is correct                                 */
/*          1 if DECLITEND is incorrect and should be 1               */
/*         -1 if DECLITEND is incorrect and should be 0               */
/*                                                                    */
/* A message is displayed if the return value is not 0 and quiet==0.  */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextTestEndian(mut quiet: uint8_t) -> int32_t {
    let mut res: int32_t = 0 as libc::c_int; // optimist
    let mut dle: uint32_t = 1 as libc::c_int as uint32_t; // unsign
    if dle > 1 as libc::c_int as libc::c_uint {
        dle = 1 as libc::c_int as uint32_t
    } // ensure 0 or 1
    if *mfctop as libc::c_int != 1 as libc::c_int {
        if quiet == 0 {
            // always refer to this
            let mut adj: *const libc::c_char = 0 as *const libc::c_char;
            if *mfctop != 0 {
                adj = b"little\x00" as *const u8 as *const libc::c_char
            } else { adj = b"big\x00" as *const u8 as *const libc::c_char }
            printf(b"Warning: DECLITEND is set to %d, but this computer appears to be %s-endian\n\x00"
                       as *const u8 as *const libc::c_char, 1 as libc::c_int,
                   adj);
        }
        res =
            (*mfctop as int32_t as libc::c_uint).wrapping_sub(dle) as int32_t
    }
    return res;
}
// decContextTestEndian
/* ------------------------------------------------------------------ */
/* decContextTestSavedStatus -- test bits in saved status             */
/*                                                                    */
/*  oldstatus is the status word to be tested                         */
/*  mask indicates the bits to be tested (the oldstatus bits that     */
/*    correspond to each 1 bit in the mask are tested)                */
/*  returns 1 if any of the tested bits are 1, or 0 otherwise         */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextTestSavedStatus(mut oldstatus: uint32_t,
                                                   mut mask: uint32_t)
 -> uint32_t {
    return (oldstatus & mask != 0 as libc::c_int as libc::c_uint) as
               libc::c_int as uint32_t;
}
// decContextTestSavedStatus
/* ------------------------------------------------------------------ */
/* decContextTestStatus -- test bits in current status                */
/*                                                                    */
/*  context is the context structure to be updated                    */
/*  mask indicates the bits to be tested (the status bits that        */
/*    correspond to each 1 bit in the mask are tested)                */
/*  returns 1 if any of the tested bits are 1, or 0 otherwise         */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextTestStatus(mut context: *mut decContext,
                                              mut mask: uint32_t)
 -> uint32_t {
    return ((*context).status & mask != 0 as libc::c_int as libc::c_uint) as
               libc::c_int as uint32_t;
}
/* flags which are normally errors (result is qNaN, infinite, or 0) */
/* flags which cause a result to become qNaN                        */
/* flags which are normally for information only (finite results)   */
/* IEEE 854 names (for compatibility with older decNumber versions) */
/* Name strings for the exceptional conditions                      */
/* length of the longest string,   */
                                   /* including terminator            */
/* Initialization descriptors, used by decContextDefault            */
/* Synonyms */
/* decContext routines                                              */
// decContextTestStatus
/* ------------------------------------------------------------------ */
/* decContextZeroStatus -- clear all status bits                      */
/*                                                                    */
/*  context is the context structure to be updated                    */
/*  returns context                                                   */
/*                                                                    */
/* No error is possible.                                              */
/* ------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern "C" fn decContextZeroStatus(mut context: *mut decContext)
 -> *mut decContext {
    (*context).status = 0 as libc::c_int as uint32_t;
    return context;
}
// decContextZeroStatus
