use std::cell::RefCell;
use std::cell::RefMut;
use std::fs;
use std::io::{self, BufRead};
use std::rc::Rc;

#[derive(Debug)]
struct MyFile {
    filename: String,
    size: usize,
    dir: Rc<RefCell<Box<Dir>>>,
}

impl MyFile {
    fn new(name: &str, size: usize, dir: Rc<RefCell<Box<Dir>>>) -> Self {
        MyFile {
            filename: name.to_string(),
            size: size,
            dir: dir,
        }
    }
}

#[derive(Debug)]
struct Dir {
    dirname: String,
    files: Vec<Rc<RefCell<MyFile>>>,
    dirs: Vec<Rc<RefCell<Box<Dir>>>>,
    up_dir: Option<Rc<RefCell<Box<Dir>>>>,
    size: usize,
}

impl Dir {
    fn new(name: &str, up: Option<Rc<RefCell<Box<Dir>>>>) -> Self {
        Dir {
            dirname: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
            up_dir: match up {
                None => None,
                Some(n) => Some(n),
            },
            size: 0,
        }
    }
}

fn calc_size(mut root: RefMut<Box<Dir>>, cnt: &mut usize) {
    let v = root.dirs.clone();
    for i in v {
        let t = i.clone();
        calc_size(t.borrow_mut(), cnt);
        root.size += t.borrow().size;
    }
    if root.size <= 100000 {
        *cnt += root.size;
    }
}

fn main() {
    let file = fs::File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let root = Rc::new(RefCell::new(Box::new(Dir::new("/", None))));
    let mut cur = root.clone();
    let mut up = None;

    for line in lines {
        let data: Vec<&str> = line.split(" ").collect();
        match data[0] {
            "$" => match data[1] {
                "ls" => (),
                "cd" => {
                    if data[2] != "/" && data[2] != ".." {
                        let t = cur.clone();
                        let new = &t.borrow().dirs;
                        for i in new {
                            if i.borrow().dirname == data[2] {
                                up = Some(cur.clone());
                                cur = i.clone();
                                break;
                            }
                        }
                    } else if data[2] == ".." {
                        let t = cur.clone();
                        cur = up.clone().unwrap();
                        up = t.borrow().up_dir.clone();
                    }
                }
                _ => panic!("cmd not found!"),
            },
            "dir" => {
                // dir
                let new = Rc::new(RefCell::new(Box::new(Dir::new(data[1], up.clone()))));
                cur.borrow_mut().dirs.push(new.clone());
            }
            _ => {
                // file
                let size: usize = data[0].parse().unwrap();
                let new = Rc::new(RefCell::new(MyFile::new(data[1], size, cur.clone())));
                cur.borrow_mut().files.push(new);
                cur.borrow_mut().size += size;
            }
        }
    }
    let mut cnt = 0;
    calc_size(root.borrow_mut(), &mut cnt);
    println!("cnt {}", cnt);
}
