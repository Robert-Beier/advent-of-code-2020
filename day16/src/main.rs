use lib::{read_input, solve};

struct Field {
    min1: u32,
    max1: u32,
    min2: u32,
    max2: u32,
}

#[test]
fn get_invalid_values_should_work_for_example1() {
    let values = vec![7, 3, 47, 40, 4, 50, 55, 2, 20, 38, 6, 12];
    let fields = vec![
        Field {
            min1: 1,
            max1: 3,
            min2: 5,
            max2: 7,
        },
        Field {
            min1: 6,
            max1: 11,
            min2: 33,
            max2: 44,
        },
        Field {
            min1: 13,
            max1: 40,
            min2: 45,
            max2: 50,
        },
    ];
    assert_eq!(get_invalid_values(&values, &fields).iter().sum::<u32>(), 71);
}

fn get_invalid_values(values: &Vec<u32>, fields: &Vec<Field>) -> Vec<u32> {
    values
        .iter()
        .map(|v| *v)
        .filter(|&v| {
            !fields
                .iter()
                .any(|f| (v >= f.min1 && v <= f.max1) || (v >= f.min2 && v <= f.max2))
        })
        .collect()
}

fn parse_nearby_tickets(input: &String) -> Vec<Vec<u32>> {
    input
        .split("nearby tickets:")
        .last()
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.split(',').map(|v| v.parse().unwrap()).collect())
        .collect()
}

fn parse_fields(input: &String) -> Vec<Field> {
    input
        .split("your ticket:")
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            let values: Vec<u32> = l
                .split(": ")
                .last()
                .unwrap()
                .split(" or ")
                .flat_map(|r| r.split('-').map(|v| v.parse().unwrap()))
                .collect();
            Field {
                min1: *values.get(0).unwrap(),
                max1: *values.get(1).unwrap(),
                min2: *values.get(2).unwrap(),
                max2: *values.get(3).unwrap(),
            }
        })
        .collect()
}

fn part_one(tickets: &Vec<Vec<u32>>, fields: &Vec<Field>) {
    let values = tickets.iter().flatten().map(|&v| v).collect();
    solve("Part one", || {
        get_invalid_values(&values, &fields).iter().sum::<u32>()
    });
}

fn main() {
    let input = read_input();
    let tickets = parse_nearby_tickets(&input);
    let fields = parse_fields(&input);
    part_one(&tickets, &fields);
}
