// https://www.acmicpc.net/problem/2440

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line: i32 = line.trim().parse().unwrap();

    for i in 0..line {
        for _ in i..line {
            print!("*");
        }
        println!();
    }
}