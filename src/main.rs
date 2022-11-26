use bevy::{
  prelude::*, 
  input::keyboard::*
};


mod components;
mod resources;


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

    let body_batch: Vec<(components::Player, components::HitBox, components::Velocity, SpriteBundle)> = (0..=5).map( |x: u32|{
      println!("x: {}",x);
      let size = Vec2::new(95.,95.);
      let translation = Vec3{
        x:0.,
        y:0. - (100. * x as f32),
        ..default()
      };

      (
        if x == 0 { components::Player::Head } else { components::Player::Body } ,
        components::HitBox::from_translation(size.x, size.y, &translation),
        components::Velocity{x: 100., y: 100.},
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(
                0.25,
                0.95, 
                if x == 0 { 0.9 } else { 0.1} ,
                ),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform{
              translation,
              ..default()
            },
            ..default()
          }
      )
    }).collect();

    // player's body
    commands.spawn_batch( body_batch );



  const REMAINDER_DIVIDER: u32 = 30;
  let mut position : (f32,f32) = (0., 0.);
  let block_batch: Vec<(components::Block, components::HitBox, SpriteBundle)> = (0..120).map( 
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
        components::HitBox::from_translation(100., 100., &Vec3 { x: position.0, y: position.1, ..default() }),
        SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(1.,0.,0.2),
          custom_size: Some(Vec2::new(90., 90.)),
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

  //blocks
  commands.spawn_batch( block_batch);

  

  // let food_batch: Vec<(components::Food, SpriteBundle)> = (0..=1).map(
  //   |_| {
  //     (
  //       components::Food, 
  //       SpriteBundle {
  //         sprite: Sprite {
  //           color: Color::rgb(1.,0.5,0.0),
  //           custom_size: Some(Vec2::new(100., 100.)),
  //           ..default()
  //         },
  //         transform: Transform{
  //           translation: Vec3 { 
  //             x: 400.,
  //             y: 400.,
  //             ..default() 
  //           },
  //           ..default()
  //         },
  //         ..default()
  //       }
  //     )
  //   }
  // ).collect();

  // food batch
  commands.spawn(
    (
        components::Food, 
        SpriteBundle {
          sprite: Sprite {
            color: Color::rgb(1.,0.5,0.0),
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
          },
          transform: Transform{
            translation: Vec3 { 
              x: 400.,
              y: 400.,
              ..default() 
            },
            ..default()
          },
          ..default()
        }
    )
    );

    // commands.re
    

}


fn keyboard_input( 
  mut key_evr: EventReader<KeyboardInput>, 
  mut query: Query<(&components::Player, &components::Velocity, &mut components::HitBox, &mut Transform)>, mut previous_position: ResMut<resources::PreviousPosition>
){
  use bevy::input::ButtonState;


  for ev in key_evr.iter(){
    match ev.state{
      ButtonState::Pressed => {
        let query_iter_mut = query.iter_mut();
        for( player, velocity, mut hitbox, mut transform) in query_iter_mut{
          if  components::Player::Head == *player{  
            *previous_position = resources::PreviousPosition( transform.translation.clone() );
            let prev_translation= transform.translation.clone();
            let translation = &mut transform.translation;
            match ev.key_code{
              Some(x) =>  match x{
                KeyCode::A => translation.x -= velocity.x,
                KeyCode::D => translation.x += velocity.x,
                KeyCode::W => translation.y += velocity.y,
                KeyCode::S => translation.y -= velocity.y,
                _ => {}
              },
              None => {}
            }
            if prev_translation != translation.clone() {
                *hitbox = components::HitBox::from_translation(100., 100.,  &Vec3 { x: translation.x, y: translation.y, z: translation.z });
            }
          }else{
            let previous_position_copy = previous_position.0;
            *previous_position = resources::PreviousPosition( transform.translation.clone() );
            let translation = &mut transform.translation;
            *translation = previous_position_copy;



          }
        }
      }
      ButtonState::Released => {}
    }
  }
  

}


fn push_body_part( commands: &mut Commands, last_body_part_translation: &Vec3, push_direction: Vec3 ){
  commands.spawn(
  ( 
      components::Player::Body,
      components::HitBox::from_translation(100., 100., &Vec3{x: 0., y: 0., ..default()}),
      components::Velocity{x: 100., y: 100.},
      SpriteBundle {
          sprite: Sprite {
              color: Color::rgb(
              0.25,
              0.95, 
              0.1 ,
              ),
              custom_size: Some(Vec2{x: 90., y:90.}),
              ..default()
          },
          transform: Transform{
            // translation: Vec3{x:0., y:0., z: 0.1},
            translation: *last_body_part_translation + push_direction,
            ..default()
          },
          ..default()
        }
    )
);
}

pub fn check_intersection( 
  player_query: Query<(&components::Player, &components::HitBox, &Transform)>, 
  block_query: Query<(&components::Block, &Transform)>, 
  food_query: Query<(&components::Food, &Transform)>,
  mut commands: Commands
){
  for (player_type, hitbox_player, player_transform) in player_query.iter(){
    if *player_type == components::Player::Head{

      // food collision
      for food in food_query.iter(){

        if hitbox_player.intersects_point(&food.1.translation){

          println!("intersects_food {:?}", food.1.translation );

          match player_query.iter().last(){
            Some(last_body_part) =>{
              let last_body_part_translation = &last_body_part.2.translation;
              if player_transform.translation.y > last_body_part_translation.y {
                println!("PUSH DOWN");
                push_body_part( &mut commands, last_body_part_translation, Vec3{x:0., y:-100., ..default()});
                return;
              }
              if player_transform.translation.y < last_body_part_translation.y {
                println!("PUSH UP");
                push_body_part( &mut commands, last_body_part_translation, Vec3{x:0., y:100., ..default()});
                return;
              }
              if player_transform.translation.x > last_body_part_translation.x {
                println!("PUSH LEFT");
                push_body_part( &mut commands, last_body_part_translation, Vec3{x:-100., y:0., ..default()});
                return;
              }
              if player_transform.translation.x < last_body_part_translation.x {
                println!("PUSH RIGHT");
                push_body_part( &mut commands, last_body_part_translation, Vec3{x:100., y:0., ..default()});
              }
              
              
            },
            None => panic!("ERROR: No final item")
          }
        }
      }
      // block collision
      for (_, transform) in block_query.iter(){
        if hitbox_player.intersects_point(&transform.translation){
          println!("intersects_block {:?}", transform.translation );
        }
      }
    }
  }
}


// endregion: --- PlayerPlugin systems


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
  fn build(&self, app: &mut App){
    app
    .add_startup_system(setup)
    .insert_resource(resources::PreviousPosition(Vec3 { x: 0., y: 0., ..default() }))
    .add_system(keyboard_input)
    .add_system(check_intersection)
    ;
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
