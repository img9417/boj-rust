use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();
    let a = nums[0];
    let b = nums[1];
    let c = nums[2];

    println!("{}", (a+b)%c);
    println!("{}", ((a%c)+(b%c))%c);
    println!("{}", (a*b)%c);
    println!("{}", ((a%c)*(b%c))%c);
}