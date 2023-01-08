
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransitionState {
    Start(usize),
    Play,
    End(usize),
}

impl TransitionState {
    pub const TRANS_FRAMES: usize = 20;

    pub fn is_play(self) -> bool {
        matches!(self, TransitionState::Play)
    }

    pub fn needs_world_switch(self) -> bool {
        matches!(self, TransitionState::End(0))
    }

    pub fn update(&mut self, force_end: bool) {
        use TransitionState::*;

        *self = match *self {
            Start(0) => Play,
            End(0) => Start(Self::TRANS_FRAMES),
            Play => if force_end { End(Self::TRANS_FRAMES) } else { Play },
            Start(x) => Start(x - 1),
            End(x) => End(x - 1),
        }
    }

    pub fn x_scale(self) -> f32 {
        use TransitionState::*;

        let total = Self::TRANS_FRAMES as f32;

        match self {
            Start(x) => (Self::TRANS_FRAMES - x) as f32 / total,
            Play => 1.0,
            End(x) => 1.0 - ((Self::TRANS_FRAMES - x) as f32 / total),
        }
    }
}