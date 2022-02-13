use crate::egg_arena::items::FlyCamTransformText;
use bevy::prelude::{Query, Text, Transform, With};
use bevy_flycam::FlyCam;

pub fn report_fly_cam_transform(
    flycam_transform_query: Query<&Transform, With<FlyCam>>,
    mut flycam_transform_text_query: Query<&mut Text, With<FlyCamTransformText>>,
) {
    let flycam_transform = flycam_transform_query.single();

    let mut flycam_transform_text = flycam_transform_text_query.single_mut();

    // second section is where we display transform values
    flycam_transform_text.sections[1].value = format!("{:?}", flycam_transform);
}
