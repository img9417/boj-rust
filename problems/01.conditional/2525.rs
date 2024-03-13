use std::io;

fn read_time() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();
    
    (nums[0], nums[1])
}

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim_end().parse().unwrap()
}

fn main() {
    let (mut h, mut m) = read_time();
    m += read_int();

    h = (h+m/60)%24;
    m %= 60;

    println!("{h} {m}");
}