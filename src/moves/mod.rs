pub mod parse_error;

use std::fmt::{self, Formatter};
use regex::Regex;

use parse_error::*;

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
            Move::U => write!(f, "U"), Move::Up => write!(f, "U'"),
            Move::D => write!(f, "D"), Move::Dp => write!(f, "D'"),
            Move::R => write!(f, "R"), Move::Rp => write!(f, "R'"),
            Move::L => write!(f, "L"), Move::Lp => write!(f, "L'"),
            Move::F => write!(f, "F"), Move::Fp => write!(f, "F'"),
            Move::B => write!(f, "B"), Move::Bp => write!(f, "B'"),

            Move::X => write!(f, "X"), Move::Xp => write!(f, "X'"),
            Move::Y => write!(f, "Y"), Move::Yp => write!(f, "Y'"),
            Move::Z => write!(f, "Z"), Move::Zp => write!(f, "Z'"),

            Move::M(_) => write!(f, "M"), Move::Mp(_) => write!(f, "M'"),
            Move::E(_) => write!(f, "E"), Move::Ep(_) => write!(f, "E'"),
            Move::S(_) => write!(f, "S"), Move::Sp(_) => write!(f, "S'"),
        }
    }
}

const REGEX: &str = r"(?<c>\d*)(?<a>[udrlfbxyzmesUDRLFBXYZMES]'?)(?<l>\d*)";

