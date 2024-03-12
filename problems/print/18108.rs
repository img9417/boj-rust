// https://www.acmicpc.net/problem/18108

use std::io;

fn main() {
    let mut year = String::new();
    io::stdin().read_line(&mut year).unwrap();
    let year: i32 = year.trim_end().parse().unwrap();
    println!("{}", year-543);
}