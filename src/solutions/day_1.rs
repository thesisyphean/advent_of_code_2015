fn parse_parentheses(input: &str) -> Vec<i32> {
    input.chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        }).collect()
}

pub fn solve_part_1(input: &str) -> u32 {
    // this conversion seems suspicious,
    // but it is never negative
    parse_parentheses(input).into_iter()
        .sum::<i32>() as u32
}

pub fn solve_part_2(input: &str) -> u32 {
    (parse_parentheses(input).into_iter()
        .scan(0, |state, p| {
            *state += p;

            if *state < 0 {
                None
            } else {
                Some(*state)
            }
        }).count() + 1) as u32
}
