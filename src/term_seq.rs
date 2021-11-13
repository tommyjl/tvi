use std::fmt::{Display, Formatter, Result};

pub struct MoveHome;

impl Display for MoveHome {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\x1B[H")
    }
}

pub struct ClearScreen;

impl Display for ClearScreen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\x1B[2J")
    }
}

pub struct Goto(pub usize, pub usize);

impl Display for Goto {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\x1B[{};{}H", self.1, self.0)
    }
}

pub struct AlternativeScreen;

impl Display for AlternativeScreen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\x1B[?1049h")
    }
}

pub struct NormalScreen;

impl Display for NormalScreen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\x1B[?1049l")
    }
}
