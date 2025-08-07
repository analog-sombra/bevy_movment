use crate::booting::boot_screen::BootPlugin;
use crate::gameui::gameover::GameOverPlugin;
use crate::gameui::menu::MainMenuPlugin;
use crate::gameui::pause::PauseMenuPlugin;
use crate::player::player::PlayerPlugin;
use crate::window::window::CustomWindowPlugin;
use avian2d::prelude::*;
use bevy::winit::WinitSettings;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_asset_loader::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

mod booting;
mod camera;
mod gameui;
mod player;
mod window;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Restarting,
    BootingApp,
    ErrorScreen,
    MainMenu,
    InGameLoading,
    InGame,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
    GameOver
}

#[derive(Resource, Default, AssetCollection)]
pub struct MyAssets {
    #[asset(path = "background.png")]
    background: Handle<Image>,
    #[asset(path = "ground.png")]
    ground: Handle<Image>,
    #[asset(path = "apple.png")]
    apple: Handle<Image>,
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
    .add_plugins(SimpleSubsecondPlugin::default())
    .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    .insert_resource(WinitSettings::game())
    .init_state::<AppState>()
    .add_sub_state::<IsPaused>()
    .add_loading_state(
        LoadingState::new(AppState::BootingApp)
            .continue_to_state(AppState::MainMenu)
            .on_failure_continue_to_state(AppState::ErrorScreen)
            .load_collection::<MyAssets>(),
    )
    .add_systems(OnEnter(AppState::Restarting), go_to_running)
    .add_systems(PreUpdate, detect_restart_key)
    .add_plugins(BootPlugin)
    .add_plugins(CustomWindowPlugin)
    .add_plugins(MainMenuPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(GameOverPlugin)
    .add_plugins(PauseMenuPlugin);

    app.run();
}

fn go_to_running(mut next: ResMut<NextState<AppState>>) {
    next.set(AppState::BootingApp);
}

fn detect_restart_key(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::F2) {
        println!("F2 pressed → restarting...");
        next.set(AppState::Restarting);
    }
}
