use crate::{AppState, IsPaused, player::player::Score};
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IsPaused::GameOver), setup_game_over_screen);
    }
}

pub fn setup_game_over_screen(mut commands: Commands, mut query: Query<&Score>) {
    commands
        .spawn((
            StateScoped(IsPaused::GameOver),
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
                let gameover_font = TextFont {
                    font_size: 32.0,
                    ..default()
                };

                p.spawn((
                    Text::new("Game Over"),
                    gameover_font,
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
                
                let score_font = TextFont {
                    font_size: 24.0,
                    ..default()
                };

                let mut bestscore = 0;
                for score in &mut query {
                    if score.0 > bestscore {
                        bestscore = score.0;
                    }
                }
                p.spawn((
                    Text::new(format!("Score: {}", bestscore)),
                    score_font,
                    TextColor(Color::srgb(199.0 / 255.0, 236.0 / 255.0, 250.0 / 255.0)),
                ));

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
