use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;

const BALL_OFFSET: f32 = Ball::RADIUS * 2.;
const ADDITIVE: f32 = 1.;

// x coordinates
const LEFT_WALL: f32 = crate::SCREEN_WIDTH / 2. * -1. - ADDITIVE;
const RIGHT_WALL: f32 = crate::SCREEN_WIDTH / 2. + ADDITIVE;
// y coordinates
const BOTTOM_WALL: f32 = crate::SCREEN_HEIGHT / 2. * -1. + BALL_OFFSET - ADDITIVE;
const TOP_WALL: f32 = crate::SCREEN_HEIGHT / 2. - BALL_OFFSET + ADDITIVE;

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
            transform: Transform::from_xyz(0.0, BOTTOM_WALL, 0.5),
            ..Default::default()
        },
    ));

    // Create the roof
    commands.spawn((
        Collider::cuboid(crate::SCREEN_WIDTH, 50.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(crate::SCREEN_WIDTH, 50.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, TOP_WALL, 0.5),
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
            transform: Transform::from_xyz(LEFT_WALL, 0., 0.5),
            ..Default::default()
        },
    ));

    // Create the right wall
    commands.spawn((
        Collider::cuboid(50., crate::SCREEN_HEIGHT),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50., crate::SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(RIGHT_WALL, 0., 0.5),
            ..Default::default()
        },
    ));
}
