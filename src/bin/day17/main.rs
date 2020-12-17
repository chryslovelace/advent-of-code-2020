use itertools::iproduct;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Clone)]
struct Space {
    active: HashSet<(i32, i32, i32, i32)>,
    xrange: RangeInclusive<i32>,
    yrange: RangeInclusive<i32>,
    zrange: RangeInclusive<i32>,
    wrange: RangeInclusive<i32>,
}

impl Space {
    fn new() -> Self {
        Self {
            active: HashSet::new(),
            xrange: 0..=0,
            yrange: 0..=0,
            zrange: 0..=0,
            wrange: 0..=0,
        }
    }

    fn activate(&mut self, pos: (i32, i32, i32, i32)) {
        self.active.insert(pos);
        if &pos.0 <= self.xrange.start() {
            self.xrange = RangeInclusive::new(pos.0 - 1, *self.xrange.end());
        } else if &pos.0 >= self.xrange.end() {
            self.xrange = RangeInclusive::new(*self.xrange.start(), pos.0 + 1);
        }
        if &pos.1 <= self.yrange.start() {
            self.yrange = RangeInclusive::new(pos.1 - 1, *self.yrange.end());
        } else if &pos.1 >= self.yrange.end() {
            self.yrange = RangeInclusive::new(*self.yrange.start(), pos.1 + 1);
        }
        if &pos.2 <= self.zrange.start() {
            self.zrange = RangeInclusive::new(pos.2 - 1, *self.zrange.end());
        } else if &pos.2 >= self.zrange.end() {
            self.zrange = RangeInclusive::new(*self.zrange.start(), pos.2 + 1);
        }
        if &pos.3 <= self.wrange.start() {
            self.wrange = RangeInclusive::new(pos.3 - 1, *self.wrange.end());
        } else if &pos.3 >= self.wrange.end() {
            self.wrange = RangeInclusive::new(*self.wrange.start(), pos.3 + 1);
        }
    }

    fn active_neighbors_3d(&self, pos: (i32, i32, i32, i32)) -> usize {
        iproduct!(
            pos.0 - 1..=pos.0 + 1,
            pos.1 - 1..=pos.1 + 1,
            pos.2 - 1..=pos.2 + 1,
            0..=0
        )
        .filter(|p| p != &pos && self.active.contains(p))
        .count()
    }

    fn cycle_3d(&mut self) {
        let mut next = Self::new();
        for pos in iproduct!(
            self.xrange.clone(),
            self.yrange.clone(),
            self.zrange.clone(),
            0..=0
        ) {
            if self.active.contains(&pos) {
                if (2..=3).contains(&self.active_neighbors_3d(pos)) {
                    next.activate(pos);
                }
            } else if self.active_neighbors_3d(pos) == 3 {
                next.activate(pos);
            }
        }
        *self = next
    }

    fn active_neighbors_4d(&self, pos: (i32, i32, i32, i32)) -> usize {
        iproduct!(
            pos.0 - 1..=pos.0 + 1,
            pos.1 - 1..=pos.1 + 1,
            pos.2 - 1..=pos.2 + 1,
            pos.3 - 1..=pos.3 + 1
        )
        .filter(|p| p != &pos && self.active.contains(p))
        .count()
    }

    fn cycle_4d(&mut self) {
        let mut next = Self::new();
        for pos in iproduct!(
            self.xrange.clone(),
            self.yrange.clone(),
            self.zrange.clone(),
            self.wrange.clone()
        ) {
            if self.active.contains(&pos) {
                if (2..=3).contains(&self.active_neighbors_4d(pos)) {
                    next.activate(pos);
                }
            } else if self.active_neighbors_4d(pos) == 3 {
                next.activate(pos);
            }
        }
        *self = next
    }
}

lazy_static! {
    static ref INITIAL_STATE: Space = {
        let mut space = Space::new();
        for (y, line) in include_str!("input.txt").lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    space.activate((x as i32, y as i32, 0, 0));
                }
            }
        }
        space
    };
}

fn part1() {
    let mut space = INITIAL_STATE.clone();
    for _ in 0..6 {
        space.cycle_3d();
    }
    println!("*   {}", space.active.len());
}

fn part2() {
    let mut space = INITIAL_STATE.clone();
    for _ in 0..6 {
        space.cycle_4d();
    }
    println!("**  {}", space.active.len());
}

fn main() {
    part1();
    part2();
}
