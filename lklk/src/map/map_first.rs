
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


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
        Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
        RigidBody::Fixed,
        Collider::cuboid(50.0, 30.0),
        Restitution::coefficient(0.7),
        GlobalTransform::default(),
        Wall
        )
    );

}