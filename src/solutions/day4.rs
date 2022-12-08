use crate::Solution;

pub struct Day4 {}

impl Solution for Day4 {
    fn solve_part_1(&self, input: &String) -> i32 {
        input
            .split("\n")
            .filter(|s| line_fully_contains(s))
            .fold(0, |acc, _| acc + 1)
    }

    fn solve_part_2(&self, input: &String) -> i32 {
        input
            .split("\n")
            .filter(|s| line_overlaps(s))
            .fold(0, |acc, _| acc + 1)
    }
}

fn line_fully_contains(s: &str) -> bool {
    let (ass1, ass2) = string_to_assignments(s);
    return either_contained(ass1, ass2);
}

fn string_to_assignments(s: &str) -> (Assignment, Assignment) {
    let ss = s.split(",").collect::<Vec<&str>>();
    return (string_to_assignment(ss[0]), string_to_assignment(ss[1]));
}

fn string_to_assignment(s: &str) -> Assignment {
    let ss = s.split("-").collect::<Vec<&str>>();
    let start = ss[0].parse::<i32>().unwrap();
    let end = ss[1].parse::<i32>().unwrap();
    return Assignment {
        start: start,
        offset: end - start,
    };
}

struct Assignment {
    start: i32,
    offset: i32,
}

fn either_contained(ass1: Assignment, ass2: Assignment) -> bool {
    if (ass1.start <= ass2.start) && (ass1.start + ass1.offset >= ass2.start + ass2.offset) {
        return true;
    }
    if (ass1.start >= ass2.start) && (ass2.start + ass2.offset >= ass1.start + ass1.offset) {
        return true;
    }
    return false;
}

fn line_overlaps(s: &str) -> bool {
    let (ass1, ass2) = string_to_assignments(s);

    return overlap(ass1, ass2);
}

fn overlap(ass1: Assignment, ass2: Assignment) -> bool {
    let (x1, x2, y1, y2) = (
        ass1.start,
        ass1.start + ass1.offset,
        ass2.start,
        ass2.start + ass2.offset,
    );

    if x1 <= y1 && x2 >= y1 {
        return true;
    }
    if y1 <= x1 && y2 >= x1 {
        return true;
    }
    if x1 <= y2 && x2 >= y2 {
        return true;
    }
    if y1 <= x2 && y2 >= x2 {
        return true;
    }

    return false;
}
