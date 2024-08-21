use bevy::prelude::*;

use crate::core::ui_components::UiComponent;

use super::style_bundles::SliderStyleBundle;

#[derive(Component, Debug, Clone, Default)]
pub struct Slider;

#[derive(Component, Debug, Clone, Default)]
pub struct Slide;

#[derive(Component, Debug, Clone, Default)]
pub struct Handle;

#[derive(Bundle, Debug, Clone, Default)]
pub struct SliderBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Slider,
    pub style: SliderStyleBundle,
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct SlideBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Slide,
    pub style: SliderStyleBundle,
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct HandleBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Handle,
    pub style: SliderStyleBundle,
}

impl SliderBundle {
    pub fn new(id: &str, style: SliderStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            style,
            ..Default::default()
        }
    }
    // pub fn add_event(&mut self, event: Interaction, handler: Fn) {}
}

impl SlideBundle {
    pub fn new(id: &str, style: SliderStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            style,
            ..Default::default()
        }
    }
    // pub fn add_event(&mut self, event: Interaction, handler: Fn) {}
}

impl HandleBundle {
    pub fn new(id: &str, style: SliderStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            style,
            ..Default::default()
        }
    }
    // pub fn add_event(&mut self, event: Interaction, handler: Fn) {}
}
