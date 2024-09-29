use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
    text::BreakLineOn,
};

use super::{
    style_bundles::{ButtonStyleBundle, ContainerStyleBundle, TextStyleBundle},
    ui_components::UiComponent,
    ui_systems::ChildrenResource,
};

pub trait WithChildren {
    fn child(self, child_id: &str) -> Self;

    fn children<'a>(self, child_ids: impl IntoIterator<Item = &'a str>) -> Self;
}

pub trait AddChildren {
    fn child(
        &mut self,
        child_id: &str,
        children_resource: &mut ResMut<ChildrenResource>,
    ) -> &mut Self;

    fn children<'a>(
        &mut self,
        child_ids: impl IntoIterator<Item = &'a str>,
        children_resource: &mut ResMut<ChildrenResource>,
    ) -> &mut Self;

    fn insert_id(
        &mut self,
        id: &str,
        children_resource: &mut ResMut<ChildrenResource>,
    ) -> &mut Self;
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct SButtonBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Button,
    pub style: ButtonStyleBundle,
}

impl SButtonBundle {
    pub fn new(id: &str, style: ButtonStyleBundle) -> Self {
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

impl AddChildren for EntityCommands<'_> {
    fn child(
        &mut self,
        child_id: &str,
        children_resource: &mut ResMut<ChildrenResource>,
    ) -> &mut Self {
        let parent = self.id();

        // Look up the entity associated with the given child_id
        if let Some(&child_entity) = children_resource.entity_id_map.get(child_id) {
            println!("in here: {:?}", child_entity);
            // Add the child entity to the parent's list of children
            children_resource
                .parent_children_map
                .entry(parent)
                .or_insert_with(Vec::new)
                .push(child_entity);
        } else {
            println!("Child ID '{}' not found in ChildrenResource. Make sure id for child has been inserted correctly.", child_id);
        }

        self
    }

    fn children<'a>(
        &mut self,
        child_ids: impl IntoIterator<Item = &'a str>,
        children_resource: &mut ResMut<ChildrenResource>,
    ) -> &mut Self {
        let parent = self.id();

        // Iterate over all child IDs and add them to the parent
        for child_id in child_ids {
            if let Some(&child_entity) = children_resource.entity_id_map.get(child_id) {
                children_resource
                    .parent_children_map
                    .entry(parent)
                    .or_insert_with(Vec::new)
                    .push(child_entity);
            } else {
                println!("Child ID '{}' not found in ChildrenResource. Make sure id for child has been inserted correctly.", child_id);
            }
        }

        self
    }

    fn insert_id(
        &mut self,
        id: &str,
        children_resource: &mut ResMut<ChildrenResource>,
    ) -> &mut Self {
        let entity = self.id();

        children_resource
            .entity_id_map
            .insert(id.to_string(), entity);

        self
    }
}

impl WithChildren for SButtonBundle {
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

impl WithChildren for Node {
    fn child(self, child_id: &str) -> Self {
        todo!()
    }

    fn children<'a>(self, child_ids: impl IntoIterator<Item = &'a str>) -> Self {
        todo!()
    }
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct SContainerBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    pub style: ContainerStyleBundle,
}

impl SContainerBundle {
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

impl WithChildren for SContainerBundle {
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

#[derive(Bundle, Debug, Default)]
pub struct STextBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    pub style: TextStyleBundle,
    /// Contains the text of the node
    pub text: Text,
}

impl STextBundle {
    pub fn new(id: &str, style: TextStyleBundle) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            style,
            ..Default::default()
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.ui_component.id = id.to_string();
        self
    }
}

// #[cfg(feature = "bevy_text")]
//standard bevy text implementations
impl STextBundle {
    /// Create a [`TextBundle`] from a single section.
    ///
    /// See [`Text::from_section`] for usage.
    pub fn from_section(value: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: Text::from_section(value, style),
            ..Default::default()
        }
    }

    /// Create a [`TextBundle`] from a list of sections.
    ///
    /// See [`Text::from_sections`] for usage.
    pub fn from_sections(sections: impl IntoIterator<Item = TextSection>) -> Self {
        Self {
            text: Text::from_sections(sections),
            ..Default::default()
        }
    }

    /// Returns this [`TextBundle`] with a new [`JustifyText`] on [`Text`].
    pub const fn with_text_justify(mut self, justify: JustifyText) -> Self {
        self.text.justify = justify;
        self
    }

    /// Returns this [`TextBundle`] with a new [`Style`].
    pub fn with_style(mut self, style: TextStyleBundle) -> Self {
        self.style = style;
        self
    }

    /// Returns this [`TextBundle`] with a new [`BackgroundColor`].
    pub const fn with_background_color(mut self, color: Color) -> Self {
        self.style.background_color = BackgroundColor(color);
        self
    }

    /// Returns this [`TextBundle`] with soft wrapping disabled.
    /// Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, will still occur.
    pub const fn with_no_wrap(mut self) -> Self {
        self.text.linebreak_behavior = BreakLineOn::NoWrap;
        self
    }
}

// #[cfg(feature = "bevy_text")]
impl<I> From<I> for STextBundle
where
    I: Into<TextSection>,
{
    fn from(value: I) -> Self {
        Self::from_sections(vec![value.into()])
    }
}
