use chibi_scheme_sys::*;

use std::mem;
use std::ffi;
use std::fmt;
use std::fmt::Debug;
use std::slice;
use std::ptr;

pub struct SExp<'a>(sexp, &'a Context);

trait Visitor {

    type Result;

    fn visit_bool(&mut self, stmt: &Bool) -> Self::Result;
    fn visit_char(&mut self, name: &Char) -> Self::Result;
    fn visit_string(&mut self, expr: &String) -> Self::Result;
    fn visit_nil(&mut self, expr: &Nil) -> Self::Result;
    fn visit_pair(&mut self, expr: &Pair) -> Self::Result;
}

impl <'a> SExp<'a> {

    fn accept<V : Visitor>(&self, visitor: &mut V) -> V::Result {
        if sexp_booleanp(self.0) {
            visitor.visit_bool(&Bool(self.0))
        } else if sexp_charp(self.0) {
            visitor.visit_char(&Char(self.0))
        } else if sexp_stringp(self.0) {
            // TODO: Need to forget the memory
            visitor.visit_string(&String(self.0, self.1))
        } else if sexp_nullp(self.0) {
            visitor.visit_nil(&NIL)
            // TODO: Need to forget the pair
        } else if sexp_pairp(self.0) {
            visitor.visit_pair(&Pair(self.0, self.1))
        } else {
            unreachable!()
        }
    }

    pub fn nullp(&self) -> bool {
        sexp_nullp(self.0)
    }

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

    pub fn string(self) -> Option<String<'a>> {
        if sexp_stringp(self.0) {
            Some(String(self.0, self.1))
        } else {
            None
        }
    }

    pub fn pair(self) -> Option<Pair<'a>> {
        if sexp_pairp(self.0) {
            Some(Pair(self.0, self.1))
        } else {
            None
        }
    }

    pub fn nil(self) -> Option<Nil> {
        if self.nullp() {
            Some(NIL)
        } else {
            None
        }
    }
}

struct FormatVisitor<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl <'a, 'b> Visitor for FormatVisitor<'a, 'b> {
    type Result = Result<(), fmt::Error>;


    fn visit_bool(&mut self, bool: &Bool) -> Self::Result {
        bool.fmt(self.0)
    }

    fn visit_char(&mut self, char: &Char) -> Self::Result {
        char.fmt(self.0)
    }
    fn visit_string(&mut self, string: &String) -> Self::Result {
        string.fmt(self.0)
    }
    fn visit_nil(&mut self, nil: &Nil) -> Self::Result {
        nil.fmt(self.0)
    }
    fn visit_pair(&mut self, pair: &Pair) -> Self::Result {
        pair.fmt(self.0)
    }
}


impl <'a> fmt::Debug for SExp<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut visitor = FormatVisitor(fmt);
        self.accept(&mut visitor)
    }
}

pub struct String<'a>(sexp, &'a Context);

impl <'a> String<'a> {
}

impl <'a> Drop for String<'a> {
    fn drop(&mut self) {
        unsafe { sexp_release_object((self.1).0, self.0) }
    }
}

impl <'a> Into<&'a ffi::CStr> for String<'a> {
    fn into(self: Self) -> &'a ffi::CStr {
        let len = (sexp_string_size(self.0) + 1) as _;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) };
        // Only drop once the context is destroyed
        mem::forget(self);
        c_str
    }
}

impl <'a> Into<ffi::CString> for String<'a> {
    fn into(self: Self) -> ffi::CString {
        let len = (sexp_string_size(self.0)) as _;
        let mut data = Vec::with_capacity(len);
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        data.extend_from_slice(slice);
        unsafe { ffi::CString::from_vec_unchecked(data) }
    }
}

impl <'a> fmt::Debug for String<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let len = (sexp_string_size(self.0) + 1) as _;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) };
        fmt.write_str(&format!("\"{:?}\"", c_str))
    }
}

pub struct Pair<'a>(sexp, &'a Context);

impl <'a> Pair<'a> {
    pub fn car(&self) -> SExp<'a> {
        SExp(sexp_car(self.0), self.1)
    }

    pub fn cdr(&self) -> SExp<'a> {
        SExp(sexp_cdr(self.0), self.1)
    }

    pub fn listp(&self) -> bool {
        Bool(sexp_listp((self.1).0, self.0)) == Bool::TRUE
    }
}

impl <'a> fmt::Debug for Pair<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.listp() {
            fmt.write_str("(")?;
            let mut h = self.car();
            let mut t = self.cdr();
            while !t.nullp() {
                h.fmt(fmt)?;
                fmt.write_str(" ")?;
                let pair = t.pair().unwrap();
                h = pair.car();
                t = pair.cdr();
            }
            h.fmt(fmt)?;
            fmt.write_str(")")
        } else {
            fmt.write_fmt(format_args!("({:?} . {:?})", self.car(), self.cdr()))
        }
    }
}

pub struct Nil(sexp);

impl fmt::Debug for Nil {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("()")
    }
}

pub const NIL: Nil = Nil(SEXP_NULL);

impl PartialEq for Nil {
    fn eq(self: &Self, _rhs: &Self) -> bool {
        true
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
        fmt.write_str(&format!("#\\{}", (sexp_unbox_character(self.0) as u8) as char))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_unbox_character(self.0) ==
            sexp_unbox_character(rhs.0)
    }
}

pub struct Bool(sexp);

impl Bool {
    const TRUE: Bool = Bool(SEXP_TRUE);
}

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

pub struct Context(sexp);

impl Context {
    fn eval_string<T: Into<std::string::String>>(&self, t: T) -> Result<SExp, ffi::NulError> {
        let string = ffi::CString::new(t.into())?;
        Ok(SExp(
            unsafe {
                sexp_eval_string(self.0, string.as_ptr(), -1, ptr::null_mut())
            }, self))
    }
}

impl Default for Context {
    fn default() -> Self {
        Context(unsafe {
            sexp_make_eval_context(ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 0, 0)
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sexp_destroy_context(self.0) };
    }
}

mod tests {

    use std::ffi;
    use crate::*;

    #[test]
    fn test_pair() {
        let context = Context::default();
        assert_eq!("(#t . #f)", format!("{:?}", context.eval_string("'(#t . #f)").unwrap()));
        assert_eq!("(#\\c #t (#t . #f))", format!("{:?}", context.eval_string("'(#\\c #t (#t . #f))").unwrap()))
    }

    #[test]
    fn test_nil() {
        let context = Context::default();
        assert_eq!(context.eval_string("'()").unwrap().nil(), Some(NIL))
    }

    #[test]
    fn test_bool() {
        let context = Context::default();
        assert_eq!(context.eval_string("#t").unwrap().bool(), Some(true.into()));
        assert_eq!(context.eval_string("#f").unwrap().bool(), Some(false.into()));
        assert_eq!(context.eval_string("#t").unwrap().char(), None);
    }

    #[test]
    fn test_char() {
        let context = Context::default();
        assert_eq!(context.eval_string("#\\s").unwrap().bool(), None);
        assert_eq!(context.eval_string("#\\h").unwrap().char(), Some('h'.into()))
    }

    #[test]
    fn test_string() {

        let context = Context::default();
        let a: &ffi::CStr = {
            context.eval_string("\"a\"").unwrap().string().unwrap().into()
        };
        let b: &ffi::CStr = {
            context.eval_string("\"b\"").unwrap().string().unwrap().into()
        };
        assert_ne!(a, b);
    }
}
