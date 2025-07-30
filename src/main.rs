use crate::camera::camera::spawn_camera;
use crate::gameui::menu::{delete_menu, spawn_menu};
use crate::player::player::{player_input, player_movement, spawn_player};
use bevy::winit::WinitSettings;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_asset_loader::prelude::*;

mod camera;
mod gameui;
mod player;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    BootingApp,
    ErrorScreen,
    MainMenu,
    InGame,
    Paused,
}

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Resource, Default, AssetCollection)]
struct MyAssets {
    #[asset(path = "background.png")]
    background: Handle<Image>,
}

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
    .insert_resource(WinitSettings::desktop_app())
    .init_state::<AppState>()
    .add_loading_state(
        LoadingState::new(AppState::BootingApp)
            .continue_to_state(AppState::MainMenu)
            .on_failure_continue_to_state(AppState::ErrorScreen)
            .load_collection::<MyAssets>(),
    )
    .add_systems(OnEnter(AppState::BootingApp), boot_loading_screen)
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
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

pub fn boot_loading_screen(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0)));

    // Spawn a simple 2D sprite as a loading screen
    commands.spawn((
        Text::new("Loading..."),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        BackgroundColor(Color::WHITE),
    ));
}
