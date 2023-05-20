use std::collections::{HashMap, HashSet};
use Octant::*;

pub struct ShadowCasting {
    origin: (isize, isize),
    position_block_map: HashMap<(isize, isize), bool>,
    visible_positions: HashSet<(isize, isize)>,
    max_distance: usize,
}

impl ShadowCasting {
    pub fn new(
        origin: (isize, isize),
        position_block_iter: impl IntoIterator<Item=((isize, isize), bool)>,
        max_distance: usize,
    ) -> Self {
        ShadowCasting {
            origin,
            position_block_map: position_block_iter.into_iter().collect(),
            visible_positions: HashSet::new(),
            max_distance,
        }
    }

    pub fn calculate_los(mut self) -> HashSet<(isize, isize)> {
        println!("origin: {:?}", self.origin);

        for octant in Octant::octants() {
            println!("{:?}", octant);
            self.scan_octant(octant, 1, 1.0, 0.0)
        }

        self.visible_positions
    }

    fn scan_octant(
        &mut self,
        octant: Octant,
        depth: usize,
        start_slope: f32,
        end_slope: f32,
    ) {
        if depth == self.max_distance {
            return;
        }

        let mut prev_pos = None;

        for i in (0..=depth).rev() {
            let temp_pos = (i as isize, depth as isize);
            let pos = octant.get_world_coordinate(self.origin, i, depth);
            let left_slope = Self::calculate_left_slope(temp_pos);
            let right_slope = Self::calculate_right_slope(temp_pos);

            println!("template pos: {:?}", (i, depth));
            println!("world pos: {:?}", pos);
            println!("left: {}", left_slope);
            println!("right: {}", right_slope);

            if left_slope < end_slope {
                return;
            }

            if right_slope > start_slope {
                continue;
            }

            // TODO prev pos?
            if self.pos_blocks_view(pos) {
                self.set_pos_visible(pos);
                self.scan_octant(octant, depth + 1, start_slope, left_slope);
            } else {
                self.set_pos_visible(pos);
            }


            // if end_slope < left_slope {
            //     return;
            // }
            //
            // if right_slope < start_slope {
            //     return;
            // }

            prev_pos = Some(pos)
        }

        self.scan_octant(octant, depth + 1, start_slope, end_slope)
    }

    fn calculate_left_slope((x, y): (isize, isize)) -> f32 {
        (x as f32 + 0.5) / (y as f32 - 0.5)
    }

    fn calculate_right_slope((x, y): (isize, isize)) -> f32 {
        (x as f32 - 0.5) / (y as f32 + 0.5)
    }

    fn pos_in_visible_area(pos: (isize, isize), start_slope: f32, end_slope: f32) -> bool {
        let left_slope = Self::calculate_left_slope(pos);
        let right_slope = Self::calculate_right_slope(pos);

        left_slope < start_slope && right_slope > end_slope
    }

    fn pos_blocks_view(&self, pos: (isize, isize)) -> bool {
        match self.position_block_map.get(&pos) {
            Some(blocking) => *blocking,
            None => false,
        }
    }

    fn set_pos_visible(&mut self, pos: (isize, isize)) {
        if self.pos_on_board(pos) {
            self.visible_positions.insert(pos);
        }
    }

    fn pos_on_board(&self, pos: (isize, isize)) -> bool {
        self.position_block_map.contains_key(&pos)
    }
}

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

#[cfg(test)]
mod tests {
    use crate::my::ShadowCasting;

    #[test]
    fn test_slopes() {
        for x in 1..=12 {
            for y in 1..=12 {
                if x > y {
                    continue
                }

                let pos = (x, y);
                let right = ShadowCasting::calculate_right_slope(pos);
                let left = ShadowCasting::calculate_left_slope(pos);
                println!("{:?} -> {:?}", pos, (left, right));
            }
        }
    }
}