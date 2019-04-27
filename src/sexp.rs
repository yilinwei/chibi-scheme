use chibi_scheme_sys::*;
use std::ffi;
use std::fmt;
use std::ptr;
use std::slice;
use std::string;
use std::ops;

pub struct RawSExp<'a>{
    sexp: sexp,
    context: Option<&'a Context>
}

impl RawSExp<'_> {
    fn new(sexp: sexp) -> Self {
        RawSExp {
            sexp: sexp,
            context: None
        }
    }
}

impl Drop for RawSExp<'_> {
    fn drop(&mut self) {
        if sexp_pointerp(self.sexp) {
            unsafe { sexp_release_object(self.context.unwrap().0, self.sexp) }
        }
    }
}

pub enum SExp<'a> {
    String(String<'a>),
    Bool(Bool),
    Char(Char),
    Integer(Integer),
    Rational(Rational<'a>),
    Null(Null),
    Symbol(Symbol<'a>),
    Pair(Pair<'a>),
    Exception(Exception<'a>)
}

impl <'a> ops::Deref for SExp<'a> {
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
            SExp::Exception(e) => e
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
            SExp::Symbol(s) => s.fmt(fmt)
        }
        // // TODO: Change when PolyMut is merged
        // let cell = cell::RefCell::new(fmt);
        // self.0.to_ref().fold(frunk::hlist![
        //     |b: &Bool| b.fmt(&mut cell.borrow_mut()),
        //     |c: &Char| c.fmt(&mut cell.borrow_mut()),
        //     |i: &Integer| i.fmt(&mut cell.borrow_mut()),
        //     |i: &Rational| i.fmt(&mut cell.borrow_mut()),
        //     |n: &Null| n.fmt(&mut cell.borrow_mut()),
        //     |p: &Pair<'a>| p.fmt(&mut cell.borrow_mut()),
        //     |s: &String<'a>| s.fmt(&mut cell.borrow_mut()),
        //     |s: &Exception<'a>| s.fmt(&mut cell.borrow_mut()),
        //     |s: &Symbol<'a>| s.fmt(&mut cell.borrow_mut())
        // ])
    }
}

pub struct String<'a>(RawSExp<'a>);

impl <'a> ops::Deref for String<'a> {
    type Target = RawSExp<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl String<'_> {

    fn len(&self) -> usize {
        sexp_string_size(self.sexp) as usize
    }

    fn data(&self) -> &ffi::CStr {
        let len = self.len() + 1;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.sexp) as _, len) };
        unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) }
    }
}

impl PartialEq for String<'_> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_truep(unsafe { sexp_equalp(self.context.unwrap().0, self.sexp, rhs.sexp) })
    }
}

impl fmt::Debug for String<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("\"{:?}\"", self.data()))
    }
}

pub struct Pair<'a>(RawSExp<'a>);

//TODO: Macro please
impl <'a> ops::Deref for Pair<'a> {
    type Target = RawSExp<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'a> Pair<'a> {
    pub fn car<'b>(&'b self) -> SExp<'a> {
        let sexp = RawSExp {
            sexp: sexp_car(self.sexp),
            context: self.context
        };
        sexp.into()
    }

    pub fn cdr<'b>(&'b self) -> SExp<'a> {
        let sexp = RawSExp {
            sexp: sexp_cdr(self.sexp),
            context: self.context
        };
        sexp.into()
    }

    pub fn is_list(&self) -> bool {
        sexp_truep(sexp_listp(self.context.unwrap().0, self.sexp))
    }
}

impl<'a> PartialEq for Pair<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_truep(sexp_equalp(self.context.unwrap().0, self.sexp, rhs.sexp))
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
                _ => true
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

pub struct Null(RawSExp<'static>);

impl ops::Deref for Null {
    type Target = RawSExp<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

pub struct Symbol<'a>(RawSExp<'a>);

impl <'a> ops::Deref for Symbol<'a> {
    type Target = RawSExp<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'a> From<&Symbol<'a>> for String<'a> {
    fn from(s: &Symbol<'a>) -> String<'a> {
        String(RawSExp {
            sexp: sexp_symbol_to_string(s.context.unwrap().0, s.sexp),
            context: s.context
        })
    }
}

impl<'a> PartialEq for Symbol<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_truep(sexp_equalp(self.context.unwrap().0, self.sexp, rhs.sexp))
    }
}

impl<'a> fmt::Debug for Symbol<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{:?}", String::from(self).data()))
    }
}

pub struct Char(RawSExp<'static>);

impl ops::Deref for Char {
    type Target = RawSExp<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Char> for char {
    fn from(c: &Char) -> char {
        (sexp_unbox_character(c.sexp) as u8) as char
    }
}

impl fmt::Debug for Char {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("#\\{}", char::from(self)))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        char::from(self) == char::from(rhs)
    }
}

