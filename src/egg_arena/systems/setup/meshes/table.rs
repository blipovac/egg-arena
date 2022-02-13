use bevy::prelude::*;
use crate::egg_arena::items::AssetsLoading;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut loading: ResMut<AssetsLoading>,
) {
    let table_handle: Handle<Scene> = asset_server.load("table/wooden_table_02_1k.gltf#Scene0");

    loading.0.push(table_handle.clone_untyped());

    commands.spawn_scene(table_handle);
}