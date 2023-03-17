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
}

impl std::ops::Add<f32> for Velocity {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            inner: self.inner + rhs,
        }
    }
}

impl std::ops::AddAssign<f32> for Velocity {
    fn add_assign(&mut self, rhs: f32) {
        self.inner += rhs;
    }
}

impl std::ops::Sub<f32> for Velocity {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            inner: self.inner - rhs,
        }
    }
}

impl std::ops::SubAssign<f32> for Velocity {
    fn sub_assign(&mut self, rhs: f32) {
        self.inner -= rhs;
    }
}

impl std::ops::Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            inner: self.inner * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Velocity {
    fn mul_assign(&mut self, rhs: f32) {
        self.inner *= rhs;
    }
}

impl std::ops::Div<f32> for Velocity {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            inner: self.inner / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Velocity {
    fn div_assign(&mut self, rhs: f32) {
        self.inner /= rhs;
    }
}

// TODO: Fix degradation, to not "ease out"
pub(crate) fn apply_velocity(
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    time: Res<Time>,
) {
    let (mut velocity, mut transform) = query.get_single_mut().unwrap();

    if velocity.x().abs() < MIN_VAL && velocity.y().abs() < MIN_VAL {
        velocity.set(Vec2::ZERO);
    } else if velocity.x().abs() < VERY_SLOW && velocity.y().abs() < VERY_SLOW {
        *velocity.get_mut() *= 0.8;
    } else {
        *velocity.get_mut() *= 0.99;
    }

    let delta = time.delta_seconds();

    let delta_applied = *velocity * delta;
    transform.translation += Vec3::new(delta_applied.x(), delta_applied.y(), 0.);
}
