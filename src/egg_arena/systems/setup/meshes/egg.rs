use bevy::prelude::{
    Commands,
    ResMut,
    Res,
    Assets,
    Mesh,
    StandardMaterial,
    AssetServer,
    Handle,
    Texture,
    PbrBundle,
    Transform,
    Color,
    LightBundle,
    shape,
    PerspectiveCameraBundle,
    Vec3,
    Query,
    With
};

use crate::egg_arena::components::{EggCount, Rotates};
use crate::egg_arena::entities::{Game};

pub fn setup (
    query: Query<&EggCount, With<Game>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let egg_count = query.single().expect("Getting egg count failed!").0.clone();
    let egg_column_count: u32 = 2;
    let egg_padding: f32 = 2.0;


    let egg_handle: Handle<Mesh> = asset_server.load("egg/scene.gltf#Mesh0/Primitive0");

    let egg_texture_handle: Handle<Texture> = asset_server
        .load("egg/textures/DefaultMaterial_baseColor.jpeg");
    
    let egg_metallic_roughness_texture_handle: Handle<Texture> = asset_server.load("egg/textures/DefaultMaterial_metallicRoughness.png");

    let egg_normal_map_handle: Handle<Texture> = asset_server.load("egg/textures/DefaultMaterial_normal.jpeg");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(egg_texture_handle.clone()),
        metallic_roughness_texture: Some(egg_metallic_roughness_texture_handle.clone()),
        normal_map: Some(egg_normal_map_handle.clone()),
        ..Default::default()
    });

    for i in 0..egg_count {
        commands.spawn_bundle(PbrBundle {
            mesh: egg_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_xyz(
                (i / egg_column_count) as f32 * egg_padding,
                1.0,
                (i % egg_column_count) as f32 * egg_padding,
            ),
            ..Default::default()
        });
    }

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-5.0, 5.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Rotates);
}