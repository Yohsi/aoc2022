use std::fs;
use colored::*;

const DAY: u32 = 2;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn points_for_round(round: &str) -> i64 {
    let our_shape_points = match round.chars().nth(2) {
        Some('X') => 1,
        Some('Y') => 2,
        Some('Z') => 3,
        _ => panic!("unexpected value for our shape"),
    };
    let their_shape_points = match round.chars().nth(0) {
        Some('A') => 1,
        Some('B') => 2,
        Some('C') => 3,
        _ => panic!("unexpected value for their shape"),
    };

    let winner_points = match (our_shape_points - their_shape_points + 3) % 3 {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("unexpected result"),
    };
    our_shape_points + winner_points
}

fn part1(input: &str) -> i64 {
    input
        .split_terminator("\n")
        .map(points_for_round)
        .sum()
}

fn points_for_round2(round: &str) -> i64 {
    let their_shape_points = match round.chars().nth(0) {
        Some('A') => 1,
        Some('B') => 2,
        Some('C') => 3,
        _ => panic!("unexpected value for their shape"),
    };

    let (our_shape_points, winner_points) = match round.chars().nth(2) {
        Some('X') => ((their_shape_points + 1) % 3 + 1, 0),
        Some('Y') => ((their_shape_points + 2) % 3 + 1, 3),
        Some('Z') => (their_shape_points % 3 + 1, 6),
        _ => panic!("unexpected value for our instruction"),
    };
    our_shape_points + winner_points
}

fn part2(input: &str) -> i64 {
    input
        .split_terminator("\n")
        .map(points_for_round2)
        .sum()
}
