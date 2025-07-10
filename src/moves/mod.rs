pub mod parse_error;

use std::fmt::{self, Formatter};
use regex::Regex;

use parse_error::*;

use Move::*;

#[derive(Debug, Copy, Clone)]
pub enum Move {
    // turns
    U, Up,
    D, Dp,
    R, Rp,
    L, Lp,
    F, Fp,
    B, Bp,

    // rotations
    X, Xp,
    Y, Yp,
    Z, Zp,

    // slices
    M(usize), Mp(usize),
    E(usize), Ep(usize),
    S(usize), Sp(usize)
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            U => write!(f, "U"), Up => write!(f, "U'"),
            D => write!(f, "D"), Dp => write!(f, "D'"),
            R => write!(f, "R"), Rp => write!(f, "R'"),
            L => write!(f, "L"), Lp => write!(f, "L'"),
            F => write!(f, "F"), Fp => write!(f, "F'"),
            B => write!(f, "B"), Bp => write!(f, "B'"),

            X => write!(f, "X"), Xp => write!(f, "X'"),
            Y => write!(f, "Y"), Yp => write!(f, "Y'"),
            Z => write!(f, "Z"), Zp => write!(f, "Z'"),

            M(_) => write!(f, "M"), Mp(_) => write!(f, "M'"),
            E(_) => write!(f, "E"), Ep(_) => write!(f, "E'"),
            S(_) => write!(f, "S"), Sp(_) => write!(f, "S'"),
        }
    }
}


impl Move {
    pub fn parse_moves(pattern: String, size: usize) -> (Vec<Self>, Vec<ParseErrorDetails>) {
        let regex = Regex::new(r"(?<c>\d*)(?<a>[udrlfbxyzmesUDRLFBXYZMES]'?)(?<l>\d*)").unwrap();
        let mut moves = Vec::new();
        let mut errors = Vec::new();

        for item in regex.captures_iter(&pattern) {
            match Move::parse_move(&item, size) {
                Ok(result) => {
                    for _ in 0..(result.0) {
                        moves.push(result.1);
                    }
                }
                Err(e) => {
                    errors.push(
                        ParseErrorDetails { 
                            e,
                            m: item[0].to_string()
                        }
                    );
                }
            }
        }

        (moves, errors)
    }

    fn parse_move(input: &regex::Captures, size: usize) -> Result<(usize, Self), ParseError> {
        let count = match input["c"].is_empty() {
            true => 1,
            false => input["c"].parse::<usize>().unwrap() % 4 // 4 in a row of the same move is a no-op
        };

        let action = input["a"].to_uppercase();

        let layer = match input["l"].is_empty() {
            true => {
                if matches!(action.as_str(), "M" | "M'" | "E" | "E'" | "S" | "S'") {
                    if size > 3 { return Err(ParseError::MissingLayer); }
                    if size < 3 { return Err(ParseError::TooSmallForSlice); }
                }

                1 // targeting the middle layer on a 3x3 does not need to be specified
            }
            false => {
                if !matches!(action.as_str(), "M" | "M'" | "E" | "E'" | "S" | "S'") {
                    return Err(ParseError::UnnecessaryLayer);
                }

                let user_layer = input["l"].parse::<usize>().unwrap() - 1;

                if user_layer < 1 || user_layer > size - 2 {
                    return Err(ParseError::InvalidLayer)
                }

                user_layer
            }
        };

        let action = match action.as_str() {
            "U" => Move::U, "U'" => Move::Up,
            "D" => Move::D, "D'" => Move::Dp,
            "R" => Move::R, "R'" => Move::Rp,
            "L" => Move::L, "L'" => Move::Lp,
            "F" => Move::F, "F'" => Move::Fp,
            "B" => Move::B, "B'" => Move::Bp,

            "X" => Move::X, "X'" => Move::Xp,
            "Y" => Move::Y, "Y'" => Move::Yp,
            "Z" => Move::Z, "Z'" => Move::Zp,

            "M" => Move::M(layer), "M'" => Move::Mp(layer),
            "E" => Move::E(layer), "E'" => Move::Ep(layer),
            "S" => Move::S(layer), "S'" => Move::Sp(layer),

            _ => unreachable!()
        };

        Ok((count, action))
    }
}
