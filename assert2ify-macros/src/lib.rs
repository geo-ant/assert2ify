use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{BinOp, Error, Expr, ExprMacro, Ident, ItemFn, Macro, MacroDelimiter, parse_macro_input, Pat, Path, PathArguments, PathSegment, Token};
use syn::fold::{self, Fold};
use syn::group::Parens;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Token;

use assert2ification::Assert2Ification;

mod assert2ification;
mod assertion_macro;

//TODO: https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
//TODO: read this and understand how the syntax tree traversal is implemented and how I can use it

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