use std::collections::{HashMap, HashSet};

use Sector::*;

mod new;

// see
// http://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting
// https://journal.stuffwithstuff.com/2015/09/07/what-the-hero-sees/
// https://www.roguebasin.com/index.php?title=Field_of_Vision
pub struct ShadowCasting {
    origin: Tile,
    tile_state_map: HashMap<Tile, TileState>,
}

impl ShadowCasting {
    pub fn new(
        start_x: isize,
        start_y: isize,
        tiles: impl IntoIterator<Item=(isize, isize, bool)>,
    ) -> Self {
        let mut tile_state_map = HashMap::new();

        tiles.into_iter().for_each(|(x, y, blocking)| {
            tile_state_map.insert(Tile::new(x, y), TileState { visible: false, blocking });
        });

        ShadowCasting {
            origin: Tile::new(start_x, start_y),
            tile_state_map,
        }
    }

    pub fn compute_los(&mut self) -> HashSet<(isize, isize)> {
        self.mark_visible(self.origin);

        for sector in [North, East, South, West] {
            let quadrant = Quadrant::new(sector, self.origin);
            let first_row = Row::new(1, -1.0, 1.0);
            self.scan(quadrant, first_row);
        }

        self.tile_state_map.iter()
            .filter(|(_, state)| state.visible)
            .map(|(tile, _)| (tile.x, tile.y))
            .collect()
    }

    fn scan(&mut self, quadrant: Quadrant, mut row: Row) {
        let mut prev_tile = None;

        for tile in row.tiles() {
            if self.is_wall(quadrant, Some(tile)) || Self::is_symmetric(row, tile) {
                self.reveal(quadrant, tile)
            }

            if self.is_wall(quadrant, prev_tile) && self.is_floor(quadrant, Some(tile)) {
                row.start_slope = Self::slope(tile);
            }

            if self.is_floor(quadrant, prev_tile) && self.is_wall(quadrant, Some(tile)) {
                let mut next_row = row.next();
                next_row.end_slope = Self::slope(tile);
                self.scan(quadrant, next_row)
            }

            prev_tile = Some(tile)
        }

        if self.is_floor(quadrant, prev_tile) {
            self.scan(quadrant, row.next())
        }
    }

    fn reveal(&mut self, quadrant: Quadrant, tile: Tile) {
        self.mark_visible(quadrant.transform(tile))
    }

    fn is_wall(&self, quadrant: Quadrant, tile_opt: Option<Tile>) -> bool {
        match tile_opt {
            Some(tile) => self.is_blocking(quadrant.transform(tile)),
            None => false
        }
    }

    fn is_floor(&self, quadrant: Quadrant, tile_opt: Option<Tile>) -> bool {
        match tile_opt {
            Some(tile) => self.tile_state_map.contains_key(&tile) && !self.is_blocking(quadrant.transform(tile)),
            None => false
        }
    }

    fn slope(tile: Tile) -> f32 {
        let (row_depth, col) = (tile.x, tile.y);
        (2 * col - 1) as f32 / (2 * row_depth) as f32
    }

    fn is_symmetric(row: Row, tile: Tile) -> bool {
        let col = tile.y;
        col as f32 >= row.depth as f32 * row.start_slope &&
            col as f32 <= row.depth as f32 * row.end_slope
    }

    fn is_blocking(&self, tile: Tile) -> bool {
        match self.tile_state_map.get(&tile) {
            Some(state) => state.blocking,
            None => false
        }
    }

    fn mark_visible(&mut self, tile: Tile) {
        if let Some(mut state) = self.tile_state_map.get_mut(&tile) {
            state.visible = true
        }
    }
}

struct TileState {
    visible: bool,
    blocking: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Tile {
    x: isize,
    y: isize,
}

impl Tile {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
enum Sector {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
struct Quadrant {
    sector: Sector,
    origin: Tile,
}

impl Quadrant {
    fn new(sector: Sector, origin: Tile) -> Self {
        Quadrant {
            sector,
            origin,
        }
    }

    fn transform(&self, tile: Tile) -> Tile {
        let (row, col) = (tile.x, tile.y);

        match self.sector {
            North => Tile::new(self.origin.x + col, self.origin.y - row),
            South => Tile::new(self.origin.x + col, self.origin.y + row),
            East => Tile::new(self.origin.x + row, self.origin.y + col),
            West => Tile::new(self.origin.x - row, self.origin.y + col),
        }
    }
}

#[derive(Copy, Clone)]
struct Row {
    depth: usize,
    start_slope: f32,
    end_slope: f32,
}

impl Row {
    pub fn new(depth: usize, start_slope: f32, end_slope: f32) -> Self {
        Self { depth, start_slope, end_slope }
    }

    fn next(&self) -> Row {
        Row::new(
            self.depth + 1,
            self.start_slope,
            self.end_slope,
        )
    }

    fn tiles(self) -> impl IntoIterator<Item=Tile> {
        let min_col = Self::round_ties_up(self.depth as f32 * self.start_slope);
        let max_col = Self::round_ties_down(self.depth as f32 * self.end_slope);

        (min_col..=max_col).map(move |col| Tile::new(self.depth as isize, col))
    }

    fn round_ties_up(value: f32) -> isize {
        (value + 0.5).floor() as isize
    }

    fn round_ties_down(value: f32) -> isize {
        (value - 0.5).ceil() as isize
    }
}
