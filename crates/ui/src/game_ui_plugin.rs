use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use world_generation::world_ready::WorldReady;

use crate::{
    fps_text::{FpsText, update_fps_ui},
    task_text::{ChunkTaskText, CountryTaskText, update_task_ui},
    triangle_count_text::{TriangleText, update_triangle_ui},
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(
                Update,
                (update_fps_ui, update_task_ui, update_triangle_ui),
            )
            .add_observer(spawn_ui);
    }
}

fn spawn_ui(_: On<WorldReady>, mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|commands| {
            commands.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Px(32.0),
                    margin: UiRect::new(
                        Val::Auto,
                        Val::Auto,
                        Val::Px(15.0),
                        Val::Px(0.0),
                    ),
                    ..default()
                },
                Text("FPS!".to_string()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                FpsText,
            ));
            commands.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Px(32.0),
                    margin: UiRect::new(
                        Val::Auto,
                        Val::Auto,
                        Val::Px(15.0),
                        Val::Px(0.0),
                    ),
                    ..default()
                },
                Text("TRIANGLES!".to_string()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TriangleText,
            ));
            commands.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Px(32.0),
                    margin: UiRect::new(
                        Val::Auto,
                        Val::Auto,
                        Val::Px(15.0),
                        Val::Px(0.0),
                    ),
                    ..default()
                },
                Text("Country Tasks!".to_string()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                CountryTaskText,
            ));
            commands.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Px(32.0),
                    margin: UiRect::new(
                        Val::Auto,
                        Val::Auto,
                        Val::Px(15.0),
                        Val::Px(0.0),
                    ),
                    ..default()
                },
                Text("Chunk Tasks!".to_string()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                ChunkTaskText,
            ));
        });
}
