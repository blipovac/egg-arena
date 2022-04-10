use bevy::prelude::Component;

#[derive(Component)]
pub struct EggCount(pub u32);

#[derive(Component)]
pub struct Rotates;

#[derive(Component)]
pub struct Selected(pub bool);

#[derive(Component)]
pub enum ButtonVariant {
    Play,
    Quit,
    IncreacePlayerNumber,
    DecreasePlayerNumber,
}
