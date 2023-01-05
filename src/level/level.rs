use std::rc::Rc;
use std::str::FromStr;

use macroquad::texture::Image;
use xf::map::tiled_json::tilemap::JsonTilemap;
use xf::map::tilemap::Tilemap;
use xf::map::{tileset::Tileset, tiled_json::tileset::JsonTile};
use xf::map::tiled_json::tileset::JsonTileset;
use xf::num::ivec2::IVec2;

use crate::graphics::image::convert_mq_image_to_xf_texture;

use super::{room::Room, level_info::LevelId, tile::Tile};



pub struct Level {
    pub room: Room,
}

impl Level {
    pub fn draw(&self, org: IVec2) {
        self.room.draw(org);
    }

    pub fn load(id: LevelId) -> Result<Self, String> {
        let tilemap_info = id.info().tilemap_info;
        let tileset_info = tilemap_info.tileset_info;

        // Load tileset image.
        let tileset_image = tileset_info.image;
        let image = Image::from_file_with_format(tileset_image, None);
        let texture = convert_mq_image_to_xf_texture(&image);

        // Load tileset JSON and convert to a Tileset.
        let tileset = tileset_info.tileset;
        let tileset_json: JsonTileset = serde_json::from_slice(tileset).or_else(|e| {
            Err(format!("Load Tileset JSON: {}",e.to_string()))
        })?;

        let tileset: Tileset<Tile> = Tileset::from_json(&tileset_json, texture, load_tile).or_else(|e| {
            Err(format!("Convert Tileset JSON: {}",e.to_string()))
        })?;

        // Load tilemap JSON and convert to a Tilemap.
        let tilemap_json: JsonTilemap = serde_json::from_slice(tilemap_info.tilemap).or_else(|e| {
            Err(format!("Load Tilemap JSON: {}",e.to_string()))
        })?;
        let tilemap_layers: Vec<Tilemap<Tile>> = Tilemap::from_json(&tilemap_json, Rc::new(tileset)).or_else(|e| {
            Err(format!("Convert Tilemap JSON: {}",e.to_string()))
        })?;
        let tilemap = tilemap_layers.into_iter().nth(0).unwrap();

        // Create level.
        let room = Room { tilemap };

        Ok(Self {
            room,
        })
    }
}

fn load_tile(json_tile: &JsonTile) -> Result<Tile, String> {
    Ok(if let Some(type_) = &json_tile.type_ {
        Tile::from_str(type_).unwrap()
    } else {
        Tile::Empty
    })
}