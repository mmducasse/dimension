use std::rc::Rc;
use std::str::FromStr;

use macroquad::texture::Image;
use xf::map::tiled_json::tilemap::{JsonTilemap, Layer, Object};
use xf::map::tilemap::Tilemap;
use xf::map::{tileset::Tileset, tiled_json::tileset::JsonTile};
use xf::map::tiled_json::tileset::JsonTileset;
use xf::num::irect::{IRect, ir, rect};
use xf::num::ivec2::{IVec2, i2};

use crate::consts::P16;
use crate::data::item::ItemType;
use crate::data::scene_state::SceneState;
use crate::entities::door::Door;
use crate::entities::entities::Entities;
use crate::entities::entity::{Entity, DrawData};
use crate::entities::item::Item;
use crate::graphics::image::convert_mq_image_to_xf_texture;

use super::tilemap_info::TilemapInfo;
use super::{room::Room, level_info::LevelId, tile::Tile};



pub struct Level {
    pub day_room: Room,
    pub night_room: Room,
}

impl Level {
    pub fn new(day_room: Room, night_room: Room) -> Self {
        assert_eq!(day_room.tilemap.size(), night_room.tilemap.size());

        Self {
            day_room,
            night_room,
        }
    }

    pub fn curr_room(&self, scene_state: SceneState) -> &Room {
        match scene_state {
            SceneState::Day => &self.day_room,
            SceneState::Night => &self.night_room,
        }
    }

    pub fn draw(&self, view: IRect, scene_state: SceneState) {
        self.curr_room(scene_state).draw(view);
        self.curr_room(scene_state).entities.draw(&DrawData {
            org: view.pos,
            scene_state,
        });
    }

    pub fn p16_size(&self) -> IVec2 {
        self.day_room.tilemap.size()
    }

    pub fn bounds(&self) -> IRect {
        IRect::of_size(self.p16_size() * P16)
    }

    /// Returns colliders for all tiles.
    pub fn get_colliders_near(&self, center: IVec2) -> Vec<IRect> {
        const AREA: IRect = rect(-1, -1, 3, 3);
        let mut vec = vec![];

        let pos_p16 = center / P16;

        // Get bounds of colliding tiles.
        let tilemap = &self.day_room.tilemap;

        for offset in AREA.iter() {
            let tile_p16_pos = pos_p16 + offset;
            if let Some(tile) = tilemap.get(tile_p16_pos) {
                if tile.is_impassable() {
                    let tile_bounds = ir(tile_p16_pos * P16, P16);
    
                    vec.push(tile_bounds);
                }
            }
        }
        
        vec
    }

    pub fn load(id: LevelId) -> Result<Self, String> {
        
        let day_room = load_room(&id.info().day_tilemap_info)?;
        let night_room = load_room(&id.info().night_tilemap_info)?;

        Ok(Self {
            day_room,
            night_room,
        })
    }
}

fn load_room(tilemap_info: &TilemapInfo) -> Result<Room, String> {
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
    Ok(Room { 
        tilemap,
        entities: load_entities(&tilemap_json),
    })
}

fn load_tile(json_tile: &JsonTile) -> Result<Tile, String> {
    Ok(if let Some(type_) = &json_tile.type_ {
        Tile::from_str(type_).unwrap()
    } else {
        Tile::Empty
    })
}

fn load_entities(json: &JsonTilemap) -> Entities {
    let mut entities = Entities::new();

    for layer in &json.layers {
        if let Layer::Objectgroup { objects, .. } = layer {
            for object in objects {
                let entity = load_entity(&object);
                entities.add(entity);
            }
        }
    }

    entities
}

fn load_entity(object: &Object) -> Box<dyn Entity> {
    let pos = i2(object.x, object.y);
    match object.name.as_str() {
        "Item" => Box::new(Item::new(
            pos, 
            ItemType::from_str(&object.type_).unwrap(),
        )),
        "Door" => Box::new(Door::new(
            pos,
        )),
        _ => panic!("Unexpected object name: {}", object.name),
    }
}