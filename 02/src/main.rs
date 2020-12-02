use regex::Regex;
use std::fs;
use std::time::Instant;

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

fn part_one(passwords_and_policies: Vec<PasswordAndPolicy>) {
    let now = Instant::now();
    let number_of_valid_passwords = passwords_and_policies
        .iter()
        .filter(|x| {
            let count = x.password.matches(x.required_character).count();
            count <= x.required_character_max_count as usize
                && count >= x.required_character_min_count as usize
        })
        .count();
    println!("Part one took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part one:\n{}\n", number_of_valid_passwords);
}

fn main() {
    let passwords_and_policies = read_input();
    part_one(passwords_and_policies);
}
