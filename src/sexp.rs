use chibi_scheme_sys::*;

use std::fmt;

struct SExp(sexp);

impl SExp {
    fn bool(self) -> Option<Bool> {
        if sexp_booleanp!(self.0) {
            Some(Bool(self.0))
        } else {
            None
        }
    }

    fn char(self) -> Option<Char> {
        if sexp_booleanp!(self.0) {
            Some(Char(self.0))
        } else {
            None
        }
    }
}

struct Symbol(sexp);

struct Char(sexp);

struct Bool(sexp);

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

impl fmt::Debug for Bool {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if sexp_truep!(self.0) {
            fmt.write_str("#t")
        } else {
            fmt.write_str("#f")
        }
    }
}
