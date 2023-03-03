use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;

// x coordinates
const LEFT_WALL: f32 = crate::SCREEN_WIDTH * -1. + Ball::RADIUS;
const RIGHT_WALL: f32 = 0.;
// y coordinates
const BOTTOM_WALL: f32 = crate::SCREEN_HEIGHT * -1. + Ball::RADIUS;
const TOP_WALL: f32 = 0.;

pub fn init(mut commands: Commands) {
    // Create the ground
    commands.spawn((
        Collider::cuboid(crate::SCREEN_WIDTH, 50.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(crate::SCREEN_WIDTH, 50.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, BOTTOM_WALL * 2., 0.0),
            ..Default::default()
        },
    ));

    // Create the left wall
    commands.spawn((
        Collider::cuboid(50., crate::SCREEN_HEIGHT),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50., crate::SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(LEFT_WALL, 0., 0.),
            ..Default::default()
        },
    ));
}
