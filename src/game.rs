use std::time::SystemTime;

use macroquad::{window::next_frame, prelude::{is_key_pressed, KeyCode}};
use xf::{num::{ivec2::i2, irect::ir}, time::time};

use crate::{
    graphics::{buffer::render_buffer, camera, window::set_scale},
    level::level::Level, 
    common::update_data::UpdateData, 
    consts::{VIEW_SIZE, HUD_ORIGIN}, 
    entities::{player::{player::Player, update_data::PlayerUpdateData}, entities::Entities}, ui::hud, data::item::ItemType, global
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
        check_toggle_item();
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


// todo: delete
fn check_toggle_item() {
    use KeyCode::*;
    const KEY_CODES: [KeyCode; ItemType::COUNT] =
        [Q, W, E];
        
    for (item_idx, &key_code) in KEY_CODES.iter().enumerate() {
        if is_key_pressed(key_code) {
            let item = &mut global::player_state::get_mut().items[item_idx];
            *item = !(*item);
        }
    }
}