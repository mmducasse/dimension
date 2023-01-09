use std::time::SystemTime;

use macroquad::{window::next_frame, prelude::{is_key_pressed, KeyCode}};
use xf::{num::{ivec2::{i2, IVec2}, irect::ir}, time::time};

use crate::{
    graphics::{buffer::render_buffer, camera, window::set_scale},
    level::level::Level,
    consts::{VIEW_SIZE, HUD_ORIGIN}, 
    entities::{
        player::{player::Player, update_data::PlayerUpdateData}, entity::UpdateData,
    }, 
    ui::{hud, win_dlg}, 
    data::{scene_state::SceneState, transition_state::TransitionState}, global,
};


pub async fn run() {
    let mut level = match Level::load(crate::level::level_info::LevelId::Test) {
        Ok(level) => level,
        Err(e) => panic!("{}", e),
    };

    let mut transition_state = TransitionState::Play;
    let mut state = SceneState::Day;
    let mut player = Player::new(i2(32, 32));
    //let mut entities = Entities::new();   

    let mut time = SystemTime::now();

    // Main game loop.
    loop {
        let won = global::player_state::get().won;

        // Update time.
        let next_time = SystemTime::now();
        let delta = next_time.duration_since(time).unwrap_or_default();
        time::update(&delta);
        time = next_time;

        if !won {
            global::player_state::get_mut().play_time += delta;
        }

        // Update game state.
        let mut entered_door = false;
        if transition_state.is_play() && !won {
            let mut d = UpdateData {
                player: &mut player,
                entered_door: false,
            };
            level.curr_room_mut(state).entities.update(&mut d);
            entered_door = d.entered_door;
            drop(d);
    
    
            player.update(&PlayerUpdateData {
                level: &level,
                entities: &level.day_room.entities,
                scene_state: state,
            });
        }

        // Draw.
        let org = camera::follow(player.bounds().center(), VIEW_SIZE, level.bounds());

        level.draw(ir(org, VIEW_SIZE), state);
        player.draw(org);
        hud::draw(HUD_ORIGIN);

        if won {
            win_dlg::draw(IVec2::ZERO);
        }

        transition_state.update(entered_door);
        
        // Finish frame.
        check_requested_new_scale();
        check_toggle_item();
        render_buffer(state != SceneState::Day, transition_state.x_scale());

        if transition_state.needs_world_switch() {
            state = match state {
                SceneState::Day => SceneState::Night,
                SceneState::Night => SceneState::Day,
            }
        }

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

// todo: comment out
// for troubleshooting
fn check_toggle_item() {
    use crate::data::item::ItemType;
    use KeyCode::*;
    const KEY_CODES: [KeyCode; ItemType::COUNT] =
        [Q, W, E, R, T, Y, U];
        
    for (item_idx, &key_code) in KEY_CODES.iter().enumerate() {
        if is_key_pressed(key_code) {
            let item = &mut crate::global::player_state::get_mut().items[item_idx];
            *item = !(*item);
        }
    }
}