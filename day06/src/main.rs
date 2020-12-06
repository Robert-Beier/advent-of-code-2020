use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn part_one(input: &String) {
    let now = Instant::now();
    let groups: Vec<usize> = input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|char| *char != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .collect();
    let solution = groups.iter().fold(0, |a, n| a + n);

    println!("Part one took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part one:\n{:?}\n", solution);
}

fn part_two(input: &String) {
    let now = Instant::now();
    let groups: Vec<usize> = input
        .split("\n\n")
        .map(|group| {
            (
                group.split('\n').count(),
                group.chars().filter(|char| *char != '\n').fold(
                    HashMap::new(),
                    |mut map: HashMap<char, usize>, c| {
                        *map.entry(c).or_insert(0) += 1;
                        map
                    },
                ),
            )
        })
        .map(|(members, answers): (usize, HashMap<char, usize>)| {
            answers.iter().filter(|(_, v)| **v == members).count()
        })
        .collect();

    let solution = groups.iter().fold(0, |a, n| a + n);

    println!("Part two took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part two:\n{:?}\n", solution);
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("Failed reading input.txt")
        .trim()
        .to_string();
    part_one(&input);
    part_two(&input);
}
