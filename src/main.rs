use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity(Vec3);

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Circle Example".to_string(),
                resolution: (600.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }),
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 22.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: Color::srgb(0.0, 1.0, 0.0),
                // We can also set the refresh interval for the FPS counter
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
            },
        },
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, (player_input, player_movement));
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0)));

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
fn player_input(
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
fn player_movement(
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
