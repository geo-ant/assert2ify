//! this is just a module reexporting the assert! macro from assert2
pub mod assertions;

pub use assert2ify_macros::assert2ify;

#[macro_export]
macro_rules! make_let_assert {
    ($(::)? $(std::)? matches!($expression:expr, $($pattern:pat)|+ $( if $guard: expr )? $(,)?)) => {
        // todo: make let assertion configurable also
        $crate::assertions::let_assert!($($pattern)|+ = $expression);
        $(
            // todo make this configurable
            $crate::assertions::assert!($guard);
        )?
    }
}

fn something() -> Result<i32,String> {
    Ok(1337)
}

fn error() -> Result<i32,String> {
    Err("bla".to_string())
}


#[test]
fn test_syntax() {
    make_let_assert!(std::matches!(error(), Err(s) if s.contains("xxx")));
}

#[cfg(test)]
mod test;
