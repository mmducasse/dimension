use xf::num::{ivec2::IVec2, irect::{ir, IRect}};

use crate::{
    data::item::ItemType, 
    graphics::{textures::TextureId, buffer::draw_texture}, 
    consts::P16, global
};

use super::entity::{Entity, next_entity_id, UpdateData, DrawData};




pub struct Item {
    id: usize,
    pub pos: IVec2,
    pub type_: ItemType,
}

impl Item {
    pub fn new(pos: IVec2, type_: ItemType) -> Self {
        Self {
            id: next_entity_id(),
            pos,
            type_,
        }
    }
}

impl Entity for Item {
    fn id(&self) -> usize { self.id }

    fn bounds(&self) -> IRect {
        ir(self.pos, P16)
    }

    fn update(&mut self, d: &mut UpdateData) {
        if self.bounds().intersection(d.player.bounds()).is_some() {
            global::player_state::get_mut().items[self.type_ as usize] = true;
        }
    }

    fn draw(&self, d: &DrawData) {
        // Only draw if this item hasn't been found.
        if false == global::player_state::get().items[self.type_ as usize] {
            let texture = TextureId::Items.texture();
    
            let src = ir(self.type_.p16_src() * P16, P16);
            let dst_pos = self.pos - d.org;
            draw_texture(&texture, src, dst_pos);
        }
    }
}