#[macro_use]
extern crate lazy_static;
extern crate regex;

use lib::{read_input, solve};
use regex::{Captures, Regex};
use std::collections::HashMap;

enum Instruction {
    UpdateMask(String),
    WriteToMemory(u64, u64),
}

#[test]
fn get_masked_value_should_work_for_example_1() {
    let value = 11u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(Memory::get_masked_value(value, mask), 73);
}

#[test]
fn get_masked_value_should_work_for_example_2() {
    let value = 101u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(Memory::get_masked_value(value, mask), 101);
}

#[test]
fn get_masked_value_should_work_for_example_3() {
    let value = 0u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(Memory::get_masked_value(value, mask), 64);
}

#[test]
fn execute_should_execute_example_instructions() {
    let instructions = vec![
        Instruction::UpdateMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
        Instruction::WriteToMemory(8, 11),
        Instruction::WriteToMemory(7, 101),
        Instruction::WriteToMemory(8, 0),
    ];
    let mut memory = Memory::new();
    memory.execute(&instructions);
    assert_eq!(*memory.memory.get(&7u64).unwrap(), 101u64);
    assert_eq!(*memory.memory.get(&8u64).unwrap(), 64u64);
}

#[test]
fn sum_should_return_correct_sum_after_executing_example_instructions() {
    let instructions = vec![
        Instruction::UpdateMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
        Instruction::WriteToMemory(8, 11),
        Instruction::WriteToMemory(7, 101),
        Instruction::WriteToMemory(8, 0),
    ];
    let mut memory = Memory::new();
    memory.execute(&instructions);
    let sum = memory.sum();
    assert_eq!(sum, 165)
}

struct Memory {
    memory: HashMap<u64, u64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    fn update(&mut self, position: u64, value: u64, mask: &str) {
        let masked_value = Self::get_masked_value(value, mask);
        self.memory.insert(position, masked_value);
    }

    fn get_masked_value(value: u64, mask: &str) -> u64 {
        let mask_ones = mask.replace('X', "0");
        let mask_zeroes = mask.replace('X', "1");
        let mask_ones = u64::from_str_radix(&*mask_ones, 2).unwrap();
        let mask_zeroes = u64::from_str_radix(&*mask_zeroes, 2).unwrap();
        (value | mask_ones) & mask_zeroes
    }

    fn execute(&mut self, instructions: &Vec<Instruction>) {
        let mut mask = "";
        for instruction in instructions {
            match instruction {
                Instruction::UpdateMask(updated_mask) => {
                    mask = updated_mask;
                }
                Instruction::WriteToMemory(position, value) => {
                    self.update(*position, *value, mask);
                }
            }
        }
    }

    fn sum(self) -> u64 {
        self.memory.values().fold(0, |a, n| a + n)
    }
}

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
