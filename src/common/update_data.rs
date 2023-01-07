use crate::entities::player::player::Player;


pub struct UpdateData<'a> {
    pub player: &'a Player,
    pub entered_door: bool,
}