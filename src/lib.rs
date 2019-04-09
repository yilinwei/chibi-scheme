use chibi_scheme_sys::*;
use frunk;
use std::cell;
use std::ffi;
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::string;

pub struct RawSExp<'a>(sexp, Option<&'a RawContext>);

// TODO: Add generic typing please
pub type Coprod<'a> = frunk::Coprod!(Bool, Char, Null, Pair<'a>, String<'a>);

//
pub struct SExp<'a>(Coprod<'a>);

impl<'a> SExp<'a> {
    pub fn from<T, Index>(t: T) -> SExp<'a>
    where
        Coprod<'a>: frunk::coproduct::CoprodInjector<T, Index>,
    {
        SExp(Coprod::inject(t))
    }

    pub fn expect<T, Index>(self) -> Option<T>
    where
        Coprod<'a>: frunk::coproduct::CoproductTaker<T, Index>,
    {
        self.0.take::<T, _>()
    }
}

impl<'a> RawSExp<'a> {
    fn booleanp(&self) -> bool {
        sexp_booleanp(self.0)
    }

    fn charp(&self) -> bool {
        sexp_charp(self.0)
    }

    fn stringp(&self) -> bool {
        sexp_stringp(self.0)
    }

    fn nullp(&self) -> bool {
        sexp_nullp(self.0)
    }

    fn pairp(&self) -> bool {
        sexp_pairp(self.0)
    }
}

impl<'a> fmt::Debug for SExp<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO: Change when PolyMut is merged
        let cell = cell::RefCell::new(fmt);
        self.0.to_ref().fold(frunk::hlist![
            |b: &Bool| b.fmt(&mut cell.borrow_mut()),
            |c: &Char| c.fmt(&mut cell.borrow_mut()),
            |n: &Null| n.fmt(&mut cell.borrow_mut()),
            |p: &Pair<'a>| p.fmt(&mut cell.borrow_mut()),
            |s: &String<'a>| s.fmt(&mut cell.borrow_mut())
        ])
    }
}

pub struct String<'a>(RawSExp<'a>);

impl<'a> String<'a> {}

impl<'a> PartialEq for String<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        (self.0).1.unwrap().equalp(&self.0, &rhs.0)
    }
}

// impl<'a> Drop for String<'a> {
//     fn drop(&mut self) {
//         unsafe { sexp_release_object((self.1).0, self.0) }
//     }
// }

// impl<'a> Into<&'a ffi::CStr> for String<'a> {
//     fn into(self: Self) -> &'a ffi::CStr {
//         let len = (sexp_string_size(self.0) + 1) as _;
//         let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
//         let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) };
//         // Only drop once the context is destroyed
//         mem::forget(self);
//         c_str
//     }
// }

// impl<'a> Into<ffi::CString> for String<'a> {
//     fn into(self: Self) -> ffi::CString {
//         let len = (sexp_string_size(self.0)) as _;
//         let mut data = Vec::with_capacity(len);
//         let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
//         data.extend_from_slice(slice);
//         unsafe { ffi::CString::from_vec_unchecked(data) }
//     }
// }

impl<'a> fmt::Debug for String<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c_str =
            unsafe { ffi::CStr::from_bytes_with_nul_unchecked(RawContext::string_data(&self.0)) };
        fmt.write_fmt(format_args!("\"{:?}\"", c_str))
    }
}

pub struct Pair<'a>(RawSExp<'a>);

impl<'a> Pair<'a> {
    pub fn car<'b>(&'b self) -> SExp<'a> {
        RawContext::car(&self.0).into()
    }

    pub fn cdr<'b>(&'b self) -> SExp<'a> {
        RawContext::cdr(&self.0).into()
    }
}

impl<'a> PartialEq for Pair<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        (self.0).1.unwrap().equalp(&self.0, &rhs.0)
    }
}

impl<'a> fmt::Debug for Pair<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if (self.0).1.unwrap().listp(&self.0) {
            fmt.write_str("(")?;
            let mut h = self.car();
            let mut t = self.cdr();
            while t.0.to_ref().fold(frunk::hlist![
                |_| unreachable!(),
                |_| unreachable!(),
                |_: &Null| false,
                |_: &Pair| true,
                |_| unreachable!()
            ]) {
                h.fmt(fmt)?;
                fmt.write_str(" ")?;
                let pair = t.expect::<Pair, _>().unwrap();
                h = pair.car();
                t = pair.cdr();
            }
            fmt.write_fmt(format_args!("{:?})", h))
        } else {
            fmt.write_fmt(format_args!("({:?} . {:?})", self.car(), self.cdr()))
        }
    }
}

pub struct Null(RawSExp<'static>);

impl fmt::Debug for Null {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("()")
    }
}

pub const NULL: Null = Null(RawSExp(SEXP_NULL, None));

impl PartialEq for Null {
    fn eq(self: &Self, _rhs: &Self) -> bool {
        true
    }
}

pub struct Symbol(sexp);

pub struct Char(RawSExp<'static>);

impl Into<char> for Char {
    fn into(self: Self) -> char {
        //TODO: Need to check casting
        (sexp_unbox_character((self.0).0) as u8) as char
    }
}

impl From<char> for Char {
    fn from(c: char) -> Char {
        //TODO: check
        Char(RawSExp(sexp_make_character(c as _), None))
    }
}

impl fmt::Debug for Char {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("#\\{}", RawContext::unbox_character(&self.0)))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        RawContext::unbox_character(&self.0) as char == RawContext::unbox_character(&rhs.0)
    }
}

