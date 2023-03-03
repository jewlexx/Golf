use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;

pub const MULTIPLIER: f32 = 20.;
const VERY_SLOW: f32 = 30.;
const MIN_VAL: f32 = 10.;

// TODO: Fix degradation, to not "ease out"
pub fn apply_velocity(mut query: Query<(&mut Velocity, With<Ball>)>) {
    for (mut velocity, _) in query.iter_mut() {
        if velocity.linvel.x.abs() < MIN_VAL && velocity.linvel.y.abs() < MIN_VAL {
            velocity.linvel = Vec2::ZERO;
        } else if velocity.linvel.x.abs() < VERY_SLOW && velocity.linvel.y.abs() < VERY_SLOW {
            velocity.linvel *= 0.8;
        } else {
            velocity.linvel *= 0.99;
        }
    }
}
