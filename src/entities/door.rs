use macroquad::prelude::{is_key_pressed, KeyCode};
use xf::num::{ivec2::{IVec2, i2}, irect::{ir, IRect}};

use crate::{ 
    graphics::{textures::TextureId, buffer::draw_texture}, 
    consts::P16, data::{scene_state::SceneState, item::ItemType},
};

use super::{entity::{Entity, next_entity_id, UpdateData, DrawData}, player::player::Player};


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
        if Player::has_item(ItemType::Glasses) &&
           self.bounds().intersection(d.player.bounds()).is_some() &&
           is_key_pressed(KeyCode::Up) &&
           d.player.can_enter_door()
        {
            println!("Enter door {}", self.id);
            d.entered_door = true;
        }
    }

    fn draw(&self, d: &DrawData) {
        if Player::has_item(ItemType::Glasses) {
            let texture = TextureId::Misc.texture();
        
            let src = ir(src_p16(d.scene_state) * P16, SIZE_P16 * P16);
            let dst_pos = self.pos - d.org;
            draw_texture(&texture, src, dst_pos);
        }
    }
}

fn src_p16(scene_state: SceneState) -> IVec2 {
    match scene_state {
        SceneState::Day => i2(0, 0),
        SceneState::Night => i2(1, 0),
    }
}