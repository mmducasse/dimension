use xf::num::ivec2::IVec2;

use crate::common::update_data::UpdateData;

static mut NEXT_ID: usize = 0;

pub fn next_entity_id() -> usize {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}


pub trait Entity {
    fn id(&self) -> usize;
    fn update(&mut self, d: &UpdateData);
    fn draw(&self, org: IVec2);
}