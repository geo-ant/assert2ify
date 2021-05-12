/// reexport of the `assert!` macro of the assert2 crate to make this crate self-contained
pub use assert2::assert;
/// reexport of the `check!` macro of the assert2 crate to make this crate self-contained
pub use assert2::check;
/// reexport of the `let_assert!` macro of the assert2 crate to make this crate self-contained
pub use assert2::let_assert;

#[macro_export]
#[doc(hidden)]
///TODO DOCUMENT
macro_rules! __xify {
    (new_assertion = $new_assertion:ident, $(::)? $(std::)? matches!($expression:expr, $($pattern:pat)|+ $( if $guard: expr )? $(,)?) $(,$info_args:tt)* $(,)?) => {
        {
            $crate::assertions::let_assert!($($pattern)|+ = $expression $(,$info_args)*);
            $(
                $crate::assertions::$new_assertion!($guard);
            )?
        }
    };
    (new_assertion = $new_assertion:ident, $($args:tt)+) => {
        $crate::assertions::$new_assertion!($($args)+);
    };
}

#[macro_export]
#[doc(hidden)]
///TODO DOCUMENT
macro_rules! __assertify {
    ($($args:tt)+) => {
        $crate::__xify!(new_assertion=assert, $($args)+)
    };

    () => {::std::compile_error!("Too few arguments in assertion")}
}

#[macro_export]
#[doc(hidden)]
///TODO DOCUMENT
macro_rules! __checkify {
    ($($args:tt)+) => {
        $crate::__xify!(new_assertion=check, $($args)+)
    };
    () => {::std::compile_error!("Too few arguments in assertion")}
}
