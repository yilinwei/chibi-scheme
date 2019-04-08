#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! sexp_make_immediate {
    ($n: expr) => {
        (($n << $crate::SEXP_EXTENDED_BITS) + $crate::SEXP_EXTENDED_TAG) as $crate::sexp
    };
}

#[macro_export]
macro_rules! i8_cstring {
    ($s: expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

#[macro_export]
macro_rules! sexp_unbox_fixnum {
    ($n: expr) => {
        ($n as $crate::sexp_sint_t) >> $crate::SEXP_FIXNUM_BITS
    };
}

#[macro_export]
macro_rules! sexp_make_character {
    ($n: expr) => {
        ((($n as $crate::sexp_sint_t) << $crate::SEXP_EXTENDED_BITS) + ($crate::SEXP_CHAR_TAG as $crate::sexp_sint_t)) as $crate::sexp
    }
}

#[macro_export]
macro_rules! sexp_unbox_character {
    ($n: expr) => {
        (($n as $crate::sexp_sint_t) >> $crate::SEXP_EXTENDED_BITS) as std::os::raw::c_char
    }
}

pub const SEXP_FALSE: sexp = sexp_make_immediate!(0);
pub const SEXP_TRUE: sexp = sexp_make_immediate!(1);
pub const SEXP_NULL: sexp = sexp_make_immediate!(2);
pub const SEXP_EOF: sexp = sexp_make_immediate!(3);
pub const SEXP_VOID: sexp = sexp_make_immediate!(4);

#[macro_export]
macro_rules! sexp_truep {
    ($x: expr) => {
        $x != $crate::SEXP_FALSE
    };
}

#[macro_export]
macro_rules! sexp_not {
    ($x: expr) => {
        $x == $crate::SEXP_FALSE
    };
}

#[macro_export]
macro_rules! sexp_nullp {
    ($x: expr) => {
        $x == $crate::SEXP_NULL
    };
}

#[macro_export]
macro_rules! sexp_fixnump {
    ($x: expr) => {
        (($x as sexp_uint_t) & SEXP_FIXNUM_MASK as u64) == SEXP_FIXNUM_TAG as u32
    };
}

#[macro_export]
macro_rules! sexp_isymbolp {
    ($x: expr) => {
        (($x as sexp_uint_t) & SEXP_IMMEDIATE_MASK) == SEXP_ISYMBOL_TAG as u32
    };
}

#[macro_export]
macro_rules! sexp_charp {
    ($x: expr) => {
        (($x as $crate::sexp_uint_t) & (SEXP_EXTENDED_MASK as $crate::sexp_uint_t)) == SEXP_CHAR_TAG as $crate::sexp_uint_t
    };
}

#[macro_export]
macro_rules! sexp_booleanp {
    ($x: expr) => {
        (($x) == $crate::SEXP_TRUE) || (($x) == $crate::SEXP_FALSE)
    }
}
