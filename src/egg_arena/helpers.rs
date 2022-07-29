use bevy::{
    math::{Quat, Vec3},
    prelude::{Mut, Transform},
};

/// Find the biggest distance of x, y and z components
/// Use the distance to "slow down" incrementation of transform components that are closer to the destination

pub fn translate_towards(
    global_slowdown_factor: f32,
    time_delta_second: f32,
    source_translation: Vec3,
    destination_translation: Vec3,
) -> Vec3 {
    let x_distance = (source_translation.x).abs() + (destination_translation.x).abs();
    let y_distance = (source_translation.y).abs() + (destination_translation.y).abs();
    let z_distance = (source_translation.z).abs() + (destination_translation.z).abs();

    let mut transform_distances = [x_distance, y_distance, z_distance];

    // Reverse sorts from max to min
    transform_distances.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    let max_distance = transform_distances[0];

    let slowdown_factor_x = x_distance / max_distance;
    let slowdown_factor_y = y_distance / max_distance;
    let slowdown_factor_z = z_distance / max_distance;

    let direction_factor_x =
        calculate_direction_factor(source_translation.x, destination_translation.x);
    let direction_factor_y =
        calculate_direction_factor(source_translation.y, destination_translation.y);
    let direction_factor_z =
        calculate_direction_factor(source_translation.z, destination_translation.z);

    let should_move_factor_x = should_move(
        source_translation.x,
        destination_translation.x,
        direction_factor_x,
    );
    let should_move_factor_y = should_move(
        source_translation.y,
        destination_translation.y,
        direction_factor_y,
    );
    let should_move_factor_z = should_move(
        source_translation.z,
        destination_translation.z,
        direction_factor_z,
    );

    source_translation
        + Vec3::new(
            should_move_factor_x
                * time_delta_second
                * slowdown_factor_x
                * direction_factor_x
                * global_slowdown_factor,
            should_move_factor_y
                * time_delta_second
                * slowdown_factor_y
                * direction_factor_y
                * global_slowdown_factor,
            should_move_factor_z
                * time_delta_second
                * slowdown_factor_z
                * direction_factor_z
                * global_slowdown_factor,
        )
}

pub fn rotate_towards(
    global_slowdown_factor: f32,
    time_delta_second: f32,
    mut source_rotation: Quat,
    destination_rotation: Quat,
) -> Quat {
    let x_distance = (source_rotation.x).abs() + (destination_rotation.x).abs();
    let y_distance = (source_rotation.y).abs() + (destination_rotation.y).abs();
    let z_distance = (source_rotation.z).abs() + (destination_rotation.z).abs();
    let w_distance = (source_rotation.w).abs() + (destination_rotation.w).abs();

    println!("y distance: {}", y_distance);

    let mut transform_distances = [x_distance, y_distance, z_distance, w_distance];

    // Reverse sorts from max to min
    transform_distances.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    let max_distance = transform_distances[0];

    let slowdown_factor_x = x_distance / max_distance;
    let slowdown_factor_y = y_distance / max_distance;
    let slowdown_factor_z = z_distance / max_distance;
    let slowdown_factor_w = w_distance / max_distance;

    println!("slowdown_factor_y: {}", slowdown_factor_y);

    let direction_factor_x = calculate_direction_factor(source_rotation.x, destination_rotation.x);
    let direction_factor_y = calculate_direction_factor(source_rotation.y, destination_rotation.y);
    let direction_factor_z = calculate_direction_factor(source_rotation.z, destination_rotation.z);
    let direction_factor_w = calculate_direction_factor(source_rotation.w, destination_rotation.w);

    println!("direction factor y: {}", direction_factor_y);

    println!("source rotation y: {}", source_rotation.y);
    println!("destination rotation y: {}", destination_rotation.y);

    let should_move_factor_x = should_move(
        source_rotation.x,
        destination_rotation.x,
        direction_factor_x,
    );
    let should_move_factor_y = should_move(
        source_rotation.y,
        destination_rotation.y,
        direction_factor_y,
    );
    let should_move_factor_z = should_move(
        source_rotation.z,
        destination_rotation.z,
        direction_factor_z,
    );
    let should_move_factor_w = should_move(
        source_rotation.w,
        destination_rotation.w,
        direction_factor_w,
    );

    println!("should move factor y: {}", should_move_factor_y);

    source_rotation
        + Quat::from_xyzw(
            should_move_factor_x
                * time_delta_second
                * slowdown_factor_x
                * direction_factor_x
                * global_slowdown_factor,
            should_move_factor_y
                * time_delta_second
                * slowdown_factor_y
                * direction_factor_y
                * global_slowdown_factor,
            should_move_factor_z
                * time_delta_second
                * slowdown_factor_z
                * direction_factor_z
                * global_slowdown_factor,
            should_move_factor_w
                * time_delta_second
                * slowdown_factor_w
                * direction_factor_w
                * global_slowdown_factor,
        )
}

/// Determines whether value of translation component should increase or decrease
/// Depending on the relative position of the source and destination points

pub fn calculate_direction_factor(
    source_translation_component: f32,
    destination_translation_component: f32,
) -> f32 {
    return (destination_translation_component - source_translation_component)
        / (destination_translation_component - source_translation_component).abs();
}

/// Determines whether the translation component value should update anymore i.e. has the
/// Entity reaches its destination point for the respective translation component

fn should_move(
    source_translation_component: f32,
    destination_translation_component: f32,
    component_direction_factor: f32,
) -> f32 {
    let should_move: i32 = (((destination_translation_component - source_translation_component) * component_direction_factor) > 0.0) as i32;

    return should_move as f32;
}
