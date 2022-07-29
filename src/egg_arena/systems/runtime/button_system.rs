use bevy::app;
use bevy::{app::AppExit, prelude::*};

use crate::egg_arena::components::{ButtonVariant, EggCount};
use crate::egg_arena::AppState;

const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.70);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const MAX_EGG_NO: u32 = 12;
const MIN_EGG_NO: u32 = 2;

pub fn listen_for_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ButtonVariant),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
    mut egg_count: ResMut<EggCount>,
    mut app_state: ResMut<State<AppState>>
) {
    for (interaction, mut color, button_variant) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                resolve_button_action(button_variant, &mut exit, &mut egg_count, &mut app_state);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn resolve_button_action(
    button_variant: &ButtonVariant,
    exit: &mut EventWriter<AppExit>,
    egg_count: &mut ResMut<EggCount>,
    app_state: &mut ResMut<State<AppState>>
) {
    match *button_variant {
        ButtonVariant::Play => {
            app_state
            .set(AppState::InGame)
            .expect("Failed transitioning to GameMenu state!");
        }
        ButtonVariant::Quit => {
            exit.send(AppExit);
        }
        ButtonVariant::IncreasePlayerNumber => {
            // Ensure we don't go over the max player number
            egg_count.0 = std::cmp::min(egg_count.0 + 1, MAX_EGG_NO);
        }
        ButtonVariant::DecreasePlayerNumber => {
            // Ensure we don't go under the min player number
            egg_count.0 = std::cmp::max(egg_count.0 - 1, MIN_EGG_NO);
        }
    }
}
