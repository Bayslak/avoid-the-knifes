use::bevy::prelude::*;

use crate::{movement::movement::Movement, terrain::terrain::Terrain, GameState};

pub struct GravityPlugin<GameState: States> {
    pub state: GameState
}

const GRAVITY_SCALE: f32 = 9.8;

impl Plugin for GravityPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_gravity, entity_is_touching_terrain).run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct Gravity {
    pub is_touching_terrain: bool
}

fn apply_gravity(mut movement_query: Query<&mut Movement>, time: Res<Time>) {
    for mut movement in &mut movement_query {
        movement.body.velocity.y -= movement.body.mass * GRAVITY_SCALE * time.delta_seconds();
    }
}

fn entity_is_touching_terrain(mut movement_query: Query<(&mut Movement, &Transform, &Sprite)>, terrain_query: Query<(&Transform, &Sprite, &Terrain)>) {

    if let Ok((terrain_transform, terrain_sprite, _terrain)) = terrain_query.get_single() {
        let terrain_half_size = terrain_sprite.custom_size.unwrap() * terrain_transform.scale.truncate() / 2.0;

        for (mut movement, transform, sprite) in movement_query.iter_mut() {
            let entity_half_size = sprite.custom_size.unwrap() * transform.scale.truncate() / 2.0;

            let distance = transform.translation - terrain_transform.translation;

            if distance.x.abs() < entity_half_size.x + terrain_half_size.x && 
               distance.y.abs() < entity_half_size.y + terrain_half_size.y {
                movement.gravity.is_touching_terrain = true;
            } else {
                movement.gravity.is_touching_terrain = false;
            }
        }
    }
}