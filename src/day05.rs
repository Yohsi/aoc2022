use std::fs;
use colored::*;

const DAY: u32 = 5;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

fn part1(input: &str) -> String {
    let (raw_stacks, raw_moves) = input.split_once("\n\n").unwrap();
    let mut raw_stacks = raw_stacks.lines().rev();
    let raw_moves = raw_moves.lines();

    // get the nb of columns based on the length of the indices line
    let len = (raw_stacks.next().unwrap().len() + 1) / 4;
    let mut stacks : Vec<Vec<char>> = vec![vec![]; len];

    for row in raw_stacks {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate()
        .filter(|(_, c)| {*c != ' '}) {
            stacks[i].push(c);
        }
    }

    for mov in raw_moves {
        let nb = mov.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        let from = mov.split_whitespace().nth(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = mov.split_whitespace().nth(5).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..nb {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    let mut res = String::new();
    for stack in &mut stacks {
        res.push(stack.pop().unwrap());
    }
    res
}

fn part2(input: &str) -> String {
    let (raw_stacks, raw_moves) = input.split_once("\n\n").unwrap();
    let mut raw_stacks = raw_stacks.lines().rev();
    let raw_moves = raw_moves.lines();

    // get the nb of columns based on the length of the indices line
    let len = (raw_stacks.next().unwrap().len() + 1) / 4;
    let mut stacks : Vec<Vec<char>> = vec![vec![]; len];

    for row in raw_stacks {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate()
        .filter(|(_, c)| {*c != ' '}) {
            stacks[i].push(c);
        }
    }

    for mov in raw_moves {
        let nb = mov.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        let from = mov.split_whitespace().nth(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = mov.split_whitespace().nth(5).unwrap().parse::<usize>().unwrap() - 1;

        let len = stacks[from].len();
        let mut slice = stacks[from][len-nb..].to_owned();
        stacks[to].append(&mut slice);
        stacks[from].resize(len-nb, ' ');
    }

    let mut res = String::new();
    for stack in &mut stacks {
        res.push(stack.pop().unwrap());
    }
    res
}
