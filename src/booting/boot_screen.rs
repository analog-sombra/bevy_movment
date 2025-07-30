use crate::AppState;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BootLoadingScreen;

pub struct BootPlugin;

impl Plugin for BootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::BootingApp), boot_loading_screen)
            .add_systems(OnExit(AppState::BootingApp), delete_boot_screen);
    }
}

fn boot_loading_screen(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        BootLoadingScreen,
    ));

    // Spawn a simple 2D sprite as a loading screen
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        children![
            Text::new("Game Name"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ],
        BootLoadingScreen,
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            Text::new("Loading..."),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ],
        BootLoadingScreen,
    ));
}

fn delete_boot_screen(mut commands: Commands, query: Query<Entity, With<BootLoadingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
