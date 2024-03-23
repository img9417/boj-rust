// https://www.acmicpc.net/problem/1934

use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn euclid(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    let r = a % b;
    euclid(b, r)
}

fn main() {
    let input = stdin().lock();
    let mut line = input.lines();

    let ab: Vec<u64> = line.next().unwrap().unwrap().split_whitespace().flat_map(str::parse).collect();

    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}", (ab[0]*ab[1])/euclid(ab[0], ab[1])).unwrap();
}
