use std::str::FromStr;

use crate::common::void::Void;



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Wall,
    Water,
    Spikes,
}

impl TileType {
    pub fn is_impassable(self) -> bool {
        use TileType::*;

        matches!(self, Wall)
    }
}

impl Default for TileType {
    fn default() -> Self { TileType::Empty }
}

impl FromStr for TileType {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TileType::*;

        Ok(match s {
            "Wall" => Wall,
            "Water" => Water,
            "Spikes" => Spikes,
            _ => Empty,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub type_: TileType,
    pub frames: Option<u64>,
}

impl Default for Tile {
    fn default() -> Self {
        Self { 
            type_: Default::default(), 
            frames: None, 
        }
    }
}