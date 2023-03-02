use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;

pub const MULTIPLIER: f32 = 5.;

// TODO: Fix degradation, to not "ease out"
pub fn apply_velocity(mut query: Query<(&mut Velocity, With<Ball>)>) {
    for (mut velocity, _) in query.iter_mut() {
        if velocity.linvel.x.abs() < 5. && velocity.linvel.y.abs() < 5. {
            velocity.linvel = Vec2::ZERO;
        } else {
            velocity.linvel *= 0.99;
        }
    }
}
