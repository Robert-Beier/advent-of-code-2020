use lib::solve;
use std::collections::HashMap;

#[test]
fn get_nth_number_should_work_for_example_1() {
    let starting_numbers: Vec<usize> = vec![0, 3, 6];
    assert_eq!(get_nth_number(&starting_numbers, 9), 4);
}

#[test]
fn get_nth_number_should_work_for_example_2() {
    let starting_numbers: Vec<usize> = vec![1, 3, 2];
    assert_eq!(get_nth_number(&starting_numbers, 2020), 1);
}

#[test]
fn get_nth_number_should_work_for_example_3() {
    let starting_numbers: Vec<usize> = vec![2, 1, 3];
    assert_eq!(get_nth_number(&starting_numbers, 2020), 10);
}

#[test]
fn get_nth_number_should_work_for_example_4() {
    let starting_numbers: Vec<usize> = vec![3, 1, 2];
    assert_eq!(get_nth_number(&starting_numbers, 2020), 1836);
}

fn push_occurrences(
    number_occurrences: &mut HashMap<usize, Vec<usize>>,
    number: usize,
    turn: usize,
) {
    let occurrences = number_occurrences.entry(number).or_insert(Vec::new());
    occurrences.push(turn);
}

fn get_next_number(
    number_occurrences: &HashMap<usize, Vec<usize>>,
    last_number: usize,
    turn: usize,
) -> usize {
    match number_occurrences.get(&last_number) {
        None => 0,
        Some(occurrences) => turn - occurrences.last().unwrap(),
    }
}

fn get_nth_number(starting_numbers: &Vec<usize>, n: usize) -> usize {
    let mut last_number = starting_numbers.last().unwrap().clone();
    let mut number_occurrences: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, starting_number) in starting_numbers
        .iter()
        .take(starting_numbers.len() - 1)
        .enumerate()
    {
        push_occurrences(&mut number_occurrences, *starting_number, i + 1);
    }
    for turn in starting_numbers.len() + 1..n {
        let new_number = get_next_number(&number_occurrences, last_number, turn - 1);
        push_occurrences(&mut number_occurrences, last_number, turn - 1);
        last_number = new_number;
    }
    get_next_number(&number_occurrences, last_number, n - 1)
}

fn part_one(starting_numbers: &Vec<usize>) {
    solve("Part one", || get_nth_number(starting_numbers, 2020));
}

fn main() {
    let input = vec![9, 19, 1, 6, 0, 5, 4];
    part_one(&input);
}
