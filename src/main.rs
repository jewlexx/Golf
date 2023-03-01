use bevy::{prelude::*, sprite::MaterialMesh2dBundle, DefaultPlugins};

#[derive(Default, Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball {
    pos: Vec3,
    radius: f32,
    mouse_diff: Option<Vec2>,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            pos: Self::STARTING_POS,
            radius: Self::RADIUS,
            mouse_diff: None,
        }
    }
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

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

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
        ball.mouse_diff = None;

        return;
    }

    if let Some(mouse_pos) = window.cursor_position() {
        if mouse_buttons.pressed(MouseButton::Left) {
            // Convert ball translation to window logical position
            let ball_x = transform.translation.x + window.width() / 2.;
            let ball_y = transform.translation.y + window.height() / 2.;

            let mouse_x = mouse_pos.x;
            let mouse_y = mouse_pos.y;

            let diff_x = ball_x - mouse_x;
            let diff_y = ball_y - mouse_y;

            ball.mouse_diff = Some(Vec2::new(diff_x, diff_y) * -1.);
        } else if let Some(mouse_diff) = ball.mouse_diff {
            velocity.x -= mouse_diff.x;
            velocity.y -= mouse_diff.y;
            ball.mouse_diff = None;
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
