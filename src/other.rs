use std::collections::HashSet;

// source: https://raw.githubusercontent.com/irskep/clubsandwich/afc79ed/clubsandwich/line_of_sight.py

const MULT: [[isize; 8]; 4] = [
    [1, 0, 0, -1, -1, 0, 0, 1],
    [0, 1, -1, 0, 0, -1, 1, 0],
    [0, 1, 1, 0, 0, -1, -1, 0],
    [1, 0, 0, 1, -1, 0, 0, -1],
];

pub fn get_visible_points(
    origin: (isize, isize),
    allows_light: impl Fn((isize, isize)) -> bool,
    max_distance: usize,
) -> HashSet<(isize, isize)> {
    let mut los_cache = HashSet::new();
    los_cache.insert(origin);

    for region in 0..8 {
        cast_light(
            &mut los_cache,
            &allows_light,
            origin,
            1,
            1.0,
            0.0,
            max_distance as isize,
            MULT[0][region],
            MULT[1][region],
            MULT[2][region],
            MULT[3][region],
        );
    }

    los_cache
}

fn cast_light(
    los_cache: &mut HashSet<(isize, isize)>,
    allows_light: &impl Fn((isize, isize)) -> bool,
    origin: (isize, isize),
    row: isize,
    mut start: f32,
    end: f32,
    radius: isize,
    xx: isize,
    xy: isize,
    yx: isize,
    yy: isize,
) {
    if start < end {
        return;
    }

    let radius_squared = radius ^ 2;

    for r in row..=radius {
        let mut new_start = 0.0;
        // initial: (-2, -1)
        let (mut dx, dy) = (-r - 1, -r);
        let mut blocked = false;

        while dx <= 0 {
            dx += 1;

            // xx 1, xy 0, yx 0, yy 1
            let point = (origin.0 + dx * xx + dy * xy, origin.1 + dx * yx + dy * yy);

            let (l_slope, r_slope) = ((dx as f32 - 0.5) / (dy as f32 + 0.5), (dx as f32 + 0.5) / (dy as f32 - 0.5));

            if start < r_slope {
                continue;
            } else if end > l_slope {
                break;
            } else {
                if dx * dx + dy * dy < radius_squared {
                    los_cache.insert(point);
                }

                if blocked {
                    if !allows_light(point) {
                        new_start = r_slope;
                        continue;
                    } else {
                        blocked = false;
                        start = new_start;
                    }
                } else {
                    if !allows_light(point) && r < radius {
                        blocked = true;
                        cast_light(
                            los_cache,
                            allows_light,
                            origin,
                            r + 1,
                            start,
                            l_slope,
                            radius,
                            xx,
                            xy,
                            yx,
                            yy,
                        );
                        new_start = r_slope;
                    }
                }
            }
        }

        if blocked {
            break;
        }
    }
}