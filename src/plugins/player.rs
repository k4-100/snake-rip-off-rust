use bevy::{
  prelude::*, 
  input::keyboard::*
};
use rand::prelude::*;

// mod super::super::utils;

pub use super::super::utils::{
  components,
  resources
};


// region: --- PlayerPlugin itself
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
  fn build(&self, app: &mut App){
    app
    .add_startup_system(setup)
    .insert_resource(resources::PreviousPosition(Vec3 { x: 0., y: 0., ..default() }))
    .insert_resource(resources::Score(0))
    .add_system(keyboard_input)
    .add_system(check_intersection)
    ;
  }
}

// endregion: --- PlayerPlugin itself



// region: --- PlayerPlugin systems

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){


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

  

  // food entity
  commands.spawn(
    (
        components::Food, 
        SpriteBundle {
          sprite: Sprite {
            color: Color::rgb(1.,0.5,0.0),
            custom_size: Some(Vec2::new(90., 90.)),
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

    commands.spawn(
      (
        TextBundle::from_section(
          "Score: 0", 
          TextStyle { 
            font: asset_server.load("fonts/Ubuntu-M.ttf"), 
            font_size: 50., 
            color: Color::WHITE 
          }
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.0),
                left: Val::Px(290.0),
                ..default()
            },
            ..default()
        }),
      )
    );

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

fn get_random_food_position() -> Vec3{
    let mut rng = rand::thread_rng();
    let x: i16 = rng.gen_range(-1400..=1400) / 100 * 100;
    let y: i16 = rng.gen_range(-1400..=1400) / 100 * 100;

    return Vec3{ 
      x: x as f32, 
      y: y as f32, 
      ..default()
    }
}


pub fn check_intersection( 
  // block_query: 
  // mut food_query: Query<&mut Transform, With<components::Food>>,
  // mut commands: Commands
  mut set: ParamSet<(
    Query<(&components::Player, &components::HitBox, &Transform)>, 
    Query<&mut Transform, With<components::Food>>,
    Commands,
    Query<(&components::Block, &Transform)>, 
  )>,
  mut text_query: Query<&mut Text>,
  mut score: ResMut<resources::Score>

){

  let mut player: Option<(components::Player, components::HitBox, Transform)> = None;
  // for (player_type, player_hitbox_local, player_transform) in set.p0().iter(){
    for player_local in set.p0().iter(){
      if *player_local.0 == components::Player::Head{
        player = Some( (*player_local.0, *player_local.1, *player_local.2) );
      }
    }
    
    // food collision
    let mut food = *set.p1().single();
    let mut intersected_food : bool = false;
      match player {
        Some( pl ) => {
          if pl.1.intersects_point(&food.translation){
            println!("intersects_food {:?}", food.translation );
            intersected_food = true;
          }
        },
        None => {

        }
      }

      // if player.1.intersects_point(&food.translation){


      // }



      let mut player_tail: Option<(components::Player, components::HitBox, Transform)> = None;
      match set.p0().iter().last() {
        Some(pl) =>{
          player_tail = Some( (*pl.0, *pl.1, *pl.2) );
        },
        None => {}
      }




      if intersected_food{
        match player_tail{
          Some(last_body_part) =>{
            let last_body_part_translation = &last_body_part.2.translation;
            match player{
              Some(pl) =>{
                if pl.2.translation.y > last_body_part_translation.y {
                  // println!("PUSH DOWN");
                  push_body_part( &mut set.p2(), last_body_part_translation, Vec3{x:0., y:-100., ..default()});
                }
                else if pl.2.translation.y < last_body_part_translation.y {
                  // println!("PUSH UP");
                  push_body_part( &mut set.p2(), last_body_part_translation, Vec3{x:0., y:100., ..default()});
                }
                else if pl.2.translation.x > last_body_part_translation.x {
                  // println!("PUSH LEFT");
                  push_body_part( &mut set.p2(), last_body_part_translation, Vec3{x:-100., y:0., ..default()});
                }
                else if pl.2.translation.x < last_body_part_translation.x {
                  // println!("PUSH RIGHT");
                  push_body_part( &mut set.p2(), last_body_part_translation, Vec3{x:100., y:0., ..default()});
                }

                food.translation = get_random_food_position();
                score.0 += 1;
                let mut text = text_query.single_mut();
                text.sections[0].value = format!("Score: {}", score.0);
              },
              None =>{}
            }
            
            
          },
          None => panic!("ERROR: No final item")
        }
      }
    
    // let food_query = set.p1().iter().last().unwrap();
    // if &food != food_query{

    // }


    for mut food_query in set.p1().iter_mut().last(){
      // println!("{:?} || {:?}", food.translation, food_query.translation);
      if food.translation != food_query.translation{
        println!("##########################\nFOOD MOVED\n##################");
        food_query.translation = food.translation;
      }
    }

    // block collision
    // for (_, transform) in block_query.iter(){
    //   // if hitbox_player.intersects_point(&transform.translation){
    //   //   println!("intersects_block {:?}", transform.translation );
    //   // }
    // }


}


// endregion: --- PlayerPlugin systems