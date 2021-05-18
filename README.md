# assert2ify
![build](https://github.com/geo-ant/assert2ify/workflows/build/badge.svg?branch=main)
![lints](https://github.com/geo-ant/assert2ify/workflows/lints/badge.svg?branch=main)
![tests](https://github.com/geo-ant/assert2ify/workflows/tests/badge.svg?branch=main)

This crates offers the `#[assert2ify]` attribute, a one-line solution to replace standard
library assertions in your code with the more expressive assertions from the
[assert2](https://crates.io/crates/assert2) crate.

The attribute not only replaces simple assertions
such as `assert!(...)` or `assert_eq!(...)` but also more complicated constructs like `assert!(matches!(...))` with assertions with way
better error messages.

# Motivation
Rust's built-in support for tests and assertions is great in so many ways, but the standard library
assertions are lacking in terms of error messages. This is where the [assert2](https://crates.io/crates/assert2) crate comes in handy
because it offers assertions with very helpful error messages.

Let's say we have some assertion on the length of a vector, such as `assert!(my_vector.len() < 5)`. 
If this assertion panics, it will just tell you _that_ the condition was violated, 
but it will not tell you what the actual length of the
vector was. To know that, you would have to debug the
test or add extra logging output. The assert2 crate
comes to the rescue, because if we had written the assertion as `assert2::assert!(my_vector.len() < 5)`
then a possible failure would look like this (plus some nice colors):
```shell
Assertion failed at tests/my_test.rs:107:5:
  assert!( my_vector.len() < 5 )
with expansion:
  7 < 5
```

You can annotate any function (usually a test case) with `#[assert2ify]` and it
takes care of replacing the assertions inside the functions.

# Usage
Just annotate any function with the `#[assert2ify]` attribute and have the attribute
replace the assertions in your code with the assertions of the assert2 crate.
See what more the `#[assert2ify] attribute can do for you in the documentation.
It does not only replace simple assertions, but also more complicated constructs like
`assert!(matches!(...))` with something much more helpful. The assertions work as they did before,
but they offer way better error messages.

Furthermore, the attribute allows you to replace assertions with checks, which do not fail right away but only once the test case is completely
finished. This can help if you want to catch every assertion that fails in a test in a single pass. Just use
the attribute as `#[assert2ify(check)]` for this.

## Limitations
Have a look at the documentation for limitations on what will be replaced and what won't be.