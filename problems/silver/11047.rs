// https://www.acmicpc.net/problem/11047
use std::io::stdin;
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.next();
    let mut k: u32 = scanner.next();
    let mut coins: Vec<u32> = Vec::with_capacity(n);
    let mut res = 0;

    for i in 0..n {
        let v: u32 = scanner.next();
        coins.push(v);
    }

    for coin in coins.iter().rev() {
        if k == 0 {
            break;
        }

        res += k/coin;
        k %= coin;
    }
    
    println!("{}", res);
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