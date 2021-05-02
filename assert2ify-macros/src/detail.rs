use crate::assertion_macro::StandardLibraryAssertion;

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
        StandardLibraryAssertion::ASSERT_EQ => {"assert_eq"}
        StandardLibraryAssertion::ASSERT_NE => {"assert_ne"}
        StandardLibraryAssertion::ASSERT => {"assert"}
    };

    let segments : Vec<syn::Ident> = path.segments.iter().map(|s|s.ident.clone()).collect();

    (segments.len() == 1 && segments[0] == assertion_name) || (segments.len()==2 && segments[0] == "std" && segments[1] == assertion_name)
}