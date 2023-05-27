use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use pad::{p, Position};
use crate::constants::TILE_SIZE;

pub(super) struct MouseCoursorPlugin;

impl Plugin for MouseCoursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EMouseClicked>()
            .init_resource::<CursorCoordinates>()
            .add_systems((
                update_cursor_position,
                send_event_when_mouse_clicked
            ))
        ;
    }
}

#[derive(Default, Deref, DerefMut, Resource)]
pub struct CursorCoordinates(Vec2);

pub struct EMouseClicked {
    pub button: PressedButton,
    pub pos: Position,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PressedButton {
    Left,
    Right,
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorCoordinates>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = cameras.single();
    let window = windows.single();

    if let Some(position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let ndc = (position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();
        **cursor_position = world_pos
    }
}

fn send_event_when_mouse_clicked(
    cursor_position: Res<CursorCoordinates>,
    mouse_input: Res<Input<MouseButton>>,
    mut event_writer: EventWriter<EMouseClicked>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let pos = coordinates_to_xy(**cursor_position);

        event_writer.send(EMouseClicked {
            pos,
            button: PressedButton::Left,
        });
    }

    if mouse_input.just_pressed(MouseButton::Right) {
        let pos = coordinates_to_xy(**cursor_position);

        event_writer.send(EMouseClicked {
            pos,
            button: PressedButton::Right,
        });
    }
}

fn coordinates_to_xy(coordinates: Vec2) -> Position {
    p!(
        (coordinates.x + (TILE_SIZE / 2.0)) / TILE_SIZE,
        (coordinates.y + (TILE_SIZE / 2.0)) / TILE_SIZE
    )
}