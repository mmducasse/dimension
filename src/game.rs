use macroquad::window::next_frame;
use xf::num::ivec2::{i2, IVec2};

use crate::{
    graphics::buffer::render_buffer,
    level::level::Level, 
    entities::{player::Player, entity::Entity}
};

pub async fn run() {
    let level = match Level::load(crate::level::level_info::LevelId::Test) {
        Ok(level) => level,
        Err(e) => panic!("{}", e),
    };

    let mut player = Player::new(i2(32, 32));

    // Main game loop.
    loop {
        player.update();
        //object.draw();

        level.draw(IVec2::ZERO);
        player.draw(IVec2::ZERO);
        
        render_buffer();
        next_frame().await;
    }
}

