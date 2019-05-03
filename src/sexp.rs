use chibi_scheme_derive::SExp;
use chibi_scheme_sys::*;
use std::ffi;
use std::fmt;
use std::ops;
use std::os::raw;
use std::ptr;
use std::slice;
use std::string::String as RustString;

pub struct RawSExp<'a> {
    sexp: sexp,
    context: Option<&'a Context>,
}

impl RawSExp<'_> {
    const fn new(sexp: sexp) -> Self {
        RawSExp {
            sexp: sexp,
            context: None,
        }
    }
}

#[derive(PartialEq)]
pub enum SExp<'a> {
    String(String<'a>),
    Bool(Bool),
    Char(Char),
    Integer(Integer),
    Rational(Rational<'a>),
    Null(Null),
    Symbol(Symbol<'a>),
    Pair(Pair<'a>),
    Exception(Exception<'a>),
    Void(Void),
    Env(Env<'a>)
}

impl<'a> ops::Deref for SExp<'a> {
    type Target = RawSExp<'a>;
    fn deref(&self) -> &Self::Target {
        match self {
            SExp::String(s) => s,
            SExp::Bool(b) => b,
            SExp::Char(c) => c,
            SExp::Integer(i) => i,
            SExp::Rational(r) => r,
            SExp::Null(n) => n,
            SExp::Symbol(s) => s,
            SExp::Pair(p) => p,
            SExp::Exception(e) => e,
            SExp::Void(v) => v,
            SExp::Env(e) => e
        }
    }
}

impl<'a> fmt::Debug for SExp<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            SExp::Bool(b) => b.fmt(fmt),
            SExp::Char(c) => c.fmt(fmt),
            SExp::Integer(i) => i.fmt(fmt),
            SExp::Null(n) => n.fmt(fmt),
            SExp::Pair(p) => p.fmt(fmt),
            SExp::String(s) => s.fmt(fmt),
            SExp::Exception(e) => e.fmt(fmt),
            SExp::Rational(r) => r.fmt(fmt),
            SExp::Symbol(s) => s.fmt(fmt),
            SExp::Void(v) => v.fmt(fmt),
            SExp::Env(e) => e.fmt(fmt)
        }
    }
}

#[derive(SExp)]
pub struct String<'a>(RawSExp<'a>);

impl String<'_> {
    fn len(&self) -> usize {
        sexp_string_size(self.sexp) as usize
    }

    fn data(&self) -> &str {
        let len = self.len() + 1;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.sexp) as _, len) };
        let str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) };
        str.to_str().unwrap()
    }
}

impl fmt::Debug for String<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("\"{:?}\"", self.data()))
    }
}

impl From<&String<'_>> for RustString {
    fn from(s: &String) -> RustString {
        RustString::from(s.data())
    }
}

#[derive(SExp)]
pub struct Pair<'a>(RawSExp<'a>);

impl<'a> Pair<'a> {
    pub fn car<'b>(&'b self) -> SExp<'a> {
        let sexp = RawSExp {
            sexp: sexp_car(self.sexp),
            context: self.context,
        };
        sexp.into()
    }

    pub fn cdr<'b>(&'b self) -> SExp<'a> {
        let sexp = RawSExp {
            sexp: sexp_cdr(self.sexp),
            context: self.context,
        };
        sexp.into()
    }

    pub fn is_list(&self) -> bool {
        sexp_truep(sexp_listp(self.context.unwrap().0, self.sexp))
    }
}

impl<'a> fmt::Debug for Pair<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.is_list() {
            fmt.write_str("(")?;
            let mut h = self.car();
            let mut t = self.cdr();
            while match t {
                SExp::Null(_) => false,
                _ => true,
            } {
                if let SExp::Pair(pair) = t {
                    h.fmt(fmt)?;
                    fmt.write_str(" ")?;
                    h = pair.car();
                    t = pair.cdr();
                }
            }
            fmt.write_fmt(format_args!("{:?})", h))
        } else {
            fmt.write_fmt(format_args!("({:?} . {:?})", self.car(), self.cdr()))
        }
    }
}

#[derive(SExp)]
pub struct Null(RawSExp<'static>);

impl fmt::Debug for Null {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("()")
    }
}

pub const NULL: Null = Null(RawSExp::new(SEXP_NULL));

impl PartialEq for Null {
    fn eq(self: &Self, _rhs: &Self) -> bool {
        true
    }
}

#[derive(SExp)]
pub struct Void(RawSExp<'static>);

impl fmt::Debug for Void {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}

pub const VOID: Void = Void(RawSExp::new(SEXP_VOID));

impl PartialEq for Void {
    fn eq(self: &Self, _rhs: &Self) -> bool {
        true
    }
}

#[derive(SExp)]
pub struct Symbol<'a>(RawSExp<'a>);

impl<'a> From<&Symbol<'a>> for String<'a> {
    fn from(s: &Symbol<'a>) -> String<'a> {
        String(RawSExp {
            sexp: sexp_symbol_to_string(s.context.unwrap().0, s.sexp),
            context: s.context,
        })
    }
}

