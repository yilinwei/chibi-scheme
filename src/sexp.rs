use chibi_scheme_sys::*;

use std::ffi;
use std::fmt;
use std::os::raw;
use std::slice;

pub struct SExp(pub sexp);

impl SExp {
    pub fn bool(self) -> Option<Bool> {
        if sexp_booleanp(self.0) {
            Some(Bool(self.0))
        } else {
            None
        }
    }

    pub fn char(self) -> Option<Char> {
        if sexp_charp(self.0) {
            Some(Char(self.0))
        } else {
            None
        }
    }

    pub fn string(self) -> Option<String> {
        if sexp_stringp(self.0) {
            Some(String(self.0))
        } else {
            None
        }
    }
}

pub struct String(sexp);

impl String {
    // TODO: Once lifetime has worked, then remove
    fn as_c_str(&self) -> &ffi::CStr {
        let len = (sexp_string_size(self.0) + 1) as _;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) }
    }
}

impl Into<ffi::CString> for String {
    fn into(self: Self) -> ffi::CString {
        let len = (sexp_string_size(self.0) + 1) as _;
        let mut data = Vec::with_capacity(len);
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        data.extend_from_slice(slice);
        unsafe { ffi::CString::from_vec_unchecked(data) }
    }
}

impl fmt::Debug for String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("\"{:?}\"", self.as_c_str()))
    }
}

impl PartialEq for String {
    fn eq(self: &Self, rhs: &Self) -> bool {
        self.as_c_str() == rhs.as_c_str()
    }
}

pub struct Symbol(sexp);

pub struct Char(sexp);

impl Into<char> for Char {
    fn into(self: Self) -> char {
        //TODO: Need to check casting
        (sexp_unbox_character(self.0) as u8) as char
    }
}

impl From<char> for Char {
    fn from(c: char) -> Char {
        //TODO: check
        Char(sexp_make_character(c as _))
    }
}

impl fmt::Debug for Char {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("#\\{}", sexp_unbox_character(self.0)))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_unbox_character(self.0) ==
            sexp_unbox_character(rhs.0)
    }
}

pub struct Bool(sexp);

impl Into<bool> for Bool {
    fn into(self: Self) -> bool {
        sexp_truep(self.0)
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
        sexp_truep(self.0) && sexp_truep(rhs.0)
            || sexp_not(self.0) && sexp_not(rhs.0)
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if sexp_truep(self.0) {
            fmt.write_str("#t")
        } else {
            fmt.write_str("#f")
        }
    }
}
