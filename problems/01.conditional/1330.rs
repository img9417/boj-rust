use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();

    match nums[0].cmp(&nums[1]) {
        Ordering::Less => println!("<"),
        Ordering::Greater => println!(">"),
        Ordering::Equal => println!("=="),
    }
}