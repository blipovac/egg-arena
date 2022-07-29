use bevy::prelude::*;

pub fn setup(font_handle: Handle<Font>, text_value: &str) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text_value,
            TextStyle {
                font: font_handle,
                font_size: 30.0,
                color: Color::rgb(0.95, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}