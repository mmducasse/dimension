use std::str::FromStr;

use crate::common::void::Void;



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
}

impl Tile {
    pub fn is_impassable(self) -> bool {
        use Tile::*;

        matches!(self, Wall)
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Empty }
}

impl FromStr for Tile {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Tile::*;

        Ok(match s {
            "Wall" => Wall,
            _ => Empty,
        })
    }
}