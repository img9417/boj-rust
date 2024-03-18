// https://www.acmicpc.net/problem/2750

use std::io::{stdin, stdout, Write, BufRead, BufWriter};

fn r() -> i32 {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().parse().unwrap()
}
fn main() {
    let t = r();
    let mut nums: Vec<i32> = vec![0; t as usize];

    for i in 0..t {
        nums[i as usize] = r();
    }

    // nums.sort();

    // bubble sort
    for i in 0..t {
        let mut already_sorted = true;
        
        for j in 0..t-i-1 {
            let j = j as usize;
            if nums[j] > nums[j+1] {
                already_sorted = false;
                let temp = nums[j];
                nums[j] = nums[j+1];
                nums[j+1] = temp;
            }
        }

        if already_sorted {
            break;
        }
    }

    let mut output = BufWriter::new(stdout().lock());
    for x in nums {
        writeln!(output, "{}", x).unwrap();
    }
}