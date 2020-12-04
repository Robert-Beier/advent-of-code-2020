use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_input() -> Vec<HashMap<String, String>> {
    let input: String = fs::read_to_string("input.txt").expect("Failed reading input.txt");
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
    let now = Instant::now();
    let number_of_valid_passports = passports
        .iter()
        .filter(|passport| {
            required_fields
                .iter()
                .filter(|field| !passport.contains_key(**field))
                .count()
                == 0
        })
        .count();
    println!("Part one took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part one:\n{}\n", number_of_valid_passports);
}

fn main() {
    let passports = read_input();
    part_one(&passports)
}
