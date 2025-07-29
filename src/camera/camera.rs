use bevy::prelude::*;

pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0)));
}
