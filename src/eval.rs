use std::ffi;
use std::ptr;

use chibi_scheme_sys::*;
use crate::sexp::SExp;

pub struct Context(sexp);

impl Context {
    fn eval_string<T: Into<String>>(&mut self, t: T) -> Result<SExp, ffi::NulError> {
        let string = ffi::CString::new(t.into())?;
        Ok(SExp(unsafe {
            sexp_eval_string(self.0, string.as_ptr(), -1, ptr::null_mut())
        }))
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

    use crate::eval::Context;
    use std::ffi;

    #[test]
    fn test_bool() {
        let mut context = Context::default();
        assert_eq!(context.eval_string("#t").unwrap().bool(), Some(true.into()));
        assert_eq!(context.eval_string("#f").unwrap().bool(), Some(false.into()));
        assert_eq!(context.eval_string("#t").unwrap().char(), None);
        assert_eq!(context.eval_string("#t").unwrap().string(), None);
    }

    #[test]
    fn test_char() {
        let mut context = Context::default();
        assert_eq!(context.eval_string("#\\s").unwrap().bool(), None);
        assert_eq!(context.eval_string("#\\s").unwrap().string(), None);
        assert_eq!(context.eval_string("#\\h").unwrap().char(), Some('h'.into()))
    }

    #[test]
    fn test_string() {
        let mut context = Context::default();

        assert_eq!(context.eval_string("\"foo\"").unwrap().bool(), None);
        assert_eq!(context.eval_string("\"a\"").unwrap().char(), None);
        assert_eq!(context.eval_string("\"bar\"").unwrap().string(), context.eval_string("\"bar\"").unwrap().string());
        assert_ne!(context.eval_string("\"foo\"").unwrap().string(), context.eval_string("\"bar\"").unwrap().string());
    }
}
