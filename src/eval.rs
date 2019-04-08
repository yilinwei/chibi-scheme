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

mod tests {

    use crate::eval::Context;

    #[test]
    fn test_bool() {
        let mut context = Context::default();
        assert_eq!(context.eval_string("#t").unwrap().bool(), Some(true.into()));
        assert_eq!(context.eval_string("#f").unwrap().bool(), Some(false.into()));
        assert_eq!(context.eval_string("#t").unwrap().char(), None);
    }

    #[test]
    fn test_char() {
        let mut context = Context::default();
        assert_eq!(context.eval_string("#\\s").unwrap().bool(), None);
        assert_eq!(context.eval_string("#\\h").unwrap().char(), Some('h'.into()))
    }
}
