use bevy::{
    ecs::{entity, query},
    gizmos::config,
    input::{
        keyboard::{Key, KeyboardInput},
        mouse::{MouseScrollUnit, MouseWheel},
        ButtonState,
    },
    prelude::*,
    reflect::List,
    text::TextLayoutInfo,
    transform::{commands, components},
};
use bevy_mod_picking::{
    events::{Click, Drag, DragEnd, Out, Over, Pointer},
    prelude::On,
    PickableBundle,
};

use crate::{
    core::{
        style_bundles::{ContainerStyleBundle, TextStyleBundle},
        ui_bundles::SContainerBundle,
        ui_components::{Draggable, Dragging},
    },
    utils::colors::SubmergeColors,
    widgets::widget_components::Observing,
};

use super::{
    checkbox::{Checkbox, CheckboxIndicator, CheckboxIndicatorBundle},
    input::{Input, InputCaret, InputCaretBundle, InputInnerContainer, InputText, InputTextBundle},
    selector::{self, Selector, SelectorType},
    slider::{Handle, Slide, SlideBundle, Slider},
    style_bundles::{
        CheckboxComponentStyle, InputComponentStyle, InputStyleBundle, SliderComponentsStyle,
        SliderStyleBundle, ToggleComponentStyle,
    },
    toggle::{self, Toggle, ToggleIndicator, ToggleIndicatorBundle},
    widget_components::{Hovering, SpawnWidgetComponent},
};

