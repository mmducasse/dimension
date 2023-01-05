use macroquad::window::next_frame;

use crate::{graphics::buffer::render_buffer, object::Object, level::level::Level};

pub async fn run() {

    let mut object = Object::new();

    let level = match Level::load(crate::level::level_info::LevelId::Test) {
        Ok(level) => level,
        Err(e) => panic!("{}", e),
    };

    // Main game loop.
    loop {
        object.update();
        //object.draw();

        level.draw(object.pos);
        
        render_buffer();
        next_frame().await;
    }
}

