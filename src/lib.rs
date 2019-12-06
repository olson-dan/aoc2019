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

#[aoc_generator(day4)]
pub fn input_generator_day4(_input: &str) -> Vec<usize> {
    Vec::new()
}

fn day4(val: i32, part2: bool) -> bool {
    let mut digits = Vec::new();
    let mut x = val;
    for _ in 0..6 {
        let digit = x % 10;
        x = x / 10;
        digits.push(digit);
    }
    let mut conseq = [0; 10];
    for x in digits.windows(2) {
        if x[0] < x[1] {
            return false;
        }
        if x[0] == x[1] {
            conseq[x[0] as usize] += 1;
        }
    }
    if part2 {
        conseq.iter().any(|x| *x == 1)
    } else {
        conseq.iter().any(|x| *x > 0)
    }
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(_input: &[usize]) -> usize {
    let mut count = 0;
    for x in 382345..843167 {
        if day4(x, false) {
            count += 1;
        }
    }
    count
}

#[test]
fn test_day4() {
    assert_eq!(day4(111111, false), true);
    assert_eq!(day4(223450, false), false);
    assert_eq!(day4(123789, false), false);
    assert_eq!(day4(122345, false), true);
    assert_eq!(day4(112233, true), true);
    assert_eq!(day4(123444, true), false);
    assert_eq!(day4(111122, true), true);
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(_input: &[usize]) -> usize {
    let mut count = 0;
    for x in 382345..843167 {
        if day4(x, true) {
            count += 1;
        }
    }
    count
}

/*
fn fetch1(v: &[isize], ip: usize, inst: isize) -> isize {
    let x = v[ip + 1];
    let x = match (inst / 100) % 10 {
        0 => v[x as usize],
        1 => x,
        _ => unimplemented!(),
    };
    x
}

fn fetch2(v: &[isize], ip: usize, inst: isize) -> (isize, isize) {
    let x = fetch1(v, ip, inst);
    let y = fetch1(v, ip + 1, inst / 10);
    (x, y)
}

fn fetch3(v: &[isize], ip: usize, inst: isize) -> (isize, isize, isize) {
    let x = fetch1(v, ip, inst);
    let y = fetch1(v, ip + 1, inst / 10);
    let z = fetch1(v, ip + 2, inst / 100);
    (x, y, z)
}
*/

fn day5(input: &[isize], inp: isize) -> Vec<isize> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut v = input.to_vec();
    loop {
        match v[i] {
            99 => break,
            a if a % 100 == 1 => {
                let (x, y, z) = (v[i + 1], v[i + 2], v[i + 3]);
                let mode0 = (a / 100) % 10;
                let mode1 = (a / 1000) % 10;
                let x = if mode0 == 1 { x } else { v[x as usize] };
                let y = if mode1 == 1 { y } else { v[y as usize] };
                v[z as usize] = x + y;
                i += 4;
            }
            a if a % 100 == 2 => {
                let (x, y, z) = (v[i + 1], v[i + 2], v[i + 3]);
                let mode0 = (a / 100) % 10;
                let mode1 = (a / 1000) % 10;
                let x = if mode0 == 1 { x } else { v[x as usize] };
                let y = if mode1 == 1 { y } else { v[y as usize] };
                v[z as usize] = x * y;
                i += 4;
            }
            a if a % 100 == 3 => {
                let mode0 = (a / 100) % 10;
                assert_eq!(mode0, 0);
                let x = v[i + 1];
                v[x as usize] = inp;
                i += 2;
            }
            a if a % 100 == 4 => {
                let mode0 = (a / 100) % 10;
                let x = v[i + 1];
                if mode0 == 1 {
                    out.push(x);
                } else {
                    out.push(v[x as usize]);
                }
                i += 2;
            }
            a if a % 100 == 5 => {
                let (x, y) = (v[i + 1], v[i + 2]);
                let mode0 = (a / 100) % 10;
                let mode1 = (a / 1000) % 10;
                let x = if mode0 == 1 { x } else { v[x as usize] };
                let y = if mode1 == 1 { y } else { v[y as usize] };
                if x != 0 {
                    i = y as usize;
                } else {
                    i += 3;
                }
            }
            a if a % 100 == 6 => {
                let (x, y) = (v[i + 1], v[i + 2]);
                let mode0 = (a / 100) % 10;
                let mode1 = (a / 1000) % 10;
                let x = if mode0 == 1 { x } else { v[x as usize] };
                let y = if mode1 == 1 { y } else { v[y as usize] };
                if x == 0 {
                    i = y as usize;
                } else {
                    i += 3;
                }
            }
            a if a % 100 == 7 => {
                let (x, y, z) = (v[i + 1], v[i + 2], v[i + 3]);
                let mode0 = (a / 100) % 10;
                let mode1 = (a / 1000) % 10;
                let x = if mode0 == 1 { x } else { v[x as usize] };
                let y = if mode1 == 1 { y } else { v[y as usize] };
                if x < y {
                    v[z as usize] = 1;
                } else {
                    v[z as usize] = 0;
                }
                i += 4;
            }
            a if a % 100 == 8 => {
                let (x, y, z) = (v[i + 1], v[i + 2], v[i + 3]);
                let mode0 = (a / 100) % 10;
                let mode1 = (a / 1000) % 10;
                let x = if mode0 == 1 { x } else { v[x as usize] };
                let y = if mode1 == 1 { y } else { v[y as usize] };
                if x == y {
                    v[z as usize] = 1;
                } else {
                    v[z as usize] = 0;
                }
                i += 4;
            }
            _ => unimplemented!(),
        }
    }
    out
}
#[aoc_generator(day5)]
pub fn input_generator_day5(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &[isize]) -> isize {
    let ret = day5(input, 1);
    *ret.last().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &[isize]) -> isize {
    let ret = day5(input, 5);
    *ret.last().unwrap()
}

#[aoc_generator(day6)]
pub fn input_generator_day6(input: &str) -> Vec<(String, String)> {
    let mut y = Vec::new();
    for l in input.lines() {
        let x: Vec<&str> = l.splitn(2, ')').collect();
        y.push((x[0].to_string(), x[1].to_string()));
    }
    y
}

fn count_orbits(map: &[(String, String)], this: &str) -> usize {
    let mut count = 0;
    for (x, y) in map {
        if y == this {
            count += 1 + count_orbits(map, x);
        }
    }
    count
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &[(String, String)]) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    for (_, y) in input {
        set.insert(y.clone());
    }
    let mut orbits = 0;
    for k in &set {
        orbits += count_orbits(input, k);
    }
    orbits
}

fn chain_orbits(map: &[(String, String)], this: &str) -> Vec<String> {
    let mut chain = Vec::new();
    for (x, y) in map {
        if y == this {
            chain.push(y.to_string());
            chain.extend(chain_orbits(map, x));
        }
    }
    chain
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &[(String, String)]) -> usize {
    let you = chain_orbits(input, "YOU");
    let san = chain_orbits(input, "SAN");
    let mut set = HashSet::new();
    for x in you.iter().chain(san.iter()) {
        set.insert(x);
    }
    let mut answer = usize::max_value();
    for k in set {
        if let Some((i, _)) = you.iter().enumerate().find(|(_, b)| *b == k) {
            if let Some((j, _)) = san.iter().enumerate().find(|(_, b)| *b == k) {
                answer = std::cmp::min(answer, i + j);
            }
        }
    }
    answer - 2
}

#[test]
fn test_day6() {
    assert_eq!(
        42,
        solve_day6_part1(&input_generator_day6(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
        ))
    );
    assert_eq!(
        4,
        solve_day6_part2(&input_generator_day6(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
        ))
    );
}

aoc_lib! { year = 2019 }