pub fn spawn_slider_widget_components(
    mut commands: Commands,
    query: Query<(Entity, &SliderComponentsStyle, &Slider)>,
    mut active_widget: ResMut<SpawnWidgetComponent>,
) {
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
                .id();

            //add to parent widget
            commands
                .entity(entity)
                .push_children(&[slide_entity, handle_entity])
                .insert(PickableBundle::default())
                .insert(On::<Pointer<Drag>>::run(move |mut commands: Commands| {
                    commands.entity(entity).insert(Observing);
                }))
                .insert(On::<Pointer<DragEnd>>::run(
                    move |mut commands: Commands| {
                        commands.entity(entity).remove::<Observing>();
                    },
                ));

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
            let container = commands
                .spawn(SContainerBundle::new(
                    "input_inner_container",
                    ContainerStyleBundle {
                        style: Style {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            // border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        // border_color: BorderColor(Color::BLACK),
                        ..default()
                    },
                ))
                .insert(InputInnerContainer)
                .insert(Interaction::default())
                .id();

            // Initialize placeholder if children don't exist
            let style = TextStyle {
                font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
                font_size: input_text_style.font_size,
                color: input_text_style.color,
            };

            let input_text = commands
                .spawn(
                    InputTextBundle::from_section(input.placeholder.clone(), style).with_no_wrap(),
                )
                .id();
            commands.entity(container).add_child(input_text);

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
            commands.entity(container).add_child(caret);
            commands
                .entity(entity)
                .add_child(container)
                .insert(PickableBundle::default())
                .insert(On::<Pointer<Over>>::run(move |mut commands: Commands| {
                    commands.entity(container).insert(Hovering);
                }))
                .insert(On::<Pointer<Out>>::run(move |mut commands: Commands| {
                    commands.entity(container).remove::<Hovering>();
                }))
                .insert(On::<Pointer<Click>>::run(
                    move |mut click_event: EventWriter<ClickedEntityEvent>| {
                        click_event.send(ClickedEntityEvent(entity));
                    },
                ));

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
    mut input_query: Query<(Entity, &mut Input, &Children), With<Input>>,
    mut input_container_query: Query<&mut Style, (With<InputInnerContainer>, Without<InputCaret>)>,
    mut text_query: Query<&mut Text>,
    mut caret_query: Query<&mut Style, With<InputCaret>>,
    mut key_events: EventReader<KeyboardInput>,
    time: Res<Time>,
    mut config: ResMut<CaretBlinkTimerConfig>,
    children_query: Query<&Children>,
    mut show: Local<bool>,
    mut currently_observing: Local<Option<Entity>>,
    mut click_event: EventReader<ClickedEntityEvent>,
) {
    // Handle caret blinking
    config.timer.tick(time.delta());
    if config.timer.finished() {
        *show = !*show;
    }

    // Handle observing clicked entities
    for ev in click_event.read() {
        *currently_observing = Some(ev.0);
    }

    // Check if there is currently an observed input entity
    if let Some(observing) = *currently_observing {
        if let Ok((_entity, mut input, children)) = input_query.get_mut(observing) {
            // Locate the text and caret entities from children
            let mut input_text_entity = None;
            let mut caret_entity = None;
            // let mut container_style = None;

            for container in children.iter() {
                if let Ok(container_children) = children_query.get(*container) {
                    for child in container_children.iter() {
                        if let Ok(_) = text_query.get(*child) {
                            input_text_entity = Some(*child);
                        } else if let Ok(_) = caret_query.get(*child) {
                            caret_entity = Some(*child);
                        }
                    }
                }
            }

            // Update input text content
            if let Some(input_text_entity) = input_text_entity {
                if let Ok(mut text) = text_query.get_mut(input_text_entity) {
                    for event in key_events.read() {
                        if event.state == ButtonState::Released {
                            continue;
                        }

                        // Handle backspace
                        if event.key_code == KeyCode::Backspace
                            && event.state == ButtonState::Pressed
                        {
                            input.content.pop();
                        }

                        // Handle character input
                        if let Key::Character(character) = &event.logical_key {
                            input.content.push_str(character.as_str());
                        }

                        // Handle space input
                        if event.key_code == KeyCode::Space && event.state == ButtonState::Pressed {
                            input.content.push(' ');
                        }

                        // Update text content
                        text.sections[0].value = input.content.clone();

                        // for container in children.iter() {
                        //     if let Ok(mut style) = input_container_query.get_mut(*container) {
                        //         println!("in here");
                        //         style.justify_content = JustifyContent::End;
                        //     }
                        // }
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
}

pub fn handle_input_scroll(
    mut evr_scroll: EventReader<MouseWheel>,
    mut input_query: Query<
        &Style,
        (
            With<Input>,
            Without<InputInnerContainer>,
            Without<InputCaret>,
        ),
    >,
    mut style_query_with_hovering: Query<
        (&mut Style, &Children),
        (With<InputInnerContainer>, With<Hovering>),
    >,
    text_query: Query<&TextLayoutInfo, With<InputText>>,
    caret_query: Query<&Style, (With<InputCaret>, Without<InputInnerContainer>)>,
    mut parent_width: Local<Option<f32>>,
    mut child_width: Local<Option<f32>>,
    mut caret_width: Local<Option<f32>>,
    mut text_width: Local<Option<f32>>,
) {
    for parent_style in input_query.iter_mut() {
        let parent_width_val = if let Val::Px(width) = parent_style.width {
            width
        } else {
            0.0
        };

        *parent_width = Some(parent_width_val);
    }

    // Get the input inner container width from the text logical width + the caret width.
    // This is calculated because the width of the input inner container is set to auto
    for (_, children) in style_query_with_hovering.iter() {
        for child in children {
            if let Ok(text_layout_info) = text_query.get(*child) {
                *text_width = Some(text_layout_info.logical_size[0]);
            }

            if let Ok(caret_style) = caret_query.get(*child) {
                if let Val::Px(caret_width_val) = caret_style.width {
                    *caret_width = Some(caret_width_val);
                }
            }

            if let (Some(text_width), Some(caret_width)) = (*text_width, *caret_width) {
                *child_width = Some(text_width + caret_width);
            }
        }
    }

    // Handle scroll events and scroll
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Pixel => {
                if let (Some(child_width_val), Some(parent_width_val)) =
                    (*child_width, *parent_width)
                {
                    let max_scroll_left = -(child_width_val - parent_width_val).max(0.0);
                    for (mut child_style, _) in style_query_with_hovering.iter_mut() {
                        if let Val::Auto = child_style.left {
                            child_style.left = Val::Px(0.0);
                        }

                        if let Val::Px(left) = child_style.left {
                            let new_left = (left + ev.x).clamp(max_scroll_left - 10., 0.0);
                            child_style.left = Val::Px(new_left);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

#[derive(Event)]
pub struct ClickedEntityEvent(Entity);

#[derive(Event)]
pub struct ParentChildClickedEntityEvent {
    pub parent: Entity,
    pub child: Option<Entity>,
}

pub fn spawn_selector(
    mut commands: Commands,
    selector_query: Query<(Entity, &Children), With<Selector>>,
    mut active_widget: ResMut<SpawnWidgetComponent>,
) {
    //this system adds events to spawned selectors
    for (entity, children) in selector_query.iter() {
        let parent_entity = entity.clone();
        if active_widget.entity_component_map.get(&entity).is_none() {
            commands
                .entity(entity)
                .insert(PickableBundle::default())
                .insert(On::<Pointer<Click>>::run(
                    move |mut pc_click_event: EventWriter<ParentChildClickedEntityEvent>| {
                        pc_click_event.send(ParentChildClickedEntityEvent {
                            parent: parent_entity,
                            child: None,
                        });
                    },
                ));

            // Add selector to entity_component_map
            for child in children {
                let parent_entity = entity.clone();
                let child_entity = child.clone();

                commands
                    .entity(*child)
                    .insert(PickableBundle::default())
                    .insert(On::<
                    Pointer<Click>,
                >::run(
                    move |mut pc_click_event: EventWriter<ParentChildClickedEntityEvent>,
                          mut click_event: EventWriter<ClickedEntityEvent>| {
                        pc_click_event.send(ParentChildClickedEntityEvent {
                            parent: parent_entity,
                            child: Some(child_entity),
                        });

                        click_event.send(ClickedEntityEvent(child_entity));
                    },
                ));
                active_widget
                    .entity_component_map
                    .entry(entity)
                    .or_insert_with(Vec::new)
                    .push(*child);
            }
        }
    }
}

pub fn handle_selector_children(
    mut selector_query: Query<(&mut Selector, &Children)>,
    mut click_event: EventReader<ParentChildClickedEntityEvent>,
    mut toggle_query: Query<&mut Toggle>,
    mut checkbox_query: Query<&mut Checkbox>,
) {
    for ev in click_event.read() {
        if let Ok((mut selector, children)) = selector_query.get_mut(ev.parent) {
            match selector.s_type {
                SelectorType::Radio => {
                    // Clear the active list and add the clicked entity
                    if let Some(child) = ev.child {
                        selector.active.clear();
                        selector.active.push(child);
                    }
                }
                SelectorType::Checkbox => {
                    if let Some(child) = ev.child {
                        // If the entity is already active, remove it
                        if selector.active.contains(&child) {
                            selector.active.retain(|&e| e != child);
                        } else {
                            // Otherwise, add the entity to the active list
                            selector.active.push(child);
                        }
                    }
                }
            }

            for child in children.iter() {
                if let Ok(mut toggle) = toggle_query.get_mut(*child) {
                    toggle.active = selector.active.contains(child);
                }
                if let Ok(mut checkbox) = checkbox_query.get_mut(*child) {
                    checkbox.active = selector.active.contains(child);
                }
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

            commands
                .entity(entity)
                .add_child(toggle_indicator)
                .insert(PickableBundle::default())
                .insert(On::<Pointer<Click>>::run(
                    move |mut click_event: EventWriter<ClickedEntityEvent>| {
                        click_event.send(ClickedEntityEvent(entity));
                    },
                ));

            active_widget
                .entity_component_map
                .insert(entity, vec![toggle_indicator]);
        }
    }
}

pub fn update_toggle_widget(
    mut commands: Commands,
    mut toggle_query: Query<(
        Entity,
        &mut Style,
        &mut BackgroundColor,
        &ToggleComponentStyle,
        &mut Toggle,
    )>,
    mut toggle_indicator: Query<
        (Entity, &mut BackgroundColor),
        (With<ToggleIndicator>, Without<Toggle>),
    >,
    mut prev_background_color: Local<Option<BackgroundColor>>,
    mut prev_indicator_background_color: Local<Option<BackgroundColor>>,
    parent_query: Query<&Parent>,
    mut click_event: EventReader<ClickedEntityEvent>,
) {
    // Update the toggle's appearance based on the `show` state
    // todo: Update Toggle Component show property
    for ev in click_event.read() {
        if let Ok((_, _, _, _, mut toggle)) = toggle_query.get_mut(ev.0) {
            toggle.active = !toggle.active;
        }
    }

    for (entity, mut toggle_style, mut toggle_background_color, toggle_component_style, toggle) in
        toggle_query.iter_mut()
    {
        if prev_background_color.is_none() {
            *prev_background_color = Some(*toggle_background_color);
        }

        if toggle.active == true {
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

pub fn spawn_checkbox_widget_components(
    mut commands: Commands,
    query: Query<(Entity, &CheckboxComponentStyle), With<Checkbox>>,
    mut active_widget: ResMut<SpawnWidgetComponent>,
) {
    for (entity, checkbox_style) in query.iter() {
        // println!("here");
        if let None = active_widget.entity_component_map.get(&entity) {
            let checkbox_indicator_style = ContainerStyleBundle {
                style: Style {
                    height: checkbox_style.height,
                    width: checkbox_style.width,
                    display: Display::None,
                    ..default()
                },
                border_radius: checkbox_style.border_radius,
                background_color: BackgroundColor(checkbox_style.active_color),
                ..default()
            };

            let checkbox_indicator = commands
                .spawn(CheckboxIndicatorBundle::new(
                    "checkbox_indicator",
                    checkbox_indicator_style,
                ))
                .insert(Interaction::default())
                .id();

            commands
                .entity(entity)
                // .insert(Observing)
                .add_child(checkbox_indicator)
                .insert(PickableBundle::default())
                .insert(On::<Pointer<Click>>::run(
                    move |mut click_event: EventWriter<ClickedEntityEvent>| {
                        click_event.send(ClickedEntityEvent(entity));
                    },
                ));

            active_widget
                .entity_component_map
                .insert(entity, vec![checkbox_indicator]);
        }
    }
}

pub fn update_checkbox_widget(
    mut commands: Commands,
    mut checkbox_query: Query<(
        Entity,
        &mut BorderColor,
        &CheckboxComponentStyle,
        &mut Checkbox,
    )>,
    mut checkbox_indicator: Query<
        (Entity, &mut Style),
        (With<CheckboxIndicator>, Without<Checkbox>),
    >,
    mut prev_background_color: Local<Option<BorderColor>>,
    mut click_event: EventReader<ClickedEntityEvent>,
    parent_query: Query<&Parent>,
) {
    for ev in click_event.read() {
        // Update the toggle's appearance based on the `show` state
        if let Ok((_, _, _, mut checkbox)) = checkbox_query.get_mut(ev.0) {
            checkbox.active = !checkbox.active;
        }
    }

    for (entity, mut border_color, toggle_component_style, checkbox) in checkbox_query.iter_mut() {
        if prev_background_color.is_none() {
            *prev_background_color = Some(*border_color);
        }
        if checkbox.active == true {
            *border_color = BorderColor(toggle_component_style.active_color);
            // Update the ToggleIndicator's color when active
            for (indicator_entity, mut indicator_style) in checkbox_indicator.iter_mut() {
                if let Ok(parent_entity) = parent_query.get(indicator_entity) {
                    if parent_entity.get() == entity {
                        indicator_style.display = Display::Block;
                    }
                }
            }
        } else {
            // Reset to previous background color when inactive
            if let Some(prev_bkg_color) = *prev_background_color {
                *border_color = prev_bkg_color;
            }
            // Reset the ToggleIndicator's color when inactive
            for (indicator_entity, mut indicator_style) in checkbox_indicator.iter_mut() {
                if let Ok(parent_entity) = parent_query.get(indicator_entity) {
                    if parent_entity.get() == entity {
                        indicator_style.display = Display::None;
                    }
                }
            }
        }

        // Remove Observing component after update
        commands.entity(entity).remove::<Observing>();
    }
}
