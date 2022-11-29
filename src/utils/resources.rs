use bevy::prelude::*;

// region: --- PlayerPlugin Components 

#[derive(Resource, Debug)]
pub struct PreviousPosition(pub Vec3);

#[derive(Resource, Debug)]
pub struct Score(pub u32);

// endregion: --- PlayerPlugin Components 
