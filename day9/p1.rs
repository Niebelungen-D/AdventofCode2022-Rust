use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;
struct Move {
    direction: String,
    steps: i32,
}

fn parse_input_line(line: &str) -> Option<Move> {
    let pattern = r"^(\w) (\d+)$";

    if let Some(caps) = Regex::new(pattern).unwrap().captures(line) {
        if let Some(letter_str) = caps.get(1) {
            let letter = letter_str.as_str().to_string();
            if let Some(number_str) = caps.get(2) {
                if let Ok(number) = number_str.as_str().parse::<i32>() {
                    return Some(Move {
                        direction: letter,
                        steps: number,
                    });
                } else {
                    println!("Invalid number format");
                }
            } else {
                println!("Missing number");
            }
        } else {
            println!("Missing letter");
        }
    } else {
        println!("Invalid input format: {}", line);
    }

    None
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().trim().to_owned());
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut hbook : HashSet<(i32, i32)> = HashSet::new();
    for line in lines {
        if let Some(cmd) = parse_input_line(&line) {
            for _ in 0..cmd.steps {
                match cmd.direction.as_str() {
                    "L" => {
                        let dx = head.0 - tail.0;
                        if dx < 0 { // move
                            tail.0 = head.0;
                            tail.1 = head.1;
                        }
                        head.0 -=1
                    }
                    "R" => {
                        let dx = head.0 - tail.0;
                        if dx > 0 { // move
                            tail.0 = head.0;
                            tail.1 = head.1;
                        }
                        head.0 +=1
                    }
                    "U" => {
                        let dy = head.1 - tail.1;
                        if dy > 0 { // move
                            tail.0 = head.0;
                            tail.1 = head.1;
                        }
                        head.1 +=1
                    }
                    "D" => {
                        let dy = head.1 - tail.1;
                        if dy < 0 { // move
                            tail.0 = head.0;
                            tail.1 = head.1;
                        }
                        head.1 -=1
                    }
                    _ => {
                        println!("Invalid direction: {}", cmd.direction)
                    }
                }
                hbook.insert(tail);
            }
        }
    }
    println!("res: {}", hbook.len())
}
