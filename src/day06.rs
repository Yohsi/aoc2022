use std::fs;
use colored::*;

const DAY: u32 = 6;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn part1(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(4)
        .enumerate()
        .filter(|(_, s)| {
            let mut v = s.to_vec();
            v.sort();
            v.dedup();
            v.len() == s.len()
        })
        .next()
        .unwrap()
        .0 + 4
}

fn part2(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(14)
        .enumerate()
        .filter(|(_, s)| {
            let mut v = s.to_vec();
            v.sort();
            v.dedup();
            v.len() == s.len()
        })
        .next()
        .unwrap()
        .0 + 14
}
