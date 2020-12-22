use lib::{read_input, solve};
use std::cmp::Ordering;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use crate::{get_winner_score, play_game, play_round};
    use std::collections::VecDeque;

    #[test]
    fn play_round_should_work_for_example1() {
        let mut player_one = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut player_two = VecDeque::from(vec![5, 8, 4, 7, 10]);
        play_round(&mut player_one, &mut player_two);
        assert_eq!(player_one, vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(player_two, vec![8, 4, 7, 10]);
    }

    #[test]
    fn play_round_should_work_for_example2() {
        let mut player_one = VecDeque::from(vec![2, 6, 3, 1, 9, 5]);
        let mut player_two = VecDeque::from(vec![8, 4, 7, 10]);
        play_round(&mut player_one, &mut player_two);
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
        let mut player_one = VecDeque::from(vec![]);
        let mut player_two = VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(get_winner_score(&player_one, &player_two), 306);
    }
}

/** Requires the parameter vectors to not be empty. There also may not be duplicate numbers.
*/
fn play_round(deck_player_one: &mut VecDeque<u32>, deck_player_two: &mut VecDeque<u32>) {
    let card_player_one = deck_player_one.pop_front().unwrap();
    let card_player_two = deck_player_two.pop_front().unwrap();
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

/** There may not be duplicate numbers.
*/
fn play_game(deck_player_one: &mut VecDeque<u32>, deck_player_two: &mut VecDeque<u32>) {
    while deck_player_one.len() > 0 && deck_player_two.len() > 0 {
        play_round(deck_player_one, deck_player_two);
    }
}

fn get_winner_score(deck_player_one: &VecDeque<u32>, deck_player_two: &VecDeque<u32>) -> u32 {
    let winner_deck = if deck_player_one.len() > 0 && deck_player_two.len() == 0 {
        deck_player_one
    } else if deck_player_two.len() > 0 && deck_player_one.len() == 0 {
        deck_player_two
    } else {
        panic!()
    };
    winner_deck
        .iter()
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
        play_game(&mut deck_player_one, &mut deck_player_two);
        get_winner_score(&deck_player_one, &deck_player_two)
    });
}

fn main() {
    let input = read_input();
    let (deck_player_one, deck_player_two) = parse_decks(&input);
    part_one(&deck_player_one, &deck_player_two);
}
