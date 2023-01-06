use xf::{map::tilemap::Tilemap, num::{ivec2::IVec2, irect::ir}};

use crate::{graphics::buffer::draw_texture, consts::P16};

use super::tile::Tile;



pub struct Room {
    pub tilemap: Tilemap<Tile>,
}

impl Room {
    pub fn draw(&self, org: IVec2) {
        for pos_p16 in self.tilemap.tile_srcs.bounds().iter() {
            if let Some(tile_src_p16) = self.tilemap.tile_srcs[pos_p16] {
                let src = ir(tile_src_p16 * P16, P16);
                let dst_pt = pos_p16 * P16;
    
                draw_texture(&self.tilemap.tileset.texture, src, dst_pt - org);
            }
        }
    }
}