// https://www.acmicpc.net/problem/10870

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim_end().parse().unwrap();

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    match n {
        0 => println!("{}", a),
        1 => println!("{}", b),
        _ => {
            for _ in 2..=n {
                let c = a+b;
                a = b;
                b = c;
            }
            println!("{}", b);
        }
    }
}