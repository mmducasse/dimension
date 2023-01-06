use std::rc::Rc;

use xf::{
    num::{fvec2::FVec2, ivec2::IVec2}, 
    gl::{anim::{Animator, Animation}, texture::Texture, bitmap::Bitmap}
};

use crate::{
    common::dir_h::DirH, 
    consts::P16, 
    row_2_l, graphics::{textures::TextureId, buffer::buffer_mut}
};

use super::entity::Entity;

#[derive(Clone, Copy, PartialEq, Eq)]
enum AnimKey {
    Idle(DirH),
    Run(DirH),
}

pub struct Player {
    pub pos: FVec2,
    pub vel: FVec2,
    texture: Rc<Texture>,
    animator: Animator<AnimKey>
}

impl Player {
    pub fn new(pos: IVec2) -> Self {
        Self {
            pos: pos.as_fvec2(),
            vel: FVec2::ZERO,
            texture: TextureId::Player.texture(),
            animator: animator(),
        }
    }
}

impl Entity for Player {
    fn update(&mut self) {
        self.animator.update();
    }

    fn draw(&self, org: IVec2) {
        //self.animator.draw(&self.texture, org, buffer_mut());
        buffer_mut().draw_texture_full(&self.texture, self.pos.as_ivec2() + org);
    }
}

fn animator() -> Animator<AnimKey> {
    Animator::new(
        AnimKey::Idle(DirH::R), 
        P16, 
        map_fn
    )
}

fn map_fn(key: AnimKey) -> &'static dyn Animation {

    use AnimKey::*;
    const DUR: f32 = 0.125;

    match key {
        // Normal
        Idle(dir) => row_2_l!(dir, 1, DUR, i2(0, 0)),
        Run(dir) => row_2_l!(dir, 2, DUR, i2(1, 0)),
    }
}