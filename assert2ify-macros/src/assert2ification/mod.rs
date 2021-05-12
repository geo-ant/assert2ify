use std::convert::TryFrom;
use std::iter::FromIterator;

use proc_macro2::{Ident, Span};
use syn::{Expr,  fold, Path, PathArguments, PathSegment, Token, Macro};
use syn::fold::Fold;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use quote::ToTokens;
use crate::detail::idents_from_assign_expression;
use crate::macro_parsing::macro_expression::MacroExpression;

/// the crate name of the assert2ify crate and not this macro crate itself
const DEFAULT_ASSERT2IFY_CRATE_NAME: &str = "assert2ify";

/// the style of assertion with which the assertions in the
/// function will be replaced. Either assert or check of assert2.
#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    /// means all assertions will be replaced by calls to the
    /// assert macro of the assert2 crate
    ASSERTIFY,
    /// means all assertions will be replace by calls to the check
    /// macro of the assert2 crate
    CHECKIFY,
}

#[derive(Debug, Clone, PartialEq)]
/// This structure helps us to fold the macros in the syntax tree and replace them
/// by
pub struct Assert2Ification {
    /// whether to replace the assertions with calls to assert! or check! of
    /// the assert2 crate
    configuration: Style,
    /// the name of the assert2ify crate
    /// this will usually be "assert2ify", but the user can tell the macro
    /// that the crate was loaded under a different name
    crate_name: String,

}

impl Assert2Ification {
    /// create a new structure with a given configuration and an optional crate name
    /// # Arguments
    /// * `configuration`: the configuration to apply. This tells us what to replace the assertions with
    /// * `crate_name`: If Some, this is the name of the crate above in this workspace (assert2ify). It
    /// could be that the user imported this crate under another name which is why we give the option
    /// to specify it. If None, we'll just assume that the crate has not been imported under another name
    fn new<S: Into<String>>(configuration: Style, crate_name: Option<S>) -> Assert2Ification {
        Assert2Ification {
            configuration,
            crate_name: crate_name.map(|n|n.into()).unwrap_or_else(||DEFAULT_ASSERT2IFY_CRATE_NAME.to_string()),
        }
    }

    /// A helper function that takes a span (from the macro we want to replace)
    /// and gives us a path to the appropriate replacement macro in the assert2ify crate,
    /// depending on the configuration this is either ::assert2ify::__assertify or
    /// ::assert2ify::__checkify
    fn assert2_macro_path_with_span(&self, span: Span) -> syn::Path {
        let assert2ify = PathSegment {
            ident: Ident::new(self.crate_name.as_str(), span),
            arguments: PathArguments::None,
        };

        let replacement_assertion = match self.configuration {
            Style::ASSERTIFY => {
                PathSegment {
                    ident: Ident::new("__assertify", span),
                    arguments: PathArguments::None,
                }
            }
            Style::CHECKIFY => {
                PathSegment {
                    ident: Ident::new("__checkify", span),
                    arguments: PathArguments::None,
                }
            }
        };

        let assert2_segments = Punctuated::<PathSegment, syn::token::Colon2>::from_iter(vec! {assert2ify, replacement_assertion});

        Path {
            leading_colon: Some(syn::token::Colon2 { spans: [span; 2] }),
            segments: assert2_segments,
        }
    }
}

/// Parse this from the arguments given to the attribute like macro
impl Parse for Assert2Ification {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::parse::Error> {
        let arguments: Vec<Expr> = Punctuated::<Expr, Token![,]>::parse_terminated(input)?.into_iter().collect();

        // this is a somewhat unelegant way of parsing the potential arguments
        // optional argument: crate=crate_name
        let mut crate_name: Option<String> = None;
        // optional argument: check
        // this indicates to use CHECKIFY configuration. Its absence indicates ASSERTIFY
        let mut style: Option<Style> = None;

        for args in arguments.iter() {
            match args {
                Expr::Assign(expr_assign) => {
                    // this can only be crate = crate_name
                    if let Some((lhs,rhs)) = idents_from_assign_expression(&expr_assign) {
                        if lhs == "crate" {
                            if crate_name.is_none() {
                                crate_name = Some(rhs.to_string());
                            } else {
                                return Err(syn::Error::new(expr_assign.span(), "Crate name was already specified"));
                            }
                        } else {
                            return Err(syn::Error::new(expr_assign.span(), "Illegal argument. The only legal assignment is crate=..."));
                        }
                    } else {
                        return Err(syn::Error::new(expr_assign.span(), "Illegal assignment. The only legal assignment is crate=..."));
                    }
                }
                Expr::Path(expr_path) => {
                    if expr_path.path.is_ident("check") {
                        if style.is_none() {
                            style = Some(Style::CHECKIFY);
                        } else {
                            return Err(syn::Error::new(expr_path.span(), "Illegal argument. Assertification style was already specified"));
                        }
                    } else {
                        return Err(syn::Error::new(expr_path.span(), "Illegal argument. Did you mean `check`?"));
                    }
                }
                _ => { return Err(syn::Error::new(args.span(), "Invalid argument")); }
            }
        }
        Ok(Assert2Ification::new(style.unwrap_or(Style::ASSERTIFY), crate_name))
    }
}

impl Fold for Assert2Ification {
    /// Fold the syntax tree and replace standard library assert macros by the assertion macros from the super crate
    /// # Result
    /// The replaced or untouched macros.
    /// # A Note On Nested Macros
    /// If an assertion macro is encountered, then this assertion is parsed ond no further folding
    /// on the expression(s) inside the assertion are performed. That means nested asserts won't get
    /// replaced. If you have them, the chances are high that your code is unreadable anyways.
    ///
    /// If the macro is not an assertion, this function looks into the tokens and tries to fold them.
    /// However, the folding is only performed if the tokens in the macro can be parsed as an expression
    /// Then the parser is recursively invoked. This will parse nested macros **to a degree**
    /// Expr covers a lot of things, but it will
    /// not cover all possible paths, e.g. multiple statements separated by
    /// a semicolon (unless they are themselves enclosed in a block). So for assert expressions
    /// that are themselves enclosed in a macro it is not guaranteed that they will be assertifyed
    /// or checkifyed.
    fn fold_macro(&mut self, mac: Macro) -> Macro {
        let macro_parse_result = MacroExpression::try_from(mac.clone());
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
                MacroExpression::Other(other_macro) => {
                    // this looks into the tokens in the macro and if they are an expression,
                    // the parser is recursively invoked. This will parse nested macros.
                    // I don't think we need to recursively pass anything other than expressions
                    // in macros but I might be wrong. Expr covers a lot of things, but it will
                    // not cover all possible paths, for example multiple statements separated by
                    // a semicolon (unless they are themselves enclosed in a block)
                    if let Ok(nested_expr) = syn::parse2::<Expr>(other_macro.tokens.clone()) {
                        let folded = self.fold_expr(nested_expr);
                        Macro {
                            tokens : folded.to_token_stream(),
                            ..other_macro
                        }
                    } else {
                        fold::fold_macro(self, other_macro)
                    }
                }
            }
        } else {
            // an error has occurred during parsing. This indicates a syntax error
            // we'll just return the original macro and let the compiler moan at the user
            mac
        }
    }
}
