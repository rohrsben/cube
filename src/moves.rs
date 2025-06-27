use std::fmt;
use std::fmt::Formatter;

use regex::Regex;

pub enum ParseError {
    InvalidLayer,
    MissingLayer,
}

#[derive(Debug, Clone, Copy)]
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
    M(u8), Mp(u8),
    E(u8), Ep(u8),
    S(u8), Sp(u8)
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::U => write!(f, "U"),
            Self::Up => write!(f, "U'"),
            Self::D => write!(f, "D"),
            Self::Dp => write!(f, "D'"),
            Self::R => write!(f, "R"),
            Self::Rp => write!(f, "R'"),
            Self::L => write!(f, "L"),
            Self::Lp => write!(f, "L'"),
            Self::F => write!(f, "F"),
            Self::Fp => write!(f, "F'"),
            Self::B => write!(f, "B"),
            Self::Bp => write!(f, "B'"),
            Self::X => write!(f, "X"),
            Self::Xp => write!(f, "X'"),
            Self::Y => write!(f, "Y"),
            Self::Yp => write!(f, "Y'"),
            Self::Z => write!(f, "Z"),
            Self::Zp => write!(f, "Z'"),
            Self::M(_) => write!(f, "M"),
            Self::Mp(_) => write!(f, "M'"),
            Self::E(_) => write!(f, "E"),
            Self::Ep(_) => write!(f, "E'"),
            Self::S(_) => write!(f, "S"),
            Self::Sp(_) => write!(f, "S'"),
        }
    }
}

impl Move {
    pub fn parse_moves(pattern: String, size: u8, panic_on_invalid: bool) -> Vec<Self> {
        let regex = Regex::new(r"(?<c>\d*)(?<a>[udrlfbxyzmesUDRLFBXYZMES]'?)(?<l>\d*)").unwrap();
        let mut moves = Vec::new();

        for item in regex.captures_iter(&pattern) {
            match Move::parse_move(&item, size) {
                Ok(result) => {
                    for _ in 0..(result.0 % 4) {
                        moves.push(result.1);
                    }
                }
                Err(e) => {
                    match panic_on_invalid {
                        true => {
                            let message = match e {
                                ParseError::InvalidLayer => &format!("Slice layer must be between 2 and {} (cube size - 1)", size - 1),
                                ParseError::MissingLayer => "Slice moves on cubes of size > 3 require a layer",
                            };

                            println!("{message}: at {}", &item[0]);
                            panic!()
                        }
                        false => {
                            println!("Skipping invalid move: {}", &item[0]);
                        }
                    }
                }
            }
        }

        moves
    }

    fn parse_move(input: &regex::Captures, size: u8) -> Result<(usize, Self), ParseError> {
        let count = input.name("c").unwrap();
        let count = if count.is_empty() {
            1
        } else {
            count.as_str().parse().unwrap()
        };

        let action = input.name("a").unwrap().as_str().to_uppercase();

        let layer = input.name("l").unwrap();
        let layer = if layer.is_empty() {
            if matches!(action.as_str(), "M" | "E" | "S") && size > 3 {
                return Err(ParseError::MissingLayer);
            } else {
                2
            }
        } else {
            layer.as_str().parse().unwrap()
        };

        if layer < 2 || layer > size - 1 {
            return Err(ParseError::InvalidLayer);
        }

        let action = match action.as_str() {
            "U" => Move::U,
            "U'" => Move::Up,
            "D" => Move::D,
            "D'" => Move::Dp,
            "R" => Move::R,
            "R'" => Move::Rp,
            "L" => Move::L,
            "L'" => Move::Lp,
            "F" => Move::F,
            "F'" => Move::Fp,
            "B" => Move::B,
            "B'" => Move::Bp,
            "X" => Move::X,
            "X'" => Move::Xp,
            "Y" => Move::Y,
            "Y'" => Move::Yp,
            "Z" => Move::Z,
            "Z'" => Move::Zp,
            "M" => Move::M(layer),
            "M'" => Move::Mp(layer),
            "E" => Move::E(layer),
            "E'" => Move::Ep(layer),
            "S" => Move::S(layer),
            "S'" => Move::Sp(layer),
            _ => unreachable!()
        };

        Ok((count, action))
    }
}

