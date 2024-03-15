// https://www.acmicpc.net/problem/9086

use std::io;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    let t = r().parse().unwrap();

    for _ in 0..t {
        let s = r();
        let size = s.len();

        println!("{}{}", s.chars().nth(0).unwrap(), s.chars().nth(size-1).unwrap());
    }
}