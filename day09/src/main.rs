use lib::solve;
use std::fs;

#[test]
fn find_two_summands_should_solve_example_1() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_two_summands(&input, 2020), Some((1721, 299)));
}

#[test]
fn find_two_summands_should_work_with_inputs_smaller_than_sum() {
    let input = vec![1721, 3000, 366, 299, 675, 1456];
    assert_eq!(find_two_summands(&input, 2020), Some((1721, 299)));
}

fn find_two_summands(potential_summands: &[usize], sum: usize) -> Option<(usize, usize)> {
    for (index, a) in potential_summands.iter().enumerate() {
        let b = potential_summands
            .iter()
            .enumerate()
            .find(|(i, x)| *i != index && sum >= *a && **x == (sum - a));
        if let Some((_, b)) = b {
            return Some((*a, *b));
        }
    }
    None
}

#[test]
fn is_number_valid_should_return_true_for_valid_number() {
    let preamble: Vec<usize> = (1..26).collect();
    let number = 49;
    assert_eq!(is_number_valid(&*preamble, number), true);
}

#[test]
fn is_number_valid_should_return_false_for_invalid_number() {
    let preamble: Vec<usize> = (1..26).collect();
    let number = 50;
    assert_eq!(is_number_valid(&*preamble, number), false);
}

fn is_number_valid(preamble: &[usize], number: usize) -> bool {
    find_two_summands(preamble, number).is_some()
}

#[test]
fn find_first_invalid_number_should_solve_example_1() {
    let numbers = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(find_first_invalid_number(numbers, 5), 127);
}

fn find_first_invalid_number(numbers: &[usize], preamble_length: usize) -> usize {
    *numbers
        .split_at(preamble_length)
        .1
        .iter()
        .enumerate()
        .find(|(i, n)| !is_number_valid(&numbers[*i..i + preamble_length], **n))
        .unwrap()
        .1
}

#[test]
fn find_contiguous_summands_should_solve_example_1() {
    let numbers = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(find_contiguous_summands(numbers, 127), [15, 25, 47, 40]);
}

fn find_contiguous_summands(numbers: &[usize], sum: usize) -> &[usize] {
    (0..numbers.len())
        .find_map(|i| {
            for i2 in i + 1..numbers.len() {
                let potential_summands = &numbers[i..i2 + 1];
                let sum_of_potential_summands: usize = potential_summands.iter().sum();
                if sum_of_potential_summands == sum {
                    return Some(potential_summands);
                } else if sum_of_potential_summands > sum {
                    return None;
                }
            }
            None
        })
        .unwrap()
}

fn part_one(numbers: &[usize]) {
    solve("Part one", || find_first_invalid_number(numbers, 25));
}

fn part_two(numbers: &[usize]) {
    let invalid_number = find_first_invalid_number(numbers, 25);
    solve("Part two", || {
        let summands = find_contiguous_summands(numbers, invalid_number);
        let min = summands.iter().min().unwrap();
        let max = summands.iter().max().unwrap();
        min + max
    })
}

fn main() {
    let input: Vec<usize> = fs::read_to_string("input.txt")
        .expect("Failed reading input.txt")
        .trim()
        .to_string()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    part_one(&input);
    part_two(&input);
}
