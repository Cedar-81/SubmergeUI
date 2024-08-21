use bevy::{
    app::{App, Plugin, PostStartup, Update},
    prelude::IntoSystemConfigs,
};

use super::{
    widget_components::SpawnWidgetComponent,
    widget_systems::{spawn_widget_components, update_slider_position},
};

pub struct SubmergeWidgets;

impl Plugin for SubmergeWidgets {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnWidgetComponent>().add_systems(
            Update,
            (spawn_widget_components, update_slider_position).chain(),
        );
    }
}
