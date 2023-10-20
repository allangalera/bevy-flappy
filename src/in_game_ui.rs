use bevy::prelude::*;

use crate::AppState;
use crate::Score;

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_score_menu)
            .add_systems(
                Update,
                detect_score_change.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), despawn);
    }
}

#[derive(Component)]
struct ScoreMenu;

#[derive(Component)]
struct ScoreText;

fn spawn_score_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(ScoreMenu)
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new(
                            "Score: ",
                            TextStyle {
                                font: asset_server.load("fonts/Monocraft.otf"),
                                font_size: 20.0,
                                color: Color::rgb(0.086, 0.086, 0.086),
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server.load("fonts/Monocraft.otf"),
                            font_size: 20.0,
                            color: Color::rgb(0.086, 0.086, 0.086),
                        }),
                    ])
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    }),
                )
                .insert(ScoreText);
        });
}

fn detect_score_change(score: Res<Score>, mut q_text: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in q_text.iter_mut() {
            text.sections[1].value = format!("{:?}", score.0);
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<ScoreMenu>>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
