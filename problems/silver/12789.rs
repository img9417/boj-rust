// https://www.acmicpc.net/problem/12789

use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn r() -> String {
    let input = stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap()
}

fn main() {
    r();
    let wait: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();
    let mut temp: Vec<i32> = vec![];
    let mut target: i32 = 1;
    let mut possible = true;

    for i in wait {
        temp.push(i);

        loop {
            let len = temp.len();
            if len == 0 {
                break;
            }


            let last = temp[len-1];
            if last == target {
                temp.pop();
                target += 1;
            } else {
                break;
            }
        }

        let len = temp.len();
        if len > 1 && temp[len-1] > temp[len-2] {
            possible = false;
            break;
        }
    }

    let mut output = BufWriter::new(stdout().lock());
    if temp.len() > 0 || !possible {
        writeln!(output, "Sad");
    } else {
        writeln!(output, "Nice");
    }
}
