use::bevy::prelude::*;

use crate::gravity::gravity::Gravity;
use crate::movement::movement::{Body, Movement};
use crate::player::player::Player;
use crate::points::points::Points;

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEvent>();
        app.add_systems(Update, (despawn_on_terrain_touch, check_if_touch_player));
    }
}

const KNIFE_SPRITE_PATH: &str = "sprites/knife.png";

#[derive(Bundle)]
struct KnifeBundle {
    knife: Knife,
    sprite: SpriteBundle,
    movement: Movement
}

#[derive(Component)]
struct Knife {
    pub damage: f32
}

#[derive(Event)]
pub struct PlayerHitEvent {
    pub damage: f32
}

pub fn spawn_knife(mut commands: Commands, asset_server: Res<AssetServer>, spawn_position: Vec3) {

    commands.spawn( KnifeBundle {
        knife: Knife { damage: 1.0 },
        sprite: SpriteBundle {
            texture: asset_server.load(KNIFE_SPRITE_PATH),
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform {
                scale: Vec3::splat(4.0),
                translation: spawn_position,
                rotation: Quat::from_rotation_z(std::f32::consts::PI),
                ..default()
            },
            ..default()
        },
        movement : Movement {
            gravity: Gravity {
                is_touching_terrain: false
            },
            body: Body {
                mass: 50.0,
                velocity: Vec2::ZERO
            }
        }
    });
}

fn despawn_on_terrain_touch(mut commands: Commands, knife_query: Query<(Entity, &Movement), With<Knife>>, mut points: ResMut<Points>) {
    for (knife, &ref movement) in knife_query.iter() {
        if movement.gravity.is_touching_terrain {
            commands.entity(knife).despawn();
            points.value += 1;
        }
    }
}

fn check_if_touch_player(mut commands: Commands, mut ev_player_touched: EventWriter<PlayerHitEvent>, knife_query: Query<(&Movement, &Transform, &Sprite, &Knife, Entity)>, player_query: Query<(&Transform, &Sprite, &Player)>) {

    if let Ok((player_transform, player_sprite, _player)) = player_query.get_single() {
        let player_half_size = player_sprite.custom_size.unwrap() * player_transform.scale.truncate() / 2.0;

        for (_movement, transform, sprite, knife, entity) in knife_query.iter() {
            let knife_half_size = sprite.custom_size.unwrap() * transform.scale.truncate() / 2.0;
            
            let distance = transform.translation - player_transform.translation;

            if distance.x.abs() < knife_half_size.x + player_half_size.x &&
                       distance.y.abs() < knife_half_size.y + player_half_size.y {
                        ev_player_touched.send(PlayerHitEvent { damage: knife.damage });
                        commands.entity(entity).despawn();
                    }
        }
    }
}