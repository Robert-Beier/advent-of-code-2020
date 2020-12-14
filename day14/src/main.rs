enum Instruction {
    UpdateMask(String),
    WriteToMemory(u64),
}

#[test]
fn get_masked_value_should_work_for_example_1() {
    let value = 11u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(get_masked_value(value, mask), 73);
}

#[test]
fn get_masked_value_should_work_for_example_2() {
    let value = 101u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(get_masked_value(value, mask), 101);
}

#[test]
fn get_masked_value_should_work_for_example_3() {
    let value = 0u64;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(get_masked_value(value, mask), 64);
}

fn get_masked_value(value: u64, mask: &str) -> u64 {
    let mask_ones = mask.replace('X', "0");
    let mask_zeroes = mask.replace('X', "1");
    let mask_ones = u64::from_str_radix(&*mask_ones, 2).unwrap();
    let mask_zeroes = u64::from_str_radix(&*mask_zeroes, 2).unwrap();
    (value | mask_ones) & mask_zeroes
}

fn main() {
    println!("Hello, world!");
}
