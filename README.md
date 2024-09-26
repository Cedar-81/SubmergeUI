# SubmergeUI

SubmergeUI is a Rust UI library built on top of Bevy's ECS framework. It provides a structured, modular, and flexible way to define UI components using a Tailwind-like approach for styles, offering a streamlined API for creating and managing UI elements without complex nesting.

## Features

- **ECS-based UI**: Components, resources, and systems integrate directly with Bevy’s ECS.
- **Tailwind-like styling**: Apply styles in a familiar, concise syntax like `"border-5px justify_content-center align_items-center"`.
- **Child linking**: Instead of deeply nested children, UI elements are linked by explicit IDs for clarity and simplicity.
- **Custom utilities**: Submerge comes with some custom styling utilities like predefined colors, text sizes, border radius, all based on tailwind css presets.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- Bevy added as a dependency to your project.

### Setup

To get started, add `bevy_submerge_ui` as a dependency to your project. You can do this by adding the crate from [crates.io](https://crates.io/crates/bevy_submerge_ui):

In your project's `Cargo.toml`, add:

```toml
[dependencies]
bevy = "0.14.1"
bevy_submerge_ui = "0.1.1"
```

### Usage

To use SubmergeUI in your Bevy project, simply add it as a plugin:

```rust
use bevy::prelude::*;
use bevy_submerge_ui::core::{ui_plugin::SubmergeUi, ui_bundles::{SButtonBundle, ButtonStyleBundle, STextBundle}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SubmergeUi)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
        font_size: SubmergeText::text(SubmergeText::LG),
        color: SubmergeColors::color(SubmergeColors::BLACK).into(),
    };

    let mut button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px rounded-50% bg-red-100",
    );

    commands.spawn(SButtonBundle::new("play_button", button_style).child("play_button_txt"));
    commands
        .spawn(STextBundle::from_section("First Button", text_style.id("play_button_txt")));
}
```

### Reactive Widgets (Radioactive Branch)

We are currently experimenting with building **reactive widgets** in the `radioactive` branch. This branch is in its **early stages**, so it is not production-ready. However, contributions and suggestions are welcome! Currently available widgets include:

- **Toggle**: A switch-like widget.
- **Selector**: A widget for radio button or checkbox groups.
- **Input**: A basic text input field.
- **Slider**: A slider to select numeric values.

### Upcoming Widgets

We are planning to add more widgets to the library, including:

- Checkbox
- Radio button
- Tooltips
- Progress indicator
- Dropdown

Feel free to explore the `radioactive` branch and contribute! To check it out:

```bash
git checkout radioactive
```

You can find the branch [here](https://github.com/Cedar-81/SubmergeUI/tree/radioactive).
