use sscanf::sscanf;
use std::fs;
use std::io::{self, BufRead};

struct Point(i32, i32);
struct Pair {
    a: Point,
    b: Point,
}

impl Pair {
    fn new(p: (i32, i32, i32, i32)) -> Self {
        Pair {
            a: Point(p.0, p.1),
            b: Point(p.2, p.3),
        }
    }

    fn is_overlaped(&self) -> i32 {
        if self.a.1 < self.b.0 || self.a.0 > self.b.1
        {
            return 0;
        }
        1
    }
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut sum = 0;
    for line in lines {
        let t = sscanf!(line, "{i32}-{i32},{i32}-{i32}").unwrap();
        let p = Pair::new(t);
        sum += p.is_overlaped();
    }
    println!("sum {}", sum);
}
