use bevy::{
    prelude::*,
    text::TextLayoutInfo,
    ui::{widget::TextFlags, ContentSize, FocusPolicy},
};

use super::{
    box_events::EntityClickedEvent, box_resource::CurrentEntity, box_states::EntityActiveState,
};

pub fn check_entity_components(
    entity: Res<CurrentEntity>,
    style_query: Query<&Style>,
    bg_query: Query<&BackgroundColor>,
    border_query: Query<&BorderColor>,
    radius_query: Query<&BorderRadius>,
    focus_query: Query<&FocusPolicy>,
    transform_query: Query<&Transform>,
    global_transform_query: Query<&GlobalTransform>,
    visibility_query: Query<&Visibility>,
    inherited_visibility_query: Query<&InheritedVisibility>,
    view_visibility_query: Query<&ViewVisibility>,
    z_index_query: Query<&ZIndex>,
    text_layout_query: Query<&TextLayoutInfo>,
    text_flags_query: Query<&TextFlags>,
    content_size_query: Query<&ContentSize>,
) {
    let entity = entity.current; // Get the entity ID

    println!("Checking components for entity: {:?}", entity);

    if let Ok(_) = style_query.get(entity.unwrap()) {
        println!("  - Style");
    }
    if let Ok(_) = bg_query.get(entity.unwrap()) {
        println!("  - BackgroundColor");
    }
    if let Ok(_) = border_query.get(entity.unwrap()) {
        println!("  - BorderColor");
    }
    if let Ok(_) = radius_query.get(entity.unwrap()) {
        println!("  - BorderRadius");
    }
    if let Ok(_) = focus_query.get(entity.unwrap()) {
        println!("  - FocusPolicy");
    }
    if let Ok(_) = transform_query.get(entity.unwrap()) {
        println!("  - Transform");
    }
    if let Ok(_) = global_transform_query.get(entity.unwrap()) {
        println!("  - GlobalTransform");
    }
    if let Ok(_) = visibility_query.get(entity.unwrap()) {
        println!("  - Visibility");
    }
    if let Ok(_) = inherited_visibility_query.get(entity.unwrap()) {
        println!("  - InheritedVisibility");
    }
    if let Ok(_) = view_visibility_query.get(entity.unwrap()) {
        println!("  - ViewVisibility");
    }
    if let Ok(_) = z_index_query.get(entity.unwrap()) {
        println!("  - ZIndex");
    }
    if let Ok(_) = text_layout_query.get(entity.unwrap()) {
        println!("  - TextLayoutInfo");
    }
    if let Ok(_) = text_flags_query.get(entity.unwrap()) {
        println!("  - TextFlags");
    }
    if let Ok(_) = content_size_query.get(entity.unwrap()) {
        println!("  - ContentSize");
    }
}
pub fn toggle_entity_active_state(
    mut event_reader: EventReader<EntityClickedEvent>,
    state: Res<State<EntityActiveState>>,
    mut entity: ResMut<CurrentEntity>,
    mut next_state: ResMut<NextState<EntityActiveState>>,
) {
    for event in event_reader.read() {
        match state.get() {
            EntityActiveState::Active => next_state.set(EntityActiveState::Inactive),
            EntityActiveState::Inactive => next_state.set(EntityActiveState::Active),
        }
        entity.current = Some(event.entity);
    }
}
