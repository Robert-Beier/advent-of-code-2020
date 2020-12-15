use lib::solve;
use std::collections::HashMap;

type History = HashMap<usize, Vec<usize>>;
type StartingNumbers = Vec<usize>;

#[test]
fn get_nth_number_should_work_for_example_1() {
    let starting_numbers: StartingNumbers = vec![0, 3, 6];
    assert_eq!(get_nth_number(&starting_numbers, 9), 4);
}

#[test]
fn get_nth_number_should_work_for_example_2() {
    let starting_numbers: StartingNumbers = vec![1, 3, 2];
    assert_eq!(get_nth_number(&starting_numbers, 2020), 1);
}

#[test]
fn get_nth_number_should_work_for_example_3() {
    let starting_numbers: StartingNumbers = vec![2, 1, 3];
    assert_eq!(get_nth_number(&starting_numbers, 2020), 10);
}

#[test]
fn get_nth_number_should_work_for_example_4() {
    let starting_numbers: StartingNumbers = vec![3, 1, 2];
    assert_eq!(get_nth_number(&starting_numbers, 2020), 1836);
}

fn push_turn(history: &mut History, number: usize, turn: usize) {
    let turns = history.entry(number).or_insert(Vec::new());
    turns.push(turn);
}

fn get_next_number(history: &History, last_number: usize, turn: usize) -> usize {
    match history.get(&last_number) {
        None => 0,
        Some(turns) => turn - turns.last().unwrap(),
    }
}

fn get_nth_number(starting_numbers: &StartingNumbers, n: usize) -> usize {
    let mut history: History = HashMap::new();
    for (i, starting_number) in starting_numbers.iter().enumerate() {
        push_turn(&mut history, *starting_number, i + 1);
    }
    let mut next_number = get_next_number(
        &history,
        *starting_numbers.last().unwrap(),
        starting_numbers.len(),
    );
    for turn in starting_numbers.len() + 1..n {
        let current_number = next_number;
        next_number = get_next_number(&history, current_number, turn);
        push_turn(&mut history, current_number, turn);
    }
    next_number
}

fn part_one(starting_numbers: &StartingNumbers) {
    solve("Part one", || get_nth_number(starting_numbers, 2020));
}

fn main() {
    let input = vec![9, 19, 1, 6, 0, 5, 4];
    part_one(&input);
}
