use xf::num::{ivec2::{IVec2, i2}, irect::{ir, IRect}};

use crate::{
    graphics::{textures::TextureId, buffer::draw_texture}, 
    consts::P16, global,
};

use super::entity::{Entity, next_entity_id, UpdateData, DrawData};


const SRC_P16: IVec2 = i2(3, 1);

pub struct Goal {
    id: usize,
    pub pos: IVec2,
}

impl Goal {
    pub fn new(pos: IVec2) -> Self {
        Self {
            id: next_entity_id(),
            pos,
        }
    }
}

impl Entity for Goal {
    fn id(&self) -> usize { self.id }

    fn bounds(&self) -> IRect {
        ir(self.pos, P16)
    }

    fn update(&mut self, d: &mut UpdateData) {
        if self.bounds().intersection(d.player.bounds()).is_some() {
            // Won game!
            global::player_state::get_mut().won = true;
        }
    }

    fn draw(&self, d: &DrawData) {
        let texture = TextureId::Items.texture();
    
        let src = ir(SRC_P16 * P16, P16);
        let dst_pos = self.pos - d.org;
        draw_texture(&texture, src, dst_pos);
    }
}