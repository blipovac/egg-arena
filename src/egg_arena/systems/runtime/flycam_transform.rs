use bevy::prelude::{
    Transform,
    Query,
    With,
    Text
};
use bevy_flycam::FlyCam;
use crate::egg_arena::items::FlyCamTransformText;

pub fn report_fly_cam_transform(
    flycam_transform_query: Query<&Transform, With<FlyCam>>,
    mut flycam_transform_text_query: Query<&mut Text, With<FlyCamTransformText>>
) {
    let mut flycam_transform = &Transform {
        ..Default::default()
    };

    match flycam_transform_query.single() {
        Ok(transform_value) => flycam_transform = transform_value,
        Err(error) => println!("Error querying camera transform value!, {}", error),
    };

    let mut flycam_transform_text = flycam_transform_text_query
        .single_mut()
        .expect("You are trying to set text to a text box that does not exist!");
    
        // second section is where we display transform values
        flycam_transform_text.sections[1].value = format!("{:?}", flycam_transform);
}