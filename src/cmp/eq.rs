use crate::bool::{Bool, False, True};
use crate::nat::{Nat, Suc, Zero};

pub trait IsEqualTo<Rhs> {
    type Result: Bool;
}

impl IsEqualTo<False> for False {
    type Result = True;
}

impl IsEqualTo<True> for False {
    type Result = False;
}

impl IsEqualTo<False> for True {
    type Result = False;
}

impl IsEqualTo<True> for True {
    type Result = True;
}

impl IsEqualTo<Zero> for Zero {
    type Result = True;
}

impl<N: Nat> IsEqualTo<Suc<N>> for Zero {
    type Result = False;
}

impl<N: Nat> IsEqualTo<Zero> for Suc<N> {
    type Result = False;
}

impl<N: Nat, M: Nat + IsEqualTo<N>> IsEqualTo<Suc<N>> for Suc<M> {
    type Result = <M as IsEqualTo<N>>::Result;
}
