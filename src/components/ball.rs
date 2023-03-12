use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{calc_diff, normalize};

use super::vel;

#[derive(Default, Component)]
pub struct Ball {
    pub mouse_start: Option<Vec2>,
}

impl Ball {
    pub const STARTING_POS: Vec3 = Vec3::new(0., 0., 1.);
    pub const RADIUS: f32 = 15.;
    pub const COLOUR: Color = Color::WHITE;

    pub fn init(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // Ball
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Ball::COLOUR)),
                transform: Transform::from_translation(Ball::STARTING_POS)
                    .with_scale(Vec3::splat(Ball::RADIUS * 2.)),
                ..default()
            },
            Ball::default(),
            RigidBody::Dynamic,
            // Disable gravity as we will be moving it ourselves
            GravityScale(0.),
            Collider::ball(0.5),
            Restitution::coefficient(1.5),
            Velocity {
                linvel: Vec2::ZERO,
                ..Default::default()
            },
        ));
    }

    // TODO: Implement more of a drag system than a distance from ball system
    pub fn move_ball(
        windows: Query<&Window, With<PrimaryWindow>>,
        mouse_buttons: Res<Input<MouseButton>>,
        keyboard: Res<Input<KeyCode>>,
        mut query: Query<(&mut Transform, &mut Ball, &mut Velocity)>,
    ) {
        let window = windows.get_single().expect("single window");

        debug_assert_eq!(query.iter().count(), 1);
        let (mut transform, mut ball, mut velocity) = query.iter_mut().next().unwrap();

        if keyboard.just_pressed(KeyCode::R) {
            velocity.linvel = Vec2::ZERO;
            velocity.angvel = 0.;
            transform.translation = Ball::STARTING_POS;
            ball.mouse_start = None;

            return;
        }

        if let Some(mouse_pos) = window.cursor_position() {
            if mouse_buttons.pressed(MouseButton::Left) {
                if ball.mouse_start.is_none() {
                    ball.mouse_start = Some(mouse_pos);
                }
            } else if let Some(mouse_start) = ball.mouse_start {
                if velocity.linvel == Vec2::ZERO {
                    let mouse_diff = calc_diff(mouse_start, mouse_pos) * -1.;
                    let normalized_diff = normalize(mouse_diff, 100.) * vel::MULTIPLIER;
                    dbg!(mouse_start - mouse_pos, normalized_diff);
                    velocity.linvel.x -= normalized_diff.x;
                    velocity.linvel.y -= normalized_diff.y;
                }

                ball.mouse_start = None;
            }
        }
    }
}