pub struct Bool(RawSExp<'static>);

impl Bool {
    const TRUE: Bool = Bool(RawSExp(SEXP_TRUE, None));
    const FALSE: Bool = Bool(RawSExp(SEXP_FALSE, None));
}

impl Into<bool> for Bool {
    fn into(self: Self) -> bool {
        RawContext::truep(&self.0)
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Bool {
        if b {
            Bool::TRUE
        } else {
            Bool::FALSE
        }
    }
}

impl PartialEq for Bool {
    fn eq(self: &Self, rhs: &Self) -> bool {
        RawContext::truep(&self.0) && RawContext::truep(&rhs.0)
            || RawContext::not(&self.0) && RawContext::not(&rhs.0)
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if RawContext::truep(&self.0) {
            fmt.write_str("#t")
        } else {
            fmt.write_str("#f")
        }
    }
}

struct RawContext(sexp);

impl RawContext {
    fn unbox_character(a: &RawSExp) -> char {
        (sexp_unbox_character(a.0) as u8) as char
    }

    fn string_data<'a>(a: &'a RawSExp) -> &'a [u8] {
        let len = RawContext::string_size(a) + 1;
        // let len = (RawContext::string_size(&self.0) + 1) as _;
        unsafe { slice::from_raw_parts(sexp_string_data(a.0) as _, len) }
    }
    fn string_size(a: &RawSExp) -> usize {
        sexp_string_size(a.0) as usize
    }
    fn equalp(&self, a: &RawSExp, b: &RawSExp) -> bool {
        RawContext::truep(&RawSExp(sexp_equalp(self.0, a.0, b.0), Some(self)))
    }

    fn cons(&self, a: &RawSExp, b: &RawSExp) -> RawSExp {
        RawSExp(sexp_cons(self.0, a.0, b.0), Some(self))
    }

    fn listp(&self, a: &RawSExp) -> bool {
        RawContext::truep(&RawSExp(sexp_listp(self.0, a.0), Some(self)))
    }

    fn car<'a, 'b>(sexp: &'a RawSExp<'b>) -> RawSExp<'b> {
        RawSExp(sexp_car(sexp.0), sexp.1)
    }

    fn cdr<'a, 'b>(sexp: &'a RawSExp<'b>) -> RawSExp<'b> {
        RawSExp(sexp_cdr(sexp.0), sexp.1)
    }

    fn booleanp(sexp: &RawSExp) -> bool {
        sexp_booleanp(sexp.0)
    }

    fn charp(sexp: &RawSExp) -> bool {
        sexp_charp(sexp.0)
    }

    fn stringp(sexp: &RawSExp) -> bool {
        sexp_stringp(sexp.0)
    }

    fn truep(sexp: &RawSExp) -> bool {
        sexp_truep(sexp.0)
    }

    fn not(sexp: &RawSExp) -> bool {
        sexp_not(sexp.0)
    }

    fn nullp(sexp: &RawSExp) -> bool {
        sexp_nullp(sexp.0)
    }

    fn pairp(sexp: &RawSExp) -> bool {
        sexp_pairp(sexp.0)
    }

    fn eval_string<T: Into<string::String>>(&self, t: T) -> Result<RawSExp, ffi::NulError> {
        let string = ffi::CString::new(t.into())?;
        Ok(RawSExp(
            unsafe { sexp_eval_string(self.0, string.as_ptr(), -1, ptr::null_mut()) },
            Some(self),
        ))
    }
}

pub struct Context(RawContext);

impl<'a> Into<SExp<'a>> for RawSExp<'a> {
    //is the 'static lifetime not the bottom?
    fn into(self: Self) -> SExp<'a> {
        let coprod = if RawContext::booleanp(&self) {
            Coprod::inject(Bool(RawSExp(self.0, None)))
        } else if RawContext::charp(&self) {
            Coprod::inject(Char(RawSExp(self.0, None)))
        } else if RawContext::nullp(&self) {
            Coprod::inject(NULL)
        } else if RawContext::pairp(&self) {
            Coprod::inject(Pair(self))
        } else if RawContext::stringp(&self) {
            Coprod::inject(String(self))
        } else {
            unreachable!()
        };
        SExp(coprod)
    }
}

impl Default for Context {
    fn default() -> Self {
        Context(RawContext::default())
    }
}

impl Context {
    fn eval_string<T: Into<string::String>>(&self, t: T) -> Result<SExp, ffi::NulError> {
        self.0.eval_string(t).map(|x| x.into())
    }
    //The problem
    fn cons<'a>(&self, a: &'a SExp, b: &'a SExp) -> Pair {
        let f = frunk::hlist![
            |b: &'a Bool| &b.0,
            |c: &'a Char| &c.0,
            |n: &'a Null| &n.0,
            |p: &'a Pair<'a>| &p.0,
            |s: &'a String<'a>| &s.0
        ];

        let sexp: SExp = self
            .0
            .cons(a.0.to_ref().fold(f), b.0.to_ref().fold(f))
            .into();
        sexp.expect::<Pair, _>().unwrap()
    }

    // TODO: Lens please
    pub fn listp<'a>(&self, pair: &'a Pair) -> bool {
        self.0.listp(&pair.0)
    }
}

impl Default for RawContext {
    fn default() -> Self {
        RawContext(unsafe {
            sexp_make_eval_context(ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 0, 0)
        })
    }
}

impl Drop for RawContext {
    fn drop(&mut self) {
        unsafe { sexp_destroy_context(self.0) };
    }
}

mod tests {

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
    }
}
