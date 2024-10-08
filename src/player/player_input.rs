use::bevy::prelude::*;

use crate::GameState;

pub struct InputPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for InputPlugin<GameState> {
    fn build(&self, app: &mut App) {
       app.add_event::<MovementInputEvent>();
       app.add_systems(Update, movement_input.run_if(in_state(self.state.clone()))); 
    }
}

pub enum InputDirection {
    Left,
    Right,
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
            KeyCode::ArrowLeft => input_direction = InputDirection::Left,
            KeyCode::KeyD => input_direction = InputDirection::Right,
            KeyCode::ArrowRight => input_direction = InputDirection::Right,
            _ => input_direction = InputDirection::None
        }
    }

    ev_movement.send(MovementInputEvent { direction: input_direction });
}