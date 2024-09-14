use pad::{p, Position};
use std::collections::HashSet;
use Octant::*;

/// Performs recursive shadow casting from the given origin with the specified radius
///
/// position_in_visible_area specifies if a position is relevant for shadow casting
/// position_blocks_view tells if a position blocks the view and acts as a wall
pub fn shadow_cast(
    origin: Position,
    radius: usize,
    position_in_visible_area: impl Fn(Position) -> bool,
    position_blocks_view: impl Fn(Position) -> bool + 'static
) -> HashSet<Position> {
    ShadowCasting::new(
        origin,
        radius,
        position_in_visible_area,
        position_blocks_view,
    ).calculate_los()
}

// implements https://www.roguebasin.com/index.php/FOV_using_recursive_shadowcasting
struct ShadowCasting {
    /// The position from where the line of sight originates
    origin: Position,
    /// Closure which tells if a given position blocks the view
    position_blocks_view: Box<dyn Fn(Position) -> bool + 'static>,
    /// The set which holds all visible positions. Initially empty
    visible_positions: HashSet<Position>,
    /// The radius of the area where the line of sight should be performed.
    radius: usize,
    /// The area in where the shadowcast should be performed
    area: HashSet<Position>
}

impl ShadowCasting {
    pub fn new(
        origin: Position,
        radius: usize,
        position_in_visible_area: impl Fn(Position) -> bool,
        position_blocks_view: impl Fn(Position) -> bool + 'static,
    ) -> Self {
        ShadowCasting {
            origin,
            position_blocks_view: Box::new(position_blocks_view),
            visible_positions: HashSet::new(),
            radius,
            area: origin
                .circle_filled(radius)
                .into_iter()
                .filter(|p| position_in_visible_area(*p))
                .collect()
        }
    }

    /// Calculate the line of sight in all directions and relative to origin with recursive shadow casting.
    /// Returns a set with all visible positions.
    pub fn calculate_los(mut self) -> HashSet<Position> {
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
        if depth == self.radius {
            return;
        }

        let mut prev_pos_blocks_view = false;

        for i in (0..=depth).rev() {
            let template_pos = p!(i, depth);
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

    fn calculate_left_slope(pos: Position) -> f32 {
        (pos.x as f32 + 0.5) / (pos.y as f32 - 0.5)
    }

    fn calculate_right_slope(pos: Position) -> f32 {
        (pos.x as f32 - 0.5) / (pos.y as f32 + 0.5)
    }

    fn pos_blocks_view(&self, pos: Position) -> bool {
        (self.position_blocks_view)(pos)
    }

    fn set_pos_visible(&mut self, pos: Position) {
        if self.area.contains(&pos) {
            self.visible_positions.insert(pos);
        }
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

#[cfg(test)]
mod tests {
    use crate::shadow_cast;
    use pad::{p, Position};

    #[test]
    fn works() {
        let origin = p!(0, 0);
        let radius = 8;

        let position_in_visible_area = |pos: Position| pos.x >= -2 && pos.y >= -2;
        let position_blocks_view = |pos: Position| pos == p!(3, 3) || pos == p!(4, 3);

        let positions = shadow_cast(
            origin,
            radius,
            position_in_visible_area,
            position_blocks_view
        );

        Position::print_positions(positions);
    }
}