#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;



pub mod sprite_player;
pub mod sprite;
use sprite::sprite_move::*;

fn main()  {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpriteMovePlugin)
        .run();
    
}




