use bevy::prelude::{Res, Time, Query, Transform, With, Quat};

use crate::egg_arena::components::Rotates;

pub fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}