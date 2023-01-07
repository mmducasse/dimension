use macroquad::prelude::*;

use crate::common::dir_h::DirH;

pub fn get_dir_h_down(reverse: bool) -> Option<DirH> {
    if is_key_down(KeyCode::Left) {
        Some(if reverse { DirH::R } else { DirH::L })
    } else if is_key_down(KeyCode::Right) {
        Some(if reverse { DirH::L } else { DirH::R })
    } else {
        None
    }
}