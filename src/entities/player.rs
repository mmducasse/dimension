use std::rc::Rc;

use macroquad::prelude::{KeyCode, is_key_pressed};
use xf::{
    num::{fvec2::FVec2, ivec2::{IVec2, i2}, irect::{IRect, ir}}, 
    gl::{anim::{Animator, Animation}, texture::Texture}
};

use crate::{
    common::{dir_h::DirH, update_data::UpdateData}, 
    consts::{P16, GRAVITY}, 
    row_2_l, 
    graphics::{
        textures::TextureId, 
        buffer::buffer_mut
    }, 
    io::controller::get_dir_h_down, 
    systems::collision::collide
};

use super::entity::Entity;




const RUN_SPEED: f32 = 2.0;
const JUMP_VEL_Y: f32 = -5.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Run,
    Jump,
}

impl State {
    pub fn to_anim_key(self, dir: DirH) -> AnimKey {
        use State::*;

        match self {
            Idle => AnimKey::Idle(dir),
            Run => AnimKey::Run(dir),
            Jump => AnimKey::Jump(dir),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AnimKey {
    Idle(DirH),
    Run(DirH),
    Jump(DirH),
}

pub struct Player {
    pub pos: FVec2,
    pub vel: FVec2,
    pub dir: DirH,
    state: State,
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

    pub fn jump(&mut self) {
        self.pos.y -= 1.0;
        self.vel.y = JUMP_VEL_Y;
        self.state = State::Jump;
    }
}

impl Entity for Player {

    fn update(&mut self, d: &UpdateData) {
        if let Some(dir) = get_dir_h_down() {
            self.dir = dir;
            if self.state == State::Idle {
                self.state = State::Run;
            }
            self.vel.x = dir.unit().x as f32 * RUN_SPEED;
        } else {
            if self.state == State::Run {
                self.state = State::Idle;
            }
            self.vel.x = 0.0;
        }

        if is_key_pressed(KeyCode::Z) &&
           self.state != State::Jump {
            self.jump();
        }

        self.pos += self.vel;
        self.vel += GRAVITY;

        let deflection = collide(self.bounds(), d.level.get_colliders(), Some(d.level.bounds()));
        self.pos += deflection.as_fvec2();

        if self.vel.y > 0.0 && deflection.y < 0 {
            self.vel.y = 0.0;
            if self.state == State::Jump {
                self.state = State::Idle;
            }
        }

        self.animator.set_key(self.state.to_anim_key(self.dir), false);
        self.animator.update();
    }

    fn draw(&self, org: IVec2) {
        self.animator.draw(&self.texture, self.pos.as_ivec2() +  org, buffer_mut());
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
    const DUR: f32 = 0.08;

    match key {
        // Normal
        Idle(dir) => row_2_l!(dir, 1, DUR, i2(0, 0)),
        Run(dir) => row_2_l!(dir, 4, DUR, i2(0, 2)),
        Jump(dir) => row_2_l!(dir, 1, DUR, i2(2, 2)),
    }
}