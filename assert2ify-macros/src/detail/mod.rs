use crate::assertion_macro::StandardLibraryAssertion;
use proc_macro2::Span;
use syn::{BinOp, Path};

/// This helper function will return true iff the given path points to a standard assertion macro.
/// It will return true for a paths like `::std::assert`, for `std::assert`, or just `assert` and
/// similar for the other macros, if the given assertion enumeration matches.
/// # Arguments
/// * `path` the path in question
/// * `assertion`: the type of assertion to check against
/// # Return
/// true iff the macro points to this assertion macro
/// # Caveat
/// If `assert!` and the assertion macros in scope do not point to the standard library asserts,
/// then we have to way to check that. They will be classified as std asserts as well.
/// If the std library was used as something else, then there is also no way to check that...
pub fn is_path_for_std_assertion(path : &syn::Path, assertion : StandardLibraryAssertion) -> bool {
    let assertion_name = match assertion{
        StandardLibraryAssertion::AssertEq => {"assert_eq"}
        StandardLibraryAssertion::AssertNe => {"assert_ne"}
        StandardLibraryAssertion::Assert => {"assert"}
    };

    let segments : Vec<syn::Ident> = path.segments.iter().map(|s|s.ident.clone()).collect();

    (segments.len() == 1 && segments[0] == assertion_name) || (segments.len()==2 && segments[0] == "std" && segments[1] == assertion_name)
}

pub enum MacroKind {
    Assertion(StandardLibraryAssertion),
    /// any other kind of macro
    Other,
}

impl From<StandardLibraryAssertion> for MacroKind {
    fn from(ass: StandardLibraryAssertion) -> Self {
        Self::Assertion(ass)
    }
}

impl MacroKind {
    /// helper function that indicates whether the class of macro
    /// is an assertion macro of any kind
    pub fn is_assertion(&self) -> bool {
        match self {
            Self::Assertion(_) => {true}
            Self::Other => {false}
        }
    }

    /// helper function that indicates whether the class of macro
    /// is a binary assertion (`assert_eq!` or `assert_ne!`)
    pub fn is_binary_assertion(&self) -> bool {
        match self {
            Self::Assertion(StandardLibraryAssertion::AssertEq) => {true}
            Self::Assertion(StandardLibraryAssertion::AssertNe) => {true}
            Self::Assertion(StandardLibraryAssertion::Assert) => {false}
            Self::Other => {false}
        }
    }

    /// Helper function to get the binary comparison operator of the macro,
    /// if such an operator exists
    /// # Arguments
    /// * `span` the span to be assigned to the binary operator
    /// # Returns
    /// If the macro kind is a binary assertion, then this returns
    /// the binary operator used to compare left and right argument.
    /// Otherwise returns None.
    pub fn binary_operator(&self, span : Span) -> Option<BinOp> {
        match self {
            Self::Assertion(StandardLibraryAssertion::AssertEq) => { Some(BinOp::Eq(syn::token::EqEq { spans: [span;2] }))}
            Self::Assertion(StandardLibraryAssertion::AssertNe) => {Some(BinOp::Ne(syn::token::Ne { spans: [span;2] }))}
            Self::Assertion(StandardLibraryAssertion::Assert) => { None}
            Self::Other => { None}
        }
    }
}

/// Using the path from the macro infer whether it is `assert_eq!`, `assert_ne!`, `assert!` or
/// some entirely different macro.
/// # Arguments
/// * `path` the path in question. If the path begins with ::std or std, the next segment of the
/// path is checked whether it is one of the assertions in question.
/// # Return
/// The kind of assertion
/// # Caveat
/// If `assert!` and the assertion macros in scope do not point to the standard library asserts,
/// then we have to way to check that. They will be classified as std asserts as well.
/// If the std library was used as something else, then there is also no way to check that...
pub fn infer_macro_kind_from_path(path : &syn::Path) -> MacroKind {

    let segments : Vec<syn::Ident> = path.segments.iter().map(|s|s.ident.clone()).collect();

    // helper function
    fn macro_kind(ident : &syn::Ident) -> MacroKind {
        let assert_eq = "assert_eq";
        let assert_ne = "assert_ne";
        let assert = "assert";

        if ident == assert_eq {
            MacroKind::from(StandardLibraryAssertion::AssertEq)
        } else if ident == assert_ne {
            MacroKind::from(StandardLibraryAssertion::AssertNe)
        } else if ident == assert {
            MacroKind::from(StandardLibraryAssertion::Assert)
        } else {
            MacroKind::Other
        }
    }

    if segments.len() == 1 {
        macro_kind(&segments[0])
    } else if segments.len() == 2 {
        if segments[0] != "std" {
            MacroKind::Other
        } else {
            macro_kind(&segments[1])
        }
    } else {
        MacroKind::Other
    }
}