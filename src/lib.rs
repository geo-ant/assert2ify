//! this is just a module reexporting the assert! macro from assert2
pub mod reexports;

pub use assert2ify_macros::assert2ify;


#[cfg(test)]
mod test;
