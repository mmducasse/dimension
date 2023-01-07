use xf::num::ivec2::IVec2;

use crate::common::update_data::UpdateData;

use super::entity::Entity;



pub struct Entities {
    all: Vec<Box<dyn Entity>>,
}

impl Entities {
    pub fn new() -> Self {
        Self { all: vec![] }
    }

    pub fn add(&mut self, e: Box<dyn Entity>) {
        self.all.push(e);
    }

    pub fn update(&mut self, d: &UpdateData) {
        for e in &mut self.all {
            e.update(d);
        }
    }

    pub fn draw(&self, org: IVec2) {
        for e in &self.all {
            e.draw(org);
        }
    }
}