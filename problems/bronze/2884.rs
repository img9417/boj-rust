// https://www.acmicpc.net/problem/2884

use std::io;

fn main() {
    let mut time = String::new();
    io::stdin().read_line(&mut time).unwrap();
    let time: Vec<i32> = time.split_whitespace().flat_map(str::parse).collect();

    let (mut h, mut m) = (time[0], time[1]-45);

    if m < 0 {
        m += 60;
        h -= 1;
    }

    if h < 0 {
        h = 23
    }

    println!("{h} {m}");
}