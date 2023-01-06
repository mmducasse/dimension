use xf::num::ivec2::IVec2;



pub trait Entity {
    fn update(&mut self);
    fn draw(&self, org: IVec2);
}