use bevy::prelude::*;
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

#[derive(Component)]
pub struct CurrentPosition {
    pub x: usize,
    pub y: usize
}

fn spawn_current_pos(
    mut commands: Commands
) {
    commands.spawn((
        CurrentPosition {
            x: 0,
            y: 0
        },
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
        let (x, y) = (e.x, e.y);

        if e.button == Right && position_in_map((x, y)) {
            let (mut pos, mut transform) = query.single_mut();
            pos.x = x;
            pos.y = y;
            transform.translation = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 10.0);

            event_writer.send(EMustUpdateLos);
            return;
        }
    }
}

fn position_in_map((x, y): (usize, usize)) -> bool {
    x < MAP_WIDTH && y < MAP_HEIGHT
}