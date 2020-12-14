use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

enum Inst {
    Mask(String),
    Mem { address: u64, value: u64 },
}

impl FromStr for Inst {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"mask = ([X01]{36})|mem\[(\d+)\] = (\d+)").unwrap();
        }

        if let Some(captures) = REGEX.captures(s) {
            if let Some(mask) = captures.get(1) {
                Ok(Inst::Mask(mask.as_str().to_owned()))
            } else {
                let address = captures[2].parse().unwrap();
                let value = captures[3].parse().unwrap();
                Ok(Inst::Mem { address, value })
            }
        } else {
            Err(())
        }
    }
}

lazy_static! {
    static ref PROGRAM: Vec<Inst> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

fn part1() {
    let mut and_mask: u64 = !0;
    let mut or_mask: u64 = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for inst in &*PROGRAM {
        match inst {
            Inst::Mask(mask) => {
                and_mask = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|&(_, c)| c == '0')
                    .fold(!0, |mask, (i, _)| mask & !(1 << i));
                or_mask = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|&(_, c)| c == '1')
                    .fold(0, |mask, (i, _)| mask | (1 << i));
            }
            Inst::Mem { address, value } => {
                mem.insert(*address, (value & and_mask) | or_mask);
            }
        }
    }
    println!("*   {}", mem.values().sum::<u64>());
}

fn decode_address(address: u64, mask: &str) -> impl Iterator<Item = u64> {
    let mut addresses = vec![0];
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '0' => addresses.iter_mut().for_each(|a| *a += address & (1 << i)),
            '1' => addresses.iter_mut().for_each(|a| *a += 1 << i),
            'X' => {
                let mut new_addresses = addresses.clone();
                new_addresses.iter_mut().for_each(|a| *a += 1 << i);
                addresses.append(&mut new_addresses);
            }
            _ => unreachable!(),
        }
    }
    addresses.into_iter()
}

fn part2() {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    for inst in &*PROGRAM {
        match inst {
            Inst::Mask(m) => mask = m.as_str(),
            Inst::Mem { address, value } => {
                for address in decode_address(*address, mask) {
                    mem.insert(address, *value);
                }
            }
        }
    }
    println!("**  {}", mem.values().sum::<u64>());
}

fn main() {
    part1();
    part2();
}
