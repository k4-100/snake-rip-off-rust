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

#[derive(Component, Debug)]
pub struct Block;

#[derive(Component, Debug)]
pub struct HitBox{
  pub bottom_left: Vec2,
  pub top_right: Vec2,
  pub width: f32,
  pub height: f32
}

impl HitBox{
  // pub fn intersects( hitbox: Vec2 ) -> bool{
  //   return false;
  // }

  pub fn from_translation(width:f32, height:f32, translation: &Vec3 ) -> HitBox{
    let x = translation.x;
    let y = translation.y;

    HitBox { 
      bottom_left: Vec2 { x: x - width/2., y: y - height/2. } , 
      top_right:  Vec2 { x: x + width/2., y: y + height/2. },
      width,
      height
    }
  }
}


// endregion: --- PlayerPlugin Components