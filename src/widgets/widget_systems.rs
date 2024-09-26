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
    core::{
        style_bundles::ContainerStyleBundle,
        ui_components::{Draggable, Dragging},
    },
    utils::colors::SubmergeColors,
    widgets::widget_components::Observing,
};

use super::{
    input::{Input, InputCaret, InputCaretBundle, InputTextBundle},
    selector::{self, Selector, SelectorType},
    slider::{Handle, Slide, SlideBundle, Slider},
    style_bundles::{
        InputComponentStyle, InputStyleBundle, SliderComponentsStyle, SliderStyleBundle,
        ToggleComponentStyle,
    },
    toggle::{Toggle, ToggleIndicator, ToggleIndicatorBundle},
    widget_components::SpawnWidgetComponent,
};

pub fn spawn_slider_widget_components(
    mut commands: Commands,
    query: Query<(Entity, &SliderComponentsStyle, &Slider)>,
    mut active_widget: ResMut<SpawnWidgetComponent>,
) {
    // commands.spawn(SlideBundle::new("slide", slide_style));
    // commands.spawn(HandleBundle::new("handle", handle_style));

    for (entity, slider_style, _slider) in query.iter() {
        if let None = active_widget.entity_component_map.get(&entity) {
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
            active_widget
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

    // Track cursor movement
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

pub fn spawn_input_widget_components(
    mut commands: Commands,
    mut query: Query<(Entity, &InputComponentStyle, &mut Input)>,
    asset_server: Res<AssetServer>,
    // mut prev_input_text: Local<Option<Entity>>,
    // mut prev_caret: Local<Option<Entity>>,
    mut active_widget: ResMut<SpawnWidgetComponent>,
) {
    for (entity, input_text_style, input) in query.iter_mut() {
        if active_widget.entity_component_map.get(&entity).is_none() {
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
            commands.entity(entity).add_child(input_text);

            // Add input_text to entity_component_map
            active_widget
                .entity_component_map
                .insert(entity, vec![input_text]);

            // Spawn the caret if it doesn't exist
            let caret_style = InputStyleBundle {
                style: Style {
                    height: input_text_style.caret_height,
                    width: input_text_style.caret_width,
                    display: Display::Block, // Visible by default
                    ..Default::default()
                },
                background_color: BackgroundColor(input_text_style.caret_color),
                border_radius: BorderRadius::all(Val::Px(5.)),
                ..Default::default()
            };

            let caret = commands
                .spawn(InputCaretBundle::new("input_caret", caret_style))
                .id();
            commands.entity(entity).add_child(caret);

            // Add caret to entity_component_map
            active_widget
                .entity_component_map
                .entry(entity)
                .or_insert_with(Vec::new)
                .push(caret);
        }
    }
}

pub fn update_input_components_system(
    mut commands: Commands,
    mut input_query: Query<Entity, (With<Input>, Without<Observing>)>,
    mut observing_query: Query<(Entity, &mut Input, &Children), With<Observing>>,
    interaction_query: Query<(Entity, &Interaction)>,
    mut text_query: Query<&mut Text>,
    mut caret_query: Query<&mut Style, With<InputCaret>>,
    mut key_events: EventReader<KeyboardInput>,
    time: Res<Time>,
    mut config: ResMut<CaretBlinkTimerConfig>,
    parent_query: Query<&Parent>,
    cursor_button_input: Res<ButtonInput<MouseButton>>,
    mut show: Local<bool>,
    mut currently_observing: Local<Option<Entity>>,
) {
    // Handle caret blinking
    config.timer.tick(time.delta());
    if config.timer.finished() {
        *show = !*show;
    }

    for (entity, interaction) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                // Detect if the left mouse button was just pressed
                if cursor_button_input.just_pressed(MouseButton::Left) {
                    // Initialize the current entity's parent as the pressed one
                    if let Ok(input_entity) = input_query.get_mut(entity) {
                        commands.entity(input_entity).insert(Observing);
                        *currently_observing = Some(input_entity);
                        break;
                    }

                    let mut current_entity_parent = parent_query.get(entity).ok();
                    println!("current_entity_parent: {:?}", current_entity_parent);

                    // Loop to check each parent until a matching entity is found or no more parents exist
                    while let Some(parent) = current_entity_parent {
                        println!("passed here");
                        if let Ok(input_entity) = input_query.get_mut(parent.get()) {
                            println!("in here");
                            // Insert the Observing component into the parent entity
                            commands.entity(input_entity).insert(Observing);
                            *currently_observing = Some(input_entity);
                            println!("inserted observing");
                            break;
                        } else {
                            // Move up the hierarchy to the next parent
                            current_entity_parent = parent_query.get(parent.get()).ok();
                            println!("didn't find parent to insert ");
                        }
                    }
                }
            }
            _ => (),
        }

        //remove observing from entities not being observed
        if let Ok(input_entity) = observing_query.get(entity) {
            if currently_observing.is_some() && currently_observing.unwrap() != input_entity.0 {
                commands.entity(entity).remove::<Observing>();
            }
        }
    }

    for (_entity, mut input, children) in observing_query.iter_mut() {
        // Locate the text and caret entities from children
        let mut input_text_entity = None;
        let mut caret_entity = None;

        for child in children.iter() {
            if let Ok(_) = text_query.get(*child) {
                input_text_entity = Some(*child);
            } else if let Ok(_) = caret_query.get(*child) {
                caret_entity = Some(*child);
            }
        }

        // Update input text content
        if let Some(input_text_entity) = input_text_entity {
            if let Ok(mut text) = text_query.get_mut(input_text_entity) {
                for event in key_events.read() {
                    if event.state == ButtonState::Released {
                        continue;
                    }

                    if event.state == ButtonState::Pressed && event.key_code == KeyCode::Backspace {
                        input.content.pop();
                    }

                    if let Key::Character(character) = &event.logical_key {
                        input.content.push_str(character.as_str());
                    }

                    // Update text content
                    text.sections[0].value = input.content.clone();
                }
            }
        }

        // Update caret visibility using display property
        if let Some(caret_entity) = caret_entity {
            if let Ok(mut caret_style) = caret_query.get_mut(caret_entity) {
                if *show {
                    caret_style.display = Display::None;
                } else {
                    caret_style.display = Display::Block;
                }
            }
        }
    }
}

