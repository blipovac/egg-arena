use bevy::{prelude::{
    shape, AssetServer, Assets, Color, Commands, Handle, Image, Mesh, PbrBundle,
    PointLight, Res, ResMut, StandardMaterial, Transform, Vec3, PointLightBundle
}};

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
            depth: 5.0,
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

    commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 250000.0,
                range: 10000.0,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 54.0, 0.),
            ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::YELLOW,
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(42.0, 5.0, 26.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::YELLOW,
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-45.0, 5.0, 14.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::YELLOW,
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-38.0, 5.0, -32.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::YELLOW,
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(9.0, 5.0, -48.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(0.8, 0.8, 1.0),
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(34.0, 11.0, -35.0),
        ..Default::default()
    });
}