impl<'a> fmt::Debug for Symbol<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{:?}", String::from(self).data()))
    }
}

#[derive(SExp)]
pub struct Char(RawSExp<'static>);

impl From<&Char> for raw::c_char {
    fn from(c: &Char) -> raw::c_char {
        sexp_unbox_character(c.sexp)
    }
}

impl From<raw::c_char> for Char {
    fn from(c: raw::c_char) -> Char {
        Char(RawSExp::new(sexp_make_character(c)))
    }
}

impl fmt::Debug for Char {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!(
            "#\\{}",
            (raw::c_char::from(self) as u8) as char
        ))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        raw::c_char::from(self) == raw::c_char::from(rhs)
    }
}

#[derive(SExp)]
pub struct Bool(RawSExp<'static>);

pub const TRUE: Bool = Bool(RawSExp::new(SEXP_TRUE));
pub const FALSE: Bool = Bool(RawSExp::new(SEXP_FALSE));

impl From<&Bool> for bool {
    fn from(b: &Bool) -> bool {
        sexp_truep(b.sexp)
    }
}

impl PartialEq for Bool {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_truep(self.sexp) && sexp_truep(rhs.sexp) || sexp_not(self.sexp) && sexp_not(rhs.sexp)
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if sexp_truep(self.sexp) {
            fmt.write_str("#t")
        } else {
            fmt.write_str("#f")
        }
    }
}

// A Fixnum http://www.chiark.greenend.org.uk/doc/mit-scheme-doc/html/mit-scheme-user/Fixnum-arithmetic.html#Fixnum-arithmetic
#[derive(SExp)]
pub struct Integer(RawSExp<'static>);

impl From<i64> for Integer {
    fn from(i: i64) -> Integer {
        Integer(RawSExp::new(sexp_make_fixnum(i)))
    }
}

impl From<&Integer> for i64 {
    fn from(i: &Integer) -> i64 {
        sexp_unbox_fixnum(i.sexp)
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{}", i64::from(self)))
    }
}

impl PartialEq for Integer {
    fn eq(self: &Self, rhs: &Self) -> bool {
        i64::from(self) == i64::from(rhs)
    }
}

// https://groups.csail.mit.edu/mac/ftpdir/scheme-7.4/doc-html/scheme_5.html
#[derive(SExp)]
pub struct Rational<'a>(RawSExp<'a>);

impl From<&Rational<'_>> for f64 {
    fn from(i: &Rational) -> f64 {
        sexp_flonum_value(i.sexp)
    }
}

impl<'a> fmt::Debug for Rational<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{}", f64::from(self)))
    }
}

#[derive(SExp)]
pub struct Exception<'a>(RawSExp<'a>);

impl Exception<'_> {
    fn message(&self) -> String {
        String(RawSExp {
            sexp: sexp_exception_message(self.sexp),
            context: self.context,
        })
    }
}

impl<'a> fmt::Debug for Exception<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("error: {:?}", self.message()))
    }
}

#[derive(SExp)]
pub struct Env<'a>(RawSExp<'a>);

impl<'a> fmt::Debug for Env<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("")
    }
}

pub struct Context(sexp);

