use crate::nat::{Add, Nat, Suc, Zero};

pub trait Mul<Rhs: Nat>: Nat {
    type Result: Nat;
}

impl<Lhs: Nat> Mul<Zero> for Lhs {
    type Result = Zero;
}

impl<Rhs: Nat, Lhs: Nat> Mul<Suc<Rhs>> for Lhs
where
    Lhs: Mul<Rhs>,
    Lhs: Add<<Lhs as Mul<Rhs>>::Result>,
{
    type Result = <Lhs as Add<<Lhs as Mul<Rhs>>::Result>>::Result;
}
