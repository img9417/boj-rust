// https://www.acmicpc.net/problem/1110

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim_end().parse().unwrap();
    let mut x = input;
    let mut cnt: i32 = 1;

    loop {
        x = (x%10*10) + (x/10 + x%10)%10;
        if x==input {
            break;
        }
        cnt += 1
    }

    println!("{cnt}");
}