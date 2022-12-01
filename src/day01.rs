use std::fs;
use colored::*;

const DAY: u32 = 1;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn part1(input: &str) -> u64 {
    input
        .split_terminator("\n\n")
        .map(|s| {
            s.split_terminator("\n")
                .map(|nb_str| nb_str.parse::<u64>().unwrap())
                .sum()
        })
        .max()
        .unwrap_or_default()
}

fn part2(input: &str) -> u64 {
    let mut vec: Vec<u64> = input
        .split_terminator("\n\n")
        .map(|s| {
            s.split_terminator("\n")
                .map(|nb_str| nb_str.parse::<u64>().unwrap())
                .sum()
        })
        .collect();
    vec.sort();
    vec.iter().copied().rev().take(3).sum()
}
