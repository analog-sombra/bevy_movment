use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

use crate::player::player::{player_input, player_movement, spawn_player};

use crate::camera::camera::spawn_camera;

mod camera;
mod player;

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

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_player(&mut commands, &mut meshes, &mut materials);
}
