use bevy::prelude::*;

use crate::terrain::{self, Terrain};

const PLAYER_SPRITE_PATH: &str = "sprites/skeleton.png";
const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_movement, player_is_touching_terrain));
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
    pub is_touching: bool,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE_PATH),
        sprite: Sprite {
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        transform: Transform {
            scale: Vec3::splat(4.0),
            ..default()
        },
        ..default()
    }, 
        Player { speed: PLAYER_SPEED, is_touching: false })).insert(Name::new("Player"));
}

fn player_movement(mut player: Query<(&mut Transform, &Player)>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>) {

    for (mut transform, player) in &mut player {

        if(player.is_touching) {
            if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
                transform.translation.x += player.speed * time.delta_seconds();
            }
            
            if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
                transform.translation.x -= player.speed * time.delta_seconds();
            }
        } else {
            transform.translation.y -= 980.0 * time.delta_seconds();
        }

    }
}

fn player_is_touching_terrain(mut player_query: Query<(&mut Player, &Transform, &Sprite)>, terrain_query: Query<(&Transform, &Sprite, &Terrain), Without<Player>>,) {

    if let Ok((mut player, player_transform, player_sprite)) = player_query.get_single_mut() {

        if let Ok((terrain_transform, terrain_sprite, _terran)) = terrain_query.get_single() {
            let player_half_size = player_sprite.custom_size.unwrap() * player_transform.scale.truncate() / 2.0;
            let terrain_half_size = terrain_sprite.custom_size.unwrap() * terrain_transform.scale.truncate() / 2.0;
            
            let distance = player_transform.translation - terrain_transform.translation;
    
            if distance.x < player_half_size.x + terrain_half_size.x &&
               distance.y < player_half_size.y + terrain_half_size.y {
                player.is_touching = true;
                return; 
            }
    
            player.is_touching = false;
            return;
        }
        
        player.is_touching = false;
        return;
    }
}