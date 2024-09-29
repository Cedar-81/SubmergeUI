use std::time::Duration;

use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::IntoSystemConfigs,
    time::{Timer, TimerMode},
};
use bevy_mod_picking::DefaultPickingPlugins;

use super::{
    widget_components::SpawnWidgetComponent,
    widget_systems::{
        handle_input_scroll, handle_selector_children, spawn_checkbox_widget_components,
        spawn_input_widget_components, spawn_selector, spawn_slider_widget_components,
        spawn_toggle_widget_components, update_checkbox_widget, update_input_components_system,
        update_slider_position, update_toggle_widget, CaretBlinkTimerConfig, ClickedEntityEvent,
        ParentChildClickedEntityEvent,
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
        .add_event::<ParentChildClickedEntityEvent>()
        .add_plugins(DefaultPickingPlugins)
        // .add_systems(Update, handle_click)
        .add_systems(
            Update,
            (
                spawn_selector,
                spawn_slider_widget_components,
                spawn_input_widget_components,
                spawn_toggle_widget_components,
                spawn_checkbox_widget_components,
                update_toggle_widget,
                update_slider_position,
                update_input_components_system,
                update_checkbox_widget,
                handle_selector_children,
                handle_input_scroll,
            ),
        );
    }
}
