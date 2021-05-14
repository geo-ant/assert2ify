use syn::Expr;

/// An intermediate structure which helps parsing assert use cases and variants
/// from the std lib and can translate them into assert2 assertions.
pub enum Assertion {
    /// The binary assertions `std::assert_eq!` and `std::assert_ne`
    /// Those are transalated into the equivalent assertion of the assert2 crate
    AssertBinary {
        lhs: Expr,
        operator: syn::BinOp,
        rhs: Expr,
    },
    /// Catch all for any other kind of `std::assert!` macro
    /// which is not one of the above.
    /// This means any other kind of assertion on one expression,
    /// notably also assertions on binary expressions like `assert!(a==b)`
    /// or `assert!(v.len() < 5)`.
    AssertUnary { expr: Expr },
}

impl Assertion {
    /// Convenience constructor for binary assertions
    pub fn new_binary(lhs: Expr, operator: syn::BinOp, rhs: Expr) -> Self {
        Self::AssertBinary { lhs, operator, rhs }
    }
    /// convenience case for a general assertion case on one argument
    pub fn new_assert(expr: Expr) -> Self {
        Self::AssertUnary { expr }
    }
}
