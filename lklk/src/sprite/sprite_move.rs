

use bevy::prelude::*;

use crate::sprite_player::*;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    
}
fn sprite_move(time:Res<Time>,mut sprite_position:Query<(&mut Direction,&mut Transform,&mut Sprite)>)
{
    for (mut logo,mut transform,mut sprite) in &mut sprite_position 
    {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
            Direction::Right => transform.translation.x += 150. * time.delta_secs(),
        }
        if transform.translation.x > 400. {
            *logo = Direction::Left;
            sprite.flip_x = true;
        }else if transform.translation.x < -400. {
            *logo = Direction::Right;
            sprite.flip_x = false;
        }

        if transform.translation.y > 400. {
            transform.translation.y = -400.;
        }else if transform.translation.y < -400. {
            transform.translation.y = 400.;
        }
    }
}
    



pub struct SpriteMovePlugin;

impl Plugin for SpriteMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animation_sprite)
            .add_systems(Update,sprite_move);
    }
}



fn setup(mut commands: Commands,asset_server:Res<AssetServer>,mut texture_atlas_layouts:ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layouts = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        first: 1,
        last: 6,
    };

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas{
                layout:texture_atlas_layouts,
                index:animation_indices.first,
            }
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Direction::Right,
    ));
}


#[derive(Debug,Component,Clone,Copy,PartialEq,Default,Deref,DerefMut)]
struct AccumulatedInput(Vec2);

#[derive(Debug,Component,Clone,Copy,PartialEq,Default,Deref,DerefMut)]
struct Velocity(Vec3);

#[derive(Debug,Component,Clone,Copy,PartialEq,Default,Deref,DerefMut)]
struct PhysicalTranslation(Vec3);

#[derive(Debug,Component,Clone,Copy,PartialEq,Default,Deref,DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

fn spawn_player(mut commands:Commands,asset_server:Res<AssetServer>)
{
    commands.spawn(Camera2d);
    
}
