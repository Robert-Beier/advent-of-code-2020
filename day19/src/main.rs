use lib::read_input;

type Sequence = Vec<u32>;

#[derive(Debug)]
enum Rule {
    Char(char),
    PossibleSequences(Vec<Sequence>),
}

/** input should be something like `"a"`, `12 13 | 13 12` or `12 13`
*/
fn parse_rule(input: &str) -> Rule {
    return if input.contains('|') {
        Rule::PossibleSequences(
            input
                .split('|')
                .map(|s| s.trim().split(' ').map(|n| n.parse().unwrap()).collect())
                .collect(),
        )
    } else if input.contains('"') {
        Rule::Char(input.chars().filter(|&c| c != '"').next().unwrap())
    } else {
        Rule::PossibleSequences(vec![input.split(' ').map(|n| n.parse().unwrap()).collect()])
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
