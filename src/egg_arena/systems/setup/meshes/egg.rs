use bevy::prelude::*;

use crate::egg_arena::components::EggCount;
use crate::egg_arena::items::Game;

pub fn setup(
    query: Query<&EggCount, With<Game>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let egg_count = query.single().0;
    let egg_column_count: u32 = 2;
    let egg_padding: f32 = 0.06;
    let egg_x_offset = 0.1;
    let egg_z_offset = 0.03;

    let egg_handle: Handle<Scene> = asset_server.load("egg/scene.gltf#Scene0");

    asset_server.watch_for_changes().unwrap();

    for i in 0..egg_count {
        commands
            .spawn_bundle((
                Transform::from_xyz(
                    (i / egg_column_count) as f32 * egg_padding - egg_x_offset,
                    0.852,
                    (i % egg_column_count) as f32 * egg_padding - egg_z_offset,
                )
                .with_scale(Vec3::new(0.03, 0.03, 0.03)),
                GlobalTransform::identity(),
            ))
            .with_children(|builder| {
                builder.spawn_scene(egg_handle.clone());
            });
    }
}
