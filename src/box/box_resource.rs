use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CurrentEntity {
    pub current: Option<Entity>,
}
