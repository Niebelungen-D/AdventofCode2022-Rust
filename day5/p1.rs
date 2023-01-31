use sscanf::sscanf;
use std::fs;
use std::io::{self, BufRead};

fn read_arr(a: &mut Vec<Vec<char>>, l: &str) {
    for (k, v) in l.chars().enumerate() {
        match v {
            'A'..='Z' => a[k / 4].insert(0, v),
            _ => (),
        }
    }
}

fn do_move(a: &mut Vec<Vec<char>>, num: usize, from: usize, to: usize) {
    for _ in 1..=num {
        let c = a[from - 1].pop().unwrap();
        a[to - 1].push(c);
    }
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut arr: Vec<Vec<char>> = Vec::new();
    let mut cnt = 1;

    for _ in 1..=9 {
        arr.push(Vec::new());
    }

    for line in lines {
        if cnt <= 8 {
            read_arr(&mut arr, &line);
        } else if cnt >= 11 {
            let p = sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();
            do_move(&mut arr, p.0, p.1, p.2);
        }
        cnt += 1;
    }
    for i in arr {
        print!("{}", i.get(i.len() - 1).unwrap());
    }
    // println!("{:#?}", arr);
}
