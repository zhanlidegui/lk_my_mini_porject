use bevy::prelude::*;


#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}


#[derive(Component,Deref,DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animation_sprite(
    time:Res<Time>,
    mut query:Query<(&mut AnimationIndices,&mut AnimationTimer,&mut Sprite)>,
)
{
    for (indices,mut timer,mut sprite) in &mut query {
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