#[derive(Event)]
pub struct ClickedEntityEvent(Entity);

pub fn handle_click(
    mut query: Query<(Entity, &Interaction)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut click_event: EventWriter<ClickedEntityEvent>,
) {
    for (entity, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // When the mouse is pressed, store the initial click position
                if buttons.just_pressed(MouseButton::Left) {
                    click_event.send(ClickedEntityEvent(entity));
                }
            }
            _ => (),
        }
    }
}

pub fn handle_selector_children(
    mut selector: Query<&mut Selector>,
    mut click_event: EventReader<ClickedEntityEvent>,
    parent_query: Query<&Parent>,
) {
    for selector in selector.iter() {
        println!("active items: {:?}", selector.active);
    }
    for ev in click_event.read() {
        // Start with the clicked entity's parent
        let mut current_parent = parent_query.get(ev.0).ok();

        while let Some(parent_entity) = current_parent {
            if let Ok(mut selector) = selector.get_mut(parent_entity.get()) {
                // Handle different selector types
                match selector.s_type {
                    SelectorType::Radio => {
                        // Clear the active list and add the clicked entity
                        selector.active.clear();
                        selector.active.push(ev.0);
                    }
                    SelectorType::Checkbox => {
                        // If the entity is already active, remove it
                        if selector.active.contains(&ev.0) {
                            selector.active.retain(|&e| e != ev.0);
                        } else {
                            // Otherwise, add the entity to the active list
                            selector.active.push(ev.0);
                        }
                    }
                }
                break; // Exit the loop once the parent with a Selector is found
            } else {
                // Move to the next parent up the hierarchy
                current_parent = parent_query.get(parent_entity.get()).ok();
            }
        }
    }
}

pub fn spawn_toggle_widget_components(
    mut commands: Commands,
    query: Query<(Entity, &ToggleComponentStyle), With<Toggle>>,
    mut active_widget: ResMut<SpawnWidgetComponent>,
) {
    for (entity, toggle_style) in query.iter() {
        // println!("here");
        if let None = active_widget.entity_component_map.get(&entity) {
            let toggle_indicator_style = ContainerStyleBundle {
                style: Style {
                    height: toggle_style.height,
                    width: toggle_style.width,
                    ..default()
                },
                border_radius: toggle_style.border_radius,
                background_color: BackgroundColor(toggle_style.indicator_color),
                ..default()
            };

            let toggle_indicator = commands
                .spawn(ToggleIndicatorBundle::new(
                    "toggle_indicator",
                    toggle_indicator_style,
                ))
                .insert(Interaction::default())
                .id();

            commands.entity(entity).add_child(toggle_indicator);

            active_widget
                .entity_component_map
                .insert(entity, vec![toggle_indicator]);
        }
    }
}

