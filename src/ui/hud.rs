use xf::num::ivec2::IVec2;

use crate::graphics::{textures::TextureId, buffer::draw_texture_full};



pub fn draw(org: IVec2) {
    let texture = TextureId::Hud.texture();
    draw_texture_full(&texture, org);
}