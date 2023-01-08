use xf::num::{ivec2::IVec2, irect::IRect};

use crate::{data::scene_state::SceneState};

use super::player::player::Player;

static mut NEXT_ID: usize = 0;

pub fn next_entity_id() -> usize {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

pub struct UpdateData<'a> {
    pub player: &'a Player,
    pub entered_door: bool,
}

pub struct DrawData {
    pub org: IVec2,
    pub scene_state: SceneState,
}

pub trait Entity {
    fn id(&self) -> usize;
    fn bounds(&self) -> IRect;
    fn update(&mut self, d: &mut UpdateData);
    fn draw(&self, d: &DrawData);
}