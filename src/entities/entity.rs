use xf::num::ivec2::IVec2;

use crate::common::update_data::UpdateData;



pub trait Entity {
    fn update(&mut self, d: &UpdateData);
    fn draw(&self, org: IVec2);
}