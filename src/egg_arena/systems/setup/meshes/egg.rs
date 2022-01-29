use bevy::prelude::*;

use crate::egg_arena::components::EggCount;
use crate::egg_arena::items::Game;

pub fn setup(
    query: Query<&EggCount, With<Game>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let egg_count = query.single().0;
    let egg_column_count: u32 = 2;
    let egg_padding: f32 = 2.0;

    let egg_handle: Handle<Scene> = asset_server.load("egg/scene.gltf#Scene0");

    for i in 0..egg_count {
        commands
            .spawn_bundle((
                Transform::from_xyz(
                    (i / egg_column_count) as f32 * egg_padding,
                    1.0,
                    (i % egg_column_count) as f32 * egg_padding,
                ),
                GlobalTransform::identity(),
            ))
            .with_children(|builder| {
                builder.spawn_scene(egg_handle.clone());
            });
    }

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
}
