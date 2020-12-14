use std::collections::HashMap;

pub enum Instruction {
    UpdateMask(String),
    WriteToMemory(u64, u64),
}

pub type Mask = (u64, u64);

#[test]
fn get_masked_value_should_work_for_example_1() {
    let value = 11u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let mask = Memory::get_mask(mask);
    assert_eq!(Memory::get_masked_value(value, mask), 73);
}

#[test]
fn get_masked_value_should_work_for_example_2() {
    let value = 101u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let mask = Memory::get_mask(mask);
    assert_eq!(Memory::get_masked_value(value, mask), 101);
}

#[test]
fn get_masked_value_should_work_for_example_3() {
    let value = 0u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let mask = Memory::get_mask(mask);
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

pub struct Memory {
    memory: HashMap<u64, u64>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    fn update(&mut self, position: u64, value: u64, mask: Mask) {
        let masked_value = Self::get_masked_value(value, mask);
        self.memory.insert(position, masked_value);
    }

    fn get_masked_value(value: u64, mask: Mask) -> u64 {
        (value | mask.1) & mask.0
    }

    fn get_mask(mask: &str) -> Mask {
        let mask_zeroes = mask.replace('X', "1");
        let mask_ones = mask.replace('X', "0");
        (
            u64::from_str_radix(&*mask_zeroes, 2).unwrap(),
            u64::from_str_radix(&*mask_ones, 2).unwrap(),
        )
    }

    pub fn execute(&mut self, instructions: &Vec<Instruction>) {
        let mut mask = (0, 0);
        for instruction in instructions {
            match instruction {
                Instruction::UpdateMask(updated_mask) => {
                    mask = Self::get_mask(updated_mask);
                }
                Instruction::WriteToMemory(position, value) => {
                    self.update(*position, *value, mask);
                }
            }
        }
    }

    pub fn sum(self) -> u64 {
        self.memory.values().fold(0, |a, n| a + n)
    }
}
