use::bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Update, movement_input); 
    }
}

pub enum InputDirection {
    Left,
    Right,
    Up,
    Down,
    None
}

#[derive(Event)]
pub struct MovementInputEvent {
    pub direction: InputDirection
}

fn movement_input(mut ev_movement: EventWriter<MovementInputEvent>, input: Res<ButtonInput<KeyCode>>) {
    
    let mut input_direction = InputDirection::None;

    for key in input.get_pressed() {
        match key {
            KeyCode::KeyA => input_direction = InputDirection::Left,
            KeyCode::KeyD => input_direction = InputDirection::Right,
            _ => input_direction = InputDirection::None
        }
    }

    ev_movement.send(MovementInputEvent { direction: input_direction });
}