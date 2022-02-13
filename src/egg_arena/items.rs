use bevy::prelude::{Component, HandleUntyped, Transform};

#[derive(Component)]
pub struct Game;

#[derive(Component)]
pub struct FlyCamTransformText;

#[derive(Component)]
pub struct Egg;

pub struct LeftFaceoffPosition {
    pub transform: Transform,
}

pub struct RightFaceoffPosition {
    pub transform: Transform,
}

pub struct AssetsLoading(pub Vec<HandleUntyped>);
