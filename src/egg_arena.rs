use bevy::prelude::*;
use bevy::pbr::AmbientLight;
use bevy_flycam::PlayerPlugin;

mod systems;
mod components;
mod items;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    GameMenu,
    InGame,
}

pub fn run() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.05
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_state(AppState::GameMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::GameMenu)
                .with_system(set_egg_count)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(systems::setup::meshes::egg::setup)
                .with_system(systems::setup::meshes::background::setup)
                .with_system(systems::setup::lights::setup)
                .with_system(systems::setup::text::flycam_transform::setup)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(systems::runtime::rotation::rotate)
                .with_system(systems::runtime::flycam_transform::report_fly_cam_transform)
        )
        .run();
}

fn set_egg_count(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
) {
    commands
        .spawn()
        .insert(items::Game)
        .insert(components::EggCount(10));

    app_state.set(AppState::InGame).expect("Failed transitioning to InGame state!");
}