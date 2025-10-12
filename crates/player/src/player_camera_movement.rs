use bevy::prelude::*;
use bevy_hookup_core::owner_component::Owner;
use bevy_panorbit_camera::PanOrbitCamera;
use debug_resource::debug_resource::SpellhavenDebugResource;

use crate::player_component::{Player, PlayerCamera};

pub(super) fn move_camera(
    player: Query<&Transform, (With<Owner<Player>>, Without<PlayerCamera>)>,
    mut camera: Query<
        &mut PanOrbitCamera,
        (With<PlayerCamera>, Without<Owner<Player>>),
    >,
    options: Res<SpellhavenDebugResource>,
) {
    if options.unlock_camera {
        return;
    }

    let (Ok(player), Ok(mut camera)) = (player.single(), camera.single_mut())
    else {
        return;
    };

    let camera_position = camera.target_focus;
    let difference = (player.translation + Vec3::Y) - camera_position;
    camera.target_focus += difference * 0.25;
}
