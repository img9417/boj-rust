// https://www.acmicpc.net/problem/10989

use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> i32 {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().parse().unwrap()
}

fn main() {
    let n = r();
    let mut nums: Vec<i32> = vec![0; 10001];

    for _ in 0..n {
        nums[r() as usize] += 1;
    }

    let mut output = BufWriter::new(stdout().lock());
    for i in 1..10001 {
        if nums[i] > 0 {
            for _ in 0..nums[i] {
                writeln!(output, "{}", i).unwrap();
            }
        }
    }
}