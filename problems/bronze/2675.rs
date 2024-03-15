// https://www.acmicpc.net/problem/2675

use std::io;

fn r() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main() {
    let t = r().parse().unwrap();
    for _ in 0..t {
        let data: Vec<String> = r().split_whitespace().flat_map(str::parse).collect();
        let rep: i32 = data[0].parse().unwrap();
        let s = &data[1];
        for ch in s.chars() {
            for _ in 0..rep {
                print!("{ch}");
            }
        }
        println!();
    }
}