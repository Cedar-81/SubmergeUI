---

# SubmergeUI

**SubmergeUI** is a simple, flexible, and modular Rust UI library built on top of Bevy’s ECS framework. Designed for ease of use, it allows developers to create and style UI components using a Tailwind-like syntax. The goal of SubmergeUI is to make UI development straightforward and scalable in Rust.

## Features

- **Nested UI components**: Create hierarchical UI layouts with ease by referencing components by ID, without the need for complex child nesting.
- **Tailwind-like styling**: Apply styles directly in a familiar, space-separated format for quick UI customization.
- **Scalable design**: Designed for both small and large projects, allowing for modular and customizable UI layouts.
- **Custom utilities**: Submerge comes with some custom styling utilities like predefined colors, text sizes, border radius all based on tailwind css presets.

## Getting Started

SubmergeUI provides an easy way to build and style nested UI components in Bevy. With minimal setup, you can start creating and styling components using intuitive syntax.

### Installation

To add SubmergeUI to your project, include the following in your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.11"
submergeui = "0.1"  # replace with the actual version when released

```

### Example: Creating and Styling a Button

Here’s how you can quickly set up a button with custom styles using SubmergeUI:

```rust
use bevy::prelude::*;
use submergeui::{SubmergeUIPlugin, SButtonBundle, ButtonStyleBundle};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SubmergeUIPlugin)
        .add_startup_system(setup_ui.system())
        .run();
}

fn setup_ui(mut commands: Commands) {
    // Creating a button with custom styles using Tailwind-like syntax
    let button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px border_color-white rounded-50% bg-red-100",
    );

    // Spawning a button with an ID for easy reference
    commands.spawn(SButtonBundle::new("play_button", button_style));
}

```

### Creating Nested Components

SubmergeUI makes creating nested UI structures simple, using IDs to link components together:

```rust
fn setup_nested_ui(mut commands: Commands) {

	let text_style = TextStyle {
        font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
        font_size: SubmergeText::text(SubmergeText::LG),
        color: SubmergeColors::color(SubmergeColors::BLACK).into(),
    };

    // Create two buttons that will be children of the container
    let play_button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px border_color-white rounded-50% bg-red-100",
    );
    
    let another_play_button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px border_color-white rounded-50% bg-red-100",
    );
    
    commands.spawn(SButtonBundle::new("play_button", play_button_style).child("play_button_txt"));
    commands.spawn(
        SButtonBundle::new("another_play_button", another_play_button_style).child("another_play_button_txt"),
    );
    
    commands
        .spawn(STextBundle::from_section("First Button", text_style.clone()).id("play_button_txt"));
    commands.spawn(
        STextBundle::from_section("Another Button", text_style).id("another_play_button_txt"),
    );
}

```

### Mixing and Matching with SubmergeUi styling and Bevy styling

SubmergeUI gives you the freedom of using both Submerge’s styling and Bevy’s standard styling to manipulate your UI([full code here](https://github.com/Cedar-81/SubmergeUI/blob/main/examples/button.rs)):

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let a = NORMAL_BUTTON.into();
    let s_button_style: ButtonStyleBundle = ButtonStyleBundle {
        style: Style {
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,

            // vertically center child text
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        border_radius: BorderRadius::all(SubmergeBR::radius(SubmergeBR::FULL)),
        background_color: SubmergeColors::color(SubmergeColors::RED100).into(),
        ..default()
    };

    let button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px border_color-white rounded-50% bg-red-100",
    );

    let text_style = TextStyle {
        font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
        font_size: SubmergeText::text(SubmergeText::LG),
        color: SubmergeColors::color(SubmergeColors::BLACK).into(),
    };

    let _container_style = ContainerStyleBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        SContainerBundle::new(
            "main_container",
            ContainerStyleBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        )
        .children(["play_button", "another_play_button"]),
    );

    commands.spawn(SButtonBundle::new("play_button", button_style).child("play_button_txt"));
    commands.spawn(
        SButtonBundle::new("another_play_button", s_button_style).child("another_play_button_txt"),
    );

    commands
        .spawn(STextBundle::from_section("First Button", text_style.clone()).id("play_button_txt"));
    commands.spawn(
        STextBundle::from_section("Another Button", text_style).id("another_play_button_txt"),
    );
}
```

### Styling with Tailwind-like Syntax

SubmergeUI allows you to style components easily with a Tailwind-inspired syntax. The style definitions are human-readable and allow quick customization without deep knowledge of Bevy's internal `Style` API:

