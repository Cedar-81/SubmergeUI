use bevy::{
    app::{App, Startup},
    prelude::*,
};
use bevy_submerge_ui::{
    core::{
        style_bundles::{ButtonStyleBundle, ContainerStyleBundle, SubmergeStyle},
        ui_bundles::{SButtonBundle, SContainerBundle, STextBundle, WithChildren},
        ui_components::UiComponent,
        ui_plugin::SubmergeUi,
    },
    r#box::box_plugin::SubmergeBox,
    utils::{border_radius::SubmergeBR, colors::SubmergeColors, font_size::SubmergeText},
    widgets::{
        input::InputBundle,
        slider::SliderBundle,
        style_bundles::{
            InputComponentStyle, InputStyleBundle, SliderComponentsStyle, SliderStyleBundle,
        },
        widget_plugin::SubmergeWidgets,
    },
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());

    // let slider_style = ButtonStyleBundle::apply_style(
    //     "height-20px width-100px border-2px border_color-green-400 justify_content-start align_items-center",
    // );

    // let handle_style = ButtonStyleBundle::apply_style("height-25px width-18px bg-slate-400");

    // let slide_style = ButtonStyleBundle::apply_style("height-20px width-38px bg-slate-300");

    let slider_style = SliderStyleBundle {
        style: Style {
            height: Val::Px(20.),
            width: Val::Px(100.),
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        component_style: SliderComponentsStyle {
            slide_background_color: BackgroundColor(SubmergeColors::color(
                SubmergeColors::SLATE300,
            )),
            slide_width: Val::Px(38.),
            slide_height: Val::Px(16.),
            handle_background_color: BackgroundColor(SubmergeColors::color(
                SubmergeColors::SLATE500,
            )),
            handle_height: Val::Px(25.),
            handle_width: Val::Px(18.),
            // handle_border_radius: BorderRadius::all(Val::Percent(100.)),
            ..default()
        },
        border_color: BorderColor(SubmergeColors::color(SubmergeColors::EMERALD700)),
        ..default()
    };

    let slider_style_2 = SliderStyleBundle {
        style: Style {
            height: Val::Px(20.),
            width: Val::Px(100.),
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            overflow: Overflow::clip(),
            ..default()
        },
        component_style: SliderComponentsStyle {
            slide_background_color: BackgroundColor(SubmergeColors::color(SubmergeColors::PINK500)),
            slide_width: Val::Px(38.),
            slide_height: Val::Px(16.),
            slide_border_radius: BorderRadius::all(Val::Px(30.)),
            ..default()
        },
        background_color: BackgroundColor(SubmergeColors::color(SubmergeColors::PINK300)),
        border_color: BorderColor(SubmergeColors::color(SubmergeColors::PINK500)),
        border_radius: BorderRadius::all(Val::Px(30.)),
        ..default()
    };

    let input_style = InputStyleBundle { ..default() };

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
        .children(["slider", "slider2", "input1"]),
    );

    commands.spawn(SliderBundle::new("slider", slider_style));
    commands.spawn(SliderBundle::new("slider2", slider_style_2));
    // commands.spawn(SliderBundle::new(
    //     "slider3",
    //     SliderStyleBundle { ..default() },
    // )); //todo: Modify default style for SliderStyleBundle
    commands.spawn(InputBundle::new("input1", input_style, "My first text"));
}
