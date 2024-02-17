use crate::nat::{Add, Nat, Suc, Zero};

pub trait SumAll: Nat {
    type Result: Nat;
}

impl SumAll for Zero {
    type Result = Zero;
}

impl<N: Nat> SumAll for Suc<N>
where
    N: SumAll,
    Suc<N>: Add<<N as SumAll>::Result>,
{
    type Result = <Suc<N> as Add<<N as SumAll>::Result>>::Result;
}
