// https://www.acmicpc.net/problem/15894

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u64 = input.trim().parse().unwrap();
    let ans: u64 = input * 4;
    println!("{ans}");
}