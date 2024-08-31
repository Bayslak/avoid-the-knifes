use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resizable: true,
                            focused: true,
                            resolution: (WW, WH).into(),
                            ..default()
                        }),
                        ..default()
                    }),
            )
    .insert_resource(Msaa::Off)
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(FrameTimeDiagnosticsPlugin)
    .add_systems(Startup, setup) 
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}