use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(n) => write!(f, "{}", n),
            Node::List(l) => write!(f, "{:?}", l),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];

    let mut packets = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line: &str| serde_json::from_str::<Node>(line).unwrap())
        .chain(dividers.clone())
        .collect::<Vec<_>>();

    packets.sort();

    let decode_key = dividers.iter()
        .map(|d| packets.binary_search(d).unwrap() + 1).product::<usize>();

    println!("decode_key = {}", decode_key);

}
