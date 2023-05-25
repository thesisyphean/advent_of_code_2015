fn parse_parentheses(input: &str) -> Vec<i32> {
    input.chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        }).collect()
}

pub fn solve_part_1(input: &str) -> i32 {
    parse_parentheses(input).into_iter()
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    parse_parentheses(input).into_iter()
        .scan(0, |state, p| {
            *state += p;

            if *state < 0 {
                None
            } else {
                Some(*state)
            }
        }).count() + 1
}
