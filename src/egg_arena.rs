use bevy::pbr::AmbientLight;
use bevy::prelude::*;

mod components;
mod items;
mod plugins;
mod systems;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    GameMenu,
    InGame,
}

pub fn run() {
    let mut app: App = App::new();

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
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(systems::setup::meshes::table::setup)
                .with_system(systems::setup::meshes::background::setup)
                .with_system(systems::setup::lights::setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(systems::runtime::utils::check_loading_progress),
        )
        .add_system_set(SystemSet::on_enter(AppState::GameMenu).with_system(set_egg_count))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(systems::setup::meshes::egg::setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(systems::runtime::rotation::rotate)
                .with_system(
                    systems::runtime::move_egg_to_faceoff_position::move_egg_to_faceoff_position,
                ),
        );

    #[cfg(debug_assertions)]
    app.add_plugin(plugins::dev_cam::DevCam);

    #[cfg(not(debug_assertions))]
    app.add_system_set(SystemSet::on_enter(AppState::GameMenu).with_system(systems::setup::camera::setup));

    app.run();
}

fn set_egg_count(mut commands: Commands, mut app_state: ResMut<State<AppState>>) {
    commands
        .spawn()
        .insert(items::Game)
        .insert(components::EggCount(10));

    app_state
        .set(AppState::InGame)
        .expect("Failed transitioning to InGame state!");
}
