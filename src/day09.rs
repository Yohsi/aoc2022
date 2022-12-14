use std::collections::HashSet;
use std::fs;

use colored::*;

const DAY: u32 = 9;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn vec_from_dir(d: char) -> (i32, i32) {
    match d {
        'U' => (1, 0),
        'D' => (-1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => panic!("unexpected direction")
    }
}

fn add((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}

fn adj((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
    x1.abs_diff(x2) <= 1 && y1.abs_diff(y2) <= 1
}

fn new_pos((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    if x1 == x2 {
        (x1, (y1 + y2) / 2)
    } else if y1 == y2 {
        ((x1 + x2) / 2, y1)
    } else {
        let dx = (x1 - x2).signum();
        let dy = (y1 - y2).signum();
        (x2 + dx, y2 + dy)
    }
}

fn part1(input: &str) -> usize {
    let steps = input.lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(d, n)| (d.chars().next().unwrap(), n.parse::<u32>().unwrap()));

    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(t);

    for (dir, nb) in steps {
        let vec = vec_from_dir(dir);

        for _ in 0..nb {
            h = add(h, vec);
            if !adj(h, t) {
                t = new_pos(h, t);
                visited.insert(t);
            }
        }
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let steps = input.lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(d, n)| (d.chars().next().unwrap(), n.parse::<u32>().unwrap()));

    let mut rope = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert(rope[9]);

    for (dir, nb) in steps {
        let vec = vec_from_dir(dir);

        for _ in 0..nb {
            rope[0] = add(rope[0], vec);
            for i in 1..rope.len() {
                if !adj(rope[i - 1], rope[i]) {
                    rope[i] = new_pos(rope[i - 1], rope[i]);
                }
            }
            visited.insert(rope[9]);
        }
    }
    visited.len()
}
