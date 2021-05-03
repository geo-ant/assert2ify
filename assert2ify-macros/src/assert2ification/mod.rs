use proc_macro2::{Ident, Span};
use syn::{ExprMacro, fold, Path, PathArguments, PathSegment, Token};
use syn::fold::Fold;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;


use crate::assertion_macro::MacExpr;
use std::iter::FromIterator;

/// holds the configuration of the assert macro
#[derive(Debug, Clone, PartialEq)]
pub enum Configuration {
    /// means all assertions will be replaced by calls to the
    /// assert macro of the assert2 crate
    ASSERTIFY,
    /// means all assertions will be replace by calls to the check
    /// macro of the assert2 crate
    CHECKIFY,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assert2Ification {
    /// whether to replace the assertions with calls to assert! or check! of
    /// the assert2 crate
    configuration: Configuration,
}

impl Assert2Ification {
    pub fn new(configuration: Configuration) -> Result<Assert2Ification, syn::Error> {
        Ok(Assert2Ification {
            configuration,
        })
    }

    pub fn assert2_macro_path_with_span(&self, span: Span) -> syn::Path {
        match self.configuration {
            Configuration::ASSERTIFY => {
                //TODO clean this up and make it use the reexported path
                let assert2 = PathSegment {
                    ident: Ident::new("assert2", span.clone()),
                    arguments: PathArguments::None,
                };

                let assert2_segments = Punctuated::<PathSegment, syn::token::Colon2>::from_iter(vec! {assert2});

                Path {
                    leading_colon: None,
                    segments: assert2_segments,
                }
            }

            Configuration::CHECKIFY => { todo!() }
        }
    }
}


impl Parse for Assert2Ification {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::parse::Error> {
        let arguments: Vec<Ident> = Punctuated::<Ident, Token![,]>::parse_terminated(input)?.into_iter().collect();

        if arguments.is_empty() {
            Ok(Assert2Ification::new(Configuration::ASSERTIFY)?)
        } else if arguments.len() == 1 {
            if arguments[0] == "check" {
                Ok(Assert2Ification::new(Configuration::CHECKIFY)?)
            } else {
                Err(syn::Error::new(input.span(), "Invalid macro argument: either use no arguments or use `check` as a macro argument"))
            }
        } else {
            Err(syn::Error::new(input.span(), "Too many macro arguments: either use no arguments or use `check` as a macro argument"))
        }
    }
}

impl Fold for Assert2Ification {
    fn fold_expr_macro(&mut self, expr_macro: ExprMacro) -> ExprMacro {
        println!("macro path = '{:?}'", &expr_macro.mac.path);

        let m_span = expr_macro.span();
        let macro_expression = MacExpr::from(expr_macro);

        match macro_expression {
            MacExpr::Assertion(assertion) => {
                //todo! get the replacement path for the span
                //todo: then we don't need the extra span argument anymore and can just use the
                //replacement path span or the token span that we have anyways
                assertion.assert2ify_with(self.assert2_macro_path_with_span(m_span.clone()).clone())
            }
            MacExpr::Other(expr_macro) => {
                // see https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
                // I think we do it like this, to keep on visiting the nodes recursively even if we
                // are inside another macro invocation
                fold::fold_expr_macro(self, expr_macro)
            }
        }
    }
}
