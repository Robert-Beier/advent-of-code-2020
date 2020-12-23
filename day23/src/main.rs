use lib::solve;

#[cfg(test)]
mod tests {
    use crate::{
        get_destination_cup_index, get_next_cup_label, get_solution_labels, insert_cups, play_game,
        play_move, take_three_cups,
    };

    #[test]
    fn take_three_cups_should_work_for_three_cups_in_the_middle() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let current_cup_label = 3;
        let three_cups = take_three_cups(&mut cups, current_cup_label);
        assert_eq!(cups, vec![3, 2, 5, 4, 6, 7]);
        assert_eq!(three_cups, vec![8, 9, 1]);
    }

    #[test]
    fn take_three_cups_should_work_for_three_cups_in_the_beginning() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let current_cup_label = 7;
        let three_cups = take_three_cups(&mut cups, current_cup_label);
        assert_eq!(cups, vec![1, 2, 5, 4, 6, 7]);
        assert_eq!(three_cups, vec![3, 8, 9]);
    }

    #[test]
    fn take_three_cups_should_work_for_three_cups_partially_in_the_beginning_and_the_end() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let current_cup_label = 4;
        let three_cups = take_three_cups(&mut cups, current_cup_label);
        assert_eq!(cups, vec![8, 9, 1, 2, 5, 4]);
        assert_eq!(three_cups, vec![6, 7, 3]);
    }

    #[test]
    fn insert_cups_should_work_for_inserting_at_the_end() {
        let mut cups = vec![3, 2, 5, 4, 6, 7];
        let cups_to_insert = vec![8, 9, 1];
        let destination_cup_index = 5;
        insert_cups(&mut cups, cups_to_insert, destination_cup_index);
        assert_eq!(cups, vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);
    }

    #[test]
    fn insert_cups_should_work_for_inserting_in_the_middle() {
        let mut cups = vec![3, 2, 5, 4, 6, 7];
        let cups_to_insert = vec![8, 9, 1];
        let destination_cup_index = 1;
        insert_cups(&mut cups, cups_to_insert, destination_cup_index);
        assert_eq!(cups, vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }

    #[test]
    fn get_destination_cup_index_should_return_cup_with_minus_one_label_when_it_is_in_the_cups() {
        let cups = vec![3, 2, 5, 4, 8, 7];
        let current_cup_label = 3;
        let max_cup_label = 9;
        let destination_cup_index =
            get_destination_cup_index(&cups, current_cup_label, max_cup_label);
        assert_eq!(destination_cup_index, 1);
    }

    #[test]
    fn get_destination_cup_index_should_return_cup_with_next_lowest_label_when_minus_one_is_not_in_the_cups(
    ) {
        let cups = vec![3, 2, 5, 4, 8, 7];
        let current_cup_label = 7;
        let max_cup_label = 9;
        let destination_cup_index =
            get_destination_cup_index(&cups, current_cup_label, max_cup_label);
        assert_eq!(destination_cup_index, 2);
    }

    #[test]
    fn get_destination_cup_index_should_return_cup_with_highest_label_in_cups_when_there_is_no_cup_with_lower_label_in_the_cups(
    ) {
        let cups = vec![3, 2, 5, 4, 8, 7];
        let current_cup_label = 2;
        let max_cup_label = 9;
        let destination_cup_index =
            get_destination_cup_index(&cups, current_cup_label, max_cup_label);
        assert_eq!(destination_cup_index, 4);
    }

    #[test]
    fn play_move_should_work_for_example1() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let current_cup_label = 3;
        play_move(&mut cups, current_cup_label);
        assert_eq!(cups, vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }

    #[test]
    fn play_move_should_work_for_example2() {
        let mut cups = vec![3, 2, 8, 9, 1, 5, 4, 6, 7];
        let current_cup_label = 2;
        play_move(&mut cups, current_cup_label);
        assert_eq!(cups, vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);
    }

    #[test]
    fn play_move_should_work_for_example3() {
        let mut cups = vec![8, 3, 6, 7, 4, 1, 9, 2, 5];
        let current_cup_label = 2;
        play_move(&mut cups, current_cup_label);
        assert_eq!(cups, vec![6, 7, 4, 1, 5, 8, 3, 9, 2]);
    }

    #[test]
    fn get_next_cup_label_should_work_for_current_cup_in_the_middle() {
        let cups = vec![8, 3, 6, 7, 4, 1, 9, 2, 5];
        let current_cup_label = 7;
        let next_cup_label = get_next_cup_label(&cups, current_cup_label);
        assert_eq!(next_cup_label, 4);
    }

    #[test]
    fn get_next_cup_label_should_work_for_current_cup_at_the_end() {
        let cups = vec![8, 3, 6, 7, 4, 1, 9, 2, 5];
        let current_cup_label = 5;
        let next_cup_label = get_next_cup_label(&cups, current_cup_label);
        assert_eq!(next_cup_label, 8);
    }

    #[test]
    fn play_game_should_work_for_10_moves() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let number_of_moves = 10;
        play_game(&mut cups, number_of_moves);
        assert_eq!(cups, vec![5, 8, 3, 7, 4, 1, 9, 2, 6])
    }

    #[test]
    fn play_game_should_work_for_100_moves() {
        let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let number_of_moves = 100;
        play_game(&mut cups, number_of_moves);
        assert_eq!(cups, vec![2, 9, 1, 6, 7, 3, 8, 4, 5])
    }

    #[test]
    fn get_solution_labels_works_for_example1() {
        let cups = vec![5, 8, 3, 7, 4, 1, 9, 2, 6];
        let solution_labels = get_solution_labels(&cups);
        assert_eq!(solution_labels, "92658374")
    }

    #[test]
    fn get_solution_labels_works_for_example2() {
        let cups = vec![2, 9, 1, 6, 7, 3, 8, 4, 5];
        let solution_labels = get_solution_labels(&cups);
        assert_eq!(solution_labels, "67384529")
    }
}

