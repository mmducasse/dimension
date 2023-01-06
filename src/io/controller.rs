use macroquad::prelude::*;

use crate::common::dir_h::DirH;

pub fn get_dir_h_down() -> Option<DirH> {
    if is_key_down(KeyCode::Left) {
        Some(DirH::L)
    } else if is_key_down(KeyCode::Right) {
        Some(DirH::R)
    } else {
        None
    }
}