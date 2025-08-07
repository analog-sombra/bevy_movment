use crate::{AppState, IsPaused};
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_pause).run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(IsPaused::Paused), setup_paused_screen);
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<IsPaused>>,
    mut next_state: ResMut<NextState<IsPaused>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(match current_state.get() {
            IsPaused::Running => IsPaused::Paused,
            IsPaused::Paused => IsPaused::Running,
        });
    }
}

pub fn setup_paused_screen(mut commands: Commands) {
    commands
        .spawn((
            StateScoped(IsPaused::Paused),
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Px(400.),
                    height: Val::Px(400.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|p| {
                p.spawn(create_menu_button("Resume")).observe(
                    |mut trigger: Trigger<Pointer<Released>>,
                     mut next: ResMut<NextState<IsPaused>>| {
                        trigger.propagate(false);
                        next.set(IsPaused::Running)
                    },
                );

                // Restart the Game
                p.spawn(create_menu_button("Restart")).observe(
                    |mut trigger: Trigger<Pointer<Released>>,
                     mut next: ResMut<NextState<AppState>>| {
                        trigger.propagate(false);
                        next.set(AppState::InGameLoading)
                    },
                );

                // Go to Main Menu
                p.spawn(create_menu_button("Menu")).observe(
                    |mut trigger: Trigger<Pointer<Released>>,
                     mut next: ResMut<NextState<AppState>>| {
                        trigger.propagate(false);
                        next.set(AppState::MainMenu)
                    },
                );

                // Exit Button
                p.spawn(create_menu_button("Exit")).observe(
                    |mut trigger: Trigger<Pointer<Released>>| {
                        trigger.propagate(false);
                        std::process::exit(0);
                    },
                );
            });
        });
}

// Helper function to create button
fn create_menu_button(text: &str) -> impl Bundle {
    (
        // MainMenuButton,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Percent(10.0)),
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2).into()),
        Button,
        children![(
            Text::new(text),
            TextFont {
                font_size: 18.0,
                ..default()
            },
        )],
    )
}
