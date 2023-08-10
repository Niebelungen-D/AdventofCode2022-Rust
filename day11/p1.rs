// use regex::Regex;
use std::collections::VecDeque;
// use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::process::exit;
// use std::{thread, time};
#[derive(PartialEq, Eq, Clone, Debug)]
enum Opcode {
    Add(i32),
    Mul(i32),
    Double,
    Square,
    Invalid,
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Opcode,
    test: i32,
    true_monkey: i32,
    false_monkey: i32,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: Opcode::Invalid,
            test: -1,
            true_monkey: -1,
            false_monkey: -1,
        }
    }
}

fn parse_operation(operation_str: &str) -> Opcode {
    let parts: Vec<&str> = operation_str.split_whitespace().collect();
    if parts.len() == 5 {
        let opcode_str = parts[3];
        match opcode_str {
            "+" => {
                if let Ok(val) = parts[4].parse::<i32>() {
                    Opcode::Add(val)
                } else if parts[4] == "old" {
                    Opcode::Double
                } else {
                    Opcode::Invalid
                }
            }
            "*" => {
                if let Ok(val) = parts[4].parse::<i32>() {
                    Opcode::Mul(val)
                } else if parts[4] == "old" {
                    Opcode::Square
                } else {
                    Opcode::Invalid
                }
            }
            _ => Opcode::Invalid,
        }
    } else {
        Opcode::Invalid
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let file = fs::File::open(input).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().trim().to_owned());
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey: Option<Monkey> = None;

    for line in lines {
        if line.contains("Monkey") {
            if current_monkey.is_none() {
                current_monkey = Some(Monkey::new());
            } else {
                println!("Error: Monkey already exists");
                exit(-1);
            }
        } else if let Some(monkey) = current_monkey.as_mut() {
            if line.starts_with("Starting items:") {
                let items: VecDeque<i64> = line
                    .trim_start_matches("Starting items:")
                    .split(',')
                    .filter_map(|item| item.trim().parse().ok())
                    .collect();
                monkey.items = items;
            } else if line.starts_with("Operation:") {
                let operation_str = line.trim_start_matches("Operation: ").trim();
                monkey.operation = parse_operation(operation_str);
            } else if line.starts_with("Test:") {
                monkey.test = line
                    .trim_start_matches("Test: divisible by")
                    .trim()
                    .to_string()
                    .parse()
                    .unwrap();
            } else if line.starts_with("If true:") {
                monkey.true_monkey = line
                    .trim_start_matches("If true: throw to monkey")
                    .trim()
                    .parse()
                    .unwrap_or(0);
            } else if line.starts_with("If false:") {
                monkey.false_monkey = line
                    .trim_start_matches("If false: throw to monkey")
                    .trim()
                    .parse()
                    .unwrap_or(0);
            }
        }
        if line == "" {
            if let Some(monkey) = current_monkey.take() {
                monkeys.push(monkey);
            }
            current_monkey = None;
        }
    }
    if let Some(monkey) = current_monkey.take() {
        monkeys.push(monkey);
    }
    monkeys
}

fn main() {
    let mut mks = parse_input("./input");
    let mut inspect_cnt = vec![0; mks.len()];

    for j in 0..20 {
        for i in 0..mks.len() {
            while !mks[i].items.is_empty() {
                let mut item = mks[i].items.pop_front().unwrap();
                inspect_cnt[i] += 1;
                match mks[i].operation {
                    Opcode::Add(x) => {
                        item += x as i64;
                    }
                    Opcode::Mul(x) => {
                        item *= x as i64;
                    }
                    Opcode::Square => {
                        item = item * item;
                    }
                    Opcode::Double => {
                        item = item * 2;
                    }
                    Opcode::Invalid => {
                        println!("Invalid opcode");
                        exit(1);
                    }
                }
                item /= 3;
                if (item % mks[i].test as i64) == 0 {
                    let dst = mks[i].true_monkey as usize;
                    mks[dst].items.push_back(item);
                    // println!("Monkey {} throws {} to monkey {}", i, item, dst);
                } else {
                    let dst = mks[i].false_monkey as usize;
                    mks[dst].items.push_back(item);
                    // println!("Monkey {} throws {} to monkey {}", i, item, dst);
                }
            }
        }
        // for m in &mks {
        //     println!("Monkey {:?}", m.items);
        // }
    }

    inspect_cnt.sort();
    let x = inspect_cnt.pop().unwrap() as i64;
    let y = inspect_cnt.pop().unwrap() as i64;
    println!("x: {}, y: {}, {}", x, y, x*y);
}
