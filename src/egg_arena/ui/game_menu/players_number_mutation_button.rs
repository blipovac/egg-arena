use bevy::prelude::*;

use crate::egg_arena::constants::NORMAL_BUTTON;

pub mod text;

pub fn setup() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(10.0), Val::Percent(20.0)),
            ..Default::default()
        },
        color: NORMAL_BUTTON.into(),
        ..Default::default()
    }
}