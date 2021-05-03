use proc_macro2::{Ident, Span};
use syn::{BinOp, Expr, ExprMacro, Macro, MacroDelimiter, Pat, Path, PathArguments, PathSegment, Attribute};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use quote::{quote, quote_spanned};
use std::iter::FromIterator;
use crate::detail::{infer_macro_kind_from_path, is_path_for_std_assertion};
use std::convert::TryFrom;
use std::fmt::Error;


pub struct AssertionMacro {
    /// the span of the original assertion macro
    /// (e.g. of the original assert_eq!)
    pub span: Span,
    /// A vector of arguments that are given as the additional
    /// arguments of the info message of the macro. This vector
    /// can be empty
    pub info_args: Vec<Expr>,
    /// the parsed assertion type. This contains the interesting stuff
    /// of what will be replaced
    pub assertion: Assrt,
    /// the vector of attributes carried over from the macro
    pub attrs: Vec<Attribute>,
}

impl AssertionMacro {
    /// Convenience constructor
    fn new(assrt: Assrt, span: Span, info_args: Vec<Expr>, attrs: Vec<Attribute>) -> Self {
        Self {
            assertion: assrt,
            span,
            info_args,
            attrs,
        }
    }

    //TODO document
    fn assert2ify_with(self, assert2_macro_path: syn::Path) -> ExprMacro {
        let info_args = self.info_args;

        match self.assertion {
            Assrt::AssertBinary { lhs, operator, rhs } => {
                ExprMacro {
                    attrs: self.attrs,
                    mac: assert2_macro_with(assert2_macro_path, quote_spanned! {self.span => #lhs #operator #rhs, #(#info_args),* }.into(), self.span),
                }
            }
            Assrt::AssertMatches { .. } => {
                todo!()
            }
            Assrt::AssertGeneral { expr } => {
                ExprMacro {
                    attrs: self.attrs,
                    mac: assert2_macro_with(assert2_macro_path, quote_spanned! {self.span => #expr, #(#info_args),* }.into(), self.span),
                }
            }
        }
    }
}

/// An intermediate structure which helps parsing assert use cases and variants
/// from the std lib and can translate them into assert2 assertions.
pub enum Assrt {
    /// The binary assertions `std::assert_eq!` and `std::assert_ne`
    /// Those are transalated into the equivalent assertion of the assert2 crate
    AssertBinary {
        lhs: Expr,
        operator: syn::BinOp,
        rhs: Expr,
    },
    /// A macro `std::assert!(matches!(expr,pat))` that can
    /// potentially made into a `assert!(let pat = expr)` of the
    /// assert2 crate.
    /// **Note**: The `assert!(let ...)` syntax of assert2 does not
    /// (yet) support if guards in the statements, while the match! may allow it.
    /// In this case we do not translate into assert2.
    AssertMatches {
        expr: Expr,
        pat: Pat,
    },
    /// Catch all for any other kind of `std::assert!` macro
    /// which is not one of the above.
    /// This means any other kind of assertion on one expression,
    /// notably also assertions on binary expressions like `assert!(a==b)`
    /// or `assert!(v.len() < 5)`.
    AssertGeneral {
        expr: Expr,
    },
}

impl Assrt {
    /// Convenience constructor for binary assertions
    pub fn new_binary(lhs: Expr, operator: syn::BinOp, rhs: Expr) -> Self {
        Self::AssertBinary {
            lhs,
            operator,
            rhs,
        }
    }
    /// Convenience constructor for assertions with match
    pub fn new_assert_matches(expr: Expr, pat: Pat) -> Self {
        Self::AssertMatches { expr, pat }
    }

    /// convenience case for a general assertion case on one argument
    pub fn new_assert(expr: Expr) -> Self {
        Self::AssertGeneral { expr }
    }
}

pub enum MacroExpression {
    Assertion(AssertionMacro),
    Other(ExprMacro),
}


impl MacroExpression {
    /// Convenience constructor for a macro containing an assertion
    pub fn new_assertion(ass: AssertionMacro) -> Self {
        Self::Assertion(ass)
    }

    /// Convenience constructor for a macro containing any other kind
    /// of macro expression
    pub fn new_other(other: ExprMacro) -> Self {
        Self::Other(other)
    }


    //TODO DOCUMENT
    pub fn assert2ify_with(self, assert2_macro_path: syn::Path) -> ExprMacro {
        match self {
            MacroExpression::Assertion(assertion) => {
                assertion.assert2ify_with(assert2_macro_path)
            }
            MacroExpression::Other(expr_macro) => {
                expr_macro
            }
        }
    }
}


impl TryFrom<ExprMacro> for MacroExpression {
    type Error = syn::Error;
    fn try_from(expr_macro: ExprMacro) -> Result<Self, Self::Error> {
        // get the span and parse the macro arguments
        let span = expr_macro.span();
        let mut macro_arguments = expr_macro.mac.parse_body_with(Punctuated::<Expr, syn::Token![,]>::parse_terminated)?.into_iter();
        let create_compile_error = |err_msg| syn::Error::new(span, err_msg);

        let macro_kind = infer_macro_kind_from_path(&expr_macro.mac.path);
        if macro_kind.is_binary_assertion() {
            // binary assertions:
            // the arguments inside assert_eq!(...) or assert_ne!(...)
            // split off the arguments one by one and collect the rest as the message / info arguments
            let lhs = macro_arguments.next().ok_or_else(||create_compile_error("Too few arguments. Binary assertion like assert_eq! or assert_ne! must have at least two arguments"))?;
            let operator = macro_kind.binary_operator(span.clone()).expect("Getting the binary operator for a binary assertion should return Some result");
            let rhs = macro_arguments.next().ok_or_else(||create_compile_error("Too few arguments. Binary assertion like assert_eq! or assert_ne! must have at least two arguments"))?;
            let info_args: Vec<Expr> = macro_arguments.collect();
            Ok(Self::new_assertion(AssertionMacro::new(Assrt::new_binary(lhs, operator, rhs), span, info_args, expr_macro.attrs)))
        } else if macro_kind.is_assertion() {
            // unary assertions:
            println!("TODO: PARSE assert!(matches!(...)) CORRECTLY!");
            let expr = macro_arguments.next().ok_or_else(||create_compile_error("Too few arguments. Assertion must have at least one argument"))?;
            let info_args: Vec<Expr> = macro_arguments.collect();
            Ok(Self::new_assertion(AssertionMacro::new(Assrt::new_assert(expr),span,info_args,expr_macro.attrs)))
        } else {
            // no assertions
            Ok(MacroExpression::Other(expr_macro))
        }
    }
}


/// enumeration that names all the standard assertions that can
/// be replaced with the lib
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StandardLibraryAssertion {
    /// the assertion `assert_eq!`
    AssertEq,
    /// the assertion `assert_ne!`
    AssertNe,
    /// the assertion `assert!`
    Assert,
}

// todo document
pub enum Assertion {
    AssertCompare {
        lhs: Expr,
        operator: syn::BinOp,
        rhs: Expr,
        span: Span,
        msg: Vec<Expr>,
    },
    AssertUnary {
        expr: Expr,
    },
    AssertMatches {
        pat: Pat,
        span: Span,
        expr: Expr,
    },
}

pub fn assert2_macro_with(assert2_macro_path: syn::Path, tokens: proc_macro2::TokenStream, span: Span) -> Macro {
    ///TODO HACKY WAY OF MAKING THE ASSERT2 macro path point to just assert2
    ///THIS JUST RESOLVES TO ASSERT2 without any sexy leading colons or ANYTHING
    /// TODO: OK, the trick is to give the
    let assert2 = PathSegment {
        ident: Ident::new("assert2", span.clone()),
        arguments: PathArguments::None,
    };

    let assert2_segments = Punctuated::<PathSegment, syn::token::Colon2>::from_iter(vec! {assert2});

    let assert2_path = Path {
        leading_colon: None,
        segments: assert2_segments,
    };

    Macro {
        path: assert2_path,
        bang_token: syn::token::Bang { spans: [span.clone(); 1] },
        delimiter: MacroDelimiter::Paren(syn::token::Paren { span: span.clone() }),
        tokens,
    }
}

impl Assertion {
    pub fn assert2ify_with(self, assert2_macro_path: syn::Path) -> ExprMacro {
        match self {
            Assertion::AssertCompare { lhs, operator, rhs, span, msg } => {
                ExprMacro {
                    attrs: vec![],
                    mac: assert2_macro_with(assert2_macro_path, quote! {#lhs #operator #rhs, #(#msg),* }.into(), span),
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
    Other(ExprMacro),
}

impl From<ExprMacro> for MacExpr {
    fn from(expr_macro: ExprMacro) -> Self {
        println!("path = {:#?}", &expr_macro.mac.path.segments);

        if is_path_for_std_assertion(&expr_macro.mac.path, StandardLibraryAssertion::AssertEq) {
            //see https://users.rust-lang.org/t/unable-to-parse-a-tokenstream-into-a-parsebuffer-with-the-syn-crate/44815/2
            //TODO: pass the additional arguments that assert_eq can have as well.
            //TODO: THIS GOES FOR ALL ASSERTIONS
            let expressions = expr_macro.mac.parse_body_with(Punctuated::<Expr, syn::Token![,]>::parse_terminated).unwrap();

            Self::Assertion(Assertion::AssertCompare {
                lhs: expressions[0].clone(),
                operator: BinOp::Eq(syn::token::EqEq { spans: [expr_macro.span(); 2] }),
                rhs: expressions[1].clone(),
                span: expr_macro.span(),
                msg: expressions.iter().skip(2).cloned().collect(),
            })
        } else {
            Self::Other(expr_macro)
        }
    }
}
