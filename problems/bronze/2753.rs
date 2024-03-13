// https://www.acmicpc.net/problem/2753

use std::io;

fn main() {
    let mut year = String::new();
    io::stdin().read_line(&mut year).unwrap();
    let year: i32 = year.trim().parse().unwrap();

    let is_leap_year = (year%4==0 && year%100 != 0) || year % 400 == 0;
    match is_leap_year {
        true => println!("1"),
        false => println!("0"),
    }
}