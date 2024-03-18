// https://www.acmicpc.net/problem/2751

use std::io::{stdin, stdout, BufRead, Write, BufWriter};

fn r() -> i32 {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().parse().unwrap()
}

fn main() {
    let n = r() as usize;
    let mut nums: Vec<i32> = vec![0; n];

    for i in 0..n {
        nums[i] = r();
    }

    nums.sort();

    let mut output = BufWriter::new(stdout().lock());
    for x in nums {
        writeln!(output, "{}", x).unwrap();
    }
}