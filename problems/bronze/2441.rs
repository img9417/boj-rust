// https://www.acmicpc.net/problem/2441

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim_end().parse().unwrap();

    for i in 0..n {
        for _ in 0..i {
            print!(" ");
        }

        for _ in i..n {
            print!("*");
        }

        println!();
    }
}