// https://www.acmicpc.net/problem/24883

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse().unwrap();

    match input {
        'n' | 'N' => println!("Naver D2"),
        _ => println!("Naver Whale")
    }
}