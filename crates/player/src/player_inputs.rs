use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInputs {
    pub forward: bool,
    pub backwards: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub fly: bool,
    pub up: bool,
    pub down: bool,
}

pub fn update_player_inputs(
    mut player_inputs: ResMut<PlayerInputs>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    player_inputs.forward = keyboard_input.pressed(KeyCode::KeyW);
    player_inputs.backwards = keyboard_input.pressed(KeyCode::KeyS);
    player_inputs.left = keyboard_input.pressed(KeyCode::KeyA);
    player_inputs.right = keyboard_input.pressed(KeyCode::KeyD);
    player_inputs.jump = keyboard_input.pressed(KeyCode::Space);
    player_inputs.sprint = keyboard_input.pressed(KeyCode::ShiftLeft);
    player_inputs.fly = keyboard_input.just_pressed(KeyCode::KeyF);
    player_inputs.up = keyboard_input.pressed(KeyCode::KeyE);
    player_inputs.down = keyboard_input.pressed(KeyCode::KeyQ);
}
