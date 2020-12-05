use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

struct Passport {
    fields: HashMap<String, String>,
}

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
    fn has_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|&field| self.fields.contains_key(field))
    }

    fn is_valid(&self) -> bool {
        self.has_required_fields()
            && REQUIRED_FIELDS
                .iter()
                .all(|&field| field_is_valid(field, &self.fields[field]))
    }
}

fn field_is_valid(field_name: &str, value: &str) -> bool {
    match field_name {
        "byr" => {
            if let Ok(year) = value.parse() {
                (1920..=2002).contains(&year)
            } else {
                false
            }
        }
        "iyr" => {
            if let Ok(year) = value.parse() {
                (2010..=2020).contains(&year)
            } else {
                false
            }
        }
        "eyr" => {
            if let Ok(year) = value.parse() {
                (2020..=2030).contains(&year)
            } else {
                false
            }
        }
        "hgt" => {
            let range = if value.ends_with("cm") {
                150..=193
            } else if value.ends_with("in") {
                59..=76
            } else {
                return false;
            };
            if let Ok(height) = value[..value.len() - 2].parse() {
                range.contains(&height)
            } else {
                false
            }
        }
        "hcl" => {
            value.len() == 7
                && value.starts_with('#')
                && value[1..]
                    .chars()
                    .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
        }
        "ecl" => EYE_COLORS.contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| ('0'..='9').contains(&c)),
        _ => true,
    }
}

lazy_static! {
    static ref PASSPORTS: Vec<Passport> = include_str!("input.txt")
        .split("\n\n")
        .map(|entry| {
            Passport {
                fields: entry
                    .split_whitespace()
                    .flat_map(|field| field.split(':').map(|s| s.to_owned()).collect_tuple())
                    .collect(),
            }
        })
        .collect();
}

fn part1() {
    let count = PASSPORTS.iter().filter(|p| p.has_required_fields()).count();
    println!("*   {}", count);
}

fn part2() {
    let count = PASSPORTS.iter().filter(|p| p.is_valid()).count();
    println!("**  {}", count);
}

fn main() {
    part1();
    part2();
}
