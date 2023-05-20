use std::collections::HashSet;
use Octant::*;

pub fn calculate_los(
    origin: (isize, isize),
    blocks_view: impl Fn((isize, isize)) -> bool,
    max_distance: usize,
) -> HashSet<(isize, isize)> {
    let mut visible_points = HashSet::new();

    for octant in Octant::octants() {
        scan_octant(
            origin,
            &blocks_view,
            &mut visible_points,
            max_distance as isize,
            octant,
            1,
            1.0,
            0.0
        )
    }

    visible_points
}

fn scan_octant(
    origin: (isize, isize),
    blocks_view: &impl Fn((isize, isize)) -> bool,
    visible_points: &mut HashSet<(isize, isize)>,
    max_distance: isize,
    octant: Octant,
    depth: isize,
    // 1.0
    start_slope: f32,
    // 0.0
    end_slope: f32,
) {
    for i in (0..=depth).rev() {
        let point = octant.get_world_coordinate(origin, i, depth);
        let left_slope = get_left_slope(point);
        let right_slope = get_right_slope(point);

        if blocks_view(point) {
            scan_octant(
                origin,
                blocks_view,
                visible_points,
                max_distance,
                octant,
                depth + 1,
                start_slope,
                end_slope
            )
        } else {
            visible_points.insert(point);
        }
    }

    if depth < max_distance {
        scan_octant(
            origin,
            blocks_view,
            visible_points,
            max_distance,
            octant,
            depth + 1,
            start_slope,
            end_slope
        )
    }
}

fn get_left_slope((x, y): (isize, isize)) -> f32 {
    (x as f32 - 0.5) / (y as f32 - 0.5)
}

fn get_right_slope((x, y): (isize, isize)) -> f32 {
    (x as f32 + 0.5) / (y as f32 + 0.5)
}

#[derive(Copy, Clone)]
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
        i: isize,
        depth: isize,
    ) -> (isize, isize) {
        let (xx, xy, yx, yy) = self.get_diffs();

        (
            origin.0 + i * xx + depth * xy,
            origin.1 + i * yx + depth * yy
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