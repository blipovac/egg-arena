use bevy::prelude::*;

pub fn setup(font_handle: Handle<Font>) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Percent(5.00)),
            ..Default::default()
        },
        text: Text::with_section(
            "Choose the number of players:",
            TextStyle {
                font: font_handle,
                font_size: 40.0,
                color: Color::rgb(0.95, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}