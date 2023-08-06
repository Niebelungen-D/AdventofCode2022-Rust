use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
// use std::{thread, time};
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

fn snake_move(new: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dx = new.0 - tail.0;
    let dy = new.1 - tail.1;
    let mut ntail = tail;

    if dy.abs() < 2 && dx.abs() < 2 {
        return ntail;
    }

    if dx > 0 {
        ntail.0 += 1;
    } else if dx < 0 {
        ntail.0 -= 1;
    }

    if dy > 0 {
        ntail.1 += 1;
    } else if dy < 0 {
        ntail.1 -= 1;
    }

    ntail
}

fn draw(snake: &Vec<(i32, i32)>) {
    let min_x = -25;
    let max_x = 25;
    let min_y = -10;
    let max_y = 20;

    let mut watcher = HashSet::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if snake.contains(&(x, y)) {
                for (i, (sx, sy)) in snake.iter().enumerate() {
                    if *sx == x && *sy == y && !watcher.contains(&(x, y)) {
                        print!("{}", i);
                        watcher.insert((x, y));
                    }
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().trim().to_owned());
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut hbook: HashSet<(i32, i32)> = HashSet::new();
    for line in lines {
        if let Some(cmd) = parse_input_line(&line) {
            for _ in 0..cmd.steps {
                match cmd.direction.as_str() {
                    "L" => snake[0].0 -= 1,
                    "R" => snake[0].0 += 1,
                    "U" => snake[0].1 += 1,
                    "D" => snake[0].1 -= 1,
                    _ => println!("Invalid direction"),
                }
                for i in 1..snake.len() {
                    snake[i] = snake_move(snake[i - 1], snake[i]);
                    // print!("\x1b[2J");
                    // print!("\x1b[H");
                    // println!("========= {} ==========", line.as_str());
                    // draw(&snake);
                    // let ten_millis = time::Duration::from_millis(100);
                    // thread::sleep(ten_millis);
                }
                hbook.insert(snake[snake.len() - 1]);
            }
        }
    }
    println!("res: {}", hbook.len())
}
