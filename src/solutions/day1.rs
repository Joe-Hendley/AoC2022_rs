use crate::Solution;

#[derive(Copy, Clone)]
pub struct Day1 {}

impl Solution for Day1 {
    fn solve_part_1(&self, input: &String) -> i32 {
        let mut max = 0;
        let mut calories = 0;

        let lines = input.split("\n");

        for line in lines {
            if line == "" {
                max = compare_one(max, calories);
                calories = 0
            } else {
                calories += line.parse::<i32>().unwrap();
            }
        }

        max = compare_one(max, calories);
        return max;
    }

    fn solve_part_2(&self, input: &String) -> i32 {
        let mut maximums = vec![0; 3];
        let mut calories = 0;

        let lines = input.split("\n");

        for line in lines {
            if line == "" {
                maximums = compare_multiple(maximums, calories);
                calories = 0
            } else {
                calories += line.parse::<i32>().unwrap();
            }
        }
        maximums = compare_multiple(maximums, calories);
        return maximums.iter().sum();
    }
}

fn compare_one(max: i32, calories: i32) -> i32 {
    if calories > max {
        return calories;
    }
    return max;
}

fn compare_multiple(mut maximums: Vec<i32>, num: i32) -> Vec<i32> {
    for i in 0..maximums.len() {
        if num > maximums[i] {
            maximums[i] = num;
            maximums.sort(); // maximums is small so this isn't too bad
            break;
        }
    }
    return maximums;
}
