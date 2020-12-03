use std::fs;
use std::time::Instant;

fn part_one(input: String) {
    let now = Instant::now();
    let mut x: usize = 0;
    let mut number_of_trees = 0;
    let line_length = input.lines().next().map(|line| line.len()).unwrap();
    for line in input.lines() {
        if line.chars().nth(x).unwrap() == '#' {
            number_of_trees += 1;
        }
        x = (x + 3) % line_length;
    }
    println!("Part one took {} nano seconds", now.elapsed().as_nanos());
    println!("Result of part one:\n{}\n", number_of_trees);

}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed reading input.txt");
    part_one(input);
}
