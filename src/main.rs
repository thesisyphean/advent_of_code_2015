use std::fs::File;
use std::io::{prelude::*, Error};

mod solutions;

use solutions::day_1;
use solutions::day_2;

fn main() {
    match get_input("inputs/day_2.txt") {
        Ok(input) => println!("{}", day_2::solve_part_2(&input)),
        Err(error) => eprintln!("{}", error),
    }
}

fn get_input(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}
