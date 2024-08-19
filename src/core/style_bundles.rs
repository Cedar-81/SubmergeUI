use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use bevy::ui::widget::TextFlags;
use bevy::ui::{ContentSize, FocusPolicy, Interaction, Style};

use crate::core::parser::parser;

pub trait SubmergeStyleBundle {
    type Output;

    fn apply_style(style: &str) -> Self::Output;
}

#[derive(Bundle, Clone, Debug)]
pub struct ButtonStyleBundle {
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
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

impl Default for ButtonStyleBundle {
    fn default() -> Self {
        Self {
            // node: Default::default(),
            // button: Default::default(),
            style: Default::default(),
            interaction: Default::default(),
            focus_policy: FocusPolicy::Block,
            border_color: Default::default(),
            border_radius: Default::default(),
            image: Default::default(),
            background_color: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct ContainerStyleBundle {
    /// Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style, //Unstyled child warning can be change to Style component missing from entity
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The border radius of the node
    pub border_radius: BorderRadius,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
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

impl Default for ContainerStyleBundle {
    fn default() -> Self {
        Self {
            // node: Default::default(),
            // button: Default::default(),
            style: Default::default(),
            focus_policy: FocusPolicy::Block,
            border_color: Default::default(),
            border_radius: Default::default(),
            background_color: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct TextStyleBundle {
    // Styles which control the layout (size and position) of the node and its children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// Cached cosmic buffer for layout
    // pub buffer: CosmicBuffer,
    /// Text layout information
    pub text_layout_info: TextLayoutInfo,
    /// Text system flags
    pub text_flags: TextFlags,
    /// The calculated size based on the given image
    pub calculated_size: ContentSize,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `TextBundle`, use the properties of the [`Style`] component.
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
    /// The background color that will fill the containing node
    pub background_color: BackgroundColor,
}

impl SubmergeStyleBundle for ButtonStyleBundle {
    type Output = ButtonStyleBundle;

    fn apply_style(style: &str) -> Self::Output {
        let mut bundle = GeneralStyle::default();
        parser(style, &mut bundle);
        // println!("Applying ButtonStyleBundle: {:?}", bundle);
        bundle.into()
        // Handle button-specific logic here
    }
}

impl SubmergeStyleBundle for ContainerStyleBundle {
    type Output = ContainerStyleBundle;

    fn apply_style(style: &str) -> Self::Output {
        let mut bundle = GeneralStyle::default();
        parser(style, &mut bundle);
        // println!("Applying ContainerStyleBundle");
        // Handle container-specific logic here

        bundle.into()
    }
}

impl SubmergeStyleBundle for TextStyleBundle {
    type Output = TextStyleBundle;

    fn apply_style(style: &str) -> Self::Output {
        let mut bundle = GeneralStyle::default();
        parser(style, &mut bundle);
        // println!("Applying TextStyleBundle");

        bundle.into()
        // Handle text-specific logic here
    }
}

#[derive(Default, Debug)]
pub struct GeneralStyle {
    // Common fields
    pub style: Style,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility, //remove_ this is a private fild
    pub view_visibility: ViewVisibility,           //remove later field is private
    pub z_index: ZIndex,

    // Fields specific to ButtonStyleBundle
    pub interaction: Option<Interaction>, // Optional field
    pub image: Option<UiImage>,           // Optional field

    // Fields specific to TextStyleBundle
    pub text_layout_info: Option<TextLayoutInfo>, // Optional field
    pub text_flags: Option<TextFlags>,            // Optional field
    pub calculated_size: Option<ContentSize>,     // Optional field
    pub text_color: Option<Color>,
    pub text_size: Option<f32>,

    //basic_styles
    pub val: Val,
    pub bvec: Vec<Val>,
    pub color: Color,
    pub grid_auto_flow: GridAutoFlow,

    pub grid_template_rows: Vec<RepeatedGridTrack>,
    pub grid_template_columns: Vec<RepeatedGridTrack>,
    pub grid_auto_rows: Vec<GridTrack>,
    pub grid_auto_columns: Vec<GridTrack>,
    pub grid_placement: GridPlacement,
}

impl Into<ButtonStyleBundle> for GeneralStyle {
    fn into(self) -> ButtonStyleBundle {
        ButtonStyleBundle {
            style: self.style,
            interaction: self.interaction.unwrap_or_default(), // Provide a default value if None
            focus_policy: self.focus_policy,
            border_color: self.border_color,
            border_radius: self.border_radius,
            image: self.image.unwrap_or_default(), // Provide a default value if None
            background_color: self.background_color,
            transform: self.transform,
            global_transform: self.global_transform,
            visibility: self.visibility,
            inherited_visibility: self.inherited_visibility,
            view_visibility: self.view_visibility,
            z_index: self.z_index,
        }
    }
}

impl Into<ContainerStyleBundle> for GeneralStyle {
    fn into(self) -> ContainerStyleBundle {
        ContainerStyleBundle {
            style: self.style,
            focus_policy: self.focus_policy,
            border_color: self.border_color,
            border_radius: self.border_radius,
            background_color: self.background_color,
            transform: self.transform,
            global_transform: self.global_transform,
            visibility: self.visibility,
            inherited_visibility: self.inherited_visibility,
            view_visibility: self.view_visibility,
            z_index: self.z_index,
        }
    }
}

impl Into<TextStyleBundle> for GeneralStyle {
    fn into(self) -> TextStyleBundle {
        TextStyleBundle {
            style: self.style,
            text_layout_info: self.text_layout_info.unwrap_or_default(),
            text_flags: self.text_flags.unwrap_or_default(),
            calculated_size: self.calculated_size.unwrap_or_default(),
            focus_policy: self.focus_policy,
            transform: self.transform,
            global_transform: self.global_transform,
            visibility: self.visibility,
            inherited_visibility: self.inherited_visibility,
            view_visibility: self.view_visibility,
            z_index: self.z_index,
            background_color: self.background_color,
        }
    }
}
