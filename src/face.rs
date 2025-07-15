use crate::tile_color::TileColor;

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
            Face::Front  => TileColor::White,
            Face::Top    => TileColor::Green,
            Face::Right  => TileColor::Orange,
            Face::Left   => TileColor::Red,
            Face::Bottom => TileColor::Blue,
            Face::Back   => TileColor::Yellow
        }
    }

    pub fn as_vec() -> Vec<Face> {
        vec![Face::Front, Face::Top, Face::Right, Face::Left, Face::Bottom, Face::Back]
    }
}
