use std::collections::HashMap;
use bevy::prelude::*;
use bevy::utils::HashSet;
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
pub struct LineOfSight(HashSet<(usize, usize)>);

impl LineOfSight {
    pub fn position_visible(&self, x: usize, y: usize) -> bool {
        self.0.contains(&(x, y))
    }
}

pub struct EMustUpdateLos;

fn create_los(
    mut commands: Commands
) {
    let mut positions = HashSet::new();

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            positions.insert((x, y));
        }
    }

    commands.insert_resource(LineOfSight(positions))
}

fn update_los(
    mut los: ResMut<LineOfSight>,
    mut event_reader: EventReader<EMustUpdateLos>,
    pos_query: Query<&CurrentPosition>,
    tile_query: Query<&Tile>,
) {
    let tiles_type_map = tile_query
        .iter()
        .map(|tile| ((tile.x as isize, tile.y as isize), tile.tile_type))
        .collect::<HashMap<_, _>>();

    for _ in event_reader.iter() {
        let pos = pos_query.single();

        let shadow_casting = ShadowCasting::new(
            (pos.x as isize, pos.y as isize),
            |pos| match tiles_type_map.get(&pos) {
                Some(tile_type) => match tile_type {
                    TileType::Wall => true,
                    TileType::Floor => false,
                }
                None => false
            },
            30
        );


        let visible_points = shadow_casting.calculate_los()
            .into_iter()
            .map(|(x, y)| (x as usize, y as usize))
            .collect();
        *los = LineOfSight(visible_points);
    }
}