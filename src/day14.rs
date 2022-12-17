use std::fs;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::Add;

use colored::*;

const DAY: u32 = 14;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    const ZERO: Coord = Coord { x: 0, y: 0 };
    const DOWN: Coord = Coord { x: 0, y: 1 };
    const DOWN_LEFT: Coord = Coord { x: -1, y: 1 };
    const DOWN_RIGHT: Coord = Coord { x: 1, y: 1 };
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

struct CoordRange {
    current: Coord,
    end: Coord,
    direction: Coord,
}

impl CoordRange {
    fn inclusive(from: Coord, to: Coord) -> CoordRange {
        CoordRange { current: from, end: to, direction: Coord { x: (to.x - from.x).signum(), y: (to.y - from.y).signum() } }
    }
}

impl Iterator for CoordRange {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.direction == Coord::ZERO {
            return None;
        }
        if self.current == self.end {
            self.direction = Coord::ZERO;
        }
        let ret = self.current;
        self.current = self.current + self.direction;
        Some(ret)
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Point {
    Air,
    Rock,
    Sand,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Point::Air => '.',
            Point::Rock => '#',
            Point::Sand => 'o',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Debug)]
struct Cave {
    scan: Vec<VecDeque<Point>>,
    x_offset: i32,
}

impl Cave {
    fn from_paths(paths: Vec<Vec<Coord>>) -> Cave {
        let (max_x, max_y) = paths.iter()
            .flatten()
            .fold((i32::MIN, i32::MIN),
                  |(x, y), coord| (max(x, coord.x), max(y, coord.y)));
        let min_x = paths.iter()
            .flatten()
            .fold(i32::MAX, |x, coord| min(x, coord.x));

        let width = (max_x - min_x + 3) as usize;
        let height = (max_y + 2) as usize;

        let mut cave = Cave {
            scan: vec![VecDeque::from(vec![Point::Air; width]); height],
            x_offset: min_x - 1,
        };


        for path in &paths {
            for window in path.windows(2) {
                for coord in CoordRange::inclusive(window[0], window[1]) {
                    *cave.get_mut(coord) = Point::Rock;
                }
            }
        }
        cave
    }

    fn get_mut(&mut self, coord: Coord) -> &mut Point {
        if coord.x < self.x_offset {
            self.expand_left();
        }
        if coord.x >= self.x_offset + self.scan[0].len() as i32 {
            self.expand_right();
        }
        let y = coord.y as usize;
        let x = (coord.x - self.x_offset) as usize;
        self.scan.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    fn get(&self, coord: Coord) -> Point {
        if coord.x < self.x_offset || coord.x >= self.x_offset + self.scan[0].len() as i32 {
            return Point::Air;
        }
        let y = coord.y as usize;
        let x = (coord.x - self.x_offset) as usize;
        self.scan[y][x]
    }

    fn expand_left(&mut self) {
        for row in &mut self.scan {
            row.push_front(Point::Air);
        }
        self.x_offset -= 1;
    }

    fn expand_right(&mut self) {
        for row in &mut self.scan {
            row.push_back(Point::Air);
        }
    }

    fn add_sand(&mut self, floor: bool) -> bool {
        let mut sand_coord = Coord { x: 500, y: 0 };
        let mut rest = false;
        while !rest && (sand_coord.y as usize) < self.scan.len() - 1 {
            let down_coord = sand_coord + Coord::DOWN;
            if self.get(down_coord) == Point::Air {
                sand_coord = down_coord;
                continue;
            }
            let down_l_coord = sand_coord + Coord::DOWN_LEFT;
            if self.get(down_l_coord) == Point::Air {
                sand_coord = down_l_coord;
                continue;
            }
            let down_r_coord = sand_coord + Coord::DOWN_RIGHT;
            if self.get(down_r_coord) == Point::Air {
                sand_coord = down_r_coord;
                continue;
            }
            rest = true;
        }
        if rest || floor {
            *self.get_mut(sand_coord) = Point::Sand;
        }
        if floor {
            sand_coord != Coord { x: 500, y: 0 }
        } else {
            rest
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.scan {
            for p in row {
                write!(f, "{p}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let paths: Vec<Vec<_>> = input.lines()
        .map(|line| line.split(" -> ").map(|coord| {
            let (x, y) = coord.split_once(",").unwrap();
            Coord { x: x.parse().unwrap(), y: y.parse().unwrap() }
        }).collect())
        .collect();

    let mut scan = Cave::from_paths(paths);

    let mut count = 0;
    while scan.add_sand(false) {
        count += 1;
    }
    count
}

fn part2(input: &str) -> usize {
    let paths: Vec<Vec<_>> = input.lines()
        .map(|line| line.split(" -> ").map(|coord| {
            let (x, y) = coord.split_once(",").unwrap();
            Coord { x: x.parse().unwrap(), y: y.parse().unwrap() }
        }).collect())
        .collect();

    let mut scan = Cave::from_paths(paths);

    let mut count = 0;
    while scan.add_sand(true) {
        count += 1;
    }
    count + 1
}

