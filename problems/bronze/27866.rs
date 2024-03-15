// https://www.acmicpc.net/problem/27866

use std::io;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    let str = r();
    let index: usize = r().trim().parse().unwrap();

    println!("{}", &str.chars().nth(index - 1).unwrap());
}