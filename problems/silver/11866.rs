// https://www.acmicpc.net/problem/11866

use std::io::{stdin, BufRead};
use std::collections::VecDeque;

fn main() {
    let input = stdin().lock();
    let mut line = input.lines();
    let input: Vec<i32> = line.next().unwrap().unwrap().split_whitespace().flat_map(str::parse).collect();

    let mut q: VecDeque<i32> = Default::default();
    let mut res: Vec<i32> = vec![];

    for i in 1..=input[0] {
        q.push_back(i);
    }

    let k = input[1] as usize;
    let mut i: usize = 0;

    while q.len() > 0 {
        i += k - 1;
        i %= q.len();
        res.push(q.remove(i).unwrap());
    }

    print!("<{}>", res.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
}

// 1 2 3 4 5 6 7 => 3 (k=3)
// 1 2 4 5 6 7 => 6 => (k=5)
// 1 2 4 5 7 => 2 => (k=2)