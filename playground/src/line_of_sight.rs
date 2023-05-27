use std::collections::HashMap;
use bevy::prelude::*;
use bevy::utils::HashSet;
use pad::{p, Position};
use shadowcasting::ShadowCasting;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::current_position::CurrentPosition;
use crate::map::{Tile, TileType};

pub(super) struct LineOfSightPlugin;

impl Plugin for LineOfSightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EMustUpdateLos>()
            .add_startup_system(create_los)
            .add_system(update_los)
        ;
    }
}

#[derive(Resource)]
pub struct LineOfSight(HashSet<Position>);

impl LineOfSight {
    pub fn position_visible(&self, pos: Position) -> bool {
        self.0.contains(&pos)
    }
}

pub struct EMustUpdateLos;

fn create_los(
    mut commands: Commands
) {
    let positions = p!(0, 0)
        .iter_to(p!(MAP_WIDTH, MAP_HEIGHT))
        .into_iter()
        .collect();

    commands.insert_resource(LineOfSight(positions))
}

fn update_los(
    mut los: ResMut<LineOfSight>,
    mut event_reader: EventReader<EMustUpdateLos>,
    pos_query: Query<&CurrentPosition>,
    tile_query: Query<&Tile>,
) {
    let pos_type_map = tile_query
        .iter()
        .map(|tile| (tile.pos, tile.tile_type))
        .collect::<HashMap<_, _>>();

    for _ in event_reader.iter() {
        let pos = pos_query.single();

        let shadow_casting = ShadowCasting::new(
            (pos.x as isize, pos.y as isize),
            |pos| match pos_type_map.get(&p!(pos.0, pos.1)) {
                Some(tile_type) => match tile_type {
                    TileType::Wall => true,
                    TileType::Floor => false,
                }
                None => false
            },
            30,
        );


        let visible_points = shadow_casting.calculate_los()
            .into_iter()
            .map(|(x, y)| p!(x, y))
            .collect();
        *los = LineOfSight(visible_points);
    }
}