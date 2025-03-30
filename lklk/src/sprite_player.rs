use bevy::prelude::*;

use crate::player:: *;


#[derive(Component)]
pub struct AnimationIndices {
    pub zero: usize,
    pub first: usize,
    pub last: usize,
}


#[derive(Component,Deref,DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animation_sprite(
    time:Res<Time>,
    mut query:Query<(&mut AnimationIndices,&mut AnimationTimer,&mut Sprite,&Player)>,
)
{
    for (indices,mut timer,mut sprite,player) in &mut query {

        if player.move_state == false {
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = indices.zero;
            }
            timer.reset();
            continue;
        }

        timer.tick(time.delta());

        if timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = if atlas.index == indices.last{
                    indices.first
                }else{
                    atlas.index + 1
                };
            }
        }
    }
}

