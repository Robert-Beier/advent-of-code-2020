#[test]
fn find_two_summands_should_solve_example_1() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_two_summands(&input, 2020), Some((1721, 299)));
}

#[test]
fn find_two_summands_should_work_with_inputs_smaller_than_sum() {
    let input = vec![1721, 3000, 366, 299, 675, 1456];
    assert_eq!(find_two_summands(&input, 2020), Some((1721, 299)));
}

pub fn find_two_summands(potential_summands: &[usize], sum: usize) -> Option<(usize, usize)> {
    for (index, a) in potential_summands.iter().enumerate() {
        let b = potential_summands
            .iter()
            .enumerate()
            .find(|(i, x)| *i != index && sum >= *a && **x == (sum - a));
        if let Some((_, b)) = b {
            return Some((*a, *b));
        }
    }
    None
}
