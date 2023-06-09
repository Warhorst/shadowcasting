use std::collections::HashSet;
use Octant::*;

pub struct ShadowCasting<'a> {
    /// The position from where the line of sight originates
    origin: (isize, isize),
    /// Closure which tells if if a given position blocks the view
    position_blocks_view: Box<dyn Fn((isize, isize)) -> bool + 'a>,
    /// The set which holds all visible positions. Initially empty
    visible_positions: HashSet<(isize, isize)>,
    /// The maximum distance of the line of sight from the origin.
    /// Everything beyond this distance will not be visible.
    max_distance: usize,
}

impl<'a> ShadowCasting<'a> {
    pub fn new(
        origin: (isize, isize),
        position_blocks_view: impl Fn((isize, isize)) -> bool + 'a,
        max_distance: usize,
    ) -> Self {
        ShadowCasting {
            origin,
            position_blocks_view: Box::new(position_blocks_view),
            visible_positions: HashSet::new(),
            max_distance,
        }
    }

    /// Calculate the line of sight in all directions and relative to origin with recursive shadow casting.
    /// Returns a set with all visible positions.
    pub fn calculate_los(mut self) -> HashSet<(isize, isize)> {
        self.set_pos_visible(self.origin);

        for octant in Octant::octants() {
            self.collect_visible_positions_in_octant(octant, 1, 1.0, 0.0)
        }

        self.visible_positions
    }

    fn collect_visible_positions_in_octant(
        &mut self,
        octant: Octant,
        depth: usize,
        mut start_slope: f32,
        end_slope: f32,
    ) {
        if depth == self.max_distance {
            return;
        }

        let mut prev_pos_blocks_view = false;

        for i in (0..=depth).rev() {
            let template_pos = (i as isize, depth as isize);
            let pos = octant.get_world_coordinate(self.origin, i, depth);
            let left_slope = Self::calculate_left_slope(template_pos);
            let right_slope = Self::calculate_right_slope(template_pos);

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

            if !prev_pos_blocks_view && self.pos_blocks_view(pos) {
                self.collect_visible_positions_in_octant(octant, depth + 1, start_slope, left_slope);
            }

            if prev_pos_blocks_view && !self.pos_blocks_view(pos) {
                start_slope = right_slope;
            }

            self.set_pos_visible(pos);

            prev_pos_blocks_view = self.pos_blocks_view(pos)
        }

        // scan the next depth only if the previous position wasn't a blocker
        if !prev_pos_blocks_view {
            self.collect_visible_positions_in_octant(octant, depth + 1, start_slope, end_slope)
        }
    }

    fn calculate_left_slope((x, y): (isize, isize)) -> f32 {
        (x as f32 + 0.5) / (y as f32 - 0.5)
    }

    fn calculate_right_slope((x, y): (isize, isize)) -> f32 {
        (x as f32 - 0.5) / (y as f32 + 0.5)
    }

    fn pos_blocks_view(&self, pos: (isize, isize)) -> bool {
        (self.position_blocks_view)(pos)
    }

    fn set_pos_visible(&mut self, pos: (isize, isize)) {
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
            LeftTop
        ]
    }

    /// Transform the given depth and index to world coordinates relative to origin.
    fn get_world_coordinate(
        &self,
        origin: (isize, isize),
        i: usize,
        depth: usize,
    ) -> (isize, isize) {
        let (xx, xy, yx, yy) = self.get_diffs();

        (
            origin.0 + i as isize * xx + depth as isize * xy,
            origin.1 + i as isize * yx + depth as isize * yy
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