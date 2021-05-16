//! This crates offers the `#[assert2ify]` attribute, a one-line solution to replace standard
//! library assertions your code with the powerful and more expressive assertions from the
//! [assert2](https://crates.io/crates/assert2) crate.
//! # Motivation
//! ## The Simple Case: Assertions on Expressions
//! The built-in support for testing and assertions in Rust is great, but the standard library
//! assertions are not super descriptive. Say for example you have some assertion on the length
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
//! Additionally, the [assert2::assert!](assert2::assert) macro adds nicely colored output.
//!
//! ## Assertions and Pattern Matching
//!
//! # What the #[assert2ify] Attribute Does
//! TODO
//! TODO: stick further doc in the attribute itself
//!
//! # Limitations and Caveats
//! ## Renaming the Crate
//!
//! * Do not rename this (the assert2ify) crate in your Cargo.toml. This will confuse the attribute and give you
//! compile errors, or UB if you did some really naughty things (like importing another crate as assert2ify).
//!
//! ## Replacing Assertions in Nested Code
//! TODO: normally fine, but there are limitations
//! * TODO once an assertion macro is encountered, only the outer assertion is replaced and no
//! potential inner assertions. But I consider this evil anyways, and have zero pity.
//! * TODO: REPLACEMENTS INSIDE NESTED MACROS ARE SHAKY. ONly inside macros taking expressions, so far.
//! ALSO ONLY IN MACRO INVOCATIONS AND NOT MACRO DEFINITIONS
//!
//! TODO: But: assertions will not be removed, so it will just be the standard lib assertions
//! that we had anyways.
pub mod assertions;

pub use assert2ify_macros::assert2ify;
