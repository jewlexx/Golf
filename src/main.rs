#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, DefaultPlugins};

#[must_use]
pub fn normalize(vec: Vec2, max: f32) -> Vec2 {
    let mut v = vec;

    if v.x > max {
        v.x = max;
    } else if v.x < -max {
        v.x = -max;
    }
    if v.y > max {
        v.y = max;
    } else if v.y < -max {
        v.y = -max;
    }

    v
}

#[derive(Default, Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Default, Component)]
struct Ball {
    mouse_start: Option<Vec2>,
}

impl Ball {
    const STARTING_POS: Vec3 = Vec3::ZERO;
    const RADIUS: f32 = 30.0;
    const SIZE: Vec3 = Vec3::splat(Self::RADIUS);
    const COLOUR: Color = Color::WHITE;
}

fn init_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Ball::COLOUR)),
            transform: Transform::from_translation(Ball::STARTING_POS).with_scale(Ball::SIZE),
            ..default()
        },
        Ball::default(),
        Velocity(Vec2::ZERO),
    ));
}

// TODO: Fix degradation, to not "ease out"
fn degrade_velocity(velocity: &mut Velocity) {
    velocity.0 *= 0.99;
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        degrade_velocity(velocity.as_mut());
    }
}

fn calc_diff(a: Vec2, b: Vec2) -> Vec2 {
    a - b
}

// TODO: Implement more of a drag system than a distance from ball system
fn move_ball(
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Ball, &mut Velocity)>,
) {
    let window = windows.get_primary().expect("single window");

    debug_assert_eq!(query.iter().count(), 1);
    let (mut transform, mut ball, mut velocity) = query.iter_mut().next().unwrap();

    if keyboard.just_pressed(KeyCode::R) {
        velocity.0 = Vec2::ZERO;
        transform.translation = Vec3::ZERO;
        ball.mouse_start = None;

        return;
    }

    if let Some(mouse_pos) = window.cursor_position() {
        if mouse_buttons.pressed(MouseButton::Left) {
            if ball.mouse_start.is_none() {
                ball.mouse_start = Some(mouse_pos);
            }
        } else if let Some(mouse_start) = ball.mouse_start {
            let mouse_diff = calc_diff(mouse_start, mouse_pos) * -1.;
            let normalized_diff = normalize(mouse_diff, 100.) * 7.5;
            dbg!(mouse_start - mouse_pos, normalized_diff);
            velocity.x -= normalized_diff.x;
            velocity.y -= normalized_diff.y;
            ball.mouse_start = None;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_ball)
        .add_system(move_ball)
        .add_system(apply_velocity.after(move_ball))
        .run();
}
