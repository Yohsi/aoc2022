use std::{
    collections::HashMap,
    fmt::{Debug, Display}, fs,
};

use colored::*;
use graph::Graph;
use regex::Regex;

mod graph;

const DAY: u32 = 16;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!(
        "{}{}",
        "Part 1 sample: ".bold().yellow(),
        format!("{}", part1(&sample)).bright_white()
    );
    println!(
        "{}{}",
        "Part 1 input : ".bold().blue(),
        format!("{}", part1(&input)).bright_white()
    );
    println!(
        "{}{}",
        "Part 2 sample: ".bold().yellow(),
        format!("{}", part2(&sample)).bright_white()
    );
    println!(
        "{}{}",
        "Part 2 input : ".bold().blue(),
        format!("{}", part2(&input)).bright_white()
    );
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct ValveName {
    name: [u8; 2],
}

impl Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0] as char, self.name[1] as char)
    }
}

impl Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&str> for ValveName {
    fn from(s: &str) -> Self {
        assert!(s.len() == 2);
        ValveName {
            name: [s.bytes().nth(0).unwrap(), s.bytes().nth(1).unwrap()],
        }
    }
}

#[derive(Debug)]
struct Valve {
    name: ValveName,
    flowrate: u32,
    tunnels: Vec<ValveName>,
}

impl Graph<ValveName> for HashMap<ValveName, Valve> {
    fn neighbors(&self, node: ValveName) -> Vec<ValveName> {
        self[&node].tunnels.clone()
    }
}

fn parse_valves(input: &str) -> HashMap<ValveName, Valve> {
    let re =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels lead to valves ([A-Z]{2}(?:, [A-Z]{2})*)").unwrap();
    let mut valves = HashMap::new();

    for m in re.captures_iter(input) {
        let name = m.get(1).unwrap().as_str().into();
        valves.insert(name, Valve {
            name,
            flowrate: m.get(2).unwrap().as_str().parse().unwrap(),
            tunnels: m.get(3).unwrap().as_str().split(", ").map(|s| s.into()).collect(),
        });
    }
    valves
}

fn part1(input: &str) -> usize {
    let valves = parse_valves(input);
    dbg!(valves);
    0
}

fn part2(input: &str) -> usize {
    0
}
