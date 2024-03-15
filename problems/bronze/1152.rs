// https://www.acmicpc.net/problem/1152

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<_> = input.trim().split_whitespace().collect();
    println!("{}", input.len());
}