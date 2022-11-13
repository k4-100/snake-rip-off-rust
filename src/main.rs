use bevy::prelude::*;


#[derive(Component)]
struct ShapeComponent;
#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Velocity{
  x: f32,
  y: f32
}

impl Velocity {
  fn new(x:f32, y:f32) -> Velocity{
    Velocity{
      x,
      y
    }
  }
}

#[derive(Component)]
struct Player;


fn setup(mut commands: Commands ){
    commands.spawn(Camera2dBundle::default());
    // Rectangle
    commands.spawn((
    Name("square1".to_string()), 
    Player,
    Velocity::new(10.0,0.0),
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

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>){
  for (velocity, mut transform) in query.iter_mut(){
    let translation = &mut transform.translation;
    translation.x += velocity.x;
    translation.y += velocity.y;
  }
}

fn main(){
  App::new()
    .add_startup_system( setup )
    .add_system( player_movement_system )
    .add_plugins(DefaultPlugins)
    .run();
}
