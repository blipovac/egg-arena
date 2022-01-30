use bevy::prelude::{
    shape, AssetServer, Assets, Color, Commands, Handle, Image, Mesh, PbrBundle, Res, ResMut,
    StandardMaterial, Transform, Vec3,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let background_handle: Handle<Image> = asset_server.load("background/comfy_cafe_4k.hdr");

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 50.0,
            depth: 3.0,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: Color::WHITE,
            emissive_texture: Some(background_handle),
            ..Default::default()
        }),
        transform: Transform::from_scale(Vec3::new(-1.0, -1.0, -1.0)),
        ..Default::default()
    });
}
