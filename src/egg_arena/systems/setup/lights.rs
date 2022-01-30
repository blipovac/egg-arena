use bevy::prelude::*;

pub fn setup(
    mut commands: Commands
) {
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
            color: Color::rgb(1.0, 1.0, 0.75),
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(42.0, 5.0, 26.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 0.75),
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-45.0, 5.0, 14.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 0.75),
            intensity: 7500.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-38.0, 5.0, -32.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 0.75),
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
