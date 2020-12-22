use lib::{read_input, solve};
use regex::Regex;

type Sequence = Vec<usize>;

#[derive(Debug)]
enum Rule {
    Char(char),
    PossibleSequences(Vec<Sequence>),
}

fn compile_rule(rules: &Vec<Rule>, index: usize) -> String {
    return match rules.get(index).unwrap() {
        Rule::Char(c) => c.to_string(),
        Rule::PossibleSequences(possible_sequences) => {
            let possible_sequences: Vec<String> = possible_sequences
                .iter()
                .map(|s| {
                    s.iter()
                        .map(|&r| compile_rule(&rules, r))
                        .collect::<String>()
                })
                .collect();
            format!("(?:{})", possible_sequences.join("|"))
        }
    };
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
    let mut lines: Vec<(usize, Rule)> = input
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

fn parse_messages(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_one(rules: &Vec<Rule>, messages: &Vec<&str>) {
    solve("Part one", || {
        let pattern = compile_rule(&rules, 0);
        let pattern = format!("^{}$", pattern);
        let pattern = Regex::new(&*pattern).unwrap();
        messages.iter().filter(|m| pattern.is_match(m)).count()
    });
}

fn part_two(rules: &Vec<Rule>, messages: &Vec<&str>) {
    solve("Part two", || {
        let rule_42 = compile_rule(&rules, 42);
        let rule_31 = compile_rule(&rules, 31);
        let pattern = format!("^((?:{})+)((?:{})+)$", rule_42, rule_31);
        let pattern = Regex::new(&*pattern).unwrap();
        let rule_42 = format!("({})", rule_42);
        let rule_42 = Regex::new(&*rule_42).unwrap();
        let rule_31 = format!("({})", rule_31);
        let rule_31 = Regex::new(&*rule_31).unwrap();
        messages
            .iter()
            .filter(|m| {
                let captures = pattern.captures(m);
                match captures {
                    None => false,
                    Some(captures) => {
                        let rule_42_captures =
                            rule_42.find_iter(captures.get(1).unwrap().as_str()).count();
                        let rule_31_captures =
                            rule_31.find_iter(captures.get(2).unwrap().as_str()).count();
                        rule_42_captures > rule_31_captures
                    }
                }
            })
            .count()
    });
}

fn main() {
    let input = read_input();
    let input: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(input.get(0).unwrap());
    let messages = parse_messages(input.get(1).unwrap());
    part_one(&rules, &messages);
    part_two(&rules, &messages);
}
