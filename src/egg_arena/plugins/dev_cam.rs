use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
pub struct DevCam;

mod systems;

// TODO think about how to go about this, maybe use #[cfg] to load normal vs dev camera
impl Plugin for DevCam {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_startup_system(systems::flycam_transform_text::setup.label("spawn_dev_cam"))
            .add_system(systems::runtime::report_fly_cam_transform.after("spawn_dev_cam"));
    }
}