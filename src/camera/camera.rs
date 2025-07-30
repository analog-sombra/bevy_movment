use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0), MainCamera));
}
