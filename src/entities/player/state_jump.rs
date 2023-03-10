use macroquad::prelude::{KeyCode, is_key_down};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::collide, 
    consts::GRAVITY, 
    data::item::ItemType
};

use super::{
    player::Player, 
    state::State, 
    consts::{JUMP_RELEASE_VEL_Y, JUMP_VEL_Y, WALLSLIDE_VEL_Y, RUN_SPEED_X, MAX_FALL_VEL_Y}, 
    state_dash, 
    update_data::PlayerUpdateData, 
    state_normal
};

pub fn start(player: &mut Player) {
    player.pos.y -= 1.0;
    player.vel.y = JUMP_VEL_Y;
    player.state = State::Jump;
}

pub fn update(player: &mut Player, d: &PlayerUpdateData) {
    if let Some(dir) = get_dir_h_down(d.scene_state.reversed()) {
        player.dir = dir;
        let speed = player.vel.x.abs().max(RUN_SPEED_X);
        player.vel.x = dir.unit().x as f32 * speed;
    } else {
        player.vel.x = 0.0;
    }

    if !is_key_down(KeyCode::Z) {
        player.vel.y = player.vel.y.max(JUMP_RELEASE_VEL_Y);
    }

    player.pos += player.vel;
    player.vel += GRAVITY;

    player.vel.y = player.vel.y.min(MAX_FALL_VEL_Y);

    let deflection = collide(player.bounds(), d.get_colliders_near(player.bounds().center()), Some(d.level.bounds()));
    player.pos += deflection.as_fvec2();

    if player.vel.y > 0.0 && deflection.y < 0 {
        player.vel.y = 0.0;
        if Player::has_item(ItemType::Boots) &&
           is_key_down(KeyCode::Down) {
            state_dash::start(player.dir, player);
        } else {
            state_normal::start(player);
        }
    }
    else if player.vel.y < JUMP_RELEASE_VEL_Y && deflection.y > 0 {
        player.vel.y = JUMP_RELEASE_VEL_Y;
    }

    if Player::has_item(ItemType::Gloves) &&
       player.vel.y > WALLSLIDE_VEL_Y &&
       (player.vel.x * deflection.x as f32) < 0.0 {
        player.state = State::WallSlide
    }
}