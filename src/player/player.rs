use crate::{
    AppState, IsPaused, MyAssets,
    player::food::{Food, spawn_food},
};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

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

const SPEED: f32 = 140.0;

#[derive(Component)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;
#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeSegments::default())
            .add_systems(
                OnEnter(AppState::InGame),
                setup.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), teardown_game_object)
            .add_systems(
                Update,
                transition_to_ingame.run_if(in_state(AppState::InGameLoading)),
            )
            .add_systems(
                Update,
                (player_input, player_movement, food_collision_system)
                    .run_if(in_state(IsPaused::Running)),
            );
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<MyAssets>,
    window: Query<&Window>,
    player_query: Query<&Transform, With<Player>>,
    mut snake_segments: ResMut<SnakeSegments>,
) {
    let get_window = window.single().unwrap();
    let window_h = get_window.resolution.height();
    let window_w = get_window.resolution.width();

    spawn_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        &assets,
        window,
        &mut snake_segments,
    );
    spawn_food(&mut commands, &assets, window_w, window_h, &player_query);
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    assets: &MyAssets,
    window: Query<&Window>,
    snake_segments: &mut ResMut<SnakeSegments>,
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

    // Spawn the player head and capture the entity ID
    let player_entity = commands
        .spawn((
            InGameEntity,
            Player,
            SnakeSegment, // <-- Mark it as a segment
            Direction::default(),
            Mesh2d(meshes.add(shape)),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            GlobalTransform::default(),
            Visibility::default(),
            Collider::rectangle(40., 40.),
            CollisionEventsEnabled,
        ))
        .id();

    // Add the head to the SnakeSegments resource
    snake_segments.0.push(player_entity);

    let text_font = TextFont {
        font_size: 40.0,
        ..default()
    };

    commands.spawn((
        InGameEntity,
        ScoreText,
        Score(0),
        Text2d::new("Score: 0"),
        text_font,
        Transform::from_translation(Vec3::new(
            // -window_w / 2.0 + 20.0,
            0.,
            window_h / 2.0 - 30.0,
            0.0,
        )),
    ));
}

fn food_collision_system(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    food_query: Query<(), With<Food>>,
    assets: Res<MyAssets>,  // ⬅ asset for apple
    window: Query<&Window>, // ⬅ to get screen size
    mut query: Query<(&mut Score, &mut Text2d), With<ScoreText>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_segments: ResMut<SnakeSegments>,
    segment_transforms: Query<&Transform, With<SnakeSegment>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let is_entity1_player = player_query.get(*entity1).is_ok();
        let is_entity2_player = player_query.get(*entity2).is_ok();

        let is_entity1_food = food_query.get(*entity1).is_ok();
        let is_entity2_food = food_query.get(*entity2).is_ok();

        let window = window.single().unwrap();
        let window_h = window.resolution.height();
        let window_w = window.resolution.width();
        // Despawn the food if player collides with it
        if is_entity1_player && is_entity2_food {
            commands.entity(*entity2).despawn();
            println!("Player ate food (entity {:?})", entity2);
        } else if is_entity2_player && is_entity1_food {
            commands.entity(*entity1).despawn();
            println!("Player ate food (entity {:?})", entity1);
        }
        for (mut score, mut text) in &mut query {
            score.0 += 1;
            text.0 = format!("Score: {}", score.0);
        }
        spawn_food(&mut commands, &assets, window_w, window_h, &player_query);
        grow_snake_size(
            &mut commands,
            &mut snake_segments,
            &mut meshes,
            &mut materials,
            &segment_transforms,
        );
    }
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
    // mut query: Query<(&mut Transform, &Direction), With<Player>>,
    mut snake_segments: ResMut<SnakeSegments>,
    mut segment_query: Query<&mut Transform, With<SnakeSegment>>,
    direction_query: Query<&Direction, With<Player>>,
    window: Query<&Window>,
    mut next_state: ResMut<NextState<IsPaused>>,
) {
    // if user touch the border of the window change the state to GameOver
    let window = window.single().unwrap();
    let window_h = window.resolution.height();
    let window_w = window.resolution.width();

    // Don't do anything if no segments or no direction
    if snake_segments.0.is_empty() {
        return;
    }

    let head_entity = snake_segments.0[0];

    let direction = direction_query.get(head_entity).ok();
    if direction.is_none() {
        return;
    }

    let direction = direction.unwrap();

    let mut positions: Vec<Vec3> = Vec::with_capacity(snake_segments.0.len());

    if let Ok(mut transform) = segment_query.get_mut(head_entity) {
        let mut translation = transform.translation;
        match direction.0 {
            DIRECTION::Right => translation.x += SPEED * time.delta_secs(),
            DIRECTION::Up => translation.y += SPEED * time.delta_secs(),
            DIRECTION::Down => translation.y -= SPEED * time.delta_secs(),
            DIRECTION::Left => translation.x -= SPEED * time.delta_secs(),
        }
        positions.push(transform.translation); // store old position
        transform.translation = translation;

        // Assume player size is 40x40 (from spawn_player)
        let half_player_size = 20.0;
        if transform.translation.x.abs() + half_player_size > window_w / 2.0
            || transform.translation.y.abs() + half_player_size > window_h / 2.0
        {
            next_state.set(IsPaused::GameOver);
        }
    }

    // Step 2: collect previous positions first to avoid mutable/immutable borrow conflict
    let mut prev_positions: Vec<Vec3> = Vec::with_capacity(snake_segments.0.len());
    for &entity in &snake_segments.0 {
        if let Ok(transform) = segment_query.get(entity) {
            prev_positions.push(transform.translation);
        } else {
            prev_positions.push(Vec3::ZERO);
        }
    }

    // Now update each segment's transform using mutable borrow
    for i in 1..snake_segments.0.len() {
        let curr = snake_segments.0[i];
        let prev_position = prev_positions[i - 1];
        if let Ok(mut curr_transform) = segment_query.get_mut(curr) {
            curr_transform.translation = prev_position;
        }
    }
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

pub fn grow_snake_size(
    commands: &mut Commands,
    snake_segments: &mut ResMut<SnakeSegments>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    segment_query: &Query<&Transform, With<SnakeSegment>>,
) {
    // Get the last segment
    if let Some(&last_segment) = snake_segments.0.last() {
        if let Ok(last_transform) = segment_query.get(last_segment) {            
            
            // Get its position
            // You'll need a query to get the transform of the last segment
            // This is pseudocode, adapt it:
            // let last_position = last_transform.translation;

            let last_position = last_transform.translation - Vec3::X * 40.0; // 40.0 = segment size

            let shape = Rectangle::new(40., 40.);
            // rgb(65, 171, 93)
            // let color = Color::srgb(65.0, 171.0, 93.0);
            let color = Color::srgb(255., 255.0, 255.0);

            let new_segment = commands
                .spawn((
                    SnakeSegment,
                    Mesh2d(meshes.add(shape)),
                    MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                    Transform::from_translation(last_position), // Or offset based on direction
                    GlobalTransform::default(),
                    Visibility::default(),
                ))
                .id();

            snake_segments.0.push(new_segment);
        }
    }
}