```rust
let button_style = ButtonStyleBundle::apply_style(
    "border-2px padding-10px rounded-8px bg-yellow-200 text-center",
);

```

## Styling in SubmergeUI

The style definition uses a Tailwind-like system where each property or style is defined by a string and then applied to components. Here's a breakdown of the available style properties(check out out [pest](https://github.com/Cedar-81/SubmergeUI/blob/main/rules.pest) file for a more in-depth insight):

| **Style** | **Description** | **Example** |
| --- | --- | --- |
| `border_color` | Specifies the border color of an element. | `border_color-red` |
| `border_radius` | Defines the border radius for rounded corners. | `rounded-lg` |
| `background_color` | Sets the background color of an element. | `bg-blue-500` |
| `text_color` | Specifies the text color of an element. | `text_color-white` |
| `text_size` | Sets the size of the text. | `text-lg` |
| `interaction` | Specifies the interaction state of an element. | `interaction-hovered` |
| `focus_policy` | Defines how an element should respond to focus events. | `focus-none` |
| `image` | Specifies an image for the element. | `image-my_image` *(unimplemented)* |
| `transform` | Applies a transformation such as translation, rotation, or scaling. | `transform-r-45_0.5_0` |
| `global_transform` | Specifies a global transformation for the element. | `global_transform-my_global_transform` *(unimplemented)* |
| `visibility` | Controls the visibility of an element. | `visibility-hidden` |
| `inherited_visibility` | Indicates whether an element inherits visibility from its parent. | `inherited_visibility-true` |
| `view_visibility` | Controls the visibility of the view. | `view_visibility-visible` |
| `z_index` | Sets the stack order of an element. | `z-10` |
| `display` | Defines the display style of the element (e.g., flex, grid, none). | `display-flex` |
| `position` | Specifies the positioning type of an element (e.g., relative, absolute). | `position-absolute` |
| `overflow` | Controls how content that overflows the element's box is handled. | `overflow-clip` |
| `direction` | Defines the text direction (e.g., left-to-right, right-to-left). | `direction-rtl` |
| `left` | Sets the left position of an absolutely positioned element. | `left-10px` |
| `right` | Sets the right position of an absolutely positioned element. | `right-0px` |
| `top` | Sets the top position of an absolutely positioned element. | `top-20px` |
| `bottom` | Sets the bottom position of an absolutely positioned element. | `bottom-30px` |
| `width` | Sets the width of the element. | `width-100%` |
| `height` | Sets the height of the element. | `height-50vh` |
| `min_width` | Sets the minimum width of the element. | `min_width-300px` |
| `min_height` | Sets the minimum height of the element. | `min_height-200px` |
| `max_width` | Sets the maximum width of the element. | `max_width-500px` |
| `max_height` | Sets the maximum height of the element. | `max_height-400px` |
| `aspect_ratio` | Defines the aspect ratio of the element. | `aspect-16_9` |
| `align_items` | Sets the alignment of items within a flex or grid container. | `align_items-center` |
| `justify_items` | Sets the justification of items within a flex or grid container. | `justify_items-space-between` |
| `align_self` | Overrides the default alignment for a specific item. | `align_self-end` |
| `justify_self` | Overrides the default justification for a specific item. | `justify_self-center` |
| `align_content` | Aligns the content within a flex or grid container when there is extra space. | `align_content-around` |
| `justify_content` | Justifies the content within a flex or grid container. | `justify_content-evenly` |
| `margin` | Sets the margin around the element. | `margin-10px` |
| `padding` | Sets the padding within the element. | `padding-15px` |
| `border` | Sets the border size around the element. | `border-2px` |
| `flex_direction` | Defines the direction of flex items. | `flex_direction-row` |
| `flex_wrap` | Controls whether flex items should wrap onto multiple lines. | `flex_wrap-wrap` |
| `flex_grow` | Specifies how much a flex item can grow relative to the rest. | `flex_grow-1` |
| `flex_shrink` | Specifies how much a flex item can shrink relative to the rest. | `flex_shrink-0` |
| `flex_basis` | Defines the default size of a flex item before space is distributed. | `flex_basis-200px` |
| `row_gap` | Sets the gap between rows in a grid or flex layout. | `row_gap-20px` |
| `column_gap` | Sets the gap between columns in a grid or flex layout. | `column_gap-10px` |
| `grid_auto_flow` | Defines how auto-placed items in a grid layout are flowed. | `grid_auto_flow-col` *(unimplemented)* |
| `grid_template_rows` | Defines the row sizes in a grid layout. | `grid_template_rows-100px,200px` *(unimplemented)* |
| `grid_template_columns` | Defines the column sizes in a grid layout. | `grid_template_columns-50%,25%` *(unimplemented)* |
| `grid_auto_rows` | Sets the size of automatically placed rows. | `grid_auto_rows-100px,200px` *(unimplemented)* |
| `grid_auto_columns` | Sets the size of automatically placed columns. | `grid_auto_columns-50%,25%` *(unimplemented)* |
| `grid_row` | Places an item in a specific row of a grid layout. | `grid_row-1 2` *(unimplemented)* |
| `grid_column` | Places an item in a specific column of a grid layout. | `grid_column-1 2` *(unimplemented)* |

