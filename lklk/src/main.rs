#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;


pub mod main_camera;
pub mod sprite_player;
pub mod sprite;
pub mod map;
pub mod player;
use main_camera::MainCameraPlugin;
use sprite::sprite_move::*;
use map::map_first::*;
fn main()  {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(MainCameraPlugin)
        .add_plugins(SpriteMovePlugin)
        .add_plugins(MapFirstPlugin)
        .run();
    
}






