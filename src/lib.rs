#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! sexp_make_immediate {
    ($n: expr) => {
        (($n << SEXP_EXTENDED_BITS) + SEXP_EXTENDED_TAG) as sexp
    };
}

macro_rules! i8_cstring {
    ($s: expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

macro_rules! sexp_unbox_fixnum {
    ($n: expr) => {
        ($n as sexp_sint_t) >> SEXP_FIXNUM_BITS
    };
}

const SEXP_FALSE: sexp = sexp_make_immediate!(0);
const SEXP_TRUE: sexp = sexp_make_immediate!(1);
const SEXP_NULL: sexp = sexp_make_immediate!(2);
const SEXP_EOF: sexp = sexp_make_immediate!(3);
const SEXP_VOID: sexp = sexp_make_immediate!(4);

macro_rules! sexp_truep {
    ($x: expr) => {
        $x != SEXP_FALSE
    };
}

macro_rules! sexp_not {
    ($x: expr) => {
        $x == SEXP_FALSE
    };
}

macro_rules! sexp_nullp {
    ($x: expr) => {
        $x == SEXP_NULL
    };
}

macro_rules! sexp_fixnump {
    ($x: expr) => {
        (($x as sexp_uint_t) & SEXP_FIXNUM_MASK as u64) == SEXP_FIXNUM_TAG as u32
    };
}

macro_rules! sexp_issymbolp {
    ($x: expr) => {
        (($x as sexp_uint_t) & SEXP_IMMEDIATE_MASK) == SEXP_ISYMBOL_TAG as u32
    };
}

macro_rules! sexp_ischarp {
    ($x: expr) => {
        (($x as sexp_uint_t) & SEXP_EXTENDED_MASK) == SEXP_CHAR_TAG as u64
    };
}

fn eval(expr: *const i8) -> sexp {
    unsafe {
        let ctx = sexp_make_eval_context(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            0,
        );
        sexp_eval_string(ctx, expr, -1, std::ptr::null_mut())
    }
}

#[test]
fn test_unbox_fixnum() {
    assert_eq!(10, sexp_unbox_fixnum!(eval(i8_cstring!("(+ 3 7)"))))
}
