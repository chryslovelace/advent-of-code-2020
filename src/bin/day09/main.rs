use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref DATA: Vec<usize> = include_str!("input.txt")
        .lines()
        .flat_map(|line| line.parse())
        .collect();
}

const PREAMBLE_SIZE: usize = 25;

fn invalid_value() -> usize {
    *(DATA
        .windows(PREAMBLE_SIZE + 1)
        .find_map(|window| {
            let (value, preamble) = window.split_last().unwrap();
            if preamble
                .iter()
                .tuple_combinations()
                .any(|(a, b)| a + b == *value)
            {
                None
            } else {
                Some(value)
            }
        })
        .unwrap())
}

fn part1() {
    println!("*   {}", invalid_value());
}

fn part2() {
    let invalid_value = invalid_value();
    for range_size in 2.. {
        if let Some(range) = DATA
            .windows(range_size)
            .find(|range| range.iter().sum::<usize>() == invalid_value)
        {
            let (min, max) = range.iter().minmax().into_option().unwrap();
            println!("**  {}", min + max);
            return;
        }
    }
}

fn main() {
    part1();
    part2();
}
