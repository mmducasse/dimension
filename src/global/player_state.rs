use std::time::Duration;

use xf::num::fvec2::FVec2;

use crate::data::item::ItemType;


static mut PLAYER_STATE: PlayerState = PlayerState::new();

pub fn get() -> &'static PlayerState {
    unsafe {
        &PLAYER_STATE
    }
}

pub fn get_mut() -> &'static mut PlayerState {
    unsafe {
        &mut PLAYER_STATE
    }
}


pub struct PlayerState {
    pub items: [bool; ItemType::COUNT],
    pub last_checkpoint_pos: FVec2,
    pub won: bool,
    pub play_time: Duration,
}

impl PlayerState {
    const fn new() -> Self {
        Self {
            items: [false; ItemType::COUNT],
            last_checkpoint_pos: FVec2::ZERO,
            won: false,
            play_time: Duration::ZERO,
        }
    }
}