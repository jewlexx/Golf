use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;

// x coordinates
const RIGHT_WALL: f32 = crate::SCREEN_WIDTH / 2. - Ball::RADIUS;
const LEFT_WALL: f32 = RIGHT_WALL * -1.;
// y coordinates
const TOP_WALL: f32 = crate::SCREEN_HEIGHT / 2. - Ball::RADIUS;
const BOTTOM_WALL: f32 = TOP_WALL * -1.;

enum Axis {
    X,
    Y,
}

fn invert_velocity(velocity: &mut Velocity, axis: Axis) {
    match axis {
        Axis::X => velocity.linvel.x *= -1.,
        Axis::Y => velocity.linvel.y *= -1.,
    }
}

/// Checks if the ball has reached any of the edges of the screen
/// Will assign the ball's postion to said edge, to ensure that it does not clip out of bounds
pub(crate) fn check_collide(mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    let (mut pos, mut vel) = balls.get_single_mut().unwrap();

    if pos.translation.x < LEFT_WALL {
        pos.translation.x = LEFT_WALL;

        invert_velocity(&mut vel, Axis::X);
    }

    if pos.translation.x > RIGHT_WALL {
        pos.translation.x = RIGHT_WALL;

        invert_velocity(&mut vel, Axis::X);
    }

    if pos.translation.y < BOTTOM_WALL {
        pos.translation.y = BOTTOM_WALL;

        invert_velocity(&mut vel, Axis::Y);
    }

    if pos.translation.y > TOP_WALL {
        pos.translation.y = TOP_WALL;

        invert_velocity(&mut vel, Axis::Y);
    }
}
