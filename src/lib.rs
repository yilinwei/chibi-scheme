use chibi_scheme_sys::*;
use frunk;
use std::cell;
use std::ffi;
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::string;

pub struct RawSExp<'a>(sexp, &'a Context);

pub type Coprod<'a> = frunk::Coprod!(Bool, Char, Null, Pair<'a>, String<'a>);

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

    fn typed(self) -> SExp<'a> {
        let coprod = if self.booleanp() {
            Coprod::inject(Bool(self.0))
        } else if self.charp() {
            Coprod::inject(Char(self.0))
        } else if self.nullp() {
            Coprod::inject(NULL)
        } else if self.pairp() {
            Coprod::inject(Pair(self.0, self.1))
        } else if self.stringp() {
            Coprod::inject(String(self.0, self.1))
        } else {
            unreachable!()
        };
        SExp(coprod)
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

pub struct String<'a>(sexp, &'a Context);

impl<'a> String<'a> {}

impl<'a> PartialEq for String<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        RawSExp(sexp_equalp((self.1).0, self.0, rhs.0), self.1)
            .typed()
            .expect::<Bool, _>()
            .unwrap()
            == Bool::TRUE
    }
}

impl<'a> Drop for String<'a> {
    fn drop(&mut self) {
        unsafe { sexp_release_object((self.1).0, self.0) }
    }
}

impl<'a> Into<&'a ffi::CStr> for String<'a> {
    fn into(self: Self) -> &'a ffi::CStr {
        let len = (sexp_string_size(self.0) + 1) as _;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) };
        // Only drop once the context is destroyed
        mem::forget(self);
        c_str
    }
}

impl<'a> Into<ffi::CString> for String<'a> {
    fn into(self: Self) -> ffi::CString {
        let len = (sexp_string_size(self.0)) as _;
        let mut data = Vec::with_capacity(len);
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        data.extend_from_slice(slice);
        unsafe { ffi::CString::from_vec_unchecked(data) }
    }
}

impl<'a> fmt::Debug for String<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let len = (sexp_string_size(self.0) + 1) as _;
        let slice = unsafe { slice::from_raw_parts(sexp_string_data(self.0) as _, len) };
        let c_str = unsafe { ffi::CStr::from_bytes_with_nul_unchecked(slice) };
        fmt.write_fmt(format_args!("\"{:?}\"", c_str))
    }
}

pub struct Pair<'a>(sexp, &'a Context);

impl<'a> Pair<'a> {
    pub fn car(&self) -> RawSExp<'a> {
        RawSExp(sexp_car(self.0), self.1)
    }

    pub fn cdr(&self) -> RawSExp<'a> {
        RawSExp(sexp_cdr(self.0), self.1)
    }

    pub fn listp(&self) -> bool {
        Bool(sexp_listp((self.1).0, self.0)) == Bool::TRUE
    }
}

impl<'a> PartialEq for Pair<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        RawSExp(sexp_equalp((self.1).0, self.0, rhs.0), self.1)
            .typed()
            .expect::<Bool, _>()
            .unwrap()
            == Bool::TRUE
    }
}

impl<'a> fmt::Debug for Pair<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.listp() {
            fmt.write_str("(")?;
            let mut h = self.car();
            let mut t = self.cdr();
            while !t.nullp() {
                h.typed().fmt(fmt)?;
                fmt.write_str(" ")?;
                let pair = t.typed().0.take::<Pair, _>().unwrap();
                h = pair.car();
                t = pair.cdr();
            }
            fmt.write_fmt(format_args!("{:?})", h.typed()))
        } else {
            fmt.write_fmt(format_args!(
                "({:?} . {:?})",
                self.car().typed(),
                self.cdr().typed()
            ))
        }
    }
}

pub struct Null(sexp);

impl fmt::Debug for Null {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("()")
    }
}

pub const NULL: Null = Null(SEXP_NULL);

impl PartialEq for Null {
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
        fmt.write_fmt(format_args!(
            "#\\{}",
            (sexp_unbox_character(self.0) as u8) as char
        ))
    }
}

impl PartialEq for Char {
    fn eq(self: &Self, rhs: &Self) -> bool {
        sexp_unbox_character(self.0) == sexp_unbox_character(rhs.0)
    }
}

pub struct Bool(sexp);

impl Bool {
    const TRUE: Bool = Bool(SEXP_TRUE);
    const FALSE: Bool = Bool(SEXP_FALSE);
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
        sexp_truep(self.0) && sexp_truep(rhs.0) || sexp_not(self.0) && sexp_not(rhs.0)
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
    fn eval_string<T: Into<string::String>>(&self, t: T) -> Result<RawSExp, ffi::NulError> {
        let string = ffi::CString::new(t.into())?;
        Ok(RawSExp(
            unsafe { sexp_eval_string(self.0, string.as_ptr(), -1, ptr::null_mut()) },
            self,
        ))
    }

    fn cons<'a>(&self, a: &'a SExp, b: &'a SExp) -> Pair {
        let f = frunk::hlist![
            |b: &Bool| b.0,
            |c: &Char| c.0,
            |n: &Null| n.0,
            |p: &Pair<'a>| p.0,
            |s: &String<'a>| s.0
        ];
        RawSExp(
            sexp_cons(self.0, a.0.to_ref().fold(f), b.0.to_ref().fold(f)),
            self,
        ).typed()
        .expect::<Pair, _>()
        .unwrap()
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

    use crate::*;
    use std::ffi;

    #[test]
    fn test_pair() {
        let context = Context::default();
        assert_eq!(
            Some(context.cons(&SExp::from(Bool::TRUE), &SExp::from(Bool::FALSE))),
            context
                .eval_string("'(#t . #f)")
                .unwrap()
                .typed()
                .expect::<Pair, _>()
        );

        assert_eq!(
            "(#t . #f)",
            format!("{:?}", context.eval_string("'(#t . #f)").unwrap().typed())
        );
        assert_eq!(
            "(#\\c #t (#t . #f))",
            format!(
                "{:?}",
                context.eval_string("'(#\\c #t (#t . #f))").unwrap().typed()
            )
        );
    }

    #[test]
    fn test_null() {
        let context = Context::default();
        assert_eq!(
            context
                .eval_string("'()")
                .unwrap()
                .typed()
                .0
                .take::<Null, _>(),
            Some(NULL)
        );
    }

    #[test]
    fn test_bool() {
        let context = Context::default();
        assert_eq!(
            context
                .eval_string("#t")
                .unwrap()
                .typed()
                .0
                .take::<Bool, _>(),
            Some(true.into())
        );
        assert_eq!(
            context
                .eval_string("#f")
                .unwrap()
                .typed()
                .expect::<Bool, _>(),
            Some(false.into())
        );
        assert_eq!(
            context
                .eval_string("#t")
                .unwrap()
                .typed()
                .expect::<Char, _>(),
            None
        );
    }

    #[test]
    fn test_char() {
        let context = Context::default();
        assert_eq!(
            context
                .eval_string("#\\s")
                .unwrap()
                .typed()
                .expect::<Bool, _>(),
            None
        );
        assert_eq!(
            context
                .eval_string("#\\h")
                .unwrap()
                .typed()
                .expect::<Char, _>(),
            Some('h'.into())
        )
    }

    #[test]
    fn test_string() {
        let context = Context::default();
        let foo = context
            .eval_string("\"foo\"")
            .unwrap()
            .typed()
            .expect::<String, _>();

        let bar = context
            .eval_string("\"bar\"")
            .unwrap()
            .typed()
            .expect::<String, _>();
        assert_eq!(foo, foo);
        assert_ne!(foo, bar);
    }
}
