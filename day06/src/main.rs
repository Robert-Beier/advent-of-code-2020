use std::collections::HashSet;
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
fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    part_one(&input);
}
