//This example uses the new methods for inserting id and child/children components

use bevy::{
    app::{App, Startup},
    prelude::*,
};
use bevy_submerge_ui::{
    core::{
        style_bundles::{ButtonStyleBundle, ContainerStyleBundle, SubmergeStyle},
        ui_bundles::{AddChildren, SButtonBundle, SContainerBundle, STextBundle},
        ui_plugin::SubmergeUi,
        ui_systems::ChildrenResource,
    },
    utils::{colors::SubmergeColors, font_size::SubmergeText},
    widgets::widget_plugin::SubmergeWidgets,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SubmergeUi)
        .add_plugins(SubmergeWidgets)
        // .add_plugins(SubmergeBox)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut children_resource: ResMut<ChildrenResource>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let button_style = ButtonStyleBundle::apply_style(
        "border-5px justify_content-center align_items-center padding-15px border_color-white rounded-50% bg-red-100",
    );

    let text_style = TextStyle {
        font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
        font_size: SubmergeText::text(SubmergeText::LG),
        color: SubmergeColors::color(SubmergeColors::BLACK).into(),
    };

    commands
        .spawn(STextBundle::from_section("Some Button", text_style.clone()))
        .insert_id("some_text", &mut children_resource);

    commands
        .spawn(SButtonBundle::new("play_button_2", button_style.clone()))
        .insert_id("some_button", &mut children_resource)
        .child("some_text", &mut children_resource);

    commands
        .spawn(SContainerBundle::new(
            "main_container",
            ContainerStyleBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..default()
            },
        ))
        .insert_id("container", &mut children_resource)
        .children(["some_button"], &mut children_resource);
}
