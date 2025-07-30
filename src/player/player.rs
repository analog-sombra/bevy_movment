use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(Vec3);

#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct Player;

const SPEED: f32 = 5.0;

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let size = 30.0;
    let shape = Circle::new(size);
    let color = Color::srgb(0.0, 0.0, 1.0);
    commands.spawn((
        Player,
        Velocity::default(),
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(ColorMaterial::from(color))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}

const PADDLE_SPEED: f32 = 5.;
pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.single_mut() {
        velocity.0 = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            velocity.0.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            velocity.0.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y -= 1.0;
        }
    }
}
pub fn player_movement(
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2.0 - 30.0; // Assuming the paddle height is 60.0
        let min_y = -max_y;

        let window_width = window.resolution.width();
        let max_x = window_width / 2.0 - 30.0; // Assuming the paddle width is 60.0
        let min_x = -max_x;

        for (mut transform, velocity) in &mut query {
            let mut new_position = transform.translation + velocity.0 * PADDLE_SPEED;
            new_position.x = new_position.x.clamp(min_x, max_x);
            new_position.y = new_position.y.clamp(min_y, max_y);
            transform.translation = new_position;
        }
    }
}
