use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn get_row(encoded_row: &str) -> usize {
    let binary_row: String = encoded_row
        .chars()
        .map(|char| match char {
            'F' => '0',
            'B' => '1',
            _ => panic!(),
        })
        .collect();
    usize::from_str_radix(&*binary_row, 2).unwrap()
}

fn get_column(encoded_column: &str) -> usize {
    let binary_column: String = encoded_column
        .chars()
        .map(|char| match char {
            'L' => '0',
            'R' => '1',
            _ => panic!(),
        })
        .collect();
    usize::from_str_radix(&*binary_column, 2).unwrap()
}

fn part_one(input: &String) {
    let now = Instant::now();
    let max_seat = input
        .lines()
        .map(|line| get_row(&line[0..7]) * 8 + get_column(&line[7..10]))
        .max_by(|seat_a, seat_b| (seat_a).cmp(seat_b))
        .unwrap();
    println!("Part one took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part one:\n{}\n", max_seat);
}

fn part_two(input: &String) {
    let now = Instant::now();
    let ids: HashSet<usize> = input
        .lines()
        .map(|line| get_row(&line[0..7]) * 8 + get_column(&line[7..10]))
        .collect();
    let mut solution: Option<usize> = None;
    for id in &ids {
        if !ids.contains(&(id + 1)) && ids.contains(&(id + 2)) {
            solution = Some(*id + 1);
            break;
        }
    }
    let solution = solution.unwrap();
    println!("Part two took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part two:\n{}\n", solution);
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    part_one(&input);
    part_two(&input);
}
