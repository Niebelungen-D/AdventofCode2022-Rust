use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

fn find_comm(a: &str, b: &str) -> Vec<char> {
    let mut map = HashMap::new();
    let mut res: Vec<char> = Vec::new();
    for i in a.chars() {
        map.insert(i, 1);
    }
    for i in b.chars() {
        if map.contains_key(&i) {
            res.push(i);
        }
    }
    res
}

fn find_comm_tree(a: &str, b: &str, c: &str) -> i32 {
    let mut num = 0;
    let r1 = find_comm(a, b);
    for i in r1 {
        if c.contains(i) {
            num = match i {
                'a'..='z' => i as i32 - 0x61 + 1,
                'A'..='Z' => i as i32 - 0x41 + 1 + 26,
                _ => 0,
            };
        }
    }
    num
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut sum = 0;

    loop {
        let l1 = &lines.next();
        let l2 = &lines.next();
        let l3 = &lines.next();
        if l1.is_none() || l2.is_none() || l3.is_none() {
            break;
        }
        sum += find_comm_tree(
            l1.as_ref().unwrap(),
            l2.as_ref().unwrap(),
            l3.as_ref().unwrap(),
        );
    }
    println!("sum {}", sum);
}
