use proc_macro2::Span;
use syn::{Expr, Macro, MacroDelimiter};

use crate::macro_parsing::assertion::Assertion;
use quote::quote_spanned;

/// a structure describing an assertion macro
pub struct AssertionMacro {
    /// the span of the original assertion macro
    /// (e.g. of the original assert_eq!)
    pub span: Span,
    /// A vector of arguments that are given as the additional
    /// arguments of the info message of the macro. This vector
    /// can be empty. Those are the extra comments (format string and
    /// arguments) passed to the assertion macro.
    pub info_args: Vec<Expr>,
    /// the actual assertion. This contains the interesting stuff
    /// of what will be replaced
    pub assertion: Assertion,
}

impl AssertionMacro {
    /// Convenience constructor
    pub fn new(assrt: Assertion, span: Span, info_args: Vec<Expr>) -> Self {
        Self {
            assertion: assrt,
            span,
            info_args,
        }
    }

    /// replace the macro invocation by the appropriate __assertify! or __checkify! invocations
    /// of the supercrate
    /// # Arguments
    /// * `assert2_macro_path`: the full path to the assertify or checkify macros
    pub fn assert2ify_with(self, assert2_macro_path: syn::Path) -> Macro {
        let info_args = self.info_args;

        let tokens  = match self.assertion {
            Assertion::AssertBinary { lhs, operator, rhs } => {
                 quote_spanned! {self.span => #lhs #operator #rhs, #(#info_args),* }
            }
            Assertion::AssertUnary { expr } => {
                quote_spanned! {self.span => #expr, #(#info_args),* }
            }
        };

        Macro {
            path: assert2_macro_path,
            bang_token: syn::token::Bang { spans: [self.span; 1] },
            delimiter: MacroDelimiter::Paren(syn::token::Paren { span:self.span }),
            tokens,
        }
    }
}

