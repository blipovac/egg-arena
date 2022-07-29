use bevy::prelude::*;
use crate::egg_arena::items;
use crate::egg_arena::components;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(items::Game)
        .insert(components::EggCount(12));
}