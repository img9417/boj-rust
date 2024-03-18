// https://www.acmicpc.net/problem/11650

use std::cmp::Ordering;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap()
}

fn main() {
    let n: i32 = r().trim().parse().unwrap();
    let mut xy: Vec<(i32, i32)> = vec![];

    for _ in 0..n {
        let input: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();
        xy.push((input[0], input[1]));
    }

    xy.sort_by(|a, b| match (a, b) {
        (a, b) if a.0 < b.0 => Ordering::Less,
        (a, b) if a.0 == b.0 && a.1 < b.1 => Ordering::Less,
        _ => Ordering::Greater,
    });

    let mut output = BufWriter::new(stdout().lock());
    for x in xy {
        writeln!(output, "{} {}", x.0, x.1).unwrap();
    }
}
