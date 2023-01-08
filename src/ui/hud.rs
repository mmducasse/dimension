use xf::num::{ivec2::{IVec2, i2}, irect::ir};

use crate::{
    graphics::{textures::TextureId, buffer::{draw_texture_full, draw_texture}}, 
    data::item::ItemType, 
    consts::P16, 
    global
};



pub fn draw(org: IVec2) {
    let texture = TextureId::Hud.texture();
    draw_texture_full(&texture, org);

    let items_texture = TextureId::Items.texture();
    let mut pos = i2(32, 8);
    for (idx, item_type) in ItemType::ALL.iter().enumerate() {
        if global::player_state::get().items[idx] {
            let src = ir(item_type.p16_src() * P16, P16);
            let dst_pos = pos + org;
    
            draw_texture(&items_texture, src, dst_pos);
        }
        pos.x += if idx >= 4 { 16 } else { 32 };
    }
}