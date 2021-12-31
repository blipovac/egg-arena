use bevy::prelude::{Commands, ResMut, Res, Assets, Mesh, StandardMaterial, AssetServer, Handle, Texture, PbrBundle, Transform, Color, LightBundle, shape, PerspectiveCameraBundle, Vec3};

pub fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
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

    commands.spawn_bundle(PbrBundle {
        mesh: egg_handle,
        material: material_handle,
        transform: Transform::from_xyz(-0.0, 1.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}