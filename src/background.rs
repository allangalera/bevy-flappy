use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::time::Duration;

const LAYER_1_SPEED: f32 = -50.0;

const CLOUD_SPEED_MIN: f32 = -500.0;
const CLOUD_SPEED_MAX: f32 = -50.0;

const CLOUD_SCALE_MIN: f32 = 0.25;
const CLOUD_SCALE_MAX: f32 = 2.0;

const CLOUD_DISTANCE_MIN: f32 = 0.0;
const CLOUD_DISTANCE_MAX: f32 = 1.0;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_despawn_area, spawn, setup_cloud_spawn_timer),
        )
        .add_systems(Update, (detect_and_despawn, cloud_spawner));
    }
}

#[derive(Resource)]
struct CloudsSpawnConfig {
    timer: Timer,
}
#[derive(Component)]
struct Scenario;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Cloud;

#[derive(Component)]
struct DespawnArea;

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("mountains.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -(window.width() / 2.0 - 100.0), 0.0),
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(512.0, 400.0))
        .insert(CollisionGroups::new(
            Group::from_bits(0b0001).unwrap(),
            Group::from_bits(0b0001).unwrap(),
        ))
        .insert(Velocity::linear(Vec2::new(LAYER_1_SPEED, 0.0)))
        .insert(Background)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Scenario);
}

fn setup_cloud_spawn_timer(mut commands: Commands) {
    commands.insert_resource(CloudsSpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
}

fn translate_value_from_one_range_to_another(
    current_value: f32,
    current_range_min: f32,
    current_range_max: f32,
    new_range_min: f32,
    new_range_max: f32,
) -> f32 {
    let current_range = current_range_max - current_range_min;
    let new_range = new_range_max - new_range_min;

    (((current_value - current_range_min) * new_range) / current_range) + new_range_min
}

fn cloud_spawner(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut config: ResMut<CloudsSpawnConfig>,
    asset_server: Res<AssetServer>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let Ok(window) = window_query.get_single() else {
            return;
        };
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            let initial_height_variation = window.height() - window.height() * 0.1;
            let initial_position_y =
                rng.gen_range(-initial_height_variation..initial_height_variation);
            let initial_position_x = window.width() * 2.0;

            let speed = rng.gen_range(CLOUD_SPEED_MIN..CLOUD_SPEED_MAX);
            let scale = translate_value_from_one_range_to_another(
                speed,
                CLOUD_SPEED_MIN,
                CLOUD_SPEED_MAX,
                CLOUD_SCALE_MIN,
                CLOUD_SCALE_MAX,
            );
            let distance = CLOUD_DISTANCE_MAX
                - translate_value_from_one_range_to_another(
                    speed,
                    CLOUD_SPEED_MIN,
                    CLOUD_SPEED_MAX,
                    CLOUD_DISTANCE_MIN,
                    CLOUD_DISTANCE_MAX,
                );

            let image_path = match rng.gen_range(1..9) {
                1 => "cloud1.png",
                2 => "cloud2.png",
                3 => "cloud3.png",
                4 => "cloud4.png",
                5 => "cloud5.png",
                6 => "cloud6.png",
                7 => "cloud7.png",
                8 => "cloud8.png",
                _ => "cloud1.png",
            };

            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load(image_path),
                    transform: Transform {
                        translation: Vec3::new(initial_position_x, initial_position_y, distance),
                        scale: Vec3::new(scale, scale, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        flip_x: true,
                        // custom_size: Vec2::new(),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::KinematicVelocityBased)
                .insert(Collider::cuboid(250.0, 250.0))
                .insert(CollisionGroups::new(
                    Group::from_bits(0b0001).unwrap(),
                    Group::from_bits(0b0001).unwrap(),
                ))
                .insert(Velocity::linear(Vec2::new(speed, 0.0)))
                .insert(Cloud)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Scenario);
        }
    }
}

fn spawn_despawn_area(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let x_position = -(4.0 * window.width());
    // let x_position = -window.width() - 200.0;
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
        .insert(CollisionGroups::new(
            Group::from_bits(0b0001).unwrap(),
            Group::from_bits(0b0001).unwrap(),
        ))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(DespawnArea);
}
fn detect_and_despawn(
    rapier_context: Res<RapierContext>,
    query_despawn_area: Query<Entity, With<DespawnArea>>,
    query_scenario: Query<Entity, With<Scenario>>,
    mut commands: Commands,
) {
    for entity_despawn_area in query_despawn_area.iter() {
        for entity_scenario in query_scenario.iter() {
            if let Some(_value) =
                rapier_context.intersection_pair(entity_despawn_area, entity_scenario)
            {
                commands.entity(entity_scenario).despawn_recursive();
            }
        }
    }
}
