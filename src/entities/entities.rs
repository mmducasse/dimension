
use super::entity::{Entity, UpdateData, DrawData};



pub struct Entities {
    all: Vec<Box<dyn Entity>>,
}

impl Entities {
    pub fn new() -> Self {
        Self { all: vec![] }
    }

    pub fn all(&self) -> &Vec<Box<dyn Entity>> {
        &self.all
    }

    pub fn add(&mut self, e: Box<dyn Entity>) {
        self.all.push(e);
    }

    pub fn update(&mut self, d: &mut UpdateData) {
        for e in &mut self.all {
            e.update(d);
        }
    }

    pub fn draw(&self, d: &DrawData) {
        for e in &self.all {
            e.draw(d);
        }
    }
}