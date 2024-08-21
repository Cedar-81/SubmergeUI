use bevy::prelude::{Bundle, *};

use crate::core::ui_components::UiComponent;

use super::style_bundles::InputStyleBundle;

#[derive(Component, Debug, Clone, Default)]
pub struct Input;

#[derive(Component, Debug, Clone, Default)]
pub struct InputText;

#[derive(Bundle, Debug, Clone, Default)]
pub struct InputBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Input,
    pub style: InputStyleBundle,
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct InputTextBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: InputText,
    pub style: InputStyleBundle,
}
