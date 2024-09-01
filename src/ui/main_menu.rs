use bevy::prelude::*;

use crate::{main, CleanupMenuStateExit, GameState};

pub struct MainMenuPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for MainMenuPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
        app.add_systems(Update, (main_menu_buttons_pressed_system, main_menu_buttons_hovered_system).run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub enum MenuButton {
    Play,
    Quit
}

fn setup_menu(mut commands: Commands) {

    commands.spawn( NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
            // Play Button
            parent.spawn( ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn( TextBundle {
                    text: Text::from_section("Play",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            }),
                            ..default()
                });
            }).insert(MenuButton::Play);

            //Quit button
            parent.spawn( ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                }, 
                ..default()
            }).with_children(|parent| {
                parent.spawn( TextBundle {
                    text: Text::from_section("Quit",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            }),
                            ..default()
                });
            }).insert(MenuButton::Quit);

        }).insert(CleanupMenuStateExit); 
}


fn main_menu_buttons_pressed_system(
    mut interaction_query: Query<(&Interaction, &MenuButton), With<Button>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>) {

    for (interaction, menu_button) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            match menu_button {
                MenuButton::Play => {
                    app_state.set(GameState::Game);
                },
                MenuButton::Quit => {
                    exit.send(AppExit::Success);
                },
            }
        }
    }
}

fn main_menu_buttons_hovered_system(mut interaction_query: Query<(&Interaction, &mut Text), With<Button>>) {

    for (interaction, mut text) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                for section in &mut text.sections {
                    section.style.font_size = 45.0;
                }
            },
            Interaction::Hovered => {
                for section in &mut text.sections {
                    section.style.font_size = 50.0;
                }
            },
            Interaction::None => {
                for section in &mut text.sections {
                    section.style.font_size = 40.0;
                }
            },
        }
    }

}