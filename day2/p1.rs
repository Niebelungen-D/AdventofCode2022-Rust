use std::fs;
use std::io::{self, BufRead};

fn check_score(opponent: &str, your: &str) -> i32 {
    let mut res = 0;
    if (opponent == "A" && your == "X")
        || (opponent == "B" && your == "Y")
        || (opponent == "C" && your == "Z")
    {
        // draw
        res = 3;
    } else if (opponent == "A" && your == "Y")
        || (opponent == "B" && your == "Z")
        || (opponent == "C" && your == "X")
    {
        // win
        res = 6;
    };

    // lose
    res
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut sum = 0;

    for line in io::BufReader::new(file).lines().by_ref() {
        let buf = line.unwrap();
        let num: Vec<&str> = buf.split(' ').collect();
        let my = if num[1] == "X" {
            1
        } else if num[1] == "Y" {
            2
        } else {
            3
        };
        let score = my + check_score(num[0], num[1]);
        sum += score;
    }
    println!("sum {}", sum);
}

