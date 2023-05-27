use bevy::prelude::*;
use pad::{Position, p};
use pad::Direction::{XM, XP, YM, YP};
use crate::constants::*;
use crate::line_of_sight::EMustUpdateLos;
use crate::mouse_cursor::EMouseClicked;
use crate::mouse_cursor::PressedButton::Right;

pub(super) struct CurrentPositionPlugin;

impl Plugin for CurrentPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_current_pos)
            .add_systems((
                move_position_when_right_mouse_button_clicked,
                move_position_when_arrow_key_was_pressed,
                update_transform_when_position_changed
            ))
        ;
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct CurrentPosition(Position);

fn spawn_current_pos(
    mut commands: Commands
) {
    commands.spawn((
        CurrentPosition(p!(0, 0)),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        }
    ));
}

fn move_position_when_right_mouse_button_clicked(
    mut event_reader: EventReader<EMouseClicked>,
    mut event_writer: EventWriter<EMustUpdateLos>,
    mut query: Query<&mut CurrentPosition>,
) {
    for e in event_reader.iter() {
        let (x, y) = (e.pos.x, e.pos.y);

        if e.button == Right && position_in_map(e.pos) {
            let mut pos = query.single_mut();
            pos.x = x;
            pos.y = y;

            event_writer.send(EMustUpdateLos);
            return;
        }
    }
}

fn move_position_when_arrow_key_was_pressed(
    input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<EMustUpdateLos>,
    mut query: Query<&mut CurrentPosition>,
) {
    for mut pos in &mut query {
        if input.just_pressed(KeyCode::Up) {
            **pos = pos.neighbour_in_direction(YP);
            event_writer.send(EMustUpdateLos)
        }

        if input.just_pressed(KeyCode::Down) {
            **pos = pos.neighbour_in_direction(YM);
            event_writer.send(EMustUpdateLos)
        }

        if input.just_pressed(KeyCode::Left) {
            **pos = pos.neighbour_in_direction(XM);
            event_writer.send(EMustUpdateLos)
        }

        if input.just_pressed(KeyCode::Right) {
            **pos = pos.neighbour_in_direction(XP);
            event_writer.send(EMustUpdateLos)
        }
    }
}

fn update_transform_when_position_changed(
    mut query: Query<(&CurrentPosition, &mut Transform), Changed<CurrentPosition>>
) {
    for (pos, mut transform) in &mut query {
        transform.translation = Vec3::new(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE, 10.0);
    }
}

fn position_in_map(pos: Position) -> bool {
    pos.x < MAP_WIDTH as isize && pos.y < MAP_HEIGHT as isize
}