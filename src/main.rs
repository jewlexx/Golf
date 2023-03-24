// Disable the terminal on Windows
#![cfg_attr(windows, windows_subsystem = "windows")]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;

use components::{ball::Ball, vel, walls};
use levels::{
    def::{BallStartingPosition, GoalPosition},
    loader::LevelLoader,
};

mod components;
mod graphics;
mod levels;

#[must_use]
pub(crate) fn normalize(vec: Vec2, max: f32) -> Vec2 {
    let mut v = vec;

    if v.x > max {
        v.x = max;
    } else if v.x < -max {
        v.x = -max;
    }
    if v.y > max {
        v.y = max;
    } else if v.y < -max {
        v.y = -max;
    }

    v
}

/// Calculates the first number higher than `base` that is a multiple of `multiple`.
/// Kinda hacky solution but it works
const fn nearest_multiple(base: u16, multiple: u16) -> u16 {
    base - (base % multiple) + multiple
}

// Calculates the nearest multiple of 48, higher than the given base
// Useful for tiling as our textures are 48 square pixels
const SCREEN_WIDTH: u16 = nearest_multiple(900, 48);
const SCREEN_HEIGHT: u16 = nearest_multiple(600, 48);

fn calc_diff(a: Vec2, b: Vec2) -> Vec2 {
    a - b
}

fn print_level_assets(server: Res<AssetServer>) {
    // server.get_
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // Converts screen width and height into [`WindowReselution`]
            resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
            title: "Mini Golf".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(ClearColor(Color::rgb_u8(131, 224, 76)))
        .init_resource::<levels::loader::ActiveLevel>()
        .init_asset_loader::<LevelLoader>();

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::default());

    #[cfg(debug_assertions)]
    app.add_system(graphics::camera::shift);

    app.add_startup_system(graphics::camera::setup)
        // .add_startup_system(walls::init)
        .add_startup_system(Ball::init)
        .add_system(levels::loader::load_current)
        .add_system(walls::check_collide)
        .add_system(Ball::move_ball)
        .add_system(vel::apply_velocity)
        .add_system(print_level_assets)
        .run();
}

#[cfg(test)]
mod tests {
    use super::nearest_multiple;

    #[test]
    fn test_nearest_multiple() {
        assert_eq!(nearest_multiple(99, 10), 100);

        // Ensure screen width and height are correct
        assert_eq!(nearest_multiple(900, 48), 912);
        assert_eq!(nearest_multiple(600, 48), 624);
    }
}
