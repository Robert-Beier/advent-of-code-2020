use std::fs;
use std::time::Instant;

struct Config {
    x: usize,
    y: usize,
}

fn count_trees(input: &String, config: &Config) -> usize {
    let mut x: usize = 0;
    let mut number_of_trees = 0;
    let line_length = input.lines().next().map(|line| line.len()).unwrap();
    for (y, line) in input.lines().enumerate() {
        if y % config.y == 0 {
            if line.chars().nth(x).unwrap() == '#' {
                number_of_trees += 1;
            }
            x = (x + config.x) % line_length;
        }
    }
    number_of_trees
}

fn part_two(input: &String) {
    let now = Instant::now();
    let configs = [
        Config { x: 1, y: 1 },
        Config { x: 3, y: 1 },
        Config { x: 5, y: 1 },
        Config { x: 7, y: 1 },
        Config { x: 1, y: 2 },
    ];
    let number_of_trees = configs
        .iter()
        .map(|config| count_trees(input, config))
        .fold(1, |a, n| a * n);
    println!("Part two took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part two:\n{}\n", number_of_trees);
}

fn part_one(input: &String) {
    let now = Instant::now();
    let number_of_trees = count_trees(&input, &Config { x: 3, y: 1 });
    println!("Part one took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part one:\n{}\n", number_of_trees);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    part_one(&input);
    part_two(&input);
}
