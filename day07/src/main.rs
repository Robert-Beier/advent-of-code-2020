#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::{Captures, Regex};
use std::fs;

#[derive(Debug)]
struct Rule {
    outer_bag: String,
    inner_bags: Vec<(String, usize)>,
}

fn parse_inner_bags(inner_bags: &str) -> Vec<(String, usize)> {
    lazy_static! {
        static ref INNER_BAGS: Regex =
            Regex::new(r"^(?P<amount>\d+) (?P<name>[a-z ]+) bags?$").unwrap();
    }
    if inner_bags == "no other bags" {
        return vec![];
    }
    inner_bags
        .split(", ")
        .map(|inner_bag| INNER_BAGS.captures(inner_bag).unwrap() as Captures)
        .map(|inner_bag| {
            (
                inner_bag.name("name").unwrap().as_str().to_string(),
                inner_bag.name("amount").unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn create_rules_list(input: &String) -> Vec<Rule> {
    let rule: Regex =
        Regex::new(r"^(?P<outer_bag>[a-z ]+) bags contain (?P<inner_bags>.+)\.$").unwrap();
    // const INNER_BAGS: Regex = Regex::new(r"");
    input
        .lines()
        .map(|line| rule.captures(line).unwrap())
        .map(|captures| Rule {
            outer_bag: captures.name("outer_bag").unwrap().as_str().to_string(),
            inner_bags: parse_inner_bags(captures.name("inner_bags").unwrap().as_str()),
        })
        .collect()
}

fn create_rules_tree() {}

fn read_input() {
    let input: String = fs::read_to_string("input.txt")
        .expect("Failed reading input.txt")
        .trim()
        .to_string();
    let rules = create_rules_list(&input);
    println!("{:?}", rules);
}

fn main() {
    read_input();
}