pub fn update_toggle_widget(
    mut commands: Commands,
    mut observing: Query<
        (
            Entity,
            &mut Style,
            &mut BackgroundColor,
            &ToggleComponentStyle,
        ),
        (With<Toggle>, With<Observing>),
    >,
    mut toggle_indicator: Query<
        (Entity, &mut BackgroundColor),
        (With<ToggleIndicator>, Without<Toggle>),
    >,
    toggle_click_query: Query<(Entity, &Interaction), (With<Toggle>, Without<ToggleIndicator>)>,
    toggle_indicator_click_query: Query<
        (Entity, &Interaction),
        (With<ToggleIndicator>, Without<Toggle>),
    >,
    parent: Query<&Parent>,
    cursor_button_input: Res<ButtonInput<MouseButton>>, // Fix: Use `Input<MouseButton>` instead of `ButtonInput<MouseButton>`
    mut show: Local<Option<bool>>,
    mut prev_background_color: Local<Option<BackgroundColor>>,
    mut prev_indicator_background_color: Local<Option<BackgroundColor>>,
    parent_query: Query<&Parent>,
) {
    if show.is_none() {
        *show = Some(false);
    }

    // Handle interaction on the parent (Toggle) first
    for (entity, interaction) in toggle_click_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                // Detect if left mouse button was just pressed
                if cursor_button_input.just_pressed(MouseButton::Left) {
                    // Insert Observing component to mark this toggle as active
                    commands.entity(entity).insert(Observing);

                    if let Some(show_val) = show.as_mut() {
                        *show_val = !*show_val;
                    }
                }
            }
            _ => (),
        }
    }

    // Handle interaction on the child (ToggleIndicator)
    for (entity, interaction) in toggle_indicator_click_query.iter() {
        // Get the parent of the ToggleIndicator entity
        if let Ok(parent_entity) = parent.get(entity) {
            match *interaction {
                Interaction::Pressed => {
                    // Detect if left mouse button was just pressed
                    if cursor_button_input.just_pressed(MouseButton::Left) {
                        // Insert Observing component on the parent Toggle
                        commands.entity(parent_entity.get()).insert(Observing);

                        if let Some(show_val) = show.as_mut() {
                            *show_val = !*show_val;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    // Update the toggle's appearance based on the `show` state
    for (entity, mut toggle_style, mut toggle_background_color, toggle_component_style) in
        observing.iter_mut()
    {
        if prev_background_color.is_none() {
            *prev_background_color = Some(*toggle_background_color);
        }

        if show.unwrap() == true {
            toggle_style.justify_content = JustifyContent::End;
            *toggle_background_color = BackgroundColor(toggle_component_style.active_color);

            // Update the ToggleIndicator's color when active
            for (indicator_entity, mut background_color) in toggle_indicator.iter_mut() {
                if prev_indicator_background_color.is_none() {
                    *prev_indicator_background_color = Some(*background_color);
                }
                if let Ok(parent_entity) = parent_query.get(indicator_entity) {
                    if parent_entity.get() == entity {
                        *background_color =
                            BackgroundColor(toggle_component_style.indicator_active_color);
                    }
                }
            }
        } else {
            toggle_style.justify_content = JustifyContent::Start;

            // Reset to previous background color when inactive
            if let Some(prev_bkg_color) = *prev_background_color {
                *toggle_background_color = prev_bkg_color;
            }

            // Reset the ToggleIndicator's color when inactive
            for (indicator_entity, mut background_color) in toggle_indicator.iter_mut() {
                if let Ok(parent_entity) = parent_query.get(indicator_entity) {
                    if parent_entity.get() == entity {
                        if let Some(prev_bkg_color) = *prev_indicator_background_color {
                            *background_color = prev_bkg_color;
                        }
                    }
                }
            }
        }

        // Remove Observing component after update
        commands.entity(entity).remove::<Observing>();
    }
}
