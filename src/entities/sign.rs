use xf::num::{ivec2::{IVec2, i2}, irect::{ir, IRect}};

use crate::{
    data::scene_state::SceneState, 
    graphics::{textures::TextureId, buffer::draw_texture}, 
    consts::P16, ui::text,
};

use super::entity::{Entity, next_entity_id, UpdateData, DrawData};


const SIZE_P16: IVec2 = i2(1, 2);

pub struct Sign {
    id: usize,
    pub pos: IVec2,
    pub message: String,
    pub show: bool,
}

impl Sign {
    pub fn new(pos: IVec2, message: String) -> Self {
        Self {
            id: next_entity_id(),
            pos,
            message,
            show: false,
        }
    }
}

impl Entity for Sign {
    fn id(&self) -> usize { self.id }

    fn bounds(&self) -> IRect {
        ir(self.pos, SIZE_P16 * P16)
    }

    fn update(&mut self, d: &mut UpdateData) {
        self.show = self.bounds().intersection(d.player.bounds()).is_some();
    }

    fn draw(&self, d: &DrawData) {
        let texture = TextureId::Misc.texture();
        
        let src = ir(src_p16(d.scene_state) * P16, SIZE_P16 * P16);
        let dst_pos = self.pos - d.org;
        draw_texture(&texture, src, dst_pos);

        if self.show {
            text::draw(&self.message, self.pos - d.org, true);
        }
    }
}

fn src_p16(scene_state: SceneState) -> IVec2 {
    match scene_state {
        SceneState::Day => i2(2, 0),
        SceneState::Night => i2(3, 0),
    }
}