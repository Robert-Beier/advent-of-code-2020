use lib::{read_input, solve};
use std::collections::{HashMap, HashSet};

fn part_one(input: &String) {
    solve("Part one", || {
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
        groups.iter().fold(0, |a, n| a + n)
    });
}

fn part_two(input: &String) {
    solve("Part two", || {
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

        groups.iter().fold(0, |a, n| a + n)
    });
}

fn main() {
    let input: String = read_input();
    part_one(&input);
    part_two(&input);
}
