// alternative implementation found here: https://www.albertford.com/shadowcasting/
// does not work correctly

struct ShadowCasting {
    origin: Position,
    position_state_map: HashMap<Position, TileState>,
}

impl ShadowCasting {
    fn new(
        origin: Position,
        tile_positions: impl IntoIterator<Item=Position>,
        obstacle_positions: impl IntoIterator<Item=Position>,
    ) -> Self {
        let mut tile_state_map = HashMap::new();
        let tiles_in_range = origin.circle_filled(15).into_iter().collect::<HashSet<_>>();

        tile_positions
            .into_iter()
            .filter(|pos| tiles_in_range.contains(pos))
            .for_each(|pos| {
                tile_state_map.insert(pos, TileState { visible: false, blocking: false });
            });

        obstacle_positions
            .into_iter()
            .filter(|pos| tiles_in_range.contains(pos))
            .for_each(|pos| {
                tile_state_map.insert(pos, TileState { visible: false, blocking: true });
            });

        ShadowCasting {
            origin,
            position_state_map: tile_state_map,
        }
    }

    pub fn compute_fov(&mut self) -> HashSet<Position> {
        self.mark_visible(self.origin);

        for sector in [North, East, South, West] {
            let quadrant = Quadrant::new(sector, self.origin);
            let first_row = Row::new(1, -1.0, 1.0);
            self.scan(quadrant, first_row);
        }

        self.position_state_map.iter()
            .filter(|(_, state)| state.visible)
            .map(|(pos, _)| *pos)
            .collect()
    }

    fn scan(&mut self, quadrant: Quadrant, mut row: Row) {
        let mut prev_tile = None;

        for tile in row.positions() {
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

    fn reveal(&mut self, quadrant: Quadrant, pos: Position) {
        self.mark_visible(quadrant.transform(pos))
    }

    fn is_wall(&self, quadrant: Quadrant, pos_opt: Option<Position>) -> bool {
        match pos_opt {
            Some(tile) => self.is_blocking(quadrant.transform(tile)),
            None => false
        }
    }

    fn is_floor(&self, quadrant: Quadrant, pos_opt: Option<Position>) -> bool {
        match pos_opt {
            Some(tile) => self.position_state_map.contains_key(&tile) && !self.is_blocking(quadrant.transform(tile)),
            None => false
        }
    }

    fn slope(pos: Position) -> f32 {
        let (row_depth, col) = (pos.x, pos.y);
        (2 * col - 1) as f32 / (2 * row_depth) as f32
    }

    fn is_symmetric(row: Row, pos: Position) -> bool {
        let col = pos.y;
        col as f32 >= row.depth as f32 * row.start_slope &&
            col as f32 <= row.depth as f32 * row.end_slope
    }

    fn is_blocking(&self, pos: Position) -> bool {
        match self.position_state_map.get(&pos) {
            Some(state) => state.blocking,
            None => false
        }
    }

    fn mark_visible(&mut self, pos: Position) {
        if let Some(state) = self.position_state_map.get_mut(&pos) {
            state.visible = true
        }
    }
}

struct TileState {
    visible: bool,
    blocking: bool,
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
    origin: Position,
}

impl Quadrant {
    fn new(sector: Sector, origin: Position) -> Self {
        Quadrant {
            sector,
            origin,
        }
    }

    fn transform(&self, pos: Position) -> Position {
        match self.sector {
            North => p!(self.origin.x + pos.y, self.origin.y - pos.x),
            South => p!(self.origin.x + pos.y, self.origin.y + pos.x),
            East => p!(self.origin.x + pos.x, self.origin.y + pos.y),
            West => p!(self.origin.x - pos.x, self.origin.y + pos.y),
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

    fn positions(self) -> impl IntoIterator<Item=Position> {
        let min_col = Self::round_ties_up(self.depth as f32 * self.start_slope);
        let max_col = Self::round_ties_down(self.depth as f32 * self.end_slope);

        (min_col..=max_col).map(move |col| p!(self.depth, col))
    }

    fn round_ties_up(value: f32) -> isize {
        (value + 0.5).floor() as isize
    }

    fn round_ties_down(value: f32) -> isize {
        (value - 0.5).ceil() as isize
    }
}
