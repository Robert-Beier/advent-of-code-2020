use lib::read_input;

type Sequence = Vec<usize>;

#[derive(Debug)]
enum Rule {
    Char(char),
    PossibleSequences(Vec<Sequence>),
}

#[cfg(test)]
mod tests {
    use crate::{is_valid, Rule};

    fn get_example_rules() -> Vec<Rule> {
        vec![
            Rule::PossibleSequences(vec![vec![4, 1, 5]]),
            Rule::PossibleSequences(vec![vec![2, 3], vec![3, 2]]),
            Rule::PossibleSequences(vec![vec![4, 4], vec![5, 5]]),
            Rule::PossibleSequences(vec![vec![4, 5], vec![5, 4]]),
            Rule::Char('a'),
            Rule::Char('b'),
        ]
    }

    #[test]
    fn is_valid_should_return_true_for_matching_single_character() {
        let rules = vec![Rule::Char('a')];
        assert_eq!(is_valid("a", 0, &rules), true)
    }

    #[test]
    fn is_valid_should_return_false_for_not_matching_single_character() {
        let rules = vec![Rule::Char('a')];
        assert_eq!(is_valid("b", 0, &rules), false)
    }

    #[test]
    fn is_valid_should_return_true_for_matching_sequence() {
        let rules = vec![
            Rule::PossibleSequences(vec![vec![1, 2]]),
            Rule::Char('a'),
            Rule::Char('b'),
        ];
        assert_eq!(is_valid("ab", 0, &rules), true)
    }

    #[test]
    fn is_valid_should_return_false_for_not_matching_sequence() {
        let rules = vec![
            Rule::PossibleSequences(vec![vec![1, 2]]),
            Rule::Char('a'),
            Rule::Char('b'),
        ];
        assert_eq!(is_valid("ba", 0, &rules), false)
    }
}

fn get_next_combination(mut combination: Vec<usize>, length: usize) -> Option<Vec<usize>> {
    // TODO adapt to return start index positions instead of lengths
    for i in (0..combination.len()).rev() {
        if (1 + combination.split_at(i).1.iter().sum::<usize>()) < length {
            return Some(combination);
        }
        combination[i] = 1;
    }
    None
}

fn is_valid(message: &str, rule_index: usize, rules: &Vec<Rule>) -> bool {
    match rules.get(rule_index).unwrap() {
        Rule::Char(c) => c.to_string() == message,
        Rule::PossibleSequences(possible_sequences) => {
            let possible_sequence = possible_sequences.get(0).unwrap();
            if possible_sequence.len() >= message.len() {
                return false;
            }
            let mut combination: Vec<usize> = (0..possible_sequence.len()).into_iter().collect();
            loop {
                // TODO cut message in slices and validate those against the rules
                // if possible_sequence.iter().all(|&r| is_valid(message[])) {
                //     return true;
                // }
                let next_combination = get_next_combination(combination, message.len());
                match next_combination {
                    None => return false,
                    Some(c) => combination = c,
                }
            }
        }
    }
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

fn main() {
    let input = read_input();
    let input: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(input.get(0).unwrap());
    println!("{:?}", rules);
}
