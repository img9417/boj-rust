// https://www.acmicpc.net/problem/18870

use std::io::{stdin, stdout, BufRead, Write, BufWriter};
use std::collections::HashMap;

fn r() -> String {
    let input = stdin().lock();
    let mut lines = input.lines();
    lines.next().unwrap().unwrap()
}

fn main() {
    r();
    let y: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();
    let mut x = y.clone();
    x.sort();

    let mut rank = 0;
    let mut map = HashMap::new();
    for v in x {
        if map.contains_key(&v) == false {
            map.insert(v, rank);
            rank += 1;
        }
    }

    let mut output = BufWriter::new(stdout().lock());
    for v in y {
        write!(output, "{} ", map.get(&v).unwrap()).unwrap();
    }

}