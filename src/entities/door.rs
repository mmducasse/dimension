use macroquad::prelude::{is_key_pressed, KeyCode};
use xf::num::{ivec2::{IVec2, i2}, irect::{ir, IRect}};

use crate::{
    common::update_data::UpdateData, 
    graphics::{textures::TextureId, buffer::draw_texture}, 
    consts::P16,
};

use super::entity::{Entity, next_entity_id};


const SRC_P16: IVec2 = i2(0, 0);
const SIZE_P16: IVec2 = i2(1, 2);

pub struct Door {
    id: usize,
    pub pos: IVec2,
}

impl Door {
    pub fn new(pos: IVec2) -> Self {
        Self {
            id: next_entity_id(),
            pos,
        }
    }
}

impl Entity for Door {
    fn id(&self) -> usize { self.id }

    fn bounds(&self) -> IRect {
        ir(self.pos, SIZE_P16 * P16)
    }

    fn update(&mut self, d: &mut UpdateData) {
        if self.bounds().intersection(d.player.bounds()).is_some() &&
           is_key_pressed(KeyCode::Up) &&
           d.player.can_enter_door()
        {
            println!("Enter door {}", self.id);
            d.entered_door = true;
        }
    }

    fn draw(&self, org: IVec2) {
        let texture = TextureId::Misc.texture();
    
        let src = ir(SRC_P16 * P16, SIZE_P16 * P16);
        let dst_pos = self.pos - org;
        draw_texture(&texture, src, dst_pos);
    }
}