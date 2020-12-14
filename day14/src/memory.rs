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
    let mask = Memory::get_mask_version1(mask);
    assert_eq!(Memory::get_masked_value(value, mask), 73);
}

#[test]
fn get_masked_value_should_work_for_example_2() {
    let value = 101u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let mask = Memory::get_mask_version1(mask);
    assert_eq!(Memory::get_masked_value(value, mask), 101);
}

#[test]
fn get_masked_value_should_work_for_example_3() {
    let value = 0u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let mask = Memory::get_mask_version1(mask);
    assert_eq!(Memory::get_masked_value(value, mask), 64);
}

#[test]
fn execute_version1_should_execute_example_instructions() {
    let instructions = vec![
        Instruction::UpdateMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
        Instruction::WriteToMemory(8, 11),
        Instruction::WriteToMemory(7, 101),
        Instruction::WriteToMemory(8, 0),
    ];
    let mut memory = Memory::new();
    memory.execute_version1(&instructions);
    assert_eq!(*memory.memory.get(&7u64).unwrap(), 101u64);
    assert_eq!(*memory.memory.get(&8u64).unwrap(), 64u64);
}

#[test]
fn execute_version2_should_execute_example_instructions() {
    let instructions = vec![
        Instruction::UpdateMask("000000000000000000000000000000X1001X".to_string()),
        Instruction::WriteToMemory(42, 100),
        Instruction::UpdateMask("00000000000000000000000000000000X0XX".to_string()),
        Instruction::WriteToMemory(26, 1),
    ];
    let mut memory = Memory::new();
    memory.execute_version2(&instructions);
    let mut expected = HashMap::new();
    expected.insert(16, 1);
    expected.insert(17, 1);
    expected.insert(18, 1);
    expected.insert(19, 1);
    expected.insert(24, 1);
    expected.insert(25, 1);
    expected.insert(26, 1);
    expected.insert(27, 1);
    expected.insert(58, 100);
    expected.insert(59, 100);
    assert_eq!(memory.memory, expected);
}

#[test]
fn sum_should_return_correct_sum_after_executing_version1_instructions() {
    let instructions = vec![
        Instruction::UpdateMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
        Instruction::WriteToMemory(8, 11),
        Instruction::WriteToMemory(7, 101),
        Instruction::WriteToMemory(8, 0),
    ];
    let mut memory = Memory::new();
    memory.execute_version1(&instructions);
    let sum = memory.sum();
    assert_eq!(sum, 165)
}

#[test]
fn get_masked_addresses_should_work_for_example1() {
    let address = 42u64;
    let mask = "000000000000000000000000000000X1001X";
    let mut expected = Vec::new();
    expected.push(58u64);
    expected.push(59u64);
    expected.push(26u64);
    expected.push(27u64);
    assert_eq!(Memory::get_masked_addresses(address, mask), expected);
}

#[test]
fn flip_bit_should_flip_example1() {
    let number = 0b0000_0000_0000_0000_0000_0000_0000_0010_1010u64;
    let flip_position = 5;
    let expected = 0b0000_0000_0000_0000_0000_0000_0000_0000_1010u64;
    assert_eq!(Memory::flip_bit(number, flip_position), expected);
}

#[test]
fn flip_bit_should_flip_example2() {
    let number = 0b0000_0000_0000_0000_0000_0000_0000_0010_1010u64;
    let flip_position = 0;
    let expected = 0b0000_0000_0000_0000_0000_0000_0000_0010_1011u64;
    assert_eq!(Memory::flip_bit(number, flip_position), expected);
}

pub struct Memory {
    memory: HashMap<u64, u64>,
}

impl Memory {
    const BIT_SIZE: usize = 36;

    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    fn update_version1(&mut self, address: u64, value: u64, mask: Mask) {
        let masked_value = Self::get_masked_value(value, mask);
        self.memory.insert(address, masked_value);
    }

    fn update_version2(&mut self, initial_address: u64, value: u64, mask: &str) {
        let addresses = Self::get_masked_addresses(initial_address, mask);
        for address in addresses {
            self.memory.insert(address, value);
        }
    }

    fn get_masked_value(value: u64, mask: Mask) -> u64 {
        (value | mask.1) & mask.0
    }

    fn get_mask_ones(mask: &str) -> u64 {
        let mask_ones = mask.replace('X', "0");
        u64::from_str_radix(&*mask_ones, 2).unwrap()
    }

    fn get_mask_zeros(mask: &str) -> u64 {
        let mask_zeros = mask.replace('X', "1");
        u64::from_str_radix(&*mask_zeros, 2).unwrap()
    }

    fn get_mask_version1(mask: &str) -> Mask {
        (Self::get_mask_zeros(mask), Self::get_mask_ones(mask))
    }

    /**
     * Lower positions are on the right, higher position at the left.
     */
    fn flip_bit(number: u64, position: usize) -> u64 {
        number ^ 2u64.pow(position as u32)
    }

    fn get_masked_addresses(address: u64, mask: &str) -> Vec<u64> {
        let mask_ones = Self::get_mask_ones(mask);
        let mut addresses = Vec::new();
        addresses.push(address | mask_ones);
        let floating_positions: Vec<usize> = mask
            .match_indices('X')
            .map(|(i, _)| Memory::BIT_SIZE - i - 1)
            .collect();
        for floating_position in floating_positions {
            addresses = addresses
                .iter()
                .flat_map(|n| vec![*n, Self::flip_bit(*n, floating_position)])
                .collect();
        }

        addresses
    }

    pub fn execute_version1(&mut self, instructions: &Vec<Instruction>) {
        let mut mask = (0, 0);
        for instruction in instructions {
            match instruction {
                Instruction::UpdateMask(updated_mask) => {
                    mask = Self::get_mask_version1(updated_mask);
                }
                Instruction::WriteToMemory(address, value) => {
                    self.update_version1(*address, *value, mask);
                }
            }
        }
    }

    pub fn execute_version2(&mut self, instructions: &Vec<Instruction>) {
        let mut mask = "";
        for instruction in instructions {
            match instruction {
                Instruction::UpdateMask(updated_mask) => {
                    mask = updated_mask;
                }
                Instruction::WriteToMemory(address, value) => {
                    self.update_version2(*address, *value, mask);
                }
            }
        }
    }

    pub fn sum(self) -> u64 {
        self.memory.values().fold(0, |a, n| a + n)
    }
}