pub struct Bool(RawSExp<'static>);

impl ops::Deref for Bool {
    type Target = RawSExp<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Bool {
    pub const TRUE: Bool = Bool(RawSExp { sexp: SEXP_TRUE, context: None });
    pub const FALSE: Bool = Bool(RawSExp { sexp: SEXP_FALSE, context: None });
}

impl From<&Bool> for bool {
    fn from(b: &Bool) -> bool {
        sexp_truep(b.sexp)
    }
}

impl PartialEq for Bool {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_truep(self.sexp) && sexp_truep(rhs.sexp) ||
            sexp_not(self.sexp) && sexp_not(rhs.sexp)
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
pub struct Integer(RawSExp<'static>);

impl ops::Deref for Integer {
    type Target = RawSExp<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i32> for Integer {
    fn from(i: i32) -> Integer {
        Integer(RawSExp::new(sexp_make_fixnum(i as _)))
    }
}

impl From<i64> for Integer {
    fn from(i: i64) -> Integer {
        Integer(RawSExp::new(sexp_make_fixnum(i as _)))
    }
}

impl From<&Integer> for i64 {
    fn from(i: &Integer) -> i64 {
        sexp_unbox_fixnum(i.sexp) as _
    }
}

impl From<&Integer> for i32 {
    fn from(i: &Integer) -> i32 {
        sexp_unbox_fixnum(i.sexp) as _
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{}", i32::from(self)))
    }
}

impl PartialEq for Integer {
    fn eq(self: &Self, rhs: &Self) -> bool {
        i32::from(self) == i32::from(self)
    }
}

// https://groups.csail.mit.edu/mac/ftpdir/scheme-7.4/doc-html/scheme_5.html
pub struct Rational<'a>(RawSExp<'a>);

impl <'a> ops::Deref for Rational<'a> {
    type Target = RawSExp<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Rational<'_>> for f32 {
    fn from(i: &Rational) -> f32 {
        sexp_flonum_value(i.sexp) as _
    }
}

impl From<&Rational<'_>> for f64 {
    fn from(i: &Rational) -> f64 {
        sexp_flonum_value(i.sexp) as _
    }
}

impl<'a> fmt::Debug for Rational<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("{}", f64::from(self)))
    }
}

impl<'a> PartialEq for Rational<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        f64::from(self) == f64::from(rhs)
    }
}

pub struct Exception<'a>(RawSExp<'a>);

impl <'a> ops::Deref for Exception<'a> {
    type Target = RawSExp<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Exception<'_> {
    fn message(&self) -> String {
        String(RawSExp {
            sexp: sexp_exception_message(self.sexp),
            context: self.context
        })
    }
}

impl<'a> fmt::Debug for Exception<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        //TODO: What should this be printed as?
        fmt.write_fmt(format_args!("Exception: {:?}", self.message()))
    }
}

pub struct Context(sexp);

impl<'a> From<RawSExp<'a>> for SExp<'a> {
    //is the 'static lifetime not the bottom?
    fn from(sexp: RawSExp<'a>) -> SExp<'a> {
        if sexp_booleanp(sexp.sexp) {
            if sexp_truep(sexp.sexp) {
                SExp::Bool(Bool::TRUE)
            } else {
                SExp::Bool(Bool::FALSE)
            }
        } else if sexp_charp(sexp.sexp) {
            SExp::Char(Char(RawSExp::new(sexp.sexp)))
        } else if sexp_nullp(sexp.sexp) {
            SExp::Null(NULL)
        } else if sexp_pairp(sexp.sexp) {
            SExp::Pair(Pair(sexp))
        } else if sexp_stringp(sexp.sexp) {
            SExp::String(String(sexp))
        } else if sexp_flonump(sexp.sexp) {
            SExp::Rational(Rational(sexp))
        } else {
            unreachable!()
        }
        // let coprod = if RawContext::booleanp(&self) {
        //     Coprod::inject(Bool(RawSExp(self.0, None)))
        // } else if RawContext::charp(&self) {
        //     Coprod::inject(Char(RawSExp(self.0, None)))
        // } else if RawContext::nullp(&self) {
        //     Coprod::inject(NULL)
        // } else if RawContext::pairp(&self) {
        //     Coprod::inject(Pair(self))
        // } else if RawContext::stringp(&self) {
        //     Coprod::inject(String(self))
        // } else if RawContext::integerp(&self) {
        //     Coprod::inject(Integer(RawSExp(self.0, None)))
        // } else if RawContext::flonump(&self) {
        //     Coprod::inject(Rational(self))
        // } else if RawContext::symbolp(&self) {
        //     Coprod::inject(Symbol(self))
        // } else if RawContext::exceptionp(&self) {
        //     Coprod::inject(Exception(self))
        // } else {
        //     unreachable!()
        // };
    }
}

impl Default for Context {
    fn default() -> Self {
        //TODO: switch to different default
        Context(sexp_make_eval_context(ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 0, 0))
    }
}

