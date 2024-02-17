use std::marker::PhantomData;

pub trait Nat {}

pub struct Zero;
impl Nat for Zero {}

pub struct Suc<N: Nat>(PhantomData<N>);
impl<N: Nat> Nat for Suc<N> {}

pub type One = Suc<Zero>;
pub type Two = Suc<One>;
pub type Three = Suc<Two>;
pub type Four = Suc<Three>;
pub type Five = Suc<Four>;
pub type Six = Suc<Five>;
pub type Seven = Suc<Six>;
pub type Eight = Suc<Seven>;
pub type Nine = Suc<Eight>;
pub type Ten = Suc<Nine>;
