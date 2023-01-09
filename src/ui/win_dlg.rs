
use xf::num::ivec2::{IVec2, i2};

use crate::{
    graphics::{textures::TextureId, buffer::draw_texture_full},
    consts::P8, common::time::min_sec_ms_string, global,
};

use super::text;

const TEXT_1_P8: IVec2 = i2(13, 10);
const TEXT_2_P8: IVec2 = i2(10, 15);
const TIME_POS_P8: IVec2 = i2(15, 15);
const TEXT_3_P8: IVec2 = i2(9, 20);

pub fn draw(org: IVec2) {
    let texture = TextureId::WinDlg.texture();
    draw_texture_full(&texture, org);

    // You win!!!
    text::draw("YOU WIN!!!", TEXT_1_P8 * P8, false);

    // Time:
    text::draw("Time:", TEXT_2_P8 * P8, false);

    // Play time
    let play_time = &global::player_state::get().play_time;
    let text = min_sec_ms_string(play_time);
    text::draw(&text, TIME_POS_P8 * P8, false);
    
    // Thank for playing
    text::draw("Thanks for playing!", TEXT_3_P8 * P8, false);
}