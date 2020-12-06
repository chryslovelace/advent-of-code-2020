use lazy_static::lazy_static;

lazy_static! {
    static ref ANSWERS: Vec<Vec<String>> = include_str!("input.txt")
        .split("\n\n")
        .map(|grp| grp.lines().map(|line| line.to_owned()).collect())
        .collect();
}

fn part1() {
    let sum: usize = ANSWERS
        .iter()
        .map(|grp| {
            ('a'..='z')
                .filter(|&q| grp.iter().any(|answer| answer.contains(q)))
                .count()
        })
        .sum();
    println!("*   {}", sum);
}

fn part2() {
    let sum: usize = ANSWERS
        .iter()
        .map(|grp| {
            ('a'..='z')
                .filter(|&q| grp.iter().all(|answer| answer.contains(q)))
                .count()
        })
        .sum();
    println!("**  {}", sum);
}

fn main() {
    part1();
    part2();
}
