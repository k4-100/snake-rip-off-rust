use bevy::{
  prelude::*, 
  input::keyboard::*
};


mod components;


pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

pub const CLEAR: Color = Color::rgb(0.1, 0.1,0.1);

// region: --- PlayerPlugin systems

fn setup(mut commands: Commands ){


    commands.spawn(Camera2dBundle{
      transform: Transform{
        scale: Vec3{x: 5.0, y: 5.0, z: 1.0},
        ..default()
      },
      ..default()
    });
    // player
    commands.spawn((
    components::Name("square1".to_string()), 
    components::Player,
    components::Velocity{x:100.0,y:100.0},
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    })
  );
  const REMAINDER_DIVIDER: u32 = 30;
  let mut position : (f32,f32) = (0., 0.);
  let block_batch: Vec<(components::Block, SpriteBundle)> = (0..120).map( 
    |x: u32| {
      match x{
        0..=29 => position = (
             1500. + 0.0 * ( (x % REMAINDER_DIVIDER) as f32), 
             1500. - 100.0 * ( (x % REMAINDER_DIVIDER) as f32), 
        ),
        30..=59 => position = (
             1500. - 100.0 * ( (x % REMAINDER_DIVIDER) as f32), 
             -1500. + 0.0 * ( (x % REMAINDER_DIVIDER) as f32), 
        ),
        60..=89 => position = (
             -1500. + 0.0 * ( (x % REMAINDER_DIVIDER) as f32), 
             -1500. + 100.0 * ( (x % REMAINDER_DIVIDER) as f32), 
        ),
        90..=119 => position = (
             -1500. + 100.0 * ( (x % REMAINDER_DIVIDER) as f32), 
             1500. + 0.0 * ( (x % REMAINDER_DIVIDER) as f32), 
        ),
        _ => {}
      }

      (
        components::Block, 
        SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(1.,0.,0.2),
          custom_size: Some(Vec2::new(100., 100.)),
          ..default()
        },
        transform: Transform{
          translation: Vec3 { 
            x: position.0,
            y: position.1,
            ..default() 
          },
          ..default()
        },
        ..default()
      })
    }
).collect();

  commands.spawn_batch( block_batch);

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
          // scale_factor_override: Some(1.0),
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
