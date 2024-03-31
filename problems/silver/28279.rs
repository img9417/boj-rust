// https://www.acmicpc.net/problem/28279

use std::io::{stdin, stdout, BufWriter, Write};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;


fn main() {
    let mut scanner = Scanner::new();
    let t: i32 = scanner.next();
    let mut deque: VecDeque<i32> = Default::default();
    let mut output = BufWriter::new(stdout().lock());
    for _ in 0..t {
        match scanner.next::<i32>() {
            1 => {
                deque.push_front(scanner.next());
            },
            2 => {
                deque.push_back(scanner.next());
            },
            3 => {
                let v = deque.pop_front().unwrap_or("-1".parse::<i32>().unwrap());
                writeln!(output, "{v}").unwrap();
            },
            4 => {
                let v = deque.pop_back().unwrap_or("-1".parse::<i32>().unwrap());
                writeln!(output, "{v}").unwrap();
            },
            5 => {
                writeln!(output, "{}", deque.len()).unwrap();
            },
            6 => {
                writeln!(output, "{}", deque.is_empty() as i32).unwrap();
            },
            7 => {
                let not_found: i32 = -1;
                let v = deque.front().unwrap_or(&not_found);
                writeln!(output, "{v}").unwrap();
            },
            8 => {
                let not_found: i32 = -1;
                let v = deque.back().unwrap_or(&not_found);
                writeln!(output, "{v}").unwrap();
            },
            _ => unreachable!()
        }
    }
}

struct Scanner{
    buffer: String,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            idx: 0,
        }
    }

    fn next<T>(&mut self) -> T where
    T: FromStr,
    <T as FromStr>::Err: Debug {
        loop {
            if self.idx >= self.buffer.len() {
                self.buffer.clear();
                stdin().read_line(&mut self.buffer).expect("Failed to read line");
                self.idx = 0;
            } else {
                let left = &self.buffer[self.idx..];
                let token = left.split_ascii_whitespace().next();

                match token {
                    Some(s) => {
                        self.idx = (left.as_ptr() as usize) - (self.buffer.as_ptr() as usize) + s.len() + 1;
                        return s.parse().expect("Failed to parse")
                    },
                    None => {
                        self.buffer.clear();
                        stdin().read_line(&mut self.buffer).expect("Failed to read line");
                        self.idx = 0;
                    },
                }
            }
        }
    }
}