use crate::bool::{Bool, False, True};
use crate::nat::{Nat, Suc, Zero};

pub trait IsGreaterThanEqualTo<Rhs: Nat>: Nat {
    type Result: Bool;
}

impl IsGreaterThanEqualTo<Zero> for Zero {
    type Result = True;
}

impl<Rhs: Nat> IsGreaterThanEqualTo<Suc<Rhs>> for Zero {
    type Result = False;
}

impl<Lhs: Nat> IsGreaterThanEqualTo<Zero> for Suc<Lhs> {
    type Result = True;
}

impl<Lhs: Nat, Rhs: Nat> IsGreaterThanEqualTo<Suc<Rhs>> for Suc<Lhs>
where
    Lhs: IsGreaterThanEqualTo<Rhs>,
{
    type Result = <Lhs as IsGreaterThanEqualTo<Rhs>>::Result;
}
