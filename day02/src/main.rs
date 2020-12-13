use lib::{read_input, solve};
use regex::Regex;

#[derive(Debug)]
struct PasswordAndPolicy {
    password: String,
    required_character: char,
    first_number: u32,
    second_number: u32,
}

fn parse_input(input: String) -> Vec<PasswordAndPolicy> {
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    input
        .lines()
        .map(|line| regex.captures(line))
        .map(|cap| cap.unwrap())
        .map(|cap| PasswordAndPolicy {
            first_number: (&cap[1]).parse().unwrap(),
            second_number: (&cap[2]).parse().unwrap(),
            required_character: (&cap[3]).parse().unwrap(),
            password: (&cap[4]).to_string(),
        })
        .collect()
}

fn part_one(passwords_and_policies: &Vec<PasswordAndPolicy>) {
    solve("Part one", || {
        passwords_and_policies
            .iter()
            .filter(|x| {
                let count = x.password.matches(x.required_character).count();
                count <= x.second_number as usize && count >= x.first_number as usize
            })
            .count()
    })
}
fn part_two(passwords_and_policies: &Vec<PasswordAndPolicy>) {
    solve("Part two", || {
        passwords_and_policies
            .iter()
            .filter(|x| {
                (x.password
                    .chars()
                    .nth((x.first_number - 1) as usize)
                    .unwrap()
                    == x.required_character)
                    != (x
                        .password
                        .chars()
                        .nth((x.second_number - 1) as usize)
                        .unwrap()
                        == x.required_character)
            })
            .count()
    })
}

fn main() {
    let passwords_and_policies = parse_input(read_input());
    part_one(&passwords_and_policies);
    part_two(&passwords_and_policies);
}
