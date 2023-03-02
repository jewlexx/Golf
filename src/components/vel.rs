use bevy::prelude::*;

#[derive(Default, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub const MULTIPLIER: f32 = 5.;

    // TODO: Fix degradation, to not "ease out"
    pub fn degrade(&mut self) {
        if self.0.x.abs() < 5. && self.0.y.abs() < 5. {
            self.0 = Vec2::ZERO;
        } else {
            self.0 *= 0.99;
        }
    }
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        velocity.degrade();
    }
}
