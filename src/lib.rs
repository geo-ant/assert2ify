//! This crates offers the `#[assert2ify]` attribute, a one-line solution to replace standard
//! library assertions your code with the powerful and more expressive assertions from the
//! [assert2](https://crates.io/crates/assert2) crate.
//! # Motivation
//! The built-in support for testing and assertions in Rust is great, but the standard library
//! assertions are not super descriptive. Let's say we have some assertion on the length
//! of a vector like `assert!(my_vector.len() < 5)`. If this assertion panics, it will just tell
//! you _that_ the condition was violated, but it will not tell you what the actual length of the
//! vector was. To know that, you would have to debug the
//! test or add extra logging output. Here is where the [assert2](https://crates.io/crates/assert2) crate
//! comes to the rescue, because if we had written the assertion as `assert2::assert!(my_vector.len() < 5)`
//! then a possible failure would look like this:
//! ```shell
//! Assertion failed at tests/my_test.rs:107:5:
//!   assert!( my_vector.len() < 5 )
//! with expansion:
//!   7 < 5
//! ```
//! Additionally, the [assert2::assert!](assert2::assert) macro adds nicely colored output. Just
//! stick the `#[assert2ify]` macro into the list of attributes of a test case and take advantage
//! of more expressive assertions. See what more the [assert2ify](assert2ify) attribute can do for you.
//! It also replaces `assert!(matches!(...))` expressions by something that has way better error messages.
//!

#[doc(hidden)]
pub mod assertions;

/// Use this macro to replace the standard library assertions in your test case by sticking this
/// attribute in the list of attributes of your test function (or any other function that has
/// assertions inside it).
///
/// # Usage
/// Stick this attribute above any function in which you want the assertions replaced by the
/// assertions from the assert2 crate. This is primarily useful for functions that are already
/// annotated `#[test]`, but it is possible to use with any function. See below for examples what
/// exactly is replaced and how to use the macro
///
/// ## Arguments
///
/// ### check
/// The attribute can be used as `#[assert2ify]` without arguments, in which case it will replace
/// assertions with assertions. If you want to replace assertions with the `check!` macro from the
/// assert2 crate, use this attribute as `#[assert2ify(check)]`.
///
/// **Caveat**: `assert!(matches!(...))` expressions will still be replaced by assertions because
/// there is no `let_check!` in assert2 as it would not make sense.
///
/// ### crate = ...
/// In case you felt the need to rename this crate in your cargo toml, the compiler will get confused
/// and through an error. You can help the compiler by giving the attribute another argument in the form
/// `crate = new_crate_name`. The arguments can be combined, e.g.
/// `#[assert2ify(check, crate = new_name)]`. However, most of the time the `crate = ...` argument
/// should not be necessary.
///
/// # Which Assertions are Replaced
/// ## Simple Assertions
/// The attribute replaces assertions `assert!`, `assert_eq!`, `assert_ne!` by the corresponding
/// assertions from the assert2 crate. This means that this test
/// ```rust
/// # fn hidden(my_vector:Vec<i32>, my_number : i32, my_other_number : i32) {
/// #[test]
/// #[assert2ify]
/// fn my_test() {
///     /*...*/
///     assert!(my_vector.len()<20);
///     assert_eq!(my_number,1234);
///     assert_ne!(my_other_number, 2);
/// }
/// # }
/// ```
/// now behaves as if you had written it like this:
/// ```rust
/// # fn hidden(my_vector:Vec<i32>, my_number : i32, my_other_number : i32) {
/// #[test]
/// fn my_test() {
///     /*...*/
///     assert2::assert!(my_vector.len()<20);
///     assert2::assert!(my_number == 1234);
///     assert2::assert!(my_other_number != 2);
/// }
/// # }
/// ```
///
/// ## `assert!(matches!(...))` Expressions
/// These statements are used when pattern matching the result of some calculation. Unfortunately
/// they are pretty useless, because it would be nice to see not only _that_ the match failed
/// but what the type of the matched expression actually was. This is where
/// [let_assert](https://docs.rs/assert2/latest/assert2/macro.let_assert.html) of the assert2
/// crate shines. So if we want to test the result of some function `foo` like so
/// ```
/// fn foo(input : i32) -> Result<i32,String> {
///     //[...] some calculation
/// # Err(String::from("negative value!"))
/// }
/// assert!(matches!(foo(-1),Err(s) if s.contains("negative value")));
/// ```
/// then the corresponding assertion will be replaced by
/// ```
/// # fn foo(i : i32) -> Result<i32,String> {Err(String::from("negative value!"))}
/// assert2::let_assert!(Err(s) = foo(-1));
/// assert2::assert!(s.contains("negative value"));
/// ```
/// We have to make this two assertions because as of yet, `let_assert!` does not support additional
/// if statements. This will give you infinitely more helpful messages in case of panics.
///
/// ## Additional Arguments to the Assertions
/// A format string and additional arguments to the assertions are handled as you would expect
/// and will produce additional info output in case of panics.
///
/// ## `assert!(matches!(...))` Expressions
///
/// # Limitations and Caveats
/// The crate traverses the syntax tree given by the contents of the function and replaces the occurrences
/// of standard library assertions by the assertions of the assert2 crate. For most of the uses
/// cases this will be fine, since assertions inside loops, closures, if-statements, etc. will be
/// replaced
///
/// ## Assertions in Nested Code
/// However, there are some edge cases where replacing will not occur. In these cases
/// the standard library assertions will be left untouched and the test will behave as it did previously.
///
/// ### Assertions Inside Assertions
/// Assertions inside assertions are not replaced. Only the outer assertion will be replaced while the
/// other one is left as it was. I personally think that assertions inside assertions indicate
/// a flawed test design and probably won't be working to fix this issue.
///
/// ### Assertions Inside Macros
/// Assertions inside macros invocations (and definitions) are only replaced if the tokens inside
/// the macros can be parsed as an expression. This does not cover all possible cases and I will
/// work to change this in future versions of this crate.
pub use assert2ify_macros::assert2ify;
