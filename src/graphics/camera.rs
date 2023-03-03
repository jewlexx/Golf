use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn shift(mut camera: Query<&mut Transform, With<Camera>>, inputs: Res<Input<KeyCode>>) {
    for mut cam in camera.iter_mut() {
        let mut translation = Vec3::ZERO;

        if inputs.pressed(KeyCode::A) {
            translation.x -= 1.0;
        }
        if inputs.pressed(KeyCode::D) {
            translation.x += 1.0;
        }
        if inputs.pressed(KeyCode::W) {
            translation.y += 1.0;
        }
        if inputs.pressed(KeyCode::S) {
            translation.y -= 1.0;
        }

        cam.translation += translation;
    }
}
