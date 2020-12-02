use std::fs;
use std::time::Instant;

#[test]
fn find_two_summands_should_solve_example_1() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_two_summands(&input, 2020), Some((1721, 299)));
}

#[test]
fn find_three_summands_should_solve_example_1() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_three_summands(&input, 2020), Some((979, 366, 675)));
}

fn find_two_summands(potential_summands: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    for a in potential_summands {
        let b = potential_summands.iter().find(|&&x| x == (sum - a));
        if let Some(b) = b {
            return Some((*a, *b));
        }
    }
    None
}

fn find_three_summands(potential_summands: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    for a in potential_summands {
        let summands = find_two_summands(potential_summands, sum - a);
        if let Some(summands) = summands {
            return Some((*a, summands.0, summands.1));
        }
    }
    None
}

fn read_input() ->  Vec<i32> {
    let input = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    input.lines().map(|n| n.parse::<i32>().expect("Failed to parse number.")).collect()
}

fn part_one(potential_summands: &mut Vec<i32>) {
    let now = Instant::now();
    potential_summands.sort();
    let summands = find_two_summands(potential_summands, 2020);
    if let Some(summands) = summands {
        println!("Part one took {} nano seconds", now.elapsed().as_nanos());
        println!("Result of part one:\n{}\n", summands.0 * summands.1);
    } else {
        println!("No result found for part one.");
    }
}

fn part_two(potential_summands: &mut Vec<i32>) {
    let now = Instant::now();
    potential_summands.sort();
    let summands = find_three_summands(potential_summands, 2020);
    if let Some(summands) = summands {
        println!("Part one took {} nano seconds", now.elapsed().as_nanos());
        println!("Result of part two:\n{}\n", summands.0 * summands.1 * summands.2);
    } else {
        println!("No result found for part two.");
    }

}

fn main() {
    let mut input = read_input();
    part_one(&mut input);
    part_two(&mut input);
}
