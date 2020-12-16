use lib::{read_input, solve};

type Field = Vec<(u32, u32)>;

#[test]
fn get_invalid_values_should_work_for_example1() {
    let values = vec![7, 3, 47, 40, 4, 50, 55, 2, 20, 38, 6, 12];
    let fields = vec![
        vec![(1, 3), (5, 7)],
        vec![(6, 11), (33, 44)],
        vec![(13, 40), (45, 50)],
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
                .any(|f| f.iter().any(|(min, max)| v >= *min && v <= *max))
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
            l.split(": ")
                .last()
                .unwrap()
                .split(" or ")
                .map(|r| {
                    let values: Vec<u32> = r.split('-').map(|v| v.parse().unwrap()).collect();
                    (*values.get(0).unwrap(), *values.get(1).unwrap())
                })
                .collect()
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
