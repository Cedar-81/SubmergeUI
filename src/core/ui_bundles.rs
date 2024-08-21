use bevy::{prelude::*, text::BreakLineOn};

use super::{
    style_bundles::{ButtonStyleBundle, ContainerStyleBundle, TextStyleBundle},
    ui_components::UiComponent,
};

pub trait WithChildren {
    fn child(self, child_id: &str) -> Self;

    fn children<'a>(self, child_ids: impl IntoIterator<Item = &'a str>) -> Self;
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
