//! this is just a module reexporting the assert! macro from assert2
pub mod reexports;

pub use assert2_ify_macros::assert2_ify;


#[cfg(test)]
mod test;
