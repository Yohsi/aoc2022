use std::fs;
use colored::*;

const DAY: u32 = 4;

struct Range {
    min: u64,
    max: u64,
}

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
        .split_terminator("\n")
        .map(|pair| {
            pair.split(",")
                .map(|range| {
                    range.split("-")
                        .map(|section_id| section_id.parse::<u64>().unwrap()).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }).filter(|pair| {
        pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1] ||
            pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1]
    })
        .count() as u64
}

fn part2(input: &str) -> u64 {
    input
        .split_terminator("\n")
        .map(|pair| {
            pair.split(",")
                .map(|range| {
                    range.split("-")
                        .map(|section_id| section_id.parse::<u64>().unwrap()).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }).filter(|pair| {
        let p1_min = pair[0][0];
        let p1_max = pair[0][1];
        let p2_min = pair[1][0];
        let p2_max = pair[1][1];
        p2_min <= p1_min && p1_min <= p2_max ||
        p2_min <= p1_max && p1_max <= p2_max ||
        p1_min <= p2_min && p2_min <= p1_max ||
        p1_min <= p2_max && p2_max <= p1_max

    })
        .count() as u64
}
