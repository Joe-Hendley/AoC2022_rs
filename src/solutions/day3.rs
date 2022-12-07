use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day3 {}

impl Solution for Day3 {
    fn solve_part_1(&self, input: &String) -> i32 {
        return input.split("\n").map(|l| solve_line(l)).sum();
    }

    fn solve_part_2(&self, input: &String) -> i32 {
        let mut lines = input.split("\n").peekable();
        let mut sum = 0;
        while lines.peek().is_some() {
            let group = lines.by_ref().take(3).collect::<Vec<&str>>();
            sum += char_to_priority(common_in_group(group))
        }
        return sum;
    }
}

fn solve_line(s: &str) -> i32 {
    // in hindsight hashset would have been appropriate, but I can't be bothered fixing it
    let (first, second) = s.split_at(s.len() / 2);
    let mut items: HashMap<char, i32> = HashMap::new();
    // fill in the map of items in the row, might as well keep track of the quantity for part 2 // turns out this didn't matter
    for item in first.chars() {
        items.entry(item).and_modify(|x| *x += 1).or_insert(1);
    }

    for item in second.chars() {
        match items.entry(item) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return char_to_priority(item);
            }
            std::collections::hash_map::Entry::Vacant(_) => (),
        };
    }

    return 0;
}

fn char_to_priority(c: char) -> i32 {
    let num = c as i32;
    match num {
        n if n >= 97 && n <= 122 => return n - 96, // a is 97, z is 122, so offset of 96 for a range of 1-26
        n if n >= 65 && n <= 90 => return n - 38, // A is 65, Z is 90, so offset of 38 for a range of 27-52
        _ => {
            return 0;
        }
    }
}

fn common_in_group(lines: Vec<&str>) -> char {
    let items_1 = HashSet::<char>::from_iter(lines[0].chars());
    let items_2 = HashSet::<char>::from_iter(lines[1].chars());
    let items_3 = HashSet::<char>::from_iter(lines[2].chars());

    let i = &(&items_1 & &items_2) & &items_3;
    match i.iter().next() {
        Some(c) => return *c,
        None => {
            return '0';
        }
    }
}
