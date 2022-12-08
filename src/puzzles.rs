use std::error::Error;

use crate::solutions::{day1, day2, day3, day4};
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
        3 => Ok(three()),
        4 => Ok(four()),
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

fn three() -> Puzzle {
    return Puzzle {
        id: 3,
        part_one_example: 157,
        part_two_example: 70,
        solution: Box::new(day3::Day3 {}),
    };
}

fn four() -> Puzzle {
    return Puzzle {
        id: 4,
        part_one_example: 2,
        part_two_example: 4,
        solution: Box::new(day4::Day4 {}),
    };
}
