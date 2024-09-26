use bevy::{
    prelude::*,
    text::TextLayoutInfo,
    ui::{widget::TextFlags, ContentSize, FocusPolicy},
};

use crate::utils::{colors::SubmergeColors, font_size::SubmergeText};

#[derive(Component, Debug, Clone, Default)]
pub struct SliderComponentsStyle {
    pub slide_height: Val,
    pub slide_width: Val,
    pub slide_border_color: BorderColor,
    pub slide_border_radius: BorderRadius,
    pub slide_border: UiRect,
    pub slide_background_color: BackgroundColor,

    //handle styles
    pub handle_height: Val,
    pub handle_width: Val,
    pub handle_border_color: BorderColor,
    pub handle_border: UiRect,
    pub handle_border_radius: BorderRadius,
    pub handle_background_color: BackgroundColor,
}

#[derive(Component, Debug, Clone)]
pub struct InputComponentStyle {
    pub color: Color,
    pub font_size: f32,
    pub caret_width: Val,
    pub caret_height: Val,
    pub caret_color: Color,
}

#[derive(Component, Debug, Clone, Default)]
pub struct ToggleComponentStyle {
    pub active_color: Color,
    pub border_radius: BorderRadius,
    pub width: Val,
    pub height: Val,
    pub indicator_color: Color,
    pub indicator_active_color: Color,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct ToggleStyleBundle {
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,

    pub component_style: ToggleComponentStyle, //impl parser for this

    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// The image of the node
    pub image: UiImage,
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct SliderStyleBundle {
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,

    pub component_style: SliderComponentsStyle, //impl parser for this

    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// The image of the node
    pub image: UiImage,
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

#[derive(Bundle, Clone, Debug)]
pub struct InputStyleBundle {
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,

    pub component_style: InputComponentStyle,

    // pub component_style: SliderComponentsStyle, //impl parser for this
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// The image of the node
    pub image: UiImage,
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Default for InputStyleBundle {
    fn default() -> Self {
        Self {
            style: Style {
                height: Val::Px(40.),
                width: Val::Px(100.),
                max_width: Val::Px(100.),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                overflow: Overflow::clip_x(),
                ..Default::default()
            },
            component_style: default(),
            interaction: default(),
            focus_policy: default(),
            border_color: SubmergeColors::color(SubmergeColors::GRAY400).into(),
            border_radius: default(),
            image: default(),
            background_color: BackgroundColor(SubmergeColors::color(SubmergeColors::GRAY100)),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            z_index: default(),
        }
    }
}

impl Default for InputComponentStyle {
    fn default() -> Self {
        Self {
            font_size: SubmergeText::text(SubmergeText::LG),
            color: SubmergeColors::color(SubmergeColors::BLACK).into(),
            caret_width: Val::Px(3.0),
            caret_height: Val::Px(30.0),
            caret_color: SubmergeColors::color(SubmergeColors::GRAY400),
        }
    }
}
