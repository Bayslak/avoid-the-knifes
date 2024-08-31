use std::cell::RefMut;

use bevy::{ecs::event, prelude::*};

use crate::{coin::CoinTouchedEvent, gravity::Gravity, knife::PlayerHitEvent, movement::{Body, Movement}, player_input::{InputDirection, MovementInputEvent}, points::Points, terrain::Terrain};

const PLAYER_SPRITE_PATH: &str = "sprites/skeleton.png";
const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (listen_movement_input, listen_for_knives));
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
    movement: Movement
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn(PlayerBundle {
        player: Player { speed: PLAYER_SPEED },
        sprite: SpriteBundle {
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
        movement: Movement {
            gravity: Gravity {
                is_touching_terrain: false
            },
            body: Body {
                mass: 100.0,
                velocity: Vec2::ZERO
            }
        }
    }).insert(Name::new("Player"));
}

fn listen_movement_input(mut ev_movement: EventReader<MovementInputEvent>, mut movement_query: Query<(&mut Movement, &Player)>) {
    
    if let Ok((mut movement, player)) = movement_query.get_single_mut() {
        for input_direction in ev_movement.read() {
            match input_direction.direction {
                InputDirection::Left => movement.body.velocity.x = -player.speed,
                InputDirection::Right => movement.body.velocity.x = player.speed,
                InputDirection::Up => todo!(),
                InputDirection::Down => todo!(),
                InputDirection::None => movement.body.velocity.x = 0.0,
            }
        }
    }
}

fn listen_for_knives(mut ev_player_hit: EventReader<PlayerHitEvent>) {
    for event in ev_player_hit.read() {
        println!("Ouch, we took {} damage.", event.damage);
    }
}

fn listen_for_coins(mut ev_coin_collected: EventReader<CoinTouchedEvent>, mut points: RefMut<Points>) {
    for event in ev_coin_collected.read() {
        points.value += event.value;
    }
}