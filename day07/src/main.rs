use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Rule {
    outer_bag: String,
    inner_bags: String,
    // inner_bags: Vec<(String, usize)>,
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
            inner_bags: captures.name("inner_bags").unwrap().as_str().to_string(),
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
