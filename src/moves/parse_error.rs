use std::fmt::{self, Formatter};

use ParseError::*;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
/// Used in Move::parse_moves. Contains the parsing error that was encountered, and
/// a string slice pointing to the substring that caused the error
pub struct ParseErrorDetails<'a> {
    pub e: ParseError,
    pub m: &'a str
}

impl fmt::Display for ParseErrorDetails<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "At {}: {}", self.m, self.e)
    }
}
