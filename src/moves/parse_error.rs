use std::fmt::{self, Formatter};

use ParseError::*;

pub enum ParseError {
    InvalidLayer,
    MissingLayer,
    TooSmallForSlice,
    UnnecessaryLayer
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidLayer => write!(f, "Slice layer must be between 2 and (cube size - 1)"),
            MissingLayer => write!(f, "Slice moves on cubes of size > 3 require a layer"),
            TooSmallForSlice => write!(f, "Cube is too small for a slice. Please use a side turn like U or R"),
            UnnecessaryLayer => write!(f, "Action does not require a layer. Maybe meant to be a count?")
        }
    }
}

pub struct ParseErrorDetails {
    pub e: ParseError,
    pub m: String
}

impl fmt::Display for ParseErrorDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "At {}: {}", self.m, self.e)
    }
}
