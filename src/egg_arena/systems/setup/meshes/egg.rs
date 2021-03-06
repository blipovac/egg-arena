use bevy::prelude::*;

use crate::egg_arena::components::{EggCount, Selected};
use crate::egg_arena::items::{Game, Egg};

const EGG_HEIGHT: f32 = 0.840;

pub fn setup(
    egg_count: Res<EggCount>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let egg_column_count: u32 = 2;
    let egg_padding: f32 = 0.06;
    let egg_x_offset = 0.2;
    let egg_z_offset = 0.06;

    let egg_handle: Handle<Scene> = asset_server.load("egg/scene.gltf#Scene0");

    for i in 0..egg_count.0 {
        let selected: bool = i < 2;
        commands
            .spawn_bundle((
                Transform::from_xyz(
                    (i / egg_column_count) as f32 * egg_padding - egg_x_offset,
                    EGG_HEIGHT,
                    (i % egg_column_count) as f32 * egg_padding - egg_z_offset,
                )
                .with_scale(Vec3::new(0.03, 0.03, 0.03)),
                GlobalTransform::identity(),
            ))
            .with_children(|builder| {
                builder.spawn_scene(egg_handle.clone());
            })
            .insert(Egg)
            .insert(Selected(selected));
    }
}
