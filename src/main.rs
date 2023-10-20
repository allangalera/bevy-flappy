use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

// mod background;
mod flappy;
mod in_game_ui;
mod main_menu;
mod pipes;

// use background::BackgroundPlugin;
use flappy::FlappyPlugin;
use in_game_ui::InGameUiPlugin;
use main_menu::MainMenuPlugin;
use pipes::PipesPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
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
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Flappy".into(),
                        resolution: (1200.0, 600.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(BackgroundPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(InGameUiPlugin)
        .add_plugins(FlappyPlugin)
        .add_plugins(PipesPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
pub struct Score(u32);

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 2.0;
    commands.spawn(camera);
    commands.insert_resource(Score(0));
}
