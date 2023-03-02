#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::{prelude::*, DefaultPlugins};
use bevy_rapier2d::prelude::*;

use components::{
    ball::Ball,
    vel::{apply_velocity, Velocity},
};

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

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

fn calc_diff(a: Vec2, b: Vec2) -> Vec2 {
    a - b
}

fn detect_collisions(mut query: Query<(&mut Transform, &Ball, &mut Velocity)>) {
    for (transform, _, mut velocity) in query.iter_mut() {
        let Vec3 { x, y, .. } = transform.translation;
        // TODO: Find a better way to detect wall collisions (maybe just use wall entities)
        if !(LEFT_WALL..=RIGHT_WALL).contains(&(x + Ball::RADIUS))
            || !(LEFT_WALL..=RIGHT_WALL).contains(&(x - Ball::RADIUS))
        {
            velocity.x *= -1.;
        }
        if !(BOTTOM_WALL..=TOP_WALL).contains(&(y + Ball::RADIUS))
            || !(BOTTOM_WALL..=TOP_WALL).contains(&(y - Ball::RADIUS))
        {
            velocity.y *= -1.;
        }
    }
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn main() {
    let mut app = App::new();

    #[cfg(features = "debug-render")]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 900.,
            height: 600.,
            title: "Mini Golf".to_string(),
            resizable: false,
            ..default()
        },
        ..default()
    }))
    // .insert_resource(ClearColor(Color::rgb(
    //     0xF9 as f32 / 255.0,
    //     0xF9 as f32 / 255.0,
    //     0xFF as f32 / 255.0,
    // )))
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_startup_system(graphics::setup)
    .add_startup_system(Ball::init)
    .add_system(Ball::move_ball)
    // .add_system(apply_velocity)
    // .add_system(detect_collisions)
    .add_system(print_ball_altitude)
    .run();
}
