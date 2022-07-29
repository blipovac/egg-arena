use bevy::input::system;
use bevy::pbr::AmbientLight;
use bevy::prelude::*;

use self::components::EggCount;

mod components;
mod constants;
mod helpers;
mod items;
mod plugins;
mod systems;
mod ui;

const INITIAL_EGG_COUNT: u32 = 12;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    GameMenu,
    InGame,
}

pub fn run() {
    let mut app: App = App::new();

    // Resource setup
    app.insert_resource(items::AssetsLoading(Vec::new()))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(items::LeftFaceoffPosition {
            transform: Transform::from_xyz(0.75, 0.0, 0.0),
        })
        .insert_resource(items::RightFaceoffPosition {
            transform: Transform::from_xyz(-0.75, 0.0, 0.0),
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.05,
        })
        .insert_resource(EggCount(INITIAL_EGG_COUNT))
        .insert_resource(items::EggPickCameraPosition {
            transform: Transform::from_xyz(-0.018, 1.10, 0.42)
                .with_rotation(Quat::from_xyzw(-0.3, 0.0, 0.0, 0.95)),
        });

    app.add_plugins(DefaultPlugins);

    #[cfg(debug_assertions)]
    app.add_plugin(plugins::dev_cam::DevCam);

    // Loading state, load all meshes, lights, cameras etc. that can be loaded at this point
    app.add_state(AppState::Loading)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(systems::setup::meshes::table::setup)
                .with_system(systems::setup::meshes::background::setup)
                .with_system(systems::setup::lights::setup)
                .with_system(systems::setup::meshes::egg::setup)
                .with_system(ui::ui_camera::setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(systems::runtime::utils::check_loading_progress),
        );

    // Game menu
    #[cfg(not(debug_assertions))]
    app.add_system_set(
        SystemSet::on_enter(AppState::GameMenu)
            .with_system(ui::game_menu::setup)
            .with_system(systems::setup::camera::setup),
    )
    .add_system_set(
        SystemSet::on_update(AppState::GameMenu)
            .with_system(systems::runtime::rotation::rotate)
            .with_system(systems::runtime::button_system::listen_for_button_interaction)
            .with_system(systems::runtime::egg_count::update_egg_count_text),
    );

    #[cfg(not(debug_assertions))]
    app.add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(systems::setup::de_spawn_game_menu)
            .with_system(systems::setup::meshes::egg::setup),
    )
    .add_system_set(SystemSet::on_update(AppState::InGame).with_system(
        systems::runtime::move_camera_to_egg_pick_position::move_egg_to_faceoff_position,
    ));

    // In game
    // spawn a number of eggs corresponding to the player number
    // Move the camera to the egg picking position
    // add the 3d mouse picker
    // create labels and make them reflect camera rotation
    // .add_system_set(
    //     SystemSet::on_enter(AppState::InGame).with_system(systems::setup::meshes::egg::setup),
    // )
    // .add_system_set(
    //     SystemSet::on_update(AppState::InGame)
    //         .with_system(systems::runtime::rotation::rotate)
    //         .with_system(systems::runtime::button_system::listen_for_button_interaction)
    //         .with_system(
    //             systems::runtime::move_egg_to_faceoff_position::move_egg_to_faceoff_position,
    //         ),
    // );

    app.run();
}
