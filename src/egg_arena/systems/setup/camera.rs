use crate::egg_arena::components::{Rotates, Translates};
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|builder| {
            builder
                .spawn_bundle(PerspectiveCameraBundle {
                    transform: Transform::from_xyz(-1.10, 1.55, 0.75)
                        .with_rotation(Quat::from_xyzw(-0.14, -0.43, -0.07, 0.88)),
                    ..Default::default()
                })
                .insert(Rotates)
                .insert(Translates);
        });
}
