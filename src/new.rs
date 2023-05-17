use std::collections::HashMap;
use Octant::*;

pub struct ShadowCasting {
    origin: Tile,
    tile_state_map: HashMap<Tile, TileState>
}

impl ShadowCasting {
    fn new(
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

    fn scan(&mut self) {
        for octant in Octant::VALUES {
            self.scan_octant(octant, 1, 1.0, 0.0);
        }
    }

    fn scan_octant(&mut self, octant: Octant, depth: usize, start_slope: f32, end_slope: f32) {
        // Cases:

        for tile in octant.get_tiles(self.origin, depth) {
            // slope (0, 0) -> (-9, 11)
            // (0 - (-9)) / (0 - (11)) = 9 / -11 = -0.82
        }

        // 1. just floor

        // 2. Block, but previous was floor
        // -> adapt end slope

        // 3. Block, but previous was block
        // -> adapt start slope
    }

    fn tile_blocks_view(&self, tile: Tile) -> bool {
        self.tile_state_map.get(&tile).map_or(false, |state| state.blocking)
    }
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

struct TileState {
    visible: bool,
    blocking: bool,
}

/// The octants the area gets sliced into.
/// Source: https://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting#Recursive_Shadowcasting
///
/// \1111|2222/
/// 8\111|222/3
/// 88\11|22/33
/// 888\1|2/333
/// 8888\|/3333
/// -----@-----
/// 7777/|\4444
/// 777/6|5\444
/// 77/66|55\44
/// 7/666|555\4
/// /6666|5555\
///
/// 1 = TopLeft
/// 2 = TopRight
/// 3 = RightTop
/// 4 = RightBottom
/// 5 = BottomRight
/// 6 = BottomLeft
/// 7 = LeftBottom
/// 8 = LeftTop
#[derive(Copy, Clone)]
enum Octant {
    TopLeft,
    TopRight,
    RightTop,
    RightBottom,
    BottomRight,
    BottomLeft,
    LeftBottom,
    LeftTop
}

impl Octant {
    const VALUES: [Octant; 8] = [TopLeft, TopRight, RightTop, RightBottom, BottomRight, BottomLeft, LeftBottom, LeftTop];

    /// Return the tiles at the given depth relative to the given origin.
    /// The depth is the current row or column.
    fn get_tiles(&self, origin: Tile, depth: usize) -> impl IntoIterator<Item=Tile> + '_ {
        let depth = depth as isize;

        let range = match self {
            TopLeft | RightBottom | BottomLeft | LeftBottom => -depth..=0,
            TopRight | RightTop | BottomRight | LeftTop => depth..=0
        };

        range.map(move |i| match self {
            TopLeft | TopRight => Tile::new(origin.x + i, origin.y + depth),
            RightTop | RightBottom => Tile::new(origin.x + depth, origin.y + i),
            BottomRight | BottomLeft => Tile::new(origin.x + i, origin.y - depth),
            LeftBottom | LeftTop => Tile::new(origin.x - depth, origin.y + i),
        })
    }
}