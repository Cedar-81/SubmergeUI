use bevy::app::{App, Plugin, Update};

use super::ui_systems::resolve_ui_hierarchy;

pub struct SubmergeUi;

impl Plugin for SubmergeUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resolve_ui_hierarchy);
    }
}
