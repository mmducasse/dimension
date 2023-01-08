use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::collide, 
    consts::GRAVITY, 
    data::item::ItemType, 
    global
};

use super::{player::Player, state::State, consts::RUN_SPEED_X, state_jump, state_dash, update_data::PlayerUpdateData};


pub fn start(player: &mut Player) {
    player.state = State::Idle;
    global::player_state::get_mut().last_checkpoint_pos = player.pos;
}

pub fn update(player: &mut Player, d: &PlayerUpdateData) {
    if let Some(dir) = get_dir_h_down(d.scene_state.reversed()) {
        player.dir = dir;
        player.state = State::Run;
        player.vel.x = dir.unit().x as f32 * RUN_SPEED_X;
    } else {
        player.state = State::Idle;
        player.vel.x = 0.0;
    }

    if Player::has_item(ItemType::Boots) &&
       player.state == State::Run &&
       is_key_pressed(KeyCode::Down) {
        state_dash::start(player.dir, player);
    }
    else if is_key_pressed(KeyCode::Z) {
        state_jump::start(player);
    }

    player.pos += player.vel;
    player.vel += GRAVITY;

    let deflection = collide(player.bounds(), d.get_colliders_near(player.bounds().center()), Some(d.level.bounds()));
    player.pos += deflection.as_fvec2();

    if player.vel.y > 0.0 && deflection.y < 0 {
        player.vel.y = 0.0;
    }

    if player.vel.y > GRAVITY.y * 4.0 {
        player.state = State::Jump;
    }
}