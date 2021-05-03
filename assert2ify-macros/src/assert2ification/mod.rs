use proc_macro2::{Ident, Span};
use syn::{ExprMacro, fold, Path, PathArguments, PathSegment, Token};
use syn::fold::Fold;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use std::convert::TryFrom;


use crate::assertion_macro::{MacroExpression};
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
        let assert2ify = PathSegment {
            ident: Ident::new("assert2ify", span),
            arguments: PathArguments::None,
        };

        let reexports = PathSegment {
            ident: Ident::new("reexports", span),
            arguments: PathArguments::None,
        };


        match self.configuration {
            Configuration::ASSERTIFY => {
                let assert = PathSegment {
                    ident: Ident::new("assert", span),
                    arguments: PathArguments::None,
                };

                let assert2_segments = Punctuated::<PathSegment, syn::token::Colon2>::from_iter(vec! {assert2ify, reexports, assert});

                Path {
                    leading_colon: Some(syn::token::Colon2 { spans: [span; 2] }),
                    segments: assert2_segments,
                }
            }

            Configuration::CHECKIFY => {
                let check = PathSegment {
                    ident: Ident::new("check", span),
                    arguments: PathArguments::None,
                };

                let assert2_segments = Punctuated::<PathSegment, syn::token::Colon2>::from_iter(vec! {assert2ify, reexports, check});

                Path {
                    leading_colon: Some(syn::token::Colon2 { spans: [span; 2] }),
                    segments: assert2_segments,
                }
            }
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

        let macro_parse_result = MacroExpression::try_from(expr_macro.clone());
        // we check whether the macro could be parsed. If not, this indicates a syntax error in the
        // original code like an assert! with no arguments or an assert_eq! with just one
        // in this case we return the original macro and let the compiler give an error so the user
        // can fix it
        if let Ok(macro_expression) = macro_parse_result {
            let span = macro_expression.span();

            match macro_expression {
                MacroExpression::Assertion(assertion) => {
                    assertion.assert2ify_with(self.assert2_macro_path_with_span(span))
                }
                MacroExpression::Other(expr_macro) => {
                    // see https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
                    // I think we do it like this, to keep on visiting the nodes recursively even if we
                    // are inside another macro invocation
                    fold::fold_expr_macro(self, expr_macro)
                }
            }
        } else {
            expr_macro
        }
    }
}
