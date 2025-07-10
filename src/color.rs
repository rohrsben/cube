use colored::*;
use std::fmt::{self, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Green,
    White,
    Red,
    Orange,
    Yellow,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Green => write!(f, "{}", "G".green()),
            Self::White  => write!(f, "{}", "W".truecolor(255, 255, 255)),
            Self::Red    => write!(f, "{}", "R".red()),
            Self::Orange => write!(f, "{}", "O".truecolor(255, 172, 28)),
            Self::Yellow => write!(f, "{}", "Y".truecolor(255, 250, 51)),
            Self::Blue   => write!(f, "{}", "B".blue()),
        }
    }
}
