use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, AppExtStates, IntoSystemConfigs},
};

use super::{
    box_events::{detect_click_interaction, EntityClickedEvent},
    box_resource::CurrentEntity,
    box_states::EntityActiveState,
    box_systems::{check_entity_components, toggle_entity_active_state},
};

pub struct SubmergeBox;

impl Plugin for SubmergeBox {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityClickedEvent>()
            .init_resource::<CurrentEntity>()
            .init_state::<EntityActiveState>()
            .add_systems(
                Update,
                (detect_click_interaction, toggle_entity_active_state),
            )
            .add_systems(
                Update,
                check_entity_components.run_if(in_state(EntityActiveState::Active)),
            );
    }
}
