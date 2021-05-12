use proc_macro::TokenStream;

use quote::{quote};
use syn::{ ItemFn, parse_macro_input};
use syn::fold::{Fold};

use assert2ification::Assert2Ification;
use syn::spanned::Spanned;

mod assert2ification;
mod macro_parsing;
mod detail;

//TODO: https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
//TODO: read this and understand how the syntax tree traversal is implemented and how I can use it

#[proc_macro_attribute]
pub fn assert2ify(args: TokenStream, input: TokenStream) -> TokenStream {
    // See this example by dtolnay on how to traverse a syntax tree and replace nodes
    // https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
    let input = parse_macro_input!(input as ItemFn);

    // guard this agains redefinition of this attribute
    // if so, report an error.
    // Remark: this attribute itself will not be visible in the list of attributes of the function,
    // which is why this works
    // this will not work 100% reliably because someone might use this thing under a different name,
    // but it does guard against accidental duplication
    if let Some(other_assertify_macro) = input.attrs.iter().find(|attr|attr.path.segments.last().map(|s|s.ident=="assert2ify").unwrap_or(false)) {
        return syn::Error::new(other_assertify_macro.span(),"Duplicate attribute. This attribute must only be specified once for each function").into_compile_error().into();
    }

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