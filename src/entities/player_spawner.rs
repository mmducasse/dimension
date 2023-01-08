use xf::num::{ivec2::IVec2, irect::{ir, IRect}};

use crate::consts::P16;

use super::entity::{Entity, next_entity_id, UpdateData, DrawData};




pub struct PlayerSpawner {
    id: usize,
    pub pos: IVec2,
    expired: bool,
}

impl PlayerSpawner {
    pub fn new(pos: IVec2) -> Self {
        Self {
            id: next_entity_id(),
            pos,
            expired: false,
        }
    }
}

impl Entity for PlayerSpawner {
    fn id(&self) -> usize { self.id }

    fn bounds(&self) -> IRect {
        ir(self.pos, P16)
    }

    fn update(&mut self, d: &mut UpdateData) {
        if self.expired == false {
            d.player.pos = self.pos.as_fvec2();
        }

        self.expired = true;
    }

    fn draw(&self, _: &DrawData) {
    }
}