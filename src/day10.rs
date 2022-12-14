use std::fs;

use colored::*;

const DAY: u32 = 10;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample:\n".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input :\n".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn part1(input: &str) -> i64 {
    let mut x_values: Vec<i64> = vec![1, 1];

    for line in input.lines() {
        let op = &line[0..4];
        let current_x = *x_values.last().unwrap();
        match op {
            "noop" => {
                x_values.push(current_x);
            }
            "addx" => {
                let arg = line[5..].parse::<i64>().unwrap();
                x_values.push(current_x);
                x_values.push(current_x + arg);
            }
            _ => panic!("unexpected op"),
        }
    }
    let mut sum: i64 = 0;
    for (cycle, x) in x_values.iter().copied().enumerate().skip(20).step_by(40) {
        sum += cycle as i64 * x;
    }
    sum
}

fn part2(input: &str) -> String {
    let mut x_values: Vec<i64> = vec![1, 1];

    for line in input.lines() {
        let op = &line[0..4];
        let current_x = *x_values.last().unwrap();
        match op {
            "noop" => {
                x_values.push(current_x);
            }
            "addx" => {
                let arg = line[5..].parse::<i64>().unwrap();
                x_values.push(current_x);
                x_values.push(current_x + arg);
            }
            _ => panic!("unexpected op"),
        }
    }
    let mut crt = String::new();
    for (cycle, x) in x_values.iter().copied().enumerate().skip(1) {
        if dbg!(x).abs_diff(dbg!((cycle as i64 - 1) % 40)) <= 1 {
            crt.push('#');
        } else {
            crt.push('.');
        }
        if cycle % 40 == 0 {
            crt.push('\n');
        }
    }
    crt
}
