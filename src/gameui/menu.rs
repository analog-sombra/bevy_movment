use crate::AppState;
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component, Default)]
pub struct MainMenuEntity;

pub fn spawn_menu(mut commands: Commands, assets: Res<AssetServer>) {
    // Load the background texture
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0), MainMenuEntity));
    let background_handle: Handle<Image> = assets.load("background.jpg");

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
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(background_handle.clone()),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                Interaction::default(),
                Outline {
                    width: Val::Px(2.),
                    offset: Val::Px(2.),
                    color: Color::NONE,
                },
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column, // stack children vertically
                justify_content: JustifyContent::FlexStart, // align children at top
                column_gap: Val::Px(10.0), // space between buttons
                align_items: AlignItems::Center, // center children horizontally
                // Set other fields if needed, or ...Default
                ..default()
            },
            // rgb(244, 144, 183)
            BackgroundColor(Color::srgba(
                244.0 / 255.0,
                144.0 / 255.0,
                183.0 / 255.0,
                0.75,
            )), // fallback color if image fails
        ))
        .with_children(|parent| {
            parent
                .spawn((MainMenuEntity, play_button(&assets)))
                .observe(
                    |mut trigger: Trigger<Pointer<Released>>,
                     mut state: ResMut<NextState<AppState>>| {
                        trigger.propagate(false);
                        // let event = trigger.event();
                        // let target = event.target;
                        state.set(AppState::InGame);
                    },
                );
            parent
                .spawn((MainMenuEntity, exit_button(&assets)))
                .observe(
                    |mut trigger: Trigger<Pointer<Released>>,
                     mut state: ResMut<NextState<AppState>>| {
                        trigger.propagate(false);

                        // exit the bevy app
                        std::process::exit(0);
                    },
                );
        });

    //
}

fn play_button(asset_server: &AssetServer) -> impl Bundle + use<> {
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

fn exit_button(asset_server: &AssetServer) -> impl Bundle + use<> {
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

pub fn delete_menu(mut commands: Commands, query: Query<Entity, With<MainMenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    println!("Main menu entities despawned.");
}
