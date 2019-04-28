#![recursion_limit="128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(SExp)]
pub fn sexp_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_sexp_macro(&ast)
}

fn impl_sexp_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let lifetime = &ast
        .generics
        .lifetimes()
        .next();

    let gen = match lifetime {
        Some(lifetime) =>
            quote! {
                impl <#lifetime> std::ops::Deref for #name<#lifetime> {
                    type Target = crate::sexp::RawSExp<#lifetime>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                impl Drop for #name<'_> {
                    fn drop(&mut self) {
                        if let Some(context) = self.context {
                            unsafe { chibi_scheme_sys::sexp_release_object(context.0, self.sexp)}
                        }
                    }
                }
                impl<#lifetime> PartialEq for #name<#lifetime> {
                    fn eq(self: &Self, rhs: &Self) -> bool {
                        chibi_scheme_sys::sexp_truep(
                            chibi_scheme_sys::sexp_equalp(self.context.unwrap().0, self.sexp, rhs.sexp)
                        )
                    }
                }
                impl <#lifetime> From<#name<#lifetime>> for crate::sexp::SExp<#lifetime> {
                    fn from(sexp: #name) -> crate::sexp::SExp {
                        crate::sexp::SExp::#name(sexp)
                    }
                }
            },
        None =>
            quote! {
                impl std::ops::Deref for #name {
                    type Target = crate::sexp::RawSExp<'static>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                impl From<#name> for crate::sexp::SExp<'static> {
                    fn from(sexp: #name) -> crate::sexp::SExp<'static> {
                        crate::sexp::SExp::#name(sexp)
                    }
                }
            },
    };
    gen.into()
}
