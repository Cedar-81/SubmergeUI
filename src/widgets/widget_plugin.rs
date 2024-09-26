use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    prelude::IntoSystemConfigs,
    time::{Timer, TimerMode},
};

use super::{
    widget_components::SpawnWidgetComponent,
    widget_systems::{
        handle_click, handle_selector_children, spawn_input_widget_components,
        spawn_slider_widget_components, spawn_toggle_widget_components,
        update_input_components_system, update_slider_position, update_toggle_widget,
        CaretBlinkTimerConfig, ClickedEntityEvent,
    },
};

pub struct SubmergeWidgets;

impl Plugin for SubmergeWidgets {
    fn build(&self, app: &mut App) {
        app.insert_resource(CaretBlinkTimerConfig {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
        })
        .init_resource::<SpawnWidgetComponent>()
        .add_event::<ClickedEntityEvent>()
        .add_systems(Update, handle_click)
        .add_systems(
            Update,
            (spawn_slider_widget_components, update_slider_position).chain(),
        )
        .add_systems(
            Update,
            (
                spawn_input_widget_components,
                spawn_toggle_widget_components,
                handle_selector_children,
                update_toggle_widget,
                update_input_components_system,
            ),
        );
    }
}
