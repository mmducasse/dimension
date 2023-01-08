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
}

impl PlayerState {
    const fn new() -> Self {
        Self {
            // todo: set to false.
            items: [true; ItemType::COUNT],
        }
    }
}