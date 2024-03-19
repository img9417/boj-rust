// https://www.acmicpc.net/problem/14425

use std::io::{stdin, stdout, BufRead, Write, BufWriter};
use std::collections::HashSet;

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap()
}

fn main() {
    let nm: Vec<usize> = r().split_whitespace().flat_map(str::parse).collect();
    let mut s: HashSet<String> = HashSet::new();

    for _ in 0..nm[0] {
        let word = r().trim_end().to_owned();
        s.insert(word);
    }

    let mut cnt = 0;
    for _ in 0..nm[1] {
        let word = r().trim_end().to_owned();
        if s.contains(&word) {
            cnt += 1;
        }
    }

    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}", cnt).unwrap();
}