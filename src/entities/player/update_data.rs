use crate::{level::level::Level, entities::entities::Entities};



pub struct PlayerUpdateData<'a> {
    pub level: &'a Level,
    pub entities: &'a Entities,
}