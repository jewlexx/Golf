use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;

const BALL_OFFSET: f32 = Ball::RADIUS * 2.;
const ADDITIVE: f32 = 50.;

// x coordinates
const LEFT_WALL: f32 = crate::SCREEN_WIDTH / 2. * -1. - ADDITIVE;
const RIGHT_WALL: f32 = crate::SCREEN_WIDTH / 2. + ADDITIVE;
// y coordinates
const BOTTOM_WALL: f32 = crate::SCREEN_HEIGHT / 2. * -1. + BALL_OFFSET - ADDITIVE;
const TOP_WALL: f32 = crate::SCREEN_HEIGHT / 2. - BALL_OFFSET + ADDITIVE;
const WALL_Z: f32 = 0.5;

pub fn init(mut commands: Commands) {
    // Create the ground
    commands.spawn((
        Collider::cuboid(crate::SCREEN_WIDTH, 50.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(crate::SCREEN_WIDTH, 50.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, BOTTOM_WALL, WALL_Z),
            ..Default::default()
        },
    ));

    // Create the roof
    commands.spawn((
        Collider::cuboid(crate::SCREEN_WIDTH, 50.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(crate::SCREEN_WIDTH, 50.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, TOP_WALL, WALL_Z),
            ..Default::default()
        },
    ));

    // Create the left wall
    commands.spawn((
        Collider::cuboid(50., crate::SCREEN_HEIGHT),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(50., crate::SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(LEFT_WALL, 0., WALL_Z),
            ..Default::default()
        },
    ));

    // Create the right wall
    commands.spawn((
        Collider::cuboid(50., crate::SCREEN_HEIGHT),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(50., crate::SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(RIGHT_WALL, 0., WALL_Z),
            ..Default::default()
        },
    ));
}
