use crate::bool::{Bool, False, True};
use crate::nat::{Nat, Suc, Zero};

pub trait IsSmallerThan<Rhs: Nat + ?Sized>: Nat {
    type Result: Bool;
}

impl IsSmallerThan<Zero> for Zero {
    type Result = False;
}

impl<N: Nat> IsSmallerThan<Suc<N>> for Zero {
    type Result = True;
}

impl<N: Nat> IsSmallerThan<Zero> for Suc<N> {
    type Result = False;
}

impl<Lhs: Nat, Rhs: Nat> IsSmallerThan<Suc<Rhs>> for Suc<Lhs>
where
    Lhs: IsSmallerThan<Rhs>,
{
    type Result = <Lhs as IsSmallerThan<Rhs>>::Result;
}
