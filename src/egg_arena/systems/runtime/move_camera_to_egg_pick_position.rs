use bevy::{
    core::Time,
    prelude::{Query, Res, Transform, With},
};
use bevy_flycam::FlyCam;

use crate::egg_arena::{
    components::Translates,
    helpers::{rotate_towards, translate_towards},
    items::EggPickCameraPosition,
};

const GLOBAL_SLOWDOWN: f32 = 0.5;

pub fn move_egg_to_faceoff_position(
    egg_pick_camera_position: Res<EggPickCameraPosition>,
    time: Res<Time>,
    mut camera_transform_query: Query<&mut Transform, With<Translates>>,
) {
    let mut camera_transform = camera_transform_query.single_mut();

    println!("{:?}", &camera_transform);

    let increment = time.delta_seconds();

    camera_transform.translation = translate_towards(
        GLOBAL_SLOWDOWN,
        increment,
        camera_transform.translation,
        egg_pick_camera_position.transform.translation,
    );

    camera_transform.rotation = rotate_towards(
        GLOBAL_SLOWDOWN,
        increment,
        camera_transform.rotation,
        egg_pick_camera_position.transform.rotation,
    );
}
