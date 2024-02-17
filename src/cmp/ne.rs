use crate::bool::{Bool, False, True};
use crate::nat::{Nat, Suc, Zero};

pub trait IsNotEqualTo<Rhs: Nat>: Nat {
    type Result: Bool;
}

impl IsNotEqualTo<Zero> for Zero {
    type Result = False;
}

impl<Rhs: Nat> IsNotEqualTo<Suc<Rhs>> for Zero {
    type Result = False;
}

impl<Lhs: Nat> IsNotEqualTo<Zero> for Suc<Lhs> {
    type Result = False;
}

impl<Lhs: Nat, Rhs: Nat> IsNotEqualTo<Suc<Rhs>> for Suc<Lhs>
where
    Lhs: IsNotEqualTo<Rhs>,
{
    type Result = <Lhs as IsNotEqualTo<Rhs>>::Result;
}
