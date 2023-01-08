
use xf::{
    num::{ivec2::{IVec2, i2}, 
    irect::{ir, IRect}}, 
    gl::anim::{Animator, Animation}
};

use crate::{ 
    graphics::{textures::TextureId, buffer::buffer_mut}, 
    consts::P16, 
    data::item::ItemType, 
    row, 
    row_l,
};

use super::{
    entity::{Entity, next_entity_id, UpdateData, DrawData}, 
    player::player::Player
};


const SIZE_P16: IVec2 = i2(1, 2);
const RADIUS: i32 = 4;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    Closed(ItemType),
    Opening,
    Open,
}

pub struct Gate {
    id: usize,
    pub pos: IVec2,
    key: ItemType,
    animator: Animator<State>,
}

impl Gate {
    pub fn new(pos: IVec2, key: ItemType) -> Self {
        Self {
            id: next_entity_id(),
            pos,
            key,
            animator: animator(key),
        }
    }
}

impl Entity for Gate {
    fn id(&self) -> usize { self.id }

    fn bounds(&self) -> IRect {
        ir(self.pos, SIZE_P16 * P16)
    }

    fn collides(&self) -> bool {
        self.animator.curr_key != State::Open
    }

    fn update(&mut self, d: &mut UpdateData) {
        match self.animator.curr_key {
            State::Closed(_) => {
                if Player::has_item(self.key) &&
                self.bounds().expand(RADIUS).intersection(d.player.bounds()).is_some()
                {
                    self.animator.set_key(State::Opening, false);
                }
            },
            State::Opening => if self.animator.is_done() {
                self.animator.set_key(State::Open, false);
            },
            State::Open => {},
        }

        self.animator.update();
    }

    fn draw(&self, d: &DrawData) {
        if self.animator.curr_key != State::Open {
            let texture = TextureId::Gates.texture();
            self.animator.draw(&texture, self.pos - d.org, buffer_mut());
        }
    }
}

fn animator(key: ItemType) -> Animator<State> {
    Animator::new(
        State::Closed(key), 
        SIZE_P16 * P16, 
        map_fn
    )
}

fn map_fn(key: State) -> &'static dyn Animation {

    use State::*;
    const DUR: f32 = 0.10;

    match key {
        Closed(ItemType::KeyRed) => row_l!(1, DUR, i2(0, 0)),
        Closed(ItemType::KeyGreen) => row_l!(1, DUR, i2(1, 0)),
        Closed(ItemType::KeyBlue) => row_l!(1, DUR, i2(2, 0)),
        Opening => row!(8, DUR, i2(0, 1)),
        _ => row_l!(1, DUR, i2(0, 0)),
    }
}