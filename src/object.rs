use macroquad::prelude::{is_key_pressed, KeyCode};
use xf::{
    num::{ivec2::{IVec2, i2}, irect::ir}, 
    gl::color::Color
};

use crate::graphics::buffer::draw_rect;

pub struct Object {
    pub pos: IVec2,
}

const GREEN: Color = Color::rgba(0x00FF_00FF);

impl Object {
    pub fn new() -> Self {
        Self {
            pos: i2(10, 10),
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.pos.x -= 2;
        }
        if is_key_pressed(KeyCode::Right) {
            self.pos.x += 2;
        }
        if is_key_pressed(KeyCode::Up) {
            self.pos.y -= 2;
        }
        if is_key_pressed(KeyCode::Down) {
            self.pos.y += 2;
        }
    }

    pub fn draw(&self) {
        let rect = ir(self.pos, i2(10, 10));
        draw_rect(rect, GREEN);
    }
}