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
    utils::{border_radius::SubmergeBR, colors::SubmergeColors},
};

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(SubmergeUi)
        .add_systems(Startup, setup)
        .run();
}

// struct MyStruct<T: System<In = (), Out = ()>> {
//     some_value: T,
// }

// impl<T> MyStruct<T>
// where
//     T: System<In = (), Out = ()>,
// {
//     fn new(system: T) -> Self {
//         MyStruct { some_value: system }
//     }
// }

fn _sys(query: Query<&Style>) {
    for a in &query {
        println!("a: {:?}", a);
    }
}
// fn after_setup(mut commands: Commands) {
//     // let a = sys.();

//     let a = MyStruct::new(sys);

//     let mut system = IntoSystem::into_system(sys);
//     // a.add_system(system.clone());

//     commands.add(move |world: &mut World| {
//         // do whatever you want with `world` here

//         // note: it's a closure, you can use variables from
//         // the parent scope/function
//         let id = world.register_system(system.clone());
//         eprintln!("System id: {:?}", id);

//         let new_stuff = IntoSystem::into_system(a);
//         world.run_system_once(new_stuff);

//         eprintln!("Now to run the world");
//         world.run_system_once(&system);
//     });
// }
// asset_server: Res<AssetServer>
fn setup(mut commands: Commands) {
    // let a = NORMAL_BUTTON.into();
    let s_button_style: ButtonStyleBundle = ButtonStyleBundle {
        style: Style {
            // width: Val::Px(150.0),
            // height: Val::Px(65.0),
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

    // button_style.border_color = BorderColor(Color::BLACK);

    // println!("button: {:?}", button_style);

    // let text_style = TextStyle {
    //     font: asset_server.load("fonts/OpenSans-SemiBold.ttf"),
    //     font_size: SubmergeText::text(SubmergeText::LG),
    //     color: SubmergeColors::color(SubmergeColors::BLACK).into(),
    // };

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

    // commands
    //     .spawn(STextBundle::from_section("First Button", text_style.clone()).id("play_button_txt"));
    // commands.spawn(
    //     STextBundle::from_section("Another Button", text_style).id("another_play_button_txt"),
    // );
}
