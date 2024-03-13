// https://www.acmicpc.net/problem/27889

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end();

    match input {
        "NLCS" => println!("North London Collegiate School"),
        "BHA" => println!("Branksome Hall Asia"),
        "KIS" => println!("Korea International School"),
        "SJA" => println!("St. Johnsbury Academy"),
        _ => ()
    }
}