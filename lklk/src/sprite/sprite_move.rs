

use bevy::input::keyboard;
use bevy::prelude::*;

use bevy_rapier2d::prelude::*;


use crate::player;
use crate::sprite_player::*;

use crate::player::*;

// #[derive(Component)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
    
// }
// fn sprite_move(time:Res<Time>,mut sprite_position:Query<(&mut Direction,&mut Transform,&mut Sprite)>)
// {
//     for (mut logo,mut transform,mut sprite) in &mut sprite_position 
//     {
//         match *logo {
//             Direction::Up => transform.translation.y += 150. * time.delta_secs(),
//             Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
//             Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
//             Direction::Right => transform.translation.x += 150. * time.delta_secs(),
//         }
//         if transform.translation.x > 400. {
//             *logo = Direction::Left;
//             sprite.flip_x = true;
//         }else if transform.translation.x < -400. {
//             *logo = Direction::Right;
//             sprite.flip_x = false;
//         }

//         if transform.translation.y > 400. {
//             transform.translation.y = -400.;
//         }else if transform.translation.y < -400. {
//             transform.translation.y = 400.;
//         }
//     }
// }
    


pub struct SpriteMovePlugin;

impl Plugin for SpriteMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            // .add_systems(Update, handle_movement_state)
            .add_systems(Update, animation_sprite)
            .add_systems(
                FixedUpdate, 
                // advance_physics,
                player_move
            )
            // .add_systems(RunFixedMainLoop,
            // (
            //     handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
            //     interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            // ))
            ;
    }
}



fn setup(mut commands: Commands,asset_server:Res<AssetServer>,mut texture_atlas_layouts:ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layouts = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        zero: 0,
        first: 1,
        last: 6,
    };
    let player = Player{
        move_speed: 1.0,
        move_state:false,
        collision_bottom:false,
        collision_left:false,
        collision_right:false,
        collision_top:false
    };
    // commands.spawn(Camera2d);
    commands.spawn((
        Name::new("Player"),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas{
                layout:texture_atlas_layouts,
                index:animation_indices.first,
            }
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        player,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),

        RigidBody::KinematicPositionBased,
        KinematicCharacterController{
            offset:CharacterLength::Absolute(0.01),
            ..default()
        },
        Collider::cuboid(10.0, 10.0),
        
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




// fn handle_input(
//     keyboard_input:Res<ButtonInput<KeyCode>>,
//     mut query:Query<(&mut AccumulatedInput,&mut Velocity,&mut Sprite,&mut Player)>,
// )
// {
    
//     const SPEED:f32 = 210.0;
//     for(mut input,mut velocity,mut sprite,mut player) in query.iter_mut()
//     {
//         let any_movement_key_pressed = keyboard_input.any_pressed([
//             KeyCode::KeyW,
//             KeyCode::KeyS,
//             KeyCode::KeyA,
//             KeyCode::KeyD
//         ]);
//         player.move_state = any_movement_key_pressed;
//         if keyboard_input.pressed(KeyCode::Space) {
//             player.move_speed = 2.0;
//         }else {
//             player.move_speed = 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::KeyW) && player.collision_top != true {
//             input.0.y += 1.0;
//         }else if player.collision_top == true {
//             input.0.y -= 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::KeyS) && player.collision_bottom != true {
//             input.0.y -= 1.0;
//         }else if player.collision_bottom == true {
//             input.0.y += 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::KeyA) && player.collision_left != true {
//             input.0.x -= 1.0;
//             sprite.flip_x = true;
//         }else if player.collision_left == true {
//             input.0.x += 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::KeyD) && player.collision_right != true {
//             input.0.x += 1.0;
//             sprite.flip_x = false;
//         }else if player.collision_right == true {
//             input.0.x -= 1.0;
            
//         }
//         velocity.0 = input.extend(0.0).normalize_or_zero() * SPEED * player.move_speed;
//     }
// }

// fn advance_physics(
//     fixed_time:Res<Time<Fixed>>,
//     mut query:Query<(
//         &mut PhysicalTranslation,
//         &mut PreviousPhysicalTranslation,
//         &mut AccumulatedInput,
//         &Velocity,
//     )>,
// )
// {
//     for(mut current_physical_translation,
//         mut previous_physical_translation,
//         mut input,
//         velocity,
//     ) in query.iter_mut()
//     {
//         previous_physical_translation.0 = current_physical_translation.0;
//         current_physical_translation.0 += velocity.0 *fixed_time.delta_secs();

//         input.0 = Vec2::ZERO;
//     }
// }

// fn interpolate_rendered_transform(
//     fixed_time:Res<Time<Fixed>>,
//     mut query:Query<(
//         &mut Transform,
//         &PhysicalTranslation,
//         &PreviousPhysicalTranslation
//     )>
// )
// {
//     for (mut transform,
//         current_physical_translation,
//         previous_physical_translation
//     ) in query.iter_mut()
//     {
//         let previous = previous_physical_translation.0;
//         let current = current_physical_translation.0;

//         let alpha = fixed_time.overstep_fraction();

//         let rendered_translation = previous.lerp(current,alpha);

//         transform.translation = rendered_translation;
//     }
// }



fn player_move(
    mut controller:Query<&mut KinematicCharacterController>,
    mut player:Query<(&mut Sprite, &mut Player), With<Player>>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
)
{
    let any_movement_key_pressed = keyboard_input.any_pressed([
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyA,
        KeyCode::KeyD
    ]);
    let (mut sprite, mut player) = player.get_single_mut().unwrap();
    player.move_state = any_movement_key_pressed;

    if keyboard_input.pressed(KeyCode::Space) {
        player.move_speed = 4.0;
    }else {
        player.move_speed = 2.0;
    }

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        sprite.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        sprite.flip_x = false;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize() * player.move_speed;
    }

    for mut controller in &mut controller {
        controller.translation = Some(direction);
    }
}
