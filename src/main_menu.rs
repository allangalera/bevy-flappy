use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameStart), spawn_main_menu)
            .add_systems(
                Update,
                wait_for_interaction_to_start.run_if(in_state(AppState::GameStart)),
            )
            .add_systems(OnExit(AppState::GameStart), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

fn wait_for_interaction_to_start(
    mut app_state: ResMut<NextState<AppState>>,
    mouse_buttons: Res<Input<MouseButton>>,
    key_buttons: Res<Input<KeyCode>>,
) {
    if mouse_buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        app_state.set(AppState::InGame);
    }
    if key_buttons.just_pressed(KeyCode::Space) {
        app_state.set(AppState::InGame);
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn(
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
            parent.spawn(
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
