use macroquad::window::next_frame;

use crate::{graphics::buffer::render_buffer, object::Object};

pub async fn run() {

    let mut object = Object::new();

    // Main game loop.
    loop {
        object.update();
        object.draw();
        
        render_buffer();
        next_frame().await;
    }
}

