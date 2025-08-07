use crate::{player::player::{InGameEntity, Player}, MyAssets};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Food;

pub fn spawn_food(commands: &mut Commands, assets: &MyAssets, window_w: f32, window_h: f32, player_query: &Query<&Transform, With<Player>>) {
    let apple_texture = assets.apple.clone();

    // Get player position
    let player_pos = if let Ok(player_transform) = player_query.single() {
        player_transform.translation
    } else {
        Vec3::ZERO // fallback if no player found
    };

    // Ensure food doesn't spawn on the border by adding a margin
    let margin = 32.0;
    let min_distance = 200.0;
    
    let mut x;
    let mut y;
    let mut attempts = 0;
    const MAX_ATTEMPTS: i32 = 100;
    
    // Keep trying to find a position that's far enough from the player
    loop {
        x = rand::random::<f32>() * (window_w - 2.0 * margin) - (window_w / 2.0) + margin;
        y = rand::random::<f32>() * (window_h - 2.0 * margin) - (window_h / 2.0) + margin;
        
        let food_pos = Vec3::new(x, y, 0.0);
        let distance = player_pos.distance(food_pos);
        
        attempts += 1;
        if distance >= min_distance || attempts >= MAX_ATTEMPTS {
            break;
        }
    }

    commands.spawn((
        Food,
        InGameEntity,
        Sprite::from_image(apple_texture),
        Transform::from_xyz(x, y, 10.0),
        Collider::rectangle(32., 32.),
    ));
}
