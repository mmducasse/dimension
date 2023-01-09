use std::rc::Rc;

use xf::{
    num::{fvec2::FVec2, ivec2::{IVec2, i2}, irect::{IRect, ir}}, 
    gl::{anim::Animator, texture::Texture}, time::timer::Timer
};

use crate::{
    common::{dir_h::DirH},
    graphics::{
        textures::TextureId, 
        buffer::buffer_mut
    }, 
    entities::entity::next_entity_id, 
    data::item::ItemType, global, 
    level::tile::TileType,
};

use super::{state::State, anim::{AnimKey, animator}, update_data::PlayerUpdateData};


pub struct Player {
    id: usize,
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
            id: next_entity_id(),
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

    pub fn anchor(&self) -> IVec2 {
        self.bounds().center() + i2(0, 4)
    }

    pub fn has_item(item: ItemType) -> bool {
        global::player_state::get().items[item as usize]
    }

    pub fn can_touch(tile_type: TileType) -> bool {
        match tile_type {
            TileType::Water => Self::has_item(ItemType::Snorkel),
            TileType::Spikes => false,
            _ => true,
        }
    }

    pub fn can_enter_door(&self) -> bool {
        matches!(self.state, State::Idle | State::Run)
    }

    pub fn update(&mut self, d: &PlayerUpdateData) {
        self.state_timer.update();
        self.state.update(self, d);

        let touch_tile = d.level.tile_type_at(self.anchor(), d.scene_state);
        if !Self::can_touch(touch_tile) {
            self.pos = global::player_state::get().last_checkpoint_pos;
            self.vel = FVec2::ZERO;
        }

        self.animator.set_key(self.state.to_anim_key(self), false);
        self.animator.update();
    }

    pub fn draw(&self, org: IVec2) {
        self.animator.draw(&self.texture, self.pos.as_ivec2() - org, buffer_mut());
    }
}