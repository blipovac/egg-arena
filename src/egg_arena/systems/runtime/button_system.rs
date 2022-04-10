use bevy::prelude::*;

use crate::egg_arena::components::ButtonVariant;

const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15, 0.70);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn listen_for_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &ButtonVariant),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color, button_variant) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                resolve_button_action(button_variant);
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

fn resolve_button_action(button_variant: &ButtonVariant) {
    match *button_variant {
        ButtonVariant::Play => {
            println!("Play Button Pressed");
        }
        ButtonVariant::Quit => {
            println!("Quit");
        }
        ButtonVariant::IncreacePlayerNumber => {
            println!("Increaes");
        }
        ButtonVariant::DecreasePlayerNumber => {
            println!("Decrease");
        }
    }
}