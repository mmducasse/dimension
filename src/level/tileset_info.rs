
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TilesetId {
    Day,
    Night,
}

impl TilesetId {
    pub const fn info(self) -> TilesetInfo {
        use TilesetId::*;

        match self {
            Day => DAY_TILESET,
            Night => NIGHT_TILESET,
        }
    }
}

pub struct TilesetInfo {
    pub tileset: &'static [u8],
    pub image: &'static [u8],
}

const DAY_TILESET: TilesetInfo = TilesetInfo {
    tileset: include_bytes!("../../assets/tilesets/DayTileset.tsj"),
    image: include_bytes!("../../assets/tilesets/DayTileset.png"),
};

const NIGHT_TILESET: TilesetInfo = TilesetInfo {
    tileset: include_bytes!("../../assets/tilesets/DayTileset.tsj"),
    image: include_bytes!("../../assets/tilesets/NightTileset.png"),
};