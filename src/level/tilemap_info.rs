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
            Night => NIGHT_TILEMAP,
        }
    }
}

pub struct TilemapInfo {
    pub tilemap: &'static [u8],
    pub tileset_info: &'static TilesetInfo,
}

const DAY_TILEMAP: TilemapInfo = TilemapInfo {
    tilemap: include_bytes!("../../assets/Tilemaps/TestLevel02.tmj"),
    tileset_info: &TilesetId::Day.info(),
};

const NIGHT_TILEMAP: TilemapInfo = TilemapInfo {
    tilemap: include_bytes!("../../assets/Tilemaps/TestLevel02_Nite.tmj"),
    tileset_info: &TilesetId::Night.info(),
};