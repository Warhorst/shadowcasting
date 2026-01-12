use Octant::*;
use pad::p;
use pad::position::Position;
use std::collections::HashSet;

// TODO maybe add an optional parameter to limit the area (The shadow cast will scan the whole octant if not limited)

/// Performs recursive shadow casting from the given origin with the specified radius
///
/// * `origin` - The position from where the shadows will be cast
/// * `radius` - The distance from the origin to the edge of the max view range
/// * `position_blocks_view` - Tells if a position blocks the view and acts as a wall
pub fn shadow_cast(
    origin: Position,
    radius: usize,
    position_blocks_view: impl Fn(Position) -> bool,
) -> HashSet<Position> {
    ShadowCasting::new(origin, radius, position_blocks_view).calculate_los()
}

struct ShadowCasting<'a> {
    /// The position from where the line of sight originates
    origin: Position,
    /// Closure which tells if a given position blocks the view
    position_blocks_view: Box<dyn Fn(Position) -> bool + 'a>,
    /// The set which holds all visible positions. Initially empty
    visible_positions: HashSet<Position>,
    /// The radius of the area where the line of sight should be performed.
    radius: usize,
}

impl<'a> ShadowCasting<'a> {
    fn new(
        origin: Position,
        radius: usize,
        position_blocks_view: impl Fn(Position) -> bool + 'a,
    ) -> Self {
        ShadowCasting {
            origin,
            position_blocks_view: Box::new(position_blocks_view),
            visible_positions: HashSet::new(),
            radius,
        }
    }

    /// Calculate the line of sight in all directions and relative to origin with recursive shadow casting.
    /// Returns a set with all visible positions.
    fn calculate_los(mut self) -> HashSet<Position> {
        self.set_pos_visible(self.origin);

        for octant in Octant::octants() {
            self.cast(octant, 1, 1.0, 0.0);
        }

        self.visible_positions
    }

    fn cast(
        &mut self,
        octant: Octant,
        depth: usize,
        mut start_slope: f32,
        end_slope: f32,
    ) {
        let mut prev_pos_blocks_view = false;

        let mut saved_right_slope = 0.0;

        for y in depth..=self.radius {
            for x in (0..=y).rev() {
                let pos = octant.get_world_coordinate(self.origin, x, y);

                let left_slope = (x as f32 + 0.5) / (y as f32 - 0.5);
                let right_slope = (x as f32 - 0.5) / (y as f32 + 0.5);

                // if the rightmost slope of the position is in front of the visible
                // area, move along until it is in it
                if right_slope > start_slope {
                    continue;
                }

                // if the leftmost slope is behind the visible area,
                // nothing can be set visible anymore. break
                if left_slope < end_slope {
                    break;
                }

                self.set_pos_visible(pos);

                if prev_pos_blocks_view {
                    if self.pos_blocks_view(pos) {
                        saved_right_slope = right_slope;
                    } else {
                        prev_pos_blocks_view = false;
                        start_slope = saved_right_slope;
                    }
                } else if self.pos_blocks_view(pos) {
                    // A blocking cell was found and the previous position did not block the view,
                    // so start the recursion
                    if left_slope <= start_slope {
                        self.cast(octant, y + 1, start_slope, left_slope);
                    }
                    prev_pos_blocks_view = true;
                    saved_right_slope = right_slope;
                }
            }

            if prev_pos_blocks_view {
                break;
            }
        }
    }

    fn pos_blocks_view(
        &self,
        pos: Position,
    ) -> bool {
        (self.position_blocks_view)(pos)
    }

    fn set_pos_visible(
        &mut self,
        pos: Position,
    ) {
        self.visible_positions.insert(pos);
    }
}

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
#[derive(Copy, Clone, Debug)]
enum Octant {
    TopLeft,
    TopRight,
    RightTop,
    RightBottom,
    BottomRight,
    BottomLeft,
    LeftBottom,
    LeftTop,
}

impl Octant {
    fn octants() -> [Octant; 8] {
        [
            TopLeft,
            TopRight,
            RightTop,
            RightBottom,
            BottomRight,
            BottomLeft,
            LeftBottom,
            LeftTop,
        ]
    }

    /// Transform the given depth and index to world coordinates relative to origin.
    fn get_world_coordinate(
        &self,
        origin: Position,
        i: usize,
        depth: usize,
    ) -> Position {
        let (xx, xy, yx, yy) = self.get_diffs();

        p!(
            origin.x + i as isize * xx + depth as isize * xy,
            origin.y + i as isize * yx + depth as isize * yy
        )
    }

    fn get_diffs(&self) -> (isize, isize, isize, isize) {
        match self {
            TopLeft => (-1, 0, 0, 1),
            TopRight => (1, 0, 0, 1),
            RightTop => (0, 1, 1, 0),
            RightBottom => (0, 1, -1, 0),
            BottomRight => (1, 0, 0, -1),
            BottomLeft => (-1, 0, 0, -1),
            LeftBottom => (0, -1, -1, 0),
            LeftTop => (0, -1, 1, 0),
        }
    }
}
