use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

struct Policy {
    lower: usize,
    upper: usize,
    character: u8,
}

struct Record {
    policy: Policy,
    password: String,
}

impl Record {
    fn is_valid_v1(&self) -> bool {
        let count = self
            .password
            .bytes()
            .filter(|&b| b == self.policy.character)
            .count();

        count >= self.policy.lower && count <= self.policy.upper
    }

    fn is_valid_v2(&self) -> bool {
        let at_lower = self.password.as_bytes()[self.policy.lower - 1] == self.policy.character;
        let at_upper = self.password.as_bytes()[self.policy.upper - 1] == self.policy.character;
        at_lower ^ at_upper
    }
}

#[derive(Debug)]
struct RecordParseError;

impl FromStr for Record {
    type Err = RecordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
        }

        let caps = REGEX.captures(s).ok_or(RecordParseError)?;
        let lower = caps[1].parse().map_err(|_| RecordParseError)?;
        let upper = caps[2].parse().map_err(|_| RecordParseError)?;
        let character = caps[3].as_bytes()[0];
        let password = caps[4].to_owned();

        Ok(Record {
            policy: Policy {
                lower,
                upper,
                character,
            },
            password,
        })
    }
}

lazy_static! {
    static ref INPUT: Vec<Record> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

fn part1() {
    let valid_count = INPUT.iter().filter(|record| record.is_valid_v1()).count();
    println!("*   {}", valid_count);
}

fn part2() {
    let valid_count = INPUT.iter().filter(|record| record.is_valid_v2()).count();
    println!("**  {}", valid_count);
}

fn main() {
    part1();
    part2();
}
