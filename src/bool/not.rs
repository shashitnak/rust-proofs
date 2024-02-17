use crate::bool::{Bool, False, True};

pub trait Not: Bool {
    type Result: Bool;
}

impl Not for True {
    type Result = False;
}

impl Not for False {
    type Result = True;
}
