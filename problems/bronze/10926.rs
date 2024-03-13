// https://www.acmicpc.net/problem/10926

use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();
    println!("{}??!", name);
}