impl Context {

    pub fn eval_string<T: Into<string::String>>(&self, t: T) -> Result<SExp, ffi::NulError> {
        let string = ffi::CString::new(t.into())?;
        let sexp = RawSExp {
            sexp: unsafe { sexp_eval_string(self.0, string.as_ptr(), -1, ptr::null_mut()) },
            context: Some(self),
        };
        Ok(sexp.into())
    }

    pub fn cons<'a>(&self, a: &'a SExp, b: &'a SExp) -> SExp {
        let sexp = RawSExp {
            sexp: sexp_cons(self.0, a.sexp, b.sexp),
            context: Some(self)
        };
        if !sexp_exceptionp(sexp.sexp) {
            SExp::Pair(Pair(sexp))
        } else {
            SExp::Exception(Exception(sexp))
        }
    }

    pub fn flonum(&self, i: f64) -> Rational {
        let sexp = { sexp_make_flonum(self.0, i) };
        Rational(RawSExp {
            sexp: sexp,
            context: Some(self)
        })
    }
}

mod tests {

    use crate::sexp::*;
    use crate::*;
    use std::ffi;

    #[test]
    fn test_pair() {
        let context = Context::default();
        assert_eq!(
            Some(context.cons(&SExp::from(Char::from('c')), &SExp::from(Bool::FALSE))),
            context
                .eval_string("'(#\\c . #f)")
                .unwrap()
                .expect::<Pair, _>()
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

        // We now require generic serialization
        // frunnk has a labelled generic (with a Repr)
        // we need to take the repr, and transform field names into symbols and values into values
        // first let's just write the methods
        let pair_struct_string = "'((x . 1) (y . 2))";
        let pair_struct = context
            .eval_string(pair_struct_string)
            .unwrap()
            .expect::<Pair, _>()
            .unwrap();
        let pair_x = pair_struct.car().expect::<Pair, _>().unwrap();
        let x_symbol = pair_x.car().expect::<Symbol, _>().unwrap();
        let x_value = pair_x.cdr().expect::<Integer, _>().unwrap();
        let pair_y = pair_struct.cdr().expect::<Pair, _>().unwrap();
        let x_value_i64: i64 = x_value.into();
        assert_eq!(x_value_i64, 1)
    }

    #[test]
    fn test_null() {
        let context = Context::default();
        assert_eq!(
            context.eval_string("'()").unwrap().0.take::<Null, _>(),
            Some(NULL)
        );
    }

    #[test]
    fn test_bool() {
        let context = Context::default();
        assert_eq!(
            context.eval_string("#t").unwrap().0.take::<Bool, _>(),
            Some(true.into())
        );
        assert_eq!(
            context.eval_string("#f").unwrap().expect::<Bool, _>(),
            Some(false.into())
        );
        assert_eq!(context.eval_string("#t").unwrap().expect::<Char, _>(), None);
    }

    #[test]
    fn test_char() {
        let context = Context::default();
        assert_eq!(
            context.eval_string("#\\s").unwrap().expect::<Bool, _>(),
            None
        );
        assert_eq!(
            context.eval_string("#\\h").unwrap().expect::<Char, _>(),
            Some('h'.into())
        )
    }

    #[test]
    fn test_integer() {
        let context = Context::default();
        assert_eq!(
            context.eval_string("#\\s").unwrap().expect::<Integer, _>(),
            None
        );
        assert_eq!(
            context
                .eval_string("(+ 1 3)")
                .unwrap()
                .expect::<Integer, _>(),
            Some(4.into())
        );

        // let max_value = "9223372036854775807";
        let max_value = u32::max_value().to_string();
        println!("Max value: {:}", max_value);
        assert_eq!(
            context
                .eval_string(max_value)
                .unwrap()
                .expect::<Integer, _>(),
            Some(i64::max_value().into())
        )
    }

    // TODO: What are the constraints of the system?
    #[test]
    fn test_rational() {
        let context = Context::default();
        assert_eq!("4.5", format!("{:?}", context.eval_string("4.5").unwrap()));

        context.make_flonum(4.5);
    }

    #[test]
    fn test_string() {
        let context = Context::default();
        let foo = context
            .eval_string("\"foo\"")
            .unwrap()
            .expect::<String, _>();

        let bar = context
            .eval_string("\"bar\"")
            .unwrap()
            .expect::<String, _>();
        assert_eq!(foo, foo);
        assert_ne!(foo, bar);
        assert_eq!("\"foo\"", format!("{:?}", foo.unwrap().data()));
    }

    #[test]
    fn test_symbol() {
        let context = Context::default();
        let foo = context.eval_string("'foo").unwrap().expect::<Symbol, _>();

        let bar = context.eval_string("'bar").unwrap().expect::<Symbol, _>();
        assert_eq!(foo, foo);
        assert_ne!(foo, bar);
    }
}
