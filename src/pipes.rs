use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::AppState;
use crate::Score;

const SPRITE_SIZE: f32 = 250.0;

const PIPE_HEIGHT: f32 = SPRITE_SIZE * 8.0;
const PIPE_WIDTH: f32 = SPRITE_SIZE;

const GAP_SIZE: f32 = 600.0;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_pipe_despawn_area, setup_spawn_pipe),
        )
        .add_systems(
            Update,
            (spawn_pipe, detect_pipe_despawn_and_pipes_collision)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), despawn);
    }
}

#[derive(Resource)]
struct PipesSpawnConfig {
    timer: Timer,
}

#[derive(Component)]
pub struct PipeTop;

#[derive(Component)]
pub struct PipeBottom;

#[derive(Component)]
pub struct GapSensor {
    pub counted: bool,
}

#[derive(Component)]
struct PipeGroup;

#[derive(Component)]
struct PipeDespawnArea;

fn setup_spawn_pipe(mut commands: Commands) {
    commands.insert_resource(PipesSpawnConfig {
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    });
}

fn spawn_pipe_despawn_area(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let x_position = -(window.width() + PIPE_WIDTH + 20.0);
    let width = 50.0;
    let height = window.height() * 2.0;

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(x_position, 0.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, height)),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(PipeDespawnArea);
}

fn spawn_pipe(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut config: ResMut<PipesSpawnConfig>,
    asset_server: Res<AssetServer>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut rng = rand::thread_rng();
        let Ok(window) = window_query.get_single() else {
            return;
        };

        let sensor_width = 50.0;

        let initial_position_x = window.width() + PIPE_WIDTH;
        let initial_height_variation = window.height() - GAP_SIZE / 2.0 - 20.0;
        let initial_position_y = rng.gen_range(-initial_height_variation..initial_height_variation);

        commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(initial_position_x, initial_position_y, 1.0),
                sprite: Sprite {
                    color: Color::NONE.into(),
                    ..default()
                },
                ..default()
            })
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Velocity::linear(Vec2::new(-300.0, 0.0)))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(PipeGroup)
            .with_children(|parent| {
                // pipe top
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(
                            0.0,
                            PIPE_HEIGHT / 2.0 + GAP_SIZE / 2.0,
                            0.0,
                        ),
                        ..default()
                    })
                    .insert(Collider::cuboid(PIPE_WIDTH / 2.0, PIPE_HEIGHT / 2.0))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(PipeTop)
                    .with_children(|parent| {
                        let number_of_sprites = (PIPE_HEIGHT / SPRITE_SIZE) as u32;
                        let initial_position = -(PIPE_HEIGHT / 2.0 - SPRITE_SIZE / 2.0);
                        for pipe_index in 0..number_of_sprites {
                            parent.spawn(SpriteBundle {
                                transform: Transform::from_xyz(
                                    0.0,
                                    initial_position + (pipe_index as f32 * SPRITE_SIZE),
                                    0.0,
                                ),
                                texture: asset_server.load("pipe.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(PIPE_WIDTH, SPRITE_SIZE)),
                                    color: Color::rgb(60.0 / 255.0, 185.0 / 255.0, 120.0 / 255.0),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    });
                // Gap Sensor
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        sprite: Sprite {
                            color: Color::NONE.into(),
                            custom_size: Some(Vec2::new(sensor_width, GAP_SIZE)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Collider::cuboid(sensor_width / 2.0, GAP_SIZE / 2.0))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Sensor)
                    .insert(GapSensor { counted: false });
                // pipe bottom
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(
                            0.0,
                            -(PIPE_HEIGHT / 2.0 + GAP_SIZE / 2.0),
                            0.0,
                        ),
                        ..default()
                    })
                    .insert(Collider::cuboid(PIPE_WIDTH / 2.0, PIPE_HEIGHT / 2.0))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(PipeBottom)
                    .with_children(|parent| {
                        let number_of_sprites = (PIPE_HEIGHT / SPRITE_SIZE) as u32;
                        let initial_position = -(PIPE_HEIGHT / 2.0 - SPRITE_SIZE / 2.0);
                        for pipe_index in 0..number_of_sprites {
                            parent.spawn(SpriteBundle {
                                transform: Transform::from_xyz(
                                    0.0,
                                    initial_position + (pipe_index as f32 * SPRITE_SIZE),
                                    0.0,
                                ),
                                texture: asset_server.load("pipe.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(PIPE_WIDTH, SPRITE_SIZE)),
                                    color: Color::rgb(60.0 / 255.0, 185.0 / 255.0, 120.0 / 255.0),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    });
            });
    }
}

fn detect_pipe_despawn_and_pipes_collision(
    rapier_context: Res<RapierContext>,
    query_despawn_area: Query<Entity, With<PipeDespawnArea>>,
    q_pipe_top: Query<(Entity, &Parent), With<PipeTop>>,
    q_pipe_group: Query<Entity, With<PipeGroup>>,
    mut commands: Commands,
) {
    for entity_pipe_despawn_area in query_despawn_area.iter() {
        for (entity_pipe_top, parent) in q_pipe_top.iter() {
            if let Some(_value) =
                rapier_context.intersection_pair(entity_pipe_despawn_area, entity_pipe_top)
            {
                let parent_entity = q_pipe_group.get(parent.get()).unwrap();
                commands.entity(parent_entity).despawn_recursive();
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, Or<(With<PipeGroup>, With<PipeDespawnArea>)>>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
