use crate::bool::True;
use crate::cmp::IsGreaterThanEqualTo;
use crate::nat::{Nat, Sub, Suc, Zero};

pub trait Div<Den: Nat>: Nat {
    type Result: Nat;
}

impl<N: Nat> Div<Suc<N>> for Zero {
    type Result = Zero;
}

impl<Den: Nat, Num: Nat> Div<Den> for Num
where
    Num: IsGreaterThanEqualTo<Den, Result = True>,
    Num: Sub<Den>,
    <Num as Sub<Den>>::Result: Div<Den>,
{
    type Result = Suc<<<Num as Sub<Den>>::Result as Div<Den>>::Result>;
}
