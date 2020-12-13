use day01::find_two_summands;
use lib::solve;
use std::fs;

fn find_three_summands(
    potential_summands: &Vec<usize>,
    sum: usize,
) -> Option<(usize, usize, usize)> {
    for a in potential_summands {
        let summands = find_two_summands(potential_summands, sum - a);
        if let Some(summands) = summands {
            return Some((*a, summands.0, summands.1));
        }
    }
    None
}

fn read_input() -> Vec<usize> {
    let input = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    input
        .lines()
        .map(|n| n.parse::<usize>().expect("Failed to parse number."))
        .collect()
}

fn part_one(potential_summands: &mut Vec<usize>) {
    // sorting should be moved in again for realistic measurement
    potential_summands.sort();
    solve("Part one", || {
        let summands = find_two_summands(potential_summands, 2020).unwrap();

        summands.0 * summands.1
    });
}

fn part_two(potential_summands: &mut Vec<usize>) {
    potential_summands.sort();
    solve("Part two", || {
        let summands = find_three_summands(potential_summands, 2020).unwrap();
        summands.0 * summands.1 * summands.2
    })
}

fn main() {
    let mut input = read_input();
    part_one(&mut input);
    part_two(&mut input);
}
