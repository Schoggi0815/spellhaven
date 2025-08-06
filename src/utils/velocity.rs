use bevy::prelude::*;

#[derive(Component, Default)]
pub struct HorizontalVelocity {
    velocities: [f32; 10],
    current_index: usize,
    last_position: Vec2,
}

impl HorizontalVelocity {
    pub fn get_average_velocity(&self) -> f32 {
        let sum: f32 = self.velocities.iter().sum();
        sum / self.velocities.len() as f32
    }
}

pub fn update_velocity(
    velocities: Query<(&mut HorizontalVelocity, &Transform)>,
    time: Res<Time>,
) {
    for (mut velocity, transform) in velocities {
        let current_velocity = (transform.translation.xz()
            - velocity.last_position)
            / time.delta_secs();
        velocity.last_position = transform.translation.xz();

        let new_index =
            (velocity.current_index + 1) % velocity.velocities.len();
        velocity.current_index = new_index;
        velocity.velocities[new_index] = current_velocity.length();
    }
}
