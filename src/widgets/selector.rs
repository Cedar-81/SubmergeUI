use bevy::{
    prelude::{Bundle, Component, Entity},
    ui::Node,
};

use crate::core::{
    style_bundles::ContainerStyleBundle, ui_bundles::WithChildren, ui_components::UiComponent,
};

#[derive(Debug, Clone, Default)]
pub enum SelectorType {
    Radio,
    #[default]
    Checkbox,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Selector {
    pub active: Vec<Entity>,
    pub children: Vec<Entity>,
    pub s_type: SelectorType,
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct SelectorBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Selector,
    pub style: ContainerStyleBundle,
}

impl SelectorBundle {
    pub fn new(id: &str, style: ContainerStyleBundle, s_type: SelectorType) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            tag: Selector {
                s_type,
                ..Default::default()
            },
            style,
            ..Default::default()
        }
    }

    pub fn get_active(&self) {
        println!("Active: {:?}", self.tag.active);
    }
}

impl WithChildren for SelectorBundle {
    fn child(mut self, child_id: &str) -> Self {
        self.ui_component.children.push(child_id.to_string());
        self
    }

    fn children<'a>(mut self, child_ids: impl IntoIterator<Item = &'a str>) -> Self {
        for id in child_ids {
            self.ui_component.children.push(id.to_string());
        }
        self
    }
}