impl Move {
    /// Given a string, returns all matched and parseable Move notation substrings
    /// as an in-order `Vec<Move>,` and all matched but improper substrings as an
    /// in-order `Vec<ParseErrorDetails>`
    pub fn parse_moves(pattern: &str, size: usize) -> (Vec<Self>, Vec<ParseErrorDetails<'_>>) {
        let reg = Regex::new(REGEX).unwrap();
        let mut moves = Vec::new();
        let mut errors = Vec::new();

        for item in reg.captures_iter(pattern) {
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
                            m: item.get(0).unwrap().as_str()
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
                return Err(ParseError::InvalidLayer);
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

        let caps = caps_vec(test_input);

        assert_eq!(caps.len(), 4);

        let cap = &caps[0];
        assert_eq!(&cap[0], "u");
        assert!(&cap["c"].is_empty());
        assert_eq!(&cap["a"], "u");
        assert!(&cap["l"].is_empty());

        let cap = &caps[1];
        assert_eq!(&cap[0], "m'1");
        assert!(&cap["c"].is_empty());
        assert_eq!(&cap["a"], "m'");
        assert_eq!(&cap["l"], "1");

        let cap = &caps[2];
        assert_eq!(&cap[0], "2R");
        assert_eq!(&cap["c"], "2");
        assert_eq!(&cap["a"], "R");
        assert!(&cap["l"].is_empty());

        let cap = &caps[3];
        assert_eq!(&cap[0], "3B'4");
        assert_eq!(&cap["c"], "3");
        assert_eq!(&cap["a"], "B'");
        assert_eq!(&cap["l"], "4");
    }

    #[test]
    fn parsemove_err_missinglayer() {
        let caps = caps_vec("m");
        let result = Move::parse_move(&caps[0], 4);

        assert!(result.is_err_and(|e| matches!(e, ParseError::MissingLayer)));
    }

    #[test]
    fn parsemove_err_toosmallforslice() {
        let caps = caps_vec("m");
        let result = Move::parse_move(&caps[0], 2);

        assert!(result.is_err_and(|e| matches!(e, ParseError::TooSmallForSlice)));
    }

    #[test]
    fn parsemove_err_unnecessarylayer() {
        let caps = caps_vec("r2");
        let result = Move::parse_move(&caps[0], 4);

        assert!(result.is_err_and(|e| matches!(e, ParseError::UnnecessaryLayer)));
    }

    #[test]
    fn parsemove_err_invalidlayer_too_small() {
        let caps = caps_vec("m1");
        let result = Move::parse_move(&caps[0], 4);

        assert!(result.is_err_and(|e| matches!(e, ParseError::InvalidLayer)));
    }

    #[test]
    fn parsemove_err_invalidlayer_too_big() {
        let caps = caps_vec("m5");
        let result = Move::parse_move(&caps[0], 4);

        assert!(result.is_err_and(|e| matches!(e, ParseError::InvalidLayer)));
    }

    #[test]
    fn parsemove_user_layer() {
        let caps = caps_vec("m4");

        let result = Move::parse_move(&caps[0], 5);
        let expected = Ok((
            1,
            Move::M(3)
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parsemove_auto_layer() {
        let caps = caps_vec("m");

        let result = Move::parse_move(&caps[0], 3);
        let expected = Ok((
            1,
            Move::M(1)
        ));
        
        assert_eq!(result, expected);
    }

    #[test]
    fn parsemove_big_count() {
        let caps = caps_vec("6r");

        let result = Move::parse_move(&caps[0], 3);
        let expected = Ok((
            2,
            Move::R
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parsemove_missing_count() {
        let caps = caps_vec("r");

        let result = Move::parse_move(&caps[0], 3);
        let expected = Ok((
            1,
            Move::R
        ));

        assert_eq!(result, expected);
    }

    #[test]
    fn parsemoves_empty_string() {
        let input = "";
        let (moves, errors) = Move::parse_moves(input, 3);

        assert!(moves.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn parsemoves_no_matches() {
        let input = "123";
        let (moves, errors) = Move::parse_moves(input, 3);

        assert!(moves.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn parsemoves_convert_all_normals() {
        let input = "udlrfbxyzmes";
        let (moves, errors) = Move::parse_moves(input, 3);

        assert_eq!(moves.len(), 12);
        assert_eq!(moves[0], Move::U);
        assert_eq!(moves[1], Move::D);
        assert_eq!(moves[2], Move::L);
        assert_eq!(moves[3], Move::R);
        assert_eq!(moves[4], Move::F);
        assert_eq!(moves[5], Move::B);
        assert_eq!(moves[6], Move::X);
        assert_eq!(moves[7], Move::Y);
        assert_eq!(moves[8], Move::Z);
        assert_eq!(moves[9], Move::M(1));
        assert_eq!(moves[10], Move::E(1));
        assert_eq!(moves[11], Move::S(1));

        assert!(errors.is_empty());
    }

    #[test]
    fn parsemoves_convert_all_primes() {
        let input = "u'd'l'r'f'b'x'y'z'm'e's'";
        let (moves, errors) = Move::parse_moves(input, 3);

        assert_eq!(moves.len(), 12);
        assert_eq!(moves[0], Move::Up);
        assert_eq!(moves[1], Move::Dp);
        assert_eq!(moves[2], Move::Lp);
        assert_eq!(moves[3], Move::Rp);
        assert_eq!(moves[4], Move::Fp);
        assert_eq!(moves[5], Move::Bp);
        assert_eq!(moves[6], Move::Xp);
        assert_eq!(moves[7], Move::Yp);
        assert_eq!(moves[8], Move::Zp);
        assert_eq!(moves[9], Move::Mp(1));
        assert_eq!(moves[10], Move::Ep(1));
        assert_eq!(moves[11], Move::Sp(1));

        assert!(errors.is_empty());
    }

    #[test]
    fn parsemoves_count_is_correct() {
        let input = "3u";
        let (moves, errors) = Move::parse_moves(input, 3);

        assert_eq!(moves.len(), 3);
        assert!(errors.is_empty());
    }

    #[test]
    fn parsemoves_mixed_errors_and_moves() {
        // errors don't disrupt following parses
        let input = "u3 r m3";
        let (moves, errors) = Move::parse_moves(input, 3);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::R);

        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0], 
            ParseErrorDetails {
                e: ParseError::UnnecessaryLayer,
                m: "u3"
            }
        );
        assert_eq!(
            errors[1],
            ParseErrorDetails {
                e: ParseError::InvalidLayer,
                m: "m3"
            }
        );
    }
}
