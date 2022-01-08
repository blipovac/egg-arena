use bevy::prelude::*;
use bevy::pbr::AmbientLight;
use bevy_flycam::PlayerPlugin;

mod systems;
mod components;
mod entities;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    GameMenu,
    InGame,
}

pub fn run() {
    App::build()
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
                .with_system(set_egg_count.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(systems::setup::meshes::egg::setup.system())
                .with_system(systems::setup::background::setup.system())
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(systems::runtime::rotation::rotate.system())
                .with_system(systems::runtime::fly_cam_transform::report_fly_cam_transform.system())
        )
        .run();
}

fn set_egg_count(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
) {
    commands
        .spawn()
        .insert(entities::Game)
        .insert(components::EggCount(10));

    app_state.set(AppState::InGame).expect("Failed transitioning to InGame state!");
}