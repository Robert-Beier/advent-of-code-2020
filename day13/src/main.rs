use lib::{read_input, solve};

#[test]
fn get_wait_time_should_work_for_min_time() {
    assert_eq!(get_wait_time(20, 10), 0);
}

#[test]
fn get_wait_time_should_work_for_max_time() {
    assert_eq!(get_wait_time(21, 10), 9);
}

fn get_wait_time(earliest_start: u64, bus: u64) -> u64 {
    bus - ((earliest_start - 1) % bus) - 1
}

#[test]
fn get_earliest_bus_should_work_for_example1() {
    let earliest_start = 939u64;
    let buses = vec![7, 13, 59, 31, 19];
    assert_eq!(get_earliest_bus(earliest_start, &buses), 59);
}

fn get_earliest_bus(earliest_start: u64, buses: &Vec<u64>) -> u64 {
    *buses
        .iter()
        .min_by_key(|&&b| get_wait_time(earliest_start, b))
        .unwrap()
}

fn parse_input(input: &str) -> (u64, Vec<u64>) {
    let lines: Vec<&str> = input.lines().collect();
    (
        lines.get(0).unwrap().parse().unwrap(),
        lines
            .get(1)
            .unwrap()
            .split(',')
            .filter(|&s| s != "x")
            .map(|n| n.parse().unwrap())
            .collect(),
    )
}

fn parse_input2(input: &str) -> Vec<(u64, u64)> {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, n)| (i as u64, n.parse().unwrap()))
        .collect()
}

fn part_one(earliest_start: u64, buses: &Vec<u64>) {
    solve("Part one", || {
        let earliest_bus = get_earliest_bus(earliest_start, buses);
        get_wait_time(earliest_start, earliest_bus) * earliest_bus
    });
}

fn main() {
    let input = read_input();
    let (earliest_start, buses) = parse_input(&input);
    part_one(earliest_start, &buses);
    let buses = parse_input2(&input);
}
