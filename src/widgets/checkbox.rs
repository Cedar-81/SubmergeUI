use bevy::{
    prelude::{Bundle, Component},
    ui::Node,
};

use crate::core::{style_bundles::ContainerStyleBundle, ui_components::UiComponent};

use super::style_bundles::{CheckboxComponentStyle, CheckboxStyleBundle};

#[derive(Component, Debug, Clone)]
pub struct Checkbox {
    pub active: bool,
}

#[derive(Component, Debug, Clone, Default)]
pub struct CheckboxIndicator;

#[derive(Bundle, Debug, Clone, Default)]
pub struct CheckboxBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Checkbox,
    pub style: CheckboxStyleBundle,
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct CheckboxIndicatorBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: CheckboxIndicator,
    pub style: ContainerStyleBundle,
}

impl CheckboxBundle {
    pub fn new(id: &str, style: CheckboxStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            tag: Checkbox { active: false },
            style,
            ..Default::default()
        }
    }
}

impl CheckboxIndicatorBundle {
    pub fn new(id: &str, style: ContainerStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            style,
            ..Default::default()
        }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self { active: false }
    }
}
