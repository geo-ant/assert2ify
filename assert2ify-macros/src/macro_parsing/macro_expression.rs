use std::convert::TryFrom;

use proc_macro2::Span;
use syn::{Expr, Macro};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::detail::infer_macro_kind_from_path;
use crate::macro_parsing::assertion::Assertion;
use crate::macro_parsing::assertion_macro::AssertionMacro;


/// an enumeration that can capture any kind of syn::ExprMacro type
/// this can be an assertion macro or any other kind of macro
pub enum MacroExpression {
    /// an assertion macro that we can parse in this crate
    Assertion(AssertionMacro),
    /// any other kind of macro not parsed by this crate
    Other(Macro),
}

impl Spanned for MacroExpression {
    fn span(&self) -> Span {
        match self {
            MacroExpression::Assertion(ass) => {ass.span}
            MacroExpression::Other(mac) => {mac.span()}
        }
    }
}

impl MacroExpression {
    /// Convenience constructor for a macro containing an assertion
    pub fn new_assertion(ass: AssertionMacro) -> Self {
        Self::Assertion(ass)
    }

    /// Convenience constructor for a macro containing any other kind
    /// of macro expression
    pub fn new_other(other: Macro) -> Self {
        Self::Other(other)
    }
}

impl TryFrom<Macro> for MacroExpression {
    type Error = syn::Error;

    /// try to generate the structure from an ExprMacro.
    /// # Arguments
    /// * `expr_macro`: the macro expression we want to parse
    /// # Returns
    /// The parsed macro expression which contains either a parsed assertion or any other kind
    /// of macro.
    /// Parsing the assertions might fail if the assertions are used incorrectly, e.g.
    /// if assert_eq! is used with just one argument. In this case we report an error.
    fn try_from(mac: Macro) -> Result<Self, Self::Error> {
        // get the span and parse the macro arguments
        let span = mac.span();
        let mut macro_arguments = mac.parse_body_with(Punctuated::<Expr, syn::Token![,]>::parse_terminated)?.into_iter();

        let create_compile_error = |err_msg| {
            syn::Error::new(span, err_msg)
        };

        let macro_kind = infer_macro_kind_from_path(&mac.path);
        if macro_kind.is_binary_assertion() {
            // binary assertions:
            // the arguments inside assert_eq!(...) or assert_ne!(...)
            // split off the arguments one by one and collect the rest as the message / info arguments
            let lhs = macro_arguments.next().ok_or_else(||create_compile_error("Too few arguments: expected 2 or more, got 0"))?;
            let operator = macro_kind.binary_operator(span).expect("Getting the binary operator for a binary assertion should return Some result");
            let rhs = macro_arguments.next().ok_or_else(||create_compile_error("Too few arguments: expected 2 or more, got 1"))?;
            let info_args: Vec<Expr> = macro_arguments.collect();
            Ok(Self::new_assertion(AssertionMacro::new(Assertion::new_binary(lhs, operator, rhs), span, info_args)))
        } else if macro_kind.is_assertion() {
            // all kinds of unary assertions:
            // Remark: assert!(matches!(...)) asertions are handled by the __assertify!
            // and __checkify! macros in the crate one level above
            // in the context of this crate, they are just another unary assertion
            let expr = macro_arguments.next().ok_or_else(||create_compile_error("Too few arguments: expected 1 or more, got 0"))?;
            let info_args: Vec<Expr> = macro_arguments.collect();
            Ok(Self::new_assertion(AssertionMacro::new(Assertion::new_assert(expr), span, info_args)))
        } else {
            // no assertions: just pass them though unaltered
            Ok(MacroExpression::new_other(mac))
        }
    }
}