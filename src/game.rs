use std::time::SystemTime;

use macroquad::{window::next_frame, prelude::{is_key_pressed, KeyCode}};
use xf::{num::{ivec2::i2, irect::ir}, time::time};

use crate::{
    graphics::{buffer::render_buffer, camera, window::set_scale},
    level::level::Level, 
    common::update_data::UpdateData, 
    consts::{SCREEN_SIZE, VIEW_SIZE, HUD_P16_ORIGIN, HUD_ORIGIN}, 
    entities::{player::{player::Player, update_data::PlayerUpdateData}, entities::Entities}, ui::hud
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
        let org = camera::follow(player.bounds().center(), VIEW_SIZE, level.bounds());

        level.draw(ir(org, VIEW_SIZE));
        entities.draw(org);
        player.draw(org);
        hud::draw(HUD_ORIGIN);
        
        // Finish frame.
        check_requested_new_scale();
        render_buffer();
        next_frame().await;
    }
}

fn check_requested_new_scale() {
    use KeyCode::*;
    const KEY_CODES: [KeyCode; 9] =
        [Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8];
        
    for (scale, &key_code) in KEY_CODES.iter().enumerate() {
        if is_key_pressed(key_code) {
            set_scale(scale);
        }
    }
}

