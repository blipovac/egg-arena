use bevy::prelude::*;

pub fn setup(font_handle: Handle<Font>) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Percent(10.00)),
            ..Default::default()
        },
        text: Text::with_section(
            "6",
            TextStyle {
                font: font_handle,
                font_size: 60.0,
                color: Color::rgb(0.95, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}