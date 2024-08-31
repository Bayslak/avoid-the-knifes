use::bevy::prelude::*;
use bevy::render::render_resource::AsBindGroupShaderType;

use crate::{gravity::Gravity, movement::{self, Body, Movement}};

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_on_terrain_touch);
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

fn despawn_on_terrain_touch(mut commands: Commands, knife_query: Query<(Entity, &Movement), With<Knife>>) {
    for (knife, &ref movement) in knife_query.iter() {
        if movement.gravity.is_touching_terrain {
            commands.entity(knife).despawn();
        }
    }
}