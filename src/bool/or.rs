use crate::bool::{Bool, False, True};

pub trait Or<Rhs: Bool>: Bool {
    type Result: Bool;
}

impl Or<False> for False {
    type Result = False;
}

impl Or<False> for True {
    type Result = True;
}

impl Or<True> for False {
    type Result = True;
}

impl Or<True> for True {
    type Result = True;
}
