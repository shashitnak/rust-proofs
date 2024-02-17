use crate::nat::{Nat, Suc, Zero};

pub trait Add<Rhs: Nat>: Nat {
    type Result: Nat;
}

impl Add<Zero> for Zero {
    type Result = Zero;
}

impl<Rhs: Nat> Add<Suc<Rhs>> for Zero {
    type Result = Suc<Rhs>;
}

impl<Lhs: Nat> Add<Zero> for Suc<Lhs> {
    type Result = Suc<Lhs>;
}

impl<Rhs: Nat, Lhs: Nat + Add<Rhs>> Add<Suc<Rhs>> for Suc<Lhs> {
    type Result = Suc<Suc<<Lhs as Add<Rhs>>::Result>>;
}
