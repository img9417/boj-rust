// https://www.acmicpc.net/problem/15000

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let str = input.trim_end();

    println!("{}", str.to_uppercase());
}