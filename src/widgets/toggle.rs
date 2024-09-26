use bevy::{
    prelude::{Bundle, Component},
    ui::Node,
};

use crate::core::{style_bundles::ContainerStyleBundle, ui_components::UiComponent};

use super::style_bundles::{ToggleComponentStyle, ToggleStyleBundle};

#[derive(Component, Debug, Clone, Default)]
pub struct Toggle {
    pub active: bool,
}

#[derive(Component, Debug, Clone, Default)]
pub struct ToggleIndicator;

#[derive(Bundle, Debug, Clone, Default)]
pub struct ToggleBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Toggle,
    pub style: ToggleStyleBundle,
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct ToggleIndicatorBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: ToggleIndicator,
    pub style: ContainerStyleBundle,
}

impl ToggleBundle {
    pub fn new(id: &str, style: ToggleStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            tag: Toggle { active: false },
            style,
            ..Default::default()
        }
    }
}

impl ToggleIndicatorBundle {
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
