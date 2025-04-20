
use bevy::{prelude::*,
    math::bounding::{Aabb2d,IntersectsVolume,BoundingVolume}};

use crate::player::{self,  *};
use crate::map::*;


#[derive(Component)]
pub struct Collider;







pub fn collision_system(
    mut player: Single<(&Transform,&mut Player),With<Player>>, 
    mut colliders: Query<(&Transform,Option<&Wall>),With<Collider>>,
    // mut collision_events: EventWriter<CollisionEvent>,
)
{
    let mut collis = false;
    let (player_transform,mut player) = player.into_inner();
    for (mut collider_transform,wall) in &colliders {


        let player_aabb = Aabb2d{
            min: Vec2 { x: player_transform.translation.truncate()[0] - player_transform.scale.truncate()[0] * 8., y: player_transform.translation.truncate()[1] - player_transform.scale.truncate()[1] * 11.0 },
            max: Vec2 { x: player_transform.translation.truncate()[0] + player_transform.scale.truncate()[0] * 8. , y: player_transform.translation.truncate()[1] + player_transform.scale.truncate()[1] * 8.5 },
        };
        let collider_aabb = Aabb2d::new(
            collider_transform.translation.truncate(),
            collider_transform.scale.truncate() * 8.,
        );
        
        let collision = player_aabb.intersects(&collider_aabb);
        



        if collision
        {
            collis = true;


            let penetration_x = (player_aabb.max.x.min(collider_aabb.max.x) - player_aabb.min.x.max(collider_aabb.min.x)).abs();
    
            let penetration_y = (player_aabb.max.y.min(collider_aabb.max.y) - player_aabb.min.y.max(collider_aabb.min.y)).abs();
            


            if penetration_x < penetration_y {
                // X轴碰撞
                if player_aabb.center().x < collider_aabb.center().x {
                    player.collision_right = true;
                } else {
                    player.collision_left = true;
                }
            } else {
                // Y轴碰撞（优先处理地面/天花板碰撞）
                if player_aabb.center().y > collider_aabb.center().y {
                    player.collision_bottom = true;
                } else {
                    player.collision_top = true;
                }
            }
            
           
        }
    }
    if collis == false{
        player.collision_left = false;
        player.collision_right = false;
        player.collision_top = false;
        player.collision_bottom = false;
    }



}




// fn player_collison(player:Aabb2d,bounding_box:Aabb2d) -> bool
// {
//     if !player.intersects(&bounding_box)
//     {
//         return false;
//     }
//     println!("{:?}    {:?}",bounding_box.min,bounding_box.max);
//     println!("{:?}    {:?}",player.min,player.max);
//     true
// }
