// https://www.acmicpc.net/problem/2588

use std::io;

fn read_int() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim_end().parse().unwrap()
}

fn main() {
    let x = read_int();
    let mut y = read_int();
    let z = x*y;

    println!("{}", x*(y%10));
    y /= 10;
    println!("{}", x*(y%10));
    y /= 10;
    println!("{}", x*y);

    println!("{}", z);
}
