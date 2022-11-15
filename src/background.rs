use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::time::Duration;

const LAYER_1_SPEED: f32 = -50.0;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn)
            .add_startup_system(setup_cloud_spawn_timer)
            .add_system(cloud_spawner);
    }
}

#[derive(Component)]
struct CloudsSpawnConfig {
    timer: Timer,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Cloud;

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.primary();
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("mountains.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -(window.width() / 2.0 - 100.0), 0.0),
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::linear(Vec2::new(LAYER_1_SPEED, 0.0)))
        .insert(Background);
}

fn setup_cloud_spawn_timer(mut commands: Commands) {
    commands.insert_resource(CloudsSpawnConfig {
        timer: Timer::new(Duration::from_secs(2), true),
    });
}

fn cloud_spawner(
    mut commands: Commands,
    windows: Res<Windows>,
    time: Res<Time>,
    mut config: ResMut<CloudsSpawnConfig>,
    asset_server: Res<AssetServer>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            let window = windows.primary();
            let initial_height_variation = window.height() - window.height() * 0.1;
            let initial_position_y =
                rng.gen_range(-initial_height_variation..initial_height_variation);
            let initial_position_x = window.width() * 2.0;

            let speed = rng.gen_range(-500.0..-50.0);

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
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(image_path),
                    transform: Transform {
                        translation: Vec3::new(initial_position_x, initial_position_y, 0.0),
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
                .insert(Velocity::linear(Vec2::new(speed, 0.0)))
                .insert(Cloud);
        }
    }
}

// fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
//     for (mut transform) in query.iter_mut() {
//         transform.translation.x = transform.translation.x + LAYER_1_SPEED * time.delta_seconds();
//     }
// }
