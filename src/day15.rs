use std::collections::HashSet;
use std::fs;

use colored::*;
use regex::Regex;

const DAY: u32 = 15;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample, 10)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input, 2000000)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample, 20)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input, 4000000)).bright_white());
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance_to(&self, o: Coord) -> u32 {
        self.x.abs_diff(o.x) + self.y.abs_diff(o.y)
    }
}


#[derive(Debug)]
struct Sensor {
    coord: Coord,
    nearest_beacon: Coord,
    distance: u32,
}

impl Sensor {
    fn new(coord: Coord, nearest_beacon: Coord) -> Sensor {
        Sensor {
            coord,
            nearest_beacon,
            distance: coord.distance_to(nearest_beacon),
        }
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let mut v = vec![];
    for captures in re.captures_iter(input) {
        v.push(Sensor::new(
            Coord {
                x: captures.get(1).unwrap().as_str().parse().unwrap(),
                y: captures.get(2).unwrap().as_str().parse().unwrap(),
            },
            Coord {
                x: captures.get(3).unwrap().as_str().parse().unwrap(),
                y: captures.get(4).unwrap().as_str().parse().unwrap(),
            },
        ))
    }
    v
}

fn part1(input: &str, row: i32) -> usize {
    let sensors = parse_sensors(input);
    let mut impossible_x = HashSet::new();

    for sensor in sensors {
        let y_distance = sensor.coord.y.abs_diff(row);
        let min_x = sensor.coord.x - sensor.distance as i32 + y_distance as i32;
        let max_x = sensor.coord.x + sensor.distance as i32 - y_distance as i32;
        for x in min_x..=max_x {
            if x != sensor.nearest_beacon.x || row != sensor.nearest_beacon.y {
                impossible_x.insert(x);
            }
        }
    }

    impossible_x.len()
}

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, x: i32) -> bool {
        x >= self.min && x <= self.max
    }
}

fn get_first_not_included(ranges: &[Range], from: i32, to: i32) -> Option<i32> {
    let mut current = from;
    let mut last_modif = None;
    for (i, range) in ranges.iter().enumerate().cycle() {
        if last_modif == Some(i) {
            return Some(current)
        }
        if range.contains(current) {
            current = range.max + 1;
            last_modif = Some(i);
        }
        if current > to {
            return None;
        }
    }
    None
}

fn tuning_freq(coord: Coord) -> i64 {
    coord.x as i64 * 4000000 + coord.y as i64
}

fn part2(input: &str, max_coord: i32) -> i64 {
    let sensors = parse_sensors(input);
    let mut ranges = Vec::new();

    for y in 0..max_coord {
        ranges.clear();
        for sensor in &sensors {
            let y_distance = sensor.coord.y.abs_diff(y);
            let min_x = sensor.coord.x - sensor.distance as i32 + y_distance as i32;
            let max_x = sensor.coord.x + sensor.distance as i32 - y_distance as i32;
            ranges.push(Range { min: min_x, max: max_x });

            if y == sensor.nearest_beacon.y {
                ranges.push(Range { min: sensor.nearest_beacon.x, max: sensor.nearest_beacon.x })
            }
        }
        if let Some(x) = get_first_not_included(&ranges, 0, max_coord) {
            return tuning_freq(Coord { x, y });
        }
    }
    panic!("not found")
}

