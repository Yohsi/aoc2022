use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;

use colored::*;

const DAY: u32 = 11;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample:\n".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input :\n".bold().blue(), format!("{}", part2(&input)).bright_white());
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Arg {
    Old,
    Litteral(u64),
}

#[derive(Debug, Copy, Clone)]
struct Item {
    worry_level: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    op: Op,
    arg: Arg,
    test: u64,
    next_monkey_t: u32,
    next_monkey_f: u32,
}

impl Monkey {
    fn inspect(&mut self) -> Item {
        let mut item = self.items.pop_back().unwrap();
        let arg = match self.arg {
            Arg::Old => { item.worry_level }
            Arg::Litteral(l) => { l }
        };
        item.worry_level = match self.op {
            Op::Add => item.worry_level + arg,
            Op::Mul => item.worry_level * arg,
        };
        item
    }
    fn get_next_monkey(&self, item: Item) -> u32 {
        if item.worry_level % self.test == 0 { self.next_monkey_t } else { self.next_monkey_f }
    }
}

fn parse_monkeys(input: &str) -> Vec<RefCell<Monkey>> {
    let mut monkeys = vec![];
    for raw in input.split("\n\n") {
        let mut lines = raw.lines().skip(1);
        let items = lines
            .next().unwrap()
            .strip_prefix("  Starting items: ").unwrap()
            .split(", ")
            .map(|n| Item { worry_level: n.parse().unwrap() })
            .collect();
        let (op_str, arg_str) = lines.next().unwrap()
            .strip_prefix("  Operation: new = old ").unwrap()
            .split_once(" ").unwrap();
        let op = match op_str {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("unexpected operation")
        };
        let arg = match arg_str {
            "old" => Arg::Old,
            a => Arg::Litteral(a.parse().unwrap()),
        };
        let test = lines.next().unwrap()
            .strip_prefix("  Test: divisible by ").unwrap()
            .parse().unwrap();
        let next_monkey_t = lines.next().unwrap()
            .strip_prefix("    If true: throw to monkey ").unwrap()
            .parse().unwrap();
        let next_monkey_f = lines.next().unwrap()
            .strip_prefix("    If false: throw to monkey ").unwrap()
            .parse().unwrap();
        monkeys.push(RefCell::new(Monkey { items, op, arg, test, next_monkey_f, next_monkey_t }));
    }
    monkeys
}

fn part1(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    let mut monkey_activity = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            while !monkey.borrow().items.is_empty() {
                monkey_activity[i] += 1;
                let mut item = monkey.borrow_mut().inspect();
                item.worry_level /= 3;
                let next_monkey = monkey.borrow().get_next_monkey(item) as usize;
                monkeys[next_monkey].borrow_mut().items.push_front(item);
            }
        }
    }
    monkey_activity.sort_unstable();
    monkey_activity.pop().unwrap() * monkey_activity.pop().unwrap()
}

fn part2(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    let common_multiple: u64 = monkeys.iter().map(|m| m.borrow().test).product();
    let mut monkey_activity = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            while !monkey.borrow().items.is_empty() {
                monkey_activity[i] += 1;
                let mut item = monkey.borrow_mut().inspect();
                item.worry_level = item.worry_level % common_multiple;
                let next_monkey = monkey.borrow().get_next_monkey(item) as usize;
                monkeys[next_monkey].borrow_mut().items.push_front(item);
            }
        }
    }
    monkey_activity.sort_unstable();
    monkey_activity.pop().unwrap() * monkey_activity.pop().unwrap()
}
