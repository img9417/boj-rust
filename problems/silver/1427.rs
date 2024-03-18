// https://www.acmicpc.net/problem/1427

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums: Vec<char> = input.chars().collect();
    nums.sort_by(|a, b| b.cmp(a));
    let nums: String = nums.into_iter().collect();

    println!("{nums}");
}