#![feature(str_split_once)]
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref RULES: HashMap<String, Vec<(usize, String)>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(" bags contain ").unwrap();
            if values == "no other bags." {
                return (key.to_owned(), Vec::new());
            }
            let values = values
                .split(", ")
                .map(|s| {
                    let (s, _) = s.rsplit_once(' ').unwrap();
                    let (amount, name) = s.split_once(' ').unwrap();
                    (amount.parse().unwrap(), name.to_owned())
                })
                .collect();
            (key.to_owned(), values)
        })
        .collect();
}

fn can_contain_eventually(outer: &str, inner: &str) -> bool {
    RULES[outer]
        .iter()
        .any(|(_, bag)| bag == inner || can_contain_eventually(bag, inner))
}

fn part1() {
    let count = RULES
        .keys()
        .filter(|bag| can_contain_eventually(bag, "shiny gold"))
        .count();
    println!("*   {}", count);
}

fn count_inner_bags(outer: &str) -> usize {
    RULES[outer]
        .iter()
        .map(|(count, bag)| count * (1 + count_inner_bags(bag)))
        .sum()
}

fn part2() {
    let count = count_inner_bags("shiny gold");
    println!("**  {}", count);
}

fn main() {
    part1();
    part2();
}
