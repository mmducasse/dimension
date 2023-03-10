use xf::gl::anim::{Animator, Animation};

use crate::{common::dir_h::DirH, consts::P16, row_2_l};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimKey {
    Idle(DirH),
    Run(DirH),
    JumpUp(DirH),
    JumpDown(DirH),
    Dash(DirH),
    WallSlide(DirH),
}

pub fn animator() -> Animator<AnimKey> {
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
        JumpUp(dir) => row_2_l!(dir, 1, DUR, i2(2, 2)),
        JumpDown(dir) => row_2_l!(dir, 1, DUR, i2(0, 2)),
        Dash(dir) => row_2_l!(dir, 1, DUR, i2(0, 4)),
        WallSlide(dir) => row_2_l!(dir, 1, DUR, i2(0, 6)),
    }
}