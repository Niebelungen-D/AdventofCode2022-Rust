use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};

fn main() {
    let file = fs::File::open("./input").unwrap();
    let mut data = Vec::new();
    let cnt = io::BufReader::new(file).read_to_end(&mut data).unwrap();
    let mut map: HashMap<u8, usize> = HashMap::new();
    let mut res = 0;
    let mut left = 0;

    for right in 0..cnt {
        if map.contains_key(&data[right]) {
            left = match map.get(&data[right]) {
                Some(&n) => {
                    if n >= left {
                        n + 1
                    } else {
                        left
                    }
                }
                _ => left,
            }
        }
        if right - left + 1 == 14 {
            res = right + 1;
            break;
        }
        map.insert(data[right], right);
    }
    println!("res {}", res);
}
