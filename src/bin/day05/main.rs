use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref SEAT_IDS: Vec<u16> = include_str!("input.txt")
        .lines()
        .map(|line| u16::from_str_radix(
            &line
                .replace(&['B', 'R'][..], "1")
                .replace(&['F', 'L'][..], "0"),
            2,
        )
        .unwrap())
        .collect();
}

fn part1() {
    let max_seat_id = SEAT_IDS.iter().max().unwrap();
    println!("*   {}", max_seat_id);
}

fn part2() {
    let (seat_before, _) = SEAT_IDS
        .iter()
        .sorted()
        .tuple_windows()
        .find(|(&a, &b)| a + 2 == b)
        .unwrap();
    println!("**  {}", seat_before + 1);
}

fn main() {
    part1();
    part2();
}
