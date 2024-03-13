// https://www.acmicpc.net/problem/27294

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();
    let (time, drink) = (nums[0], nums[1]);

    match (time, drink) {
        (_, drink) if drink==1 => println!("280"),
        (time, _) if time < 12 || time > 16 => println!("280"),
        (time, drink) if (time >= 12 && time <= 16) && drink==0 => println!("320"),
        _ => println!("error")
    }
}