fn take_three_cups(cups: &mut Vec<u32>, current_cup_label: u32) -> Vec<u32> {
    let current_cup_index = cups.iter().position(|c| *c == current_cup_label).unwrap();
    let number_of_cups_until_the_end = cups.len() - 1 - current_cup_index;
    (1..4)
        .into_iter()
        .map(|i| {
            if i > number_of_cups_until_the_end {
                0
            } else {
                current_cup_index + 1
            }
        })
        .map(|i| cups.remove(i))
        .collect()
}

fn insert_cups(cups: &mut Vec<u32>, cups_to_insert: Vec<u32>, destination_cup_index: usize) {
    cups_to_insert
        .into_iter()
        .rev()
        .for_each(|c| cups.insert(destination_cup_index + 1, c));
}

fn get_destination_cup_index(cups: &Vec<u32>, current_cup_label: u32, max_cup_label: u32) -> usize {
    let max_cup_label_plus_one = (max_cup_label + 1) as i32;
    let mut destination_cup_index: Option<usize> = None;
    let mut label: i32 = current_cup_label as i32;
    while destination_cup_index == None {
        // this is a workaround needed to achieve actual modulus
        label = (((label - 1) % max_cup_label_plus_one) + max_cup_label_plus_one)
            % max_cup_label_plus_one;
        destination_cup_index = cups.iter().position(|c| *c == label as u32);
    }
    destination_cup_index.unwrap()
}

fn play_move(cups: &mut Vec<u32>, current_cup_label: u32) {
    let max_cup_label = cups.len() as u32;
    let three_cups = take_three_cups(cups, current_cup_label);
    let destination_cup_index = get_destination_cup_index(cups, current_cup_label, max_cup_label);
    insert_cups(cups, three_cups, destination_cup_index);
}

fn get_next_cup_label(cups: &Vec<u32>, current_cup_label: u32) -> u32 {
    let current_cup_index = cups.iter().position(|c| *c == current_cup_label).unwrap();
    let next_cup_index = (current_cup_index + 1) % cups.len();
    *cups.get(next_cup_index).unwrap()
}

fn play_game(cups: &mut Vec<u32>, number_of_moves: u32) {
    let mut current_cup_label = *cups.get(0).unwrap();
    for _ in 0..number_of_moves {
        play_move(cups, current_cup_label);
        current_cup_label = get_next_cup_label(cups, current_cup_label);
    }
}

fn get_solution_labels(cups: &Vec<u32>) -> String {
    let label_one_index = cups.iter().position(|c| *c == 1).unwrap();
    let before = &cups[0..label_one_index];
    let behind = if label_one_index + 1 < cups.len() {
        &cups[label_one_index + 1..cups.len()]
    } else {
        // will be empty
        &cups[cups.len()..cups.len()]
    };
    behind.iter().chain(before).map(|c| c.to_string()).collect()
}

fn part_one(cups: &Vec<u32>) {
    solve("Part one", || {
        let mut cups = cups.clone();
        play_game(&mut cups, 100);
        get_solution_labels(&cups)
    });
}

fn main() {
    let cups = vec![9, 6, 3, 2, 7, 5, 4, 8, 1];
    part_one(&cups);
}
