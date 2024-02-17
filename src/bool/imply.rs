use crate::bool::{Bool, False, True};

pub trait Implies<Rhs: Bool>: Bool {
    type Result: Bool;
}

impl Implies<False> for False {
    type Result = True;
}

impl Implies<False> for True {
    type Result = False;
}

impl Implies<True> for False {
    type Result = True;
}

impl Implies<True> for True {
    type Result = True;
}
