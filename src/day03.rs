use std::fs;
use colored::*;

const DAY: u32 = 3;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn priorities(c: char) -> u64 {
    return match c {
        'A'..='Z' => 27 + c as u8 - 'A' as u8,
        'a'..='z' => 1 + c as u8 - 'a' as u8,
        _ => panic!("unexpected char")
    } as u64;
}

fn part1(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|s| {
            let compartments = s.split_at(s.len() / 2);
            for c in compartments.0.chars() {
                if compartments.1.contains(c) {
                    return c;
                }
            }
            panic!("shared item not found");
        })
        .map(priorities)
        .sum::<u64>()
}

fn find_shared(v: &[&str]) -> char {
    for c in v[0].chars() {
        if v[1].contains(c) && v[2].contains(c) {
            return c;
        }
    }
    panic!("shared char not found");
}

fn part2(input: &str) -> u64 {
    let a: Vec<_> = input
        .split_whitespace()
        .collect();

    let mut sum = 0 as u64;
    for group in a.chunks(3) {
        sum += priorities(find_shared(group));
    }
    sum
}
