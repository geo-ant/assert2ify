use proc_macro2::{Ident, Span};
use syn::{BinOp, Expr, ExprMacro, Macro, MacroDelimiter, Pat, Path, PathArguments, PathSegment};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use quote::quote;
use std::iter::FromIterator;
use crate::detail::is_path_for_std_assertion;


pub struct AssertionMacro {
    /// the span of the original assertion macro
    /// (e.g. of the original assert_eq!)
    pub span : Span,
    /// A vector of arguments that are given as the additional
    /// arguments of the info message of the macro. This vector
    /// can be empty
    pub message_args : Vec<Expr>,
    /// the parsed assertion type. This contains the interesting stuff
    /// of what will be replaced
    pub mac : Assertion,
}

/// enumeration that names all the standard assertions that can
/// be replaced with the lib
pub enum StandardLibraryAssertion {
    /// the assertion `assert_eq!`
    ASSERT_EQ,
    /// the assertion `assert_ne!`
    ASSERT_NE,
    /// the assertion `assert!`
    ASSERT,
}

// todo document
pub enum Assertion {
    AssertCompare {
        lhs : Expr,
        operator : syn::BinOp,
        rhs : Expr,
        span :  Span,
        msg : Vec<Expr>
    },
    AssertUnary {
        expr : Expr,
        //TODO: optional field for additional tokens / message. CAUTION: HOW TO I parse those? Can I just parse them as expressions?
        span : Span,
    },
    AssertMatches {
        pat : Pat,
        span : Span,
        expr : Expr,
        //TODO: optional field for additional tokens / message. CAUTION: HOW TO I parse those? Can I just parse them as expressions?
    }
}

pub fn assert2_macro_with(assert2_macro_path:syn::Path, tokens : proc_macro2::TokenStream, span : Span) -> Macro {
    ///TODO HACKY WAY OF MAKING THE ASSERT2 macro path point to just assert2
    ///THIS JUST RESOLVES TO ASSERT2 without any sexy leading colons or ANYTHING
    /// TODO: OK, the trick is to give the
    let assert2 = PathSegment {
        ident: Ident::new("assert2", span.clone()),
        arguments: PathArguments::None
    };

    let assert2_segments = Punctuated::<PathSegment,syn::token::Colon2>::from_iter(vec!{assert2});

    let assert2_path = Path {
        leading_colon : None,
        segments : assert2_segments,
    };

    Macro {
        path: assert2_path,
        bang_token: syn::token::Bang {spans : [span.clone();1]},
        delimiter: MacroDelimiter::Paren(syn::token::Paren{span : span.clone()}),
        tokens
    }
}

impl Assertion {
    pub fn assert2ify_with(self, assert2_macro_path : syn::Path) -> ExprMacro {
        match self {
            Assertion::AssertCompare { lhs, operator,rhs, span, msg } => {
                ExprMacro {
                    attrs: vec![],
                    mac: assert2_macro_with(assert2_macro_path,quote!{#lhs #operator #rhs, #(#msg),* }.into(),span)
                }
            }
            Assertion::AssertUnary { .. } => {
                todo!()
            }
            Assertion::AssertMatches { .. } => {
                todo!()
            }
        }
    }
}

// todo document
pub enum MacExpr {
    Assertion(Assertion),
    Other(ExprMacro)
}

impl From<ExprMacro> for MacExpr {
    fn from(expr_macro: ExprMacro) -> Self {

        println!("path = {:#?}", &expr_macro.mac.path.segments );

        if is_path_for_std_assertion(& expr_macro.mac.path, StandardLibraryAssertion::ASSERT_EQ, ) {
            //see https://users.rust-lang.org/t/unable-to-parse-a-tokenstream-into-a-parsebuffer-with-the-syn-crate/44815/2
            //TODO: pass the additional arguments that assert_eq can have as well.
            //TODO: THIS GOES FOR ALL ASSERTIONS
            let expressions = expr_macro.mac.parse_body_with(Punctuated::<Expr,syn::Token![,]>::parse_terminated).unwrap();

            Self::Assertion(Assertion::AssertCompare {
                lhs : expressions[0].clone(),
                operator : BinOp::Eq(syn::token::EqEq { spans: [expr_macro.span();2] }),
                rhs : expressions[1].clone(),
                span : expr_macro.span(),
                msg : expressions.iter().skip(2).cloned().collect()
            })
        } else {
            Self::Other(expr_macro)
        }
    }
}
