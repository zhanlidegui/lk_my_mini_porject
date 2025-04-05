

use bevy:: prelude::*;
#[derive(Component,Debug)]
pub struct Player{
    pub(crate) move_speed: f32,
    pub move_state: bool,
    pub collision_left:bool,
    pub collision_right:bool,
    pub collision_top:bool,
    pub collision_bottom:bool
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}