use bevy::prelude::{
    Commands,
    Res,
    AssetServer,
    Texture,
    shape,
    ResMut,
    Assets,
    Mesh,
    StandardMaterial,
    Handle,
    PbrBundle,
    Color, 
    Transform,
    Vec3,
    LightBundle,
    Light, 
    BuildChildren,
};

pub fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let background_handle: Handle<Texture> = asset_server.load("background/comfy_cafe_4k.hdr");    

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 50.0,
            depth: 5.0,
            ..Default::default()
         })),
        material: materials.add(StandardMaterial{
            base_color: Color::BLACK,
            emissive: Color::WHITE,
            emissive_texture: Some(background_handle),
            ..Default::default()
        }),
        transform: Transform::from_scale(Vec3::new(-1.0, -1.0, -1.0)),
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn_bundle(LightBundle {
            transform: Transform::from_xyz(10.0, 2.0, 0.0),
            light: Light {
                intensity: 1000.0,
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        });
    });  
}