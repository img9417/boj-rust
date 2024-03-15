// https://www.acmicpc.net/problem/2798

use std::io;
use std::cmp::{max};

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    let nm: Vec<u64> = r().split_whitespace().flat_map(str::parse).collect();
    let cards: Vec<u64> = r().split_whitespace().flat_map(str::parse).collect();
    let mut sum: u64 = 0;

    for i in 0..nm[0] {
        for j in i+1..nm[0] {
            for k in j+1..nm[0] {
                let v = cards[i as usize] + cards[j as usize] + cards[k as usize];
                if v <= nm[1] {
                    sum = max(v, sum);
                }
            }
        }
    }

    println!("{}", sum);
}