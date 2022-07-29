use bevy::prelude::*;

mod entire_screen_container;
mod game_title_image;
mod general_menu_button;
mod middle_elements_container;
mod players_number_mutation_button;
mod players_number_picker_container;
mod players_number_picker_label;
mod players_number_text;

use crate::egg_arena::{components::ButtonVariant, items::{EggCountText, Ui}};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Light.ttf");

    commands
        // Screen parent
        .spawn_bundle(entire_screen_container::setup())
        .with_children(|builder| {
            builder
                // Container for the logo and the buttons
                .spawn_bundle(middle_elements_container::setup())
                // Middle container children
                .with_children(|builder| {
                    // Game logo
                    builder.spawn_bundle(game_title_image::setup(&asset_server));

                    // Player number picker label
                    builder.spawn_bundle(players_number_picker_label::setup(font_handle.clone()));

                    // Number of players picker container
                    builder
                        .spawn_bundle(players_number_picker_container::setup())
                        // Number of players picker container children
                        .with_children(|builder| {
                            // Decrease player number button
                            builder
                                .spawn_bundle(players_number_mutation_button::setup())
                                .insert(ButtonVariant::DecreasePlayerNumber)
                                // Decrease player number children
                                .with_children(|builder| {
                                    // Decrease player number button text
                                    builder.spawn_bundle(
                                        players_number_mutation_button::text::setup(
                                            font_handle.clone(),
                                            "<",
                                        ),
                                    );
                                });

                            // Player number text
                            builder
                                .spawn_bundle(players_number_text::setup(font_handle.clone()))
                                .insert(EggCountText);

                            // Increase player number button
                            builder
                                .spawn_bundle(players_number_mutation_button::setup())
                                .insert(ButtonVariant::IncreasePlayerNumber)
                                // Increase player number children
                                .with_children(|builder| {
                                    // Increase player number button text
                                    builder.spawn_bundle(
                                        players_number_mutation_button::text::setup(
                                            font_handle.clone(),
                                            ">",
                                        ),
                                    );
                                });
                        });

                    // Play button
                    builder
                        .spawn_bundle(general_menu_button::setup())
                        .insert(ButtonVariant::Play)
                        // Play button children
                        .with_children(|builder| {
                            // Play button text
                            builder.spawn_bundle(general_menu_button::text::setup(
                                font_handle.clone(),
                                "Play",
                            ));
                        });

                    // Quit button
                    builder
                        .spawn_bundle(general_menu_button::setup())
                        .insert(ButtonVariant::Quit)
                        // Quit button children
                        .with_children(|builder| {
                            // Quit button text
                            builder.spawn_bundle(general_menu_button::text::setup(
                                font_handle,
                                "Quit",
                            ));
                        });
                });
        })
        .insert(Ui);
}
