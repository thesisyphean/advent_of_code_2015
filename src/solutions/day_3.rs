use std::ops::AddAssign;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl AddAssign<Move> for Position {
    fn add_assign(&mut self, rhs: Move) {
        match rhs.direction {
            Direction::Horizontal => self.x += rhs.amount,
            Direction::Vertical => self.y += rhs.amount,
        }
    }
}

struct Move {
    direction: Direction,
    amount: i32,
}

enum Direction {
    Horizontal,
    Vertical,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Move { direction: Direction::Vertical, amount: 1 },
            '>' => Move { direction: Direction::Horizontal, amount: 1 },
            'v' => Move { direction: Direction::Vertical, amount: -1 },
            '<' => Move { direction: Direction::Horizontal, amount: -1 },
            _ => return Err(()),
        })
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    // It turns out that you aren't supposed to optimise the moves
    //
    // let mut moves = vec![];

    // let mut current_move = Move {
    //     direction: Direction::Horizontal,
    //     amount: 0,
    // };

    // for smove in input.chars().filter_map(|c| Move::try_from(c).ok()) {
    //     if smove.direction == current_move.direction {
    //         current_move.amount += smove.amount;
    //     } else {
    //         moves.push(current_move);
    //         current_move = smove;
    //     }
    // }

    // moves.push(current_move);
    // moves

    input.chars()
        .filter_map(|c| Move::try_from(c).ok())
        .collect()
}

pub fn solve_part_1(input: &str) -> u32 {
    let moves = parse_moves(input);

    let mut visited_positions = HashSet::new();
    let mut position = Position::new(0, 0);
    visited_positions.insert(position);

    for smove in moves {
        position += smove;
        visited_positions.insert(position);
    }

    visited_positions.len() as u32
}

pub fn solve_part_2(input: &str) -> u32 {
    let moves = parse_moves(input);

    let mut visited_positions = HashSet::new();
    let mut positions = [Position::new(0, 0), Position::new(0, 0)];
    visited_positions.insert(positions[0]);

    for (i, smove) in moves.into_iter().enumerate() {
        positions[i % 2] += smove;
        visited_positions.insert(positions[i % 2]);
    }

    visited_positions.len() as u32
}
