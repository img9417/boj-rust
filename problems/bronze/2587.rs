// https://www.acmicpc.net/problem/2587

use std::io::{stdin, stdout, BufRead, Write, BufWriter};

fn r() -> i32 {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().parse().unwrap()
}

fn main() {
    let mut nums: Vec<i32> = vec![0; 5];
    
    let mut avg = 0;
    for i in 0..5 {
        let x = r();
        avg += x;
        nums[i] = x;
    }
    avg /= 5;

    // selection sort
    for i in 0..5 {
        let mut min_pos = 0;
        let mut min = nums[i];

        for j in i..5 {
            if nums[j] < min {
                min = nums[j];
                min_pos = j;
            }
        }

        nums[min_pos] = nums[i];
        nums[i] = min;
    }
    let median = nums[2];

    let mut output = BufWriter::new(stdout().lock());
    writeln!(output, "{}\n{}", avg, median).unwrap();
}