use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{ 
    common::update_data::UpdateData, 
    systems::collision::collide,
};

use super::{player::Player, state::State, consts::{WALLSLIDE_VEL_Y}, state_jump};


pub fn update(player: &mut Player, d: &UpdateData) {
    player.vel.x = player.dir.unit().x as f32 * WALLSLIDE_VEL_Y;
    player.vel.y = WALLSLIDE_VEL_Y;
    player.pos += player.vel;

    let deflection = collide(player.bounds(), d.level.get_colliders_near(player.bounds().center()), Some(d.level.bounds()));
    player.pos += deflection.as_fvec2();

    if is_key_pressed(KeyCode::Z) {
        state_jump::start(player);
        return;
    }
    else if player.vel.y > 0.0 && deflection.y < 0 {
        player.vel.y = 0.0;
        player.state = State::Idle;
    }
    else if deflection.x == 0 {
        player.state = State::Idle;
    }
}