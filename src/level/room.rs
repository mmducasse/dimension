use xf::{map::tilemap::Tilemap, num::irect::{ir, IRect}};

use crate::{
    graphics::buffer::draw_texture, 
    consts::P16, 
    entities::entities::Entities
};

use super::tile::Tile;



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
                    let src = ir(src * P16, P16);
                    let dst_pt = tile_pos_p16 * P16;
        
                    draw_texture(&self.tilemap.tileset.texture, src, dst_pt - view.pos);
                }
            }
        }
    }
}