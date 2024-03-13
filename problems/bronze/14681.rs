// https://www.acmicpc.net/problem/14681

use std::io;

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().parse().unwrap()
}

fn main() {
    let x = read_int();
    let y = read_int();

    match (x, y) {
        (x, y) if x > 0 && y > 0 => println!("1"),
        (x, y) if x < 0 && y > 0 => println!("2"),
        (x, y) if x < 0 && y < 0 => println!("3"),
        _ => println!("4")
    }
}