use colored::*;
use std::cmp::max;
use std::fs;
use std::ops::{Range, Index};
use std::slice::SliceIndex;

const DAY: u32 = 8;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn trees_are_all_shorter_h(grid: &[Vec<u8>], xrange: Range<usize>, y: usize, h: u8) -> bool {
    for x in xrange {
        if grid[y][x] >= h {
            return false;
        }
    }
    true
}

fn trees_are_all_shorter_v(grid: &[Vec<u8>], x: usize, yrange: Range<usize>, h: u8) -> bool {
    for y in yrange {
        if grid[y][x] >= h {
            return false;
        }
    }
    true
}

fn viewing_distance_h<R>(grid: &[Vec<u8>], xrange: R, y: usize, h: u8) -> usize
where
    R: Iterator,
    Vec<u8>: Index<R::Item>,
     {
    let mut dist = 0;
    for x in xrange {
        dist += 1;
        if grid[y][x] >= h {
            break;
        }
    }
    dist
}

fn viewing_distance_v(grid: &[Vec<u8>], x: usize, yrange: Range<usize>, h: u8) -> usize {
    let mut dist = 0;
    for y in yrange {
        dist += 1;
        if grid[y][x] >= h {
            break;
        }
    }
    dist
}

fn is_visible(grid: &[Vec<u8>], x: usize, y: usize, h: u8) -> bool {
    let nb_lines = grid.len();
    let nb_rows = grid[0].len();

    trees_are_all_shorter_h(grid, (0..x).rev(), y, h)
        || trees_are_all_shorter_h(grid, x + 1..nb_rows, y, h)
        || trees_are_all_shorter_v(grid, x, (0..y).rev(), h)
        || trees_are_all_shorter_v(grid, x, y + 1..nb_lines, h)
}

fn viewing_distance(grid: &[Vec<u8>], x: usize, y: usize, h: u8) -> usize {
    let nb_lines = grid.len();
    let nb_rows = grid[0].len();

    viewing_distance_h(grid, (0..x).rev(), y, h)
        * viewing_distance_h(grid, x + 1..nb_rows, y, h)
        * viewing_distance_v(grid, x,(0..y).rev(), h)
        * viewing_distance_v(grid, x, y + 1..nb_lines, h)
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();
    let nb_lines = grid.len();
    let nb_rows = grid[0].len();

    let mut count = nb_lines * 2 + nb_rows * 2 - 4;
    for (y, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
        for (x, h) in row.iter().copied().enumerate().skip(1).take(row.len() - 2) {
            count += is_visible(&grid, x, y, h) as usize;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();

    let mut max_viewing_dist = 0;
    for (y, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
        for (x, h) in row.iter().copied().enumerate().skip(1).take(row.len() - 2) {
            let view_dist = viewing_distance(&grid, x, y, h);
            max_viewing_dist = max(view_dist, max_viewing_dist);
        }
    }
    max_viewing_dist
}
