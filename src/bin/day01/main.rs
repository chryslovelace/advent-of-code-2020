use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: Vec<u32> = include_str!("input.txt")
        .lines()
        .flat_map(|line| line.parse())
        .collect();
}

fn part1() {
    let (a, b) = INPUT
        .iter()
        .tuple_combinations()
        .find(|(&a, &b)| a + b == 2020)
        .unwrap();
    println!("*   {}", a * b);
}

fn part2() {
    let (a, b, c) = INPUT
        .iter()
        .tuple_combinations()
        .find(|(&a, &b, &c)| a + b + c == 2020)
        .unwrap();
    println!("**  {}", a * b * c);
}

fn main() {
    part1();
    part2();
}
