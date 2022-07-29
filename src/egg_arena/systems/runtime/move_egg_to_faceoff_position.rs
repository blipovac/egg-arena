// TODO when both eggs have reached the faceoff points trigger an event to let the game know that eggs are in position

use crate::egg_arena::components::Selected;
use crate::egg_arena::helpers::translate_towards;
use crate::egg_arena::items::{Egg, LeftFaceoffPosition, RightFaceoffPosition};
use bevy::prelude::*;
use std::f32;

const GLOBAL_SLOWDOWN: f32 = 0.05;

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

    translate_towards(
        GLOBAL_SLOWDOWN,
        increment,
        a_transform.translation,
        left_faceoff_position.transform.translation,
    );
    translate_towards(
        GLOBAL_SLOWDOWN,
        increment,
        b_transform.translation,
        right_faceoff_position.transform.translation,
    );
}
