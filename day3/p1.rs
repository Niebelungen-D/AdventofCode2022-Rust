use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

fn find_comm(a: &str, b: &str) -> i32 {
    let mut map = HashMap::new();
    let mut num = 0;
    for i in a.chars() {
        map.insert(i, 1);
    }
    for i in b.chars() {
        if map.contains_key(&i) {
            num = match i {
                'a'..='z' => i as i32 - 0x61 + 1,
                'A'..='Z' => i as i32 - 0x41 + 1 + 26,
                _ => 0
            };
        }
    }
    num
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut sum = 0;

    for line in io::BufReader::new(file).lines().by_ref() {
        let rucksack = line.unwrap();
        let len = rucksack.len();
        if len % 2 != 0 {
            panic!("len is odd!");
        }
        let first = &rucksack[..(len / 2)];
        let second = &rucksack[(len / 2)..];
        sum += find_comm(first, second);

        // println!("{} + {}", first, second);
    }
    println!("sum {}", sum);
}
