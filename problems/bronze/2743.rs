// https://www.acmicpc.net/problem/2743

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.len()-1);
}