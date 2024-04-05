// https://www.acmicpc.net/problem/10866

use std::io::{stdin, stdout, BufWriter, BufRead, Write};
use std::collections::VecDeque;

#[derive(Default)]
struct Scanner {
	buffer: Vec<String>
}

impl Scanner {
	fn next<T: std::str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buffer.pop() {
				return token.parse().ok().expect("Failed parse");
			}
			let mut input = String::new();
			stdin().read_line(&mut input).expect("Failed read");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout().lock());
    let mut deq: VecDeque<i32> = VecDeque::new();

    let t = scan.next();

    for _ in 0..t {
        match scan.next::<String>().as_str() {
            "push_front" => {
                let v = scan.next();
                deq.push_front(v);
            },
            "push_back" => {
                let v = scan.next();
                deq.push_back(v);
            },
            "pop_front" => {
                let dne = -1;
                let val = deq.pop_front();
                writeln!(out, "{}", val.unwrap_or(dne));
            },
            "pop_back" => {
                let dne = -1;
                let val = deq.pop_back();
                writeln!(out, "{}", val.unwrap_or(dne));
            },
            "size" => {
                writeln!(out, "{}", deq.len());
            },
            "empty" => {
                writeln!(out, "{}", deq.is_empty() as i32);
            },
            "front" => {
                let dne = -1;
                writeln!(out, "{}", deq.front().unwrap_or(&dne));
            },
            "back" => {
                let dne = -1;
                writeln!(out, "{}", deq.back().unwrap_or(&dne));
            },
            _ => unreachable!()
        }
    }
}