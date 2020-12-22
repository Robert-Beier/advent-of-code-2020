use lib::{read_input, solve};
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum AttendingPlayer {
    One,
    Two,
}

type Configuration = (VecDeque<u32>, VecDeque<u32>);
type ConfigurationHistory = Vec<Configuration>;

#[cfg(test)]
mod tests {
    use crate::{
        get_winner_score, play_game, play_game_recursive, play_round, play_round_recursive,
        AttendingPlayer,
    };
    use std::collections::VecDeque;

    #[test]
    fn play_round_should_work_for_example1() {
        let mut player_one = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut player_two = VecDeque::from(vec![5, 8, 4, 7, 10]);
        let card_player_one = player_one.pop_front().unwrap();
        let card_player_two = player_two.pop_front().unwrap();
        play_round(
            &mut player_one,
            &mut player_two,
            card_player_one,
            card_player_two,
        );
        assert_eq!(player_one, vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(player_two, vec![8, 4, 7, 10]);
    }

    #[test]
    fn play_round_should_work_for_example2() {
        let mut player_one = VecDeque::from(vec![2, 6, 3, 1, 9, 5]);
        let mut player_two = VecDeque::from(vec![8, 4, 7, 10]);
        let card_player_one = player_one.pop_front().unwrap();
        let card_player_two = player_two.pop_front().unwrap();
        play_round(
            &mut player_one,
            &mut player_two,
            card_player_one,
            card_player_two,
        );
        assert_eq!(player_one, vec![6, 3, 1, 9, 5]);
        assert_eq!(player_two, vec![4, 7, 10, 8, 2]);
    }

    #[test]
    fn play_game_should_work_for_example() {
        let mut player_one = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut player_two = VecDeque::from(vec![5, 8, 4, 7, 10]);
        play_game(&mut player_one, &mut player_two);
        assert_eq!(player_one, vec![]);
        assert_eq!(player_two, vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
    }

    #[test]
    fn get_winner_score_should_work_for_example() {
        let deck = VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(get_winner_score(&deck), 306);
    }

    #[test]
    fn play_round_recursive_should_work_for_non_recursive_round() {
        let mut player_one = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut player_two = VecDeque::from(vec![5, 8, 4, 7, 10]);
        let card_player_one = player_one.pop_front().unwrap();
        let card_player_two = player_two.pop_front().unwrap();
        play_round_recursive(
            &mut player_one,
            &mut player_two,
            card_player_one,
            card_player_two,
        );
        assert_eq!(player_one, vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(player_two, vec![8, 4, 7, 10]);
    }

    #[test]
    fn play_round_recursive_should_work_for_recursive_round() {
        let mut player_one = VecDeque::from(vec![1, 8, 3]);
        let mut player_two = VecDeque::from(vec![4, 10, 9, 7, 5, 6, 2]);
        let card_player_one = player_one.pop_front().unwrap();
        let card_player_two = player_two.pop_front().unwrap();
        play_round_recursive(
            &mut player_one,
            &mut player_two,
            card_player_one,
            card_player_two,
        );
        assert_eq!(player_one, vec![8, 3]);
        assert_eq!(player_two, vec![10, 9, 7, 5, 6, 2, 4, 1]);
    }

    #[test]
    fn play_game_recursive_should_work_for_not_infinite_loop_example() {
        let mut player_one = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut player_two = VecDeque::from(vec![5, 8, 4, 7, 10]);
        play_game_recursive(&mut player_one, &mut player_two);
        assert_eq!(player_one, vec![]);
        assert_eq!(player_two, vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
    }

    #[test]
    fn play_game_recursive_should_work_for_infinite_loop_example() {
        let mut player_one = VecDeque::from(vec![43, 19]);
        let mut player_two = VecDeque::from(vec![2, 29, 14]);
        let winner = play_game_recursive(&mut player_one, &mut player_two);
        assert_eq!(winner, AttendingPlayer::One);
    }
}

fn play_round(
    deck_player_one: &mut VecDeque<u32>,
    deck_player_two: &mut VecDeque<u32>,
    card_player_one: u32,
    card_player_two: u32,
) {
    match card_player_one.cmp(&card_player_two) {
        Ordering::Less => {
            deck_player_two.push_back(card_player_two);
            deck_player_two.push_back(card_player_one);
        }
        Ordering::Equal => panic!(),
        Ordering::Greater => {
            deck_player_one.push_back(card_player_one);
            deck_player_one.push_back(card_player_two);
        }
    }
}

fn play_game(
    deck_player_one: &mut VecDeque<u32>,
    deck_player_two: &mut VecDeque<u32>,
) -> AttendingPlayer {
    while deck_player_one.len() > 0 && deck_player_two.len() > 0 {
        let card_player_one = deck_player_one.pop_front().unwrap();
        let card_player_two = deck_player_two.pop_front().unwrap();
        play_round(
            deck_player_one,
            deck_player_two,
            card_player_one,
            card_player_two,
        );
    }
    if deck_player_one.len() > 0 && deck_player_two.len() == 0 {
        AttendingPlayer::One
    } else if deck_player_two.len() > 0 && deck_player_one.len() == 0 {
        AttendingPlayer::Two
    } else {
        panic!()
    }
}

fn play_round_recursive(
    deck_player_one: &mut VecDeque<u32>,
    deck_player_two: &mut VecDeque<u32>,
    card_player_one: u32,
    card_player_two: u32,
) {
    if deck_player_one.len() as u32 >= card_player_one
        && deck_player_two.len() as u32 >= card_player_two
    {
        let mut sub_deck_player_one = deck_player_one.clone();
        sub_deck_player_one.truncate(card_player_one as usize);
        let mut sub_deck_player_two = deck_player_two.clone();
        sub_deck_player_two.truncate(card_player_two as usize);
        let winner = play_game_recursive(&mut sub_deck_player_one, &mut sub_deck_player_two);
        match winner {
            AttendingPlayer::One => {
                deck_player_one.push_back(card_player_one);
                deck_player_one.push_back(card_player_two);
            }
            AttendingPlayer::Two => {
                deck_player_two.push_back(card_player_two);
                deck_player_two.push_back(card_player_one);
            }
        }
    } else {
        play_round(
            deck_player_one,
            deck_player_two,
            card_player_one,
            card_player_two,
        );
    }
}

fn add_configuration_to_history(
    configuration_history: &mut ConfigurationHistory,
    deck_player_one: &VecDeque<u32>,
    deck_player_two: &VecDeque<u32>,
) {
    let configuration = (deck_player_one.clone(), deck_player_two.clone());
    configuration_history.push(configuration);
}

fn configuration_history_contains_configuration(
    configuration_history: &ConfigurationHistory,
    deck_player_one: &VecDeque<u32>,
    deck_player_two: &VecDeque<u32>,
) -> bool {
    configuration_history.iter().any(|(d1, d2)| {
        deck_player_one.iter().zip(d1).all(|(c_a, c_b)| c_a == c_b)
            && deck_player_two.iter().zip(d2).all(|(c_a, c_b)| c_a == c_b)
    })
}

/** Return value signals the winning player.
*/
fn play_game_recursive(
    deck_player_one: &mut VecDeque<u32>,
    deck_player_two: &mut VecDeque<u32>,
) -> AttendingPlayer {
    let mut configuration_history: ConfigurationHistory = Vec::new();
    while deck_player_one.len() > 0
        && deck_player_two.len() > 0
        && !configuration_history_contains_configuration(
            &configuration_history,
            deck_player_one,
            deck_player_two,
        )
    {
        add_configuration_to_history(&mut configuration_history, deck_player_one, deck_player_two);
        let card_player_one = deck_player_one.pop_front().unwrap();
        let card_player_two = deck_player_two.pop_front().unwrap();
        play_round_recursive(
            deck_player_one,
            deck_player_two,
            card_player_one,
            card_player_two,
        );
    }
    if configuration_history_contains_configuration(
        &configuration_history,
        deck_player_one,
        deck_player_two,
    ) {
        AttendingPlayer::One
    } else if deck_player_one.len() > 0 && deck_player_two.len() == 0 {
        AttendingPlayer::One
    } else if deck_player_two.len() > 0 && deck_player_one.len() == 0 {
        AttendingPlayer::Two
    } else {
        panic!()
    }
}

fn get_winner_score(deck: &VecDeque<u32>) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .fold(0, |score, (i, card)| score + (i as u32 + 1) * card)
}

fn parse_decks(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut decks: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|d| d.lines().skip(1).map(|l| l.parse().unwrap()).collect())
        .collect();
    (decks.remove(0), decks.remove(0))
}

fn part_one(deck_player_one: &Vec<u32>, deck_player_two: &Vec<u32>) {
    solve("Part one", || {
        let mut deck_player_one = VecDeque::from(deck_player_one.clone());
        let mut deck_player_two = VecDeque::from(deck_player_two.clone());
        let winner = play_game(&mut deck_player_one, &mut deck_player_two);
        let winner_deck = match winner {
            AttendingPlayer::One => deck_player_one,
            AttendingPlayer::Two => deck_player_two,
        };
        get_winner_score(&winner_deck)
    });
}

fn part_two(deck_player_one: &Vec<u32>, deck_player_two: &Vec<u32>) {
    solve("Part two", || {
        let mut deck_player_one = VecDeque::from(deck_player_one.clone());
        let mut deck_player_two = VecDeque::from(deck_player_two.clone());
        let winner = play_game_recursive(&mut deck_player_one, &mut deck_player_two);
        let winner_deck = match winner {
            AttendingPlayer::One => deck_player_one,
            AttendingPlayer::Two => deck_player_two,
        };
        get_winner_score(&winner_deck)
    });
}

/**
 * Game rule: There may be no duplicate numbers.
 */
fn main() {
    let input = read_input();
    let (deck_player_one, deck_player_two) = parse_decks(&input);
    part_one(&deck_player_one, &deck_player_two);
    part_two(&deck_player_one, &deck_player_two);
}
