use std::fs;
use std::io::{self, BufRead};

fn check_score(opponent: &str, your: &str) -> i32 {
    let mut res = 3;
    if (opponent == "A" && your == "Y")
        || (opponent == "B" && your == "X")
        || (opponent == "C" && your == "Z")
    {
        // rock
        res = 1;
    } else if (opponent == "A" && your == "Z")
        || (opponent == "B" && your == "Y")
        || (opponent == "C" && your == "X")
    {
        // paper
        res = 2;
    };

    // oh, you know
    res
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut sum = 0;

    for line in io::BufReader::new(file).lines().by_ref() {
        let buf = line.unwrap();
        let num: Vec<&str> = buf.split(' ').collect();
        let res = if num[1] == "X" {
            0
        } else if num[1] == "Y" {
            3
        } else {
            6
        };
        let score = res + check_score(num[0], num[1]);
        sum += score;
    }

    println!("sum {}", sum);
}
