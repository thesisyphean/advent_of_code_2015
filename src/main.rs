use std::fs::File;
use std::io::{self, prelude::*};
use std::error::Error;

mod solutions;

use solutions::{
    day_1, day_2, day_3, day_4, day_5,
    day_6, day_7, day_8, day_9, day_10,
    day_11, day_12, day_13, day_14, day_15,
    day_16, day_17, day_18, day_19, day_20,
    day_21, day_22, day_23, day_24, day_25,
};

fn main() {
    let (problem, part) = parse_input()
        .expect("Error");

    match solve(problem, part).expect("Error") {
        Some(solution) => println!("Solution: {solution}"),
        _ => println!("Problem {problem} part {part} has not been solved yet."),
    }
}

fn parse_input() -> Result<(u32, u32), Box<dyn Error>> {
    let mut input_text = String::new();

    println!("Problem to solve: ");
    io::stdin().read_line(&mut input_text)?;
    let problem = input_text.trim().parse()?;

    println!("Part to solve: ");
    input_text.clear();
    io::stdin().read_line(&mut input_text)?;
    let part = input_text.trim().parse()?;

    Ok((problem, part))
}

fn solve(problem: u32, part: u32) -> Result<Option<u32>, io::Error> {
    let input = get_input(&format!("inputs/day_{problem}.txt"))?;

    Ok(Some(match (problem, part) {
        (1, 1) => day_1::solve_part_1(&input),
        (1, 2) => day_1::solve_part_2(&input),
        (2, 1) => day_2::solve_part_1(&input),
        (2, 2) => day_2::solve_part_2(&input),
        (3, 1) => day_3::solve_part_1(&input),
        (3, 2) => day_3::solve_part_2(&input),
        // (4, 1) => day_4::solve_part_1(&input),
        // (4, 2) => day_4::solve_part_2(&input),
        (5, 1) => day_5::solve_part_1(&input),
        (5, 2) => day_5::solve_part_2(&input),
        (6, 1) => day_6::solve_part_1(&input),
        (6, 2) => day_6::solve_part_2(&input),
        // (7, 1) => day_7::solve_part_1(&input),
        // (7, 2) => day_7::solve_part_2(&input),
        (8, 1) => day_8::solve_part_1(&input),
        (8, 2) => day_8::solve_part_2(&input),
        // (9, 1) => day_9::solve_part_1(&input),
        // (9, 2) => day_9::solve_part_2(&input),
        (10, 1) => day_10::solve_part_1(&input),
        (10, 2) => day_10::solve_part_2(&input),
        // (11, 1) => day_11::solve_part_1(&input),
        // (11, 2) => day_11::solve_part_2(&input),
        (12, 1) => day_12::solve_part_1(&input),
        // (12, 2) => day_12::solve_part_2(&input),
        // (13, 1) => day_13::solve_part_1(&input),
        // (13, 2) => day_13::solve_part_2(&input),
        // (14, 1) => day_14::solve_part_1(&input),
        // (14, 2) => day_14::solve_part_2(&input),
        // (15, 1) => day_15::solve_part_1(&input),
        // (15, 2) => day_15::solve_part_2(&input),
        // (16, 1) => day_16::solve_part_1(&input),
        // (16, 2) => day_16::solve_part_2(&input),
        // (17, 1) => day_17::solve_part_1(&input),
        // (17, 2) => day_17::solve_part_2(&input),
        // (18, 1) => day_18::solve_part_1(&input),
        // (18, 2) => day_18::solve_part_2(&input),
        // (19, 1) => day_19::solve_part_1(&input),
        // (19, 2) => day_19::solve_part_2(&input),
        // (20, 1) => day_20::solve_part_1(&input),
        // (20, 2) => day_20::solve_part_2(&input),
        // (21, 1) => day_21::solve_part_1(&input),
        // (21, 2) => day_21::solve_part_2(&input),
        // (22, 1) => day_22::solve_part_1(&input),
        // (22, 2) => day_22::solve_part_2(&input),
        // (23, 1) => day_23::solve_part_1(&input),
        // (23, 2) => day_23::solve_part_2(&input),
        // (24, 1) => day_24::solve_part_1(&input),
        // (24, 2) => day_24::solve_part_2(&input),
        // (25, 1) => day_25::solve_part_1(&input),
        // (25, 2) => day_25::solve_part_2(&input),
        _ => return Ok(None),
    }))
}

fn get_input(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}
