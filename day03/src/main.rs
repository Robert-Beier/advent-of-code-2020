use lib::solve;
use std::fs;

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
    solve("Part two", || {
        let configs = [
            Config { x: 1, y: 1 },
            Config { x: 3, y: 1 },
            Config { x: 5, y: 1 },
            Config { x: 7, y: 1 },
            Config { x: 1, y: 2 },
        ];
        configs
            .iter()
            .map(|config| count_trees(input, config))
            .fold(1, |a, n| a * n)
    });
}

fn part_one(input: &String) {
    solve("Part one", || count_trees(&input, &Config { x: 3, y: 1 }));
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    part_one(&input);
    part_two(&input);
}
