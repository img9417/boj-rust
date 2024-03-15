// https://www.acmicpc.net/problem/11720

use std::io;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    r();
    let number = r();
    let mut sum: i32 = 0;

    for num in number.chars() {
        sum += (num as i32) - ('0' as i32);
    }

    println!("{}", sum);
}