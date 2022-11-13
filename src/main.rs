use bevy::prelude::*;

mod components;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
  fn build(&self, app: &mut App){
    app
    .add_startup_system(setup)
    .add_system(player_movement_system);
  }
}

// region: --- PlayerPlugin systems

fn setup(mut commands: Commands ){
    commands.spawn(Camera2dBundle::default());
    // Rectangle
    commands.spawn((
    components::Name("square1".to_string()), 
    components::Player,
    components::Velocity{x:1.0,y:0.0},
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    })
  );


}

fn player_movement_system(mut query: Query<(&components::Velocity, &mut Transform), With<components::Player>>){
  for (velocity, mut transform) in query.iter_mut(){
    let translation = &mut transform.translation;
    translation.x += velocity.x;
    translation.y += velocity.y;
  }
}



// endregion: --- Common Components


fn main(){
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .run();
}
