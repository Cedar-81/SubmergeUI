use bevy::{
    ecs::entity,
    prelude::*,
    transform::{commands, components},
};

use crate::{
    core::ui_components::{Draggable, Dragging},
    widgets::widget_components::Observing,
};

use super::{
    slider::{Handle, Slide, SlideBundle, Slider},
    style_bundles::{SliderComponentsStyle, SliderStyleBundle},
    widget_components::SpawnWidgetComponent,
};

pub fn spawn_widget_components(
    mut commands: Commands,
    query: Query<(Entity, &SliderComponentsStyle, &Slider)>,
    mut acitve_widget: ResMut<SpawnWidgetComponent>,
) {
    // commands.spawn(SlideBundle::new("slide", slide_style));
    // commands.spawn(HandleBundle::new("handle", handle_style));

    for (entity, slider_style, _slider) in query.iter() {
        if let None = acitve_widget.entity_component_map.get(&entity) {
            //check if child_entity for widget has been spawned
            let slide_style = SliderStyleBundle {
                style: Style {
                    height: slider_style.slide_height,
                    width: slider_style.slide_width,
                    border: slider_style.slide_border,
                    ..default()
                },
                border_color: slider_style.slide_border_color,
                border_radius: slider_style.slide_border_radius,
                background_color: slider_style.slide_background_color,
                ..default()
            };

            let handle_style = SliderStyleBundle {
                style: Style {
                    height: slider_style.handle_height,
                    width: slider_style.handle_width,
                    max_width: slider_style.handle_width,
                    min_width: slider_style.handle_width,
                    border: slider_style.handle_border,
                    ..default()
                },
                border_color: slider_style.handle_border_color,
                border_radius: slider_style.handle_border_radius,
                background_color: slider_style.handle_background_color,
                ..default()
            };

            //spawn widget child entities
            let slide_entity = commands.spawn(SlideBundle::new("slide", slide_style)).id();
            let handle_entity = commands
                .spawn(SlideBundle::new("handle", handle_style))
                .insert(Draggable)
                .id();

            //add to parent widget
            commands
                .entity(entity)
                .push_children(&[slide_entity, handle_entity]);

            //update resource
            acitve_widget
                .entity_component_map
                .insert(entity, vec![slide_entity, handle_entity]);
        }
    }
}

pub fn update_slider_position(
    mut commands: Commands,
    slider: Query<(Entity, &Interaction), (With<Slider>, Without<Slide>)>,
    mut style_query: Query<&mut Style, (Without<Slider>, Without<Handle>)>,
    slide: Query<Entity, With<Slide>>,
    parent_query: Query<&Children, With<Slider>>,
    observing: Query<(Entity, &Style), (With<Observing>, With<Slider>, Without<Slide>)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    cursor_button_input: Res<ButtonInput<MouseButton>>,
    mut prev_cursor_position: Local<Option<Vec2>>, // To store the initial click position
) {
    // Track cursor movement

    for (entity, interaction) in slider.iter() {
        match *interaction {
            Interaction::Pressed => {
                // When the mouse is pressed, store the initial click position
                if cursor_button_input.just_pressed(MouseButton::Left) {
                    commands.entity(entity).insert(Observing);
                }
            }
            _ => {
                commands.entity(entity).remove::<Observing>();
            }
        }
    }

    for (entity, slider_style) in observing.iter() {
        for event in cursor_moved_events.read() {
            let mut value = 0.4_f32;
            if let Some(prev_pos) = *prev_cursor_position {
                let current_pos = event.position;
                if current_pos.x > prev_pos.x {
                    value = value;
                } else if current_pos.x < prev_pos.x {
                    value = -value;
                }
            }

            *prev_cursor_position = Some(event.position);

            if let Ok(children) = parent_query.get(entity) {
                for &child in children.iter() {
                    if let Ok(mut child_style) = style_query.get_mut(child) {
                        if let Val::Px(slider_width) = slider_style.width {
                            let _ = value.clamp(0.0, slider_width);
                            if let Val::Px(mut child_width) = child_style.width {
                                child_width = child_width + value;
                                child_style.width = Val::Px(child_width);
                            }
                        }
                    }
                }
            }
        }
    }
}
