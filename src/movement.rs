use::bevy::prelude::*;

use crate::gravity::Gravity;


pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_entity);
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
    pub velocity: Vec2
}

fn move_entity(mut movement_query: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in movement_query.iter_mut() {
        transform.translation.x += movement.body.velocity.x * time.delta_seconds();

        if movement.gravity.is_touching_terrain == false {
            transform.translation.y += movement.body.velocity.y * time.delta_seconds();
        }

    }
}
