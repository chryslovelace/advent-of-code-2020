use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

struct FieldRule {
    name: String,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

struct Ticket(Vec<u32>);

impl Ticket {
    fn error_rate(&self, rules: &[FieldRule]) -> u32 {
        self.0
            .iter()
            .filter(|field| {
                !rules
                    .iter()
                    .any(|rule| rule.range1.contains(&field) || rule.range2.contains(&field))
            })
            .sum()
    }
}

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    static ref INPUT: (Vec<FieldRule>, Ticket, Vec<Ticket>) = {
        let mut paras = include_str!("input.txt").split("\n\n");
        let rules = paras
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let captures = RULE_REGEX.captures(line).unwrap();
                let name = captures[1].to_owned();
                let range1 = captures[2].parse().unwrap()..=captures[3].parse().unwrap();
                let range2 = captures[4].parse().unwrap()..=captures[5].parse().unwrap();
                FieldRule {
                    name,
                    range1,
                    range2,
                }
            })
            .collect();
        let my_ticket = Ticket(
            paras
                .next()
                .unwrap()
                .lines()
                .nth(1)
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
        );
        let nearby_tickets = paras
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| Ticket(line.split(',').map(|n| n.parse().unwrap()).collect()))
            .collect();
        (rules, my_ticket, nearby_tickets)
    };
}

fn part1() {
    let (rules, _, nearby_tickets) = &*INPUT;
    println!(
        "*   {}",
        nearby_tickets
            .iter()
            .map(|ticket| ticket.error_rate(rules))
            .sum::<u32>()
    );
}

fn part2() {
    let (rules, my_ticket, nearby_tickets) = &*INPUT;
    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| ticket.error_rate(rules) == 0)
        .collect();
    let mut field_position_candidates = Vec::new();
    for rule in rules {
        let mut candidates: Vec<_> = (0..my_ticket.0.len()).collect();
        for ticket in &valid_tickets {
            candidates.retain(|&idx| {
                let field = &ticket.0[idx];
                rule.range1.contains(field) || rule.range2.contains(field)
            });
        }
        field_position_candidates.push((rule.name.clone(), candidates));
    }
    field_position_candidates.sort_by_key(|(_, candidates)| candidates.len());
    let mut field_positions = HashMap::new();
    for (field, mut candidates) in field_position_candidates {
        candidates.retain(|idx| field_positions.values().all(|i| i != idx));
        field_positions.insert(field, candidates[0]);
    }
    println!(
        "**  {}",
        field_positions
            .into_iter()
            .filter_map(|(field, idx)| if field.starts_with("departure ") {
                Some(my_ticket.0[idx] as u64)
            } else {
                None
            })
            .product::<u64>()
    );
}

fn main() {
    part1();
    part2();
}
