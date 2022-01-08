use bevy::prelude::{
    Transform,
    Query,
    With
};
use bevy_flycam::FlyCam;

pub fn report_fly_cam_transform(
    query: Query<&Transform, With<FlyCam>>
) {
    for transform in query.iter() {
        println!("{:?}", &transform);
    }
}