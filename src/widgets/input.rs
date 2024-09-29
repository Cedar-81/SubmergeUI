use bevy::{
    prelude::{Bundle, *},
    text::BreakLineOn,
};

use crate::core::{style_bundles::TextStyleBundle, ui_components::UiComponent};

use super::style_bundles::InputStyleBundle;

#[derive(Component, Debug, Clone, Default)]
pub struct Input {
    pub placeholder: String,
    pub content: String,
}

#[derive(Component, Debug, Clone, Default)]
pub struct InputText;

#[derive(Component, Debug, Clone, Default)]
pub struct InputInnerContainer;

#[derive(Component, Debug, Clone, Default)]
pub struct InputCaret;

#[derive(Bundle, Debug, Clone, Default)]
pub struct InputBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: Input,
    pub style: InputStyleBundle,
}

impl InputBundle {
    pub fn new(id: &str, style: InputStyleBundle, placeholder: &str) -> Self {
        Self {
            ui_component: UiComponent {
                id: id.to_string(),
                children: Vec::new(),
            },
            tag: Input {
                placeholder: placeholder.to_string(),
                ..default()
            },
            style,
            ..Default::default()
        }
    }
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct InputCaretBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: InputCaret,
    pub style: InputStyleBundle,
}

impl InputCaretBundle {
    pub fn new(id: &str, style: InputStyleBundle) -> Self {
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

#[derive(Bundle, Debug, Default)]
pub struct InputTextBundle {
    pub ui_component: UiComponent,
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub tag: InputText,
    pub text: Text,
    pub style: TextStyleBundle,
}

impl InputTextBundle {
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

impl<I> From<I> for InputTextBundle
where
    I: Into<TextSection>,
{
    fn from(value: I) -> Self {
        Self::from_sections(vec![value.into()])
    }
}
