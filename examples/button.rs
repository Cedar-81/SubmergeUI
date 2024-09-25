use bevy::{
    app::{App, Startup},
    prelude::*,
};
use bevy_submerge_ui::{
    core::{
        style_bundles::{ButtonStyleBundle, ContainerStyleBundle, SubmergeStyle},
        ui_bundles::{SButtonBundle, SContainerBundle, WithChildren},
        ui_plugin::SubmergeUi,
    },
    utils::{border_radius::SubmergeBR, colors::SubmergeColors, font_size::SubmergeText},
};

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(SubmergeUi)
        // .add_plugins(SubmergeBox)
        .add_systems(Startup, setup)
        .run();
}

fn _sys(query: Query<&Style>) {
    for a in &query {
        println!("a: {:?}", a);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}
