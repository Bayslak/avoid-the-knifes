use::bevy::prelude::*;

use crate::{points::points::Points, GameState};


pub struct UIPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for UIPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui.run_if(in_state(self.state.clone())));
        app.add_systems(Update, update_points_ui.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct PointsText;

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }, Name::new("UI Root"),
    ))
    .with_children(|commands| {
        commands.spawn((TextBundle {
            text: Text::from_section("Points", TextStyle {
                font_size: 32.0,
                ..default()
            }),
            ..default()
        }, PointsText));
    });
}

fn update_points_ui(mut texts: Query<&mut Text, With<PointsText>>, points: Res<Points>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Points: {:?}", points.value);
    }
}