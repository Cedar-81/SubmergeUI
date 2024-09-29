use bevy::{prelude::*, utils::HashMap};

use super::ui_components::{Draggable, Dragging, SubmergeId, UiComponent};

#[derive(Resource, Default, Debug)]
pub struct ChildrenResource {
    pub parent_children_map: HashMap<Entity, Vec<Entity>>,
    pub entity_id_map: HashMap<String, Entity>,
}

pub fn resolve_ui_hierarchy(mut commands: Commands, query: Query<(Entity, &UiComponent)>) {
    let mut id_map = std::collections::HashMap::new();

    //handles entering entity & id to id_map
    for (entity, ui) in query.iter() {
        id_map.insert(ui.id.clone(), entity);
    }

    //Handles adding child entity commands to their parent commands
    for (entity, ui) in query.iter() {
        for child_id in &ui.children {
            if let Some(&child_entity) = id_map.get(child_id) {
                commands.entity(entity).push_children(&[child_entity]);
            }
        }
    }
}

pub fn resolve_children_resource_ui_hierarchy(
    mut commands: Commands,
    children_resource: Res<ChildrenResource>,
) {
    for (parent_entity, children_entities) in children_resource.parent_children_map.iter() {
        commands
            .entity(*parent_entity)
            .push_children(&children_entities[..]);
    }
}

/// This system removes the SubmergeId component from an entity and inserts the UiComponent component so that id's can be properly resolved.
pub fn insert_ui_component(
    mut commands: Commands,
    query: Query<(Entity, &SubmergeId)>,
    mut children_resource: ResMut<ChildrenResource>,
) {
    // println!("insert_called");
    for (entity, submerg_id) in query.iter() {
        let id = submerg_id.0.clone();

        children_resource.entity_id_map.insert(id.clone(), entity);

        commands.entity(entity).remove::<SubmergeId>();

        let ui_component = commands.spawn(UiComponent { id, ..default() }).id();

        commands.entity(entity).add_child(ui_component);
    }
}
