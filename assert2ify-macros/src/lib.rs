use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, Token, Ident, PathSegment, PathArguments, Error, Path};
use quote::quote;
use syn::fold::{self, Fold};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseBuffer};
use proc_macro2::Span;

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
                Configuration::ASSERTIFY => { "::assert2_ify::reexports::assert"}
                Configuration::CHECKIFY => { "::assert2_ify::reexports::check"}
            }
        )?;

         Ok(Assert2Ification {
             replacement_macro_path: replacement,
         })
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

}

#[proc_macro_attribute]
pub fn assert2_ify(args: TokenStream, input: TokenStream) -> TokenStream {
    // See this example by dtolnay on how to traverse a syntax tree and replace nodes
    // https://github.com/dtolnay/syn/blob/master/examples/trace-var/trace-var/src/lib.rs
    let input = parse_macro_input!(input as ItemFn);

    // Parse the list of variables the user wanted to print.
    let mut assert2ification = parse_macro_input!(args as Assert2Ification);

    // Use a syntax tree traversal to transform the function body.
    let output = assert2ification.fold_item_fn(input);

    TokenStream::from(quote!(#output))
}