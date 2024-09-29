use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct SpawnWidgetComponent {
    pub entity_component_map: HashMap<Entity, Vec<Entity>>,
}

#[derive(Component, Debug)]
pub struct Observing;

#[derive(Component, Debug)]
pub struct Hovering;
