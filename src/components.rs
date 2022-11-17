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

#[derive(Component)]
pub struct HitBox{
  pub bottom_left: Vec2,
  pub top_right: Vec2,
}

impl HitBox{
  pub fn intersects( hitbox: Vec2 ) -> bool{
    return false;
  }
}


// endregion: --- PlayerPlugin Components