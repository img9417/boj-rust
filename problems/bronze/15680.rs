// https://www.acmicpc.net/problem/15680

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();

    match input {
        0 => println!("YONSEI"),
        _ => println!("Leading the Way to the Future")
    }
}