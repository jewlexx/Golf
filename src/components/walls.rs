use bevy::prelude::*;

use super::{ball::Ball, vel::Velocity};

// x coordinates
const RIGHT_WALL: f32 = crate::SCREEN_WIDTH as f32 / 2. - Ball::RADIUS;
const LEFT_WALL: f32 = RIGHT_WALL * -1.;
// y coordinates
const TOP_WALL: f32 = crate::SCREEN_HEIGHT as f32 / 2. - Ball::RADIUS;
const BOTTOM_WALL: f32 = TOP_WALL * -1.;

enum Axis {
    X,
    Y,
}

fn invert_velocity(velocity: &mut Velocity, axis: Axis) {
    match axis {
        Axis::X => velocity.get_mut().x *= -1.,
        Axis::Y => velocity.get_mut().y *= -1.,
    }
}

/// Checks if the ball has reached any of the edges of the screen
/// Will assign the ball's postion to said edge, to ensure that it does not clip out of bounds
pub(crate) fn check_collide(
    sfx: Res<crate::audio::Sfx>,
    audio: Res<Audio>,
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let (mut pos, mut vel) = balls.get_single_mut().unwrap();

    let mut did_hit = false;

    // TODO: Find a better way to check collisions, than 4 different if statements, preferably that results in one code path
    if pos.translation.x < LEFT_WALL {
        did_hit = true;
        pos.translation.x = LEFT_WALL;

        invert_velocity(&mut vel, Axis::X);
    }

    if pos.translation.x > RIGHT_WALL {
        did_hit = true;
        pos.translation.x = RIGHT_WALL;

        invert_velocity(&mut vel, Axis::X);
    }

    if pos.translation.y < BOTTOM_WALL {
        did_hit = true;
        pos.translation.y = BOTTOM_WALL;

        invert_velocity(&mut vel, Axis::Y);
    }

    if pos.translation.y > TOP_WALL {
        did_hit = true;
        pos.translation.y = TOP_WALL;

        invert_velocity(&mut vel, Axis::Y);
    }

    if did_hit {
        if let Some(hit_sfx) = &sfx.hit {
            dbg!(&hit_sfx);
            audio.play(hit_sfx.clone());
        }
    }
}
