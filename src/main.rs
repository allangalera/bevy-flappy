use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod background;
mod flappy;
mod in_game_ui;
mod main_menu;
mod pipes;

use background::BackgroundPlugin;
use flappy::FlappyPlugin;
use in_game_ui::InGameUiPlugin;
use main_menu::MainMenuPlugin;
use pipes::PipesPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    GameStart,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            208.0 / 255.0,
            244.0 / 255.0,
            247.0 / 255.0,
        )))
        .add_state(AppState::GameStart)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Flappy Bevy".to_string(),
                        width: 1200.0,
                        height: 600.0,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(BackgroundPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InGameUiPlugin)
        .add_plugin(FlappyPlugin)
        .add_plugin(PipesPlugin)
        // In-Game
        .add_startup_system(setup)
        .run();
}

#[derive(Resource)]
pub struct Score {
    value: u32,
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 2.0;
    commands.spawn_bundle(camera);
    commands.insert_resource(Score { value: 0 });
}
