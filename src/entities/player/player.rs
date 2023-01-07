use std::rc::Rc;

use xf::{
    num::{fvec2::FVec2, ivec2::{IVec2, i2}, irect::{IRect, ir}}, 
    gl::{anim::Animator, texture::Texture}, time::timer::Timer
};

use crate::{
    common::{dir_h::DirH, update_data::UpdateData},
    graphics::{
        textures::TextureId, 
        buffer::buffer_mut
    }, entities::entity::Entity,
};

use super::{state::State, anim::{AnimKey, animator}};


pub struct Player {
    pub pos: FVec2,
    pub vel: FVec2,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) state_timer: Timer,
    texture: Rc<Texture>,
    animator: Animator<AnimKey>
}

impl Player {
    pub fn new(pos: IVec2) -> Self {
        Self {
            pos: pos.as_fvec2(),
            vel: FVec2::ZERO,
            dir: DirH::R,
            state: State::Idle,
            state_timer: Timer::new(0.0),
            texture: TextureId::Player.texture(),
            animator: animator(),
        }
    }

    pub fn bounds(&self) -> IRect {
        ir(
            self.pos.as_ivec2() + i2(2, 2), 
            i2(12, 14)
        )
    }
}

impl Entity for Player {

    fn update(&mut self, d: &UpdateData) {
        self.state_timer.update();
        self.state.update(self, d);

        self.animator.set_key(self.state.to_anim_key(self), false);
        self.animator.update();
    }

    fn draw(&self, org: IVec2) {
        self.animator.draw(&self.texture, self.pos.as_ivec2() - org, buffer_mut());
    }
}