// https://www.acmicpc.net/problem/15963

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<u64> = input.split_whitespace().flat_map(str::parse).collect();

    println!("{}", if nums[0] == nums[1] { 1 } else {0});
}