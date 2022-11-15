use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::GameStart).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(AppState::GameStart)
                    .with_system(wait_for_interaction_to_start),
            )
            .add_system_set(SystemSet::on_exit(AppState::GameStart).with_system(despawn_main_menu));
    }
}

#[derive(Component)]
pub struct MainMenu;

fn wait_for_interaction_to_start(
    mut app_state: ResMut<State<AppState>>,
    mouse_buttons: Res<Input<MouseButton>>,
    key_buttons: Res<Input<KeyCode>>,
) {
    if mouse_buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        app_state.set(AppState::InGame).unwrap();
    }
    if key_buttons.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Click or press space to start",
                    TextStyle {
                        font: asset_server.load("fonts/Monocraft.otf"),
                        font_size: 35.0,
                        color: Color::rgb(0.086, 0.086, 0.086),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                }),
            );
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Welcome to Flappy Bevy",
                    TextStyle {
                        font: asset_server.load("fonts/Monocraft.otf"),
                        font_size: 50.0,
                        color: Color::hsl(153.0, 0.67, 0.28),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                }),
            );
        });
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
