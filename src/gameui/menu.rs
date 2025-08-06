use crate::{AppState, MyAssets};
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Component, Default)]
pub struct MainMenuScreen;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_menu)
            .add_systems(OnExit(AppState::MainMenu), delete_menu);
    }
}

#[hot]
fn spawn_menu(mut commands: Commands, assets: Res<MyAssets>) {
    // Load the background texture
    // commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0), MainMenuScreen));

    // Fullscreen background node (no NodeBundle)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set other fields if needed, or ...Default
                ..default()
            },
            BackgroundColor(Color::BLACK), // fallback color if image fails
            MainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                MainMenuScreen,
                ImageNode::new(assets.background.clone()),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    aspect_ratio: Some(16.0 / 9.0), // maintain aspect ratio
                    ..default()
                },
                Interaction::default(),
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column, // stack children vertically
                justify_content: JustifyContent::Center, // center children vertically
                align_items: AlignItems::Center,       // center children horizontally
                row_gap: Val::Px(10.0),                // space between buttons
                // Set other fields if needed, or ...Default
                ..default()
            },
            // rgb(244, 144, 183)
            BackgroundColor(Color::srgb(244.0 / 255.0, 144.0 / 255.0, 183.0 / 255.0)), // fallback color if image fails
            MainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    MainMenuScreen,
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0), // space between buttons
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        // position_type: PositionType::Absolute,
                        ..default()
                    },
                ))
                .with_children(|child_parent| {
                    child_parent.spawn((
                        MainMenuScreen,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        children![(
                            Text::new("MENU"),
                            TextFont {
                                font_size: 22.0,
                                ..default()
                            },
                            // rgb(255, 255, 255)
                            TextColor(Color::srgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0)),
                        )],
                    ));
                    child_parent.spawn((MainMenuScreen, play_button())).observe(
                        |mut trigger: Trigger<Pointer<Released>>,
                         mut state: ResMut<NextState<AppState>>| {
                            trigger.propagate(false);
                            // let event = trigger.event();
                            // let target = event.target;
                            state.set(AppState::InGameLoading);
                        },
                    );
                    child_parent.spawn((MainMenuScreen, exit_button())).observe(
                        |mut trigger: Trigger<Pointer<Released>>| {
                            trigger.propagate(false);

                            // exit the bevy app
                            std::process::exit(0);
                        },
                    );
                });
        });

    //
}

fn play_button() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            // position_type: PositionType::Absolute,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(160.0),
                height: Val::Px(54.0),
                // border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            // rgb(199, 236, 250)
            BackgroundColor(Color::srgb(199.0 / 255.0, 236.0 / 255.0, 250.0 / 255.0)),
            children![(
                Text::new("PLAY"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                // rgb(29, 45, 60)
                TextColor(Color::srgb(29.0 / 255.0, 45.0 / 255.0, 60.0 / 255.0)),
            )]
        )],
    )
}

fn exit_button() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(160.0),
                height: Val::Px(54.0),
                // border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            // rgb(199, 236, 250)
            BackgroundColor(Color::srgb(199.0 / 255.0, 236.0 / 255.0, 250.0 / 255.0)),
            children![(
                Text::new("EXIT"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                // rgb(29, 45, 60)
                TextColor(Color::srgb(29.0 / 255.0, 45.0 / 255.0, 60.0 / 255.0)),
            )]
        )],
    )
}

fn delete_menu(mut commands: Commands, query: Query<Entity, With<MainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
