use regex::Regex;
use std::fs;

#[derive(Debug)]
struct PasswordAndPolicy {
    password: String,
    required_character: char,
    required_character_min_count: u32,
    required_character_max_count: u32,
}

fn read_input() -> Vec<PasswordAndPolicy> {
    let input = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    input
        .lines()
        .map(|line| regex.captures(line))
        .map(|cap| cap.unwrap())
        .map(|cap| PasswordAndPolicy {
            required_character_min_count: (&cap[1]).parse().unwrap(),
            required_character_max_count: (&cap[2]).parse().unwrap(),
            required_character: (&cap[3]).parse().unwrap(),
            password: (&cap[4]).to_string(),
        })
        .collect()
}

fn main() {
    let passwords_and_policies = read_input();
    println!("{:?}", passwords_and_policies);
}
