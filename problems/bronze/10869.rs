// https://www.acmicpc.net/problem/10869

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<u32> = input.split_whitespace().flat_map(str::parse).collect();
    println!("{}", nums[0] + nums[1]);
    println!("{}", nums[0] - nums[1]);
    println!("{}", nums[0] * nums[1]);
    println!("{}", nums[0] / nums[1]);
    println!("{}", nums[0] % nums[1]);
}