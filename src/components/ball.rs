use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{calc_diff, normalize};

use super::vel::Velocity;

#[derive(Default, Component)]
pub struct Ball {
    pub mouse_start: Option<Vec2>,
}

impl Ball {
    pub const STARTING_POS: Vec3 = Vec3::ZERO;
    pub const RADIUS: f32 = 15.;
    pub const SIZE: Vec2 = Vec2::splat(Self::RADIUS * 2.);
    pub const COLOUR: Color = Color::WHITE;

    pub fn init(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // Create the ground (temp)
        commands
            .spawn(Collider::cuboid(500.0, 50.0))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, -400.0, 0.0)));

        // Ball
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Ball::COLOUR)),
                transform: Transform::from_translation(Ball::STARTING_POS)
                    .with_scale(Vec3::splat(15.0)),
                ..default()
            },
            Ball::default(),
            Velocity(Vec2::ZERO),
            RigidBody::Dynamic,
            Collider::ball(Ball::SIZE.x / 2.),
            Restitution::coefficient(1.),
        ));
    }

    // TODO: Implement more of a drag system than a distance from ball system
    pub fn move_ball(
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
                let normalized_diff = normalize(mouse_diff, 100.) * Velocity::MULTIPLIER;
                dbg!(mouse_start - mouse_pos, normalized_diff);
                velocity.x -= normalized_diff.x;
                velocity.y -= normalized_diff.y;
                ball.mouse_start = None;
            }
        }
    }
}
