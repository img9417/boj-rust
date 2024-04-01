// https://www.acmicpc.net/problem/10845

use std::io::{stdin, stdout, Write, BufWriter};
use std::collections::LinkedList;

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
    let out = &mut BufWriter::new(stdout());

    let mut q: LinkedList<i32> = LinkedList::new();
    let t: i32 = scan.next();

    for _ in 0..t {
        match scan.next::<String>().as_str() {
            "push" => {
                q.push_back(scan.next::<i32>());
            },
            "pop" => {
                let dne: i32 = -1;
                let v = q.pop_front().unwrap_or(dne);
                writeln!(out, "{}", v);
            },
            "size" => {
                writeln!(out, "{}", q.len());
            },
            "empty" => {
                writeln!(out, "{}", q.is_empty() as i32);
            },
            "front" => {
                let dne: i32 = -1;
                let v = q.front().unwrap_or(&dne);
                writeln!(out, "{}", v);
            },
            "back" => {
                let dne: i32 = -1;
                let v = q.back().unwrap_or(&dne);
                writeln!(out, "{}", v);
            },
            _ => unreachable!()
        }
    }
}