use crate::{
    level::level::Level, 
    entities::entities::Entities, 
    data::scene_state::SceneState
};



pub struct PlayerUpdateData<'a> {
    pub level: &'a Level,
    pub entities: &'a Entities,
    pub scene_state: SceneState,
}