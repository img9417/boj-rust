// https://www.acmicpc.net/problem/13909
use std::io::{stdin, stdout, BufRead, Write, BufWriter};

fn main() {
    let input = stdin().lock();
    let mut line = input.lines();
    let n = line.next().unwrap().unwrap().trim().parse::<i64>().unwrap();

    for i in 1..=n {
        if i*i == n {
            println!("{}", i);
            break;
        } else if i*i > n {
            println!("{}", i-1);
            break;
        }
    }
}
