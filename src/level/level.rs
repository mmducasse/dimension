use std::rc::Rc;
use std::str::FromStr;

use macroquad::texture::Image;
use xf::map::tiled_json::tilemap::{JsonTilemap, Layer, Object};
use xf::map::tilemap::Tilemap;
use xf::map::{tileset::Tileset, tiled_json::tileset::JsonTile};
use xf::map::tiled_json::tileset::JsonTileset;
use xf::num::irect::IRect;
use xf::num::ivec2::{IVec2, i2};

use crate::consts::P16;
use crate::data::item::ItemType;
use crate::data::scene_state::SceneState;
use crate::entities::door::Door;
use crate::entities::entities::Entities;
use crate::entities::entity::{Entity, DrawData};
use crate::entities::gate::Gate;
use crate::entities::goal::Goal;
use crate::entities::item::Item;
use crate::entities::player_spawner::PlayerSpawner;
use crate::graphics::image::convert_mq_image_to_xf_texture;

use super::tile::TileType;
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

    pub fn curr_room_mut(&mut self, scene_state: SceneState) -> &mut Room {
        match scene_state {
            SceneState::Day => &mut self.day_room,
            SceneState::Night => &mut self.night_room,
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

    pub fn tile_type_at(&self, pos: IVec2, scene_state: SceneState) -> TileType {
        let pos_p16 = pos / P16;
        if let Some(tile) = self.curr_room(scene_state).tilemap.get(pos_p16) {
            tile.type_
        } else {
            TileType::Empty
        }
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
        Tile {
            type_: TileType::from_str(type_).unwrap(),
            frames: get_tile_property(json_tile, "Frames").map(|s| u64::from_str(s).unwrap()),
        }
    } else {
        Tile::default()
    })
}

fn get_tile_property<'a>(json_tile: &'a JsonTile, name: &str) -> Option<&'a str> {
    if let Some(properties) = &json_tile.properties {
        for property in properties {
            if property.name == name {
                return Some(&property.value)
            }
        }
    }
    
    None
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
        "Goal" => Box::new(Goal::new(pos)),
        "Player" => Box::new(PlayerSpawner::new(pos)),
        "Door" => Box::new(Door::new(pos)),
        "GateR" => Box::new(Gate::new(pos, ItemType::KeyRed)),
        "GateG" => Box::new(Gate::new(pos, ItemType::KeyGreen)),
        "GateB" => Box::new(Gate::new(pos, ItemType::KeyBlue)),
        _ => panic!("Unexpected object name: {}", object.name),
    }
}