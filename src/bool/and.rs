use crate::bool::{Bool, False, True};

pub trait And<Rhs: Bool>: Bool {
    type Result: Bool;
}

impl And<False> for False {
    type Result = False;
}

impl And<False> for True {
    type Result = False;
}

impl And<True> for False {
    type Result = False;
}

impl And<True> for True {
    type Result = True;
}
