use xf::num::{irect::{IRect, rect, ir}, ivec2::IVec2};

use crate::{
    level::level::Level, 
    entities::entities::Entities, 
    data::scene_state::SceneState, 
    consts::P16
};



pub struct PlayerUpdateData<'a> {
    pub level: &'a Level,
    pub entities: &'a Entities,
    pub scene_state: SceneState,
}

impl<'a> PlayerUpdateData<'a> {
    /// Returns colliders for all tiles.
    pub fn get_colliders_near(&self, center: IVec2) -> Vec<IRect> {
        const AREA: IRect = rect(-1, -1, 3, 3);
        let mut vec = vec![];

        let pos_p16 = center / P16;

        // Get bounds of colliding tiles.
        let tilemap = &self.level.curr_room(self.scene_state).tilemap;

        for offset in AREA.iter() {
            let tile_p16_pos = pos_p16 + offset;
            if let Some(tile) = tilemap.get(tile_p16_pos) {
                if tile.type_.is_impassable() {
                    let tile_bounds = ir(tile_p16_pos * P16, P16);
    
                    vec.push(tile_bounds);
                }
            }
        }

        // Get bounds of colliding entities.
        for e in self.level.curr_room(self.scene_state).entities.all() {
            if e.collides() {
                vec.push(e.bounds());
            }
        }


        vec
    }
}