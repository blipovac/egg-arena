// TODO when both eggs have reached the faceoff points trigger an event to let the game know that eggs are in position

use crate::egg_arena::components::Selected;
use crate::egg_arena::items::{Egg, LeftFaceoffPosition, RightFaceoffPosition};
use bevy::prelude::*;
use std::f32;

pub fn move_egg_to_faceoff_position(
    left_faceoff_position: Res<LeftFaceoffPosition>,
    right_faceoff_position: Res<RightFaceoffPosition>,
    time: Res<Time>,
    mut selected_eggs_transform_query: Query<&mut Transform, (With<Egg>, With<Selected>)>,
) {
    let mut selected_egg_query_iterator = selected_eggs_transform_query.iter_mut();

    let a_transform: Mut<Transform> = selected_egg_query_iterator.next().expect("no egg found!");
    let b_transform: Mut<Transform> = selected_egg_query_iterator.next().expect("no egg found!");

    let increment = time.delta_seconds();

    move_towards(increment, a_transform, left_faceoff_position.transform);
    move_towards(increment, b_transform, right_faceoff_position.transform);
}

/// Find the biggest distance of x, y and z components
/// Use the distance to "slow down" incrementation of transform components that are closer to the destination

pub fn move_towards(
    time_delta_second: f32,
    mut source_transform: Mut<Transform>,
    destination_transform: Transform,
) {
    let x_distance =
        (source_transform.translation.x).abs() + (destination_transform.translation.x).abs();
    let y_distance =
        (source_transform.translation.y).abs() + (destination_transform.translation.y).abs();
    let z_distance =
        (source_transform.translation.z).abs() + (destination_transform.translation.z).abs();

    let mut transform_distances = [x_distance, y_distance, z_distance];

    transform_distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let max_distance = transform_distances[2];

    let slowdown_factor_x = x_distance / max_distance;
    let slowdown_factor_y = y_distance / max_distance;
    let slowdown_factor_z = z_distance / max_distance;

    let global_slowdown: f32 = 0.05;

    let direction_factor_x = calculate_direction_factor(
        source_transform.translation.x,
        destination_transform.translation.x,
    );
    let direction_factor_y = calculate_direction_factor(
        source_transform.translation.y,
        destination_transform.translation.y,
    );
    let direction_factor_z = calculate_direction_factor(
        source_transform.translation.z,
        destination_transform.translation.z,
    );

    let should_move_factor_x = should_move(
        source_transform.translation.x,
        destination_transform.translation.x,
    );
    let should_move_factor_y = should_move(
        source_transform.translation.y,
        destination_transform.translation.y,
    );
    let should_move_factor_z = should_move(
        source_transform.translation.z,
        destination_transform.translation.z,
    );

    source_transform.translation = source_transform.translation
        + Vec3::new(
            should_move_factor_x
                * time_delta_second
                * slowdown_factor_x
                * direction_factor_x
                * global_slowdown,
            should_move_factor_y
                * time_delta_second
                * slowdown_factor_y
                * direction_factor_y
                * global_slowdown,
            should_move_factor_z
                * time_delta_second
                * slowdown_factor_z
                * direction_factor_z
                * global_slowdown,
        );
}

/// Determines whether value of translation component should increase or decrease
/// Depending on the relative position of the source and destination points

pub fn calculate_direction_factor(
    source_transtlation_component: f32,
    destination_translation_component: f32,
) -> f32 {
    if destination_translation_component - source_transtlation_component == 0.0 {
        return 0.0;
    }

    return (destination_translation_component - source_transtlation_component)
        / (destination_translation_component - source_transtlation_component).abs();
}

/// Determines whether the translation component value should update anymore i.e. has the
/// Entity reaches its destination point for the respective translation component

fn should_move(source_transtlation_component: f32, destination_translation_component: f32) -> f32 {
    let should_move: i32 =
        (source_transtlation_component.abs() < destination_translation_component.abs()) as i32;

    return should_move as f32;
}
