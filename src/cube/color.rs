// extern crate colored;
use colored::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Color {
    Green,
    White,
    Red,
    Orange,
    Yellow,
    Blue,
}

impl Color {
    pub fn to_string(self) -> ColoredString {
        match self {
            Color::Green  => "G".to_string().green(),
            Color::White  => "W".to_string().truecolor(255, 255, 255),
            Color::Red    => "R".to_string().red(),
            Color::Orange => "O".to_string().truecolor(255, 172, 28),
            Color::Yellow => "Y".to_string().truecolor(255, 250, 51),
            Color::Blue   => "B".to_string().blue(),
        }
    }
}
