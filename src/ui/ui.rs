use::bevy::prelude::*;

use crate::{points::points::Points, CleanupGameStateExit, GameState, Level};


pub struct UIPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for UIPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_game_ui.run_if(in_state(self.state.clone())));
        app.add_systems(Update, (update_points_ui, update_level_ui).run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct PointsText;

#[derive(Component)]
pub struct LevelText;

fn spawn_game_ui(mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }, Name::new("UI Root"),
    ))
    .with_children(|commands| {
        commands.spawn((TextBundle {
            style: Style {
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            text: Text::from_section("Points", TextStyle {
                font_size: 32.0,
                ..default()
            }),
            ..default()
        }, PointsText));

        commands.spawn((TextBundle {
            text: Text::from_section("Level", TextStyle {
                font_size: 32.0,
                ..default()
            }),
            ..default()
        }, LevelText));

    }).insert(CleanupGameStateExit);
}

fn update_points_ui(mut texts: Query<&mut Text, With<PointsText>>, points: Res<Points>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Points: {:?}", points.value);
    }
}

fn update_level_ui(mut text: Query<&mut Text, With<LevelText>>, level: Res<Level>) {
    for mut text in &mut text {
        text.sections[0].value = format!("Level: {:?}", level.value);
    }
}