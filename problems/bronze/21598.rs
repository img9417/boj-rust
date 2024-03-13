// https://www.acmicpc.net/problem/21598

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();

    for _ in 0..x {
        println!("SciComLove");
    }
}