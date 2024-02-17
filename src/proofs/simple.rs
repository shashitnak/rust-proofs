use crate::bool::{And, Bool, False, Implies, Or, True};
use crate::cmp::IsEqualTo;
use crate::nat::{Add, Div, Five, Four, Mul, Nat, One, Suc, SumAll, Three, Two};
use std::marker::PhantomData;

fn prove(a: Lhs) -> Rhs {
    a
}

type Lhs = <Three as SumAll>::Result;
type Rhs = <<Three as Mul<Four>>::Result as Div<Two>>::Result;
