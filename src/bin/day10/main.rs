#![feature(map_first_last)]
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};

lazy_static! {
    static ref ADAPTERS: BTreeSet<usize> = include_str!("input.txt")
        .lines()
        .flat_map(|line| line.parse())
        .collect();
    static ref LOWEST_JOLTAGE: usize = *ADAPTERS.first().unwrap();
    static ref BUILT_IN_JOLTAGE: usize = ADAPTERS.last().unwrap() + 3;
}

fn part1() {
    let mut differences = BTreeMap::new();
    differences.insert(*LOWEST_JOLTAGE, 1);
    // built in joltage is always 3 more than the highest adapter
    differences.insert(3, 1);
    for (a, b) in ADAPTERS.iter().tuple_windows() {
        *differences.entry(b - a).or_default() += 1;
    }
    println!("*   {}", differences[&1] * differences[&3]);
}

fn part2() {
    let mut paths = BTreeMap::new();
    paths.insert(*BUILT_IN_JOLTAGE, 1usize);
    for adapter in ADAPTERS.iter().copied().rev().chain(Some(0)) {
        paths.insert(
            adapter,
            (1..=3).filter_map(|i| paths.get(&(adapter + i))).sum(),
        );
    }
    println!("**  {}", paths[&0]);
}

fn main() {
    part1();
    part2();
}
