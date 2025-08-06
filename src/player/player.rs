use std::time::Duration;

use bevy::{prelude::*, window};

use crate::{AppState, IsPaused, MyAssets, camera::camera::spawn_camera};

// #[derive(Component, Default)]
// pub struct Velocity(Vec3);

#[derive(Component, Default)]
pub struct Direction(DIRECTION);
#[derive(Default, PartialEq)]
pub enum DIRECTION {
    #[default]
    Right,
    Up,
    Down,
    Left,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct Food;
const SPEED: f32 = 80.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            setup.run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), teardown_game_object)
        .add_systems(
            Update,
            transition_to_ingame.run_if(in_state(AppState::InGameLoading)),
        )
        .add_systems(Update, (toggle_pause).run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(IsPaused::Paused), setup_paused_screen)
        .add_systems(
            Update,
            (player_input, player_movement).run_if(in_state(IsPaused::Running)),
        );
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<MyAssets>,
    window: Query<&Window>,
) {
    spawn_player(&mut commands, &mut meshes, &mut materials, assets, window);
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    assets: Res<MyAssets>,
    window: Query<&Window>,
) {
    let window = window.single().unwrap();
    let window_h = window.resolution.height();
    let window_w = window.resolution.width();

    let ground_size = 32.0;
    let ground_texture = assets.ground.clone();

    let tiles_x = (window_w / ground_size).ceil() as i64;
    let tiles_y = (window_h / ground_size).ceil() as i64;

    let start_x = -window_w / 2.0 + ground_size / 2.0;
    let start_y = -window_h / 2.0 + ground_size / 2.0;

    for y in 0..tiles_y {
        for x in 0..tiles_x {
            let x_pos = start_x + x as f32 * ground_size;
            let y_pos = start_y + y as f32 * ground_size;

            commands.spawn((
                InGameEntity,
                Sprite::from_image(ground_texture.clone()),
                Transform::from_xyz(x_pos, y_pos, 0.0),
                GlobalTransform::default(),
                Ground,
            ));
        }
    }
    let shape = Rectangle::new(40., 40.);
    // rgb(65, 171, 93)
    let color = Color::srgb(65.0, 171.0, 93.0);
    commands.spawn((
        InGameEntity,
        Player,
        Direction::default(),
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(ColorMaterial::from(color))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        GlobalTransform::default(),
        Visibility::default(),
    ));

    let apple_texture = assets.apple.clone();
    // spawn food on random place in screen

    // Spawn food sprite with a red border
    let food_entity = commands
        .spawn((
            Food,
            Sprite::from_image(apple_texture),
            Transform::from_xyz(
                rand::random::<f32>() * window_w - window_w / 2.0,
                rand::random::<f32>() * window_h - window_h / 2.0,
                10.0,
            ),
            GlobalTransform::default(),
        ))
        .id();

    // Add a child entity for the red border
    commands.entity(food_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::srgb(255., 0., 0.),
                custom_size: Some(Vec2::new(34.0, 34.0)), // slightly larger than food
                ..Default::default()
            },
            Transform::from_xyz(0.0, 0.0, -0.1), // behind the food sprite
            GlobalTransform::default(),
            Visibility::default(),
        ));
    });
}

fn teardown_game_object(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Direction, With<Player>>,
) {
    if let Ok(mut direction) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) && direction.0 != DIRECTION::Right {
            direction.0 = DIRECTION::Left;
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) && direction.0 != DIRECTION::Left
        {
            direction.0 = DIRECTION::Right;
        } else if keyboard_input.just_pressed(KeyCode::ArrowUp) && direction.0 != DIRECTION::Down {
            direction.0 = DIRECTION::Up;
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) && direction.0 != DIRECTION::Up {
            direction.0 = DIRECTION::Down;
        }
    }
}
pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction), With<Player>>,
    window: Query<&Window>,
    food: Query<&Transform, With<Food>>,
) {
    if let Ok((mut transform, direction)) = query.single_mut() {
        let mut translation = transform.translation;
        match direction.0 {
            DIRECTION::Right => translation.x += SPEED * time.delta_secs(),
            DIRECTION::Up => translation.y += SPEED * time.delta_secs(),
            DIRECTION::Down => translation.y -= SPEED * time.delta_secs(),
            DIRECTION::Left => translation.x -= SPEED * time.delta_secs(),
        }
        transform.translation = translation;
    }

    // get food location
    if let Ok(food_transform) = food.single() {
        let food_pos = food_transform.translation;
        // You can now use food_pos for collision detection or other logic
    }

    // add collision with food if player collides with food, respawn food at random position
}

fn transition_to_ingame(
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    // Delay transition by a short amount to let the window apply
    let delay = Duration::from_millis(100); // adjust if needed
    if timer.is_none() {
        *timer = Some(Timer::new(delay, TimerMode::Once));
    }
    if let Some(t) = timer.as_mut() {
        t.tick(time.delta());
        if t.finished() {
            next_state.set(AppState::InGame);
        }
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
