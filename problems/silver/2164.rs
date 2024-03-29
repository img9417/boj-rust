// https://www.acmicpc.net/problem/2164

use std::io;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut q: VecDeque<u32> = Default::default();

    for i in 1..=input {
        q.push_back(i as u32);
    }

    while q.len() != 1 {
        q.pop_front();
        let v = q.pop_front().unwrap();
        q.push_back(v);
    }

    println!("{}", q.front().unwrap());
}