## Utilities

Submerge provides a set of utility enums that simplify the styling of UI components. These include `SubmergeText` for font sizes, `SubmergeColors` for color definitions, and `SubmergeBR` for border radius settings. Each of these utilities follows the conventions used in Tailwind CSS, making them intuitive to use for developers familiar with that framework.

### 1. SubmergeText

The `SubmergeText` enum allows you to easily specify font sizes in your UI. Each variant represents a standard font size with an associated `f32` value for size.

### Enum Definition

```rust
pub enum SubmergeText {
    XS { size: f32 },
    SM { size: f32 },
    BASE { size: f32 },
    LG { size: f32 },
    XL { size: f32 },
    XL2 { size: f32 },
    XL3 { size: f32 },
    XL4 { size: f32 },
    XL5 { size: f32 },
    XL6 { size: f32 },
    XL7 { size: f32 },
    XL8 { size: f32 },
    XL9 { size: f32 },
}

```

### Usage Example

```rust
let text_style = TextStyle {
    font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
    font_size: SubmergeText::text(SubmergeText::LG),
    color: SubmergeColors::color(SubmergeColors::BLACK).into(),
};

```

### 2. SubmergeColors

The `SubmergeColors` enum provides a range of color definitions based on the Tailwind CSS color palette. Each variant includes RGB values for the corresponding color.

### Enum Definition

```rust
#[derive(Debug, Component)]
pub enum SubmergeColors {
    BLACK { r: f32, g: f32, b: f32 },
    WHITE { r: f32, g: f32, b: f32 },
    SLATE50 { r: f32, g: f32, b: f32 },
    SLATE100 { r: f32, g: f32, b: f32 },
    SLATE200 { r: f32, g: f32, b: f32 },
    SLATE300 { r: f32, g: f32, b: f32 },
    SLATE400 { r: f32, g: f32, b: f32 },
    // ... additional colors as needed
}

```

### Usage Example

```rust
let s_button_style: ButtonStyleBundle = ButtonStyleBundle {
    style: Style {
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(15.0)),
        ..default()
    },
    border_color: BorderColor(Color::BLACK),
    border_radius: BorderRadius::all(SubmergeBR::radius(SubmergeBR::FULL)),
    background_color: SubmergeColors::color(SubmergeColors::RED100).into(),
    ..default()
};

```

### 3. SubmergeBR

The `SubmergeBR` enum is designed for defining border radii, also based on Tailwind CSS presets. It offers various radius options for easy styling.

### Enum Definition

```rust
#[derive(Debug)]
pub enum SubmergeBR {
    NONE { radius: f32 },
    SM { radius: f32 },
    DEFAULT { radius: f32 },
    MD { radius: f32 },
    LG { radius: f32 },
    XL { radius: f32 },
    XL2 { radius: f32 },
    XL3 { radius: f32 },
    FULL { radius: f32 },
}

```

### Usage Example

```rust
let button_style: ButtonStyleBundle = ButtonStyleBundle {
    border_radius: BorderRadius::all(SubmergeBR::radius(SubmergeBR::FULL)),
    // Other styles...
};

```

These utilities provide a flexible and straightforward way to handle common styling tasks in your UI components, promoting consistency and ease of use. For further details and examples, please refer to the documentation or the source code.

---

Feel free to adjust the wording or details to better fit your project's style!

## Documentation

For full API documentation, usage details, and more examples, refer to the [SubmergeUI documentation](https://docs.rs/submergeui) (coming soon).

## License

This project is licensed under the MIT License.

---

This version highlights the simplicity and modularity of SubmergeUI, making it an attractive option for developers looking for an easy-to-use, flexible UI system for Bevy projects. Let me know if you'd like any further adjustments!
