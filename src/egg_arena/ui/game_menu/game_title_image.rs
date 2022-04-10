use bevy::prelude::*;

pub fn setup(asset_server: &Res<AssetServer>) -> ImageBundle {
    let game_title_image_handle: Handle<Image> = asset_server.load("ui/game-logo.png");

    ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
            ..Default::default()
        },
        image: game_title_image_handle.into(),
        ..Default::default()
    }
}
