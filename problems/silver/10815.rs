// https://www.acmicpc.net/problem/10815

use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut lines = input.lines();
    lines.next().unwrap().unwrap()
}

fn main() {
    r();
    let sangeun: HashSet<i32> = r()
        .split_whitespace()
        .flat_map(str::parse)
        .collect();

    r();
    let check: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();

    let mut output = BufWriter::new(stdout().lock());
    for x in check {
        if sangeun.contains(&x) {
            write!(output, "1 ").unwrap();
        } else {
            write!(output, "0 ").unwrap();
        }
    }
}
