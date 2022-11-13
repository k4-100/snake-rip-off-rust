use bevy::{
  prelude::*, 
  input::keyboard::*
};


mod components;



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

// fn player_movement_system(mut query: Query<(&components::Velocity, &mut Transform), With<components::Player>>){
//   for (velocity, mut transform) in query.iter_mut(){
//     let translation = &mut transform.translation;
//     translation.x += velocity.x;
//     translation.y += velocity.y;
//   }
// }

fn keyboard_input( mut key_evr: EventReader<KeyboardInput>, mut query: Query<(&components::Velocity, &mut Transform), With<components::Player>>){
  use bevy::input::ButtonState;

  for ev in key_evr.iter(){
    match ev.state{
      ButtonState::Pressed => {
        println!("Key press: {:?} ({})",ev.key_code, ev.scan_code);
        for( velocity, mut transform) in query.iter_mut(){
          let translation = &mut transform.translation;
          match ev.key_code{
            Some(x) =>  match x{

            KeyCode::A => {translation.x -= 10.0},
            KeyCode::D => {translation.x += 10.0},
            KeyCode::W => {translation.y += 10.0},
            KeyCode::S => {translation.y -= 10.0},
            _ => {}
            },
            None => {}
          }
        }
      }
      ButtonState::Released => {}
    }
  }
  

}

// endregion: --- PlayerPlugin systems


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
  fn build(&self, app: &mut App){
    app
    .add_startup_system(setup)
    // .add_system(player_movement_system)
    .add_system(keyboard_input);
  }
}



fn main(){
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .run();
}
