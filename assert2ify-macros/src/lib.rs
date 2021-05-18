use proc_macro::TokenStream;

use quote::quote;
use syn::fold::Fold;
use syn::{parse_macro_input, ItemFn};

use crate::detail::{apply_unused_attributes_workaround, check_redefinition_of_assert2ify};
use assert2ification::Assert2Ification;

mod assert2ification;
mod detail;
mod macro_parsing;

#[proc_macro_attribute]
pub fn assert2ify(args: TokenStream, input: TokenStream) -> TokenStream {
    // See this example by dtolnay on how to traverse a syntax tree and replace nodes
    // https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
    let func = parse_macro_input!(input as ItemFn);

    // apply a workaround that will suppress clippy and compiler warnings when
    // should_panic or ignore are encountered in tests. See the doc of the function for more info
    let func = apply_unused_attributes_workaround(func);

    // guard this macro (to some degree) against having this attribute specified twice
    if let Err(error) = check_redefinition_of_assert2ify(&func) {
        return error.into_compile_error().into();
    }

    // Parse the list of variables the user wanted to print.
    let mut assert2ification = parse_macro_input!(args as Assert2Ification);

    // Use a syntax tree traversal to transform the function body.
    // there is other syntax traversal functionality like syn::visit_mut::VisitMut
    // that allows us to traverse nodes and replace them with anything else.
    // Fold just allows us to replace the node with a node of the same type,
    // (i.e. macro with macro), which is fine for my use case
    let output = assert2ification.fold_item_fn(func);

    TokenStream::from(quote!(#output))
}
