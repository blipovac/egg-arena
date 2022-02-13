use bevy::prelude::*;
use crate::egg_arena::items::AssetsLoading;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let background_handle: Handle<Image> = asset_server.load("background/comfy_cafe_4k.hdr");

    loading.0.push(background_handle.clone_untyped());

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 1.5,
            rings: 0,
            depth: 2.0,
            latitudes: 16,
            longitudes: 32,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: Color::WHITE,
            emissive_texture: Some(background_handle),
            ..Default::default()
        }),
        transform: Transform::from_scale(Vec3::new(-1.0, -1.0, -1.0))
            .with_translation(Vec3::new(0.0, 1.3, 0.0))
            .with_rotation(Quat::from_xyzw(0.0, 0.397, 0.0, 0.918)),
        ..Default::default()
    });
}
