use bevy::{
    app::{App, Plugin, Update},
    prelude::{default, IntoSystemConfigs},
};

use super::ui_systems::{
    insert_ui_component, resolve_children_resource_ui_hierarchy, resolve_ui_hierarchy,
    ChildrenResource,
};

pub struct SubmergeUi;

impl Plugin for SubmergeUi {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChildrenResource { ..default() })
            .add_systems(
                Update,
                (
                    (insert_ui_component, resolve_children_resource_ui_hierarchy).chain(),
                    resolve_ui_hierarchy,
                ),
            );
    }
}
