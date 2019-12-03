use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

#[aoc_generator(day1)]
pub fn input_generator_day1(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &[i64]) -> i64 {
    input.iter().map(|x| (x / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i64]) -> i64 {
    let mut ret = 0;
    for x in input {
        let mut x = *x;
        loop {
            x = (x / 3) - 2;
            if x <= 0 {
                break;
            }
            ret += x;
        }
    }
    ret
}

#[test]
fn test_day1() {
    assert_eq!(solve_day1_part2(&[100756]), 50346);
}

#[aoc_generator(day2)]
pub fn input_generator_day2(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn day2(input: &[usize], a: usize, b: usize) -> usize {
    let mut i = 0usize;
    let mut v = input.to_vec();
    v[1] = a;
    v[2] = b;
    loop {
        match v[i] {
            99 => break,
            1 => {
                let (x, y, z) = (v[i + 1], v[i + 2], v[i + 3]);
                v[z] = v[x] + v[y]
            }
            2 => {
                let (x, y, z) = (v[i + 1], v[i + 2], v[i + 3]);
                v[z] = v[x] * v[y]
            }
            _ => unimplemented!(),
        }
        i += 4;
    }
    v[0]
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[usize]) -> usize {
    day2(input, 12, 2)
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[usize]) -> usize {
    for x in 0..input.len() {
        for y in 0..input.len() {
            if day2(input, x, y) == 19690720 {
                return 100 * x + y;
            }
        }
    }
    0
}

use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn input_generator_day3(input: &str) -> (Vec<String>, Vec<String>) {
    let x = input.lines().collect::<Vec<_>>();
    (
        x[0].split(',').map(|y| y.to_string()).collect(),
        x[1].split(',').map(|y| y.to_string()).collect(),
    )
}

fn day3_fill_points(directions: &[String]) -> HashSet<(i64, i64)> {
    let mut ret = HashSet::new();
    let mut x = 0i64;
    let mut y = 0i64;
    for dir in directions {
        let mut dir = dir.clone();
        let amount = dir.split_off(1).parse::<i64>().unwrap();
        match dir.as_str() {
            "R" => {
                for _ in 0..amount {
                    x += 1;
                    ret.insert((x, y));
                }
            }
            "L" => {
                for _ in 0..amount {
                    x -= 1;
                    ret.insert((x, y));
                }
            }
            "U" => {
                for _ in 0..amount {
                    y -= 1;
                    ret.insert((x, y));
                }
            }
            "D" => {
                for _ in 0..amount {
                    y += 1;
                    ret.insert((x, y));
                }
            }
            _ => unimplemented!(),
        }
    }
    ret
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &(Vec<String>, Vec<String>)) -> i64 {
    let one = day3_fill_points(&input.0);
    let two = day3_fill_points(&input.1);
    let mut lowest = i64::max_value();
    for point in one.intersection(&two) {
        let (x, y) = *point;
        lowest = std::cmp::min(x.abs() + y.abs(), lowest);
    }
    lowest
}

fn day3_count_steps(directions: &[String], target: &(i64, i64)) -> i64 {
    let mut x = 0i64;
    let mut y = 0i64;
    let mut steps = 0i64;
    for dir in directions {
        let mut dir = dir.clone();
        let amount = dir.split_off(1).parse::<i64>().unwrap();
        match dir.as_str() {
            "R" => {
                for _ in 0..amount {
                    x += 1;
                    steps += 1;
                    if (x, y) == *target {
                        return steps;
                    }
                }
            }
            "L" => {
                for _ in 0..amount {
                    x -= 1;
                    steps += 1;
                    if (x, y) == *target {
                        return steps;
                    }
                }
            }
            "U" => {
                for _ in 0..amount {
                    y -= 1;
                    steps += 1;
                    if (x, y) == *target {
                        return steps;
                    }
                }
            }
            "D" => {
                for _ in 0..amount {
                    y += 1;
                    steps += 1;
                    if (x, y) == *target {
                        return steps;
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
    unreachable!();
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &(Vec<String>, Vec<String>)) -> i64 {
    let one = day3_fill_points(&input.0);
    let two = day3_fill_points(&input.1);
    let mut lowest = i64::max_value();
    for point in one.intersection(&two) {
        let x = day3_count_steps(&input.0, point);
        let y = day3_count_steps(&input.1, point);
        lowest = std::cmp::min(x + y, lowest);
    }
    lowest
}

#[test]
fn day3_test() {
    let inp =
        input_generator_day3("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(solve_day3_part2(&inp), 610);
}

aoc_lib! { year = 2019 }
