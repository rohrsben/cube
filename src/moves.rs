use std::fmt::{self, Formatter};
use regex::Regex;

use Move::*;
use ParseError::*;

enum ParseError {
    InvalidLayer,
    MissingLayer,
    TooSmallForSlice
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            InvalidLayer => write!(f, "Slice layer must be between 2 and (cube size - 1)"),
            MissingLayer => write!(f, "Slice moves on cubes of size > 3 require a layer"),
            TooSmallForSlice => write!(f, "Cube is too small for a slice. Please use a side turn like U or R")
        }
    }
}

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
    pub fn parse_moves(pattern: String, size: usize, panic_on_invalid: bool) -> Vec<Self> {
        let regex = Regex::new(r"(?<c>\d*)(?<a>[udrlfbxyzmesUDRLFBXYZMES]'?)(?<l>\d*)").unwrap();
        let mut moves = Vec::new();

        for item in regex.captures_iter(&pattern) {
            match Move::parse_move(&item, size) {
                Ok(result) => {
                    for _ in 0..(result.0) {
                        moves.push(result.1);
                    }
                }
                Err(e) => {
                    if panic_on_invalid {
                        println!("{}: at {}", e, &item[0]);
                        panic!()
                    } else {
                        println!("Skipping invalid move: {}", &item[0]);
                    }
                }
            }
        }

        moves
    }

    fn parse_move(input: &regex::Captures, size: usize) -> Result<(usize, Self), ParseError> {
        let count = match input["c"].is_empty() {
            true => 1,
            false => input["c"].parse::<usize>().unwrap() % 4 // 4 in a row of the same move is a no-op
        };

        let action = input["a"].to_uppercase();

        let layer = match input["l"].is_empty() {
            true => {
                if matches!(action.as_str(), "M" | "E" | "S") {
                    if size > 3 {
                        return Err(MissingLayer);
                    } else if size < 3 {
                        return Err(TooSmallForSlice);
                    } else {
                        1
                    }
                } else {
                    1
                }
            }
            false => {
                let user_layer = input["l"].parse::<usize>().unwrap() - 1;

                if user_layer < 1 || user_layer > size - 2 {
                    return Err(InvalidLayer)
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
