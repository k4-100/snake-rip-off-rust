use bevy::prelude::*;


// region: --- Player Components 

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Velocity{
  pub x: f32,
  pub y: f32
}

#[derive(Component)]
pub struct Player;


// endregion: --- Player Components