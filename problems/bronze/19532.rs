// https://www.acmicpc.net/problem/19532

use std::io;

fn check(a: i32, b: i32, c: i32, x: i32, y: i32) -> bool {
    a*x + b*y == c
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();
    let (a,b,c,d,e,f) = (x[0],x[1],x[2],x[3],x[4],x[5]);

    for x in -999..=999 {
        for y in -999..=999 {
            if check(a,b,c,x,y) && check(d,e,f,x,y) {
                println!("{x} {y}");
            }
        }
    }
}