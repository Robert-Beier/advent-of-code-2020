#[macro_use]
extern crate lazy_static;
extern crate regex;

use lib::{read_input, solve};
use regex::{Captures, Regex};
use std::collections::HashMap;

fn parse_input(input: String) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split(char::is_whitespace)
                .filter(|entry| !entry.is_empty())
                .map(|entry| entry.split(':').collect::<Vec<&str>>())
                .map(|entry| {
                    (
                        entry.get(0).unwrap().to_string(),
                        entry.get(1).unwrap().to_string(),
                    )
                })
                .collect::<HashMap<String, String>>()
        })
        .collect()
}

fn part_one(passports: &Vec<HashMap<String, String>>) {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    solve("Part one", || {
        passports
            .iter()
            .filter(|passport| {
                required_fields
                    .iter()
                    .filter(|field| !passport.contains_key(**field))
                    .count()
                    == 0
            })
            .count()
    });
}

fn part_two(passports: &Vec<HashMap<String, String>>) {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    solve("Part two", || {
        passports
            .iter()
            .filter(|passport| {
                required_fields
                    .iter()
                    .filter(|field| {
                        !passport.contains_key(**field)
                            || !validate_field(**field, passport.get(**field).unwrap())
                    })
                    .count()
                    == 0
            })
            .count()
    });
}

fn validate_field(field: &str, value: &str) -> bool {
    lazy_static! {
        static ref FOUR_DIGITS: Regex = Regex::new(r"^\d{4}$").unwrap();
        static ref NINE_DIGITS: Regex = Regex::new(r"^\d{9}$").unwrap();
        static ref COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref HEIGHT: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }
    match field {
        "byr" => {
            FOUR_DIGITS.is_match(value) && {
                let year: usize = value.parse().unwrap();
                year >= 1920 && year <= 2002
            }
        }
        "iyr" => {
            FOUR_DIGITS.is_match(value) && {
                let year: usize = value.parse().unwrap();
                year >= 2010 && year <= 2020
            }
        }
        "eyr" => {
            FOUR_DIGITS.is_match(value) && {
                let year: usize = value.parse().unwrap();
                year >= 2020 && year <= 2030
            }
        }
        "hgt" => match HEIGHT.captures(value) as Option<Captures> {
            Some(height_with_unit) => {
                let height: usize = height_with_unit.get(1).unwrap().as_str().parse().unwrap();
                let unit = height_with_unit.get(2).unwrap().as_str();
                match unit {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false,
                }
            }
            None => false,
        },
        "hcl" => COLOR.is_match(value),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => NINE_DIGITS.is_match(value),
        _ => panic!(),
    }
}

fn main() {
    let passports = parse_input(read_input());
    part_one(&passports);
    part_two(&passports);
}
