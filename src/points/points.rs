use::bevy::prelude::*;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Points { value: 0 });
    }
}

#[derive(Resource)]
pub struct Points {
    pub value: i32
}