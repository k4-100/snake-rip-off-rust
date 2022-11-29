use bevy::{
  prelude::*
};

mod plugins;
mod utils;

pub use crate::plugins::player::PlayerPlugin;


pub const CLEAR: Color = Color::rgb(0.1, 0.1,0.1);



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
