#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Rule {
    colour: String,
    allowed_contents: Vec<Constraint>,
}

struct Constraint {
    quantity: u8,
    colour: String,
}

impl Rule {
    fn parse(rule_str: &str) -> Rule {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new("([a-z ]+) bags contain (.+)\\.").unwrap();
        }

        match RULE_RE.captures(rule_str) {
            Some(x) => {
                let colour = x.get(1).unwrap().as_str().to_string();
                let constraints_str = x.get(2).unwrap().as_str();
                let allowed_contents: Vec<Constraint> = match constraints_str {
                    "no other bags" => Vec::new(),
                    _ => Rule::parse_constraints(constraints_str),
                };

                Rule {
                    colour,
                    allowed_contents,
                }
            }
            None => unreachable!(),
        }
    }

    fn parse_constraints(constraints_str: &str) -> Vec<Constraint> {
        lazy_static! {
            static ref CONSTRAINT_RE: Regex = Regex::new("([1-9][0-9]*) ([a-z ]+) bags?").unwrap();
        }

        constraints_str
            .split(", ")
            .map(|s| match CONSTRAINT_RE.captures(s) {
                Some(x) => Constraint {
                    quantity: x.get(1).unwrap().as_str().parse().unwrap(),
                    colour: x.get(2).unwrap().as_str().to_string(),
                },
                None => unreachable!(),
            })
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify rule file");
    }
    let filename = args.get(1).unwrap();
    let file = File::open(filename).unwrap();

    let mut containers: HashMap<String, Vec<String>> = HashMap::new();
    let mut all_rules: HashMap<String, Rule> = HashMap::new();
    for r in BufReader::new(file).lines() {
        if let Ok(line) = r {
            let rule = Rule::parse(line.as_str());

            for c in rule.allowed_contents.iter() {
                if !containers.contains_key(&c.colour) {
                    containers.insert(c.colour.clone(), Vec::new());
                }
                containers
                    .get_mut(&c.colour)
                    .unwrap()
                    .push(rule.colour.clone());
            }

            all_rules.insert(rule.colour.clone(), rule);
        }
    }

    let initial = &"shiny gold".to_string();
    let mut visited: HashMap<String, bool> = HashMap::new();
    dfs(initial, &containers, &mut visited);
    visited.remove(initial);

    let count = visited.len();
    println!(
        "Part 1: {} kinds of bags can ultimately contain a {}:",
        count, initial
    );

    let total_bags = count_bags(initial, &all_rules) - 1;
    println!("Part 2: {} can be held by a {}", total_bags, initial);
}

fn dfs(
    search: &String,
    containers: &HashMap<String, Vec<String>>,
    visited: &mut HashMap<String, bool>,
) {
    visited.insert(search.clone(), true);
    match containers.get(search) {
        Some(neighbours) => {
            for n in neighbours.iter() {
                dfs(n, containers, visited)
            }
        }
        None => return,
    }
}

fn count_bags(colour: &String, rules: &HashMap<String, Rule>) -> i32 {
    rules
        .get(colour)
        .and_then(|r| {
            Some(if r.allowed_contents.len() == 0 {
                1
            } else {
                let contents: i32 = r
                    .allowed_contents
                    .iter()
                    .map(|c| c.quantity as i32 * count_bags(&c.colour, rules))
                    .sum();
                contents + 1
            })
        })
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use crate::Rule;

    #[test]
    fn test_parse_rule() {
        let r = Rule::parse("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(String::from("light red"), r.colour);
        assert_eq!(2, r.allowed_contents.len());
        let c0 = &r.allowed_contents[0];
        assert_eq!("bright white", c0.colour);
        assert_eq!(1, c0.quantity);

        let c1 = &r.allowed_contents[1];
        assert_eq!("muted yellow", c1.colour);
        assert_eq!(2, c1.quantity);
    }

    #[test]
    fn test_parse_empty_rule() {
        let r = Rule::parse("dotted black bags contain no other bags.");
        assert_eq!(String::from("dotted black"), r.colour);
        assert_eq!(0, r.allowed_contents.len());
    }
}
