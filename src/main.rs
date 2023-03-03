#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

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

const SCREEN_WIDTH: f32 = 900.;
const SCREEN_HEIGHT: f32 = 600.;

fn calc_diff(a: Vec2, b: Vec2) -> Vec2 {
    a - b
}

fn main() {
    let mut app = App::new();

    #[cfg(features = "debug-render")]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                title: "Mini Golf".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }), // .set(AssetPlugin {
            //     watch_for_changes: cfg!(debug_assertions),
            //     ..default()
            // }),
    )
    .add_plugin(TilesetPlugin::default())
    .init_resource::<graphics::tiles::Background>()
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_startup_system(graphics::tiles::load)
    .add_startup_system(graphics::setup)
    .add_startup_system(walls::init)
    .add_startup_system(Ball::init)
    .add_system(graphics::tiles::show)
    .add_system(Ball::move_ball)
    .add_system(vel::apply_velocity)
    .run();
}
