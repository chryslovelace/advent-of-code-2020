use lazy_static::lazy_static;
use std::collections::BTreeSet;

#[derive(Debug)]
struct TreeMap {
    trees: BTreeSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl TreeMap {
    fn trees_encountered(&self, slope: (usize, usize)) -> usize {
        std::iter::successors(Some((0, 0)), |pos| {
            Some(((pos.0 + slope.0) % self.width, pos.1 + slope.1))
        })
        .take_while(|&(_, y)| y < self.height)
        .filter(|pos| self.trees.contains(pos))
        .count()
    }
}

lazy_static! {
    static ref TREE_MAP: TreeMap = {
        let input = include_str!("input.txt");
        let mut height = 0;
        let mut width = 0;
        let mut trees = BTreeSet::new();
        for (y, line) in input.lines().enumerate() {
            height += 1;
            width = line.len();
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    trees.insert((x, y));
                }
            }
        }
        TreeMap {
            trees,
            height,
            width,
        }
    };
}

fn part1() {
    println!("*   {}", TREE_MAP.trees_encountered((3, 1)));
}

fn part2() {
    let tree_product = TREE_MAP.trees_encountered((1, 1))
        * TREE_MAP.trees_encountered((3, 1))
        * TREE_MAP.trees_encountered((5, 1))
        * TREE_MAP.trees_encountered((7, 1))
        * TREE_MAP.trees_encountered((1, 2));
    println!("**  {}", tree_product);
}

fn main() {
    part1();
    part2();
}
