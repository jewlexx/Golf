// Disable the terminal on Windows
#![cfg_attr(windows, windows_subsystem = "windows")]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![feature(let_chains)]
#![feature(const_fn_floating_point_arithmetic)]

use bevy::{prelude::*, window::WindowResolution};

use components::{ball::Ball, vel, walls};

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
///  Kinda hacky solution but it works
const fn nearest_multiple(base: u16, multiple: u16) -> u16 {
    base - (base % multiple) + multiple
}

// Calculates the nearest multiple of 48, higher than the given base
// Useful for tiling as our textures are 48 square pixels
const SCREEN_WIDTH: f32 = nearest_multiple(900, 48) as f32;
const SCREEN_HEIGHT: f32 = nearest_multiple(600, 48) as f32;

fn calc_diff(a: Vec2, b: Vec2) -> Vec2 {
    a - b
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            title: "Mini Golf".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(ClearColor(Color::rgb_u8(131, 224, 76)));

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::default());

    #[cfg(debug_assertions)]
    app.add_system(graphics::camera::shift);

    app.add_startup_system(graphics::camera::setup)
        // .add_startup_system(walls::init)
        .add_startup_system(Ball::init)
        .add_system(walls::check_collide)
        .add_system(Ball::move_ball)
        .add_system(vel::apply_velocity)
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
