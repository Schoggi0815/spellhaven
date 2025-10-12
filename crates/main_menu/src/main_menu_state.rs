use bevy::prelude::*;
use world_generation::world_ready::WorldReady;

use crate::main_menu_plugin::MenuCamera;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MainMenuState {
    #[default]
    Shown,
    LoadingWorldGen,
    Hidden,
}

pub fn hide_main_menu(
    _: On<WorldReady>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    cameras: Query<Entity, With<MenuCamera>>,
    mut commands: Commands,
) {
    main_menu_state.set(MainMenuState::Hidden);

    for camera in cameras {
        commands.entity(camera).despawn();
    }
}
