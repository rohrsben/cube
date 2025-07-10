use crate::tile_color::TileColor::{self, *};
use Face::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Face {
    Front,
    Top,
    Right,
    Left,
    Bottom,
    Back
}

impl Face {
    pub fn color(&self) -> TileColor {
        match self {
            Front  => White,
            Top    => Green,
            Right  => Orange,
            Left   => Red,
            Bottom => Blue,
            Back   => Yellow
        }
    }

    pub fn as_vec() -> Vec<Face> {
        vec![Front, Top, Right, Left, Bottom, Back]
    }
}
