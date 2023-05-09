use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_systems((
                zoom_with_mouse_wheel,
                move_camera_with_keyboard_input
            ))
        ;
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 2.0,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(15.0 * 32.0, 15.0 * 32.0, 1000.0)),
            ..default()
        }
    );
}

fn zoom_with_mouse_wheel(
    time: Res<Time>,
    mut event_reader: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let dist = 20.0 * time.delta().as_secs_f32();

    for ev in event_reader.iter() {
        for mut projection in &mut query {
            let scroll = match ev.unit {
                MouseScrollUnit::Line => ev.y,
                MouseScrollUnit::Pixel => ev.y,
            };

            let log_scale = projection.scale.ln();
            projection.scale = (log_scale - dist * scroll).exp()
        }
    }
}

fn move_camera_with_keyboard_input(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in &mut query {
        let change = time.delta_seconds() * 300.0;

        if key_input.pressed(KeyCode::W) {
            transform.translation.y += change
        }

        if key_input.pressed(KeyCode::S) {
            transform.translation.y -= change
        }

        if key_input.pressed(KeyCode::A) {
            transform.translation.x -= change
        }

        if key_input.pressed(KeyCode::D) {
            transform.translation.x += change
        }
    }
}