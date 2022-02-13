use bevy::prelude::{
    Commands,
    Res,
    AssetServer,
    Style,
    TextBundle,
    AlignSelf,
    PositionType,
    Rect,
    Val,
    Text,
    TextStyle,
    Font,
    Handle,
    Color,
    UiCameraBundle,
    TextSection
};

use crate::egg_arena::items::FlyCamTransformText;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Light.ttf");

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Camera coordinates:".to_string(),
                        style: TextStyle { 
                            font: font_handle.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        }
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle { 
                            font: font_handle,
                            font_size: 15.0,
                            color: Color::WHITE,
                        }
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FlyCamTransformText);
}