use bevy::prelude::*;

use crate::egg_arena::{components::EggCount, items::EggCountText};

pub fn update_egg_count_text(
    egg_count: Res<EggCount>,
    mut egg_count_text_query: Query<&mut Text, With<EggCountText>>,
) {
    let mut egg_count_text = egg_count_text_query.single_mut();

    egg_count_text.sections[0].value = egg_count.0.to_string();
}