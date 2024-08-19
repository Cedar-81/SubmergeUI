use bevy::prelude::{BuildChildren, Commands, Entity, Query};

use super::ui_components::UiComponent;

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
