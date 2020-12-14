mod memory;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use crate::memory::{Instruction, Memory};
use lib::{read_input, solve};
use regex::{Captures, Regex};

fn parse_instructions(instructions: &String) -> Vec<Instruction> {
    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = (?P<mask>[01X]+)$").unwrap();
        static ref MEM: Regex = Regex::new(r"^mem\[(?P<position>\d+)\] = (?P<value>\d+)$").unwrap();
    }
    instructions
        .lines()
        .map(|l| {
            let mask = MASK.captures(l) as Option<Captures>;
            let mem = MEM.captures(l) as Option<Captures>;
            if mask.is_some() {
                return Instruction::UpdateMask(
                    mask.unwrap().name("mask").unwrap().as_str().to_string(),
                );
            } else if mem.is_some() {
                let mem = mem.unwrap();
                return Instruction::WriteToMemory(
                    mem.name("position").unwrap().as_str().parse().unwrap(),
                    mem.name("value").unwrap().as_str().parse().unwrap(),
                );
            }
            panic!();
        })
        .collect()
}

fn part_one(instructions: &Vec<Instruction>) {
    solve("Part one", || {
        let mut memory = Memory::new();
        memory.execute(instructions);
        memory.sum()
    });
}

fn main() {
    let input = read_input();
    let instructions = parse_instructions(&input);
    part_one(&instructions);
}
