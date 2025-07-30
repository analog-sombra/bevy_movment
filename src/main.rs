use crate::booting::boot_screen::BootPlugin;
use crate::camera::camera::spawn_camera;
use crate::gameui::menu::MainMenuPlugin;
use crate::player::player::{player_input, player_movement, spawn_player};
use crate::window::window::CustomWindowPlugin;
use bevy::winit::WinitSettings;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_asset_loader::prelude::*;

mod booting;
mod camera;
mod gameui;
mod player;
mod window;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    BootingApp,
    ErrorScreen,
    MainMenu,
    InGame,
    Paused,
}


#[derive(Resource, Default, AssetCollection)]
pub struct MyAssets {
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
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    .insert_resource(WinitSettings::game())
    .init_state::<AppState>()
    .add_loading_state(
        LoadingState::new(AppState::BootingApp)
            .continue_to_state(AppState::MainMenu)
            .on_failure_continue_to_state(AppState::ErrorScreen)
            .load_collection::<MyAssets>(),
    )
    .add_plugins(BootPlugin)
    .add_plugins(CustomWindowPlugin)
    .add_plugins(MainMenuPlugin)
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
