use std::collections::HashMap;

const STARTING_NUMBERS: &[usize] = &[0, 20, 7, 16, 1, 18, 15];

fn play_until(nth: usize) -> usize {
    let mut spoken_numbers: HashMap<_, _> = STARTING_NUMBERS
        .iter()
        .copied()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();
    let mut count = STARTING_NUMBERS.len();
    let mut next = 0;
    while count + 1 < nth {
        let next_next = {
            if let Some(prev) = spoken_numbers.get(&next) {
                count - prev
            } else {
                0
            }
        };
        spoken_numbers.insert(next, count);
        count += 1;
        next = next_next;
    }
    next
}

fn part1() {
    println!("*   {}", play_until(2020));
}

fn part2() {
    println!("**  {}", play_until(30000000));
}

fn main() {
    part1();
    part2();
}
