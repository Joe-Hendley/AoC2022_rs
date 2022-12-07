use std::error::Error;

use crate::Solution;

pub struct Day2 {}

#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum RPSResult {
    Win,
    Lose,
    Draw,
}

impl Solution for Day2 {
    fn solve_part_1(&self, input: &String) -> i32 {
        input.split("\n").map(|s| line_score(s)).sum()
    }

    fn solve_part_2(&self, input: &String) -> i32 {
        input.split("\n").map(|s| line_score_2(s)).sum()
    }
}

fn line_score(s: &str) -> i32 {
    let s: Vec<&str> = s.split_whitespace().collect();

    return score(s[0], s[1]);
}

fn str_to_rps(s: &str) -> Result<RPS, Box<dyn Error>> {
    return match s {
        "A" => Ok(RPS::Rock),
        "B" => Ok(RPS::Paper),
        "C" => Ok(RPS::Scissors),
        "X" => Ok(RPS::Rock),
        "Y" => Ok(RPS::Paper),
        "Z" => Ok(RPS::Scissors),
        _ => Err("not a valid throw".into()),
    };
}

fn did_i_win(them: RPS, me: RPS) -> RPSResult {
    let results = (them, me);
    return match results {
        (RPS::Rock, RPS::Paper) => RPSResult::Win,
        (RPS::Rock, RPS::Scissors) => RPSResult::Lose,
        (RPS::Paper, RPS::Rock) => RPSResult::Lose,
        (RPS::Paper, RPS::Scissors) => RPSResult::Win,
        (RPS::Scissors, RPS::Rock) => RPSResult::Win,
        (RPS::Scissors, RPS::Paper) => RPSResult::Lose,
        _ => RPSResult::Draw,
    };
}

fn score(them: &str, me: &str) -> i32 {
    let them_rps = str_to_rps(them).unwrap();
    let me_rps = str_to_rps(me).unwrap();

    let mut score = 0;
    score += rps_score(me_rps);
    score += result_score(did_i_win(them_rps, me_rps));
    return score;
}

fn rps_score(me: RPS) -> i32 {
    return match me {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
}

fn result_score(rslt: RPSResult) -> i32 {
    return match rslt {
        RPSResult::Win => 6,
        RPSResult::Draw => 3,
        RPSResult::Lose => 0,
    };
}

fn line_score_2(s: &str) -> i32 {
    let s: Vec<&str> = s.split_whitespace().collect();

    return score_2(s[0], s[1]);
}

fn score_2(them: &str, wanted_result: &str) -> i32 {
    let rslt = str_to_result(wanted_result).unwrap();
    let mut score = 0;
    score += result_score(rslt);
    let me = correct_throw(str_to_rps(them).unwrap(), rslt);
    score += rps_score(me);
    return score;
}

fn str_to_result(s: &str) -> Result<RPSResult, Box<dyn Error>> {
    return match s {
        "X" => Ok(RPSResult::Lose),
        "Y" => Ok(RPSResult::Draw),
        "Z" => Ok(RPSResult::Win),
        _ => Err("not a valid result".into()),
    };
}

fn correct_throw(them: RPS, wanted_result: RPSResult) -> RPS {
    return match (them, wanted_result) {
        (RPS::Rock, RPSResult::Win) => RPS::Paper,
        (RPS::Rock, RPSResult::Lose) => RPS::Scissors,
        (RPS::Paper, RPSResult::Win) => RPS::Scissors,
        (RPS::Paper, RPSResult::Lose) => RPS::Rock,
        (RPS::Scissors, RPSResult::Win) => RPS::Rock,
        (RPS::Scissors, RPSResult::Lose) => RPS::Paper,
        _ => them,
    };
}
