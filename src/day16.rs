use std::{collections::HashMap, fmt::{Debug, Display}, fs};
use std::cmp::{max, min};
use std::collections::HashSet;

use colored::*;
use regex::Regex;

use crate::graph::{CostGraph, Graph};

mod graph;

const DAY: u32 = 16;
const AA: ValveName = ValveName { name: ['A' as u8; 2] };
// const DD: ValveName = ValveName { name: ['D' as u8; 2] };
// const BB: ValveName = ValveName { name: ['B' as u8; 2] };
// const JJ: ValveName = ValveName { name: ['J' as u8; 2] };
// const HH: ValveName = ValveName { name: ['H' as u8; 2] };
// const EE: ValveName = ValveName { name: ['E' as u8; 2] };
// const CC: ValveName = ValveName { name: ['C' as u8; 2] };

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
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
        assert_eq!(s.len(), 2);
        ValveName {
            name: [s.bytes().nth(0).unwrap(), s.bytes().nth(1).unwrap()],
        }
    }
}

#[derive(Debug)]
struct Valve {
    name: ValveName,
    flowrate: u32,
    tunnels: HashMap<ValveName, u8>,
}

type Costs = HashMap<ValveName, HashMap<ValveName, u8>>;

#[derive(Debug)]
struct PipeNetwork {
    valves: HashMap<ValveName, Valve>,
}

impl PipeNetwork {
    fn simplify(&mut self) {
        let useless_valves: Costs = self.valves.iter()
            .filter(|(_, v)| v.flowrate == 0 && v.name != AA)
            .map(|(&name, v)| (name, v.tunnels.clone()))
            .collect();

        for valve in self.valves.values_mut() {
            let mut done = false;
            let mut visited = HashSet::new();
            while !done {
                let useless_tunnel = valve.tunnels.iter()
                    .filter(|(name, _)| { useless_valves.contains_key(name) }).next();


                if let Some((&dest, &cost)) = useless_tunnel {
                    for (&transitive_dest, &transitive_cost) in useless_valves[&dest].iter()
                        .filter(|(&n, _)| !visited.contains(&n) && n != valve.name) {
                        let current = valve.tunnels.entry(transitive_dest).or_insert(100);
                        *current = min(*current, cost + transitive_cost);
                    }
                    valve.tunnels.remove(&dest);
                    visited.insert(dest);
                } else {
                    done = true;
                }
            }
        }

        for useless_valve in useless_valves.keys() {
            self.valves.remove(useless_valve);
        }
    }

    fn compute_costs(&self) -> Costs {
        let mut costs = HashMap::new();
        for node in self.valves.keys().copied() {
            costs.insert(node, self.dijkstra(node));
        }
        costs
    }
    fn calculate_best_path(&mut self, time: u8, with_elephant: bool) -> u32 {
        let costs = self.compute_costs();
        let mut todo: Vec<_> = self.valves.keys().copied().filter(|&v| v != AA).collect();
        todo.sort_by(|a, b| self.valves[a].flowrate.cmp(&self.valves[b].flowrate));
        if with_elephant {
            self.calculate_best_path_elephant_rec(&costs, &todo, AA, time, AA, time)
        } else {
            self.calculate_best_path_rec(&costs, &todo, AA, time)
        }
    }

    fn calculate_best_path_rec(&mut self, costs: &Costs, todo: &[ValveName], from: ValveName, remaining: u8) -> u32 {
        let mut best_pressure = 0;
        let mut todo2 = todo.to_vec();
        for (i, &node) in todo.iter().enumerate() {
            let cost = costs[&from][&node] + 1;
            if cost > remaining {
                continue;
            }
            let remaining_after = remaining - cost;
            todo2.remove(i);
            let mut pressure = self.calculate_best_path_rec(costs, &todo2, node, remaining_after);
            pressure += self.valves[&node].flowrate * remaining_after as u32;
            best_pressure = max(pressure, best_pressure);
            todo2.insert(i, node);
        }
        best_pressure
    }
    fn calculate_best_path_elephant_rec(&mut self, costs: &Costs, todo: &[ValveName], from: ValveName, remaining: u8, other_dest: ValveName, other_remaining: u8) -> u32 {
        let mut best_pressure = 0;
        let mut todo2 = todo.to_vec();
        for (i, &node) in todo.iter().enumerate() {
            let cost = costs[&from][&node] + 1;
            if cost > remaining {
                continue;
            }
            let remaining_after = remaining - cost;
            todo2.remove(i);

            let mut pressure =
                if other_remaining > remaining_after {
                    self.calculate_best_path_elephant_rec(costs, &todo2, other_dest, other_remaining, node, remaining_after)
                } else {
                    self.calculate_best_path_elephant_rec(costs, &todo2, node, remaining_after, other_dest, other_remaining)
                };
            pressure += self.valves[&node].flowrate * remaining_after as u32;
            best_pressure = max(pressure, best_pressure);

            todo2.insert(i, node);
        }
        best_pressure
    }
}

impl Graph<ValveName> for PipeNetwork {
    fn neighbors(&self, node: ValveName) -> Vec<ValveName> {
        self.valves[&node].tunnels.keys().copied().collect()
    }
}

impl CostGraph<ValveName, u8> for PipeNetwork {
    fn cost(&self, from: ValveName, to: ValveName) -> u8 {
        self.valves[&from].tunnels[&to]
    }
}

fn parse_valves(input: &str) -> PipeNetwork {
    let re =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)").unwrap();
    let mut valves = HashMap::new();

    for m in re.captures_iter(input) {
        let name = m.get(1).unwrap().as_str().into();
        valves.insert(name, Valve {
            name,
            flowrate: m.get(2).unwrap().as_str().parse().unwrap(),
            tunnels: m.get(3).unwrap().as_str().split(", ").map(|s| (s.into(), 1)).collect(),
        });
    }
    PipeNetwork { valves }
}

fn part1(input: &str) -> u32 {
    let mut valves = parse_valves(input);
    valves.simplify();
    valves.calculate_best_path(30, false)
}

fn part2(input: &str) -> u32 {
    let mut valves = parse_valves(input);
    valves.simplify();
    valves.calculate_best_path(26, true)
}
