use bevy::prelude::*;


// region: --- PlayerPlugin Components 

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Velocity{
  pub x: f32,
  pub y: f32
}

#[derive(Component, Eq, PartialEq, Debug)]
pub enum Player{
  Head,
  Body
}

#[derive(Component)]
pub struct Block;

#[derive(Resource, Debug)]
pub struct PreviousPosition(pub Vec3);
// #[derive(Component)]
// pub struct Head;

// #[derive(Component)]
// pub struct Body;
// endregion: --- PlayerPlugin Components