impl<'a> From<RawSExp<'a>> for SExp<'a> {
    //is the 'static lifetime not the bottom?
    fn from(sexp: RawSExp<'a>) -> SExp<'a> {
        if sexp_booleanp(sexp.sexp) {
            if sexp_truep(sexp.sexp) {
                TRUE.into()
            } else {
                FALSE.into()
            }
        } else if sexp.sexp == SEXP_VOID {
            NULL.into()
        } else if sexp_charp(sexp.sexp) {
            Char(RawSExp::new(sexp.sexp)).into()
        } else if sexp_nullp(sexp.sexp) {
            NULL.into()
        } else if sexp_integerp(sexp.sexp) {
            Integer(RawSExp::new(sexp.sexp)).into()
        } else if sexp_pairp(sexp.sexp) {
            Pair(sexp).into()
        } else if sexp_stringp(sexp.sexp) {
            String(sexp).into()
        } else if sexp_flonump(sexp.sexp) {
            Rational(sexp).into()
        } else if sexp_symbolp(sexp.sexp) {
            Symbol(sexp).into()
        } else if sexp_exceptionp(sexp.sexp) {
            Exception(sexp).into()
        } else if sexp_envp(sexp.sexp) {
            Env(sexp).into()
        } else {
            unimplemented!()
            // panic!("Unexpacted type {:?}", sexp_pointer_tag(sexp.sexp))
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        //TODO: switch to different default
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

impl Context {
    pub fn eval_string(&self, str: &str) -> Result<SExp, Exception> {
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(str.as_bytes()) };
        let sexp = RawSExp {
            sexp: unsafe {
                sexp_eval_string(self.0, c_str.as_ptr(), str.len() as _, ptr::null_mut())
            },
            context: Some(self),
        };
        if sexp_exceptionp(sexp.sexp) {
            Err(Exception(sexp).into())
        } else {
            Ok(sexp.into())
        }
    }

    pub fn standard_env(&mut self) -> Result<Env, Exception> {
        let sexp = RawSExp {
            sexp: unsafe { sexp_load_standard_env(self.0, ptr::null_mut(), SEXP_SEVEN) },
            context: Some(self),
        };
        if sexp_exceptionp(sexp.sexp) {
            Err(Exception(sexp).into())
        } else {
            Ok(Env(sexp))
        }
    }
    pub fn cons<'a>(&self, a: &'a SExp, b: &'a SExp) -> SExp {
        let sexp = RawSExp {
            sexp: sexp_cons(self.0, a.sexp, b.sexp),
            context: Some(self),
        };
        if !sexp_exceptionp(sexp.sexp) {
            Pair(sexp).into()
        } else {
            Exception(sexp).into()
        }
    }

    pub fn flonum(&self, i: f64) -> Rational {
        let sexp = unsafe { sexp_make_flonum(self.0, i) };
        Rational(RawSExp {
            sexp: sexp,
            context: Some(self),
        })
    }

    pub fn string(&self, str: &str) -> String {
        let len = str.len();
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(str.as_bytes()) };
        let sexp = unsafe { sexp_c_string(self.0, c_str.as_ptr(), len as _) };
        String(RawSExp {
            sexp: sexp,
            context: Some(self),
        })
    }

    pub fn intern(&self, str: &str) -> Symbol {
        let len = str.len();
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(str.as_bytes()) };
        let sexp = unsafe { sexp_intern(self.0, c_str.as_ptr(), len as _) };
        Symbol(RawSExp {
            sexp: sexp,
            context: Some(self),
        })
    }
}

mod tests {

    use crate::sexp::*;

    #[test]
    fn test_pair() {
        let context = Context::default();
        assert_eq!(
            Ok(context.cons(&Char::from('c' as raw::c_char).into(), &FALSE.into())),
            context.eval_string("'(#\\c . #f)")
        );

        assert_eq!(
            "(#t . #f)",
            format!("{:?}", context.eval_string("'(#t . #f)").unwrap())
        );
        assert_eq!(
            "(#\\c #t (#t . #f))",
            format!("{:?}", context.eval_string("'(#\\c #t (#t . #f))").unwrap())
        );

        assert_eq!(
            "(#t . (1 . 2))",
            format!("{:?}", context.eval_string("'(#t . (1 . 2))").unwrap())
        );

        assert_eq!(
            "(1 2 3)",
            format!("{:?}", context.eval_string("'(1 2 3)").unwrap())
        );

        assert_eq!(
            "(\"x\" . 1)",
            format!("{:?}", context.eval_string("'(x . 1)").unwrap())
        );
    }

    #[test]
    fn test_null() {
        let context = Context::default();
        assert_eq!(context.eval_string("'()"), Ok(NULL.into()));
    }

    #[test]
    fn test_bool() {
        let context = Context::default();
        assert_eq!(context.eval_string("#t"), Ok(TRUE.into()));
        assert_eq!(context.eval_string("#f"), Ok(FALSE.into()));
    }

    #[test]
    fn test_char() {
        let context = Context::default();
        assert_eq!(
            context.eval_string("#\\h"),
            Ok(Char::from('h' as raw::c_char).into())
        );
    }

    #[test]
    fn test_integer() {
        let context = Context::default();
        assert_eq!(context.eval_string("(+ 1 3)"), Ok(Integer::from(4).into()));

        assert_eq!(
            context.eval_string(&SEXP_MAX_FIXNUM.to_string()),
            Ok(Integer::from(SEXP_MAX_FIXNUM).into())
        );

        assert_eq!(
            context.eval_string(&SEXP_MIN_FIXNUM.to_string()),
            Ok(Integer::from(SEXP_MIN_FIXNUM).into())
        );
    }

    #[test]
    fn test_rational() {
        let context = Context::default();
        assert_eq!("4.5", format!("{:?}", context.eval_string("4.5").unwrap()));
        assert_eq!(context.eval_string("4.5"), Ok(context.flonum(4.5).into()));
    }

    #[test]
    fn test_string() {
        let context = Context::default();
        assert_eq!(
            Ok(context.string("foo").into()),
            context.eval_string("\"foo\"")
        );
    }

    #[test]
    fn test_symbol() {
        let context = Context::default();
        assert_eq!(
            Ok(context.intern("foo").into()),
            context.eval_string("'foo")
        );
    }

    #[test]
    fn test_standard_env() {
        let mut context = Context::default();
        context.standard_env().unwrap();
        context.eval_string("(import (srfi 1))").unwrap();
        assert_eq!(
            Ok(Integer::from(1).into()),
            context.eval_string("(first '(1 2))")
        );
    }

}
