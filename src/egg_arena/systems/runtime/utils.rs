use bevy::prelude::*;
use bevy::asset::LoadState;
use crate::egg_arena::items::AssetsLoading;
use crate::egg_arena::AppState;

pub fn check_loading_progress(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>
) {
    // TODO refactor this: iter into a variable crash the game if fails etc...
    match asset_server.get_group_load_state(loading.0.iter().map(|asset_handle| asset_handle.id)) {
        LoadState::Failed => {
            panic!("Error occured while loading asset!");
        }
        LoadState::Loaded => {
            app_state
            .set(AppState::GameMenu)
            .expect("Failed transitioning to GameMenu state!");

            commands.remove_resource::<AssetsLoading>();
        }
        _  => {
            println!("Loading game, please wait...")
        }
    }
}