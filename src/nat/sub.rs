use crate::nat::{Nat, Suc, Zero};

pub trait Sub<Rhs: Nat>: Nat {
    type Result: Nat;
}

impl Sub<Zero> for Zero {
    type Result = Zero;
}

impl<N: Nat> Sub<Suc<N>> for Zero {
    type Result = Zero;
}

impl<N: Nat> Sub<Zero> for Suc<N> {
    type Result = Suc<N>;
}

impl<Rhs: Nat, Lhs: Nat> Sub<Suc<Rhs>> for Suc<Lhs>
where
    Lhs: Sub<Rhs>,
{
    type Result = <Lhs as Sub<Rhs>>::Result;
}
