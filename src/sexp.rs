use chibi_scheme_sys::*;

use std::fmt;
use std::os::raw;

pub struct SExp(pub sexp);

impl SExp {
    pub fn bool(self) -> Option<Bool> {
        if sexp_booleanp!(self.0) {
            Some(Bool(self.0))
        } else {
            None
        }
    }

    pub fn char(self) -> Option<Char> {
        if sexp_charp!(self.0) {
            Some(Char(self.0))
        } else {
            None
        }
    }
}

pub struct Symbol(sexp);

pub struct Char(sexp);

impl Into<char> for Char {
    fn into(self: Self) -> char {
        //TODO: Need to check casting
        (sexp_unbox_character!(self.0) as u8) as char
    }
}

impl From<char> for Char {
    fn from(c: char) -> Char {
        //TODO: safety first -
        Char(sexp_make_character!(c as raw::c_char))
    }
}

impl fmt::Debug for Char {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("#\\{}", sexp_unbox_character!(self.0 as u8)))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_unbox_character!(self.0 as u8) ==
            sexp_unbox_character!(rhs.0 as u8)
    }
}

pub struct Bool(sexp);

impl Into<bool> for Bool {
    fn into(self: Self) -> bool {
        sexp_truep!(self.0)
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Bool {
        if b {
            Bool(SEXP_TRUE)
        } else {
            Bool(SEXP_FALSE)
        }
    }
}

impl PartialEq for Bool {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_truep!(self.0) && sexp_truep!(rhs.0)
            || sexp_not!(self.0) && sexp_not!(rhs.0)
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if sexp_truep!(self.0) {
            fmt.write_str("#t")
        } else {
            fmt.write_str("#f")
        }
    }
}
