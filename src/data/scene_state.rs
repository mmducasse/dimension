

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SceneState {
    Day,
    //TransitionToNight,
    Night,
    //TransitionToDay,
}

impl SceneState {
    pub fn reversed(self) -> bool {
        self == SceneState::Night
    }
}