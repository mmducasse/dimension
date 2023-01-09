
use xf::num::ivec2::{IVec2, i2};

use crate::{
    graphics::{textures::TextureId, buffer::draw_texture_full},
    consts::P8, common::time::min_sec_ms_string, global,
};

use super::text;

const TIME_POS_P8: IVec2 = i2(15, 15);

pub fn draw(org: IVec2) {
    let texture = TextureId::WinDlg.texture();
    draw_texture_full(&texture, org);

    let play_time = &global::player_state::get().play_time;
    let text = min_sec_ms_string(play_time);
    text::draw(&text, TIME_POS_P8 * P8);
}