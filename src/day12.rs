use std::cmp::min;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

use colored::*;

use graph::Graph;

mod graph;

const DAY: u32 = 12;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Mountain {
    x: u8,
    y: u8,
    elevation: u8,
}

#[derive(Debug)]
struct ElevationMap {
    nodes: Vec<Vec<Mountain>>,
}

impl Graph<Mountain> for ElevationMap {
    fn neighbors(&self, node: Mountain) -> Vec<Mountain> {
        let mut neighbors = vec![];
        let x = node.x as usize;
        let y = node.y as usize;
        let mut push_if_ok = |n: Mountain| {
            if n.elevation <= node.elevation + 1 {
                neighbors.push(n);
            }
        };
        if y < self.nodes.len() - 1 {
            push_if_ok(self.nodes[y + 1][x]);
        }
        if y > 0 {
            push_if_ok(self.nodes[y - 1][x]);
        }
        if x < self.nodes[0].len() - 1 {
            push_if_ok(self.nodes[y][x + 1]);
        }
        if x > 0 {
            push_if_ok(self.nodes[y][x - 1]);
        }
        neighbors
    }
}

#[derive(Debug)]
struct Problem {
    map: ElevationMap,
    start: Mountain,
    end: Mountain,
}

fn parse_problem(input: &str) -> Problem {
    let mut nodes = vec![];
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        nodes.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            let elevation = match c {
                'S' => 'a' as u8 - 'a' as u8,
                'E' => 'z' as u8 - 'a' as u8,
                _ => c as u8 - 'a' as u8
            };
            let mountain = Mountain { y: y as u8, x: x as u8, elevation };
            nodes[y].push(mountain);
            if c == 'S' {
                start = Some(mountain);
            } else if c == 'E' {
                end = Some(mountain);
            }
        }
    }
    if start == None || end == None {
        panic!("End or start not found");
    }
    Problem { map: ElevationMap { nodes }, start: start.unwrap(), end: end.unwrap() }
}


fn part1(input: &str) -> usize {
    let problem = parse_problem(input);
    let path = problem.map.route(problem.start, problem.end);
    if let Some(path) = path {
        path.len()
    } else {
        panic!("Path not found")
    }
}

fn part2(input: &str) -> usize {
    let problem = parse_problem(input);
    let mut min_len = usize::MAX;
    for start in problem.map.nodes.iter().flatten().copied().filter(|m| m.elevation == 0) {
        let path = problem.map.route(start, problem.end);
        if let Some(path) = path {
            min_len = min(path.len(), min_len)
        }
    }
    min_len
}
