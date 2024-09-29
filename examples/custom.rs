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
        checkbox::CheckboxBundle,
        input::InputBundle,
        selector::{Selector, SelectorBundle, SelectorType},
        slider::SliderBundle,
        style_bundles::{
            CheckboxComponentStyle, CheckboxStyleBundle, InputComponentStyle, InputStyleBundle,
            SliderComponentsStyle, SliderStyleBundle, ToggleComponentStyle, ToggleStyleBundle,
        },
        toggle::ToggleBundle,
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
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..default()
            },
        )
        .children([
            "slider",
            "slider2",
            "new_toggle",
            "input1",
            "input2",
            "new_selector",
            "checkbox_1",
        ]),
    );

    commands.spawn(
        SelectorBundle::new(
            "new_selector",
            ContainerStyleBundle {
                style: Style {
                    height: Val::Px(300.),
                    width: Val::Px(500.),
                    ..default()
                },
                background_color: BackgroundColor(SubmergeColors::color(SubmergeColors::AMBER100)),
                ..Default::default()
            },
            SelectorType::Checkbox,
        )
        .children(["new_toggle1", "new_toggle2", "checkbox_2", "checkbox_3"]),
    );

    commands.spawn(SliderBundle::new("slider", slider_style));
    commands.spawn(SliderBundle::new("slider2", slider_style_2));

    let toggle_style = ToggleStyleBundle {
        style: Style {
            height: Val::Px(22.),
            width: Val::Px(50.),
            padding: UiRect::all(Val::Px(2.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            ..default()
        },
        component_style: ToggleComponentStyle {
            indicator_color: SubmergeColors::color(SubmergeColors::GRAY700),
            active_color: SubmergeColors::color(SubmergeColors::PURPLE400),
            indicator_active_color: SubmergeColors::color(SubmergeColors::WHITE),
            height: Val::Px(20.),
            width: Val::Px(20.),
            border_radius: BorderRadius::all(Val::Percent(100.)),
            ..default()
        },
        background_color: BackgroundColor(SubmergeColors::color(SubmergeColors::GRAY100)),
        border_radius: BorderRadius::all(Val::Percent(100.)),
        ..default()
    };

    let checkbox_style = CheckboxStyleBundle {
        style: Style {
            height: Val::Px(20.),
            width: Val::Px(20.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        component_style: CheckboxComponentStyle {
            active_color: SubmergeColors::color(SubmergeColors::PURPLE400),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            border_radius: BorderRadius::all(Val::Percent(100.)),
            ..default()
        },
        border_color: BorderColor(SubmergeColors::color(SubmergeColors::GRAY200)),
        background_color: BackgroundColor(SubmergeColors::color(SubmergeColors::GRAY600)),
        border_radius: BorderRadius::all(Val::Percent(100.)),
        ..default()
    };

    commands.spawn(ToggleBundle::new("new_toggle", toggle_style.clone()));
    commands.spawn(ToggleBundle::new("new_toggle1", toggle_style.clone()));
    commands.spawn(ToggleBundle::new("new_toggle2", toggle_style));

    commands.spawn(CheckboxBundle::new("checkbox_1", checkbox_style.clone()));
    commands.spawn(CheckboxBundle::new("checkbox_2", checkbox_style.clone()));
    commands.spawn(CheckboxBundle::new("checkbox_3", checkbox_style));

    commands.spawn(InputBundle::new(
        "input1",
        input_style.clone(),
        "My first text",
    ));
    commands.spawn(InputBundle::new("input2", input_style, "My second text"));
}
