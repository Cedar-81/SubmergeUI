use bevy::{
    ecs::{entity, query},
    gizmos::config,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
    reflect::List,
    transform::{commands, components},
};

use crate::{
    core::ui_components::{Draggable, Dragging},
    utils::colors::SubmergeColors,
    widgets::widget_components::Observing,
};

use super::{
    input::{Input, InputCaretBundle, InputTextBundle},
    slider::{Handle, Slide, SlideBundle, Slider},
    style_bundles::{
        InputComponentStyle, InputStyleBundle, SliderComponentsStyle, SliderStyleBundle,
    },
    widget_components::SpawnWidgetComponent,
};

pub fn spawn_slider_widget_components(
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

#[derive(Resource)]
pub struct CaretBlinkTimerConfig {
    pub timer: Timer,
}

pub fn spawn_input_text_widget_components(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &InputComponentStyle,
        &mut Input,
        Option<&mut Children>,
    )>,
    mut text_query: Query<&mut Text>,
    mut key_events: EventReader<KeyboardInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut prev_input_text: Local<Option<Entity>>,
    mut prev_caret: Local<Option<Entity>>,
    mut show: Local<bool>,
    mut config: ResMut<CaretBlinkTimerConfig>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        // Toggle show every 500 milliseconds
        *show = !*show;
    }

    for (entity, input_text_style, mut input, maybe_children) in query.iter_mut() {
        // Handle the case where there are no children initially
        let mut input_text_entity = *prev_input_text;
        let caret_entity = *prev_caret;

        if maybe_children.is_none() {
            // Initialize placeholder if children don't exist
            let style = TextStyle {
                font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
                font_size: input_text_style.font_size,
                color: input_text_style.color,
            };

            let input_text = commands
                .spawn(InputTextBundle::from_section(
                    input.placeholder.clone(),
                    style,
                ))
                .id();
            *prev_input_text = Some(input_text);
            commands.entity(entity).add_child(input_text);
            input_text_entity = Some(input_text);
        }

        // Handle input text entity update
        if let Some(input_text_entity) = input_text_entity {
            // Safely attempt to get the `Text` component
            if let Ok(mut text) = text_query.get_mut(input_text_entity) {
                for event in key_events.read() {
                    // Only check for characters when the key is pressed
                    if event.state == ButtonState::Released {
                        continue;
                    }

                    if event.state == ButtonState::Pressed && event.key_code == KeyCode::Backspace {
                        input.content.pop();
                    }

                    if let Key::Character(character) = &event.logical_key {
                        input.content.push_str(character.as_str());
                    }

                    // Update text content in place
                    text.sections[0].value = input.content.clone();
                }
            }
        }

        // Despawn caret if the timer is finished and previous caret entity exists
        if let Some(caret) = *prev_caret {
            if *show == true {
                commands.entity(caret).despawn();
                *prev_caret = None;
            }
        }

        // Spawn new caret if show is false and no caret exists
        if *show == false && caret_entity.is_none() {
            let caret_style = InputStyleBundle {
                style: Style {
                    height: input_text_style.caret_height,
                    width: input_text_style.caret_width,
                    ..Default::default()
                },
                background_color: BackgroundColor(input_text_style.caret_color),
                border_radius: BorderRadius::all(Val::Px(5.)),
                ..Default::default()
            };

            let caret = commands
                .spawn(InputCaretBundle::new("input_caret", caret_style))
                .id();
            *prev_caret = Some(caret);
            commands.entity(entity).add_child(caret);
        }
    }
}
