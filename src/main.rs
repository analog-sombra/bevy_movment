use crate::camera::camera::spawn_camera;
use crate::gameui::menu::{delete_menu, spawn_menu};
use crate::player::player::{player_input, player_movement, spawn_player};
use bevy::winit::WinitSettings;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

mod camera;
mod gameui;
mod player;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

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
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    .insert_resource(WinitSettings::desktop_app())
    .init_state::<AppState>()
    // We can add systems to trigger during transitions
    .add_systems(OnEnter(AppState::MainMenu), spawn_menu)
    .add_systems(OnExit(AppState::MainMenu), delete_menu)
    .add_systems(
        OnEnter(AppState::InGame),
        setup.run_if(in_state(AppState::InGame)),
    )
    .add_systems(
        Update,
        (player_input, player_movement).run_if(in_state(AppState::InGame)),
    );

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
