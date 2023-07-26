use std::fs;
use std::io::{self, BufRead};

fn get_score(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;

    for j in (y + 1)..map[0].len() {
        left += 1;
        if map[x][j] >= map[x][y] {
            break;
        }
    }

    for j in 1..=y {
        right += 1;
        if map[x][y - j] >= map[x][y] {
            break;
        }
    }

    for i in (x + 1)..map.len() {
        down += 1;
        if map[i][y] >= map[x][y] {
            break;
        }
    }

    for i in 1..=x {
        up += 1;
        if map[x - i][y] >= map[x][y] {
            break;
        }
    }
    left * right * up * down
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut array: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let new: Vec<u8> = line.bytes().collect();
        array.push(new);
    }

    let mut max = 0;
    for i in 1..(array.len() - 1) {
        let len = array[i].len();
        for j in 1..(len - 1) {
            max = std::cmp::max(max, get_score(&array, i, j))
        }
    }

    println!("max: {}", max)
}
