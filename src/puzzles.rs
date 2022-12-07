use std::error::Error;

use crate::solutions::{day1, day2};
use crate::Solution;

pub struct Puzzle {
    pub id: i32,
    pub part_one_example: i32,
    pub part_two_example: i32,
    pub solution: Box<dyn Solution>,
}

pub fn get_puzzle(id: i32) -> Result<Puzzle, Box<dyn Error>> {
    return match id {
        1 => Ok(one()),
        2 => Ok(two()),
        _ => Err("invalid puzzle".into()),
    };
}

fn one() -> Puzzle {
    return Puzzle {
        id: 1,
        part_one_example: 24000,
        part_two_example: 45000,
        solution: Box::new(day1::Day1 {}),
    };
}

fn two() -> Puzzle {
    return Puzzle {
        id: 2,
        part_one_example: 15,
        part_two_example: 12,
        solution: Box::new(day2::Day2 {}),
    };
}
