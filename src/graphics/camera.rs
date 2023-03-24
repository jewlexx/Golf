use bevy::prelude::*;

pub(crate) fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub(crate) fn shift(mut camera: Query<&mut Transform, With<Camera>>, inputs: Res<Input<KeyCode>>) {
    for mut transform in camera.iter_mut() {
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

        transform.translation += translation;
    }
}
