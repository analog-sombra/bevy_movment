use bevy::{
    prelude::*,
    window::{self, WindowMode},
};

use crate::AppState;
pub struct CustomWindowPlugin;

impl Plugin for CustomWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::BootingApp), set_window_size_for_boot)
            .add_systems(OnEnter(AppState::MainMenu), set_window_size_for_main_menu)
            .add_systems(OnEnter(AppState::InGameLoading), full_size_screen);
        // .add_systems(OnEnter(AppState::InGame), full_size_screen);
    }
}

fn set_window_size_for_boot(mut window: Single<&mut Window>) {
    window.resolution.set(800.0, 600.0); // Set size for boot state
}

fn set_window_size_for_main_menu(mut window: Single<&mut Window>) {
    window.resolution.set(1024.0, 768.0); // Set size for main menu state
}

fn full_size_screen(mut window: Query<&mut Window, With<window::PrimaryWindow>>) {
    if let Ok(mut window) = window.single_mut() {
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
}
