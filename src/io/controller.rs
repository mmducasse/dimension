use macroquad::prelude::*;

pub fn get_dpad_down() -> IVec2 {
    let mut dpad = IVec2::ZERO;

    if is_key_down(KeyCode::Up) { dpad.y -= 1; }
    if is_key_down(KeyCode::Down) { dpad.y += 1; }

    if is_key_down(KeyCode::Left) { dpad.x -= 1; }
    if is_key_down(KeyCode::Right) { dpad.x += 1; }

    dpad
}

pub fn get_dpad_pressed() -> IVec2 {
    let mut dpad = IVec2::ZERO;

    if is_key_pressed(KeyCode::Up) { dpad.y -= 1; }
    if is_key_pressed(KeyCode::Down) { dpad.y += 1; }

    if is_key_pressed(KeyCode::Left) { dpad.x -= 1; }
    if is_key_pressed(KeyCode::Right) { dpad.x += 1; }

    dpad
}