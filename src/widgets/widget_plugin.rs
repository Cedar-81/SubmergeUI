use std::time::Duration;

use bevy::{
    app::{App, Plugin, Update},
    prelude::IntoSystemConfigs,
    time::{Timer, TimerMode},
};

use super::{
    widget_components::SpawnWidgetComponent,
    widget_systems::{
        spawn_input_text_widget_components, spawn_slider_widget_components, update_slider_position,
        CaretBlinkTimerConfig,
    },
};

pub struct SubmergeWidgets;

impl Plugin for SubmergeWidgets {
    fn build(&self, app: &mut App) {
        app.insert_resource(CaretBlinkTimerConfig {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
        })
        .init_resource::<SpawnWidgetComponent>()
        .add_systems(
            Update,
            (spawn_slider_widget_components, update_slider_position).chain(),
        )
        .add_systems(Update, spawn_input_text_widget_components);
    }
}
