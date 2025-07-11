pub mod parse_error;

use std::fmt::{self, Formatter};
use regex::Regex;

use parse_error::*;

use Move::*;

#[derive(Debug, Copy, Clone, PartialEq)]
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

const REGEX: &'static str = r"(?<c>\d*)(?<a>[udrlfbxyzmesUDRLFBXYZMES]'?)(?<l>\d*)";

impl Move {
    // TODO take an &str, and return references inside ParseErrorDetails
    pub fn parse_moves(pattern: String, size: usize) -> (Vec<Self>, Vec<ParseErrorDetails>) {
        let reg = Regex::new(REGEX).unwrap();
        let mut moves = Vec::new();
        let mut errors = Vec::new();

        for item in reg.captures_iter(&pattern) {
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

        let layer = if input["l"].is_empty() {
            if matches!(action.as_str(), "M" | "M'" | "E" | "E'" | "S" | "S'") {
                if size > 3 { return Err(ParseError::MissingLayer); }
                if size < 3 { return Err(ParseError::TooSmallForSlice); }
            }

            1 // layer doesn't need to be specified on a 3x3
        } else {
            if !matches!(action.as_str(), "M" | "M'" | "E" | "E'" | "S" | "S'") {
                return Err(ParseError::UnnecessaryLayer);
            }

            let user_layer = input["l"].parse::<usize>().unwrap() - 1; // - 1 to account for 0-index

            if user_layer < 1 || user_layer > size - 2 { // ranges are accounting for 0-index also
                return Err(ParseError::InvalidLayer)
            }

            user_layer
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


#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    fn caps_vec(input: &str) -> Vec<regex::Captures<'_>> {
        let regex = Regex::new(REGEX).unwrap();

        regex.captures_iter(input).collect()
    }

    #[test]
    fn regex_correctness() {
        let test_input = "u m'1 2R 3B'4 5g6";

        let mut caps = caps_vec(test_input);

        assert_eq!(caps.len(), 4);

        let cap = caps.pop().unwrap();
        assert_eq!(&cap[0], "3B'4");
        assert_eq!(&cap["c"], "3");
        assert_eq!(&cap["a"], "B'");
        assert_eq!(&cap["l"], "4");

        let cap = caps.pop().unwrap();
        assert_eq!(&cap[0], "2R");
        assert_eq!(&cap["c"], "2");
        assert_eq!(&cap["a"], "R");
        assert!(&cap["l"].is_empty());

        let cap = caps.pop().unwrap();
        assert_eq!(&cap[0], "m'1");
        assert!(&cap["c"].is_empty());
        assert_eq!(&cap["a"], "m'");
        assert_eq!(&cap["l"], "1");

        let cap = caps.pop().unwrap();
        assert_eq!(&cap[0], "u");
        assert!(&cap["c"].is_empty());
        assert_eq!(&cap["a"], "u");
        assert!(&cap["l"].is_empty());
    }

    #[test]
    fn parse_errors() {
        let test_input = "m5 m1 r2 m m";

        let mut caps = caps_vec(test_input);

        let cap = caps.pop().unwrap();
        let res = Move::parse_move(&cap, 4);
        assert!(res.is_err_and(|e| matches!(e, ParseError::MissingLayer)));

        let cap = caps.pop().unwrap();
        let res = Move::parse_move(&cap, 2);
        assert!(res.is_err_and(|e| matches!(e, ParseError::TooSmallForSlice)));

        let cap = caps.pop().unwrap();
        let res = Move::parse_move(&cap, 4);
        assert!(res.is_err_and(|e| matches!(e, ParseError::UnnecessaryLayer)));

        let cap = caps.pop().unwrap();
        let res = Move::parse_move(&cap, 4);
        assert!(res.is_err_and(|e| matches!(e, ParseError::InvalidLayer)));

        let cap = caps.pop().unwrap();
        let res = Move::parse_move(&cap, 4);
        assert!(res.is_err_and(|e| matches!(e, ParseError::InvalidLayer)));
    }

    #[test]
    fn parse_correctness() {
        // missing count
        let test_input = "r";
        let caps = caps_vec(test_input);
        let output = Move::parse_move(&caps[0], 3);
        assert_eq!(output, Ok((1, Move::R)));

        // big count
        let test_input = "6r";
        let caps = caps_vec(test_input);
        let output = Move::parse_move(&caps[0], 3);
        assert_eq!(output, Ok((2, Move::R)));

        // auto-layer on 3x3
        let test_input = "m";
        let caps = caps_vec(test_input);
        let output = Move::parse_move(&caps[0], 3);
        assert_eq!(output, Ok((1, Move::M(1))));

        // user-input layer on big cube
        let test_input = "m4";
        let caps = caps_vec(test_input);
        let output = Move::parse_move(&caps[0], 5);
        assert_eq!(output, Ok((1, Move::M(3))));
    }

    #[test]
    fn parse_moves_correctness() {
        // empty string produces nothing
        let test_input = "";
        let (moves, errors) = Move::parse_moves(test_input.to_string(), 3);
        assert!(moves.is_empty());
        assert!(errors.is_empty());

        // non-matching string produces nothing
        let test_input = "123";
        let (moves, errors) = Move::parse_moves(test_input.to_string(), 3);
        assert!(moves.is_empty());
        assert!(errors.is_empty());

        // all normal moves are converted
        let test_input = "udlrfbxyzmes";
        let (moves, errors) = Move::parse_moves(test_input.to_string(), 3);
        assert_eq!(moves.len(), 12);
        assert!(errors.is_empty());

        // all prime moves are converted
        let test_input = "u'd'l'r'f'b'x'y'z'm'e's'";
        let (moves, errors) = Move::parse_moves(test_input.to_string(), 3);
        assert_eq!(moves.len(), 12);
        assert!(errors.is_empty());

        // count gets applied correctly
        let test_input = "3u";
        let (moves, errors) = Move::parse_moves(test_input.to_string(), 3);
        assert_eq!(moves.len(), 3);
        assert!(errors.is_empty());

        // errors don't disrupt following parses
        let test_input = "u3 r m3";
        let (mut moves, mut errors) = Move::parse_moves(test_input.to_string(), 3);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves.pop().unwrap(), Move::R);

        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors.pop().unwrap(), 
            ParseErrorDetails {
                e: ParseError::InvalidLayer,
                m: "m3".to_string()
            }
        );
        assert_eq!(
            errors.pop().unwrap(),
            ParseErrorDetails {
                e: ParseError::UnnecessaryLayer,
                m: "u3".to_string()
            }
        );
    }
}
