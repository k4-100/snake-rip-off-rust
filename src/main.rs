use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct QuadComponent(shape::Quad);

fn setup(mut commands: Commands ){
    commands.spawn(Camera2dBundle::default());
    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });


}

fn main(){
  App::new()
    .add_startup_system( setup )
    .add_plugins(DefaultPlugins)
    .run();
}
