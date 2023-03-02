use bevy::prelude::*;

pub mod tiles;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
