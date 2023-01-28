use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut sum = 0;
    let mut res = vec![0,0,0];
    
    for line in io::BufReader::new(file).lines() {
        let num: i32 = match line.unwrap().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };
        if num == 0 {
            if sum > res[0] {
                res[0] = sum;
            }
            sum = 0;
            res.sort();
        } else {
            sum += num;
        }
    }
    println!("max {}", res[0] + res[1] + res[2]);
}