use crate::bool::{Bool, False, True};
use crate::nat::{Nat, Suc, Zero};

pub trait IsSmallerThanEqualTo<Rhs: Nat + ?Sized>: Nat {
    type Result: Bool;
}

impl IsSmallerThanEqualTo<Zero> for Zero {
    type Result = True;
}

impl<N: Nat> IsSmallerThanEqualTo<Suc<N>> for Zero {
    type Result = True;
}

impl<N: Nat> IsSmallerThanEqualTo<Zero> for Suc<N> {
    type Result = False;
}

impl<Lhs: Nat, Rhs: Nat> IsSmallerThanEqualTo<Suc<Rhs>> for Suc<Lhs>
where
    Lhs: IsSmallerThanEqualTo<Rhs>,
{
    type Result = <Lhs as IsSmallerThanEqualTo<Rhs>>::Result;
}
