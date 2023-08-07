use regex::Regex;
// use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::process::exit;
// use std::{thread, time};

enum Opcode {
    AddX(i32),
    NoOp,
    Invalid,
}

fn parse_input_line(line: &str) -> Opcode {
    let addx_pattern = r"^addx (-?\d+)$";
    let noop_pattern = r"^noop$";

    if let Some(caps) = Regex::new(addx_pattern).unwrap().captures(line) {
        if let Some(number) = caps.get(1) {
            if let Ok(imm) = number.as_str().parse::<i32>() {
                return Opcode::AddX(imm);
            } else {
                println!("Invalid number format");
            }
        }
    } else if Regex::new(noop_pattern).unwrap().is_match(line) {
        return Opcode::NoOp;
    }

    Opcode::Invalid
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().trim().to_owned());
    let mut x = 1;
    let mut cycles = 1;
    let mut res = 0;
    for line in lines {
        let opcode = parse_input_line(&line);
        match opcode {
            Opcode::AddX(imm) => {
                for _ in 0..2 {
                    if [20, 60, 100, 140, 180, 220].contains(&cycles) {
                        res += x * cycles;
                    }
                    cycles += 1;
                }
                x += imm;
            }
            Opcode::NoOp => {
                if [20, 60, 100, 140, 180, 220, 260].contains(&cycles) {
                    res += x * cycles;
                }
                cycles += 1;
            }
            Opcode::Invalid => {
                println!("Invalid");
                exit(-1)
            }
        }
    }
    println!("Result: {}", res);
}
