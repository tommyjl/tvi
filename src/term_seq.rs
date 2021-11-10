use std::fmt;

pub struct MoveHome;

impl fmt::Display for MoveHome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[H")
    }
}

pub struct ClearScreen;

impl fmt::Display for ClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[2J")
    }
}

pub struct Goto(pub usize, pub usize);

impl fmt::Display for Goto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[{};{}H", self.1, self.0)
    }
}
