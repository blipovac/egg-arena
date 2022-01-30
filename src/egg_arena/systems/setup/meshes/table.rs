use bevy::prelude::*;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let table_handle: Handle<Scene> = asset_server.load("table/wooden_table_02_1k.gltf#Scene0");

    commands.spawn_scene(table_handle);
}