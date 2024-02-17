use crate::bool::{Bool, False, True};
use crate::nat::{Nat, Suc, Zero};

pub trait IsGreaterThan<Rhs: Nat>: Nat {
    type Result: Bool;
}

impl IsGreaterThan<Zero> for Zero {
    type Result = False;
}

impl<Rhs: Nat> IsGreaterThan<Suc<Rhs>> for Zero {
    type Result = False;
}

impl<Lhs: Nat> IsGreaterThan<Zero> for Suc<Lhs> {
    type Result = True;
}

impl<Lhs: Nat, Rhs: Nat> IsGreaterThan<Suc<Rhs>> for Suc<Lhs>
where
    Lhs: IsGreaterThan<Rhs>,
{
    type Result = <Lhs as IsGreaterThan<Rhs>>::Result;
}
