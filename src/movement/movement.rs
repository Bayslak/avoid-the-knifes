use bevy::math::VectorSpace;
use::bevy::prelude::*;

use crate::{gravity::gravity::Gravity, GameState};

pub struct MovementPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for MovementPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_entity, check_direction).run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct Movement {
    pub gravity: Gravity,
    pub body: Body
}

#[derive(Component)]
pub struct Body {
    pub mass: f32,
    pub velocity: Vec2,
    pub direction: i8
}

impl Default for Body {
    fn default() -> Self {
        Self { mass: 100.0, velocity: Vec2::ZERO, direction: 1 }
    }
}

fn move_entity(mut movement_query: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in movement_query.iter_mut() {
        transform.translation.x += movement.body.velocity.x * time.delta_seconds();

        if movement.gravity.is_touching_terrain == false {
            transform.translation.y += movement.body.velocity.y * time.delta_seconds();
        }
    }
}

fn check_direction(mut movement_query: Query<(&mut Movement, &mut Sprite)>) {
    for (mut movement, mut sprite) in &mut movement_query {
        
        if movement.body.velocity != Vec2::ZERO {

            // Check direction by velocity
            if movement.body.velocity.x < 0.0 {
                movement.body.direction = -1;
            }

            if movement.body.velocity.x > 0.0 {
                movement.body.direction = 1;
            }

            // Check flip by direction
            if movement.body.direction == -1 && sprite.flip_x == false {
                sprite.flip_x = true;
            }

            if movement.body.direction == 1 && sprite.flip_x == true {
                sprite.flip_x = false;
            }
        }
        
    }
}