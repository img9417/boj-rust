// https://www.acmicpc.net/problem/2485

use std::io::{stdin, stdout, BufWriter, BufRead, Write };

fn r() -> i64 {
    let input = stdin().lock();
    let mut lines = input.lines();
    lines.next().unwrap().unwrap().trim().parse().unwrap()
}

fn euclid(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    euclid(b, a%b)
}
fn main() {
    let n = r();
    let mut trees: Vec<i64> = vec![0; (n-1) as usize];

    let mut x = r();
    for i in 1..n {
        let i = i as usize;

        let y = r();
        trees[i-1] = y-x;
        x = y;
    }

    let mut gcd = trees[0];
    for i in 1..trees.len() {
        gcd = euclid(gcd, trees[i]);
    }
    
    let mut count = 0;
    for i in 0..n-1 {
        let i = i as usize;
        count += (trees[i] / gcd) - 1;
    }
    
    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}", count);
}