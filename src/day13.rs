use std::{cmp::Ordering, fs};

use colored::*;

const DAY: u32 = 13;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Integer(u8),
}

impl Packet {
    fn compare(left: &Packet, right: &Packet) -> Ordering {
        match (left, right) {
            (Self::List(ll), Self::List(rl)) => ll.cmp(rl),
            (Self::List(_), Self::Integer(_)) => Self::compare(left, &Packet::List(vec![right.clone()])),
            (Self::Integer(_), Self::List(_)) => Self::compare(&Packet::List(vec![left.clone()]), right),
            (Self::Integer(li), Self::Integer(ri)) => li.cmp(ri),
        }
    }

    fn from_str(input: &str) -> Packet {
        let (packet, len) = Packet::from_str_impl(input);
        if len != input.len() {
            panic!("the input has not been fully parsed")
        }
        packet
    }

    fn from_str_impl(input: &str) -> (Packet, usize) {
        let mut current = 0;
        if input.chars().nth(current).unwrap() == '[' {
            current += 1;
            let mut v = vec![];
            while input.chars().nth(current).unwrap() != ']' {
                let (packet, len) = Packet::from_str_impl(&input[current..]);
                v.push(packet);
                current += len;
                if input.chars().nth(current).unwrap() == ',' {
                    current += 1;
                }
            }
            (Packet::List(v), current + 1)
        } else {
            while input.chars().nth(current).unwrap().is_ascii_digit() {
                current += 1;
            }
            (Packet::Integer(input[..current].parse().unwrap()), current)
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        Packet::compare(self, other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Packet::compare(self, other))
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .map(|(l, r)| (Packet::from_str(l.trim()), Packet::from_str(r.trim())))
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, (_, _))| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut v: Vec<_> = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from_str(l))
        .collect();

    let div1 = Packet::from_str("[[2]]");
    let div2 = Packet::from_str("[[6]]");

    v.push(div1.clone());
    v.push(div2.clone());

    v.sort();

    let index1 = v.iter().position(|p| *p == div1).unwrap();
    let index2 = v.iter().position(|p| *p == div2).unwrap();

    (index1 + 1) * (index2 + 1)
}
