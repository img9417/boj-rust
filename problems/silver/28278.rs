// https://www.acmicpc.net/problem/28278

use std::io::{self, BufRead, Write, BufWriter};

fn r() -> String {
    let input = io::stdin().lock();
    let mut line = input.lines();
    line.next().unwrap().unwrap().trim().to_owned()
}

fn main() {
    let n = r().parse::<i32>().unwrap();
    let mut stack: Vec<i32> = vec![];
    let mut output = BufWriter::new(io::stdout().lock());
    for _ in 0..n {
        let orders: Vec<i32> = r().split_whitespace().flat_map(str::parse).collect();
        match orders[0] {
            1 => {
                stack.push(orders[1]);
            },
            2 => {
                if stack.len() == 0 {
                    writeln!(output,"-1");
                } else {
                    let x = stack.pop().unwrap();
                    writeln!(output,"{}", x);
                }
            },
            3 => {
                writeln!(output,"{}", stack.len());
            },
            4 => {
                if stack.len() == 0 {
                    writeln!(output,"1");
                } else {
                    writeln!(output,"0");
                }
            },
            5 => {
                if stack.len() == 0 {
                    writeln!(output,"-1");
                } else {
                    writeln!(output,"{}", stack.last().unwrap());
                }
            },
            _ => panic!()
        }
    }
}