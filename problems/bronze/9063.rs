// https://www.acmicpc.net/problem/9063

use std::io;
use std::cmp::{max, min};

fn r() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_owned()
}

fn main() {
    let points = r().parse::<i32>().unwrap();
    let mut max_x: i64 = -10000;
    let mut max_y: i64 = -10000;
    let mut min_x: i64 = 10000;
    let mut min_y: i64 = 10000;

    for _ in 0..points {
        let xy: Vec<i64> = r().split_whitespace().flat_map(str::parse).collect();
        max_x = max(xy[0], max_x);
        max_y = max(xy[1], max_y);
        min_x = min(xy[0], min_x);
        min_y = min(xy[1], min_y);
    }

    println!("{}", (max_x-min_x)*(max_y-min_y));
}