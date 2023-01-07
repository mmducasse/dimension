use std::time::SystemTime;

use macroquad::window::next_frame;
use xf::{num::{ivec2::i2, irect::ir}, time::time};

use crate::{
    graphics::{buffer::render_buffer, camera},
    level::level::Level, 
    common::update_data::UpdateData, 
    consts::SCREEN_SIZE, 
    entities::{player::{player::Player, update_data::PlayerUpdateData}, entities::Entities}
};

pub async fn run() {
    let level = match Level::load(crate::level::level_info::LevelId::Test) {
        Ok(level) => level,
        Err(e) => panic!("{}", e),
    };

    let mut player = Player::new(i2(32, 32));
    let mut entities = Entities::new();

    

    let mut time = SystemTime::now();

    // Main game loop.
    loop {
        // Update time.
        let next_time = SystemTime::now();
        time::update(&next_time.duration_since(time).unwrap_or_default());
        time = next_time;

        // Update game state.
        entities.update(&UpdateData {
            level: &level,
        });
        player.update(&PlayerUpdateData {
            level: &level,
            entities: &entities,
        });

        // Draw.
        let org = camera::follow(player.bounds().center(), SCREEN_SIZE, level.bounds());

        level.draw(ir(org, SCREEN_SIZE));
        entities.draw(org);
        player.draw(org);
        
        // Finish frame.
        render_buffer();
        next_frame().await;
    }
}

