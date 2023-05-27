use bevy::prelude::*;
use TileType::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::line_of_sight::{EMustUpdateLos, LineOfSight};
use crate::mouse_cursor::EMouseClicked;
use crate::mouse_cursor::PressedButton::Left;
use pad::{Position, p};

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_map)
            .add_systems((
                toggle_type_on_left_mouse_button_click,
                update_tile_visibility_when_los_changed,
                update_color_when_tile_changed
            ))
        ;
    }
}

#[derive(Component)]
pub struct Tile {
    pub pos: Position,
    pub tile_type: TileType,
    pub is_visible: bool,
}

impl Tile {
    fn toggle_type(&mut self) {
        self.tile_type = match self.tile_type {
            Floor => Wall,
            Wall => Floor
        };
    }
}

#[derive(Copy, Clone)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    fn color(&self) -> Color {
        match self {
            Floor => Color::rgba_u8(196, 164, 132, 255),
            Wall => Color::rgba_u8(101, 67, 33, 255)
        }
    }
}

fn spawn_map(
    mut commands: Commands
) {
    for pos in p!(0,0).iter_to(p!(MAP_WIDTH, MAP_HEIGHT)) {
        let tile_type = Floor;

        commands.spawn((
            Tile {
                pos,
                tile_type,
                is_visible: true,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    color: tile_type.color(),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE, 0.0)),
                ..default()
            }
        ));
    }
}

fn toggle_type_on_left_mouse_button_click(
    mut event_reader: EventReader<EMouseClicked>,
    mut event_writer: EventWriter<EMustUpdateLos>,
    mut query: Query<&mut Tile>,
) {
    for e in event_reader.iter() {
        if e.button == Left {
            let tile_opt = query.iter_mut().find(|tile| tile.pos == e.pos);

            if let Some(mut tile) = tile_opt {
                tile.toggle_type();
                event_writer.send(EMustUpdateLos);
                return;
            }
        }
    }
}

fn update_tile_visibility_when_los_changed(
    los: Res<LineOfSight>,
    mut query: Query<&mut Tile>,
) {
    if !los.is_changed() {
        return;
    }

    query.iter_mut().for_each(|mut tile| if los.position_visible(tile.pos) {
        tile.is_visible = true;
    } else {
        tile.is_visible = false;
    })
}

fn update_color_when_tile_changed(
    mut query: Query<(&Tile, &mut Sprite), Changed<Tile>>
) {
    for (tile, mut sprite) in &mut query {
        if tile.is_visible {
            sprite.color = tile.tile_type.color();
        } else {
            sprite.color = Color::BLACK;
        }
    }
}