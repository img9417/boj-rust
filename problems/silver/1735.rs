// https://www.acmicpc.net/problem/1735

use std::io::{stdin, stdout, Write, BufWriter, BufRead};

fn r() -> Vec<i32> {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().split_whitespace().flat_map(str::parse).collect()
}

fn euclid(a: i32, b: i32) -> i32 {
    if b==0 {
        return a;
    }

    euclid(b, a%b)
}

fn main() {
    let a = r();
    let b = r();
    let num = (a[1]*b[0]) + (a[0]*b[1]);
    let den = a[1]*b[1];

    let gcd = euclid(num, den);

    let num = num / gcd;
    let den = den / gcd;

    println!("{} {}", num, den);
}
