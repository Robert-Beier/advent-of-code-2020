use std::fmt::Display;
use std::fs;
use std::time::Instant;

pub fn solve<F: Fn() -> T, T: Display>(name: &str, f: F) {
    let now = Instant::now();
    let solution = f();
    println!("{}", name);
    println!("Duration: {}μs", now.elapsed().as_micros());
    println!("Solution: {}\n", solution);
}

pub fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Failed reading input.txt")
        .trim()
        .to_string()
}
