use lib::read_input;

type Sequence = (u32, u32);

#[derive(Debug)]
enum Rule {
    Char(char),
    Single(u32),
    Sequence(Sequence),
    LogicalOrSequence(Sequence, Sequence),
    LogicalOrSingle(u32, u32),
}

/** input should be something like `"a"`, `12 13 | 13 12` or `12 13`
*/
fn parse_rule(input: &str) -> Rule {
    return if input.contains('|') {
        let numbers: Vec<u32> = input
            .split('|')
            .flat_map(|s| {
                s.trim()
                    .split(' ')
                    .map(|n| n.parse().expect("TEST"))
                    .collect::<Vec<u32>>()
            })
            .collect();
        match numbers.len() {
            2 => Rule::LogicalOrSingle(*numbers.get(0).unwrap(), *numbers.get(1).unwrap()),
            4 => Rule::LogicalOrSequence(
                (*numbers.get(0).unwrap(), *numbers.get(1).unwrap()),
                (*numbers.get(2).unwrap(), *numbers.get(3).unwrap()),
            ),
            _ => {
                panic!()
            }
        }
    } else if input.contains('"') {
        let character: char = input.chars().filter(|&c| c != '"').next().unwrap();
        Rule::Char(character)
    } else {
        let numbers: Vec<u32> = input.split(' ').map(|n| n.parse().unwrap()).collect();
        match numbers.len() {
            1 => Rule::Single(*numbers.get(0).unwrap()),
            2 => Rule::Sequence((*numbers.get(0).unwrap(), *numbers.get(1).unwrap())),
            _ => {
                panic!()
            }
        }
    };
}

/** input should be something like:
```
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
```
*/
fn parse_rules(input: &str) -> Vec<Rule> {
    let mut lines: Vec<(u32, Rule)> = input
        .lines()
        .map(|l| {
            let r: Vec<&str> = l.split(':').collect();
            (
                r.get(0).unwrap().parse().unwrap(),
                parse_rule(r.get(1).unwrap().trim()),
            )
        })
        .collect();
    lines.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));
    lines.into_iter().map(|(_, r)| r).collect()
}

fn main() {
    let input = read_input();
    let input: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(input.get(0).unwrap());
    println!("{:?}", rules);
}
