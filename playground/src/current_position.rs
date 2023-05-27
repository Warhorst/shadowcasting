use bevy::prelude::*;
use pad::{Position, p};
use crate::constants::*;
use crate::line_of_sight::EMustUpdateLos;
use crate::mouse_cursor::EMouseClicked;
use crate::mouse_cursor::PressedButton::Right;

pub (super) struct CurrentPositionPlugin;

impl Plugin for CurrentPositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_current_pos)
            .add_system(move_position_when_right_mouse_button_clicked)
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
    mut query: Query<(&mut CurrentPosition, &mut Transform)>
) {
    for e in event_reader.iter() {
        let (x, y) = (e.pos.x, e.pos.y);

        if e.button == Right && position_in_map(e.pos) {
            let (mut pos, mut transform) = query.single_mut();
            pos.x = x;
            pos.y = y;
            transform.translation = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 10.0);

            event_writer.send(EMustUpdateLos);
            return;
        }
    }
}

fn position_in_map(pos: Position) -> bool {
    pos.x < MAP_WIDTH as isize && pos.y < MAP_HEIGHT as isize
}