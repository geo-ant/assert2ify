use proc_macro2::{Ident, Span};
use syn::{BinOp, Expr, ExprMacro, Macro, MacroDelimiter, Pat, Path, PathArguments, PathSegment};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use quote::quote;
use std::iter::FromIterator;
use crate::detail::is_path_for_std_assertion;
use std::convert::TryFrom;
use std::fmt::Error;


pub struct AssertionMacro {
    /// the span of the original assertion macro
    /// (e.g. of the original assert_eq!)
    pub span : Span,
    /// A vector of arguments that are given as the additional
    /// arguments of the info message of the macro. This vector
    /// can be empty
    pub info_args: Vec<Expr>,
    /// the parsed assertion type. This contains the interesting stuff
    /// of what will be replaced
    pub assrt: Assrt,
}

impl AssertionMacro {
    /// Convenience constructor
    fn new(assrt : Assrt, span : Span, info_args : Vec<Expr>) -> Self {
        Self {
            assrt,
            span,
            info_args
        }
    }
}

/// An intermediate structure which helps parsing assert use cases and variants
/// from the std lib and can translate them into assert2 assertions.
pub enum Assrt {
    /// The binary assertions `std::assert_eq!` and `std::assert_ne`
    /// Those are transalated into the equivalent assertion of the assert2 crate
    AssertBinary {
        lhs : Expr,
        operator : syn::BinOp,
        rhs : Expr,
    },
    /// A macro `std::assert!(matches!(expr,pat))` that can
    /// potentially made into a `assert!(let pat = expr)` of the
    /// assert2 crate.
    /// **Note**: The `assert!(let ...)` syntax of assert2 does not
    /// (yet) support if guards in the statements, while the match! may allow it.
    /// In this case we do not translate into assert2.
    AssertMatches {
        expr : Expr,
        pat : Pat,
    },
    /// Catch all for any other kind of `std::assert!` macro
    /// which is not one of the above.
    /// This means any other kind of assertion on one expression,
    /// notably also assertions on binary expressions like `assert!(a==b)`
    /// or `assert!(v.len() < 5)`.
    AssertGeneral {
        expr : Expr,
    },
}

impl Assrt {
    /// Convenience constructor for binary assertions
    pub fn new_binary(lhs :Expr, operator : syn::BinOp, rhs : Expr) -> Self {
        Self::AssertBinary {
            lhs,
            operator,
            rhs
        }
    }
    /// Convenience constructor for assertions with match
    pub fn new_assert_matches(expr : Expr, pat : Pat) -> Self {
        Self::AssertMatches {expr,pat}
    }

    /// convenience case for a general assertion case on one argument
    pub fn new_assert(expr:Expr) -> Self {
        Self::AssertGeneral {expr}
    }
}

pub enum MacroExpression {
    Assertion(AssertionMacro),
    Other(ExprMacro),
}


impl MacroExpression {
    /// Convenience constructor for a macro containing an assertion
    pub fn new_assertion(ass : AssertionMacro) -> Self {
        Self::Assertion(ass)
    }

    /// Convenience constructor for a macro containing any other kind
    /// of macro expression
    pub fn new_other(other : ExprMacro) -> Self {
        Self::Other(other)
    }


}


impl TryFrom<ExprMacro> for MacroExpression {
    type Error = syn::Error;
    fn try_from(m: ExprMacro) -> Result<Self,Self::Error> {
        let span = m.span();
        if is_path_for_std_assertion(& m.mac.path, StandardLibraryAssertion::ASSERT_EQ) ||
            is_path_for_std_assertion(& m.mac.path, StandardLibraryAssertion::ASSERT_NE) {
            //TODO MAKE THIS THING MORE MODULAR SO I DON'T HAVE TO DUPLICATE THE CODE FOR
            // OTHER TYPES OF BINARY ASSERTIONS


            // the arguments inside assert_eq!(...)
            let mut macro_arguments = m.mac.parse_body_with(Punctuated::<Expr,syn::Token![,]>::parse_terminated)?.into_iter();
            let create_error_too_few_arguments = || syn::Error::new(span.clone(),"assert_eq! must have at least two arguments.");
            // split off the arguments one by one and collect the rest as the message / info arguments
            let lhs = macro_arguments.next().ok_or_else(create_error_too_few_arguments)?;
            let operator = if is_path_for_std_assertion(& m.mac.path, StandardLibraryAssertion::ASSERT_EQ) {
                BinOp::Eq(syn::token::EqEq { spans: [m.span();2] })
            } else {
                BinOp::Eq(syn::token::Ne { spans: [m.span();2] })
            };
            let rhs = macro_arguments.next().ok_or_else(create_error_too_few_arguments)?;
            let info_args: Vec<Expr> = macro_arguments.collect();

            Ok(Self::new_assertion(AssertionMacro::new(Assrt::new_binary(lhs,operator,rhs), span, info_args)))

        } else {
            todo!()
        }

    }
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

        if is_path_for_std_assertion(& expr_macro.mac.path, StandardLibraryAssertion::ASSERT_EQ ) {
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
