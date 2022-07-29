use bevy::prelude::{Query, Entity, With, Commands, DespawnRecursiveExt, Or};

use crate::egg_arena::items::{Egg, Ui};

pub mod meshes;
pub mod text;
pub mod lights;
pub mod camera;
pub mod set_initial_egg_count;

pub fn de_spawn_game_menu(
    mut commands: Commands,
    egg_query: Query<Entity, Or<(With<Egg>, With<Ui>)>>
) {
    for egg in egg_query.iter() {
        commands.entity(egg).despawn_recursive();
    }
}