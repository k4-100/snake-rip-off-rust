use bevy::{
  prelude::*, 
  input::keyboard::*,
  window::*
};


mod components;

pub const CLEAR: Color = Color::rgb(0.1, 0.1,0.1);


// region: --- PlayerPlugin systems

fn setup(mut commands: Commands ){
    commands.spawn(Camera2dBundle::default());
    // Rectangle
    commands.spawn((
    components::Name("square1".to_string()), 
    components::Player,
    components::Velocity{x:10.0,y:10.0},
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

            KeyCode::A => {translation.x -= velocity.x},
            KeyCode::D => {translation.x += velocity.x},
            KeyCode::W => {translation.y += velocity.y},
            KeyCode::S => {translation.y -= velocity.y},
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
    .add_system(keyboard_input);
  }
}




fn main(){
  App::new()
    .insert_resource(ClearColor(CLEAR))
    .add_plugins(DefaultPlugins.set(
      WindowPlugin { 
        window: WindowDescriptor{
          title: "Snake rip-off".to_string(),
          width: 1200.0,
          height: 800.0,
          resizable: false,
          ..default()
        }, 
        add_primary_window: true,
        exit_on_all_closed: true,
        close_when_requested: true
      }
    ))
    .add_plugin(PlayerPlugin)
    .run();
}
