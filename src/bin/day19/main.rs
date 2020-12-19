use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub enum Rule {
    Literal(String),
    SubRules(Vec<Vec<usize>>),
}

fn is_match<'s>(rules: &[Rule], message: &'s str, rule: usize) -> Vec<&'s str> {
    match &rules[rule] {
        Rule::Literal(literal) => {
            if message.starts_with(literal) {
                vec![&message[literal.len()..]]
            } else {
                vec![]
            }
        }
        Rule::SubRules(subrules) => subrules
            .iter()
            .flat_map(|subrule| {
                subrule.iter().fold(vec![message], |messages, &rule| {
                    messages
                        .into_iter()
                        .flat_map(|message| is_match(rules, message, rule))
                        .collect()
                })
            })
            .collect(),
    }
}

fn is_complete_match(rules: &[Rule], message: &str, rule: usize) -> bool {
    is_match(rules, message, rule).contains(&"")
}

lazy_static! {
    static ref INPUT: (Vec<Rule>, Vec<String>) = {
        peg::parser! {
            grammar message_rules() for str {
                rule number() -> usize
                    = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule literal() -> String
                    = "\"" s:$(['a'..='z']+) "\"" { s.to_owned() }

                rule rule_() -> Rule
                    = l:literal() { Rule::Literal(l) }
                    / s:subrule() ** " | " { Rule::SubRules(s) }

                rule subrule() -> Vec<usize>
                    = n:number() ** " " { n }

                pub rule rule_listing() -> (usize, Rule)
                    = n:number() ": " r:rule_() { (n, r) }
            }
        }

        let mut paras = include_str!("input.txt").split("\n\n");
        let rules = paras
            .next()
            .unwrap()
            .lines()
            .map(|line| message_rules::rule_listing(line).unwrap())
            .sorted_by_key(|&(i, _)| i)
            .map(|(_, rule)| rule)
            .collect();
        let messages = paras
            .next()
            .unwrap()
            .lines()
            .map(|line| line.to_owned())
            .collect();

        (rules, messages)
    };
}

fn part1() {
    let (rules, messages) = &*INPUT;
    println!(
        "*   {}",
        messages
            .iter()
            .filter(|message| is_complete_match(rules, message, 0))
            .count()
    );
}

fn part2() {
    let (rules, messages) = &*INPUT;
    let mut rules = rules.clone();
    rules[8] = Rule::SubRules(vec![vec![42], vec![42, 8]]);
    rules[11] = Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]);
    println!(
        "**  {}",
        messages
            .iter()
            .filter(|message| is_complete_match(&rules, message, 0))
            .count()
    );
}

fn main() {
    part1();
    part2();
}
