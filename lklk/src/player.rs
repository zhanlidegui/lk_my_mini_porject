use bevy::prelude::*;
#[derive(Component,Debug)]
pub struct Player{
    pub(crate) move_speed: f32,
    pub move_state: bool,
}
