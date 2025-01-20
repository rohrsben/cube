use regex::Regex;
// use std::intrinsics::saturating_sub;

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub layer: Option<usize>,
    pub action:  String
}

pub fn parse_moves(pattern: String, size: usize, panic_on_invalid: bool) -> Vec<Move> {
    let reg = Regex::new(r"(?<count>\d*)(?<action>[UDRLFBXYZMES]'?)(?<layer>\d*)").unwrap();

    let mut moves = Vec::new();

    for item in pattern.split_whitespace() {
        let item = item.to_uppercase();
        let Some(cap) = reg.captures(&item) else {
            match panic_on_invalid {
                true => {
                    println!("Invalid input: {}", item);
                    panic!()
                }
                false => {
                    println!("Skipping invalid input: {}", item);
                    continue;
                }
            }
        };

        let count: usize = match cap.name("count").unwrap().as_str().parse() {
            Ok(n) => n,
            Err(_) => 1
        };

        if count < 1 {
            match panic_on_invalid {
                true => {
                    println!("Count must be a positive number: at {}", item);
                    panic!()
                }
                false => {
                    println!("Count not positive: skipping {}", item);
                    continue;
                }
            }
        }

        let mut layer = match cap.name("layer").unwrap().as_str().parse() {
            Ok(n) => Some(usize::saturating_sub(n, 1)),
            Err(_) => None
        };

        // match layer {
        //     Some(n) => satu
        // }

        let action = cap.name("action").unwrap().as_str().to_string();

        if action == "M" || action == "M'" || action == "E" || action == "E'" || action == "S" || action == "S'" {
            match layer {
                Some(n) => {
                    if n < 1 || n > size - 2 {
                        match panic_on_invalid {
                            true => {
                                println!("Slice layer must be between 2 and cube size - 1 (here {} - 1 = {}), inclusive: at {}", size, size - 1, item);
                                panic!()
                            }
                            false => {
                                println!("Skipping invalid slice layer (out of bounds) at {}", item);
                                continue;
                            }
                        }
                    }
                }
                None => {
                    if size == 3 {
                        layer = Some(1);
                    } else {
                        match panic_on_invalid {
                            true => {
                                println!("Slices require a layer: at {}", item);
                                panic!()
                            }
                            false => {
                                println!("Skipping slice with missing layer at {}", item);
                                continue;
                            }
                        }
                    }
                }
            }
        }

        moves.push(Move {
            count,
            layer,
            action
        });
    }

    moves
}
