// https://www.acmicpc.net/problem/2442

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim_end().parse().unwrap();

    for i in 0..n {
        for _ in (i+1)..n {
            print!(" ");
        }

        for _ in 0..=(2*i) {
            print!("*");
        }

        println!();
    }
}