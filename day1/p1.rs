use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut max = 0;
    let mut sum = 0;
    
    for line in io::BufReader::new(file).lines() {
        let num: i32 = match line.unwrap().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };
        if num == 0 {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += num;
        }
    }
    println!("max {}", max);
}
