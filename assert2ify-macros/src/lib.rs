use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, Token, Ident, PathSegment, PathArguments, Error, Path, ExprMacro, Expr, Pat, Macro, MacroDelimiter, BinOp};
use quote::{quote, quote_spanned};
use syn::fold::{self, Fold};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseBuffer};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::group::Parens;
use syn::token::Token;
use std::iter::FromIterator;


//TODO: https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
//TODO: read this and understand how the syntax tree traversal is implemented and how I can use it

/// holds the configuration of the assert macro
enum Configuration {
    /// means all assertions will be replaced by calls to the
    /// assert macro of the assert2 crate
    ASSERTIFY,
    /// means all assertions will be replace by calls to the check
    /// macro of the assert2 crate
    CHECKIFY,
}

struct Assert2Ification {
    /// whether to replace the assertions with calls to assert! or check! of
    /// the assert2 crate
    replacement_macro_path: syn::Path,
}

impl Assert2Ification {
    pub fn new(configuration : Configuration) -> Result<Assert2Ification,syn::Error> {

        let replacement :Path = syn::parse_str(
            match configuration {
                Configuration::ASSERTIFY => { "::assert2ify::reexports::assert"}
                Configuration::CHECKIFY => { "::assert2ify::reexports::check"}
            }
        )?;

         Ok(Assert2Ification {
             replacement_macro_path: replacement,
         })
    }
}

// todo document
enum AssertionMacro {
    AssertCompare {
        lhs : Expr,
        operator : syn::BinOp,
        rhs : Expr,
    },
    AssertUnary {
        expr : Expr,
    },
    AssertMatches {
        pat : Pat,
        expr : Expr,
    }
}

fn assert2_macro_with(assert2_macro_path:syn::Path, tokens : proc_macro2::TokenStream, span : Span) -> Macro {
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

impl AssertionMacro {
    pub fn assert2ify_with(self, assert2_macro_path : syn::Path, span: Span) -> proc_macro2::TokenStream {
        match self {
            AssertionMacro::AssertCompare { lhs, operator,rhs } => {
                let mac = ExprMacro {
                    attrs: vec![],
                    mac: assert2_macro_with(assert2_macro_path,quote!{#lhs #operator #rhs}.into(),span)
                };

                quote!{#mac}
            }
            AssertionMacro::AssertUnary { .. } => {
                todo!()
            }
            AssertionMacro::AssertMatches { .. } => {
                todo!()
            }
        }
    }
}

// todo document
enum MacroExpression {
    Assertion(AssertionMacro),
    Other(ExprMacro)
}

impl From<ExprMacro> for MacroExpression {
    fn from(expr_macro: ExprMacro) -> Self {

        if expr_macro.mac.path.segments.first().unwrap().ident.to_string().contains("assert_eq") {
            //see https://users.rust-lang.org/t/unable-to-parse-a-tokenstream-into-a-parsebuffer-with-the-syn-crate/44815/2
            let expressions = expr_macro.mac.parse_body_with(Punctuated::<Expr,syn::Token![,]>::parse_terminated).unwrap();

            Self::Assertion(AssertionMacro::AssertCompare {
                lhs : expressions[0].clone(),
                operator : BinOp::Eq(syn::token::EqEq { spans: [expr_macro.span();2] }),
                rhs : expressions[1].clone(),
            })
        } else {
            Self::Other(expr_macro)
        }
    }
}


impl Parse for Assert2Ification {
    fn parse(input: & ParseBuffer) -> Result<Self,syn::parse::Error> {
        let arguments =  Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let configuration = Configuration::ASSERTIFY;
        Assert2Ification::new(configuration)
    }
}

impl Fold for Assert2Ification {
    fn fold_expr_macro(&mut self, expr_macro : ExprMacro) -> ExprMacro {

        println!("macro path = '{:?}'", &expr_macro.mac.path);

        let m_span = expr_macro.span();
        let macro_expression = MacroExpression::from(expr_macro);

        match macro_expression {
            MacroExpression::Assertion(assertion) => {

                let t : proc_macro2::TokenStream = assertion.assert2ify_with(self.replacement_macro_path.clone(),m_span);
                let m : ExprMacro = syn::parse(  quote_spanned!(m_span => #t).into()             ).unwrap();
                m
            }
            MacroExpression::Other(expr_macro) => {
                expr_macro
            }
        }
    }
}

#[proc_macro_attribute]
pub fn assert2ify(args: TokenStream, input: TokenStream) -> TokenStream {
    // See this example by dtolnay on how to traverse a syntax tree and replace nodes
    // https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
    let input = parse_macro_input!(input as ItemFn);

    // Parse the list of variables the user wanted to print.
    let mut assert2ification = parse_macro_input!(args as Assert2Ification);

    // Use a syntax tree traversal to transform the function body.
    // there is other syntax traversal functionality like syn::visit_mut::VisitMut
    // that allows us to traverse nodes and replace them with anything else.
    // Fold just allows us to replace the node with a node of the same type,
    // (i.e. macro with macro), which is fine for my use case
    let output = assert2ification.fold_item_fn(input);

    TokenStream::from(quote!(#output))
}