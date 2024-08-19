# SubmergeUI

SubmergeUI is a Rust UI library built on top of Bevy's ECS framework. It provides a structured, modular, and flexible way to define UI components using a Tailwind-like approach for styles, offering a streamlined API for creating and managing UI elements without complex nesting.

## Features

- **ECS-based UI**: Components, resources, and systems integrate directly with Bevy’s ECS.
- **Tailwind-like styling**: Apply styles in a familiar, concise syntax like `"border-5px justify_content-center align_items-center"`.
- **Child linking**: Instead of deeply nested children, UI elements are linked by explicit IDs for clarity and simplicity.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- Bevy added as a dependency to your project.

### Setup

To get started, add SubmergeUI as a dependency to your project. Since the library is still in development and not yet released as a crate, you can clone the repository from GitHub:

```bash
git clone <https://github.com/Cedar-81/SubmergeUI.git>
cd SubmergeUI

```

In your project's `Cargo.toml`, add SubmergeUI as a local dependency:

```toml
[dependencies]
bevy = "0.11"
submergeui = { path = "../SubmergeUI" }

```

### Usage

To use SubmergeUI in your Bevy project, simply add it as a plugin:

```rust
use bevy::prelude::*;
use submergeui::{SubmergeUIPlugin, ButtonBundle, ButtonStyleBundle};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SubmergeUIPlugin)
        .add_startup_system(setup_ui.system())
        .run();
}

fn setup_ui(mut commands: Commands) {
    let mut button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px rounded-50% bg-red-100",
    );

    commands.spawn_bundle(ButtonBundle::new("play_button", button_style));
}

```

### Examples

For more examples, please refer to the [`examples`](https://github.com/Cedar-81/SubmergeUI/tree/main/examples) folder in the repository.
