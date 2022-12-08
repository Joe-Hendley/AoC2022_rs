mod input;
mod puzzles;
mod solutions;

use std::{env, process};

const LATEST_ID: i32 = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut id = LATEST_ID;

    if args.len() > 1 {
        let id_string = &args[1];

        let _id_result = match id_string.parse::<i32>() {
            Ok(id_temp) => id = id_temp,
            Err(_error) => {
                println!(
                    "error parsing arg [{}] as i32, using most recent puzzle",
                    args[1]
                );
                id = LATEST_ID
            }
        };
    }

    let puzzle = match puzzles::get_puzzle(id) {
        Ok(puzzle) => puzzle,
        Err(error) => {
            println!("invalid puzzle {}: {}", id, error);
            process::exit(-1)
        }
    };

    let example_input = input::get_test_input(id);
    let input = input::get_input(id);

    let mut result_1 = 0;
    let mut result_2 = 0;

    if example_input.len() > 0 {
        result_1 = puzzle.solution.solve_part_1(&example_input);
        println!("part 1 example: {}", result_1);
        result_2 = puzzle.solution.solve_part_2(&example_input);
        println!("part 2 example: {}", result_2);
    }

    if input.len() > 0 {
        if result_1 == puzzle.part_one_example {
            println!("part 1 result: {}", puzzle.solution.solve_part_1(&input))
        } else {
            println!("part 1 got: {} want: {}", result_1, puzzle.part_one_example)
        }
        if result_2 == puzzle.part_two_example {
            println!("part 2 result: {}", puzzle.solution.solve_part_2(&input))
        } else {
            println!("part 2 got: {} want: {}", result_1, puzzle.part_two_example)
        }
    }
}

pub trait Solution {
    fn solve_part_1(&self, input: &String) -> i32;
    fn solve_part_2(&self, input: &String) -> i32;
}
