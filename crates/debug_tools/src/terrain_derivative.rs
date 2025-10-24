use bevy::prelude::*;
use bevy_hookup_core::{owner_component::Owner, shared::Shared};
use debug_resource::debug_resource::SpellhavenDebugResource;
use player::player_component::Player;
use world_generation::{
    chunk_generation::noise::noise_function::NoiseFunction,
    generation_options::GenerationOptions,
};

pub fn draw_terrain_derivative(
    mut gizmos: Gizmos,
    generation_options: Single<&Shared<GenerationOptions>>,
    player: Single<&Transform, With<Owner<Player>>>,
    debug_resource: Res<SpellhavenDebugResource>,
) {
    if !debug_resource.show_derivative_debug {
        return;
    }

    let terrain_noise = generation_options.get_terrain_noise();

    let noise_result =
        terrain_noise.get(player.translation.xz().as_dvec2().to_array());

    let derivative = Vec3::new(
        noise_result.derivative[0] as f32,
        0.,
        noise_result.derivative[1] as f32,
    );

    gizmos.arrow(
        player.translation,
        player.translation + derivative * Vec3::X,
        Color::linear_rgb(1., 0., 0.),
    );

    gizmos.arrow(
        player.translation,
        player.translation + derivative * Vec3::Z,
        Color::linear_rgb(0., 0., 1.),
    );
}
