use bevy::prelude::*;


// region: --- PlayerPlugin Components 

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Velocity{
  pub x: f32,
  pub y: f32
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Body;
// endregion: --- PlayerPlugin Components