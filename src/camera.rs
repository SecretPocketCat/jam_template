use bevy::prelude::*;

#[derive(Component)]
pub struct PrimaryCamera;

pub fn camera_plugin(app: &mut App) {
    app.add_startup_system(setup_camera);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PrimaryCamera));
}
