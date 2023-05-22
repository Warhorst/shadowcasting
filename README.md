# shadow casting
Implementation of the recursive shadow casting algorithm, defined [here](http://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting).

## How it works
### Introduction
Despite having an explanation of the algorithm with examples, figures and multiple implementations in different languages, it took me quite some time to get this implementation right. Also, I didn't find any other, deeper explanation of this algorithm which would help me understand it (only some derivations like [this](https://www.albertford.com/shadowcasting/) or [this](https://journal.stuffwithstuff.com/2015/09/07/what-the-hero-sees/)).

Therefore, I want to write down my understanding of the algorithm to maybe add something to the original article.

### The basics: octants
As described in the article, we split our area in equally sized octants:

```
\1111|2222/
8\111|222/3
88\11|22/33
888\1|2/333
8888\|/3333
-----@-----
7777/|\4444
777/6|5\444
77/66|55\44
7/666|555\4
/6666|5555\
```

The origin (x, y) is at the center of the octants, and every octant is scanned from in to out, getting wider in every step.

This seems counterintuitive, but the algorithm uses a trick to simplify the operation. Imagine we have a theoretical octant which starts at (0,0) which, following the y-axis, gets wider and wider:

```
......5
 .....4
  ....3
   ...2
    ..1
     .0
     0
```

In my implementation, I call this octant the **template**. The template lies on a coordinate system with mirrored y-axis and origin in (0,0). For every octant, the scan for visible positions starts at the origin and follows along the y-axis. Other implementations call the current y-level the row, but I found this confusing, as for some octants this is the column. Therefore, I called it the **depth** and the index when going from left ro right in a y level just **i**.

Given (i, depth), which I call the **template_pos**, we can calculate the world position relative to the origin and current octant with the formulas

``
x + i * xx + depth * xy
``

and 

``
y + i * yx + depth * yy
``

(x, y) are just the origin. xx, xy, yx and yy modifiers which are fixed for every octant. For example, the top left octant (number one in the figure above) has (xx, xy, yx, yy) = (-1, 0, 0, 1).

## The basics: slopes
Slopes are used to determine if a position can be seen, given the currently cast shadow. There are two important pairs of slopes which are used in the algorithm: start_slope/end_slope and left_slope/right_slope.

The **start_slope** and **end_slope** define the currently visible area in the template and are not absolute, but relative. When starting the scan for an octant, start_slope is set to 1.0 and end_slope is set to 0.0, which are the leftmost and rightmost points at the current depth. (left_slope, right_slope) = (1.0, 0.0) means the current depth is fully visible.

The **left_slope** and **right_slope** are calculated for the current template_pos. As described in the article, they are calculated using a line originating from origin that shoots through the bottom left and top right corner of a given position, which uses the formula

``
slope = (x1 - x2) / (y1 - y2)
``

As origin is always (0, 0), this can be simplified to 

``
slope = x / y
``

The bottom left corner of a position is "half a block" to the left and to the bottom. This means the formula to calculate the left slope is

``
left_slope = x + 0.5 / y - 0.5
``

(As the coordinate system is mirrored at the y-axis, x is increased when going left, not decreased). The right_slope is equivalent, but with the top right corner:

``
right_slope = x - 0.5 / y + 0.5
``

Using these slopes, some can tell if a position is in the visible area. If the left_slope is smaller than the end_slope, the position is right of the visible area. If the right_slope is larger than the start_slope, than the position is left of the visible area.

## The algorithm
The actual algorithm now starts with adding the origin to the visible positions, as you can see your own position at all times.

```
self.set_pos_visible(self.origin);
```

An iteration through all eight octants starts next, where the initial depth is 1.0, start_slope is 1.0 and end_slope is 0.0.

```
for octant in Octant::octants() {
    self.collect_visible_positions_in_octant(octant, 1, 1.0, 0.0)
}
```

The scan stops if the current depth reached the max_distance.

```
if depth == self.max_distance {
    return;
}
```

A flag gets initialized which tells if the previous position was blocking or not. This information is important for multiple checks and gets updated during the scan.

```
let mut prev_pos_blocks_view = false;
```

Next, an iteration through every index at the current depth starts to get all template positions. The iteration goes from the highest value to the lowest (or left to right in the template), so the range gets reversed.

```
for i in (0..=depth).rev() {
...
}
```

In this loop, the first created values are the template_pos (i, depth), the world position **pos** and the left_slope/right_slope.

```
let template_pos = (i as isize, depth as isize);
let pos = octant.get_world_coordinate(self.origin, i, depth);
let left_slope = Self::calculate_left_slope(template_pos);
let right_slope = Self::calculate_right_slope(template_pos);
```

If the right_slope is larger than the start_slope, we are left of the visible area and skip the current position.

```
if right_slope > start_slope {
    continue;
}
```

If the left_slope is smaller than the end_slope, we are right of the visible area. As none of the following positions will be in the visible area, we break the loop to skip these.

```
if left_slope < end_slope {
    break;
}
```

If the previous position wasn't a blocker but the current is, we start a scan one level deeper with the left_slope as new end_slope. The new visible area will be smaller this way and positions in the shadow of the blocker won't be scanned.

```
if !prev_pos_blocks_view && self.pos_blocks_view(pos) {
    self.collect_visible_positions_in_octant(octant, depth + 1, start_slope, left_slope);
}
```

If the previous position was a blocker but the current isn't, we set the start_slope to the current right_slope. When the scan at the next depth starts, everything behind and left of the last blocker won't be scanned, as it is not in the visible area.

```
if prev_pos_blocks_view && !self.pos_blocks_view(pos) {
    start_slope = right_slope;
}
```

As the position was not skipped (it is in the visible area), it gets set visible. This also happens with walls, so they aren't black. The call just adds the position to the set of visible positions.

```
self.set_pos_visible(pos);
```

Finally, in this loop, the "last positions blocks view" flag gets updated.

```
prev_pos_blocks_view = self.pos_blocks_view(pos)
```

The last step in the scan is to check if the last visited position was a blocker. If not, the scan gets continued at the next depth.

```
if !prev_pos_blocks_view {
    self.collect_visible_positions_in_octant(octant, depth + 1, start_slope, end_slope)
}
```

When every octant was scanned, the set of visible positions gets returned.