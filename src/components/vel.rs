use bevy::prelude::*;

use super::ball::Ball;

pub(crate) const MULTIPLIER: f32 = 20.;
const VERY_SLOW: f32 = 30.;
const MIN_VAL: f32 = 10.;

#[derive(Debug, Default, Copy, Clone, Component)]
pub(crate) struct Velocity {
    pub(crate) inner: Vec2,
}

impl Velocity {
    pub(crate) fn x(self) -> f32 {
        self.inner.x
    }

    pub(crate) fn y(self) -> f32 {
        self.inner.y
    }

    pub(crate) fn get(self) -> Vec2 {
        self.inner
    }

    pub(crate) fn get_mut(&mut self) -> &mut Vec2 {
        &mut self.inner
    }

    pub(crate) fn set(&mut self, vel: Vec2) {
        self.inner = vel;
    }

    pub(crate) fn reset(&mut self) {
        self.inner = Vec2::ZERO;
    }

    pub(crate) fn add_x(&mut self, vel: f32) {
        self.inner.x += vel;
    }

    pub(crate) fn add_y(&mut self, vel: f32) {
        self.inner.y += vel;
    }

    pub(crate) fn sub_x(&mut self, vel: f32) {
        self.inner.x -= vel;
    }

    pub(crate) fn sub_y(&mut self, vel: f32) {
        self.inner.y -= vel;
    }

    pub(crate) fn mul_x(&mut self, mul: f32) {
        self.inner.x *= mul;
    }

    pub(crate) fn mul_y(&mut self, mul: f32) {
        self.inner.y *= mul;
    }

    pub(crate) fn div_x(&mut self, div: f32) {
        self.inner.x /= div;
    }

    pub(crate) fn div_y(&mut self, div: f32) {
        self.inner.y /= div;
    }

    pub(crate) fn add_xy(&mut self, vel: f32) {
        self.inner.x += vel;
        self.inner.y += vel;
    }

    pub(crate) fn sub_xy(&mut self, vel: f32) {
        self.inner.x -= vel;
        self.inner.y -= vel;
    }

    pub(crate) fn mul_xy(&mut self, mul: f32) {
        self.inner.x *= mul;
        self.inner.y *= mul;
    }

    pub(crate) fn div_xy(&mut self, vel: f32) {
        self.inner.x /= vel;
        self.inner.y /= vel;
    }
}

// TODO: Fix degradation, to not "ease out"
pub(crate) fn apply_velocity(mut query: Query<(&mut Velocity, With<Ball>)>) {
    for (mut velocity, _) in query.iter_mut() {
        if velocity.x().abs() < MIN_VAL && velocity.y().abs() < MIN_VAL {
            velocity.set(Vec2::ZERO);
        } else if velocity.x().abs() < VERY_SLOW && velocity.y().abs() < VERY_SLOW {
            *velocity.get_mut() *= 0.8;
        } else {
            *velocity.get_mut() *= 0.99;
        }
    }
}
