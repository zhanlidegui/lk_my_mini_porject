
use bevy::prelude::*;
use crate::sprite::*;

use super::Wall;
#[derive(Component)]
pub struct MapFirstPlugin;

impl Plugin for MapFirstPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,setup);
    }
}

fn setup(mut commands:Commands,asset_server:Res<AssetServer>)
{
    
    let rpg_slice = asset_server.load("textures/rpg/tiles/generic-rpg-Slice.png");
    
    commands.spawn(
        (
        Sprite::from_image(rpg_slice.clone()),
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        collision::Collider,
        Wall
        )
    );

}