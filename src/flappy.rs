use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use crate::pipes::GapSensor;
use crate::pipes::PipeBottom;
use crate::pipes::PipeTop;
use crate::AppState;
use crate::Score;

pub struct FlappyPlugin;

impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_flappy)
            .add_systems(
                Update,
                (
                    in_game_control,
                    animate_flappy,
                    detect_flappy_gap_sensor_collision,
                    detect_flappy_pipes_collision,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_flappy);
    }
}

#[derive(Component)]
struct Flappy;

fn spawn_flappy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = 100.0;

    // flappy
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("bevy.png"),
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0))
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(GravityScale(25.0))
        .insert(Flappy);
}

fn animate_flappy(mut q_flappy: Query<(&Velocity, &mut Transform), With<Flappy>>) {
    for (velocity, mut transform) in q_flappy.iter_mut() {
        if velocity.linvel[1] > 0.0 {
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_rotation_z(-PI / 2.0), 0.02);
        } else {
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_rotation_z(-PI / 2.0), 0.04);
        }
    }
}

fn despawn_flappy(mut commands: Commands, query: Query<Entity, With<Flappy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn in_game_control(
    mut app_state: ResMut<NextState<AppState>>,
    mut flappy: Query<(&mut Velocity, &mut ExternalImpulse, &mut Transform), With<Flappy>>,
    buttons: Res<Input<MouseButton>>,
    key_buttons: Res<Input<KeyCode>>,
) {
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right])
        || key_buttons.any_just_pressed([KeyCode::Space])
    {
        let (mut velocity, mut external_impulse, mut transform) = flappy.single_mut();
        external_impulse.impulse = Vec2::new(0.0, 800.0);
        velocity.linvel = Vec2::new(0.0, 0.0);
        transform.rotation = Quat::from_rotation_z(PI / 3.0);
    }
    if key_buttons.just_pressed(KeyCode::Escape) {
        println!("Escape and go to main menu again");
        app_state.set(AppState::GameStart);
    }
}

fn detect_flappy_pipes_collision(
    mut app_state: ResMut<NextState<AppState>>,
    rapier_context: Res<RapierContext>,
    query_flappy: Query<Entity, With<Flappy>>,
    query_pipe: Query<Entity, Or<(With<PipeTop>, With<PipeBottom>)>>,
) {
    for entity_flappy in query_flappy.iter() {
        for entity_pipe in query_pipe.iter() {
            if let Some(_value) = rapier_context.contact_pair(entity_flappy, entity_pipe) {
                app_state.set(AppState::GameStart);
            }
        }
    }
}

fn detect_flappy_gap_sensor_collision(
    mut collision_events: EventReader<CollisionEvent>,
    query_flappy: Query<Entity, With<Flappy>>,
    mut query_gap_sensor: Query<(Entity, &mut GapSensor), With<GapSensor>>,
    mut score: ResMut<Score>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _value) => {
                if let (Ok(_value), Ok(_value2)) =
                    (query_flappy.get(*entity1), query_gap_sensor.get(*entity2))
                {
                    let (_entity, mut sensor) = query_gap_sensor.get_mut(*entity2).unwrap();
                    if !sensor.counted {
                        sensor.counted = true;
                        score.0 += 1;
                    }
                }

                if let (Ok(_value), Ok(_value2)) =
                    (query_flappy.get(*entity2), query_gap_sensor.get(*entity1))
                {
                    let (_entity, mut sensor) = query_gap_sensor.get_mut(*entity1).unwrap();
                    if !sensor.counted {
                        sensor.counted = true;
                        score.0 += 1;
                    }
                }
            }
            _ => (),
        }
    }
}
