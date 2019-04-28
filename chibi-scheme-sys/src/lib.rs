#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw;
use std::ptr;

const fn sexp_make_immediate(n: u32) -> sexp {
    ((n << SEXP_EXTENDED_BITS) + SEXP_EXTENDED_TAG) as sexp
}

pub fn sexp_make_fixnum(n: i64) -> sexp {
    (((n as sexp_sint_t) << SEXP_FIXNUM_BITS) + (SEXP_FIXNUM_TAG as sexp_sint_t)) as sexp
}

pub fn sexp_unbox_fixnum(n: sexp) -> sexp_sint_t {
    (n as sexp_sint_t) >> SEXP_FIXNUM_BITS
}

pub fn sexp_flonum_value(x: sexp) -> f64 {
    unsafe { *(*x).value.flonum.as_ref() }
}

pub fn sexp_make_character(n: raw::c_char) -> sexp {
    (((n as sexp_sint_t) << SEXP_EXTENDED_BITS) + (SEXP_CHAR_TAG as sexp_sint_t)) as sexp
}

pub fn sexp_unbox_character(n: sexp) -> raw::c_char {
    ((n as sexp_sint_t) >> SEXP_EXTENDED_BITS) as raw::c_char
}

pub const SEXP_FALSE: sexp = sexp_make_immediate(0);
pub const SEXP_TRUE: sexp = sexp_make_immediate(1);
pub const SEXP_NULL: sexp = sexp_make_immediate(2);
pub const SEXP_EOF: sexp = sexp_make_immediate(3);
pub const SEXP_VOID: sexp = sexp_make_immediate(4);

pub fn sexp_truep(x: sexp) -> bool {
    x != SEXP_FALSE
}

pub fn sexp_not(x: sexp) -> bool {
    x == SEXP_FALSE
}

pub fn sexp_nullp(x: sexp) -> bool {
    x == SEXP_NULL
}

pub fn sexp_fixnump(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_FIXNUM_MASK as sexp_uint_t) == SEXP_FIXNUM_TAG as sexp_uint_t
}

pub fn sexp_flonump(x: sexp) -> bool {
    sexp_check_tag(x, sexp_types_SEXP_FLONUM)
}

pub fn sexp_exceptionp(x: sexp) -> bool {
    sexp_check_tag(x, sexp_types_SEXP_EXCEPTION)
}

pub fn sexp_exception_message(x: sexp) -> sexp {
    unsafe {(*x).value.exception.as_ref().message}
}

pub fn sexp_isymbolp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_IMMEDIATE_MASK as sexp_uint_t) == SEXP_ISYMBOL_TAG as sexp_uint_t
}

pub fn sexp_lsymbolp(x: sexp) -> bool {
    sexp_check_tag(x, sexp_types_SEXP_SYMBOL)
}

pub fn sexp_symbolp(x: sexp) -> bool {
    sexp_isymbolp(x) || sexp_lsymbolp(x)
}

pub fn sexp_charp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_EXTENDED_MASK as sexp_uint_t) == SEXP_CHAR_TAG as sexp_uint_t
}

pub fn sexp_integerp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_FIXNUM_MASK as sexp_uint_t) == SEXP_FIXNUM_TAG as sexp_uint_t
}

pub fn sexp_realp(x: sexp) -> bool {
    sexp_integerp(x) || sexp_flonump(x)
}

pub fn sexp_booleanp(x: sexp) -> bool {
    x == SEXP_TRUE || x == SEXP_FALSE
}

pub fn sexp_pointerp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_POINTER_MASK as sexp_uint_t) == SEXP_POINTER_TAG as sexp_uint_t
}

pub fn sexp_pointer_tag(x: sexp) -> sexp_tag_t {
    unsafe { (*x).tag }
}

pub fn sexp_check_tag(x: sexp, t: sexp_tag_t) -> bool {
    sexp_pointerp(x) &&
        (sexp_pointer_tag(x) == t)
}

pub fn sexp_stringp(x: sexp) -> bool {
    sexp_check_tag(x, sexp_types_SEXP_STRING)
}

pub fn sexp_pairp(x: sexp) -> bool {
    sexp_check_tag(x, sexp_types_SEXP_PAIR)
}

pub fn sexp_cons(ctx: sexp, a: sexp, b: sexp) -> sexp {
    unsafe { sexp_cons_op(ctx, ptr::null_mut(), 2, a, b) }
}

pub fn sexp_listp(ctx: sexp, x: sexp) -> sexp {
    unsafe { sexp_listp_op(ctx, ptr::null_mut(), 1, x) }
}

pub fn sexp_car(x: sexp) -> sexp {
    unsafe { (*x).value.pair.as_ref().car }
}

pub fn sexp_cdr(x: sexp) -> sexp {
    unsafe { (*x).value.pair.as_ref().cdr }
}

pub fn sexp_string_size(x: sexp) -> sexp_uint_t {
    unsafe { (*x).value.string.as_ref().length }
}

pub fn sexp_bytes_length(x: sexp) -> sexp_uint_t {
    unsafe { (*x).value.bytes.as_ref().length }
}

pub fn sexp_bytes_data(x: sexp) -> *const raw::c_char {
    unsafe { (*x).value.bytes.as_ref().data.as_ptr() }
}

pub fn sexp_string_offset(x: sexp) -> sexp_uint_t {
    unsafe { (*x).value.string.as_ref().offset }
}

pub fn sexp_string_data(x: sexp) -> *const raw::c_char {
    unsafe{ sexp_bytes_data((*x).value.string.as_ref().bytes).offset(sexp_string_offset(x) as isize) }
}

pub fn sexp_string_length(x: sexp) -> sexp_uint_t {
    unsafe{ (*x).value.string.as_ref().length }
}

pub fn sexp_equalp(ctx: sexp, a: sexp, b: sexp) -> sexp {
    unsafe {sexp_equalp_op(ctx, ptr::null_mut(), 2, a, b) }
}

pub fn sexp_symbol_to_string(ctx: sexp, s: sexp) -> sexp {
    unsafe {sexp_symbol_to_string_op(ctx, ptr::null_mut(), 1, s)}
}

// TODO: Safe accessor
// TODO: Add feature for stuff
