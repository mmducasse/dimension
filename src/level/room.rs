use xf::{map::tilemap::Tilemap, num::{irect::{ir, IRect}, ivec2::i2}, time::time::frame_num};

use crate::{
    graphics::buffer::draw_texture, 
    consts::P16, 
    entities::entities::Entities
};

use super::tile::Tile;

const TILE_ANIMATION_RATE: u64 = 8;

pub struct Room {
    pub tilemap: Tilemap<Tile>,
    pub entities: Entities,
}

impl Room {
    pub fn draw(&self, view: IRect) {

        let view_p16 = ir(view.pos / P16, view.size / P16).expand(1);

        for tile_pos_p16 in view_p16.iter() {
            if let Some(&src) = self.tilemap.tile_srcs.get(tile_pos_p16) {
                if let Some(src) = src {
                    let tile = self.tilemap.tileset.tiles[src];
                    let frames = tile.frames.unwrap_or(1);

                    let src_offset_x = (frame_num() / TILE_ANIMATION_RATE) % frames;
                    let src = src + i2(src_offset_x as i32, 0);

                    let src = ir(src * P16, P16);
                    let dst_pt = tile_pos_p16 * P16;
        
                    draw_texture(&self.tilemap.tileset.texture, src, dst_pt - view.pos);
                }
            }
        }
    }
}