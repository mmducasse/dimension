use super::tileset_info::{TilesetInfo, TilesetId};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TilemapId {
    Day,
    Night,
}

impl TilemapId {
    pub const fn info(self) -> TilemapInfo {
        use TilemapId::*;

        match self {
            Day => DAY_TILEMAP,
            Night => todo!(),
        }
    }
}

pub struct TilemapInfo {
    pub tilemap: &'static [u8],
    pub tileset_info: &'static TilesetInfo,
}

const DAY_TILEMAP: TilemapInfo = TilemapInfo {
    tilemap: include_bytes!("../../assets/Tilemaps/DayLevel.tmj"),
    tileset_info: &TilesetId::Day.info(),
};