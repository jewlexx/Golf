#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![feature(let_chains)]
#![feature(const_fn_floating_point_arithmetic)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tileset::prelude::*;

use components::{ball::Ball, vel, walls};

mod components;
mod graphics;

#[must_use]
pub fn normalize(vec: Vec2, max: f32) -> Vec2 {
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

pub fn print_ball_altitude(bodies: Query<&Transform, With<RigidBody>>) {
    for pos in bodies.iter() {
        dbg!(pos.translation);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Mini Golf".to_string(),
            resizable: false,
            ..default()
        },
        ..default()
    }))
    .add_plugin(TilesetPlugin::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .init_resource::<graphics::tiles::Background>();

    #[cfg(feature = "debug_render")]
    app.add_plugin(RapierDebugRenderPlugin::default());

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin);

    app.add_startup_system(graphics::tiles::load)
        .add_startup_system(graphics::camera::setup)
        .add_startup_system(walls::init)
        .add_startup_system(Ball::init)
        .add_system(graphics::tiles::show)
        .add_system(Ball::move_ball)
        .add_system(vel::apply_velocity)
        // .add_system(print_ball_altitude)
        .run();
}

#[cfg(test)]
mod tests {
    use super::{nearest_multiple, SCREEN_HEIGHT, SCREEN_WIDTH};

    #[test]
    fn test_nearest_multiple() {
        assert_eq!(nearest_multiple(99, 10), 100);

        // Ensure screen width and height are correct
        assert_eq!(nearest_multiple(900, 48), 912);
        assert_eq!(nearest_multiple(600, 48), 624);
    }
}
