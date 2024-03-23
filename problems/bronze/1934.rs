// https://www.acmicpc.net/problem/1934

use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()
}

fn euclid(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    let r = a % b;
    euclid(b, r)
}

fn main() {
    let t = r().parse::<usize>().unwrap();
    let mut output = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let ab: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();
        writeln!(output, "{}", (ab[0]*ab[1])/euclid(ab[0], ab[1])).unwrap();
    }
}
