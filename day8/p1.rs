use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut array: Vec<Vec<u8>> = Vec::new();
    let mut v: Vec<Vec<u8>> = vec![vec![0; 99]; 99];
    for line in lines {
        let new: Vec<u8> = line.bytes().collect();
        array.push(new);
    }

    for i in 0..array.len() {
        let len = array[i].len();
        let mut left = 0;
        let mut right = 0;
        let mut up = 0;
        let mut down = 0;

        for j in 0..len {
            if array[i][j] > left {
                if v[i][j] == 0 {
                    v[i][j] = 1;
                }
                left = array[i][j];
            }

            if array[i][len - j - 1] > right {
                if v[i][len - j - 1] == 0 {
                    v[i][len - j - 1] = 1;
                }
                right = array[i][len - j - 1];
            }

            if array[j][i] > up {
                if v[j][i] == 0 {
                    v[j][i] = 1;
                }
                up = array[j][i]
            }

            if array[len - j - 1][i] > down {
                if v[len - j - 1][i] == 0 {
                    v[len - j - 1][i] = 1;
                }
                down = array[len - j - 1][i];
            }
        }
    }

    let mut cnt: usize = 0;
    for i in &v {
        for j in i {
            cnt += *j as usize;
        }
    }
    println!("{:?}", cnt);
}
