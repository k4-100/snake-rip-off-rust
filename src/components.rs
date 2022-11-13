use bevy::prelude::*;

#[derive(Component)]
pub struct ShapeComponent;
#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Velocity{
  pub x: f32,
  pub y: f32
}

#[derive(Component)]
pub struct Player;